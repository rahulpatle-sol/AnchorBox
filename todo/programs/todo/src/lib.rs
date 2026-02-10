use anchor_lang::prelude::*;

declare_id!("9jj5aKahphPa4GcoJN47CFLnERiGkyYQCqvSbyvgXod1");

#[program]
pub mod todo {
    use super::*;

    pub fn create_todo(
        ctx: Context<CreateTodo>,
        text: String,
        id: u64
    ) -> Result<()> {
        let todo = &mut ctx.accounts.todo;
        todo.title = text;
        todo.is_done = false;
        todo.owner = ctx.accounts.user.key();
        todo.id = id;
        
        msg!("Todo created! ID: {}, Title: {}", id, todo.title);
        Ok(())
    }

    pub fn update_todo(
        ctx: Context<UpdateTodo>,
        new_text: String
    ) -> Result<()> {
        let todo = &mut ctx.accounts.todo;
        
        require!(
            todo.owner == ctx.accounts.user.key(),
            TodoError::Unauthorized
        );
        
        todo.title = new_text;
        msg!("Todo updated! New title: {}", todo.title);
        Ok(())
    }

    pub fn mark_todo_as_done(ctx: Context<MarkTodoAsDone>) -> Result<()> {
        let todo = &mut ctx.accounts.todo;
        
        require!(
            todo.owner == ctx.accounts.user.key(),
            TodoError::Unauthorized
        );
        
        todo.is_done = !todo.is_done;  // Toggle!
        msg!("Todo toggled! Is done: {}", todo.is_done);
        Ok(())
    }

    pub fn delete_todo(_ctx: Context<DeleteTodo>) -> Result<()> {
        msg!("Todo deleted and account closed!");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(text: String, id: u64)]
pub struct CreateTodo<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + (4 + 200) + 1 + 32 + 8,
        seeds = [b"todo", user.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub todo: Account<'info, Todo>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTodo<'info> {
    #[account(mut)]
    pub todo: Account<'info, Todo>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct MarkTodoAsDone<'info> {
    #[account(mut)]
    pub todo: Account<'info, Todo>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTodo<'info> {
    #[account(
        mut,
        close = user,
        constraint = todo.owner == user.key() @ TodoError::Unauthorized
    )]
    pub todo: Account<'info, Todo>,
    
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Todo {
    pub title: String,
    pub is_done: bool,
    pub owner: Pubkey,
    pub id: u64,
}

#[error_code]
pub enum TodoError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
}