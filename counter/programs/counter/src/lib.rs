use anchor_lang::prelude::*;

declare_id!("Gi4QzuvpRNgRBzfHLjY2VxgtbMuCjeR6cPaQWWRvFe96");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter initialized");
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;  // ← Added mut
        counter.count += 1;
        msg!("Counter incremented to: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;  // ← Added mut
        counter.count -= 1;
        msg!("Counter decremented to: {}", counter.count);
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;  // ← Added mut
        counter.count = 0;
        msg!("Counter reset to 0");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,  // ← Fixed: was 'signer'
        space = 8 + 8
    )]
    pub counter: Account<'info, Counter>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]  // ← Added mut
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]  // ← Added mut
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut)]  // ← Added mut
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
}