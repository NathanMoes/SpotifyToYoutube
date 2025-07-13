use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <div class="container">
                <div class="header-content">
                    <h1><Link<Route> to={Route::Home}>{"Spotify to YouTube"}</Link<Route>></h1>
                    <nav class="nav">
                        <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                        <Link<Route> to={Route::AddTracks} classes="nav-link">{"Add Tracks"}</Link<Route>>
                        <Link<Route> to={Route::DisplayTracks} classes="nav-link">{"View Tracks"}</Link<Route>>
                    </nav>
                </div>
            </div>
        </header>
    }
}
