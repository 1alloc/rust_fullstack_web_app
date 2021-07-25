use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use crate::locales::config;
use crate::agents::locale::LocaleStored;

pub enum Msg {
    LocaleStore(ReadOnly<LocaleStored>),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub title: i32,
}

pub struct Header {
    props: Props,
    locale: config::Locale,
    locale_agent: Box<dyn Bridge<StoreWrapper<LocaleStored>>>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let locale = config::Locale::get(&wasm_cookies::get_raw("language").unwrap());
        let locale_agent = LocaleStored::bridge(link.callback(Msg::LocaleStore));
        Self {
            props,
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
            <header>
                <h1>{"1alloc • software engineer"}</h1>
                <p class="sub_header">{self.locale.main()}</p>
                <p class="sub_header">{self.props.title}</p>
            </header>
        }
    }
}