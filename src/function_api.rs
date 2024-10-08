pub mod unicode_script_hebrew {
    use crate::function_api::unicode_block_alphabetic_presentation_form::is_apf_block;
    use crate::function_api::unicode_block_alphabetic_presentation_form::is_apf_consonant;
    use crate::function_api::unicode_block_alphabetic_presentation_form::is_apf_ligature_yiddisch;
    use crate::function_api::unicode_block_alphabetic_presentation_form::is_apf_point_reading_sign;
    use crate::function_api::unicode_block_hebrew::is_hbr_block;
    use crate::function_api::unicode_block_hebrew::is_hbr_consonant;
    use crate::function_api::unicode_block_hebrew::is_hbr_ligature_yiddish;
    use crate::function_api::unicode_block_hebrew::is_hbr_point;
    /// Checks if the given character belongs to the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr;
    /// let hebrew_ch = 'מ';
    /// assert!(is_script_hbr(hebrew_ch));
    ///
    /// let non_hbr_ch = 'д';
    /// assert!(!is_script_hbr(non_hbr_ch));
    /// ```
    pub fn is_script_hbr(c: char) -> bool {
        is_hbr_block(c) || is_apf_block(c)
    }
    /// Checks if the given character is a 'point' type within the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr_point;
    ///
    /// let test_str = "מָ";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-vowel-point)
    ///     assert!(!is_script_hbr_point(c));
    ///   } else {
    ///     assert!(is_script_hbr_point(c));
    ///   }
    /// }
    /// let reading_sign = '\u{FB1E}';
    /// assert!(is_script_hbr_point(reading_sign));
    /// ```
    pub fn is_script_hbr_point(c: char) -> bool {
        is_hbr_point(c) || is_apf_point_reading_sign(c)
    }

    // todo!()
    ///

    /// Checks if the given character is a 'consonant' type within the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr_consonant;
    ///
    /// let test_str = "אבגדהוזחטיכךלמםנןסעפףצץקרשת";
    /// for c in test_str.chars() {
    ///   assert!(is_script_hbr_consonant(c));
    /// }
    /// let afp_alternative = '\u{FB20}';
    /// assert!(is_script_hbr_consonant(afp_alternative));
    ///
    /// ```
    pub fn is_script_hbr_consonant(c: char) -> bool {
        is_hbr_consonant(c) || is_apf_consonant(c)
    }

    /// Checks if the given character is a 'ligature_yiddisch' type within the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr_ligature_yiddisch;
    ///
    ///let test_str = "װױײ";
    /// for c in test_str.chars() {
    ///   assert!(is_script_hbr_ligature_yiddisch(c));
    /// }
    /// let liga_yiddish = '\u{FB1F}';
    /// assert!(is_script_hbr_ligature_yiddisch(liga_yiddish));
    /// ```
    pub fn is_script_hbr_ligature_yiddisch(c: char) -> bool {
        is_hbr_ligature_yiddish(c) || is_apf_ligature_yiddisch(c)
    }
}

