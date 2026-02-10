use anchor_lang::prelude::*;

declare_id!("AyDBHnbTwc5P1JVTu5C2ueWthcWf48wgnHvtP1nDpMq4");

#[program]
pub mod vault {
    use super::*;
 // initialize the vault account

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let  vault_account = &mut ctx.accounts.vault_account;
        vault_account.balance = 0;
        vault_account.owner = *ctx.accounts.user.key;
        Ok(())
    }
//  deposit funds into the vault account
    pub fn  deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
//  deposi
let vault_account = &mut ctx.accounts.vault_account;
vault_account.balance += amount;
msg!("Deposited {} lamports into the vault account", amount);
        Ok(())
    }

// withdraw funds from the vault account
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
   //  widrwawing the funds from the vault account
   let  vault_account = &mut ctx.accounts.vault_account;
   vault_account.balance -= amount;
   msg!("Withdrew {} lamports from the vault account", amount);
        Ok(())
}
//  close the vault account
    pub fn close(ctx: Context<Close>) -> Result<()> {
       //  clsoing the vault account
         let vault_account = &mut ctx.accounts.vault_account;
            vault_account.balance = 0;
            msg!("Closed the vault account");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize <'info> {

    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]

#[derive(Default)]
pub struct VaultAccount {
    pub balance: u64,
    pub owner: Pubkey,

}
#[error_code]
pub enum VaultError {
    #[msg("Insufficient funds in the vault account")]
    InsufficientFunds,

}