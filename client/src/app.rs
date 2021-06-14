use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;

const KEY: &str = "yew.app.self";

pub struct App {}
pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div>
                  <header>
        <nav>
          <ul class="nav">
          <li><a href="#">{"Blog"}</a></li>
          <li><a href="#">{"About"}</a></li>
          <li><a href="#">{"Contacts"}</a></li>
          </ul>
          </nav>
        <h1>{"1alloc • software engineer"}</h1>
        <p class="sub_header">{"Rustacean"}</p>
        </header>

                        { self.view_blog() }

                    <footer>
          <p>{ "1alloc" }</p>
            <p><a href="mailto:1alloc@pm.me">{ "1alloc@pm.me" }</a></p>
        </footer>

            </div>
        }
    }
}

impl App {
    fn view_blog(&self) -> Html {
        html! {
            <section class="blog">
                <h2>{"Why Rust?"}</h2>
                <h5>{"16.05.2021 | 16:56"}</h5>
                <p>{"Some text for blog article......"}</p>
            </section>
        }
    }
}

