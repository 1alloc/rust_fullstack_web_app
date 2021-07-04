use yew::prelude::*;

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
}

pub struct Header {
    props: Props,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
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
                <p class="sub_header">{self.props.title.clone()}</p>
            </header>
        }
    }
}