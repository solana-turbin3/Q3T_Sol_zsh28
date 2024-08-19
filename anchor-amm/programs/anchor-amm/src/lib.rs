pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EGdJBLJzozGgDtZpnqowyuzEd3ioQYgjSCoW1Tgu7CBC");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>, seed: u64, fee: u16) -> Result<()> {
        ctx.accounts.initialize_config(seed, fee, ctx.bumps)
    }
}
