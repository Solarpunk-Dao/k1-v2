use anchor_lang::prelude::*;

use crate::{KaminoVaultError, MintWhitelistEntry, ProgramWhitelistEntry};

#[derive(Clone, Copy, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum UpdateStrategyWhitelistMode {
    Invest(u8),
    AddAllocation(u8),
}

fn check_bool_like_value(value: u8) -> Result<()> {
    if value > 1 {
        msg!("Invalid value passed in, should be 0 or 1, got {value}");
        return Err(KaminoVaultError::InvalidBoolLikeValue.into());
    }
    Ok(())
}

pub fn update_program_whitelist_entry(
    program_whitelist_entry: &mut ProgramWhitelistEntry,
    program_id: &Pubkey,
    update: UpdateStrategyWhitelistMode,
) -> Result<()> {
    program_whitelist_entry.program_id = *program_id;

    match update {
        UpdateStrategyWhitelistMode::Invest(value) => {
            check_bool_like_value(value)?;
            program_whitelist_entry.whitelist_invest = value;
        }
        UpdateStrategyWhitelistMode::AddAllocation(value) => {
            check_bool_like_value(value)?;
            program_whitelist_entry.whitelist_add_allocation = value;
        }
    }

    Ok(())
}

pub fn update_mint_whitelist_entry(
    mint_whitelist_entry: &mut MintWhitelistEntry,
    mint: &Pubkey,
    update: UpdateStrategyWhitelistMode,
) -> Result<()> {
    mint_whitelist_entry.mint = *mint;

    match update {
        UpdateStrategyWhitelistMode::Invest(value) => {
            check_bool_like_value(value)?;
            mint_whitelist_entry.whitelist_invest = value;
        }
        UpdateStrategyWhitelistMode::AddAllocation(value) => {
            check_bool_like_value(value)?;
            mint_whitelist_entry.whitelist_add_allocation = value;
        }
    }

    Ok(())
}

pub fn check_any_whitelist(
    program_whitelist_entry: Option<&ProgramWhitelistEntry>,
    mint_whitelist_entry: Option<&MintWhitelistEntry>,
    is_invest: bool,
) -> Result<()> {
    let program_allowed = match program_whitelist_entry {
        Some(e) => {
            if is_invest {
                e.is_invest_whitelisted()
            } else {
                e.is_add_allocation_whitelisted()
            }
        }
        None => false,
    };

    let mint_allowed = match mint_whitelist_entry {
        Some(e) => {
            if is_invest {
                e.is_invest_whitelisted()
            } else {
                e.is_add_allocation_whitelisted()
            }
        }
        None => false,
    };

    require!(program_allowed || mint_allowed, KaminoVaultError::ReserveNotWhitelisted);
    Ok(())
}
