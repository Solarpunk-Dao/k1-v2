use anchor_lang::{err, prelude::*, Result};
use kamino_lending::fraction::Fraction;

use crate::{NonKlendStrategyState, StrategyType};
use crate::KaminoVaultError;

pub fn init_reported_value_strategy(
    strategy: &mut NonKlendStrategyState,
    vault: Pubkey,
    strategy_id: Pubkey,
    escrow_token_account: Pubkey,
    reporter_authority: Pubkey,
) -> Result<()> {
    strategy.vault = vault;
    strategy.strategy_id = strategy_id;
    strategy.escrow_token_account = escrow_token_account;
    strategy.reporter_authority = reporter_authority;
    strategy.strategy_type = StrategyType::ReportedValue as u8;
    strategy.enabled = 1;
    strategy.set_last_reported_value(Fraction::ZERO);
    strategy.last_report_ts = 0;
    strategy.validate()?;

    Ok(())
}

pub fn report_strategy_value(
    strategy: &mut NonKlendStrategyState,
    signer: Pubkey,
    report_value: Fraction,
    report_ts: u64,
) -> Result<()> {
    require!(
        strategy.enabled == 1,
        KaminoVaultError::NonKlendStrategyDisabled
    );

    require!(
        strategy.strategy_type()? == StrategyType::ReportedValue,
        KaminoVaultError::InvalidNonKlendStrategyType
    );

    if signer != strategy.reporter_authority {
        return err!(KaminoVaultError::InvalidNonKlendStrategyReporter);
    }

    strategy.set_last_reported_value(report_value);
    strategy.last_report_ts = report_ts;

    Ok(())
}

pub fn aggregate_non_klend_value_for_vault(vault: &Pubkey, accounts: &[AccountInfo<'_>]) -> Fraction {
    let mut total = Fraction::ZERO;

    for account_info in accounts {
        if account_info.owner != &crate::ID {
            continue;
        }

        if let Ok(data) = account_info.try_borrow_data() {
            let mut data_slice: &[u8] = &data;
            if let Ok(strategy) = NonKlendStrategyState::try_deserialize(&mut data_slice) {
                if strategy.vault == *vault
                    && strategy.enabled == 1
                    && strategy.strategy_type().ok() == Some(StrategyType::ReportedValue)
                {
                    total += strategy.get_last_reported_value();
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_report_strategy_value_success() {
        let signer = Pubkey::new_unique();
        let mut strategy = NonKlendStrategyState::default();
        strategy.enabled = 1;
        strategy.strategy_type = StrategyType::ReportedValue as u8;
        strategy.reporter_authority = signer;

        let reported = Fraction::from(123_u64);
        report_strategy_value(&mut strategy, signer, reported, 50).unwrap();

        assert_eq!(strategy.get_last_reported_value(), reported);
        assert_eq!(strategy.last_report_ts, 50);
    }

    #[test]
    fn test_report_strategy_value_wrong_reporter_fails() {
        let signer = Pubkey::new_unique();
        let mut strategy = NonKlendStrategyState::default();
        strategy.enabled = 1;
        strategy.strategy_type = StrategyType::ReportedValue as u8;
        strategy.reporter_authority = Pubkey::new_unique();

        let err = report_strategy_value(&mut strategy, signer, Fraction::from(1_u64), 50)
            .unwrap_err();
        assert_eq!(
            err,
            error!(KaminoVaultError::InvalidNonKlendStrategyReporter)
        );
    }

    #[test]
    fn test_report_strategy_value_wrong_type_fails() {
        let signer = Pubkey::new_unique();
        let mut strategy = NonKlendStrategyState::default();
        strategy.enabled = 1;
        strategy.strategy_type = StrategyType::KLend as u8;
        strategy.reporter_authority = signer;

        let err = report_strategy_value(&mut strategy, signer, Fraction::from(1_u64), 50)
            .unwrap_err();
        assert_eq!(err, error!(KaminoVaultError::InvalidNonKlendStrategyType));
    }

    #[test]
    fn test_report_strategy_value_disabled_fails() {
        let signer = Pubkey::new_unique();
        let mut strategy = NonKlendStrategyState::default();
        strategy.enabled = 0;
        strategy.strategy_type = StrategyType::ReportedValue as u8;
        strategy.reporter_authority = signer;

        let err = report_strategy_value(&mut strategy, signer, Fraction::from(1_u64), 50)
            .unwrap_err();
        assert_eq!(err, error!(KaminoVaultError::NonKlendStrategyDisabled));
    }
}
