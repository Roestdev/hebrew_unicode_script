//use crate::collections_api::unicode_block_alphabetic_presentation_form::*;
//use crate::collections_api::unicode_block_hebrew::*;
//use crate::collections_api::unicode_script_hebrew::*;
use crate::*;
/// A trait for identification and validation of Hebrew characters
///
impl HebrewUnicodeScript for char {
    // source:: collections_api.rs
    // module:: Unicode Script Hebrew

    fn is_script_hbr(&self) -> bool {
        is_script_hbr(*self)
    }
    fn is_script_hbr_point(&self) -> bool {
        is_script_hbr_point(*self)
    }
    fn is_script_hbr_point_reading_sign(&self) -> bool {
        is_script_hbr_point_reading_sign(*self)
    }
    fn is_script_hbr_consonant(&self) -> bool {
        is_script_hbr_consonant(*self)
    }
    fn is_script_hbr_ligature_yiddisch(&self) -> bool {
        is_script_hbr_ligature_yiddisch(*self)
    }
    fn is_script_hbr_ligature(&self) -> bool {
        is_script_hbr_ligature(*self)
    }

    // source:: collections_api.rs
    // module:: Unicode Block: Hebrew

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
    fn is_hbr_ligature_yiddish(&self) -> bool {
        is_hbr_ligature_yiddish(*self)
    }

    // source:: collections_api.rs
    // module:: Unicode Block Alphabetic Presentation Form

    fn is_apf_block(&self) -> bool {
        is_apf_block(*self)
    }
    fn is_apf_consonant(&self) -> bool {
        is_apf_consonant(*self)
    }
    fn is_apf_consonant_with_vowel(&self) -> bool {
        is_apf_consonant_with_vowel(*self)
    }
    fn is_apf_point_reading_sign(&self) -> bool {
        is_apf_point_reading_sign(*self)
    }
    fn is_apf_ligature(&self) -> bool {
        is_apf_ligature(*self)
    }
    fn is_apf_alternative(&self) -> bool {
        is_apf_alternative(*self)
    }
    fn is_apf_consonant_wide(&self) -> bool {
        is_apf_consonant_wide(*self)
    }

