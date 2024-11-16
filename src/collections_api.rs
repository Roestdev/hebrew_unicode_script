pub mod unicode_script_hebrew {
    use crate::*;
    /// Checks if the given character belongs to the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr;
    /// let hebrew_ch = 'ל';
    /// assert!(is_script_hbr(hebrew_ch));
    ///
    /// let non_hbr_ch = 'д';
    /// assert!(!is_script_hbr(non_hbr_ch));
    /// ```
    pub fn is_script_hbr(c: char) -> bool {
        is_hbr_block(c) || is_apf_block(c)
    }
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
    /// Checks if the given character is a 'point' type within the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr_point_reading_sign;
    ///
    /// //let test_str = "מָ";
    /// //for (position, c) in test_str.chars().enumerate() {
    ///   //let position_u8 = u8::try_from(position).unwrap();
    ///   //if position_u8 % 2 == 0 {
    ///     // even position is a normal letter (non-vowel-point)
    ///    // assert!(!is_script_hbr_point(c));
    ///  // } else {
    ///  //   assert!(is_script_hbr_point(c));
    ///  // }
    ///// }
    /// let reading_sign = '\u{FB1E}';
    /// assert!(is_script_hbr_point_reading_sign(reading_sign));
    /// ```
    pub fn is_script_hbr_point_reading_sign(c: char) -> bool {
        is_hbr_point_reading_sign(c) || is_apf_point_reading_sign(c)
    }

    /// Checks if the given character is a 'ligature' type within the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr_ligature;
    ///
    /// let test_str = "װױײ";
    /// for c in test_str.chars() {
    ///   assert!(is_script_hbr_ligature(c));
    /// }
    /// let liga_yiddish = '\u{FB1F}';
    /// assert!(is_script_hbr_ligature(liga_yiddish));
    /// ```
    pub fn is_script_hbr_ligature(c: char) -> bool {
        is_hbr_ligature_yiddish(c) || is_apf_ligature(c)
    }

    /// Checks if the given character is a 'ligature_yiddisch' type within the unicode script 'Hebrew'.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_script_hbr_ligature_yiddisch;
    ///
    /// let test_str = "װױײ";
    /// for c in test_str.chars() {
    ///   assert!(is_script_hbr_ligature_yiddisch(c));
    /// }
    /// let liga_yiddish = '\u{FB1F}';
    /// assert!(is_script_hbr_ligature_yiddisch(liga_yiddish));
    /// ```
    pub fn is_script_hbr_ligature_yiddisch(c: char) -> bool {
        is_hbr_ligature_yiddish(c) || is_apf_ligature_yiddisch_yod_yod_patah(c)
    }
}

