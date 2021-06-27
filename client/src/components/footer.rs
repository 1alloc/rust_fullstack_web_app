use yew::prelude::*;

pub enum Msg {}

pub struct Footer {}

impl Component for Footer {
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
            <footer>
                <p>{ "1alloc" }</p>
                <p><a href="mailto:1alloc@pm.me">{ "1alloc@pm.me" }</a></p>
            </footer>
        }
    }
}