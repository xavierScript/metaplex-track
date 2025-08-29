use anchor_lang::prelude::*;

// Program ID for the Metaplex Track NFT program
declare_id!("2X4RWyDKRVvfjgyJFqTBnJQWqaFkScf7VKbBXSRpATf6");

// Import instruction handlers
mod instructions;
use instructions::*;

// Main program module for NFT collection and asset management
#[program]
pub mod metaplex_track {
    use super::*;

    // Creates a new NFT collection using Metaplex Core
    pub fn create_collection(ctx: Context<MintCollection>) -> Result<()> {
        ctx.accounts.mint_core_collection(ctx.bumps)
    }
    
    // Mints a new asset in a collection with optional freeze/burn authorities
    pub fn mint_asset(ctx: Context<MintAsset>, freeze_authority: Option<Pubkey>, burn_authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.mint_core_asset(ctx.bumps, freeze_authority, burn_authority)
    }
    
    // Freezes an asset to prevent transfers
    pub fn freeze_asset(ctx: Context<FreezeAsset>) -> Result<()> {
        ctx.accounts.freeze_asset()
    }
    
    // Unfreezes an asset to allow transfers
    pub fn unfreeze_asset(ctx: Context<FreezeAsset>) -> Result<()> {
        ctx.accounts.unfreeze_asset()
    }
    
    // Permanently burns an asset
    pub fn burn_asset(ctx: Context<BurnAsset>) -> Result<()> {
        ctx.accounts.burn_asset()
    }
    
    // Transfers asset ownership to a new owner
    pub fn transfer_asset(ctx: Context<TransferAsset>) -> Result<()> {
        ctx.accounts.transfer_asset()
    }
}

