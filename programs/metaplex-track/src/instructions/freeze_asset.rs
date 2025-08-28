use anchor_lang::prelude::*;
use mpl_core::{
    instructions::UpdatePluginV1CpiBuilder,
    types::{FreezeDelegate, Plugin},
};

#[derive(Accounts)]
pub struct FreezeAsset<'info> {
    #[account(mut)]
    pub freeze_authority: Signer<'info>,
    /// CHECK: This is the asset to be frozen/unfrozen
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: This is the collection
    pub collection: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> FreezeAsset<'info> {
    pub fn freeze_asset(&mut self) -> Result<()> {
        // Update the freeze plugin to set frozen = true
        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.freeze_authority.to_account_info())
            .authority(Some(&self.freeze_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: true }))
            .invoke()?;
        
        msg!("Asset has been frozen");
        Ok(())
    }

    pub fn unfreeze_asset(&mut self) -> Result<()> {
        // Update the freeze plugin to set frozen = false
        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.freeze_authority.to_account_info())
            .authority(Some(&self.freeze_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false }))
            .invoke()?;
        
        msg!("Asset has been unfrozen");
        Ok(())
    }
}
