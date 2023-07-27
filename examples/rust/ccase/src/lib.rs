
wit_bindgen_rust::export!("ccase.wit");
use convert_case::{Case, Casing};

struct Ccase;

impl crate::ccase::Ccase for Ccase {
    fn change_case(s: String, case_type: u32) -> String {
        match case_type {
            1 => s.to_case(Case::Alternating),
            2 => s.to_case(Case::Camel),
            3 => s.to_case(Case::Cobol),
            4 => s.to_case(Case::Flat),
            5 => s.to_case(Case::Kebab),
            6 => s.to_case(Case::Lower),
            7 => s.to_case(Case::Pascal),
            8 => s.to_case(Case::Random),
            9 => s.to_case(Case::Random),
            10 => s.to_case(Case::ScreamingSnake),
            11 => s.to_case(Case::Snake),
            12 => s.to_case(Case::Title),
            13 => s.to_case(Case::Toggle),
            14 => s.to_case(Case::Train),
            15 => s.to_case(Case::Upper),
            16 => s.to_case(Case::UpperCamel),
            17 => s.to_case(Case::UpperFlat),
            18 => s.to_case(Case::UpperKebab),
            19 => s.to_case(Case::UpperSnake),
            _ => s
        }
    }
}
