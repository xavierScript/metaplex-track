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
    pub fn mint_asset(ctx: Context<MintAsset>) -> Result<()> {
        ctx.accounts.mint_core_asset(ctx.bumps)
    }
}

