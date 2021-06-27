use yew::prelude::*;

pub enum Msg {}

pub struct BlogArticle {}

impl Component for BlogArticle {
    type Message = ();
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
            <>
                <h2>{"Why Rust?"}</h2>
                <h5>{"16.05.2021 | 16:56"}</h5>
                <p>{"Some text for blog article......"}</p>
            </>
        }
    }
}