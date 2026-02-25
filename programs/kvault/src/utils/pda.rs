use anchor_lang::prelude::Pubkey;

use crate::utils::consts::{
    GLOBAL_CONFIG_STATE_SEEDS, WHITELISTED_MINTS_SEED, WHITELISTED_PROGRAMS_SEED,
};

pub fn program_data() -> Pubkey {
    program_data_program_id(&crate::ID)
}

pub fn program_data_program_id(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[program_id.as_ref()],
        &solana_program::bpf_loader_upgradeable::ID,
    )
    .0
}

pub fn global_config() -> Pubkey {
    global_config_program_id(&crate::ID)
}

pub fn global_config_program_id(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[GLOBAL_CONFIG_STATE_SEEDS], program_id).0
}


pub fn whitelisted_program(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[WHITELISTED_PROGRAMS_SEED, program_id.as_ref()], &crate::ID).0
}

pub fn whitelisted_mint(mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[WHITELISTED_MINTS_SEED, mint.as_ref()], &crate::ID).0
}
