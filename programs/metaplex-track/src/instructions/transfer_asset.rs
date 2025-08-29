use anchor_lang::prelude::*;
use mpl_core::{
    instructions::TransferV1CpiBuilder,
};

#[derive(Accounts)]
pub struct TransferAsset<'info> {
    #[account(mut)]
    pub current_owner: Signer<'info>,
    /// CHECK: This is the new owner who will receive the asset
    pub new_owner: UncheckedAccount<'info>,
    /// CHECK: This is the asset to be transferred
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: This is the collection (optional)
    pub collection: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> TransferAsset<'info> {
    pub fn transfer_asset(&mut self) -> Result<()> {
        // Transfer the asset to the new owner
        // This will fail if the asset is frozen
        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.current_owner.to_account_info())
            .authority(Some(&self.current_owner.to_account_info()))
            .new_owner(&self.new_owner.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke()?;
        
        msg!("Asset transferred from {} to {}", 
             self.current_owner.key(), 
             self.new_owner.key());
        Ok(())
    }
}
