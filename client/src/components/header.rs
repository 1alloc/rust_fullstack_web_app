use yew::prelude::*;

pub enum Msg {}

pub struct Header {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <h1>{"1alloc • software engineer"}</h1>
                <p class="sub_header">{"Rustacean"}</p>
            </header>
        }
    }
}