use anchor_lang::prelude::*;

declare_id!(""); // ← Pega aquí tu Program ID

#[program]
pub mod memories {
    use super::*;

    // ── CREATE ────────────────────────────────────────────────
    pub fn create_memory(
        ctx: Context<CreateMemory>,
        id: u64,
        title: String,
        content: String,
    ) -> Result<()> {
        require!(!title.is_empty(),            MemoryError::EmptyTitle);
        require!(title.len()   <= 100,         MemoryError::TitleTooLong);
        require!(content.len() <= 1000,        MemoryError::ContentTooLong);

        let memory    = &mut ctx.accounts.memory;
        let clock     = Clock::get()?;

        memory.owner      = ctx.accounts.user.key();
        memory.id         = id;
        memory.title      = title;
        memory.content    = content;
        memory.created_at = clock.unix_timestamp;
        memory.updated_at = clock.unix_timestamp;
        memory.bump       = ctx.bumps.memory;

        msg!("📝 Nota creada — id: {} | título: {}", memory.id, memory.title);
        Ok(())
    }

    // ── UPDATE ────────────────────────────────────────────────
    pub fn update_memory(
        ctx: Context<UpdateMemory>,
        _id: u64,
        title: String,
        content: String,
    ) -> Result<()> {
        require!(!title.is_empty(),            MemoryError::EmptyTitle);
        require!(title.len()   <= 100,         MemoryError::TitleTooLong);
        require!(content.len() <= 1000,        MemoryError::ContentTooLong);

        let memory    = &mut ctx.accounts.memory;
        let clock     = Clock::get()?;

        memory.title      = title;
        memory.content    = content;
        memory.updated_at = clock.unix_timestamp;

        msg!("✏️ Nota actualizada — id: {}", memory.id);
        Ok(())
    }

    // ── DELETE ────────────────────────────────────────────────
    pub fn delete_memory(ctx: Context<DeleteMemory>, _id: u64) -> Result<()> {
        msg!("🗑️ Nota eliminada — id: {}", ctx.accounts.memory.id);
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────
//  CUENTA
// ─────────────────────────────────────────────────────────────

#[account]
pub struct Memory {
    pub owner:      Pubkey,   // 32
    pub id:         u64,      //  8
    pub created_at: i64,      //  8
    pub updated_at: i64,      //  8
    pub bump:       u8,       //  1
    pub title:      String,   //  4 + 100
    pub content:    String,   //  4 + 1000
}

impl Memory {
    // 8 (disc) + 32 + 8 + 8 + 8 + 1 + (4+100) + (4+1000) = 1173
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 1 + 4 + 100 + 4 + 1000;
}

// ─────────────────────────────────────────────────────────────
//  CONTEXTOS
// ─────────────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateMemory<'info> {
    #[account(
        init,
        payer = user,
        space = Memory::LEN,
        seeds = [b"memory", user.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub memory: Account<'info, Memory>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct UpdateMemory<'info> {
    #[account(
        mut,
        has_one = owner @ MemoryError::Unauthorized,
        seeds   = [b"memory", user.key().as_ref(), &id.to_le_bytes()],
        bump    = memory.bump,
    )]
    pub memory: Account<'info, Memory>,

    /// CHECK: validado por has_one
    pub owner: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct DeleteMemory<'info> {
    #[account(
        mut,
        close = user,
        has_one = owner @ MemoryError::Unauthorized,
        seeds = [b"memory", user.key().as_ref(), &id.to_le_bytes()],
        bump  = memory.bump,
    )]
    pub memory: Account<'info, Memory>,

    /// CHECK: validado por has_one
    pub owner: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
}

// ─────────────────────────────────────────────────────────────
//  ERRORES
// ─────────────────────────────────────────────────────────────

#[error_code]
pub enum MemoryError {
    #[msg("No eres el propietario de esta nota.")]
    Unauthorized,
    #[msg("El título no puede estar vacío.")]
    EmptyTitle,
    #[msg("El título supera los 100 caracteres.")]
    TitleTooLong,
    #[msg("El contenido supera los 1000 caracteres.")]
    ContentTooLong,
}