pub mod unicode_block_hebrew {
    /// Checks if the given character belongs to the unicode block 'Hebrew' (HBR)
    ///
    /// # Examples
    /// ```
    /// use hebrew_unicode_script::is_hbr_block;
    ///
    /// let test_str = "וַֽיְחִי־שֵׁ֗םאַֽחֲרֵי֙הוֹלִיד֣וֹאֶת־אַרְפַּכְשָׁ֔דחֲמֵ֥שׁמֵא֖וֹתשָׁנָ֑הוַיּ֥וֹלֶדבָּנִ֖יםוּבָנֽוֹת׃";
    /// for c in test_str.chars() {
    ///    assert!(is_hbr_block(c));
    /// }
    /// ```
    pub fn is_hbr_block(c: char) -> bool {
        is_hbr_accent(c)
            || is_hbr_mark(c)
            || is_hbr_point(c)
            || is_hbr_punctuation(c)
            || is_hbr_consonant(c)
            || is_hbr_yod_triangle(c)
            || is_hbr_ligature_yiddish(c)
    }
    /// Checks if the given character is a HBR acccent.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent;
    ///
    /// let test_str = "ב֑";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-accent)
    ///     assert!(!is_hbr_accent(c));
    ///   } else {
    ///     assert!(is_hbr_accent(c));
    ///   }
    /// }
    /// ```
    pub fn is_hbr_accent(c: char) -> bool {
        matches!(c, '\u{0591}'..='\u{05AE}')
    }
    /// Checks if the given character is a HBR mark.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_mark;
    ///
    /// let test_str = "ב֯";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-mark)
    ///     assert!(!is_hbr_mark(c));
    ///   } else {
    ///     assert!(is_hbr_mark(c));
    ///   }
    /// }
    /// ```
    pub fn is_hbr_mark(c: char) -> bool {
        // 05AF + 05C4 + 05C5
        matches!(c, '\u{05AF}' | '\u{05C4}' | '\u{05C5}')
    }
    /// Checks if the given character is a HBR point
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_point;
    ///
    /// let test_str = "מָ";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-vowel-point)
    ///     assert!(!is_hbr_point(c));
    ///   } else {
    ///     assert!(is_hbr_point(c));
    ///   }
    /// }
    /// ```
    pub fn is_hbr_point(c: char) -> bool {
        is_hbr_point_vowel(c) || is_hbr_point_semi_vowel(c) || is_hbr_point_reading_sign(c)
    }
    /// Checks if the given character is a HBR vowel
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_vowel;
    ///
    /// let test_str = "מָ";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-vowel-point)
    ///     assert!(!is_hbr_point_vowel(c));
    ///   } else {
    ///     assert!(is_hbr_point_vowel(c));
    ///   }
    /// }
    /// ```
    pub fn is_hbr_point_vowel(c: char) -> bool {
        // 05B4 .. 05BB + 05C7
        matches!(c, '\u{05B4}'..='\u{05BB}' | '\u{05C7}')
    }
    /// Checks if the given character is a HBR semi-vowel
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_semi_vowel;
    ///
    /// let test_str = "מְזֱלֲשֳ";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-vowel-point)
    ///     assert!(!is_hbr_point_semi_vowel(c));
    ///   } else {
    ///     assert!(is_hbr_point_semi_vowel(c));
    ///   }
    /// }
    /// ```
    pub fn is_hbr_point_semi_vowel(c: char) -> bool {
        // 05B0 .. 05B3
        matches!(c, '\u{05B0}'..='\u{05B3}')
    }
    /// Checks if the given character is a HBR reading sign
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_reading_sign;
    ///
    /// //let test_str = "ׁהּשׂש";
    /// let test_str = "שׁהּשׂ";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-vowel-point)
    ///     assert!(!is_hbr_point_reading_sign(c));
    ///   } else {
    ///     assert!(is_hbr_point_reading_sign(c));
    ///   }
    /// }
    /// ```
    pub fn is_hbr_point_reading_sign(c: char) -> bool {
        // 05BC .. 05BD + 05BF + 05C1 .. 05C2
        matches!(c,
            '\u{05BC}'..='\u{05BD}'
            | '\u{05BF}'
            | '\u{05C1}'..='\u{05C2}'
        )
    }
    /// Checks if the given character is a HBR punctuation.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_punctuation;
    ///
    /// let test_str = "א־ר׃";
    /// for (position, c) in test_str.chars().enumerate() {
    ///   let position_u8 = u8::try_from(position).unwrap();
    ///   if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non_mark)
    ///     assert!(!is_hbr_punctuation(c));
    ///   } else {
    ///     assert!(is_hbr_punctuation(c));
    ///   }
    /// }
    /// ```
    pub fn is_hbr_punctuation(c: char) -> bool {
        // 05BE + 05C0 + 05C3 + 05C6 + 05F3 + 05F4
        matches!(
            c,
            '\u{05BE}' | '\u{05C0}' | '\u{05C3}' | '\u{05C6}' | '\u{05F3}' | '\u{05F4}'
        )
    }
    /// Checks if the given character is a HBR letter (final OR normal)
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant;
    ///
    /// let test_str = "אבגדהוזחטיכךלמםנןסעפףצץקרשת";
    /// for c in test_str.chars() {
    ///   assert!(is_hbr_consonant(c));
    /// }
    ///
    /// let test_str = "sl";
    /// for c in test_str.chars() {
    ///   assert!(!is_hbr_consonant(c));
    /// }
    /// ```
    pub fn is_hbr_consonant(c: char) -> bool {
        is_hbr_consonant_normal(c) || is_hbr_consonant_final(c)
    }
    /// Checks if the given character is a normal HBR consonant
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::{is_hbr_consonant_normal, is_hbr_consonant};
    ///
    /// let test_str = "אבגדהוזחטיכלמנסעפצקרשת";
    /// for c in test_str.chars() {
    ///     assert!(is_hbr_consonant_normal(c));
    ///     assert!(is_hbr_consonant(c));
    /// }
    /// ```
    pub fn is_hbr_consonant_normal(c: char) -> bool {
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
    /// Checks if the given character is a HBR letter (final).
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::{is_hbr_consonant_final, is_hbr_consonant};
    ///
    /// let test_str = "ךםןףץ";
    /// for c in test_str.chars() {
    ///     assert!(is_hbr_consonant_final(c));
    ///     assert!(is_hbr_consonant(c));
    /// }
    /// ```
    pub fn is_hbr_consonant_final(c: char) -> bool {
        // 05DA + 05DD + 05DF + 05E3 + 05E5
        matches!(
            c,
            '\u{05DA}' | '\u{05DD}' | '\u{05DF}' | '\u{05E3}' | '\u{05E5}'
        )
    }
    /// Checks if the given character is a HBR yod triangle.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_yod_triangle;
    ///
    /// let yod_triangle = '\u{05EF}';
    /// assert!(is_hbr_yod_triangle(yod_triangle));
    /// let non_yod_triangle = '\u{0031}';
    /// assert!(!is_hbr_yod_triangle(non_yod_triangle));
    /// ```
    pub fn is_hbr_yod_triangle(c: char) -> bool {
        matches!(c, '\u{05EF}')
    }
    /// Checks if the given character is a Yiddish ligature.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_hbr_ligature_yiddish;
    ///
    /// let test_str = "װױײ";
    /// for c in test_str.chars() {
    ///   assert!(is_hbr_ligature_yiddish(c));
    /// }    
    /// let test_str = "ythk";
    /// for c in test_str.chars() {
    ///   assert!(!is_hbr_ligature_yiddish(c));
    /// }
    /// ```
    pub fn is_hbr_ligature_yiddish(c: char) -> bool {
        // 05F0 .. 05F2
        matches!(c, '\u{05F0}'..='\u{05F2}')
    }
}

pub mod unicode_block_alphabetic_presentation_form {
    /// Checks if the given character belongs to the unicode block 'Alphabetic Presentation Form'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_block;
    ///
    /// let apf_ch = '\u{FB22}'; // HEBREW LETTER WIDE DALET
    /// assert!(is_apf_block(apf_ch));
    ///
    /// let non_apf_ch = '\u{FB13}'; // ARMENIAN SMALL LIGATURE MEN NOW
    /// assert!(!is_apf_block(non_apf_ch));
    /// ```
    pub fn is_apf_block(c: char) -> bool {
        is_apf_point_reading_sign(c)
            || is_apf_consonant(c)
            || is_apf_ligature_yiddisch(c)
            || is_apf_ligature(c)
    }
    /// Checks if the given character is an AFP point.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_point_reading_sign;
    ///
    /// let reading_sign = '\u{FB1E}';
    /// assert!(is_apf_point_reading_sign(reading_sign));
    /// ```
    pub fn is_apf_point_reading_sign(c: char) -> bool {
        // U+FB1E
        matches!(c, '\u{FB1E}')
    }
    /// Checks if the given character is an AFP letter.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant;
    ///
    /// let afp_l = '\u{FB3E}';
    /// assert!(is_apf_consonant(afp_l));
    ///
    /// let afp_l = 'p';
    /// assert!(!is_apf_consonant(afp_l));
    /// ```
    pub fn is_apf_consonant(c: char) -> bool {
        is_apf_consonant_alternative(c)
            || is_apf_consonant_wide(c)
            || is_apf_consonant_with_vowel(c)
    }
    /// Checks if the given character is an AFP an alternative letter.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_alternative;
    ///
    /// let afp_alternative = '\u{FB20}';
    /// assert!(is_apf_consonant_alternative(afp_alternative));
    ///
    /// let non_afp_alternative = 'p';
    /// assert!(!is_apf_consonant_alternative(non_afp_alternative));
    /// ```
    pub fn is_apf_consonant_alternative(c: char) -> bool {
        // U+FB20  + U+FB29
        matches!(c, '\u{FB20}' | '\u{FB29}')
    }
    /// Checks if the given character is an AFP wide letter.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide;
    ///
    /// let afp_wide = '\u{FB21}';
    /// assert!(is_apf_consonant_wide(afp_wide));
    ///
    /// let non_afp_wide = '\u{FB29}';
    /// assert!(!is_apf_consonant_wide(non_afp_wide));
    /// ```
    pub fn is_apf_consonant_wide(c: char) -> bool {
        // U+FB21  + U+FB28
        matches!(c, '\u{FB21}'..='\u{FB28}')
    }
    /// Checks if the given character is an AFP letter with a vowel
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_with_vowel;
    ///
    /// let with_vowel = '\u{FB3E}';
    /// assert!(is_apf_consonant_with_vowel(with_vowel));
    ///
    /// let non_with_vowel = 'X';
    /// assert!(!is_apf_consonant_with_vowel(non_with_vowel));
    /// ```
    pub fn is_apf_consonant_with_vowel(c: char) -> bool {
        // U+FB1D + (U+FB2A .. U+FB36) + (U+FB38 .. U+FB3C) + U+FB3E + (U+FB40 .. U+FB41)
        // + (U+FB43 .. U+FB44) + (U+FB46 .. U+FB4E)
        matches!(
            c,
            '\u{FB1D}'
            | '\u{FB2A}'..='\u{FB36}'
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
    /// let liga_yiddish = '\u{FB1F}';
    /// assert!(is_apf_ligature_yiddisch(liga_yiddish));
    ///
    /// // ARABIC LETTER TEHEH ISOLATED FORM
    /// let non_liga_yiddish = '\u{FB62}';
    ///   assert!(!is_apf_ligature_yiddisch(non_liga_yiddish));
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
    /// let liga = '\u{FB4F}';
    /// assert!(is_apf_ligature(liga));
    ///
    /// // HEBREW LIGATURE YIDDISH YOD YOD PATAH
    /// let non_liga = '\u{FB1F}';
    /// assert!(!is_apf_ligature(non_liga));
    /// ```
    pub fn is_apf_ligature(c: char) -> bool {
        // U+FB4F
        matches!(c, '\u{FB4F}')
    }
}

#[cfg(test)]
mod test_functions {
    use crate::function_api::unicode_block_alphabetic_presentation_form::*;
    use crate::function_api::unicode_block_hebrew::*;
    use crate::function_api::unicode_script_hebrew::*;

    // Unicode Script 'Hebrew'
    #[test]
    fn test_script_hebrew() {
        let letter_a = '\u{0041}'; // A
        assert!(!is_script_hbr(letter_a));

        let letter_aleph = '\u{05D0}'; // א HEBREW LETTER ALEF
        assert!(is_script_hbr(letter_aleph));

        let letter_aleph = '\u{FB1E}'; // HEBREW POINT JUDEO-SPANISH VARIKA
        assert!(is_script_hbr(letter_aleph));
    }

    #[test]
    fn test_script_hebrew_point() {
        let test_str = "מָ";
        for (position, c) in test_str.chars().enumerate() {
            let position_u8 = u8::try_from(position).unwrap();
            if position_u8 % 2 == 0 {
                // even position is a normal letter (non-vowel-point)
                assert!(!is_script_hbr_point(c));
            } else {
                assert!(is_script_hbr_point(c));
            }
        }
        let reading_sign = '\u{FB1E}';
        assert!(is_script_hbr_point(reading_sign));
    }

    #[test]
    fn test_script_hebrew_consonant() {
        let test_str = "אבגדהוזחטיכךלמםנןסעפףצץקרשת";
        for c in test_str.chars() {
            assert!(is_script_hbr_consonant(c));
        }
        let afp_alternative = '\u{FB20}';
        assert!(is_script_hbr_consonant(afp_alternative));
    }

    #[test]
    fn test_script_hebrew_ligature_yiddisch() {
        let test_str = "װױײ";
        for c in test_str.chars() {
            assert!(is_script_hbr_ligature_yiddisch(c));
        }
        let liga_yiddish = '\u{FB1F}';
        assert!(is_script_hbr_ligature_yiddisch(liga_yiddish));
    }

    // Unicode Block 'Hebrew'
    #[test]
    fn test_hbr_accent() {
        let test_str1 = "ב֑ב֒ב֓ב֔ב֕ב֖ב֗ב֘ב֙ב֚";
        let test_str2 = "ב֛ב֜ב֝ב֞ב֟ב֠ב֡ב֢ב֣ב֤";
        let test_str3 = "ב֥ב֦ב֧ב֨ב֩ב֪ב֫ב֬ב֭ב֮";
        check_accents(test_str1);
        check_accents(test_str2);
        check_accents(test_str3);
    }
    #[test]
    fn test_hbr_mark() {
        // 05AF + 05C4 + 05C5
        let test_str = "ב֯בׄבׅ";
        check_marks(test_str);
    }
    #[test]
    fn test_hbr_point() {
        let test_str = "בְבֱבֲבֳבִבֵבֶבַבָבֹבֺבֻבּבֽבֿבׁבׂבׇ";
        check_points(test_str);
    }
    #[test]
    fn test_hbr_point_vowel() {
        assert!(is_hbr_point_vowel('\u{05B5}'));
    }
    #[test]
    fn test_hbr_point_semi_vowel() {
        assert!(is_hbr_point_semi_vowel('\u{05B0}'));
    }
    #[test]
    fn test_hbr_point_reading_sign() {
        assert!(is_hbr_point_reading_sign('\u{05BF}'));
    }
    #[test]
    fn test_hbr_puncuation() {
        let test_str = "ב־ב׀ב׃ב׆ב׳נ״";
        check_punctuations(test_str);
    }
    #[test]
    fn test_hbr_consonant() {
        let test_str = "אבגדהוזחטיכךלמםנןסעפףצץקרשת";
        for c in test_str.chars() {
            assert!(is_hbr_consonant(c));
        }
    }
    #[test]
    fn test_hbr_consonant_not_final() {
        let test_str = "אבגדהוזחטיכלמנסעפצקרשת";
        for c in test_str.chars() {
            assert!(is_hbr_consonant(c));
            assert!(is_hbr_consonant_normal(c));
        }
    }
    #[test]
    fn test_hbr_consonant_final() {
        let test_str = "ךםןףץ";
        for c in test_str.chars() {
            assert!(is_hbr_consonant_final(c));
            assert!(is_hbr_consonant(c));
        }
    }
    #[test]
    fn test_hbr_yiddish_chars() {
        let test_str = "װױײ";
        for c in test_str.chars() {
            assert!(is_hbr_ligature_yiddish(c));
        }
    }
    #[test]
    fn test_hbr_yod_triangle() {
        let yod_triangle = '\u{05EF}';
        assert!(is_hbr_yod_triangle(yod_triangle));
    }

    // Unicode Block 'Alphabetic Presentation Form'

    #[test]
    fn test_apf_point() {
        let test_ch = '\u{FB1E}'; // HEBREW POINT JUDEO-SPANISH VARIKA

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));

        // unicode block Hebrew
        assert!(!is_hbr_block(test_ch));
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(is_apf_block(test_ch));
        assert!(is_apf_point_reading_sign(test_ch));
        assert!(!is_apf_consonant(test_ch));
        assert!(!is_apf_ligature_yiddisch(test_ch));
        assert!(!is_apf_ligature(test_ch));
    }

