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

// Account validation struct for asset minting
#[derive(Accounts)]
pub struct MintAsset<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // User minting the asset (pays fees)
    /// CHECK: New asset mint account - will be created by Metaplex Core
    #[account(mut)]
    pub mint: Signer<'info>, // Asset mint account
    /// CHECK: Collection account - validated by Metaplex Core
    #[account(mut)]
    pub collection: UncheckedAccount<'info>, // Parent collection
    /// CHECK: Collection authority PDA - verified by seeds constraint
    #[account(
        seeds = [b"authority", collection.key().as_ref()],
        bump,
    )]
    pub authority: UncheckedAccount<'info>, // Collection authority PDA
    pub system_program: Program<'info, System>, // Required for account creation
    /// CHECK: Metaplex Core program - verified by address constraint
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>, // Metaplex Core program
}

impl<'info> MintAsset<'info> {
    // Mints a new asset with freeze and burn protection plugins
    pub fn mint_core_asset(&mut self, bump: MintAssetBumps, freeze_authority: Option<Pubkey>, burn_authority: Option<Pubkey>) -> Result<()> {
        // Create authority seeds for PDA signing
        let seeds = &[
            b"authority",
            self.collection.to_account_info().key.as_ref(),
            &[bump.authority],
        ];

        let signer_seeds = &[&seeds[..]];

        // Create attributes plugin for asset metadata
        let attributes_plugin = PluginAuthorityPair {
            plugin: mpl_core::types::Plugin::Attributes(Attributes { 
                attribute_list: vec![
                    Attribute { 
                        key: "Ledger".to_string(), 
                        value: "NFT".to_string() 
                    }
                ]
            }), 
            authority: None  // No specific authority needed for attributes
        };

        // Create freeze plugin for transfer control (starts unfrozen)
        let freeze_plugin = PluginAuthorityPair {
            plugin: mpl_core::types::Plugin::FreezeDelegate(FreezeDelegate { 
                frozen: false  // Asset starts unfrozen for initial transfers
            }),
            authority: Some(PluginAuthority::Address { 
                address: freeze_authority.unwrap_or(self.authority.key())
                // Use provided freeze authority or default to collection authority
            }),
        };

        // Create burn plugin for permanent asset destruction control
        let burn_plugin = PluginAuthorityPair {
            plugin: mpl_core::types::Plugin::BurnDelegate(BurnDelegate {}),
            authority: Some(PluginAuthority::Address { 
                address: burn_authority.unwrap_or(self.authority.key())
                // Use provided burn authority or default to collection authority
            }),
        };

        // Combine all plugins for asset creation
        let plugins = vec![attributes_plugin, freeze_plugin, burn_plugin];


        // Create asset via Metaplex Core CPI with all plugins
        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.mint.to_account_info())
            .collection(Some(&self.collection.to_account_info())) // Link to collection
            .authority(Some(&self.authority.to_account_info())) // Collection authority
            .payer(&self.user.to_account_info()) // User pays fees
            .owner(Some(&self.user.to_account_info())) // User owns the asset
            .update_authority(None) // No update authority (immutable)
            .system_program(&self.system_program.to_account_info())
            .data_state(DataState::AccountState) // Store data on-chain
            .name("My Asset".to_string()) // Asset name
            .uri("https://myasset.com".to_string()) // Asset metadata URI
            .plugins(plugins)  // Apply all configured plugins
            .invoke_signed(signer_seeds)?; // Sign with collection authority
        
        Ok(())
    }
}