    fn is_hbr_consonant_alef(&self) -> bool {
        is_hbr_consonant_alef(*self)
    }
    fn is_hbr_consonant_bet(&self) -> bool {
        is_hbr_consonant_bet(*self)
    }
    fn is_hbr_consonant_gimel(&self) -> bool {
        is_hbr_consonant_gimel(*self)
    }
    fn is_hbr_consonant_dalet(&self) -> bool {
        is_hbr_consonant_dalet(*self)
    }
    fn is_hbr_consonant_he(&self) -> bool {
        is_hbr_consonant_he(*self)
    }
    fn is_hbr_consonant_vav(&self) -> bool {
        is_hbr_consonant_vav(*self)
    }
    fn is_hbr_consonant_zayin(&self) -> bool {
        is_hbr_consonant_zayin(*self)
    }
    fn is_hbr_consonant_het(&self) -> bool {
        is_hbr_consonant_het(*self)
    }
    fn is_hbr_consonant_tet(&self) -> bool {
        is_hbr_consonant_tet(*self)
    }
    fn is_hbr_consonant_yod(&self) -> bool {
        is_hbr_consonant_yod(*self)
    }
    fn is_hbr_consonant_final_kaf(&self) -> bool {
        is_hbr_consonant_final_kaf(*self)
    }
    fn is_hbr_consonant_kaf(&self) -> bool {
        is_hbr_consonant_kaf(*self)
    }
    fn is_hbr_consonant_lamed(&self) -> bool {
        is_hbr_consonant_lamed(*self)
    }
    fn is_hbr_consonant_final_mem(&self) -> bool {
        is_hbr_consonant_final_mem(*self)
    }
    fn is_hbr_consonant_mem(&self) -> bool {
        is_hbr_consonant_mem(*self)
    }
    fn is_hbr_consonant_final_nun(&self) -> bool {
        is_hbr_consonant_final_nun(*self)
    }
    fn is_hbr_consonant_nun(&self) -> bool {
        is_hbr_consonant_nun(*self)
    }
    fn is_hbr_consonant_samekh(&self) -> bool {
        is_hbr_consonant_samekh(*self)
    }
    fn is_hbr_consonant_ayin(&self) -> bool {
        is_hbr_consonant_ayin(*self)
    }
    fn is_hbr_consonant_final_pe(&self) -> bool {
        is_hbr_consonant_final_pe(*self)
    }
    fn is_hbr_consonant_pe(&self) -> bool {
        is_hbr_consonant_pe(*self)
    }
    fn is_hbr_consonant_final_tsadi(&self) -> bool {
        is_hbr_consonant_final_tsadi(*self)
    }
    fn is_hbr_consonant_tsadi(&self) -> bool {
        is_hbr_consonant_tsadi(*self)
    }
    fn is_hbr_consonant_qof(&self) -> bool {
        is_hbr_consonant_qof(*self)
    }
    fn is_hbr_consonant_resh(&self) -> bool {
        is_hbr_consonant_resh(*self)
    }
    fn is_hbr_consonant_shin(&self) -> bool {
        is_hbr_consonant_shin(*self)
    }
    fn is_hbr_consonant_tav(&self) -> bool {
        is_hbr_consonant_tav(*self)
    }
    fn is_hbr_point_sheva(&self) -> bool {
        is_hbr_point_sheva(*self)
    }
    fn is_hbr_point_hataf_segol(&self) -> bool {
        is_hbr_point_hataf_segol(*self)
    }
    fn is_hbr_point_hataf_patah(&self) -> bool {
        is_hbr_point_hataf_patah(*self)
    }
    fn is_hbr_point_hataf_qamats(&self) -> bool {
        is_hbr_point_hataf_qamats(*self)
    }
    fn is_hbr_point_hiriq(&self) -> bool {
        is_hbr_point_hiriq(*self)
    }
    fn is_hbr_point_tsere(&self) -> bool {
        is_hbr_point_tsere(*self)
    }
    fn is_hbr_point_segol(&self) -> bool {
        is_hbr_point_segol(*self)
    }
    fn is_hbr_point_patah(&self) -> bool {
        is_hbr_point_patah(*self)
    }
    fn is_hbr_point_qamats(&self) -> bool {
        is_hbr_point_qamats(*self)
    }
    fn is_hbr_point_holam(&self) -> bool {
        is_hbr_point_holam(*self)
    }
    fn is_hbr_point_holam_haser_for_vav(&self) -> bool {
        is_hbr_point_holam_haser_for_vav(*self)
    }
    fn is_hbr_point_qubuts(&self) -> bool {
        is_hbr_point_qubuts(*self)
    }
    fn is_hbr_point_dagesh_or_mapiq(&self) -> bool {
        is_hbr_point_dagesh_or_mapiq(*self)
    }
    fn is_hbr_point_meteg(&self) -> bool {
        is_hbr_point_meteg(*self)
    }
    fn is_hbr_point_rafe(&self) -> bool {
        is_hbr_point_rafe(*self)
    }
    fn is_hbr_point_shin_dot(&self) -> bool {
        is_hbr_point_shin_dot(*self)
    }
    fn is_hbr_point_sin_dot(&self) -> bool {
        is_hbr_point_sin_dot(*self)
    }
    fn is_hbr_point_qamats_qatan(&self) -> bool {
        is_hbr_point_qamats_qatan(*self)
    }
    fn is_hbr_accent_etnahta(&self) -> bool {
        is_hbr_accent_etnahta(*self)
    }
    fn is_hbr_accent_segol(&self) -> bool {
        is_hbr_accent_segol(*self)
    }
    fn is_hbr_accent_shalshelet(&self) -> bool {
        is_hbr_accent_shalshelet(*self)
    }
    fn is_hbr_accent_zaqef_qatan(&self) -> bool {
        is_hbr_accent_zaqef_qatan(*self)
    }
    fn is_hbr_accent_zaqef_gadol(&self) -> bool {
        is_hbr_accent_zaqef_gadol(*self)
    }
    fn is_hbr_accent_tipeha(&self) -> bool {
        is_hbr_accent_tipeha(*self)
    }
    fn is_hbr_accent_revia(&self) -> bool {
        is_hbr_accent_revia(*self)
    }
    fn is_hbr_accent_zarqa(&self) -> bool {
        is_hbr_accent_zarqa(*self)
    }
    fn is_hbr_accent_pashta(&self) -> bool {
        is_hbr_accent_pashta(*self)
    }
    fn is_hbr_accent_yetiv(&self) -> bool {
        is_hbr_accent_yetiv(*self)
    }
    fn is_hbr_accent_tevir(&self) -> bool {
        is_hbr_accent_tevir(*self)
    }
    fn is_hbr_accent_geresh(&self) -> bool {
        is_hbr_accent_geresh(*self)
    }
    fn is_hbr_accent_geresh_muqdam(&self) -> bool {
        is_hbr_accent_geresh_muqdam(*self)
    }
    fn is_hbr_accent_gershayim(&self) -> bool {
        is_hbr_accent_gershayim(*self)
    }
    fn is_hbr_accent_qarney_para(&self) -> bool {
        is_hbr_accent_qarney_para(*self)
    }
    fn is_hbr_accent_telisha_gedola(&self) -> bool {
        is_hbr_accent_telisha_gedola(*self)
    }
    fn is_hbr_accent_pazer(&self) -> bool {
        is_hbr_accent_pazer(*self)
    }
    fn is_hbr_accent_atnah_hafukh(&self) -> bool {
        is_hbr_accent_atnah_hafukh(*self)
    }
    fn is_hbr_accent_munah(&self) -> bool {
        is_hbr_accent_munah(*self)
    }
    fn is_hbr_accent_mahapakh(&self) -> bool {
        is_hbr_accent_mahapakh(*self)
    }
    fn is_hbr_accent_merkha(&self) -> bool {
        is_hbr_accent_merkha(*self)
    }
    fn is_hbr_accent_merkha_kefula(&self) -> bool {
        is_hbr_accent_merkha_kefula(*self)
    }
    fn is_hbr_accent_darga(&self) -> bool {
        is_hbr_accent_darga(*self)
    }
    fn is_hbr_accent_qadma(&self) -> bool {
        is_hbr_accent_qadma(*self)
    }
    fn is_hbr_accent_telisha_qetana(&self) -> bool {
        is_hbr_accent_telisha_qetana(*self)
    }
    fn is_hbr_accent_yerah_ben_yomo(&self) -> bool {
        is_hbr_accent_yerah_ben_yomo(*self)
    }
    fn is_hbr_accent_ole(&self) -> bool {
        is_hbr_accent_ole(*self)
    }
    fn is_hbr_accent_iluy(&self) -> bool {
        is_hbr_accent_iluy(*self)
    }
    fn is_hbr_accent_dehi(&self) -> bool {
        is_hbr_accent_dehi(*self)
    }
    fn is_hbr_accent_zinor(&self) -> bool {
        is_hbr_accent_zinor(*self)
    }
    fn is_hbr_mark_masora_circle(&self) -> bool {
        is_hbr_mark_masora_circle(*self)
    }
    fn is_hbr_mark_upper_dot(&self) -> bool {
        is_hbr_mark_upper_dot(*self)
    }
    fn is_hbr_mark_lower_dot(&self) -> bool {
        is_hbr_mark_lower_dot(*self)
    }
    fn is_hbr_punctuation_maqaf(&self) -> bool {
        is_hbr_punctuation_maqaf(*self)
    }
    fn is_hbr_punctuation_paseq(&self) -> bool {
        is_hbr_punctuation_paseq(*self)
    }
    fn is_hbr_punctuation_sof_pasuq(&self) -> bool {
        is_hbr_punctuation_sof_pasuq(*self)
    }
    fn is_hbr_punctuation_nun_hafukha(&self) -> bool {
        is_hbr_punctuation_nun_hafukha(*self)
    }
    fn is_hbr_punctuation_geresh(&self) -> bool {
        is_hbr_punctuation_geresh(*self)
    }
    fn is_hbr_punctuation_gershayim(&self) -> bool {
        is_hbr_punctuation_gershayim(*self)
    }
    fn is_hbr_yod_triangle(&self) -> bool {
        is_hbr_yod_triangle(*self)
    }
    fn is_hbr_ligature_yiddisch_double_vav(&self) -> bool {
        is_hbr_ligature_yiddisch_double_vav(*self)
    }
    fn is_hbr_ligature_yiddisch_vav_yod(&self) -> bool {
        is_hbr_ligature_yiddisch_vav_yod(*self)
    }
    fn is_hbr_ligature_yiddisch_double_yod(&self) -> bool {
        is_hbr_ligature_yiddisch_double_yod(*self)
    }

