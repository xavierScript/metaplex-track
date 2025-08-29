// Module declarations for all instruction handlers
pub mod mint_asset;
pub mod mint_collection;
pub mod freeze_asset;
pub mod burn_asset;
pub mod transfer_asset;

// Re-export all instruction structs and implementations
pub use mint_asset::*;
pub use mint_collection::*;
pub use freeze_asset::*;
pub use burn_asset::*;
pub use transfer_asset::*;