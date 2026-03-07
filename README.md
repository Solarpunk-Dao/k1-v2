# Kvault - Kamino Lending Vault Program

Kamino vault is an open source smart contract allowing to earn yield by lending on Kamino Lending (KLend) program.

Admins of a vault define investment targets on different reserves of the same asset. User can deposit in a vault in exchange of a share of the vault.
Kvault deploy these deposits to the KLend reserves according to the configured targets.

## Deployments

Kvault is deployed using the following Program Ids:

* Mainnet: `KvauGMspG5k6rtzrqqn7WNn3oZdyKqLKwK2XWQ8FLjd`
* Staging (Mainnet): `stKvQfwRsQiKnLtMNVLHKS3exFJmZFsgfzBPWHECUYK`

## Governance / Devnet notes

- Vault authority should be configured to a multisig address (set as `vault_admin_authority`).
- Fees are charged on every deposit and withdraw (management + performance), accrue to `pending_fees`, and can be withdrawn by `fee_recipient` using `withdraw_pending_fees`.
- Deposits and withdrawals can be paused via vault config (`PauseDeposits`, `PauseWithdrawals`).
- Optional off-chain NAV can be provided via vault config (`OffchainNav`) and is included in total assets.
- Non-KLend reported-value strategy is supported with:
  - `init_non_klend_strategy`
  - `allocate_to_non_klend_strategy`
  - `report_non_klend_strategy_value`
- Devnet setup checklist:
  - reserves + lending market accounts for whitelist
  - whitelist entries for target reserves
  - fee recipient account
  - multisig vault admin authority
