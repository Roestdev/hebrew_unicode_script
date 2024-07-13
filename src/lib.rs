//! This crate provides some functions that will check if a hebrew character belongs to a certain set inside the unicode script 'Hebrew'      
//!
//! ### Short Description
//! This library crate contains some simple functions to determine
//! whether a particular character belongs to a particular Hebrew character set.
//! There are three types (layers) of character sets (actually unicode code points):  
//!  1. All Hebrew characters (unicode script 'Hebrew' [^1]).  
//!  2. Hebrew characters, belonging to a particular unicode block [^2] ( 'Hebrew' and 'Alphabetic Presentation Form').
//!     - Only the following ranges are used: *U+0590 .. U+05FF* and *U+FB1D .. U+FB4F*  
//!  3. Hebrew characters, belonging to a particular type within a unicode block ( e.g. vowels, accents, etc ... )  
//! [^1]: A unicode script (collection), may consist of one or more unicode blocks
//! [^2]: A unicode block contains one or more contiguous ranges of numeric character codes (code points)
//!
//! ### Panics
//! - Not that I am aware of.
//!
//! ### Errors
//! - Only true or false are returned.
//!
//! ### Safety
//! - All functions are written in safe Rust.
//!
//! ### References
//! [**Unicode script for Hebrew**](https://www.charactercodes.net/script/hebr)   
//! - [Supported scripts in unicode](https://www.unicode.org/standard/supported.html)
//!  
//! [**Unicode Block Names**](https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt) that contain Hebrew characters:
//! 1. [*Hebrew*](https://www.unicode.org/charts/PDF/U0590.pdf)  
//!     - See also: <https://graphemica.com/blocks/hebrew/>  
//! 2. [*Alphabetic Presentation Form*](https://www.unicode.org/charts/PDF/UFB00.pdf)  
//!     - See also: <https://graphemica.com/blocks/alphabetic-presentation-forms>  
//!

pub use self::apf_block::*;
pub use self::hebrew_block::*;
pub use self::script::*;


pub mod script {
    use crate::is_hebrew_block;
    use crate::is_hebrew_apf_block;

    /// Checks if the given character belongs to the unicode script 'Hebrew'.
    ///
    /// Reference: <https://www.charactercodes.net/script/hebr>
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hebrew_script;
    /// let hebrew_ch = 'מ';
    /// assert!(is_hebrew_script(hebrew_ch));
    ///
    /// let non_hebrew_ch = 'م';
    /// assert!(!is_hebrew_script(non_hebrew_ch));
    ///
    /// let non_hebrew_ch = 'д';
    /// assert!(!is_hebrew_script(non_hebrew_ch));
    /// ```
    pub fn is_hebrew_script(c: char) -> bool {
        match c {
            c if is_hebrew_block(c) => true,
            c if is_hebrew_apf_block(c) => true,
            _ => false,
        }
    }
}

pub mod hebrew_block {
    /// Checks if the given character belongs to the unicode block 'Hebrew'.
    ///
    /// Reference: <https://www.unicode.org/charts/PDF/U0590.pdf>
    ///
    /// Hebrew (code points: U+0590 .. U+05FF)
    ///
    /// See also: <https://graphemica.com/blocks/hebrew>
    ///
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_script::is_hebrew_block;
    ///
    /// let test_str = "וַֽיְחִי־שֵׁ֗םאַֽחֲרֵי֙הוֹלִיד֣וֹאֶת־אַרְפַּכְשָׁ֔דחֲמֵ֥שׁמֵא֖וֹתשָׁנָ֑הוַיּ֥וֹלֶדבָּנִ֖יםוּבָנֽוֹת׃";
    /// for c in test_str.chars() {
    ///    assert!(is_hebrew_block(c));
    /// }
    /// ```
    pub fn is_hebrew_block(c: char) -> bool {
        match c {
            c if is_accent(c) => true,
            c if is_mark(c) => true,
            c if is_vowel_point(c) => true,
            c if is_punctuation(c) => true,
            c if is_letter(c) => true,
            c if is_yod_triangle(c) => true,
            c if is_ligature_yiddish(c) => true,
            _ => false,
        }
    }

