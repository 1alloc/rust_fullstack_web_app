use yew::prelude::*;

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub title: String,
    pub email: String,
}

pub struct Footer {
    props: Props,
}

impl Component for Footer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut mailto: String = "mailto:".to_owned();
        mailto.push_str(&*self.props.email.clone());
        html! {
            <footer>
                <p>{self.props.title.clone()}</p>
                <p><a href=mailto>{self.props.email.clone()}</a></p>
            </footer>
        }
    }
}