use crate::function_api::unicode_block_alphabetic_presentation_form::*;
use crate::function_api::unicode_block_hebrew::*;
use crate::function_api::unicode_script_hebrew::*;

/// A trait for identification and validation of Hebrew characters
///
/// For the implementation of the trait the functions descibed in de file 'function_api.rs' are reused
pub trait HebrewUnicodeScript {
    /// Checks if the given character belongs to the unicode script 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ק'.is_script_hbr());
    /// ```
    fn is_script_hbr(&self) -> bool;
    /// Checks if the given character is a 'point' type within the unicode script 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB1E}'.is_script_hbr_point());
    /// ```
    fn is_script_hbr_point(&self) -> bool;
    /// Checks if the given character is a 'consonant' type within the unicode script 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ד'.is_script_hbr_consonant());
    /// ```
    fn is_script_hbr_consonant(&self) -> bool;
    /// Checks if the given character is a 'ligature_yiddisch' type within the unicode script 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB1F}'.is_script_hbr_ligature_yiddisch());
    /// ```
    fn is_script_hbr_ligature_yiddisch(&self) -> bool;
    /// Checks if the given character belongs to the unicode block 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ד'.is_hbr_block());
    /// ```
    fn is_hbr_block(&self) -> bool;
    /// Checks if the given character is a Hebrew acccent.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05AE}'.is_hbr_accent());
    /// ```
    fn is_hbr_accent(&self) -> bool;
    /// Checks if the given character is a Hebrew mark.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05C4}'.is_hbr_mark());
    /// ```
    fn is_hbr_mark(&self) -> bool;
    /// Checks if the given character is a Hebrew point
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05B0}'.is_hbr_point());
    /// ```
    fn is_hbr_point(&self) -> bool;
    /// Checks if the given character is a Hebrew vowel
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05BB}'.is_hbr_point_vowel());
    /// ```
    fn is_hbr_point_vowel(&self) -> bool;
    /// Checks if the given character is a Hebrew semi-vowel
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05B0}'.is_hbr_point_semi_vowel());
    /// ```
    fn is_hbr_point_semi_vowel(&self) -> bool;
    /// Checks if the given character is a Hebrew reading sign
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05BC}'.is_hbr_point_reading_sign());
    /// ```
    fn is_hbr_point_reading_sign(&self) -> bool;
    /// Checks if the given character is a Hebrew punctuation.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05F4}'.is_hbr_punctuation());
    /// ```
    fn is_hbr_punctuation(&self) -> bool;
    /// Checks if the given character is a Hebrew consonant.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ד'.is_hbr_consonant());
    /// ```
    fn is_hbr_consonant(&self) -> bool;
    /// Checks if the given character is a Hebrew consonant normal.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ד'.is_hbr_consonant_normal());
    /// ```
    fn is_hbr_consonant_normal(&self) -> bool;
    /// Checks if the given character is a Hebrew consonant final.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ף'.is_hbr_consonant_final());
    /// ```
    fn is_hbr_consonant_final(&self) -> bool;
    /// Checks if the given character is a Hebrew yod-triangle.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05EF}'.is_hbr_yod_triangle());
    /// ```
    fn is_hbr_yod_triangle(&self) -> bool;
    /// Checks if the given character is a Hebrew ligature yiddish.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05F0}'.is_hbr_ligature_yiddish());
    /// ```
    fn is_hbr_ligature_yiddish(&self) -> bool;
    /// Checks if the given character belongs to the unicode block 'Alphabetic Presentation Form'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB4E}'.is_apf_block());
    /// ```
    fn is_apf_block(&self) -> bool;
    /// Checks if the given character is an apf point.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB1E}'.is_apf_point_reading_sign());
    /// ```
    fn is_apf_point_reading_sign(&self) -> bool;
    /// Checks if the given character is an apf letter.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB20}'.is_apf_consonant());
    /// ```
    fn is_apf_consonant(&self) -> bool;
    /// Checks if the given character is an apf an alternative letter.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB20}'.is_apf_consonant_alternative());
    /// ```
    fn is_apf_consonant_alternative(&self) -> bool;
    /// Checks if the given character is an apf wide letter.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB21}'.is_apf_consonant_wide());
    /// ```
    fn is_apf_consonant_wide(&self) -> bool;
    /// Checks if the given character is an apf letter with a vowel
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB30}'.is_apf_consonant_with_vowel());
    /// ```
    fn is_apf_consonant_with_vowel(&self) -> bool;
    /// Checks if the given character is a apf Yiddish ligature.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB1F}'.is_apf_ligature_yiddisch());
    /// ```
    fn is_apf_ligature_yiddisch(&self) -> bool;
    /// Checks if the given character is a Yiddish ligature.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB4F}'.is_apf_ligature());
    /// ```
    fn is_apf_ligature(&self) -> bool;
}
impl HebrewUnicodeScript for char {
    // Unicode Script: Hebrew
    fn is_script_hbr(&self) -> bool {
        is_script_hbr(*self)
    }
    fn is_script_hbr_point(&self) -> bool {
        is_script_hbr_point(*self)
    }
    fn is_script_hbr_consonant(&self) -> bool {
        is_script_hbr_consonant(*self)
    }
    fn is_script_hbr_ligature_yiddisch(&self) -> bool {
        is_script_hbr_ligature_yiddisch(*self)
    }
    // Unicode Block: Hebrew
    fn is_hbr_block(&self) -> bool {
        is_hbr_block(*self)
    }
    fn is_hbr_accent(&self) -> bool {
        is_hbr_accent(*self)
    }
    fn is_hbr_mark(&self) -> bool {
        is_hbr_mark(*self)
    }
    fn is_hbr_point(&self) -> bool {
        is_hbr_point(*self)
    }
    fn is_hbr_point_vowel(&self) -> bool {
        is_hbr_point_vowel(*self)
    }
    fn is_hbr_point_semi_vowel(&self) -> bool {
        is_hbr_point_semi_vowel(*self)
    }
    fn is_hbr_point_reading_sign(&self) -> bool {
        is_hbr_point_reading_sign(*self)
    }
    fn is_hbr_punctuation(&self) -> bool {
        is_hbr_punctuation(*self)
    }
    fn is_hbr_consonant(&self) -> bool {
        is_hbr_consonant(*self)
    }
    fn is_hbr_consonant_normal(&self) -> bool {
        is_hbr_consonant_normal(*self)
    }
    fn is_hbr_consonant_final(&self) -> bool {
        is_hbr_consonant_final(*self)
    }
    fn is_hbr_yod_triangle(&self) -> bool {
        is_hbr_yod_triangle(*self)
    }
    fn is_hbr_ligature_yiddish(&self) -> bool {
        is_hbr_ligature_yiddish(*self)
    }
    // Unicode Block: Alphabetic Presentation Form
    fn is_apf_block(&self) -> bool {
        is_apf_block(*self)
    }
    fn is_apf_point_reading_sign(&self) -> bool {
        is_apf_point_reading_sign(*self)
    }
    fn is_apf_consonant(&self) -> bool {
        is_apf_consonant(*self)
    }
    fn is_apf_consonant_alternative(&self) -> bool {
        is_apf_consonant_alternative(*self)
    }
    fn is_apf_consonant_wide(&self) -> bool {
        is_apf_consonant_wide(*self)
    }
    fn is_apf_consonant_with_vowel(&self) -> bool {
        is_apf_consonant_with_vowel(*self)
    }
    fn is_apf_ligature_yiddisch(&self) -> bool {
        is_apf_ligature_yiddisch(*self)
    }
    fn is_apf_ligature(&self) -> bool {
        is_apf_ligature(*self)
    }
}

