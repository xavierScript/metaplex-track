use anchor_lang::prelude::*;

use mpl_core::{
    instructions::CreateCollectionV1CpiBuilder,
};

#[derive(Accounts)]
pub struct MintCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub collection: Signer<'info>,
    /// CHECK: Collection authority PDA - verified by seeds constraint
    #[account(
        seeds = [b"authority", collection.key().as_ref()],
        bump,
    )]
    pub authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: Verified by address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> MintCollection<'info> {
    pub fn mint_core_collection(&mut self, bump: MintCollectionBumps) -> Result<()> {
        let seeds = &[
            b"authority",
            self.collection.to_account_info().key.as_ref(),
            &[bump.authority],
        ];

        let signer_seeds = &[&seeds[..]];

        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .update_authority(Some(&self.authority.to_account_info()))
            .payer(&self.user.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name("Legder Collection".to_string())
            .uri("https://myasset.com".to_string())
            .invoke_signed(signer_seeds)?;
        
        Ok(())
    }
}