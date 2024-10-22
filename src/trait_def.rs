//use crate::collections_api::unicode_block_alphabetic_presentation_form::*;
//use crate::collections_api::unicode_block_hebrew::*;
//use crate::collections_api::unicode_script_hebrew::*;
//use crate::*;
/// A trait for identification and validation of Hebrew characters
///
/// For the implementation of the trait the functions descibed in de file 'collections_api.rs' are reused
pub trait HebrewUnicodeScript {
    // source:: collections_api.rs
    // module:: Unicode Script Hebrew

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
    /// Checks if the given character is a 'point' type within the unicode script 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB1E}'.is_script_hbr_point_reading_sign());
    /// ```
    fn is_script_hbr_point_reading_sign(&self) -> bool;
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
    /// Checks if the given character is a 'ligature_yiddisch' type within the unicode script 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB1F}'.is_script_hbr_ligature());
    /// ```
    fn is_script_hbr_ligature(&self) -> bool;

    // source:: collections_api.rs
    // module:: Unicode Block: Hebrew

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
    /// Checks if the given character is a Hebrew ligature yiddish.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05F0}'.is_hbr_ligature_yiddish());
    /// ```
    fn is_hbr_ligature_yiddish(&self) -> bool;

    // source:: collections_api.rs
    // module:: Unicode Block Alphabetic Presentation Form

