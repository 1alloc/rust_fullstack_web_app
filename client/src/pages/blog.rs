use yew::prelude::*;

use super::super::components;
use components::{
    blog_article::BlogArticle
};

pub struct Blog;
impl Component for Blog {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="section">
                <BlogArticle/>
            </section>
        }
    }
}