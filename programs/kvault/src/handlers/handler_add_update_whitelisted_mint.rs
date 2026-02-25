use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{
    operations::strategy_whitelist_operations::{self, UpdateStrategyWhitelistMode},
    utils::consts::{GLOBAL_CONFIG_STATE_SEEDS, MINT_WHITELIST_ENTRY_SIZE, WHITELISTED_MINTS_SEED},
    GlobalConfig, MintWhitelistEntry,
};

pub fn process(
    ctx: Context<AddUpdateWhitelistedMint>,
    update: UpdateStrategyWhitelistMode,
) -> Result<()> {
    strategy_whitelist_operations::update_mint_whitelist_entry(
        &mut ctx.accounts.mint_whitelist_entry,
        &ctx.accounts.mint_to_whitelist.key(),
        update,
    )
}

#[derive(Accounts)]
pub struct AddUpdateWhitelistedMint<'info> {
    #[account(mut)]
    pub global_admin: Signer<'info>,

    #[account(
        seeds = [GLOBAL_CONFIG_STATE_SEEDS],
        bump,
        has_one = global_admin
    )]
    pub global_config: AccountLoader<'info, GlobalConfig>,

    pub mint_to_whitelist: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = global_admin,
        space = 8 + MINT_WHITELIST_ENTRY_SIZE,
        seeds = [WHITELISTED_MINTS_SEED, mint_to_whitelist.key().as_ref()],
        bump
    )]
    pub mint_whitelist_entry: Account<'info, MintWhitelistEntry>,

    pub system_program: Program<'info, System>,
}
