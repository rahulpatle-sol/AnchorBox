use anchor_lang::prelude::*;

declare_id!("GwAEy23yyYED4UtRsm1i9ANXZWBEeiFdJL8SpvQ2vmHY");

#[program]
pub mod bank {
    use super::*;
// creating a bank  account and initializing it with 0 balance
    pub fn initialize(ctx: Context<Bank_account>, account_number: u64, account_type: String) -> Result<()> {
        
        ctx.accounts.bank_account.balance = 0;
        ctx.accounts.bank_account.owner = *ctx.accounts.user.key;
        ctx.accounts.bank_account.account_number = account_number;
        ctx.accounts.bank_account.account_type = account_type;
        Ok(())
    }

    //  deposit funds into the bank account
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
            msg!("Depositing {} lamports into the bank account", amount);
            ctx.accounts.bank_account.balance += amount;
            Ok(())
    }
// withdraw funds from the bank account
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        msg!("Withdrawing {} lamports from the bank account", amount);
        ctx.accounts.bank_account.balance -= amount;
        Ok(())
    }

    // check the balance of the bank account
    pub fn check_balance(ctx: Context<CheckBalance>) -> Result<u64> {
        let balance = ctx.accounts.bank_account.balance;
        msg!("The current balance of the bank account is: {} lamports", balance);
        Ok(balance)
}}

#[derive(Accounts)]
pub struct Bank_account<'info> {
#[account(init, payer = user, space = 8 + 32 + 8 + 32 + 8 + 32 + 8)]
    pub bank_account: Account<'info, BankAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,



}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub bank_account: Account<'info, BankAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub bank_account: Account<'info, BankAccount>,  
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckBalance<'info> {
    pub bank_account: Account<'info, BankAccount>,
}
#[account]
#[derive(Default)]
pub struct BankAccount {
    pub balance: u64,
    pub owner: Pubkey,
    pub account_number: u64,
    pub account_type: String,

    

    
}
#[error_code]
pub enum BankError {
    #[msg("Insufficient funds in the bank account")]
    InsufficientFunds,
    #[msg("Unauthorized access to the bank account")]
    UnauthorizedAccess,
    #[msg("Invalid account type")]
    InvalidAccountType,
    #[msg("Account already exists")]
    AccountAlreadyExists,
    #[msg("Account does not exist")]
    AccountDoesNotExist,
}