    // source:: unicode_block_apf.rs

    fn is_apf_point_judeo_spanish_varika(&self) -> bool {
        is_apf_point_judeo_spanish_varika(*self)
    }
    fn is_apf_consonant_vowel_yod_hiriq(&self) -> bool {
        is_apf_consonant_vowel_yod_hiriq(*self)
    }
    fn is_apf_consonant_vowel_shin_shindot(&self) -> bool {
        is_apf_consonant_vowel_shin_shindot(*self)
    }
    fn is_apf_consonant_vowel_shin_sindot(&self) -> bool {
        is_apf_consonant_vowel_shin_sindot(*self)
    }
    fn is_apf_consonant_vowel_shin_dagesh_shindot(&self) -> bool {
        is_apf_consonant_vowel_shin_dagesh_shindot(*self)
    }
    fn is_apf_consonant_vowel_shin_dagesh_sindot(&self) -> bool {
        is_apf_consonant_vowel_shin_dagesh_sindot(*self)
    }
    fn is_apf_consonant_vowel_alef_patah(&self) -> bool {
        is_apf_consonant_vowel_alef_patah(*self)
    }
    fn is_apf_consonant_vowel_alef_qamats(&self) -> bool {
        is_apf_consonant_vowel_alef_qamats(*self)
    }
    fn is_apf_consonant_vowel_alef_mapiq(&self) -> bool {
        is_apf_consonant_vowel_alef_mapiq(*self)
    }
    fn is_apf_consonant_vowel_bet_dagesh(&self) -> bool {
        is_apf_consonant_vowel_bet_dagesh(*self)
    }
    fn is_apf_consonant_vowel_gimmel_dagesh(&self) -> bool {
        is_apf_consonant_vowel_gimmel_dagesh(*self)
    }
    fn is_apf_consonant_vowel_dalet_dagesh(&self) -> bool {
        is_apf_consonant_vowel_dalet_dagesh(*self)
    }
    fn is_apf_consonant_vowel_he_mapiq(&self) -> bool {
        is_apf_consonant_vowel_he_mapiq(*self)
    }
    fn is_apf_consonant_vowel_vav_dagesh(&self) -> bool {
        is_apf_consonant_vowel_vav_dagesh(*self)
    }
    fn is_apf_consonant_vowel_zayin_dagesh(&self) -> bool {
        is_apf_consonant_vowel_zayin_dagesh(*self)
    }
    fn is_apf_consonant_vowel_tet_dagesh(&self) -> bool {
        is_apf_consonant_vowel_tet_dagesh(*self)
    }
    fn is_apf_consonant_vowel_yod_dagesh(&self) -> bool {
        is_apf_consonant_vowel_yod_dagesh(*self)
    }
    fn is_apf_consonant_vowel_final_kaf_dagesh(&self) -> bool {
        is_apf_consonant_vowel_final_kaf_dagesh(*self)
    }
    fn is_apf_consonant_vowel_kaf_dagesh(&self) -> bool {
        is_apf_consonant_vowel_kaf_dagesh(*self)
    }
    fn is_apf_consonant_vowel_lamed_dagesh(&self) -> bool {
        is_apf_consonant_vowel_lamed_dagesh(*self)
    }
    fn is_apf_consonant_vowel_mem_dagesh(&self) -> bool {
        is_apf_consonant_vowel_mem_dagesh(*self)
    }
    fn is_apf_consonant_vowel_nun_dagesh(&self) -> bool {
        is_apf_consonant_vowel_nun_dagesh(*self)
    }
    fn is_apf_consonant_vowel_samekh_dagesh(&self) -> bool {
        is_apf_consonant_vowel_samekh_dagesh(*self)
    }
    fn is_apf_consonant_vowel_final_pe_dagesh(&self) -> bool {
        is_apf_consonant_vowel_final_pe_dagesh(*self)
    }
    fn is_apf_consonant_vowel_pe_dagesh(&self) -> bool {
        is_apf_consonant_vowel_pe_dagesh(*self)
    }
    fn is_apf_consonant_vowel_tsadi_dagesh(&self) -> bool {
        is_apf_consonant_vowel_tsadi_dagesh(*self)
    }
    fn is_apf_consonant_vowel_qof_dagesh(&self) -> bool {
        is_apf_consonant_vowel_qof_dagesh(*self)
    }
    fn is_apf_consonant_vowel_resh_dagesh(&self) -> bool {
        is_apf_consonant_vowel_resh_dagesh(*self)
    }
    fn is_apf_consonant_vowel_shin_dagesh(&self) -> bool {
        is_apf_consonant_vowel_shin_dagesh(*self)
    }
    fn is_apf_consonant_vowel_tav_dagesh(&self) -> bool {
        is_apf_consonant_vowel_tav_dagesh(*self)
    }
    fn is_apf_consonant_vowel_vav_holam(&self) -> bool {
        is_apf_consonant_vowel_vav_holam(*self)
    }
    fn is_apf_consonant_vowel_bet_rafe(&self) -> bool {
        is_apf_consonant_vowel_bet_rafe(*self)
    }
    fn is_apf_consonant_vowel_kaf_rafe(&self) -> bool {
        is_apf_consonant_vowel_kaf_rafe(*self)
    }
    fn is_apf_consonant_vowel_pe_rafe(&self) -> bool {
        is_apf_consonant_vowel_pe_rafe(*self)
    }
    fn is_apf_consonant_alternative_ayin(&self) -> bool {
        is_apf_consonant_alternative_ayin(*self)
    }
    fn is_apf_letter_alternative_plus_sign(&self) -> bool {
        is_apf_letter_alternative_plus_sign(*self)
    }
    fn is_apf_consonant_wide_alef(&self) -> bool {
        is_apf_consonant_wide_alef(*self)
    }
    fn is_apf_consonant_wide_dalet(&self) -> bool {
        is_apf_consonant_wide_dalet(*self)
    }
    fn is_apf_consonant_wide_he(&self) -> bool {
        is_apf_consonant_wide_he(*self)
    }
    fn is_apf_consonant_wide_kaf(&self) -> bool {
        is_apf_consonant_wide_kaf(*self)
    }
    fn is_apf_consonant_wide_lamed(&self) -> bool {
        is_apf_consonant_wide_lamed(*self)
    }
    fn is_apf_consonant_wide_final_mem(&self) -> bool {
        is_apf_consonant_wide_final_mem(*self)
    }
    fn is_apf_consonant_wide_resh(&self) -> bool {
        is_apf_consonant_wide_resh(*self)
    }
    fn is_apf_consonant_wide_tav(&self) -> bool {
        is_apf_consonant_wide_tav(*self)
    }
    fn is_apf_ligature_yiddisch_yod_yod_patah(&self) -> bool {
        is_apf_ligature_yiddisch_yod_yod_patah(*self)
    }
    fn is_apf_ligature_alef_lamed(&self) -> bool {
        is_apf_ligature_alef_lamed(*self)
    }
}

