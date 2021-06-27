use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{
    index::Index, blog::Blog
};

mod components;
use components::{
    header::Header, footer::Footer
};

enum Msg {
    RouteChanged(Route<()>),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/blog"]
    Blog,
    #[to = "/"]
    Index,
}

struct Model {
    link: ComponentLink<Self>,
    route_service: RouteService<()>,
    route: Route<()>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service : RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let router_agent = RouteAgent::bridge(link.callback(Msg::RouteChanged));

        Self {
            link,
            route_service,
            route,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => {
                self.route_service.set_route(&route.route, ());
                self.route = route
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            { self.view_nav() }

            <Header/>

            { self.route_switch() }

            <Footer/>
            </>
        }
    }
}

impl Model {
    fn route_switch(&self) -> Html {
        match AppRoute::switch(self.route.clone()){
            Some(AppRoute::Index) => html! {<Index/>},
            Some(AppRoute::Blog) => html! {<Blog/>},
            None => html! {}
        }
    }

    fn view_nav(&self) -> Html {
        html! {
            <nav>
                <ul class="nav">
                    <li>
                        <RouterAnchor<AppRoute> route=AppRoute::Blog><a>{"Blog"}</a></RouterAnchor<AppRoute>>
                    </li>

                    <li>
                        <RouterAnchor<AppRoute> route=AppRoute::Index><a>{"About"}</a></RouterAnchor<AppRoute>>
                    </li>

                    <li><a href="#">{"Contacts"}</a></li>
                </ul>
            </nav>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}