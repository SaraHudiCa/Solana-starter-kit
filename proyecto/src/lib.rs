use anchor_lang::prelude::*;

declare_id!("GaDGYKxompZHDWQ45bAjKLFdYmFtn4JCq34RwGVzPzX8") 
        -> Result<()> {

        let profile = &mut context.accounts.profile;

        profile.owner = context.accounts.user.key();
        profile.memory_count = 0;

        Ok(())
    }

    // CREATE MEMORY

    pub fn create_memory(
        context: Context<CreateMemory>,
        title: String,
        description: String,
    ) -> Result<()> {

        let profile = &mut context.accounts.profile;
        let memory = &mut context.accounts.memory;
        let clock = Clock::get()?;

        memory.owner = context.accounts.user.key();
        memory.memory_id = profile.memory_count;

        memory.title = title;
        memory.description = description;

        memory.created_at = clock.unix_timestamp;
        memory.updated_at = clock.unix_timestamp;

        profile.memory_count += 1;

        Ok(())
    }

    // UPDATE MEMORY

    pub fn update_memory(
        context: Context<UpdateMemory>,
        title: String,
        description: String,
    ) -> Result<()> {

        let memory = &mut context.accounts.memory;
        let clock = Clock::get()?;

        require!(
            memory.owner == context.accounts.user.key(),
            ErrorCode::Unauthorized
        );

        memory.title = title;
        memory.description = description;

        memory.updated_at = clock.unix_timestamp;

        Ok(())
    }

    // DELETE MEMORY

    pub fn delete_memory(
        context: Context<DeleteMemory>,
    ) -> Result<()> {

        let memory = &context.accounts.memory;

        require!(
            memory.owner == context.accounts.user.key(),
            ErrorCode::Unauthorized
        );

        Ok(())
    }

// PROFILE ACCOUNT
#[account]
pub struct Profile {

    pub owner: Pubkey,
    pub memory_count: u64,
}

impl Profile {

    pub const SIZE: usize =
        32 + // owner
        8;   // counter
}

// MEMORY ACCOUNT

#[account]
pub struct Memory {

    pub owner: Pubkey,
    pub memory_id: u64,

    pub title: String,
    pub description: String,

    pub created_at: i64,
    pub updated_at: i64,
}
// TAMAÑO MEMORY
impl Memory {

    pub const MAX_TITLE: usize = 50;
    pub const MAX_DESCRIPTION: usize = 500;

    pub const SIZE: usize =
        32 + // owner
        8 +  // id

        4 + Self::MAX_TITLE +
        4 + Self::MAX_DESCRIPTION +

        8 + // created
        8;  // updated
}
// CREATE PROFILE

#[derive(Accounts)]
pub struct CreateProfile<'info> {

    #[account(
        init,
        payer = user,
        space = 8 + Profile::SIZE,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
// CREATE MEMORY

#[derive(Accounts)]
pub struct CreateMemory<'info> {

    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        init,
        payer = user,
        space = 8 + Memory::SIZE,
        seeds = [
            b"memory",
            user.key().as_ref(),
            &profile.memory_count.to_le_bytes()
        ],
        bump
    )]
    pub memory: Account<'info, Memory>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
// UPDATE MEMORY

#[derive(Accounts)]
pub struct UpdateMemory<'info> {

    #[account(mut)]
    pub memory: Account<'info, Memory>,

    pub user: Signer<'info>,
}

// DELETE MEMORY

#[derive(Accounts)]
pub struct DeleteMemory<'info> {

    #[account(
        mut,
        close = user
    )]
    pub memory: Account<'info, Memory>,

    #[account(mut)]
    pub user: Signer<'info>,
}

//ERRORS

#[error_code]
pub enum ErrorCode {

    #[msg("Unauthorized")]
    Unauthorized,
}
