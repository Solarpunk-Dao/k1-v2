use anchor_lang::prelude::*;

use crate::{
    operations::non_klend_strategy_operations,
    utils::consts::{NON_KLEND_STRATEGY_SEED, NON_KLEND_STRATEGY_STATE_SIZE},
    NonKlendStrategyState, VaultState,
};

pub fn process(ctx: Context<InitNonKlendStrategy>, strategy_id: Pubkey) -> Result<()> {
    let vault = ctx.accounts.vault_state.load()?;

    let strategy = &mut ctx.accounts.non_klend_strategy;
    non_klend_strategy_operations::init_reported_value_strategy(
        strategy,
        ctx.accounts.vault_state.key(),
        strategy_id,
        ctx.accounts.strategy_escrow.key(),
        vault.vault_admin_authority,
    )
}

#[derive(Accounts)]
#[instruction(strategy_id: Pubkey)]
pub struct InitNonKlendStrategy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub vault_admin_authority: Signer<'info>,

    #[account(mut,
        has_one = vault_admin_authority,
    )]
    pub vault_state: AccountLoader<'info, VaultState>,

    #[account(
        init,
        payer = payer,
        space = 8 + NON_KLEND_STRATEGY_STATE_SIZE,
        seeds = [NON_KLEND_STRATEGY_SEED, vault_state.key().as_ref(), strategy_id.as_ref()],
        bump,
    )]
    pub non_klend_strategy: Account<'info, NonKlendStrategyState>,

    /// CHECK: set during initialization and used by offchain strategy orchestration
    pub strategy_escrow: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
