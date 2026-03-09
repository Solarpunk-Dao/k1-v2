use anchor_lang::prelude::*;
use kamino_lending::fraction::Fraction;

use crate::{
    operations::non_klend_strategy_operations,
    utils::consts::NON_KLEND_STRATEGY_SEED,
    NonKlendStrategyState, VaultState,
};

pub fn process(
    ctx: Context<ReportNonKlendStrategyValue>,
    strategy_id: Pubkey,
    last_reported_value_sf: u128,
) -> Result<()> {
    let report_value = Fraction::from_bits(last_reported_value_sf);
    let ts = u64::try_from(Clock::get()?.unix_timestamp).unwrap();

    // Ensure strategy belongs to vault + strategy id pair used in PDA derivation.
    require_keys_eq!(ctx.accounts.non_klend_strategy.strategy_id, strategy_id);
    require_keys_eq!(
        ctx.accounts.non_klend_strategy.vault,
        ctx.accounts.vault_state.key()
    );

    non_klend_strategy_operations::report_strategy_value(
        &mut ctx.accounts.non_klend_strategy,
        ctx.accounts.reporter.key(),
        report_value,
        ts,
    )
}

#[derive(Accounts)]
#[instruction(strategy_id: Pubkey)]
pub struct ReportNonKlendStrategyValue<'info> {
    pub reporter: Signer<'info>,

    pub vault_state: AccountLoader<'info, VaultState>,

    #[account(
        mut,
        seeds = [NON_KLEND_STRATEGY_SEED, vault_state.key().as_ref(), strategy_id.as_ref()],
        bump,
    )]
    pub non_klend_strategy: Account<'info, NonKlendStrategyState>,
}