pub mod unicode_block_hebrew {
    use crate::*;
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
    /// Checks if the given character is a HBR accent.
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
        //matches!(c, '\u{0591}'..='\u{05AE}')
        is_hbr_accent_etnahta(c)
            || is_hbr_accent_segol(c)
            || is_hbr_accent_shalshelet(c)
            || is_hbr_accent_zaqef_qatan(c)
            || is_hbr_accent_zaqef_gadol(c)
            || is_hbr_accent_tipeha(c)
            || is_hbr_accent_revia(c)
            || is_hbr_accent_zarqa(c)
            || is_hbr_accent_pashta(c)
            || is_hbr_accent_yetiv(c)
            || is_hbr_accent_tevir(c)
            || is_hbr_accent_geresh(c)
            || is_hbr_accent_geresh_muqdam(c)
            || is_hbr_accent_gershayim(c)
            || is_hbr_accent_qarney_para(c)
            || is_hbr_accent_telisha_gedola(c)
            || is_hbr_accent_pazer(c)
            || is_hbr_accent_atnah_hafukh(c)
            || is_hbr_accent_munah(c)
            || is_hbr_accent_mahapakh(c)
            || is_hbr_accent_merkha(c)
            || is_hbr_accent_merkha_kefula(c)
            || is_hbr_accent_darga(c)
            || is_hbr_accent_qadma(c)
            || is_hbr_accent_telisha_qetana(c)
            || is_hbr_accent_yerah_ben_yomo(c)
            || is_hbr_accent_ole(c)
            || is_hbr_accent_iluy(c)
            || is_hbr_accent_dehi(c)
            || is_hbr_accent_zinor(c)
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
        is_hbr_mark_lower_dot(c) || is_hbr_mark_upper_dot(c) || is_hbr_mark_masora_circle(c)
    }
    /// Checks if the given character is a HBR point.
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
    /// Checks if the given character is a HBR point vowel.
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
        //matches!(c, '\u{05B4}'..='\u{05BB}' | '\u{05C7}')
        is_hbr_point_hiriq(c)
            || is_hbr_point_tsere(c)
            || is_hbr_point_segol(c)
            || is_hbr_point_patah(c)
            || is_hbr_point_qamats(c)
            || is_hbr_point_holam(c)
            || is_hbr_point_holam_haser_for_vav(c)
            || is_hbr_point_qubuts(c)
            || is_hbr_point_qamats_qatan(c)
    }
    /// Checks if the given character is a HBR point semi-vowel.
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
        is_hbr_point_sheva(c)
            || is_hbr_point_hataf_segol(c)
            || is_hbr_point_hataf_patah(c)
            || is_hbr_point_hataf_qamats(c)
    }
    /// Checks if the given character is a HBR reading sign.
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
        is_hbr_point_dagesh_or_mapiq(c)
            || is_hbr_point_meteg(c)
            || is_hbr_point_rafe(c)
            || is_hbr_point_shin_dot(c)
            || is_hbr_point_sin_dot(c)
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
        is_hbr_punctuation_maqaf(c)
            || is_hbr_punctuation_paseq(c)
            || is_hbr_punctuation_sof_pasuq(c)
            || is_hbr_punctuation_nun_hafukha(c)
            || is_hbr_punctuation_geresh(c)
            || is_hbr_punctuation_gershayim(c)
    }
    /// Checks if the given character is a HBR consonant (final OR normal)
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
    /// Checks if the given character is a HBR consonant normal.
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
        is_hbr_consonant_alef(c)
            || is_hbr_consonant_bet(c)
            || is_hbr_consonant_gimel(c)
            || is_hbr_consonant_dalet(c)
            || is_hbr_consonant_he(c)
            || is_hbr_consonant_vav(c)
            || is_hbr_consonant_zayin(c)
            || is_hbr_consonant_het(c)
            || is_hbr_consonant_tet(c)
            || is_hbr_consonant_yod(c)
            || is_hbr_consonant_kaf(c)
            || is_hbr_consonant_lamed(c)
            || is_hbr_consonant_mem(c)
            || is_hbr_consonant_nun(c)
            || is_hbr_consonant_samekh(c)
            || is_hbr_consonant_ayin(c)
            || is_hbr_consonant_pe(c)
            || is_hbr_consonant_tsadi(c)
            || is_hbr_consonant_qof(c)
            || is_hbr_consonant_resh(c)
            || is_hbr_consonant_shin(c)
            || is_hbr_consonant_tav(c)
    }
    /// Checks if the given character is a HBR consonant final.
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
        is_hbr_consonant_final_kaf(c)
            || is_hbr_consonant_final_mem(c)
            || is_hbr_consonant_final_nun(c)
            || is_hbr_consonant_final_pe(c)
            || is_hbr_consonant_final_tsadi(c)
    }

    /// Checks if the given character is a HBR Yiddish ligature.
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
        is_hbr_ligature_yiddisch_double_vav(c)
            || is_hbr_ligature_yiddisch_double_yod(c)
            || is_hbr_ligature_yiddisch_vav_yod(c)
    }
}