    #[test]
    fn test_apf_consonant() {
        let test_ch = '\u{FB1D}'; // HEBREW LETTER YOD WITH HIRIQ

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));

        // unicode block 'Hebrew'
        assert!(!is_hbr_block(test_ch));
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(is_apf_block(test_ch));
        assert!(!is_apf_point_reading_sign(test_ch));
        assert!(is_apf_consonant(test_ch));
        assert!(!is_apf_ligature_yiddisch(test_ch));
        assert!(!is_apf_ligature(test_ch));
    }

    #[test]
    fn test_apf_ligature_yiddisch() {
        let test_ch = '\u{FB1F}'; // HEBREW LIGATURE YIDDISH YOD YOD PATAH

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));
        // unicode block 'Hebrew'
        assert!(!is_hbr_block(test_ch));
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(is_apf_block(test_ch));
        assert!(!is_apf_point_reading_sign(test_ch));
        assert!(!is_apf_consonant(test_ch));
        assert!(is_apf_ligature_yiddisch(test_ch));
        assert!(!is_apf_ligature(test_ch));
    }

    #[test]
    fn test_apf_ligature() {
        let test_ch = '\u{FB4F}'; // HEBREW LETTER WIDE DALET

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));

        // unicode block 'Hebrew'
        assert!(!is_hbr_block(test_ch));
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(is_apf_block(test_ch));
        assert!(!is_apf_point_reading_sign(test_ch));
        assert!(!is_apf_consonant(test_ch));
        assert!(!is_apf_ligature_yiddisch(test_ch));
        assert!(is_apf_ligature(test_ch));
    }

    // supporting functions
    fn check_accents(str: &str) {
        // str is an alternation of consonant-accent-consonat-accent ...
        for (position, c) in str.chars().enumerate() {
            let position_u8 = u8::try_from(position).unwrap();
            if position_u8 % 2 == 0 {
                //println!("fn check_accents::at even position {} is: {}", pos_u8,c.escape_default());
                assert!(is_hbr_consonant(c));
            } else {
                //println!("fn check_accents::at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_hbr_accent(c));
            }
        }
    }
    fn check_marks(str: &str) {
        for (pos, c) in str.chars().enumerate() {
            let pos_u8 = u8::try_from(pos).unwrap();
            if pos_u8 % 2 == 0 {
                //println!("fn check_marks::at even position {} is: {}", pos_u8, c.escape_default());
                assert!(is_hbr_consonant(c));
            } else {
                //println!("fn check_marks::at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_hbr_mark(c));
            }
        }
    }
    fn check_points(str: &str) {
        for (pos, c) in str.chars().enumerate() {
            let pos_u8 = u8::try_from(pos).unwrap();
            if pos_u8 % 2 == 0 {
                //println!("fn check_points::char at even position {} is: {}", pos_u8, c.escape_default());
                assert!(is_hbr_consonant(c));
            } else {
                //println!("fn check_points::char at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_hbr_point(c));
            }
        }
    }
    fn check_punctuations(str: &str) {
        for (pos, c) in str.chars().enumerate() {
            let pos_u8 = u8::try_from(pos).unwrap();
            if pos_u8 % 2 == 0 {
                //println!("fn check_punctuations::char at even position {} is: {}", pos_u8, c.escape_default());
                assert!(is_hbr_consonant(c));
            } else {
                //println!("fn check_punctuations::char at odd position {} is: {}", pos_u8, c.escape_default());
                assert!(is_hbr_punctuation(c));
            }
        }
    }
}