    /// Checks if the given character is a Hebrew acccent.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_accent;
    /// use is_even::IsEven;
    ///
    /// let test_str = "ב֑";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8.is_even() {
    ///     // even position is a normal letter (non-accent)
    ///     assert!(!is_accent(c));
    ///   } else {
    ///     assert!(is_accent(c));
    ///   }
    /// }
    /// ```
    pub fn is_accent(c: char) -> bool {
        matches!(c, '\u{0591}'..='\u{05AE}')
    }

    /// Checks if the given character is a Hebrew mark.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_mark;
    /// use is_even::IsEven;
    ///
    /// let test_str = "ב֯";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8.is_even() {
    ///     // even position is a normal letter (non-mark)
    ///     assert!(!is_mark(c));
    ///   } else {
    ///     assert!(is_mark(c));
    ///   }
    /// }
    /// ```
    pub fn is_mark(c: char) -> bool {
        // 05AF + 05C4 + 05C5
        matches!(c, '\u{05AF}' | '\u{05C4}' | '\u{05C5}')
    }
    /// Checks if the given character is a Hebrew point (vowel)
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_vowel_point;
    /// use is_even::IsEven;
    ///
    /// let test_str = "מָ";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8.is_even() {
    ///     // even position is a normal letter (non-vowel-point)
    ///     assert!(!is_vowel_point(c));
    ///   } else {
    ///     assert!(is_vowel_point(c));
    ///   }
    /// }
    /// ```
    pub fn is_vowel_point(c: char) -> bool {
        // 05B0 .. 05BD + 05BF + 05C1 .. 05C2 + 05C7
        matches!(c,
            '\u{05B0}'..='\u{05BD}'
            | '\u{05BF}'
            | '\u{05C1}'..='\u{05C2}'
            | '\u{05C7}'
        )
    }
    /// Checks if the given character is a Hebrew punctuation.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_mark;
    /// use is_even::IsEven;
    ///
    /// let test_str = "ב֯";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8.is_even() {
    ///     // even position is a normal letter (non_mark)
    ///     assert!(!is_mark(c));
    ///   } else {
    ///     assert!(is_mark(c));
    ///   }
    /// }
    /// ```
    pub fn is_punctuation(c: char) -> bool {
        // 05BE + 05C0 + 05C3 + 05C6 + 05F3 + 05F4
        matches!(
            c,
            '\u{05BE}' | '\u{05C0}' | '\u{05C3}' | '\u{05C6}' | '\u{05F3}'..='\u{05F4}'
        )
    }
    /// Checks if the given character is a Hebrew letter (normal).
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_letter_normal;
    /// use hebrew_unicode_script::is_letter;
    ///
    /// let test_str = "אבגדהוזחטיכלמנסעפצקרשת";
    /// for c in test_str.chars() {
    ///     assert!(is_letter_normal(c));
    ///     assert!(is_letter(c));
    /// }
    /// ```
    pub fn is_letter_normal(c: char) -> bool {
        // 05D0..05D9 + 05DB..05DC + 05DE + 05E0..05E2 + 05E4 + 05E6..05EA
        matches!(
            c,
            '\u{05D0}'..='\u{05D9}'
            | '\u{05DB}'..='\u{05DC}'
            | '\u{05DE}'
            | '\u{05E0}'..='\u{05E2}'
            | '\u{05E4}'
            | '\u{05E6}'..='\u{05EA}'
        )
    }
    /// Checks if the given character is a Hebrew letter (final).
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_letter_final;
    /// use hebrew_unicode_script::is_letter;
    ///
    /// let test_str = "ךםןףץ";
    /// for c in test_str.chars() {
    ///     assert!(is_letter_final(c));
    ///     assert!(is_letter(c));
    /// }
    /// ```
    pub fn is_letter_final(c: char) -> bool {
        // 05DA + 05DD + 05DF + 05E3 + 05E5
        matches!(
            c,
            '\u{05DA}' | '\u{05DD}' | '\u{05DF}' | '\u{05E3}' | '\u{05E5}'
        )
    }
    /// Checks if the given character is a Hebrew letter (final OR normal)
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_letter;
    /// use is_even::IsEven;
    ///
    /// let test_str = "אבגדהוזחטיכךלמםנןסעפףצץקרשת";
    /// for c in test_str.chars() {
    ///   assert!(is_letter(c));
    /// }
    /// ```
    pub fn is_letter(c: char) -> bool {
        // 05D0 .. 05EA
        matches!(c, '\u{05D0}'..='\u{05EA}')
    }
    /// Checks if the given character is a Hebrew yod triangle.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_yod_triangle;
    ///
    /// let yod_triangle = '\u{05EF}';
    /// assert!(is_yod_triangle(yod_triangle));
    /// ```
    pub fn is_yod_triangle(c: char) -> bool {
        matches!(c, '\u{05EF}')
    }
    /// Checks if the given character is a Yiddish ligature.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_ligature_yiddish;
    ///
    /// let test_str = "װױײ";
    /// for c in test_str.chars() {
    ///   assert!(is_ligature_yiddish(c));
    /// }
    /// ```
    pub fn is_ligature_yiddish(c: char) -> bool {
        // 05F0 .. 05F2
        matches!(c, '\u{05F0}'..='\u{05F2}')
    }
}