pub mod unicode_block_alphabetic_presentation_form {
    use crate::*;
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
        is_apf_consonant_with_vowel(c)
            || is_apf_point_reading_sign(c)
            || is_apf_ligature(c)
            || is_apf_alternative(c)
            || is_apf_consonant_wide(c)
    }
    /// Checks if the given character is an AFP consonant.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant;
    ///
    /// let alternative = '\u{FB20}';
    /// assert!(is_apf_consonant(alternative));
    ///
    /// let non_alternative = 'p';
    /// assert!(!is_apf_consonant(non_alternative));
    /// ```
    pub fn is_apf_consonant(c: char) -> bool {
        is_apf_consonant_wide(c) || is_apf_consonant_alternative_ayin(c)
    }

    /*
     *   apf consonant with a vowel(s)-(33x)
     */
    /// Checks if the given character is an APF letter with vowel sign
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
        is_apf_consonant_vowel_yod_hiriq(c)
            || is_apf_consonant_vowel_shin_shindot(c)
            || is_apf_consonant_vowel_shin_sindot(c)
            || is_apf_consonant_vowel_shin_dagesh_shindot(c)
            || is_apf_consonant_vowel_shin_dagesh_sindot(c)
            || is_apf_consonant_vowel_alef_patah(c)
            || is_apf_consonant_vowel_alef_qamats(c)
            || is_apf_consonant_vowel_alef_mapiq(c)
            || is_apf_consonant_vowel_bet_dagesh(c)
            || is_apf_consonant_vowel_gimmel_dagesh(c)
            || is_apf_consonant_vowel_dalet_dagesh(c)
            || is_apf_consonant_vowel_he_mapiq(c)
            || is_apf_consonant_vowel_vav_dagesh(c)
            || is_apf_consonant_vowel_zayin_dagesh(c)
            || is_apf_consonant_vowel_tet_dagesh(c)
            || is_apf_consonant_vowel_yod_dagesh(c)
            || is_apf_consonant_vowel_final_kaf_dagesh(c)
            || is_apf_consonant_vowel_kaf_dagesh(c)
            || is_apf_consonant_vowel_lamed_dagesh(c)
            || is_apf_consonant_vowel_mem_dagesh(c)
            || is_apf_consonant_vowel_nun_dagesh(c)
            || is_apf_consonant_vowel_samekh_dagesh(c)
            || is_apf_consonant_vowel_final_pe_dagesh(c)
            || is_apf_consonant_vowel_pe_dagesh(c)
            || is_apf_consonant_vowel_tsadi_dagesh(c)
            || is_apf_consonant_vowel_qof_dagesh(c)
            || is_apf_consonant_vowel_resh_dagesh(c)
            || is_apf_consonant_vowel_shin_dagesh(c)
            || is_apf_consonant_vowel_tav_dagesh(c)
            || is_apf_consonant_vowel_vav_holam(c)
            || is_apf_consonant_vowel_bet_rafe(c)
            || is_apf_consonant_vowel_kaf_rafe(c)
            || is_apf_consonant_vowel_pe_rafe(c)
    }

    /*
     *   apf point - (1x)
     */
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
        is_apf_point_judeo_spanish_varika(c)
    }

    /*
        apf ligatures - (2x)
    */
    /// Checks if the given character is an APF ligature.
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_ligature;
    ///
    /// // HEBREW LIGATURE ALEF LAMED
    /// let liga = 'ﭏ';
    /// assert!(is_apf_ligature(liga));
    ///
    /// ```
    pub fn is_apf_ligature(c: char) -> bool {
        // U+FB4F + U+FB4F
        is_apf_ligature_yiddisch_yod_yod_patah(c) || is_apf_ligature_alef_lamed(c)
    }

    /*
     *   apf alternative -(2x)
     */
    /// Checks if the given character is an AFP alternative letter.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_unicode_script::is_apf_alternative;
    ///
    /// let afp_alternative = 'ﬠ';
    /// assert!(is_apf_alternative(afp_alternative));
    ///
    /// ```
    pub fn is_apf_alternative(c: char) -> bool {
        // U+FB20  +  U+FB29
        is_apf_consonant_alternative_ayin(c) || is_apf_letter_alternative_plus_sign(c)
    }

    /*
     *    apf consonant wide - (8x)
     */
    /// Checks if the given character is an AFP wide consonant.
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
        // U+FB21 .. U+FB28
        is_apf_consonant_wide_alef(c)
            || is_apf_consonant_wide_dalet(c)
            || is_apf_consonant_wide_he(c)
            || is_apf_consonant_wide_kaf(c)
            || is_apf_consonant_wide_lamed(c)
            || is_apf_consonant_wide_final_mem(c)
            || is_apf_consonant_wide_resh(c)
            || is_apf_consonant_wide_tav(c)
    }
}
#[cfg(test)]
mod test_functions {
    use crate::*;
    // Unicode Script 'Hebrew'
    #[test]
    fn test_script_hebrew() {
        let letter_a = '\u{0041}'; // a
        assert!(!is_script_hbr(letter_a));

        let letter_aleph = '\u{05D0}'; // א HEBREW LETTER ALEF
        assert!(is_script_hbr(letter_aleph));

        let letter_aleph = '\u{FB1E}'; // HEBREW POINT JUDEO-SPaNISH VARIKA
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
        // TODO
        //let test_str = "בְבֱבֲבֳבִבֵבֶבַבָבֹבֺבֻבּבֽבֿבׁבׂבׇ";
        let test_str = "אַ";
        check_points(test_str);
    }
    #[test]
    fn test_hbr_point_vowel() {
        // TODO
        assert!(is_hbr_point_vowel('ֶ'));
    }
    #[test]
    fn test_hbr_point_semi_vowel() {
        assert!(is_hbr_point_semi_vowel('ְ'));
    }
    #[test]
    fn test_hbr_point_reading_sign() {
        assert!(is_hbr_point_reading_sign('ֽ')); // todo
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
    fn test_apf_consonant() {
        let test_ch = '\u{FB20}'; // HEBREW LETTER ALTERNATIVE AYIN

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));
        assert!(!is_script_hbr_point(test_ch));
        assert!(!is_script_hbr_point_reading_sign(test_ch));
        assert!(is_script_hbr_consonant(test_ch));
        assert!(!is_script_hbr_ligature_yiddisch(test_ch));
        assert!(!is_script_hbr_ligature(test_ch));

