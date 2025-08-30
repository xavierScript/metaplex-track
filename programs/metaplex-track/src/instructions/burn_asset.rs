use anchor_lang::prelude::*;
use mpl_core::{
    instructions::BurnV1CpiBuilder,
};

#[derive(Accounts)]
pub struct BurnAsset<'info> {
    #[account(mut)]
    pub burn_authority: Signer<'info>,
    /// CHECK: Validated by Metaplex Core
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: Validated by Metaplex Core
    pub collection: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    /// CHECK: Verified by address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> BurnAsset<'info> {
    pub fn burn_asset(&mut self) -> Result<()> {
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
        
        msg!("Asset burned by {}", self.burn_authority.key());
        Ok(())
    }
}
