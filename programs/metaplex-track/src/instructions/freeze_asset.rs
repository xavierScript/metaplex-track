use anchor_lang::prelude::*;
use mpl_core::{
    instructions::UpdatePluginV1CpiBuilder,
    types::{FreezeDelegate, Plugin},
};

// Account validation struct for asset freezing/unfreezing
#[derive(Accounts)]
pub struct FreezeAsset<'info> {
    #[account(mut)]
    pub freeze_authority: Signer<'info>, // Authority that can freeze/unfreeze
    /// CHECK: Asset to be frozen/unfrozen - validated by Metaplex Core
    #[account(mut)]
    pub asset: UncheckedAccount<'info>, // Target asset
    /// CHECK: Collection account - optional, validated by Metaplex Core
    pub collection: Option<UncheckedAccount<'info>>, // Parent collection (optional)
    pub system_program: Program<'info, System>, // Required for plugin updates
    /// CHECK: Metaplex Core program - verified by address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>, // Metaplex Core program
}

impl<'info> FreezeAsset<'info> {
    // Freezes an asset to prevent transfers
    pub fn freeze_asset(&mut self) -> Result<()> {
        // Update freeze plugin to frozen state via Metaplex Core CPI
        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.freeze_authority.to_account_info()) // Authority pays fees
            .authority(Some(&self.freeze_authority.to_account_info())) // Must be freeze authority
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: true })) // Set frozen = true
            .invoke()?;
        
        msg!("Asset has been frozen"); // Log freeze action
        Ok(())
    }

    // Unfreezes an asset to allow transfers
    pub fn unfreeze_asset(&mut self) -> Result<()> {
        // Update freeze plugin to unfrozen state via Metaplex Core CPI
        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.freeze_authority.to_account_info()) // Authority pays fees
            .authority(Some(&self.freeze_authority.to_account_info())) // Must be freeze authority
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false })) // Set frozen = false
            .invoke()?;
        
        msg!("Asset has been unfrozen"); // Log unfreeze action
        Ok(())
    }
}