    /// Checks if the given character belongs to the unicode block 'Alphabetic Presentation Form'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB4E}'.is_apf_block());
    /// ```
    fn is_apf_block(&self) -> bool;
    /// Checks if the given character is an apf letter.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB20}'.is_apf_consonant());
    /// ```
    fn is_apf_consonant(&self) -> bool;
    /// Checks if the given character is an apf letter with a vowel
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB30}'.is_apf_consonant_with_vowel());
    /// ```
    fn is_apf_consonant_with_vowel(&self) -> bool;
    /// Checks if the given character is an apf point.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB1E}'.is_apf_point_reading_sign());
    /// ```
    fn is_apf_point_reading_sign(&self) -> bool;
    /// Checks if the given character is a Yiddish ligature.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB4F}'.is_apf_ligature());
    /// ```
    fn is_apf_ligature(&self) -> bool;
    /// Checks if the given character is an apf an alternative letter.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB20}'.is_apf_alternative());
    /// ```
    fn is_apf_alternative(&self) -> bool;
    /// Checks if the given character is an apf wide letter.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{FB21}'.is_apf_consonant_wide());
    /// ```
    fn is_apf_consonant_wide(&self) -> bool;

    ////////////////////////////////////////////////////////
    // source:: unicode_block_hbr.rs
    ////////////////////////////////////////////////////////

    /// Checks if the given character is a HBR consonant alef.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('א'.is_hbr_consonant_alef());
    /// ```
    fn is_hbr_consonant_alef(&self) -> bool;

    /// Checks if the given character is a HBR consonant bet.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ב'.is_hbr_consonant_bet());
    /// ```
    fn is_hbr_consonant_bet(&self) -> bool;

    /// Checks if the given character is a HBR consonant gimel.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ג'.is_hbr_consonant_gimel());
    /// ```
    fn is_hbr_consonant_gimel(&self) -> bool;

    /// Checks if the given character is a HBR consonant dalet.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ד'.is_hbr_consonant_dalet());
    /// ```
    fn is_hbr_consonant_dalet(&self) -> bool;

    /// Checks if the given character is a HBR consonant he.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ה'.is_hbr_consonant_he());
    /// ```
    fn is_hbr_consonant_he(&self) -> bool;

    /// Checks if the given character is a HBR consonant vav.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ו'.is_hbr_consonant_vav());
    /// ```
    fn is_hbr_consonant_vav(&self) -> bool;

    /// Checks if the given character is a HBR consonant zayin.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ז'.is_hbr_consonant_zayin());
    /// ```
    fn is_hbr_consonant_zayin(&self) -> bool;

    /// Checks if the given character is a HBR consonant het.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ח'.is_hbr_consonant_het());
    /// ```
    fn is_hbr_consonant_het(&self) -> bool;

    /// Checks if the given character is a HBR consonant tet.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ט'.is_hbr_consonant_tet());
    /// ```
    fn is_hbr_consonant_tet(&self) -> bool;

    /// Checks if the given character belongs to the unicode script 'Hebrew'.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('י'.is_hbr_consonant_yod());
    /// ```
    fn is_hbr_consonant_yod(&self) -> bool;

    /// Checks if the given character is a HBR consonant final-kaf.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ך'.is_hbr_consonant_final_kaf());
    /// ```
    fn is_hbr_consonant_final_kaf(&self) -> bool;

    /// Checks if the given character is a HBR consonant kaf.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('כ'.is_hbr_consonant_kaf());
    /// ```
    fn is_hbr_consonant_kaf(&self) -> bool;

    /// Checks if the given character is a HBR consonant lamed.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ל'.is_hbr_consonant_lamed());
    /// ```
    fn is_hbr_consonant_lamed(&self) -> bool;

    /// Checks if the given character is a HBR consonant final-mem.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ם'.is_hbr_consonant_final_mem());
    /// ```
    fn is_hbr_consonant_final_mem(&self) -> bool;

    /// Checks if the given character is a HBR consonant mem.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('מ'.is_hbr_consonant_mem());
    /// ```
    fn is_hbr_consonant_mem(&self) -> bool;

    /// Checks if the given character is a HBR consonant final-nun.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ן'.is_hbr_consonant_final_nun());
    /// ```
    fn is_hbr_consonant_final_nun(&self) -> bool;

    /// Checks if the given character is a HBR consonant nun.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('נ'.is_hbr_consonant_nun());
    /// ```
    fn is_hbr_consonant_nun(&self) -> bool;

    /// Checks if the given character is a HBR consonant samekh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ס'.is_hbr_consonant_samekh());
    /// ```
    fn is_hbr_consonant_samekh(&self) -> bool;

    /// Checks if the given character is a HBR consonant ayin.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ע'.is_hbr_consonant_ayin());
    /// ```
    fn is_hbr_consonant_ayin(&self) -> bool;

    /// Checks if the given character is a HBR consonant final-pe.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ף'.is_hbr_consonant_final_pe());
    /// ```
    fn is_hbr_consonant_final_pe(&self) -> bool;

    /// Checks if the given character is a HBR consonant pe.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('פ'.is_hbr_consonant_pe());
    /// ```
    fn is_hbr_consonant_pe(&self) -> bool;

    /// Checks if the given character is a HBR consonant final-tsadi.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ץ'.is_hbr_consonant_final_tsadi());
    /// ```
    fn is_hbr_consonant_final_tsadi(&self) -> bool;

    /// Checks if the given character is a HBR consonant tsadi.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('צ'.is_hbr_consonant_tsadi());
    /// ```
    fn is_hbr_consonant_tsadi(&self) -> bool;

    /// Checks if the given character is a HBR consonant qof.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ק'.is_hbr_consonant_qof());
    /// ```
    fn is_hbr_consonant_qof(&self) -> bool;

    /// Checks if the given character is a HBR consonant resh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ר'.is_hbr_consonant_resh());
    /// ```
    fn is_hbr_consonant_resh(&self) -> bool;

    /// Checks if the given character is a HBR consonant shin.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ש'.is_hbr_consonant_shin());
    /// ```
    fn is_hbr_consonant_shin(&self) -> bool;

    /// Checks if the given character is a HBR consonant tav.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ת'.is_hbr_consonant_tav());
    /// ```
    fn is_hbr_consonant_tav(&self) -> bool;

    /// Checks if the given character is a HBR point sheva.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ְ'.is_hbr_point_sheva());
    /// ```
    fn is_hbr_point_sheva(&self) -> bool;

    /// Checks if the given character is a HBR point hataf-segol
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֱ'.is_hbr_point_hataf_segol());
    /// ```
    fn is_hbr_point_hataf_segol(&self) -> bool;

    /// Checks if the given character is a HBR point hataf-patah.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֲ'.is_hbr_point_hataf_patah());
    /// ```
    fn is_hbr_point_hataf_patah(&self) -> bool;

    /// Checks if the given character is a HBR point hataf-qamats.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֳ'.is_hbr_point_hataf_qamats());
    /// ```
    fn is_hbr_point_hataf_qamats(&self) -> bool;

    /// Checks if the given character is a HBR point hiriq.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ִ'.is_hbr_point_hiriq());
    /// ```
    fn is_hbr_point_hiriq(&self) -> bool;

    /// Checks if the given character is a HBR point tsere.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֵ'.is_hbr_point_tsere());
    /// ```
    fn is_hbr_point_tsere(&self) -> bool;

    /// Checks if the given character is a HBR point segol.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֶ'.is_hbr_point_segol());
    /// ```
    fn is_hbr_point_segol(&self) -> bool;

    /// Checks if the given character is a HBR point patah.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ַ'.is_hbr_point_patah());
    /// ```
    fn is_hbr_point_patah(&self) -> bool;

    /// Checks if the given character is a HBR point qamats.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ָ'.is_hbr_point_qamats());
    /// ```
    fn is_hbr_point_qamats(&self) -> bool;

    /// Checks if the given character is a HBR point holam.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֹ'.is_hbr_point_holam());
    /// ```
    fn is_hbr_point_holam(&self) -> bool;

    /// Checks if the given character is a HBR point holam-haser_for_vav.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֺ'.is_hbr_point_holam_haser_for_vav());
    /// ```
    fn is_hbr_point_holam_haser_for_vav(&self) -> bool;

    /// Checks if the given character is a HBR point qubuts.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֻ'.is_hbr_point_qubuts());
    /// ```
    fn is_hbr_point_qubuts(&self) -> bool;

    /// Checks if the given character is a HBR point dagesh_or_mapiq.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ּ'.is_hbr_point_dagesh_or_mapiq());
    /// ```
    fn is_hbr_point_dagesh_or_mapiq(&self) -> bool;

    /// Checks if the given character is a HBR point meteg.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֽ'.is_hbr_point_meteg());
    /// ```
    fn is_hbr_point_meteg(&self) -> bool;

    /// Checks if the given character is a HBR point rafe.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ֿ'.is_hbr_point_rafe());
    /// ```
    fn is_hbr_point_rafe(&self) -> bool;

    /// Checks if the given character is a HBR point shin-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ׁ'.is_hbr_point_shin_dot());
    /// ```
    fn is_hbr_point_shin_dot(&self) -> bool;

    /// Checks if the given character is a HBR point sin-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ׂ'.is_hbr_point_sin_dot());
    /// ```
    fn is_hbr_point_sin_dot(&self) -> bool;

    /// Checks if the given character is a HBR point qamats-qatan.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ׇ'.is_hbr_point_qamats_qatan());
    /// ```
    fn is_hbr_point_qamats_qatan(&self) -> bool;

    /// Checks if the given character is a HBR accent etnahta.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֑'.is_hbr_accent_etnahta());
    /// ```
    fn is_hbr_accent_etnahta(&self) -> bool;

    /// Checks if the given character is a HBR accent segol.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֒'.is_hbr_accent_segol());
    /// ```
    fn is_hbr_accent_segol(&self) -> bool;

    /// Checks if the given character is a HBR accent shalshelet.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֓'.is_hbr_accent_shalshelet());
    /// ```
    fn is_hbr_accent_shalshelet(&self) -> bool;

    /// Checks if the given character is a HBR accent zaqef-qatan.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֔'.is_hbr_accent_zaqef_qatan());
    /// ```
    fn is_hbr_accent_zaqef_qatan(&self) -> bool;

    /// Checks if the given character is a HBR accent zaqef-gadol.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֕'.is_hbr_accent_zaqef_gadol());
    /// ```
    fn is_hbr_accent_zaqef_gadol(&self) -> bool;

    /// Checks if the given character is a HBR accent tipeha.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֖'.is_hbr_accent_tipeha());
    /// ```
    fn is_hbr_accent_tipeha(&self) -> bool;

    /// Checks if the given character is a HBR accent revia.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֗'.is_hbr_accent_revia());
    /// ```
    fn is_hbr_accent_revia(&self) -> bool;

    /// Checks if the given character is a HBR accent zarqa.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֘'.is_hbr_accent_zarqa());
    /// ```
    fn is_hbr_accent_zarqa(&self) -> bool;

    /// Checks if the given character is a HBR accent pashta.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֙'.is_hbr_accent_pashta());
    /// ```
    fn is_hbr_accent_pashta(&self) -> bool;

    /// Checks if the given character is a HBR accent yetiv.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֚'.is_hbr_accent_yetiv());
    /// ```
    fn is_hbr_accent_yetiv(&self) -> bool;

    /// Checks if the given character is a HBR accent tevir.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֛'.is_hbr_accent_tevir());
    /// ```
    fn is_hbr_accent_tevir(&self) -> bool;

    /// Checks if the given character is a HBR accent geresh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֜'.is_hbr_accent_geresh());
    /// ```
    fn is_hbr_accent_geresh(&self) -> bool;

    /// Checks if the given character is a HBR accent geresh-muqdam.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֝'.is_hbr_accent_geresh_muqdam());
    /// ```
    fn is_hbr_accent_geresh_muqdam(&self) -> bool;

    /// Checks if the given character is a HBR accent gershayim.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֞'.is_hbr_accent_gershayim());
    /// ```
    fn is_hbr_accent_gershayim(&self) -> bool;

    /// Checks if the given character is a HBR accent qarney-para.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֟'.is_hbr_accent_qarney_para());
    /// ```
    fn is_hbr_accent_qarney_para(&self) -> bool;

    /// Checks if the given character is a HBR accent telisha-gedola.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֠'.is_hbr_accent_telisha_gedola());
    /// ```
    fn is_hbr_accent_telisha_gedola(&self) -> bool;

    /// Checks if the given character is a HBR accent pazer.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֡'.is_hbr_accent_pazer());
    /// ```
    fn is_hbr_accent_pazer(&self) -> bool;

    /// Checks if the given character is a HBR accent atnah-hafukh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֢'.is_hbr_accent_atnah_hafukh());
    /// ```
    fn is_hbr_accent_atnah_hafukh(&self) -> bool;

    /// Checks if the given character is a HBR accent munah.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֣'.is_hbr_accent_munah());
    /// ```
    fn is_hbr_accent_munah(&self) -> bool;

    /// Checks if the given character is a HBR accent mahapakh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֤'.is_hbr_accent_mahapakh());
    /// ```
    fn is_hbr_accent_mahapakh(&self) -> bool;

    /// Checks if the given character is a HBR accent merkha.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֥'.is_hbr_accent_merkha());
    /// ```
    fn is_hbr_accent_merkha(&self) -> bool;

    /// Checks if the given character is a HBR accent merkha-kefula.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֦'.is_hbr_accent_merkha_kefula());
    /// ```
    fn is_hbr_accent_merkha_kefula(&self) -> bool;

    /// Checks if the given character is a HBR accent darga.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֧'.is_hbr_accent_darga());
    /// ```
    fn is_hbr_accent_darga(&self) -> bool;

    /// Checks if the given character is a HBR accent qadma.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֨'.is_hbr_accent_qadma());
    /// ```
    fn is_hbr_accent_qadma(&self) -> bool;

    /// Checks if the given character is a HBR accent telisha-qetana.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֩'.is_hbr_accent_telisha_qetana());
    /// ```
    fn is_hbr_accent_telisha_qetana(&self) -> bool;

    /// Checks if the given character is a HBR accent yerah-ben-yomo.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֪'.is_hbr_accent_yerah_ben_yomo());
    /// ```
    fn is_hbr_accent_yerah_ben_yomo(&self) -> bool;

    /// Checks if the given character is a HBR accent ole.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֫'.is_hbr_accent_ole());
    /// ```
    fn is_hbr_accent_ole(&self) -> bool;

    /// Checks if the given character is a HBR accent iluy.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֬'.is_hbr_accent_iluy());
    /// ```
    fn is_hbr_accent_iluy(&self) -> bool;

    /// Checks if the given character is a HBR accent dehi.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֭'.is_hbr_accent_dehi());
    /// ```
    fn is_hbr_accent_dehi(&self) -> bool;

    /// Checks if the given character is a HBR accent zinor.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֮'.is_hbr_accent_zinor());
    /// ```
    fn is_hbr_accent_zinor(&self) -> bool;

    /// Checks if the given character is a HBR mark masora-circle.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('֯'.is_hbr_mark_masora_circle());
    /// ```
    fn is_hbr_mark_masora_circle(&self) -> bool;

    /// Checks if the given character is a HBR mark upper-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ׄ'.is_hbr_mark_upper_dot());
    /// ```
    fn is_hbr_mark_upper_dot(&self) -> bool;

    /// Checks if the given character is a HBR mark lower-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ׅ'.is_hbr_mark_lower_dot());
    /// ```
    fn is_hbr_mark_lower_dot(&self) -> bool;

    /// Checks if the given character is a HBR punctuation maqaf.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('־'.is_hbr_punctuation_maqaf());
    /// ```
    fn is_hbr_punctuation_maqaf(&self) -> bool;

    /// Checks if the given character is a HBR punctuation paseq.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('׀'.is_hbr_punctuation_paseq());
    /// ```
    fn is_hbr_punctuation_paseq(&self) -> bool;

    /// Checks if the given character is a HBR punctuation sof-pasuq.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('׃'.is_hbr_punctuation_sof_pasuq());
    /// ```
    fn is_hbr_punctuation_sof_pasuq(&self) -> bool;

    /// Checks if the given character is a HBR punctuation nun-hafukha.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('׆'.is_hbr_punctuation_nun_hafukha());
    /// ```
    fn is_hbr_punctuation_nun_hafukha(&self) -> bool;

    /// Checks if the given character is a HBR punctuation geresh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('׳'.is_hbr_punctuation_geresh());
    /// ```
    fn is_hbr_punctuation_geresh(&self) -> bool;

    /// Checks if the given character is a HBR punctuation gershayim.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('״'.is_hbr_punctuation_gershayim());
    /// ```
    fn is_hbr_punctuation_gershayim(&self) -> bool;

    /// Checks if the given character is a HBR yod-triangle.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('\u{05EF}'.is_hbr_yod_triangle());
    /// ```
    fn is_hbr_yod_triangle(&self) -> bool;

    /// Checks if the given character is a HBR ligature yiddisch-double-vav.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('װ'.is_hbr_ligature_yiddisch_double_vav());
    /// ```
    fn is_hbr_ligature_yiddisch_double_vav(&self) -> bool;
    /// Checks if the given character is a HBR ligature yiddisch-vav-yod.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ױ'.is_hbr_ligature_yiddisch_vav_yod());
    /// ```
    fn is_hbr_ligature_yiddisch_vav_yod(&self) -> bool;
    /// Checks if the given character is a HBR ligature yiddisch-double-yod.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ײ'.is_hbr_ligature_yiddisch_double_yod());
    /// ```
    fn is_hbr_ligature_yiddisch_double_yod(&self) -> bool;

    // source:: unicode_block_apf.rs

    /// Checks if the given character is an APF point judeo-spanish_varika.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬞ'.is_apf_point_judeo_spanish_varika());
    /// ```
    fn is_apf_point_judeo_spanish_varika(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: yod & hiriq.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('יִ'.is_apf_consonant_vowel_yod_hiriq());
    /// ```
    fn is_apf_consonant_vowel_yod_hiriq(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: shin & shin-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('שׁ'.is_apf_consonant_vowel_shin_shindot());
    /// ```
    fn is_apf_consonant_vowel_shin_shindot(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: shin & sin-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('שׂ'.is_apf_consonant_vowel_shin_sindot());
    /// ```
    fn is_apf_consonant_vowel_shin_sindot(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: shin & dagesh & shin-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('שּׁ'.is_apf_consonant_vowel_shin_dagesh_shindot());
    /// ```
    fn is_apf_consonant_vowel_shin_dagesh_shindot(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: shin & dagesh & sin-dot.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('שּׂ'.is_apf_consonant_vowel_shin_dagesh_sindot());
    /// ```
    fn is_apf_consonant_vowel_shin_dagesh_sindot(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: alef & patah.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('אַ'.is_apf_consonant_vowel_alef_patah());
    /// ```
    fn is_apf_consonant_vowel_alef_patah(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: alef & qamats.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('אָ'.is_apf_consonant_vowel_alef_qamats());
    /// ```
    fn is_apf_consonant_vowel_alef_qamats(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: alef & mapiq.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('אּ'.is_apf_consonant_vowel_alef_mapiq());
    /// ```
    fn is_apf_consonant_vowel_alef_mapiq(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: bet & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('בּ'.is_apf_consonant_vowel_bet_dagesh());
    /// ```
    fn is_apf_consonant_vowel_bet_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: gimmel & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('גּ'.is_apf_consonant_vowel_gimmel_dagesh());
    /// ```
    fn is_apf_consonant_vowel_gimmel_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: dalet & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('דּ'.is_apf_consonant_vowel_dalet_dagesh());
    /// ```
    fn is_apf_consonant_vowel_dalet_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: he & mapiq.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('הּ'.is_apf_consonant_vowel_he_mapiq());
    /// ```
    fn is_apf_consonant_vowel_he_mapiq(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: vav & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('וּ'.is_apf_consonant_vowel_vav_dagesh());
    /// ```
    fn is_apf_consonant_vowel_vav_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: zayin & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('זּ'.is_apf_consonant_vowel_zayin_dagesh());
    /// ```
    fn is_apf_consonant_vowel_zayin_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: tet & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('טּ'.is_apf_consonant_vowel_tet_dagesh());
    /// ```
    fn is_apf_consonant_vowel_tet_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: yod & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('יּ'.is_apf_consonant_vowel_yod_dagesh());
    /// ```
    fn is_apf_consonant_vowel_yod_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: final-kaf & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ךּ'.is_apf_consonant_vowel_final_kaf_dagesh());
    /// ```
    fn is_apf_consonant_vowel_final_kaf_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: kaf & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('כּ'.is_apf_consonant_vowel_kaf_dagesh());
    /// ```
    fn is_apf_consonant_vowel_kaf_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: lamed & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('לּ'.is_apf_consonant_vowel_lamed_dagesh());
    /// ```
    fn is_apf_consonant_vowel_lamed_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: mem & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('מּ'.is_apf_consonant_vowel_mem_dagesh());
    /// ```
    fn is_apf_consonant_vowel_mem_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: nun & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('נּ'.is_apf_consonant_vowel_nun_dagesh());
    /// ```
    fn is_apf_consonant_vowel_nun_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: samekh & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('סּ'.is_apf_consonant_vowel_samekh_dagesh());
    /// ```
    fn is_apf_consonant_vowel_samekh_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: final-pe & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ףּ'.is_apf_consonant_vowel_final_pe_dagesh());
    /// ```
    fn is_apf_consonant_vowel_final_pe_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: pe & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('פּ'.is_apf_consonant_vowel_pe_dagesh());
    /// ```
    fn is_apf_consonant_vowel_pe_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: tsadi & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('צּ'.is_apf_consonant_vowel_tsadi_dagesh());
    /// ```
    fn is_apf_consonant_vowel_tsadi_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: qof & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('קּ'.is_apf_consonant_vowel_qof_dagesh());
    /// ```
    fn is_apf_consonant_vowel_qof_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: resh & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('רּ'.is_apf_consonant_vowel_resh_dagesh());
    /// ```
    fn is_apf_consonant_vowel_resh_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: shin & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('שּ'.is_apf_consonant_vowel_shin_dagesh());
    /// ```
    fn is_apf_consonant_vowel_shin_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: tav & dagesh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('תּ'.is_apf_consonant_vowel_tav_dagesh());
    /// ```
    fn is_apf_consonant_vowel_tav_dagesh(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: vav & holam.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('וֹ'.is_apf_consonant_vowel_vav_holam());
    /// ```
    fn is_apf_consonant_vowel_vav_holam(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: bet & rafe.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('בֿ'.is_apf_consonant_vowel_bet_rafe());
    /// ```
    fn is_apf_consonant_vowel_bet_rafe(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: kaf & rafe.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('כֿ'.is_apf_consonant_vowel_kaf_rafe());
    /// ```
    fn is_apf_consonant_vowel_kaf_rafe(&self) -> bool;

    /// Checks if the given character is an APF consonant with vowel: pe & rafe.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('פֿ'.is_apf_consonant_vowel_pe_rafe());
    /// ```
    fn is_apf_consonant_vowel_pe_rafe(&self) -> bool;

    /// Checks if the given character is an APF letter alternative-ayin.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬠ'.is_apf_consonant_alternative_ayin());
    /// ```
    fn is_apf_consonant_alternative_ayin(&self) -> bool;

    /// Checks if the given character is an APF letter alternative-plus-sign.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('﬩'.is_apf_letter_alternative_plus_sign());
    /// ```
    fn is_apf_letter_alternative_plus_sign(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-alef.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬡ'.is_apf_consonant_wide_alef());
    /// ```
    fn is_apf_consonant_wide_alef(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-dalet.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬢ'.is_apf_consonant_wide_dalet());
    /// ```
    fn is_apf_consonant_wide_dalet(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-he.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬣ'.is_apf_consonant_wide_he());
    /// ```
    fn is_apf_consonant_wide_he(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-kaf.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬤ'.is_apf_consonant_wide_kaf());
    /// ```
    fn is_apf_consonant_wide_kaf(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-lamed.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬥ'.is_apf_consonant_wide_lamed());
    /// ```
    fn is_apf_consonant_wide_lamed(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-final-mem.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬦ'.is_apf_consonant_wide_final_mem());
    /// ```
    fn is_apf_consonant_wide_final_mem(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-resh.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬧ'.is_apf_consonant_wide_resh());
    /// ```
    fn is_apf_consonant_wide_resh(&self) -> bool;

    /// Checks if the given character is an APF consonant wide-tav.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﬨ'.is_apf_consonant_wide_tav());
    /// ```
    fn is_apf_consonant_wide_tav(&self) -> bool;

    /// Checks if the given character is an APF ligature yiddisch-yod-yod-patah.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ײַ'.is_apf_ligature_yiddisch_yod_yod_patah());
    /// ```
    fn is_apf_ligature_yiddisch_yod_yod_patah(&self) -> bool;

    /// Checks if the given character is an APF ligature alef-lamed.
    /// # Example
    /// ```
    /// use hebrew_unicode_script::HebrewUnicodeScript;
    ///
    /// assert!('ﭏ'.is_apf_ligature_alef_lamed());
    /// ```
    fn is_apf_ligature_alef_lamed(&self) -> bool;
}
