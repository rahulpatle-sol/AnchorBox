use anchor_lang::prelude::*;
// import wvwrything from the anchor_lang prelude


declare_id!("BtJVD9nq7pW3d624oN6cQXWrLnLvm3h2iGqxn3KSehJC");
// declare the program id, this is the address of the program on the Solana blockchain


#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello  Solana this is my  hello world program : {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
