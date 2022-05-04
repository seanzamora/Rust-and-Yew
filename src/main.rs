mod views;
mod services;
mod components;

use crate::views::{
    dashboard::DashboardView
};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route { 
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound, 
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <DashboardView/>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {

    html! {
        <BrowserRouter>

<div class="flex"></div>        <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
} 