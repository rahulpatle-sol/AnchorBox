use anchor_lang::prelude::*;

declare_id!("555MWcB22ERbPcrMFtTGbMBwpH4cxGJHZNouw7RSfh26");

#[program]
pub mod calc {
    use super::*;

    // Initialize the calculator
    pub fn initialize(ctx: Context<Initialize>, greeting: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greetings = greeting;
        calculator.result = 0;
        calculator.remainder = 0;
        msg!("Calculator initialized with greeting: {}", calculator.greetings);
        Ok(())
    }
    
    // Add two numbers
    pub fn add(ctx: Context<Add>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        msg!("{} + {} = {}", num1, num2, calculator.result);
        Ok(())
    }
    
    // Subtract two numbers
    pub fn sub(ctx: Context<Sub>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        msg!("{} - {} = {}", num1, num2, calculator.result);
        Ok(())
    }
    
    // Multiply two numbers
    pub fn mult(ctx: Context<Mult>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        msg!("{} * {} = {}", num1, num2, calculator.result);
        Ok(())
    }
    
    // Divide two numbers (with error handling!)
    pub fn div(ctx: Context<Div>, num1: i64, num2: i64) -> Result<()> {
        // Check for division by zero
        if num2 == 0 {
            return err!(CalculatorError::DivisionByZero);
        }
        
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        msg!("{} / {} = {} (remainder: {})", num1, num2, calculator.result, calculator.remainder);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8  // discriminator + String + result + remainder
    )]
    pub calculator: Account<'info, Calculator>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Mult<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Div<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greetings: String,
    pub result: i64,
    pub remainder: i64,
}

#[error_code]
pub enum CalculatorError {
    #[msg("Cannot divide by zero!")]
    DivisionByZero,
}