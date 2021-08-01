use i18n_codegen::i18n;
use stdweb::js;
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

    pub fn html_lang_set(lang: &str) {
        match lang {
            "english" => js! { @(no_return) document.documentElement.lang = "en-EN";
                    window.document.title = "1alloc.com • about programming and not only";
                    },
            "russian" => js! { @(no_return) document.documentElement.lang = "ru-RU";
                    window.document.title = "1alloc.com • о программировании и не только";
                    },
            "romanian" => js! { @(no_return) document.documentElement.lang = "ro-RO";
                    window.document.title = "1alloc.com • despre programare și nu numai";
                    },
            _ => js! { @(no_return) document.documentElement.lang = "en-EN";
                    window.document.title = "1alloc.com • about programming and not only";
                    },
        }
    }
}