        // unicode block Hebrew
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(!is_apf_consonant_with_vowel(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_apf_ligature(test_ch));
        assert!(is_apf_alternative(test_ch));
        assert!(!is_apf_consonant_wide(test_ch));
    }

    #[test]
    fn test_apf_consonant_with_vowel() {
        let test_ch = '\u{FB1D}'; // HEBREW LETTER YOD WITH HIRIQ

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));
        assert!(!is_script_hbr_point(test_ch));
        assert!(!is_script_hbr_point_reading_sign(test_ch));
        assert!(!is_script_hbr_consonant(test_ch));
        assert!(!is_script_hbr_ligature_yiddisch(test_ch));
        assert!(!is_script_hbr_ligature(test_ch));

        // unicode block Hebrew
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(is_apf_consonant_with_vowel(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_apf_ligature(test_ch));
        assert!(!is_apf_alternative(test_ch));
        assert!(!is_apf_consonant_wide(test_ch));
    }

    #[test]
    fn test_apf_point_reading_sign() {
        // TODO
        let test_ch = '\u{FB1E}'; // HEBREW POINT JUDEO-SPANISH VARIKA

        // unicode script 'Hebrew'src/function_api.rs:893:9
        assert!(!is_script_hbr_ligature_yiddisch(test_ch));
        assert!(!is_script_hbr_ligature(test_ch));

        // unicode block Hebrew
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_point_vowel(test_ch));
        assert!(!is_hbr_point_semi_vowel(test_ch));
        assert!(!is_hbr_point_reading_sign(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant_normal(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(!is_apf_consonant_with_vowel(test_ch));
        assert!(is_apf_point_reading_sign(test_ch));
        assert!(!is_apf_ligature(test_ch));
        assert!(!is_apf_alternative(test_ch));
        assert!(!is_apf_consonant_wide(test_ch));
    }

    #[test]
    fn test_apf_ligature() {
        // TODO
        let test_ch = '\u{FB4F}'; // HEBREW LIGATURE ALEF LAMED

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));
        assert!(!is_script_hbr_point(test_ch));
        assert!(!is_script_hbr_point_reading_sign(test_ch));
        assert!(!is_script_hbr_consonant(test_ch));
        assert!(!is_script_hbr_ligature_yiddisch(test_ch));
        assert!(is_script_hbr_ligature(test_ch));

        // unicode block Hebrew
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(!is_apf_consonant_with_vowel(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(is_apf_ligature(test_ch));
        assert!(!is_apf_alternative(test_ch));
        assert!(!is_apf_consonant_wide(test_ch));
    }

    #[test]
    fn test_apf_alternative() {
        // TODO
        let test_ch = '\u{FB29}'; // HEBREW LETTER ALTERNATIVE PLUS SIGN

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));
        assert!(!is_script_hbr_point(test_ch));
        assert!(!is_script_hbr_point_reading_sign(test_ch));
        assert!(!is_script_hbr_consonant(test_ch));
        assert!(!is_script_hbr_ligature_yiddisch(test_ch));
        assert!(!is_script_hbr_ligature(test_ch));

        // unicode block Hebrew
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(!is_apf_consonant_with_vowel(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_apf_ligature(test_ch));
        assert!(is_apf_alternative(test_ch));
        assert!(!is_apf_consonant_wide(test_ch));
    }

    #[test]
    fn test_apf_consonant_wide() {
        let test_ch = '\u{FB28}'; // HEBREW LETTER WIDE TAV

        // unicode script 'Hebrew'
        assert!(is_script_hbr(test_ch));
        assert!(!is_script_hbr_point(test_ch));
        assert!(!is_script_hbr_point_reading_sign(test_ch));
        assert!(is_script_hbr_consonant(test_ch));
        assert!(!is_script_hbr_ligature_yiddisch(test_ch));
        assert!(!is_script_hbr_ligature(test_ch));

        // unicode block Hebrew
        assert!(!is_hbr_accent(test_ch));
        assert!(!is_hbr_mark(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_hbr_punctuation(test_ch));
        assert!(!is_hbr_consonant_final(test_ch));
        assert!(!is_hbr_consonant(test_ch));
        assert!(!is_hbr_yod_triangle(test_ch));
        assert!(!is_hbr_ligature_yiddish(test_ch));

        // unicode block 'Alphabetic Presentation Form'
        assert!(!is_apf_consonant_with_vowel(test_ch));
        assert!(!is_hbr_point(test_ch));
        assert!(!is_apf_ligature(test_ch));
        assert!(!is_apf_alternative(test_ch));
        assert!(is_apf_consonant_wide(test_ch));
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
