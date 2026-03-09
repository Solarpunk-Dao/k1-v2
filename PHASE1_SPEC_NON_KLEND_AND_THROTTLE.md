# Phase 1 Spec — Non-KLend Strategies + Withdraw Throttle (No Swap Path)

## Objective
Implement Phase 1 only:
1. Add support for non-KLend reported-value strategies (state + instructions + valuation integration).
2. Add withdraw throttle (rolling 24h window, capped redemption).
3. Keep existing token redemption path (no swap-to-USDC in this phase).

---

## Scope

### In scope
- Strategy metadata/state for non-KLend strategies.
- Strategy value reporting instruction with auth controls.
- Optional vault-level offchain NAV field.
- Extend total-assets/AUM computation to include non-KLend reported value + offchain NAV.
- Wire extended AUM into deposit/withdraw/fee charging call-sites.
- Withdraw throttle state and enforcement in withdraw path.
- Tests (unit + instruction-level integration) for all new logic.

### Out of scope
- Any DEX swap path or redemption-in-USDC logic.
- Any change to user-facing token returned on withdraw.
- Advanced oracle design; Phase 1 uses controlled reporter authority.

---

## Required behavior

### A. Non-KLend strategy model
- Add strategy type enum: `KLend | ReportedValue`.
- Add PDA account for non-KLend strategy state with at minimum:
  - `vault`
  - `strategy_id`
  - `strategy_type`
  - `reporter_authority`
  - `escrow_token_account`
  - `target_allocation_weight` (for future use)
  - `allocation_cap` (for future use)
  - `last_reported_value`
  - `last_report_ts`
  - `enabled`
- Add PDA seed constant, account-size constant.

### B. Instructions
1. `init_non_klend_strategy(strategy_id, strategy_type, reporter_authority, escrow_token_account, ...)`
   - Admin-gated.
   - Creates strategy PDA.
   - Initializes strategy fields.

2. `report_non_klend_strategy_value(strategy_id, last_reported_value)`
   - Authorized by `reporter_authority` (or vault admin if explicitly enabled by policy).
   - Valid only for `ReportedValue` strategy type.
   - Updates value + report timestamp.

3. (Optional but recommended) `set_offchain_nav(value)`
   - Admin-gated.
   - Sets vault-level additive NAV component.

### C. Extended valuation/AUM
Define canonical total assets:

`total_assets = token_available + klend_invested_total + sum(non_klend.last_reported_value) + offchain_nav`

Then AUM:

`aum = total_assets - pending_fees`

Use this extended AUM everywhere share/fee accounting depends on vault value.

### D. Withdraw throttle
- Add to vault state:
  - `withdraw_throttle_bps` (default 500 = 5%)
  - `redeemed_in_period`
  - `period_start_ts`
  - Store these in existing `padding_4` bytes to preserve `VaultState` size.
- In withdraw flow (before payout effects):
  1. If `now - period_start_ts >= 86400`: reset window.
  2. Compute `limit = throttle_bps * total_assets / 10000`.
  3. Require `redeemed_in_period + this_withdrawal <= limit`.
  4. On success, increment `redeemed_in_period`.

---

## File-by-file implementation map

### 1) `programs/kvault/src/utils/consts.rs`
- Add non-KLend PDA seed const.
- Add non-KLend account size const.

### 2) `programs/kvault/src/state.rs`
- Add `StrategyType` enum.
- Add `NonKlendStrategyState` account struct.
- Add throttle/offchain NAV fields to `VaultState`.
- Add getters/setters for new fixed-point fields.
- Maintain zero-copy/static-size invariants (consume padding if needed).

### 3) `programs/kvault/src/operations/non_klend_strategy_operations.rs`
- Add pure logic functions:
  - `init_reported_value_strategy`
  - `report_strategy_value` (auth/type/enabled checks)

### 4) `programs/kvault/src/operations/vault_operations.rs`
- Add `total_assets_extended(...)` helper.
- Add `compute_aum_from_total_assets(...)` helper.
- Update:
  - `deposit(...)`
  - `withdraw(...)`
  - `charge_fees(...)`
  to use extended AUM.
- Add throttle enforcement helper and call in `withdraw(...)`.

### 5) `programs/kvault/src/handlers/`
- Add:
  - `handler_init_non_klend_strategy.rs`
  - `handler_report_non_klend_strategy_value.rs`
  - `handler_set_offchain_nav.rs` (if included)
- Ensure account constraints enforce admin/reporter rules.

### 6) `programs/kvault/src/handlers/mod.rs`
- Export new handlers.

### 7) `programs/kvault/src/operations/mod.rs`
- Export non-KLend operations module.

### 8) `programs/kvault/src/lib.rs`
- Add new instruction entrypoints.
- Add any new error codes:
  - invalid reporter/type
  - strategy disabled/not found
  - throttle exceeded
  - stale report (if policy added)

---

## Access control policy (Phase 1)
- `init_non_klend_strategy`: vault admin only.
- `report_non_klend_strategy_value`: strategy `reporter_authority` only (optionally vault admin override).
- `set_offchain_nav`: vault admin only.
- Throttle params: vault admin (or via existing vault config flow if integrated there).

---

## Validation rules
- Reject reports if strategy is disabled.
- Reject reports if strategy type is not `ReportedValue`.
- Reject updates from unauthorized reporter.
- Require non-default reporter authority on strategy initialization.
- Prevent AUM underflow vs pending fees.
- Throttle checks must run before mutating payout accounting.
- Enforce `effective_withdraw_throttle_bps <= 10000` at state validation.

---

## Test plan (must pass)

### Unit tests
- Strategy report success.
- Strategy report fails: wrong reporter.
- Strategy report fails: wrong strategy type.
- Strategy report fails: disabled strategy.
- Throttle helper: under/over limit + period rollover.

### Integration tests (instruction-level)
- Init strategy PDA success with correct admin.
- Init strategy fails with wrong admin.
- Report value succeeds with correct reporter.
- Report value fails with wrong reporter.
- Report value updates on-chain fields correctly.
- Withdraw over throttle limit reverts.
- Withdraw within throttle limit succeeds.

### Regression tests
- Existing KLend-only flows continue working with zero non-KLend strategies.
- Deposit/withdraw/fee behavior unchanged when non-KLend values are all zero.

---

## Acceptance criteria
- Extended AUM is the single source of truth for deposit/withdraw/fee math.
- Non-KLend strategy reporting works on-chain with enforced auth.
- Withdraw throttle enforces configured cap per period.
- All tests above pass.
- No swap path introduced in this phase.
