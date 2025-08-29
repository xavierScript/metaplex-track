use anchor_lang::prelude::*;
use mpl_core::{
    instructions::TransferV1CpiBuilder,
};

// Account validation struct for asset transfers
#[derive(Accounts)]
pub struct TransferAsset<'info> {
    #[account(mut)]
    pub current_owner: Signer<'info>, // Current owner transferring the asset
    /// CHECK: New owner receiving the asset - validated by Metaplex Core
    pub new_owner: UncheckedAccount<'info>, // Recipient of the asset
    /// CHECK: Asset being transferred - validated by Metaplex Core
    #[account(mut)]
    pub asset: UncheckedAccount<'info>, // Target asset for transfer
    /// CHECK: Collection account - optional, validated by Metaplex Core
    pub collection: Option<UncheckedAccount<'info>>, // Parent collection (optional)
    pub system_program: Program<'info, System>, // Required for account updates
    /// CHECK: Metaplex Core program - verified by address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>, // Metaplex Core program
}

impl<'info> TransferAsset<'info> {
    // Transfers asset ownership to a new owner (fails if asset is frozen)
    pub fn transfer_asset(&mut self) -> Result<()> {
        // Transfer asset via Metaplex Core CPI - will fail if asset is frozen
        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.current_owner.to_account_info()) // Current owner pays fees
            .authority(Some(&self.current_owner.to_account_info())) // Must be current owner
            .new_owner(&self.new_owner.to_account_info()) // Asset recipient
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke()?;
        
        // Log successful transfer with addresses
        msg!("Asset transferred from {} to {}", 
             self.current_owner.key(), 
             self.new_owner.key());
        Ok(())
    }
}
