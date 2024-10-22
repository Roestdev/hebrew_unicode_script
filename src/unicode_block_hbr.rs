pub mod hebrew_consonants {
    /// Checks if the given character is a HBR consonant alef.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_alef;
    ///
    /// let consonant = 'א';
    /// assert!(is_hbr_consonant_alef(consonant));
    /// ```
    pub fn is_hbr_consonant_alef(c: char) -> bool {
        // U+05D0
        matches!(c, '\u{05D0}')
    }
    /// Checks if the given character is a HBR consonant bet.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_bet;
    ///
    /// let consonant = 'ב';
    /// assert!(is_hbr_consonant_bet(consonant));
    /// ```
    pub fn is_hbr_consonant_bet(c: char) -> bool {
        // U+05D1
        matches!(c, '\u{05D1}')
    }
    /// Checks if the given character is a HBR consonant gimel.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_gimel;
    ///
    /// let consonant = 'ג';
    /// assert!(is_hbr_consonant_gimel(consonant));
    /// ```
    pub fn is_hbr_consonant_gimel(c: char) -> bool {
        // U+05D2
        matches!(c, '\u{05D2}')
    }
    /// Checks if the given character is a HBR consonant dalet.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_dalet;
    ///
    /// let consonant = 'ד';
    /// assert!(is_hbr_consonant_dalet(consonant));
    /// ```
    pub fn is_hbr_consonant_dalet(c: char) -> bool {
        // U+05D3
        matches!(c, '\u{05D3}')
    }
    /// Checks if the given character is a HBR consonant he.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_he;
    ///
    /// let consonant = 'ה';
    /// assert!(is_hbr_consonant_he(consonant));
    /// ```
    pub fn is_hbr_consonant_he(c: char) -> bool {
        // U+05D4
        matches!(c, '\u{05D4}')
    }
    /// Checks if the given character is a HBR consonant vav.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_vav;
    ///
    /// let consonant = 'ו';
    /// assert!(is_hbr_consonant_vav(consonant));
    /// ```
    pub fn is_hbr_consonant_vav(c: char) -> bool {
        // U+U+05D5
        matches!(c, '\u{05D5}')
    }
    /// Checks if the given character is a HBR consonant zayin.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_zayin;
    ///
    /// let consonant = 'ז';
    /// assert!(is_hbr_consonant_zayin(consonant));
    /// ```
    pub fn is_hbr_consonant_zayin(c: char) -> bool {
        // U+05D6
        matches!(c, '\u{05D6}')
    }
    /// Checks if the given character is a HBR consonant het.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_het;
    ///
    /// let consonant = 'ח';
    /// assert!(is_hbr_consonant_het(consonant));
    /// ```
    pub fn is_hbr_consonant_het(c: char) -> bool {
        // U+05D7
        matches!(c, '\u{05D7}')
    }
    /// Checks if the given character is a HBR consonant tet.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_tet;
    ///
    /// let consonant = 'ט';
    /// assert!(is_hbr_consonant_tet(consonant));
    /// ```
    pub fn is_hbr_consonant_tet(c: char) -> bool {
        // U+05D8
        matches!(c, '\u{05D8}')
    }
    /// Checks if the given character is a HBR consonant yod.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_yod;
    ///
    /// let consonant = 'י';
    /// assert!(is_hbr_consonant_yod(consonant));
    /// ```
    pub fn is_hbr_consonant_yod(c: char) -> bool {
        // U+05D9
        matches!(c, '\u{05D9}')
    }
    /// Checks if the given character is a HBR consonant final-kaf.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_final_kaf;
    ///
    /// let consonant = 'ך';
    /// assert!(is_hbr_consonant_final_kaf(consonant));
    /// ```
    pub fn is_hbr_consonant_final_kaf(c: char) -> bool {
        // U+05DA
        matches!(c, '\u{05DA}')
    }
    /// Checks if the given character is a HBR consonant kaf.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_kaf;
    ///
    /// let consonant = 'כ';
    /// assert!(is_hbr_consonant_kaf(consonant));
    /// ```
    pub fn is_hbr_consonant_kaf(c: char) -> bool {
        // U+05DB
        matches!(c, '\u{05DB}')
    }
    /// Checks if the given character is a HBR consonant lamed.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_lamed;
    ///
    /// let consonant = 'ל';
    /// assert!(is_hbr_consonant_lamed(consonant));
    /// ```
    pub fn is_hbr_consonant_lamed(c: char) -> bool {
        // U+05DC
        matches!(c, '\u{05DC}')
    }
    /// Checks if the given character is a HBR consonant final-mem.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_final_mem;
    ///
    /// let consonant = 'ם';
    /// assert!(is_hbr_consonant_final_mem(consonant));
    /// ```
    pub fn is_hbr_consonant_final_mem(c: char) -> bool {
        // U+05DD
        matches!(c, '\u{05DD}')
    }
    /// Checks if the given character is a HBR consonant mem.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_mem;
    ///
    /// let consonant = 'מ';
    /// assert!(is_hbr_consonant_mem(consonant));
    /// ```
    pub fn is_hbr_consonant_mem(c: char) -> bool {
        // U+05DE
        matches!(c, '\u{05DE}')
    }
    /// Checks if the given character is a HBR consonant final-nun.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_final_nun;
    ///
    /// let consonant = 'ן';
    /// assert!(is_hbr_consonant_final_nun(consonant));
    /// ```
    pub fn is_hbr_consonant_final_nun(c: char) -> bool {
        // U+05DF
        matches!(c, '\u{05DF}')
    }
    /// Checks if the given character is a HBR consonant nun.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_nun;
    ///
    /// let consonant = 'נ';
    /// assert!(is_hbr_consonant_nun(consonant));
    /// ```
    pub fn is_hbr_consonant_nun(c: char) -> bool {
        // U+05E0
        matches!(c, '\u{05E0}')
    }
    /// Checks if the given character is a HBR consonant samekh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_samekh;
    ///
    /// let consonant = 'ס';
    /// assert!(is_hbr_consonant_samekh(consonant));
    /// ```
    pub fn is_hbr_consonant_samekh(c: char) -> bool {
        // U+05E1
        matches!(c, '\u{05E1}')
    }
    /// Checks if the given character is a HBR consonant ayin.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_ayin;
    ///
    /// let consonant = 'ע';
    /// assert!(is_hbr_consonant_ayin(consonant));
    /// ```
    pub fn is_hbr_consonant_ayin(c: char) -> bool {
        // U+05E2
        matches!(c, '\u{05E2}')
    }
    /// Checks if the given character is a HBR consonant final-pe.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_final_pe;
    ///
    /// let consonant = 'ף';
    /// assert!(is_hbr_consonant_final_pe(consonant));
    /// ```
    pub fn is_hbr_consonant_final_pe(c: char) -> bool {
        // U+05E3
        matches!(c, '\u{05E3}')
    }
    /// Checks if the given character is a HBR consonant pe.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_pe;
    ///
    /// let consonant = 'פ';
    /// assert!(is_hbr_consonant_pe(consonant));
    /// ```
    pub fn is_hbr_consonant_pe(c: char) -> bool {
        // U+05E4
        matches!(c, '\u{05E4}')
    }
    /// Checks if the given character is a HBR consonant final-tsadi.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_final_tsadi;
    ///
    /// let consonant = 'ץ';
    /// assert!(is_hbr_consonant_final_tsadi(consonant));
    /// ```
    pub fn is_hbr_consonant_final_tsadi(c: char) -> bool {
        // U+05E5
        matches!(c, '\u{05E5}')
    }
    /// Checks if the given character is a HBR consonant tsadi.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_tsadi;
    ///
    /// let consonant = 'צ';
    /// assert!(is_hbr_consonant_tsadi(consonant));
    /// ```
    pub fn is_hbr_consonant_tsadi(c: char) -> bool {
        // U+05E6
        matches!(c, '\u{05E6}')
    }
    /// Checks if the given character is a HBR consonant qof.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_qof;
    ///
    /// let consonant = 'ק';
    /// assert!(is_hbr_consonant_qof(consonant));
    /// ```
    pub fn is_hbr_consonant_qof(c: char) -> bool {
        // U+05E7
        matches!(c, '\u{05E7}')
    }
    /// Checks if the given character is a HBR consonant resh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_resh;
    ///
    /// let consonant = 'ר';
    /// assert!(is_hbr_consonant_resh(consonant));
    /// ```
    pub fn is_hbr_consonant_resh(c: char) -> bool {
        // U+05E8
        matches!(c, '\u{05E8}')
    }
    /// Checks if the given character is a HBR consonant shin.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_shin;
    ///
    /// let consonant = 'ש';
    /// assert!(is_hbr_consonant_shin(consonant));
    /// ```
    pub fn is_hbr_consonant_shin(c: char) -> bool {
        // U+05E9
        matches!(c, '\u{05E9}')
    }
    /// Checks if the given character is a HBR consonant tav.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_consonant_tav;
    ///
    /// let consonant = 'ת';
    /// assert!(is_hbr_consonant_tav(consonant));
    /// ```
    pub fn is_hbr_consonant_tav(c: char) -> bool {
        // U+05EA
        matches!(c, '\u{05EA}')
    }
}
pub mod hebrew_points {
    /// Checks if the given character is a HBR point sheva.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_sheva;
    ///
    /// let point = 'ְ';
    /// assert!(is_hbr_point_sheva(point));
    /// ```
    pub fn is_hbr_point_sheva(c: char) -> bool {
        // U+05B0
        matches!(c, '\u{05B0}')
    }
    /// Checks if the given character is a HBR point hataf-segol
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_hataf_segol;
    ///
    /// let point = 'ֱ';
    /// assert!(is_hbr_point_hataf_segol(point));
    /// ```
    pub fn is_hbr_point_hataf_segol(c: char) -> bool {
        // U+05B1
        matches!(c, '\u{05B1}')
    }
    /// Checks if the given character is a HBR point hataf-patah.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_hataf_patah;
    ///
    /// let point = 'ֲ';
    /// assert!(is_hbr_point_hataf_patah(point));
    /// ```
    pub fn is_hbr_point_hataf_patah(c: char) -> bool {
        // U+05B2
        matches!(c, '\u{05B2}')
    }
    /// Checks if the given character is a HBR point hataf-qamats.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_hataf_qamats;
    ///
    /// let point = 'ֳ';
    /// assert!(is_hbr_point_hataf_qamats(point));
    /// ```
    pub fn is_hbr_point_hataf_qamats(c: char) -> bool {
        // U+05B3
        matches!(c, '\u{05B3}')
    }
    /// Checks if the given character is a HBR point hiriq.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_hiriq;
    ///
    /// let point = 'ִ';
    /// assert!(is_hbr_point_hiriq(point));
    /// ```
    pub fn is_hbr_point_hiriq(c: char) -> bool {
        // U+05B4
        matches!(c, '\u{05B4}')
    }
    /// Checks if the given character is a HBR point tsere.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_tsere;
    ///
    /// let point = 'ֵ';
    /// assert!(is_hbr_point_tsere(point));
    /// ```
    pub fn is_hbr_point_tsere(c: char) -> bool {
        // U+05B5
        matches!(c, '\u{05B5}')
    }
    /// Checks if the given character is a HBR point segol.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_segol;
    ///
    /// let point = 'ֶ';
    /// assert!(is_hbr_point_segol(point));
    /// ```
    pub fn is_hbr_point_segol(c: char) -> bool {
        // U+05B6
        matches!(c, '\u{05B6}')
    }
    /// Checks if the given character is a HBR point patah.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_patah;
    ///
    /// let point = 'ַ';
    /// assert!(is_hbr_point_patah(point));
    /// ```
    pub fn is_hbr_point_patah(c: char) -> bool {
        // U+05B7
        matches!(c, '\u{05B7}')
    }
    /// Checks if the given character is a HBR point qamats.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_qamats;
    ///
    /// let point = 'ָ';
    /// assert!(is_hbr_point_qamats(point));
    /// ```
    pub fn is_hbr_point_qamats(c: char) -> bool {
        // U+05B8
        matches!(c, '\u{05B8}')
    }
    /// Checks if the given character is a HBR point holam.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_holam;
    ///
    /// let point = 'ֹ';
    /// assert!(is_hbr_point_holam(point));
    /// ```
    pub fn is_hbr_point_holam(c: char) -> bool {
        // U+05B9
        matches!(c, '\u{05B9}')
    }
    /// Checks if the given character is a HBR point holam-haser_for_vav.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_holam_haser_for_vav;
    ///
    /// let point = 'ֺ';
    /// assert!(is_hbr_point_holam_haser_for_vav(point));
    /// ```
    pub fn is_hbr_point_holam_haser_for_vav(c: char) -> bool {
        // U+05BA
        matches!(c, '\u{05BA}')
    }
    /// Checks if the given character is a HBR point qubuts.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_qubuts;
    ///
    /// let point = 'ֻ';
    /// assert!(is_hbr_point_qubuts(point));
    /// ```
    pub fn is_hbr_point_qubuts(c: char) -> bool {
        // U+05BB
        matches!(c, '\u{05BB}')
    }
    /// Checks if the given character is a HBR point dagesh_or_mapiq.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_dagesh_or_mapiq;
    ///
    /// let point = 'ּ';
    /// assert!(is_hbr_point_dagesh_or_mapiq(point));
    /// ```
    pub fn is_hbr_point_dagesh_or_mapiq(c: char) -> bool {
        // U+05BC
        matches!(c, '\u{05BC}')
    }
    /// Checks if the given character is a HBR point meteg.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_meteg;
    ///
    /// let point = 'ֽ';
    /// assert!(is_hbr_point_meteg(point));
    /// ```
    pub fn is_hbr_point_meteg(c: char) -> bool {
        // U+05BD
        matches!(c, '\u{05BD}')
    }
    /// Checks if the given character is a HBR point rafe.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_rafe;
    ///
    /// let point = 'ֿ';
    /// assert!(is_hbr_point_rafe(point));
    /// ```
    pub fn is_hbr_point_rafe(c: char) -> bool {
        // U+05BF
        matches!(c, '\u{05BF}')
    }
    /// Checks if the given character is a HBR point shin-dot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_shin_dot;
    ///
    /// let point = 'ׁ';
    /// assert!(is_hbr_point_shin_dot(point));
    /// ```
    pub fn is_hbr_point_shin_dot(c: char) -> bool {
        // U+05C1
        matches!(c, '\u{05C1}')
    }
    /// Checks if the given character is a HBR point sin-dot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_sin_dot;
    ///
    /// let point = 'ׂ';
    /// assert!(is_hbr_point_sin_dot(point));
    /// ```
    pub fn is_hbr_point_sin_dot(c: char) -> bool {
        // U+05C2
        matches!(c, '\u{05C2}')
    }
    /// Checks if the given character is a HBR point qamats-qatan.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_point_qamats_qatan;
    ///
    /// let point = 'ׇ';
    /// assert!(is_hbr_point_qamats_qatan(point));
    /// ```
    pub fn is_hbr_point_qamats_qatan(c: char) -> bool {
        // U+05C7
        matches!(c, '\u{05C7}')
    }
}
pub mod hebrew_accents {

