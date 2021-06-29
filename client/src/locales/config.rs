#[derive(Copy, Clone, Debug)]
pub enum Locale {
    En,
    Ru,
}

impl Locale {
    pub fn get(&self) -> Locale {
        match *self {
            Locale::En => Locale::En,
            Locale::Ru => Locale::Ru,
        }
    }
}