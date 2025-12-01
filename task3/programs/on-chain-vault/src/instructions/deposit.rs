//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let user_ai = ctx.accounts.user.to_account_info();
    let vault = &ctx.accounts.vault;
    let vault_ai = vault.to_account_info();
    require!(!vault.locked, VaultError::VaultLocked);
    require!(amount > 0, VaultError::Overflow);
    let user_lamports = user_ai.lamports();
    require!(user_lamports >= amount, VaultError::InsufficientBalance);
    let ix = system_instruction::transfer(&user_ai.key(), &vault_ai.key(), amount);
    invoke(
        &ix,
        &[
            user_ai.clone(),
            vault_ai.clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    emit!(DepositEvent {
        amount,
        user: user_ai.key(),
        vault: vault_ai.key(),
    });

    Ok(())
}