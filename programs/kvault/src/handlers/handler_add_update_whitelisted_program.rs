use anchor_lang::prelude::*;

use crate::{
    operations::strategy_whitelist_operations::{self, UpdateStrategyWhitelistMode},
    utils::consts::{
        GLOBAL_CONFIG_STATE_SEEDS, PROGRAM_WHITELIST_ENTRY_SIZE, WHITELISTED_PROGRAMS_SEED,
    },
    GlobalConfig, ProgramWhitelistEntry,
};

pub fn process(
    ctx: Context<AddUpdateWhitelistedProgram>,
    update: UpdateStrategyWhitelistMode,
) -> Result<()> {
    strategy_whitelist_operations::update_program_whitelist_entry(
        &mut ctx.accounts.program_whitelist_entry,
        &ctx.accounts.program_to_whitelist.key(),
        update,
    )
}

#[derive(Accounts)]
pub struct AddUpdateWhitelistedProgram<'info> {
    #[account(mut)]
    pub global_admin: Signer<'info>,

    #[account(
        seeds = [GLOBAL_CONFIG_STATE_SEEDS],
        bump,
        has_one = global_admin
    )]
    pub global_config: AccountLoader<'info, GlobalConfig>,

    /// CHECK: any strategy program pubkey can be whitelisted
    pub program_to_whitelist: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = global_admin,
        space = 8 + PROGRAM_WHITELIST_ENTRY_SIZE,
        seeds = [WHITELISTED_PROGRAMS_SEED, program_to_whitelist.key().as_ref()],
        bump
    )]
    pub program_whitelist_entry: Account<'info, ProgramWhitelistEntry>,

    pub system_program: Program<'info, System>,
}
