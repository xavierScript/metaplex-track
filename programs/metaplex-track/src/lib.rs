use anchor_lang::prelude::*;

declare_id!("2X4RWyDKRVvfjgyJFqTBnJQWqaFkScf7VKbBXSRpATf6");

mod instructions;
use instructions::*;

#[program]
pub mod metaplex_track {
    use super::*;

    pub fn create_collection(ctx: Context<MintCollection>) -> Result<()> {
        ctx.accounts.mint_core_collection(ctx.bumps)
    }
    pub fn mint_asset(ctx: Context<MintAsset>, freeze_authority: Option<Pubkey>, burn_authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.mint_core_asset(ctx.bumps, freeze_authority, burn_authority)
    }
    pub fn freeze_asset(ctx: Context<FreezeAsset>) -> Result<()> {
        ctx.accounts.freeze_asset()
    }
    pub fn unfreeze_asset(ctx: Context<FreezeAsset>) -> Result<()> {
        ctx.accounts.unfreeze_asset()
    }
    pub fn burn_asset(ctx: Context<BurnAsset>) -> Result<()> {
        ctx.accounts.burn_asset()
    }
    pub fn transfer_asset(ctx: Context<TransferAsset>) -> Result<()> {
        ctx.accounts.transfer_asset()
    }
}

