// the whole crate is no_std
#![no_std]
// include README.md with additional tests
#![doc = include_str!("../README.md")]

// contains all function API's
mod collections;
mod unicode_block_apf;
mod unicode_block_hbr;

// re-export
pub use self::collections::unicode_block_alphabetic_presentation_forms::*;
pub use self::collections::unicode_block_hebrew::*;
pub use self::collections::unicode_script_hebrew::*;

pub use self::unicode_block_hbr::hebrew_accents::*;
pub use self::unicode_block_hbr::hebrew_consonants::*;
pub use self::unicode_block_hbr::hebrew_marks::*;
pub use self::unicode_block_hbr::hebrew_misc::*;
pub use self::unicode_block_hbr::hebrew_points::*;
pub use self::unicode_block_hbr::hebrew_punctuations::*;

pub use self::unicode_block_apf::apf_consonant_wide::*;
pub use self::unicode_block_apf::apf_consonant_with_vowel::*;
pub use self::unicode_block_apf::apf_letter_alternative::*;
pub use self::unicode_block_apf::apf_ligature::*;
pub use self::unicode_block_apf::apf_point::*;

// contains the trait API
mod trait_def;
mod trait_impl;
// re-export
pub use self::trait_def::HebrewUnicodeScript;
//pub use self::trait_impl::HebrewUnicodeScript;
