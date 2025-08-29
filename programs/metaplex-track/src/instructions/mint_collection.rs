use anchor_lang::prelude::*;

use mpl_core::{
    instructions::CreateCollectionV1CpiBuilder,
};

// Account validation struct for collection creation
#[derive(Accounts)]
pub struct MintCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // User creating the collection (pays fees)
    #[account(mut)]
    pub collection: Signer<'info>, // New collection account
    /// CHECK: Collection authority PDA - verified by seeds constraint
    #[account(
        seeds = [b"authority", collection.key().as_ref()],
        bump,
    )]
    pub authority: UncheckedAccount<'info>, // PDA that controls the collection
    pub system_program: Program<'info, System>, // Required for account creation
    /// CHECK: Metaplex Core program - verified by address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>, // Metaplex Core program
}

impl<'info> MintCollection<'info> {
    // Creates a new collection using Metaplex Core
    pub fn mint_core_collection(&mut self, bump: MintCollectionBumps) -> Result<()> {
        // Create authority seeds for PDA signing
        let seeds = &[
            b"authority",
            self.collection.to_account_info().key.as_ref(),
            &[bump.authority],
        ];

        let signer_seeds = &[&seeds[..]];

        // Create collection via Metaplex Core CPI
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .update_authority(Some(&self.authority.to_account_info())) // PDA controls updates
            .payer(&self.user.to_account_info()) // User pays transaction fees
            .system_program(&self.system_program.to_account_info())
            .name("Legder Collection".to_string()) // Collection name
            .uri("https://myasset.com".to_string()) // Collection metadata URI
            .invoke_signed(signer_seeds)?; // Sign with PDA authority
        
        Ok(())
    }
}