#[cfg(test)]
mod test_ {
    use super::*;
    //unicode script Hebrew (collections)
    #[test]
    fn test_script_hebrew() {
        assert!(!'a'.is_script_hbr());
    }

    #[test]
    fn test_script_hebrew_point() {
        assert!(!'a'.is_script_hbr_point());
    }

    #[test]
    fn test_script_hebrew_point_reading_sign() {
        assert!(!'a'.is_script_hbr_point_reading_sign());
    }
    #[test]
    fn test_script_hebrew_consonant() {
        assert!(!'a'.is_script_hbr_consonant());
    }

    #[test]
    fn test_script_hebrew_ligature_yiddisch() {
        assert!(!'a'.is_script_hbr_ligature_yiddisch());
    }

    #[test]
    fn test_script_hebrew_ligature() {
        assert!(!'a'.is_script_hbr_ligature());
    }
    //unicode block Hebrew (collections)
    #[test]
    fn test_hbr_block() {
        assert!(!'a'.is_hbr_block());
    }

    #[test]
    fn test_hbr_accent() {
        assert!(!'a'.is_hbr_accent());
    }

    #[test]
    fn test_hbr_mark() {
        assert!(!'a'.is_hbr_mark());
    }

    #[test]
    fn test_hbr_point() {
        assert!(!'a'.is_hbr_point());
    }

