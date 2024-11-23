use anchor_lang::prelude::*;

declare_id!("ANdYmC2Z4yKnQeLq4GodFPMcqJGaq9y4DEbeJcBU7PHV");

#[program]
pub mod project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
