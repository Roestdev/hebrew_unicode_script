pub mod apf_consonant_wide {
    /// Checks if the given character is a APF consonant: wide alef.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_alef;
    ///
    /// let consonant = 'ﬡ';
    /// assert!(is_apf_consonant_wide_alef(consonant));
    /// ```
    pub fn is_apf_consonant_wide_alef(c: char) -> bool {
        // U+FB21
        matches!(c, '\u{FB21}')
    }
    /// Checks if the given character is a APF consonant: wide dalet.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_dalet;
    ///
    /// let consonant = 'ﬢ';
    /// assert!(is_apf_consonant_wide_dalet(consonant));
    /// ```
    pub fn is_apf_consonant_wide_dalet(c: char) -> bool {
        // U+FB22
        matches!(c, '\u{FB22}')
    }
    /// Checks if the given character is a APF consonant: wide he.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_he;
    ///
    /// let consonant = 'ﬣ';
    /// assert!(is_apf_consonant_wide_he(consonant));
    /// ```
    pub fn is_apf_consonant_wide_he(c: char) -> bool {
        // U+FB23
        matches!(c, '\u{FB23}')
    }
    /// Checks if the given character is a APF consonant: wide kaf.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_kaf;
    ///
    /// let consonant = 'ﬤ';
    /// assert!(is_apf_consonant_wide_kaf(consonant));
    /// ```
    pub fn is_apf_consonant_wide_kaf(c: char) -> bool {
        // U+FB24
        matches!(c, '\u{FB24}')
    }
    /// Checks if the given character is a APF consonant: wide lamed.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_lamed;
    ///
    /// let consonant = 'ﬥ';
    /// assert!(is_apf_consonant_wide_lamed(consonant));
    /// ```
    pub fn is_apf_consonant_wide_lamed(c: char) -> bool {
        // U+FB25
        matches!(c, '\u{FB25}')
    }
    /// Checks if the given character is a APF consonant wide: final-mem.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_final_mem;
    ///
    /// let consonant = 'ﬦ';
    /// assert!(is_apf_consonant_wide_final_mem(consonant));
    /// ```
    pub fn is_apf_consonant_wide_final_mem(c: char) -> bool {
        // U+FB26
        matches!(c, '\u{FB26}')
    }
    /// Checks if the given character is a APF consonant: wide resh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_resh;
    ///
    /// let consonant = 'ﬧ';
    /// assert!(is_apf_consonant_wide_resh(consonant));
    /// ```
    pub fn is_apf_consonant_wide_resh(c: char) -> bool {
        // U+FB27
        matches!(c, '\u{FB27}')
    }
    /// Checks if the given character is a APF consonant: with tav.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_wide_tav;
    ///
    /// let consonant = 'ﬨ';
    /// assert!(is_apf_consonant_wide_tav(consonant));
    /// ```
    pub fn is_apf_consonant_wide_tav(c: char) -> bool {
        // U+FB28
        matches!(c, '\u{FB28}')
    }
}
pub mod apf_consonant_with_vowel {
    /// Checks if the character is a APF consonant with vowel: yod & hiriq.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_yod_hiriq;
    ///
    /// let consonant = 'יִ';
    /// assert!(is_apf_consonant_vowel_yod_hiriq(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_yod_hiriq(c: char) -> bool {
        // U+FB1D
        matches!(c, '\u{FB1D}')
    }
    /// Checks if the character is a APF consonant with vowel: shin & shindot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_shin_shindot;
    ///
    /// let consonant = 'שׁ';
    /// assert!(is_apf_consonant_vowel_shin_shindot(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_shin_shindot(c: char) -> bool {
        // U+FB2A
        matches!(c, '\u{FB2A}')
    }
    /// Checks if the character is a APF consonant with vowel: shin & sindot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_shin_sindot;
    ///
    /// let consonant = 'שׂ';
    /// assert!(is_apf_consonant_vowel_shin_sindot(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_shin_sindot(c: char) -> bool {
        // U+FB2B
        matches!(c, '\u{FB2B}')
    }
    /// Checks if the character is a APF consonant with vowel: shin & dagesh & shindot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_shin_dagesh_shindot;
    ///
    /// let consonant = 'שּׁ';
    /// assert!(is_apf_consonant_vowel_shin_dagesh_shindot(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_shin_dagesh_shindot(c: char) -> bool {
        // U+FB2C
        matches!(c, '\u{FB2C}')
    }
    /// Checks if the character is a APF consonant with vowel: shin & dagesh & sindot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_shin_dagesh_sindot;
    ///
    /// let consonant = 'שּׂ';
    /// assert!(is_apf_consonant_vowel_shin_dagesh_sindot(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_shin_dagesh_sindot(c: char) -> bool {
        // U+FB2D
        matches!(c, '\u{FB2D}')
    }
    /// Checks if the given character is a APF consonant with vowel: alef & patah.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_alef_patah;
    ///
    /// let consonant = 'אַ';
    /// assert!(is_apf_consonant_vowel_alef_patah(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_alef_patah(c: char) -> bool {
        // U+FB2E
        matches!(c, '\u{FB2E}')
    }
    /// Checks if the given character is a APF consonant with vowel: alef & qamats.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_alef_qamats;
    ///
    /// let consonant = 'אָ';
    /// assert!(is_apf_consonant_vowel_alef_qamats(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_alef_qamats(c: char) -> bool {
        // U+FB2F
        matches!(c, '\u{FB2F}')
    }
    /// Checks if the given character is a APF consonant with vowel: alef & mapiq.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_alef_mapiq;
    ///
    /// let consonant = 'אּ';
    /// assert!(is_apf_consonant_vowel_alef_mapiq(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_alef_mapiq(c: char) -> bool {
        // U+FB30
        matches!(c, '\u{FB30}')
    }
    /// Checks if the given character is a APF consonant with vowel: bet & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_bet_dagesh;
    ///
    /// let consonant = 'בּ';
    /// assert!(is_apf_consonant_vowel_bet_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_bet_dagesh(c: char) -> bool {
        // U+FB31
        matches!(c, '\u{FB31}')
    }
    /// Checks if the given character is a APF consonant with vowel: gimel & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_gimmel_dagesh;
    ///
    /// let consonant = 'גּ';
    /// assert!(is_apf_consonant_vowel_gimmel_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_gimmel_dagesh(c: char) -> bool {
        // U+FB32
        matches!(c, '\u{FB32}')
    }
    /// Checks if the given character is a APF consonant with vowel: dalet & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_dalet_dagesh;
    ///
    /// let consonant = 'דּ';
    /// assert!(is_apf_consonant_vowel_dalet_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_dalet_dagesh(c: char) -> bool {
        // U+FB33
        matches!(c, '\u{FB33}')
    }
    /// Checks if the given character is a APF consonant with vowel: he & mapiq.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_he_mapiq;
    ///
    /// let consonant = 'הּ';
    /// assert!(is_apf_consonant_vowel_he_mapiq(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_he_mapiq(c: char) -> bool {
        // U+FB34
        matches!(c, '\u{FB34}')
    }
    /// Checks if the given character is a APF consonant with vowel: vav & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_vav_dagesh;
    ///
    /// let consonant = 'וּ';
    /// assert!(is_apf_consonant_vowel_vav_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_vav_dagesh(c: char) -> bool {
        // U+FB35
        matches!(c, '\u{FB35}')
    }
    /// Checks if the given character is a APF consonant with vowel: zayin & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_zayin_dagesh;
    ///
    /// let consonant = 'זּ';
    /// assert!(is_apf_consonant_vowel_zayin_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_zayin_dagesh(c: char) -> bool {
        // U+FB36
        matches!(c, '\u{FB36}')
    }
    /// Checks if the given character is a APF consonant with vowel: tet & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_tet_dagesh;
    ///
    /// let consonant = 'טּ';
    /// assert!(is_apf_consonant_vowel_tet_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_tet_dagesh(c: char) -> bool {
        // U+FB38
        matches!(c, '\u{FB38}')
    }
    /// Checks if the given character is a APF consonant with vowel: yod & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_yod_dagesh;
    ///
    /// let consonant = 'יּ';
    /// assert!(is_apf_consonant_vowel_yod_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_yod_dagesh(c: char) -> bool {
        // U+FB39
        matches!(c, '\u{FB39}')
    }
    /// Checks if the given character is a APF consonant with vowel: final-kaf & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_final_kaf_dagesh;
    ///
    /// let consonant = 'ךּ';
    /// assert!(is_apf_consonant_vowel_final_kaf_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_final_kaf_dagesh(c: char) -> bool {
        // U+FB3A
        matches!(c, '\u{FB3A}')
    }
    /// Checks if the given character is a APF consonant with vowel: kaf & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_kaf_dagesh;
    ///
    /// let consonant = 'כּ';
    /// assert!(is_apf_consonant_vowel_kaf_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_kaf_dagesh(c: char) -> bool {
        // U+FB3B
        matches!(c, '\u{FB3B}')
    }
    /// Checks if the given character is a APF consonant with vowel: lamed & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_lamed_dagesh;
    ///
    /// let consonant = 'לּ';
    /// assert!(is_apf_consonant_vowel_lamed_dagesh(consonant));
    ///
    /// ```
    pub fn is_apf_consonant_vowel_lamed_dagesh(c: char) -> bool {
        // U+FB3C
        matches!(c, '\u{FB3C}')
    }
    /// Checks if the given character is a APF consonant with vowel: mem & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_mem_dagesh;
    ///
    /// let consonant = 'מּ';
    /// assert!(is_apf_consonant_vowel_mem_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_mem_dagesh(c: char) -> bool {
        // U+FB3E
        matches!(c, '\u{FB3E}')
    }
    /// Checks if the given character is a APF consonant with vowel: nun & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_nun_dagesh;
    ///
    /// let consonant = 'נּ';
    /// assert!(is_apf_consonant_vowel_nun_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_nun_dagesh(c: char) -> bool {
        // U+FB40
        matches!(c, '\u{FB40}')
    }
    /// Checks if the given character is a APF consonant with vowel: samekh & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_samekh_dagesh;
    ///
    /// let consonant = 'סּ';
    /// assert!(is_apf_consonant_vowel_samekh_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_samekh_dagesh(c: char) -> bool {
        // U+FB41
        matches!(c, '\u{FB41}')
    }
    /// Checks if the given character is a APF consonant with vowel: final-pe & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_final_pe_dagesh;
    ///
    /// let consonant = 'ףּ';
    /// assert!(is_apf_consonant_vowel_final_pe_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_final_pe_dagesh(c: char) -> bool {
        // U+FB43
        matches!(c, '\u{FB43}')
    }
    /// Checks if the given character is a APF consonant with vowel: pe & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_pe_dagesh;
    ///
    /// let consonant = 'פּ';
    /// assert!(is_apf_consonant_vowel_pe_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_pe_dagesh(c: char) -> bool {
        // U+FB44
        matches!(c, '\u{FB44}')
    }
    /// Checks if the given character is a APF consonant with vowel: tsadi & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_tsadi_dagesh;
    ///
    /// let consonant = 'צּ';
    /// assert!(is_apf_consonant_vowel_tsadi_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_tsadi_dagesh(c: char) -> bool {
        // U+FB46
        matches!(c, '\u{FB46}')
    }
    /// Checks if the given character is a APF consonant with vowel: qof & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_qof_dagesh;
    ///
    /// let consonant = 'קּ';
    /// assert!(is_apf_consonant_vowel_qof_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_qof_dagesh(c: char) -> bool {
        // U+FB47
        matches!(c, '\u{FB47}')
    }
    /// Checks if the given character is a APF consonant with vowel: resh & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_resh_dagesh;
    ///
    /// let consonant = 'רּ';
    /// assert!(is_apf_consonant_vowel_resh_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_resh_dagesh(c: char) -> bool {
        // U+FB48
        matches!(c, '\u{FB48}')
    }
    /// Checks if the given character is a APF consonant with vowel: shin & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_shin_dagesh;
    ///
    /// let consonant = 'שּ';
    /// assert!(is_apf_consonant_vowel_shin_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_shin_dagesh(c: char) -> bool {
        // U+FB49
        matches!(c, '\u{FB49}')
    }
    /// Checks if the given character is a APF consonant with vowel: tav & dagesh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_tav_dagesh;
    ///
    /// let consonant = 'תּ';
    /// assert!(is_apf_consonant_vowel_tav_dagesh(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_tav_dagesh(c: char) -> bool {
        // U+FB4A
        matches!(c, '\u{FB4A}')
    }
    /// Checks if the given character is a APF consonant with vowel: vav & holam.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_vav_holam;
    ///
    /// let consonant = 'וֹ';
    /// assert!(is_apf_consonant_vowel_vav_holam(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_vav_holam(c: char) -> bool {
        // U+FB4B
        matches!(c, '\u{FB4B}')
    }
    /// Checks if the given character is a APF consonant with vowel:bet & rafe.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_bet_rafe;
    ///
    /// let consonant = 'בֿ';
    /// assert!(is_apf_consonant_vowel_bet_rafe(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_bet_rafe(c: char) -> bool {
        // U+FB4C
        matches!(c, '\u{FB4C}')
    }
    /// Checks if the given character is a APF consonant with vowel:  kaf & rafe.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_kaf_rafe;
    ///
    /// let consonant = 'כֿ';
    /// assert!(is_apf_consonant_vowel_kaf_rafe(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_kaf_rafe(c: char) -> bool {
        // U+FB4D
        matches!(c, '\u{FB4D}')
    }
    /// Checks if the given character is a APF consonant with vowel: pe & rafe.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_vowel_pe_rafe;
    ///
    /// let consonant = 'פֿ';
    /// assert!(is_apf_consonant_vowel_pe_rafe(consonant));
    /// ```
    pub fn is_apf_consonant_vowel_pe_rafe(c: char) -> bool {
        // U+FB4E
        matches!(c, '\u{FB4E}')
    }
}
pub mod apf_letter_alternative {
    /// Checks if the given character is a APF consonant: alternative ayin.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_consonant_alternative_ayin;
    ///
    /// let consonant = 'ﬠ';
    /// assert!(is_apf_consonant_alternative_ayin(consonant));
    /// ```
    pub fn is_apf_consonant_alternative_ayin(c: char) -> bool {
        // U+FB20
        matches!(c, '\u{FB20}')
    }
    /// Checks if the given character is a APF letter: alternative plus sign.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_letter_alternative_plus_sign;
    ///
    /// let consonant = '﬩';
    /// assert!(is_apf_letter_alternative_plus_sign(consonant));
    /// ```
    pub fn is_apf_letter_alternative_plus_sign(c: char) -> bool {
        // U+FB29
        matches!(c, '\u{FB29}')
    }
}
pub mod apf_ligature {
    /// Checks if the given character is a APF ligature: yiddish yod-yod-patah.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_ligature_yiddisch_yod_yod_patah;
    ///
    /// let consonant = 'ײַ';
    /// assert!(is_apf_ligature_yiddisch_yod_yod_patah(consonant));
    /// ```
    pub fn is_apf_ligature_yiddisch_yod_yod_patah(c: char) -> bool {
        // U+FB1F
        matches!(c, '\u{FB1F}')
    }
    /// Checks if the given character is a APF ligature: alef-lamed.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_ligature_alef_lamed;
    ///
    /// let consonant = 'ﭏ';
    /// assert!(is_apf_ligature_alef_lamed(consonant));
    /// ```
    pub fn is_apf_ligature_alef_lamed(c: char) -> bool {
        // U+FB4F
        matches!(c, '\u{FB4F}')
    }
}
pub mod apf_point {
    /// Checks if the given character is a APF point: judeo-spanish_varika.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_apf_point_judeo_spanish_varika;
    ///
    /// let consonant = 'ﬞ';
    /// assert!(is_apf_point_judeo_spanish_varika(consonant));
    /// ```
    pub fn is_apf_point_judeo_spanish_varika(c: char) -> bool {
        // U+FB1E
        matches!(c, '\u{FB1E}')
    }
}