    #[test]
    fn test_hbr_point_vowel() {
        assert!(!'a'.is_hbr_point_vowel());
    }

    #[test]
    fn test_hbr_point_semi_vowel() {
        assert!(!'a'.is_hbr_point_semi_vowel());
    }

    #[test]
    fn test_hbr_point_reading_sign() {
        assert!(!'a'.is_hbr_point_reading_sign());
    }

    #[test]
    fn test_hbr_punctuation() {
        assert!(!'a'.is_hbr_punctuation());
    }

    #[test]
    fn test_hbr_consonant() {
        assert!(!'a'.is_hbr_consonant());
    }

    #[test]
    fn test_hbr_consonant_normal() {
        assert!(!'a'.is_hbr_consonant_normal());
    }

    #[test]
    fn test_hbr_consonant_final() {
        assert!(!'a'.is_hbr_consonant_final());
    }

    #[test]
    fn test_hbr_ligature_yiddish() {
        assert!(!'a'.is_hbr_ligature_yiddish());
    }
    //unicode bolck APF (collections)
    #[test]
    fn test_apf_block() {
        assert!(!'a'.is_apf_block());
    }

    #[test]
    fn test_apf_point_reading_sign() {
        assert!(!'a'.is_apf_point_reading_sign());
    }

    #[test]
    fn test_apf_consonant() {
        assert!(!'a'.is_apf_consonant());
    }

