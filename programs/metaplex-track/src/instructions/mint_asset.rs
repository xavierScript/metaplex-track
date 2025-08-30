use anchor_lang::prelude::*;

use mpl_core::{
    instructions::CreateV1CpiBuilder, 
    types::{
        Attribute, 
        Attributes, 
        DataState, 
        PluginAuthorityPair,
        FreezeDelegate,        
        PluginAuthority,
        BurnDelegate,
    },
};

#[derive(Accounts)]
pub struct MintAsset<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Created by Metaplex Core
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: Validated by Metaplex Core
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
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

impl<'info> MintAsset<'info> {
    pub fn mint_core_asset(&mut self, bump: MintAssetBumps, freeze_authority: Option<Pubkey>, burn_authority: Option<Pubkey>) -> Result<()> {
        let seeds = &[
            b"authority",
            self.collection.to_account_info().key.as_ref(),
            &[bump.authority],
        ];

        let signer_seeds = &[&seeds[..]];

        let attributes_plugin = PluginAuthorityPair {
            plugin: mpl_core::types::Plugin::Attributes(Attributes { 
                attribute_list: vec![
                    Attribute { 
                        key: "Ledger".to_string(), 
                        value: "NFT".to_string() 
                    }
                ]
            }), 
            authority: None
        };

        let freeze_plugin = PluginAuthorityPair {
            plugin: mpl_core::types::Plugin::FreezeDelegate(FreezeDelegate { 
                frozen: false
            }),
            authority: Some(PluginAuthority::Address { 
                address: freeze_authority.unwrap_or(self.authority.key())
            }),
        };

        let burn_plugin = PluginAuthorityPair {
            plugin: mpl_core::types::Plugin::BurnDelegate(BurnDelegate {}),
            authority: Some(PluginAuthority::Address { 
                address: burn_authority.unwrap_or(self.authority.key())
            }),
        };

        let plugins = vec![attributes_plugin, freeze_plugin, burn_plugin];

        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.mint.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .authority(Some(&self.authority.to_account_info()))
            .payer(&self.user.to_account_info())
            .owner(Some(&self.user.to_account_info()))
            .update_authority(None)
            .system_program(&self.system_program.to_account_info())
            .data_state(DataState::AccountState)
            .name("My Asset".to_string())
            .uri("https://myasset.com".to_string())
            .plugins(plugins)
            .invoke_signed(signer_seeds)?;
        
        Ok(())
    }
}