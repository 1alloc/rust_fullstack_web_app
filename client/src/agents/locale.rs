use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

#[derive(Debug)]
pub enum Request {
    SetLocale(String),
}

#[derive(Debug)]
pub enum Action {
    ChangeLocale(String),
}

pub struct LocaleStored {
    pub locale: String,
}

impl Store for LocaleStored {
    type Action = Action;
    type Input = Request;

    fn new() -> Self {
        let locale = wasm_cookies::get_raw("language").unwrap();

        LocaleStored {
            locale,
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Request::SetLocale(locale) => {
                link.send_message(Action::ChangeLocale(locale));
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::ChangeLocale(lang) => {
                self.locale = lang;
            }
        }
    }
}