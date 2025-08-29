use anchor_lang::prelude::*;
use mpl_core::{
    instructions::BurnV1CpiBuilder,
};

// Account validation struct for asset burning
#[derive(Accounts)]
pub struct BurnAsset<'info> {
    #[account(mut)]
    pub burn_authority: Signer<'info>, // Authority that can burn the asset
    /// CHECK: Asset to be burned - validated by Metaplex Core
    #[account(mut)]
    pub asset: UncheckedAccount<'info>, // Target asset for burning
    /// CHECK: Collection account - optional, validated by Metaplex Core
    pub collection: Option<UncheckedAccount<'info>>, // Parent collection (optional)
    pub system_program: Program<'info, System>, // Required for account closure
    /// CHECK: Metaplex Core program - verified by address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>, // Metaplex Core program
}

impl<'info> BurnAsset<'info> {
    // Permanently burns an asset, removing it from circulation
    pub fn burn_asset(&mut self) -> Result<()> {
        // Burn asset via Metaplex Core CPI - only burn authority can execute
        BurnV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(
                self.collection
                    .as_ref()
                    .map(|c| c.to_account_info())
                    .as_ref()
            )
            .payer(&self.burn_authority.to_account_info()) // Authority pays fees
            .authority(Some(&self.burn_authority.to_account_info())) // Must be burn authority
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke()?;
        
        msg!("Asset has been burned by authorized authority"); // Log burn action
        Ok(())
    }
}
