use anchor_lang::prelude::*;

use state::*;
mod state;

use constants::*;
mod constants;

use instructions::*;
mod instructions;





declare_id!("Ag8Menp4ryy4pVRDMnEv9fY8ht4rnzQ9R51ouV4VU6JK");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx : Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx : Context<UpdateConfig>, min_health_factor : u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
}


