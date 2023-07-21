wit_bindgen_rust::export!("unicode_normalizer.wit");

use unicode_normalization::UnicodeNormalization;

struct UnicodeNormalizer;
impl unicode_normalizer::UnicodeNormalizer for UnicodeNormalizer {
    fn normalize_unicode_nfd(input: String) -> String {
        input.nfd().collect::<String>()
    }

    fn normalize_unicode_nfc(input: String) -> String {
        input.nfc().collect::<String>()
    }
}