#[cfg(test)]
mod test_trait {
    use super::*;

    #[test]
    fn test_trait_hbr_script() {
        assert!(!'a'.is_script_hbr());
    }

    #[test]
    fn test_trait_hbr_script_point() {
        assert!(!'a'.is_script_hbr_point());
    }

    #[test]
    fn test_trait_hbr_script_consonant() {
        assert!(!'a'.is_script_hbr_consonant());
    }

    #[test]
    fn test_trait_hbr_script_ligature_yiddisch() {
        assert!(!'a'.is_script_hbr_ligature_yiddisch());
    }

    #[test]
    fn test_trait_hbr_block() {
        assert!(!'a'.is_hbr_block());
    }

    #[test]
    fn test_trait_hbr_accent() {
        assert!(!'a'.is_hbr_accent());
    }

    #[test]
    fn test_trait_hbr_mark() {
        assert!(!'a'.is_hbr_mark());
    }

    #[test]
    fn test_trait_hbr_point() {
        assert!(!'a'.is_hbr_point());
    }

    #[test]
    fn test_trait_hbr_point_vowel() {
        assert!(!'a'.is_hbr_point_vowel());
    }

    #[test]
    fn test_trait_hbr_point_semi_vowel() {
        assert!(!'a'.is_hbr_point_semi_vowel());
    }

    #[test]
    fn test_trait_hbr_point_reading_sign() {
        assert!(!'a'.is_hbr_point_reading_sign());
    }

    #[test]
    fn test_trait_hbr_punctuation() {
        assert!(!'a'.is_hbr_punctuation());
    }

    #[test]
    fn test_trait_hbr_consonant() {
        assert!(!'a'.is_hbr_consonant());
    }

    #[test]
    fn test_trait_hbr_consonant_normal() {
        assert!(!'a'.is_hbr_consonant_normal());
    }

    #[test]
    fn test_trait_hbr_consonant_final() {
        assert!(!'a'.is_hbr_consonant_final());
    }

    #[test]
    fn test_trait_hbr_yod_triangle() {
        assert!(!'a'.is_hbr_yod_triangle());
    }

    #[test]
    fn test_trait_hbr_ligature_yiddish() {
        assert!(!'a'.is_hbr_ligature_yiddish());
    }

    #[test]
    fn test_trait_apf_block() {
        assert!(!'a'.is_apf_block());
    }

    #[test]
    fn test_trait_apf_point_reading_sign() {
        assert!(!'a'.is_apf_point_reading_sign());
    }

    #[test]
    fn test_trait_apf_consonant() {
        assert!(!'a'.is_apf_consonant());
    }

    #[test]
    fn test_trait_apf_consonant_alternative() {
        assert!(!'a'.is_apf_consonant_alternative());
    }

    #[test]
    fn test_trait_apf_consonant_wide() {
        assert!(!'a'.is_apf_consonant_wide());
    }

    #[test]
    fn test_trait_apf_consonant_with_vowel() {
        assert!(!'a'.is_apf_consonant_with_vowel());
    }

    #[test]
    fn test_trait_apf_ligature_yiddisch() {
        assert!(!'a'.is_apf_ligature_yiddisch());
    }

    #[test]
    fn test_trait_apf_ligature() {
        assert!(!'a'.is_apf_ligature());
    }
}
