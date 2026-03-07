use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    state::{NonKlendStrategyState, StrategyType},
    utils::{consts::NON_KLEND_STRATEGY_SEED, token_ops},
    KaminoVaultError, VaultState,
};

pub fn init(ctx: Context<InitNonKlendStrategy>) -> Result<()> {
    let strategy = &mut ctx.accounts.non_klend_strategy;
    strategy.vault = ctx.accounts.vault_state.key();
    strategy.strategy_type = StrategyType::ReportedValue;
    strategy.allocation = 0;
    strategy.last_reported_value = 0;
    strategy.bump = ctx.bumps.non_klend_strategy;
    Ok(())
}

pub fn allocate(ctx: Context<AllocateToNonKlendStrategy>, amount: u64) -> Result<()> {
    require!(amount > 0, KaminoVaultError::DepositAmountsZero);

    let vault = &mut ctx.accounts.vault_state.load_mut()?;
    require_keys_eq!(ctx.accounts.vault_admin_authority.key(), vault.vault_admin_authority);

    token_ops::tokens::transfer_to_token_account(
        &token_ops::tokens::VaultTransferAccounts {
            token_program: ctx.accounts.token_program.to_account_info(),
            token_vault: ctx.accounts.token_vault.to_account_info(),
            token_ata: ctx.accounts.strategy_escrow.to_account_info(),
            token_mint: ctx.accounts.token_mint.to_account_info(),
            base_vault_authority: ctx.accounts.base_vault_authority.to_account_info(),
            vault_state: ctx.accounts.vault_state.to_account_info(),
        },
        vault.base_vault_authority_bump as u8,
        amount,
        vault.token_mint_decimals as u8,
    )?;

    let strategy = &mut ctx.accounts.non_klend_strategy;
    strategy.allocation = strategy.allocation.saturating_add(amount);

    Ok(())
}

pub fn report(ctx: Context<ReportNonKlendStrategyValue>, reported_value: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault_state.load_mut()?;
    let signer = ctx.accounts.reporter.key();
    require!(
        signer == vault.vault_admin_authority || signer == vault.allocation_admin,
        KaminoVaultError::AdminAuthorityIncorrect
    );

    let strategy = &mut ctx.accounts.non_klend_strategy;
    strategy.last_reported_value = reported_value;
    vault.non_klend_reported_value = reported_value;

    Ok(())
}

#[derive(Accounts)]
pub struct InitNonKlendStrategy<'info> {
    #[account(mut)]
    pub vault_admin_authority: Signer<'info>,

    #[account(mut, has_one = vault_admin_authority)]
    pub vault_state: AccountLoader<'info, VaultState>,

    #[account(
        init,
        payer = vault_admin_authority,
        space = NonKlendStrategyState::space(),
        seeds = [NON_KLEND_STRATEGY_SEED, vault_state.key().as_ref()],
        bump,
    )]
    pub non_klend_strategy: Account<'info, NonKlendStrategyState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AllocateToNonKlendStrategy<'info> {
    #[account(mut)]
    pub vault_admin_authority: Signer<'info>,

    #[account(mut,
        has_one = base_vault_authority,
        has_one = token_vault,
        has_one = token_mint,
        has_one = token_program,
    )]
    pub vault_state: AccountLoader<'info, VaultState>,

    /// CHECK: checked against vault state
    pub base_vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut,
        seeds = [NON_KLEND_STRATEGY_SEED, vault_state.key().as_ref()],
        bump = non_klend_strategy.bump,
        constraint = non_klend_strategy.vault == vault_state.key()
    )]
    pub non_klend_strategy: Account<'info, NonKlendStrategyState>,

    #[account(mut,
        token::mint = token_mint,
        token::authority = non_klend_strategy,
        token::token_program = token_program,
    )]
    pub strategy_escrow: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ReportNonKlendStrategyValue<'info> {
    pub reporter: Signer<'info>,

    #[account(mut)]
    pub vault_state: AccountLoader<'info, VaultState>,

    #[account(mut,
        seeds = [NON_KLEND_STRATEGY_SEED, vault_state.key().as_ref()],
        bump = non_klend_strategy.bump,
        constraint = non_klend_strategy.vault == vault_state.key()
    )]
    pub non_klend_strategy: Account<'info, NonKlendStrategyState>,
}
