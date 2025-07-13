use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="container">
                <p>{"© 2025 Spotify to YouTube Converter"}</p>
            </div>
        </footer>
    }
}
