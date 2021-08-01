use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use crate::locales::config;
use crate::agents::locale::LocaleStored;

pub enum Msg {
    LocaleStore(ReadOnly<LocaleStored>),
}

pub struct Login {
    locale: config::Locale,
    locale_agent: Box<dyn Bridge<StoreWrapper<LocaleStored>>>,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let locale = config::Locale::get(&wasm_cookies::get_raw("language").unwrap());
        let locale_agent = LocaleStored::bridge(link.callback(Msg::LocaleStore));
        Self {
            locale,
            locale_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LocaleStore(lang) => {
                self.locale = config::Locale::get(&lang.borrow().locale);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.view_login() }
            </div>
        }
    }
}
impl Login {
    fn view_login(&self) -> Html {
        html! {
            <section class="section">
                <div class="login_form">
                    <input placeholder=self.locale.username() />
                    <input type="password" placeholder=self.locale.password() />

                    <button class="login_button">{self.locale.login()}</button>
                </div>
            </section>
        }
    }
}