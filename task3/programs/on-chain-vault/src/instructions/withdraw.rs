//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------
use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump,
        has_one = vault_authority
    )]
    pub vault: Account<'info, Vault>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_ai = vault.to_account_info();
    let authority_ai = ctx.accounts.vault_authority.to_account_info();

    require!(!vault.locked, VaultError::VaultLocked);

    require!(amount > 0, VaultError::Overflow);

    let vault_lamports = vault_ai.lamports();
    let rent = Rent::get()?;
    let min = rent.minimum_balance(vault_ai.data_len());
    require!(
        vault_lamports >= amount && vault_lamports - amount >= min,
        VaultError::InsufficientBalance
    );

    **vault_ai.try_borrow_mut_lamports()? -= amount;
    **authority_ai.try_borrow_mut_lamports()? += amount;

    emit!(WithdrawEvent {
        vault: vault.key(),
        vault_authority: authority_ai.key(),
        amount,
    });

    Ok(())
}