pub mod apf_block {
    /// Checks if the given character belongs to the unicode block 'Alphabetic Presentation Form'.
    ///
    /// Reference: <https://www.unicode.org/charts/PDF/UFB00.pdf>
    ///
    /// Alphabetic Presentation Form (code points: U+FB1D .. U+FB4F)
    ///
    /// See also: <https://graphemica.com/blocks/alphabetic-presentation-forms>
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hebrew_apf_block;
    ///
    /// let apf_ch = '\u{FB22}'; // HEBREW LETTER WIDE DALET
    /// assert!(is_hebrew_apf_block(apf_ch));
    ///
    /// let non_apf_ch = '\u{FB13}'; // ARMENIAN SMALL LIGATURE MEN NOW
    /// assert!(!is_hebrew_apf_block(non_apf_ch));
    /// ```
    pub fn is_hebrew_apf_block(c: char) -> bool {
        match c {
            c if is_apf_point(c) => true,
            c if is_apf_letter(c) => true,
            c if is_apf_ligature_yiddisch(c) => true,
            c if is_apf_ligature(c) => true,
            _ => false,
        }
    }
    /// Checks if the given character is an AFP point.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_point;
    ///
    /// let ly = '\u{FB1E}';
    /// assert!(is_apf_point(ly));
    /// ```
    pub fn is_apf_point(c: char) -> bool {
        // U+FB1E
        matches!(c, '\u{FB1E}')
    }
    /// Checks if the given character is an AFP letter.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_letter;
    ///
    /// let afp_l = '\u{FB3E}';
    /// assert!(is_apf_letter(afp_l));
    ///
    /// let afp_l = 'p';
    /// assert!(!is_apf_letter(afp_l));
    /// ```
    pub fn is_apf_letter(c: char) -> bool {
        // U+FB1D + (U+FB20 .. U+FB36) + (U+FB38 .. U+FB3C) + U+FB3E + (U+FB40 .. U+FB41)
        // + (U+FB43 .. U+FB44) + (U+FB46 .. U+FB4E)
        matches!(
            c,
            '\u{FB1D}'
            | '\u{FB20}'..='\u{FB36}'
            | '\u{FB38}'..='\u{FB3C}'
            | '\u{FB3E}'
            | '\u{FB40}'..='\u{FB41}'
            | '\u{FB43}'..='\u{FB44}'
            | '\u{FB46}'..='\u{FB4E}'
        )
    }
    /// Checks if the given character is a AFP Yiddish ligature.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_ligature_yiddisch;
    ///
    /// // HEBREW LIGATURE YIDDISH YOD YOD PATAH
    /// let test_ch = '\u{FB1F}';
    /// assert!(is_apf_ligature_yiddisch(test_ch));
    ///
    /// // ARABIC LETTER TEHEH ISOLATED FORM
    /// let test_ch = '\u{FB62}';
    ///   assert!(!is_apf_ligature_yiddisch(test_ch));
    /// ```
    pub fn is_apf_ligature_yiddisch(c: char) -> bool {
        // U+FB1F
        matches!(c, '\u{FB1F}')
    }
    /// Checks if the given character is a Yiddish ligature.
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_ligature;
    ///
    /// // HEBREW LIGATURE ALEF LAMED
    /// let test_ch = '\u{FB4F}';
    /// assert!(is_apf_ligature(test_ch));
    ///
    /// // HEBREW LIGATURE YIDDISH YOD YOD PATAH
    /// let test_ch = '\u{FB1F}';
    /// assert!(!is_apf_ligature(test_ch));
    /// ```
    pub fn is_apf_ligature(c: char) -> bool {
        // U+FB4F
        matches!(c, '\u{FB4F}')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use is_even::IsEven;

    #[test]
    fn test_hebrew_accent() {
        let test_str1 = "ב֑ב֒ב֓ב֔ב֕ב֖ב֗ב֘ב֙ב֚";
        let test_str2 = "ב֛ב֜ב֝ב֞ב֟ב֠ב֡ב֢ב֣ב֤";
        let test_str3 = "ב֥ב֦ב֧ב֨ב֩ב֪ב֫ב֬ב֭ב֮";
        check_accents(&test_str1);
        check_accents(&test_str2);
        check_accents(&test_str3);
    }
    #[test]
    fn test_hebrew_mark() {
        // 05AF + 05C4 + 05C5
        let test_str = "ב֯בׄבׅ";
        check_marks(&test_str);
    }
    #[test]
    fn test_hebrew_vowel_point() {
        let test_str = "בְבֱבֲבֳבִבֵבֶבַבָבֹבֺבֻבּבֽבֿבׁבׂבׇ";
        check_vowel_points(&test_str);
    }
    #[test]
    fn test_hebrew_puncuation() {
        let test_str = "ב־ב׀ב׃ב׆ב׳נ״";
        check_punctuations(&test_str);
    }
    #[test]
    fn test_hebrew_letter() {
        let test_str = "אבגדהוזחטיכךלמםנןסעפףצץקרשת";
        for c in test_str.chars() {
            assert!(is_letter(c));
        }
    }
    #[test]
    fn test_hebrew_letter_not_final() {
        let test_str = "אבגדהוזחטיכלמנסעפצקרשת";
        for c in test_str.chars() {
            assert!(is_letter_normal(c));
            assert!(is_letter(c));
        }
    }
    #[test]
    fn test_hebrew_letter_final() {
        let test_str = "ךםןףץ";
        for c in test_str.chars() {
            assert!(is_letter_final(c));
            assert!(is_letter(c));
        }
    }
    #[test]
    fn test_hebrew_yiddish_chars() {
        let test_str = "װױײ";
        for c in test_str.chars() {
            assert!(is_ligature_yiddish(c));
        }
    }
    #[test]
    fn test_hebrew_yod_triangle() {
        let yod_triangle = '\u{05EF}';
        assert!(is_yod_triangle(yod_triangle));
    }

    #[test]
fn test_apf_point() {

    let test_ch = '\u{FB1E}'; // HEBREW POINT JUDEO-SPANISH VARIKA

    // unicode script 'Hebrew'
    assert_eq!(is_hebrew_script(test_ch), true);

    // unicode block Hebrew
    assert_eq!(is_hebrew_block(test_ch), false);
    assert_eq!(is_accent(test_ch), false);
    assert_eq!(is_mark(test_ch), false);
    assert_eq!(is_vowel_point(test_ch), false);
    assert_eq!(is_punctuation(test_ch), false);
    assert_eq!(is_letter_normal(test_ch), false);
    assert_eq!(is_letter(test_ch), false);
    assert_eq!(is_yod_triangle(test_ch), false);
    assert_eq!(is_ligature_yiddish(test_ch), false);

    // unicode block 'Alphabetic Presentation Form'
    assert_eq!(is_hebrew_apf_block(test_ch), true);
    assert_eq!(is_apf_point(test_ch), true);
    assert_eq!(is_apf_letter(test_ch), false);
    assert_eq!(is_apf_ligature_yiddisch(test_ch), false);
    assert_eq!(is_apf_ligature(test_ch), false);

}

#[test]
fn test_apf_letter() {
    let test_ch = '\u{FB1D}'; // HEBREW LETTER YOD WITH HIRIQ

    // unicode script 'Hebrew'
    assert_eq!(is_hebrew_script(test_ch), true);

    // unicode block Hebrew
    assert_eq!(is_hebrew_block(test_ch), false);
    assert_eq!(is_accent(test_ch), false);
    assert_eq!(is_mark(test_ch), false);
    assert_eq!(is_vowel_point(test_ch), false);
    assert_eq!(is_punctuation(test_ch), false);
    assert_eq!(is_letter_normal(test_ch), false);
    assert_eq!(is_letter(test_ch), false);
    assert_eq!(is_yod_triangle(test_ch), false);
    assert_eq!(is_ligature_yiddish(test_ch), false);

    // unicode block 'Alphabetic Presentation Form'
    assert_eq!(is_hebrew_apf_block(test_ch), true);
    assert_eq!(is_apf_point(test_ch), false);
    assert_eq!(is_apf_letter(test_ch), true);
    assert_eq!(is_apf_ligature_yiddisch(test_ch), false);
    assert_eq!(is_apf_ligature(test_ch), false);

}

#[test]
fn test_apf_ligature_yiddisch() {
    let test_ch = '\u{FB1F}'; // HEBREW LIGATURE YIDDISH YOD YOD PATAH

    // unicode script 'Hebrew'
    assert_eq!(is_hebrew_script(test_ch), true);

    // unicode block Hebrew
    assert_eq!(is_hebrew_block(test_ch), false);
    assert_eq!(is_accent(test_ch), false);
    assert_eq!(is_mark(test_ch), false);
    assert_eq!(is_vowel_point(test_ch), false);
    assert_eq!(is_punctuation(test_ch), false);
    assert_eq!(is_letter_normal(test_ch), false);
    assert_eq!(is_letter(test_ch), false);
    assert_eq!(is_yod_triangle(test_ch), false);
    assert_eq!(is_ligature_yiddish(test_ch), false);

    // unicode block 'Alphabetic Presentation Form'
    assert_eq!(is_hebrew_apf_block(test_ch), true);
    assert_eq!(is_apf_point(test_ch), false);
    assert_eq!(is_apf_letter(test_ch), false);
    assert_eq!(is_apf_ligature_yiddisch(test_ch), true);
    assert_eq!(is_apf_ligature(test_ch), false);

}

#[test]
fn test_apf_ligature() {
    let test_ch = '\u{FB4F}'; // HEBREW LETTER WIDE DALET

    // unicode script 'Hebrew'
    assert_eq!(is_hebrew_script(test_ch), true);

    // unicode block Hebrew
    assert_eq!(is_hebrew_block(test_ch), false);
    assert_eq!(is_accent(test_ch), false);
    assert_eq!(is_mark(test_ch), false);
    assert_eq!(is_vowel_point(test_ch), false);
    assert_eq!(is_punctuation(test_ch), false);
    assert_eq!(is_letter_normal(test_ch), false);
    assert_eq!(is_letter(test_ch), false);
    assert_eq!(is_yod_triangle(test_ch), false);
    assert_eq!(is_ligature_yiddish(test_ch), false);

    // unicode block 'Alphabetic Presentation Form'
    assert_eq!(is_hebrew_apf_block(test_ch), true);
    assert_eq!(is_apf_point(test_ch), false);
    assert_eq!(is_apf_letter(test_ch), false);
    assert_eq!(is_apf_ligature_yiddisch(test_ch), false);
    assert_eq!(is_apf_ligature(test_ch), true);
}

    fn check_accents(str: &str) {
        // str is an alternation of consonant-accent-consonat-accent ...
        for (position, c) in str.chars().enumerate() {
            let position_u8 = u8::try_from(position).unwrap();
            if position_u8.is_even() {
                //println!("fn check_accents::at even position {} is: {}", pos_u8,c.escape_default());
                assert!(is_letter(c));
            } else {
                //println!("fn check_accents::at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_accent(c));
            }
        }
    }
    fn check_marks(str: &str) {
        for (pos, c) in str.chars().enumerate() {
            let pos_u8 = u8::try_from(pos).unwrap();
            if pos_u8.is_even() {
                //println!("fn check_marks::at even position {} is: {}", pos_u8, c.escape_default());
                assert!(is_letter(c));
            } else {
                //println!("fn check_marks::at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_mark(c));
            }
        }
    }
    fn check_vowel_points(str: &str) {
        for (pos, c) in str.chars().enumerate() {
            let pos_u8 = u8::try_from(pos).unwrap();
            if pos_u8.is_even() {
                //println!("fn check_vowel_points::char at even position {} is: {}", pos_u8, c.escape_default());
                assert!(is_letter(c));
            } else {
                //println!("fn check_vowel_points::char at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_vowel_point(c));
            }
        }
    }
    fn check_punctuations(str: &str) {
        for (pos, c) in str.chars().enumerate() {
            let pos_u8 = u8::try_from(pos).unwrap();
            if pos_u8.is_even() {
                //println!("fn check_punctuations::char at even position {} is: {}", pos_u8, c.escape_default());
                assert!(is_letter(c));
            } else {
                //println!("fn check_punctuations::char at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_punctuation(c));
            }
        }
    }
}
