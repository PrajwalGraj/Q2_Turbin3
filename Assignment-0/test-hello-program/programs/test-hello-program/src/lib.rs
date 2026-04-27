use anchor_lang::prelude::*;

declare_id!("4pRyZaAp1aCb7sUCj933kVMNuC1WmKgZiVjPt9gBt1xW");

#[program]
pub mod test_hello_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
