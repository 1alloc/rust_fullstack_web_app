use yew_router::prelude::*;
use yew::{Html, html};
use crate::Model;
use super::super::pages;
use pages::{
    index::Blog, about::About, counter::Counter, admin::login::Login
};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/admin/login"]
    Login,
    #[to = "/counter"]
    Counter,
    #[to = "/about"]
    About,
    #[to = "/"]
    Blog,
}

impl AppRoute {
    pub fn route_switch(app: &Model) -> Html {
        match AppRoute::switch(app.route.clone()) {
            Some(AppRoute::Blog) => html! {<Blog/>},
            Some(AppRoute::About) => html! {<About/>},
            Some(AppRoute::Counter) => html! {<Counter/>},
            Some(AppRoute::Login) => html! {<Login/>},
            None => html! {}
        }
    }
}