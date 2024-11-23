use anchor_lang::prelude::*;

declare_id!("ANdYmC2Z4yKnQeLq4GodFPMcqJGaq9y4DEbeJcBU7PHV");

// Instructions defined in program module
#[program]
pub mod project {
    use super::*;
 
    // Instruction to initialize a new project account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Reference to the project account from the Initialize struct
        let project = &ctx.accounts.project;
        msg!("Project account created! Current count: {}", project.count);
        Ok(())
    }
 
    // Instruction to increment a project account
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Mutable reference to the project account from the Increment struct
        let project = &mut ctx.accounts.project;
        msg!("Previous project: {}", project.count);
 
        // Increment the count value stored on the project account by 1
        project.count = project.count.checked_add(1).unwrap();
        msg!("Project incremented! Current count: {}", project.count);
        Ok(())
    }
}
 
// Accounts required by the initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The account paying to create the project account
    #[account(mut)]
    pub user: Signer<'info>, // specify account must be signer on the transaction
 
    // The project account being created and initialized in the instruction
    #[account(
        init,         // specifies we are creating this account
        payer = user, // specifies account paying for the creation of the account
        space = 8 + 8 // space allocated to the new account (8 byte discriminator + 8 byte for u64)
    )]
    pub project: Account<'info, Project>, // specify account is 'Project' type
    pub system_program: Program<'info, System>, // specify account must be System Program
}
 
// Account required by the increment instruction
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)] // specify account is mutable because we are updating its data
    pub project: Account<'info, Project>, // specify account is 'Project' type
}
 
// Define structure of `Project` account
#[account]
pub struct Project {
    pub count: u64, // define count value type as u64
}