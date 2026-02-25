# Strategy Expansion Plan (Fresh Deploy)

This plan updates `kamino_vault` from a KLend-only vault into a multi-strategy vault with minimal disruption to the existing accounting/safety model.

## Confirmed product decisions

- Support allocations beyond KLend (multiple tokens + liquidity pools).
- Whitelist policy: strategy is allowed if either program OR token mint is whitelisted.
- Deployment mode: fresh deploy (state layout can change).
- Withdraw behavior: disinvest from user-chosen allocation.
- Oracle support is allowed for heterogeneous strategies/tokens.
- Internal strategy adapter pattern is approved.

## Design goals

1. Keep share accounting and fee math battle-tested where possible.
2. Isolate protocol-specific CPIs behind adapters.
3. Make whitelist checks generic and independent of KLend reserve accounts.
4. Preserve explicit withdraw-from-chosen-allocation behavior.

## Required code touch points

### 1) State model (`programs/kvault/src/state.rs`)

- Replace KLend-centric allocation fields with strategy-generic fields.
- Proposed allocation fields:
  - `strategy_program: Pubkey`
  - `strategy_target: Pubkey` (pool/reserve/position key)
  - `underlying_token_mint: Pubkey`
  - `position_vault: Pubkey` (vault-owned receipt/share LP token account)
  - `position_amount: u64`
  - keep existing weights/caps/timing fields
- Keep `VaultState` fee/shares/accounting structure intact.
- Add whitelist account type(s):
  - `WhitelistedProgramEntry`
  - `WhitelistedMintEntry`

### 2) Constants + PDA seeds (`programs/kvault/src/utils/consts.rs`)

- Add seeds for program whitelist and mint whitelist PDA namespaces.
- Keep existing seeds for backward compatibility only if needed during migration period.

### 3) Governance/whitelist handlers

- Existing reserve whitelist handler is KLend-specific (`handler_add_update_whitelisted_reserve.rs`).
- Add generic handlers:
  - `handler_add_update_whitelisted_program.rs`
  - `handler_add_update_whitelisted_mint.rs`
- Update access rules so allocation/invest is allowed when either whitelist passes.

### 4) Allocation update instruction

- Existing `handler_update_reserve_allocation.rs` validates a KLend reserve and collateral mint.
- Add `update_strategy_allocation` instruction and handler that:
  - validates signer roles (vault admin/allocation admin behavior unchanged)
  - validates whitelist (program OR mint)
  - stores generic strategy metadata in allocation slot

### 5) Invest flow (`handler_invest.rs`, `operations/vault_operations.rs`)

- Introduce `strategy_adapter` module with trait-like interface:
  - `refresh`
  - `deposit`
  - `redeem`
  - `position_value_in_quote`
- Keep current KLend implementation as adapter `StrategyKind::KLend`.
- Add adapter selection per allocation.
- Continue to compute target deltas with existing allocation/cap logic.

### 6) Withdraw flow (`handler_withdraw.rs`, `operations/vault_operations.rs`)

- Keep current user behavior: withdraw from chosen allocation when disinvestment is needed.
- Extend withdraw disinvestment path to route through adapter chosen by allocation kind.
- Maintain all post-transfer accounting checks.

### 7) Accounting + oracle

- For same-token strategies (single-asset lending), use adapter-reported on-chain conversion.
- For LP or multi-token strategies:
  - normalize values into vault quote currency using oracle prices,
  - adapter returns position amounts; valuation layer applies oracle conversion.
- Oracle use should be isolated to valuation path only; transfer and share math remains unchanged.

## Safety rollout (minimal-risk sequencing)

1. Add generic whitelist + allocation metadata (no behavior switch).
2. Add strategy adapter interface with KLend implementation only.
3. Switch invest/withdraw dispatch to adapter (still KLend behavior).
4. Add one non-KLend adapter behind feature flag.
5. Enable multi-token accounting with oracle normalization.

## Open implementation details to finalize before coding

- Canonical quote currency for multi-token AUM (e.g., USD fixed-point vs vault-base mint).
- Oracle source and staleness policy.
- Max adapter account fanout per instruction for compute/stack budgeting.
- Whether each allocation stores adapter kind as enum or derives from `strategy_program`.
