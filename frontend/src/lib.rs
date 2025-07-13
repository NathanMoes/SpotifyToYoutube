use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod services;

use components::footer::Footer;
use components::header::Header;
use pages::home::Home;
use pages::add_tracks::AddTracks;
use pages::display_tracks::DisplayTracks;
use pages::not_found::NotFound;

/// The routes for the application
#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/add-tracks")]
    AddTracks,
    #[at("/display-tracks")]
    DisplayTracks,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Route switching function
fn switch_route(route: &Route) -> Html {
    log::info!("Matched route: {:?}", route);
    match route {
        Route::Home => html! { <Home /> },
        Route::AddTracks => html! { <AddTracks /> },
        Route::DisplayTracks => html! { <DisplayTracks /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

/// The main application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app">
                <Header />
                <Switch<Route> render={move |route: Route| switch_route(&route)} />
                <Footer />
                
                <style>
                    {include_str!("styles.css")}
                </style>
            </div>
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("error initializing log");
    
    yew::Renderer::<App>::new().render();
}
