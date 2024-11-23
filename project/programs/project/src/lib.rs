use anchor_lang::prelude::*;

declare_id!("ANdYmC2Z4yKnQeLq4GodFPMcqJGaq9y4DEbeJcBU7PHV");

#[program]
pub mod project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn login(ctx: Context<Initialize>) -> Result<()> { //scan QR code
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn logout(ctx: Context<Initialize>) -> Result<()> { //leave venue and earn points/SOL
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn read(ctx: Context<Initialize>) -> Result<()> { //read and earn point/SOL
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn write(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // The account paying to create the wwh account
    #[account(mut)]
    pub user: Signer<'info>, // specify account must be signer on the transaction
 
    // The wwh account being created and initialized in the instruction
    #[account(
        init,         // specifies we are creating this account
        payer = user, // specifies account paying for the creation of the account
        space = 8 + 8 // space allocated to the new account (8 byte discriminator + 8 byte for u64)
    )]
    pub wwh: Account<'info, wwh>, // specify account is 'wwh' type
    pub system_program: Program<'info, System>, // specify account must be System Program
}

// Account required by the increment instruction
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)] // specify account is mutable because we are updating its data
    pub wwh: Account<'info, wwh>, // specify account is 'wwh' type
}
 
// Define structure of `wwh` account
#[account]
pub struct wwh {
    pub count: u64, // define count value type as u64
}