    /// Checks if the given character is a HBR accent etnahta.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_etnahta;
    ///
    /// let accent = '֑';
    /// assert!(is_hbr_accent_etnahta(accent));
    /// ```
    pub fn is_hbr_accent_etnahta(c: char) -> bool {
        // U+0591
        matches!(c, '\u{0591}')
    }
    /// Checks if the given character is a HBR accent segol.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_segol;
    ///
    /// let accent = '֒';
    /// assert!(is_hbr_accent_segol(accent));
    /// ```
    pub fn is_hbr_accent_segol(c: char) -> bool {
        // U+0592
        matches!(c, '\u{0592}')
    }
    /// Checks if the given character is a HBR accent shalshelet.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_shalshelet;
    ///
    /// let accent = '֓';
    /// assert!(is_hbr_accent_shalshelet(accent));
    /// ```
    pub fn is_hbr_accent_shalshelet(c: char) -> bool {
        // U+0593
        matches!(c, '\u{0593}')
    }
    /// Checks if the given character is a HBR accent zaqef-qatan.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_zaqef_qatan;
    ///
    /// let accent = '֔';
    /// assert!(is_hbr_accent_zaqef_qatan(accent));
    /// ```
    pub fn is_hbr_accent_zaqef_qatan(c: char) -> bool {
        // U+0594
        matches!(c, '\u{0594}')
    }
    /// Checks if the given character is a HBR accent zaqef-gadol.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_zaqef_gadol;
    ///
    /// let accent = '֕';
    /// assert!(is_hbr_accent_zaqef_gadol(accent));
    /// ```
    pub fn is_hbr_accent_zaqef_gadol(c: char) -> bool {
        // U+0595
        matches!(c, '\u{0595}')
    }
    /// Checks if the given character is a HBR accent tipeha.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_tipeha;
    ///
    /// let accent = '֖';
    /// assert!(is_hbr_accent_tipeha(accent));
    /// ```
    pub fn is_hbr_accent_tipeha(c: char) -> bool {
        // U+0596
        matches!(c, '\u{0596}')
    }
    /// Checks if the given character is a HBR accent revia.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_revia;
    ///
    /// let accent = '֗';
    /// assert!(is_hbr_accent_revia(accent));
    /// ```
    pub fn is_hbr_accent_revia(c: char) -> bool {
        // U+0597
        matches!(c, '\u{0597}')
    }
    /// Checks if the given character is a HBR accent zarqa.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_zarqa;
    ///
    /// let accent = '֘';
    /// assert!(is_hbr_accent_zarqa(accent));
    /// ```
    pub fn is_hbr_accent_zarqa(c: char) -> bool {
        // U+0598
        matches!(c, '\u{0598}')
    }
    /// Checks if the given character is a HBR accent pashta.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_pashta;
    ///
    /// let accent = '֙';
    /// assert!(is_hbr_accent_pashta(accent));
    /// ```
    pub fn is_hbr_accent_pashta(c: char) -> bool {
        // U+0599
        matches!(c, '\u{0599}')
    }
    /// Checks if the given character is a HBR accent yetiv.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_yetiv;
    ///
    /// let accent = '֚';
    /// assert!(is_hbr_accent_yetiv(accent));
    /// ```
    pub fn is_hbr_accent_yetiv(c: char) -> bool {
        // U+059A
        matches!(c, '\u{059A}')
    }
    /// Checks if the given character is a HBR accent tevir.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_tevir;
    ///
    /// let accent = '֛';
    /// assert!(is_hbr_accent_tevir(accent));
    /// ```
    pub fn is_hbr_accent_tevir(c: char) -> bool {
        // U+059B
        matches!(c, '\u{059B}')
    }
    /// Checks if the given character is a HBR accent geresh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_geresh;
    ///
    /// let accent = '֜';
    /// assert!(is_hbr_accent_geresh(accent));
    /// ```
    pub fn is_hbr_accent_geresh(c: char) -> bool {
        // U+059C
        matches!(c, '\u{059C}')
    }
    /// Checks if the given character is a HBR accent geresh-muqdam.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_geresh_muqdam;
    ///
    /// let accent = '֝';
    /// assert!(is_hbr_accent_geresh_muqdam(accent));
    /// ```
    pub fn is_hbr_accent_geresh_muqdam(c: char) -> bool {
        // U+059D
        matches!(c, '\u{059D}')
    }
    /// Checks if the given character is a HBR accent gershayim.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_gershayim;
    ///
    /// let accent = '֞';
    /// assert!(is_hbr_accent_gershayim(accent));
    /// ```
    pub fn is_hbr_accent_gershayim(c: char) -> bool {
        // U+059E
        matches!(c, '\u{059E}')
    }
    /// Checks if the given character is a HBR accent qarney-para.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_qarney_para;
    ///
    /// let accent = '֟';
    /// assert!(is_hbr_accent_qarney_para(accent));
    /// ```
    pub fn is_hbr_accent_qarney_para(c: char) -> bool {
        // U+059F
        matches!(c, '\u{059F}')
    }
    /// Checks if the given character is a HBR accent telisha-gedola.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_telisha_gedola;
    ///
    /// let accent = '֠';
    /// assert!(is_hbr_accent_telisha_gedola(accent));
    /// ```
    pub fn is_hbr_accent_telisha_gedola(c: char) -> bool {
        // U+05A0
        matches!(c, '\u{05A0}')
    }
    /// Checks if the given character is a HBR accent pazer.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_pazer;
    ///
    /// let accent = '֡';
    /// assert!(is_hbr_accent_pazer(accent));
    /// ```
    pub fn is_hbr_accent_pazer(c: char) -> bool {
        // U+05A1
        matches!(c, '\u{05A1}')
    }
    /// Checks if the given character is a HBR accent atnah-hafukh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_atnah_hafukh;
    ///
    /// let accent = '֢';
    /// assert!(is_hbr_accent_atnah_hafukh(accent));
    /// ```
    pub fn is_hbr_accent_atnah_hafukh(c: char) -> bool {
        // U+05A2
        matches!(c, '\u{05A2}')
    }
    /// Checks if the given character is a HBR accent munah.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_munah;
    ///
    /// let accent = '֣';
    /// assert!(is_hbr_accent_munah(accent));
    /// ```
    pub fn is_hbr_accent_munah(c: char) -> bool {
        // U+05A3
        matches!(c, '\u{05A3}')
    }
    /// Checks if the given character is a HBR accent mahapakh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_mahapakh;
    ///
    /// let accent = '֤';
    /// assert!(is_hbr_accent_mahapakh(accent));
    /// ```
    pub fn is_hbr_accent_mahapakh(c: char) -> bool {
        // U+05A4
        matches!(c, '\u{05A4}')
    }
    /// Checks if the given character is a HBR accent merkha.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_merkha;
    ///
    /// let accent = '֥';
    /// assert!(is_hbr_accent_merkha(accent));
    /// ```
    pub fn is_hbr_accent_merkha(c: char) -> bool {
        // U+05A5
        matches!(c, '\u{05A5}')
    }
    /// Checks if the given character is a HBR accent merkha-kefula.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_merkha_kefula;
    ///
    /// let accent = '֦';
    /// assert!(is_hbr_accent_merkha_kefula(accent));
    /// ```
    pub fn is_hbr_accent_merkha_kefula(c: char) -> bool {
        // U+05A6
        matches!(c, '\u{05A6}')
    }
    /// Checks if the given character is a HBR accent darga.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_darga;
    ///
    /// let accent = '֧';
    /// assert!(is_hbr_accent_darga(accent));
    /// ```
    pub fn is_hbr_accent_darga(c: char) -> bool {
        // U+05A7
        matches!(c, '\u{05A7}')
    }
    /// Checks if the given character is a HBR accent qadma.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_qadma;
    ///
    /// let accent = '֨';
    /// assert!(is_hbr_accent_qadma(accent));
    /// ```
    pub fn is_hbr_accent_qadma(c: char) -> bool {
        // U+05A8
        matches!(c, '\u{05A8}')
    }
    /// Checks if the given character is a HBR accent telisha-qetana.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_telisha_qetana;
    ///
    /// let accent = '֩';
    /// assert!(is_hbr_accent_telisha_qetana(accent));
    /// ```
    pub fn is_hbr_accent_telisha_qetana(c: char) -> bool {
        // U+05A9
        matches!(c, '\u{05A9}')
    }
    /// Checks if the given character is a HBR accent yerah-ben-yomo.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_yerah_ben_yomo;
    ///
    /// let accent = '֪';
    /// assert!(is_hbr_accent_yerah_ben_yomo(accent));
    /// ```
    pub fn is_hbr_accent_yerah_ben_yomo(c: char) -> bool {
        // U+05AA
        matches!(c, '\u{05AA}')
    }
    /// Checks if the given character is a HBR accent ole.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_ole;
    ///
    /// let accent = '֫';
    /// assert!(is_hbr_accent_ole(accent));
    /// ```
    pub fn is_hbr_accent_ole(c: char) -> bool {
        // U+05AB
        matches!(c, '\u{05AB}')
    }
    /// Checks if the given character is a HBR accent iluy.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_iluy;
    ///
    /// let accent = '֬';
    /// assert!(is_hbr_accent_iluy(accent));
    /// ```
    pub fn is_hbr_accent_iluy(c: char) -> bool {
        // U+05AC
        matches!(c, '\u{05AC}')
    }
    /// Checks if the given character is a HBR accent dehi.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_dehi;
    ///
    /// let accent = '֭';
    /// assert!(is_hbr_accent_dehi(accent));
    /// ```
    pub fn is_hbr_accent_dehi(c: char) -> bool {
        // U+05AD
        matches!(c, '\u{05AD}')
    }
    /// Checks if the given character is a HBR accent zinor.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_accent_zinor;
    ///
    /// let accent = '֮';
    /// assert!(is_hbr_accent_zinor(accent));
    /// ```
    pub fn is_hbr_accent_zinor(c: char) -> bool {
        // U+05AE
        matches!(c, '\u{05AE}')
    }
}
pub mod hebrew_marks {
    /// Checks if the given character is a HBR mark masora-circle.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_mark_masora_circle;
    ///
    /// let mark = '֯';
    /// assert!(is_hbr_mark_masora_circle(mark));
    /// ```
    pub fn is_hbr_mark_masora_circle(c: char) -> bool {
        // U+05AF
        matches!(c, '\u{05AF}')
    }
    /// Checks if the given character is a HBR mark upper-dot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_mark_upper_dot;
    ///
    /// let punctuation = 'ׄ';
    /// assert!(is_hbr_mark_upper_dot(punctuation));
    /// ```
    pub fn is_hbr_mark_upper_dot(c: char) -> bool {
        // U+05C4
        matches!(c, '\u{05C4}')
    }
    /// Checks if the given character is a HBR mark lower-dot.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_mark_lower_dot;
    ///
    /// let mark = 'ׅ';
    /// assert!(is_hbr_mark_lower_dot(mark));
    /// ```
    pub fn is_hbr_mark_lower_dot(c: char) -> bool {
        // U+05C5
        matches!(c, '\u{05C5}')
    }
}
pub mod hebrew_punctuations {
    /// Checks if the given character is a HBR punctuation maqaf.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_punctuation_maqaf;
    ///
    /// let punctuation = '־';
    /// assert!(is_hbr_punctuation_maqaf(punctuation));
    /// ```
    pub fn is_hbr_punctuation_maqaf(c: char) -> bool {
        // U+05BE
        matches!(c, '\u{05BE}')
    }
    /// Checks if the given character is a HBR punctuation paseq.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_punctuation_paseq;
    ///
    /// let punctuation = '׀';
    /// assert!(is_hbr_punctuation_paseq(punctuation));
    /// ```
    pub fn is_hbr_punctuation_paseq(c: char) -> bool {
        // U+05C0
        matches!(c, '\u{05C0}')
    }
    /// Checks if the given character is a HBR punctuation sof-pasuq.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_punctuation_sof_pasuq;
    ///
    /// let punctuation = '׃';
    /// assert!(is_hbr_punctuation_sof_pasuq(punctuation));
    /// ```
    pub fn is_hbr_punctuation_sof_pasuq(c: char) -> bool {
        // U+05C3
        matches!(c, '\u{05C3}')
    }
    /// Checks if the given character is a HBR punctuation nun-hafukha.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_punctuation_nun_hafukha;
    ///
    /// let punctuation = '׆';
    /// assert!(is_hbr_punctuation_nun_hafukha(punctuation));
    /// ```
    pub fn is_hbr_punctuation_nun_hafukha(c: char) -> bool {
        // U+05C6
        matches!(c, '\u{05C6}')
    }
    /// Checks if the given character is a HBR punctuation geresh.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_punctuation_geresh;
    ///
    /// let punctuation = '׳';
    /// assert!(is_hbr_punctuation_geresh(punctuation));
    /// ```
    pub fn is_hbr_punctuation_geresh(c: char) -> bool {
        // U+05F3
        matches!(c, '\u{05F3}')
    }
    /// Checks if the given character is a HBR punctuation gershayim.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_punctuation_gershayim;
    ///
    /// let punctuation = '״';
    /// assert!(is_hbr_punctuation_gershayim(punctuation));
    /// ```
    pub fn is_hbr_punctuation_gershayim(c: char) -> bool {
        // U+05F4
        matches!(c, '\u{05F4}')
    }
}
pub mod hebrew_misc {
    /// Checks if the given character is a HBR yod-triangle.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_yod_triangle;
    ///
    /// let ligature_yiddish = '\u{05EF}';
    /// assert!(is_hbr_yod_triangle(ligature_yiddish));
    /// ```
    pub fn is_hbr_yod_triangle(c: char) -> bool {
        // U+05EF
        matches!(c, '\u{05EF}')
    }
    /// Checks if the given character is a HBR ligature yiddisch-double-vav.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_ligature_yiddisch_double_vav;
    ///
    /// let ligature_yiddish = 'װ';
    /// assert!(is_hbr_ligature_yiddisch_double_vav(ligature_yiddish));
    /// ```
    pub fn is_hbr_ligature_yiddisch_double_vav(c: char) -> bool {
        // U+05F0
        matches!(c, '\u{05F0}')
    }
    /// Checks if the given character is a HBR ligature yiddisch-vav-yod.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_ligature_yiddisch_vav_yod;
    ///
    /// let ligature_yiddish = 'ױ';
    /// assert!(is_hbr_ligature_yiddisch_vav_yod(ligature_yiddish));
    /// ```
    pub fn is_hbr_ligature_yiddisch_vav_yod(c: char) -> bool {
        // U+05F1
        matches!(c, '\u{05F1}')
    }
    /// Checks if the given character is a HBR ligature yiddisch-double-yod.
    ///
    /// # Example
    /// ```
    /// use hebrew_unicode_script::is_hbr_ligature_yiddisch_double_yod;
    ///
    /// let ligature_yiddish = 'ײ';
    /// assert!(is_hbr_ligature_yiddisch_double_yod(ligature_yiddish));
    /// ```
    pub fn is_hbr_ligature_yiddisch_double_yod(c: char) -> bool {
        // U+05F2
        matches!(c, '\u{05F2}')
    }
}
