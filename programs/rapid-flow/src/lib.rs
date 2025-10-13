use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

declare_id!("5hZHJEeFN45bz4xYu1A2RHwtfapTHiq1j8yroVnvjPtQ");

pub mod state;
pub use state::*;

#[program]
pub mod rapid_flow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