    #[test]
    fn test_apf_alternative() {
        assert!(!'a'.is_apf_alternative());
    }

    #[test]
    fn test_apf_consonant_wide() {
        assert!(!'a'.is_apf_consonant_wide());
    }

    #[test]
    fn test_apf_consonant_with_vowel() {
        assert!(!'a'.is_apf_consonant_with_vowel());
    }

    #[test]
    fn test_apf_ligature() {
        assert!(!'a'.is_apf_ligature());
    }

    #[test]
    fn test_hbr_block_consonant() {
        assert!('א'.is_hbr_consonant_alef());
        assert!('ב'.is_hbr_consonant_bet());
        assert!('ג'.is_hbr_consonant_gimel());
        assert!('ד'.is_hbr_consonant_dalet());
        assert!('ה'.is_hbr_consonant_he());
        assert!('ו'.is_hbr_consonant_vav());
        assert!('ז'.is_hbr_consonant_zayin());
        assert!('ח'.is_hbr_consonant_het());
        assert!('ט'.is_hbr_consonant_tet());
        assert!('י'.is_hbr_consonant_yod());
        assert!('ך'.is_hbr_consonant_final_kaf());
        assert!('כ'.is_hbr_consonant_kaf());
        assert!('ל'.is_hbr_consonant_lamed());
        assert!('ם'.is_hbr_consonant_final_mem());
        assert!('מ'.is_hbr_consonant_mem());
        assert!('ן'.is_hbr_consonant_final_nun());
        assert!('נ'.is_hbr_consonant_nun());
        assert!('ס'.is_hbr_consonant_samekh());
        assert!('ע'.is_hbr_consonant_ayin());
        assert!('ף'.is_hbr_consonant_final_pe());
        assert!('פ'.is_hbr_consonant_pe());
        assert!('ץ'.is_hbr_consonant_final_tsadi());
        assert!('צ'.is_hbr_consonant_tsadi());
        assert!('ק'.is_hbr_consonant_qof());
        assert!('ר'.is_hbr_consonant_resh());
        assert!('ש'.is_hbr_consonant_shin());
        assert!('ת'.is_hbr_consonant_tav());
    }

