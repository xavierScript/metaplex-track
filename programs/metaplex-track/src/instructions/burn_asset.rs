use anchor_lang::prelude::*;
use mpl_core::{
    instructions::BurnV1CpiBuilder,
};

#[derive(Accounts)]
pub struct BurnAsset<'info> {
    #[account(mut)]
    pub burn_authority: Signer<'info>,
    /// CHECK: This is the asset to be burned
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: This is the collection
    pub collection: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> BurnAsset<'info> {
    pub fn burn_asset(&mut self) -> Result<()> {
        // Only the burn authority can burn the asset
        BurnV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.burn_authority.to_account_info())
            .authority(Some(&self.burn_authority.to_account_info()))
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke()?;
        
        msg!("Asset has been burned by authorized authority");
        Ok(())
    }
}
