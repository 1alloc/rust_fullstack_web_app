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
            <footer class="site-footer">
               <ul>
                        <li><a href=mailto target="_blank"><img src="images/email_icon.png" height="20" width="22"/></a></li>
                        <li><a href="https://twitter.com/1alloc" target="_blank"><img src="images/twitter_icon.png" height="20" width="22"/></a></li>
                        <li><a href="https://github.com/1alloc" target="_blank"><img src="images/github_icon.png" height="20" width="20"/></a></li>
                        <li><a href="https://www.codewars.com/users/1alloc" target="_blank"><img src="images/codewars_icon.png" height="20" width="20"/></a></li>
                    </ul>
            </footer>
        }
    }
}