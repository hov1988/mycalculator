use anchor_lang::prelude::*;

declare_id!("9um3x7pgK1XQvzDjyhEoaeAJSVRtRm4C5SHFZmt17yMU");

#[program]
pub mod mycalculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