    #[test]
    fn test_hbr_block_point() {
        assert!('ְ'.is_hbr_point_sheva());
        assert!('ֱ'.is_hbr_point_hataf_segol());
        assert!('ֲ'.is_hbr_point_hataf_patah());
        assert!('ֳ'.is_hbr_point_hataf_qamats());
        assert!('ִ'.is_hbr_point_hiriq());
        assert!('ֵ'.is_hbr_point_tsere());
        assert!('ֶ'.is_hbr_point_segol());
        assert!('ַ'.is_hbr_point_patah());
        assert!('ָ'.is_hbr_point_qamats());
        assert!('ֹ'.is_hbr_point_holam());
        assert!('ֺ'.is_hbr_point_holam_haser_for_vav());
        assert!('ֻ'.is_hbr_point_qubuts());
        assert!('ּ'.is_hbr_point_dagesh_or_mapiq());
        assert!('ֽ'.is_hbr_point_meteg());
        assert!('ֿ'.is_hbr_point_rafe());
        assert!('ׁ'.is_hbr_point_shin_dot());
        assert!('ׂ'.is_hbr_point_sin_dot());
        assert!('ׇ'.is_hbr_point_qamats_qatan());
    }

    #[test]
    fn test_hbr_block_accent() {
        assert!('֑'.is_hbr_accent_etnahta());
        assert!('֒'.is_hbr_accent_segol());
        assert!('֓'.is_hbr_accent_shalshelet());
        assert!('֔'.is_hbr_accent_zaqef_qatan());
        assert!('֕'.is_hbr_accent_zaqef_gadol());
        assert!('֖'.is_hbr_accent_tipeha());
        assert!('֗'.is_hbr_accent_revia());
        assert!('֘'.is_hbr_accent_zarqa());
        assert!('֙'.is_hbr_accent_pashta());
        assert!('֚'.is_hbr_accent_yetiv());
        assert!('֛'.is_hbr_accent_tevir());
        assert!('֜'.is_hbr_accent_geresh());
        assert!('֝'.is_hbr_accent_geresh_muqdam());
        assert!('֞'.is_hbr_accent_gershayim());
        assert!('֟'.is_hbr_accent_qarney_para());
        assert!('֠'.is_hbr_accent_telisha_gedola());
        assert!('֡'.is_hbr_accent_pazer());
        assert!('֢'.is_hbr_accent_atnah_hafukh());
        assert!('֣'.is_hbr_accent_munah());
        assert!('֤'.is_hbr_accent_mahapakh());
        assert!('֥'.is_hbr_accent_merkha());
        assert!('֦'.is_hbr_accent_merkha_kefula());
        assert!('֧'.is_hbr_accent_darga());
        assert!('֨'.is_hbr_accent_qadma());
        assert!('֩'.is_hbr_accent_telisha_qetana());
        assert!('֪'.is_hbr_accent_yerah_ben_yomo());
        assert!('֫'.is_hbr_accent_ole());
        assert!('֬'.is_hbr_accent_iluy());
        assert!('֭'.is_hbr_accent_dehi());
        assert!('֮'.is_hbr_accent_zinor());
    }

    #[test]
    fn test_hbr_block_mark() {
        assert!('֯'.is_hbr_mark_masora_circle());
        assert!('ׄ'.is_hbr_mark_upper_dot());
        assert!('ׅ'.is_hbr_mark_lower_dot());
    }

    #[test]
    fn test_hbr_block_punctuation() {
        assert!('־'.is_hbr_punctuation_maqaf());
        assert!('׀'.is_hbr_punctuation_paseq());
        assert!('׃'.is_hbr_punctuation_sof_pasuq());
        assert!('׆'.is_hbr_punctuation_nun_hafukha());
        assert!('׳'.is_hbr_punctuation_geresh());
        assert!('״'.is_hbr_punctuation_gershayim());
    }

