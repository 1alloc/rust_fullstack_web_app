use i18n_codegen::i18n;

i18n!("src/locales/translate");

impl Locale {
    pub fn get(selected_language: &String) -> Locale {
        match selected_language.as_str() {
            "english" => Locale::En,
            "russian" => Locale::Ru,
            "romanian" => Locale::Ro,
            _ => Locale::En
        }
    }
}