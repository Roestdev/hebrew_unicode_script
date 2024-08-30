#![doc = include_str!("../README.md")]

// contains all function API's
mod function_api;
// re-export
pub use self::function_api::unicode_block_alphabetic_presentation_form::*;
pub use self::function_api::unicode_block_hebrew::*;
pub use self::function_api::unicode_script_hebrew::*;

// contains the trait API
mod trait_api;
// re-export
pub use self::trait_api::HebrewUnicodeScript;