    #[test]
    fn test_hbr_block_misc() {
        assert!('\u{05EF}'.is_hbr_yod_triangle());
        assert!('װ'.is_hbr_ligature_yiddisch_double_vav());
        assert!('ױ'.is_hbr_ligature_yiddisch_vav_yod());
        assert!('ײ'.is_hbr_ligature_yiddisch_double_yod());
    }

    #[test]
    fn test_apf() {
        assert!('ﬞ'.is_apf_point_judeo_spanish_varika());
        assert!('יִ'.is_apf_consonant_vowel_yod_hiriq());
        assert!('שׁ'.is_apf_consonant_vowel_shin_shindot());
        assert!('שׂ'.is_apf_consonant_vowel_shin_sindot());
        assert!('שּׁ'.is_apf_consonant_vowel_shin_dagesh_shindot());
        assert!('שּׂ'.is_apf_consonant_vowel_shin_dagesh_sindot());
        assert!('אַ'.is_apf_consonant_vowel_alef_patah());
        assert!('אָ'.is_apf_consonant_vowel_alef_qamats());
        assert!('אּ'.is_apf_consonant_vowel_alef_mapiq());

        assert!('בּ'.is_apf_consonant_vowel_bet_dagesh());
        assert!('גּ'.is_apf_consonant_vowel_gimmel_dagesh());
        assert!('דּ'.is_apf_consonant_vowel_dalet_dagesh());
        assert!('הּ'.is_apf_consonant_vowel_he_mapiq());
        assert!('וּ'.is_apf_consonant_vowel_vav_dagesh());
        assert!('זּ'.is_apf_consonant_vowel_zayin_dagesh());
        assert!('טּ'.is_apf_consonant_vowel_tet_dagesh());
        assert!('יּ'.is_apf_consonant_vowel_yod_dagesh());
        assert!('ךּ'.is_apf_consonant_vowel_final_kaf_dagesh());
        assert!('כּ'.is_apf_consonant_vowel_kaf_dagesh());
        assert!('לּ'.is_apf_consonant_vowel_lamed_dagesh());
        assert!('מּ'.is_apf_consonant_vowel_mem_dagesh());
        assert!('נּ'.is_apf_consonant_vowel_nun_dagesh());
        assert!('סּ'.is_apf_consonant_vowel_samekh_dagesh());
        assert!('ףּ'.is_apf_consonant_vowel_final_pe_dagesh());
        assert!('פּ'.is_apf_consonant_vowel_pe_dagesh());
        assert!('צּ'.is_apf_consonant_vowel_tsadi_dagesh());
        assert!('קּ'.is_apf_consonant_vowel_qof_dagesh());
        assert!('רּ'.is_apf_consonant_vowel_resh_dagesh());
        assert!('שּ'.is_apf_consonant_vowel_shin_dagesh());
        assert!('תּ'.is_apf_consonant_vowel_tav_dagesh());
        assert!('וֹ'.is_apf_consonant_vowel_vav_holam());
        assert!('בֿ'.is_apf_consonant_vowel_bet_rafe());
        assert!('כֿ'.is_apf_consonant_vowel_kaf_rafe());
        assert!('פֿ'.is_apf_consonant_vowel_pe_rafe());
        assert!('ﬠ'.is_apf_consonant_alternative_ayin());
        assert!('﬩'.is_apf_letter_alternative_plus_sign());

        assert!('ﬡ'.is_apf_consonant_wide_alef());
        assert!('ﬢ'.is_apf_consonant_wide_dalet());
        assert!('ﬣ'.is_apf_consonant_wide_he());

        assert!('ﬤ'.is_apf_consonant_wide_kaf());
        assert!('ﬥ'.is_apf_consonant_wide_lamed());
        assert!('ﬦ'.is_apf_consonant_wide_final_mem());
        assert!('ﬧ'.is_apf_consonant_wide_resh());
        assert!('ﬨ'.is_apf_consonant_wide_tav());
        assert!('ײַ'.is_apf_ligature_yiddisch_yod_yod_patah());
        assert!('ﭏ'.is_apf_ligature_alef_lamed());
    }
}
