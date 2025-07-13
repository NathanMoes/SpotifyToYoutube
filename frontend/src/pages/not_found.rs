use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="not-found">
            <div class="container">
                <div class="not-found-content">
                    <h1>{"404"}</h1>
                    <h2>{"Page Not Found"}</h2>
                    <p>{"The requested page could not be found."}</p>
                    <Link<Route> to={Route::Home} classes="btn btn-primary">
                        {"Go Home"}
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
