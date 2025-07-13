use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home">
            <main class="main-content">
                <div class="container">
                    <div class="hero-section">
                        <h2>{"Welcome to Spotify to YouTube Converter"}</h2>
                        <p>{"Seamlessly transfer your favorite Spotify playlists to YouTube with just a few clicks."}</p>
                        
                        <div class="button-group">
                            <button class="btn btn-primary">{"Connect Spotify"}</button>
                            <button class="btn btn-secondary">{"Connect YouTube"}</button>
                        </div>
                    </div>
                    
                    <div class="features">
                        <h3>{"Features"}</h3>
                        <div class="feature-grid">
                            <div class="feature-card">
                                <h4>{"Easy Connection"}</h4>
                                <p>{"Connect your Spotify and YouTube accounts securely"}</p>
                            </div>
                            <div class="feature-card">
                                <h4>{"Playlist Transfer"}</h4>
                                <p>{"Transfer entire playlists with song matching"}</p>
                            </div>
                            <div class="feature-card">
                                <h4>{"Fast & Reliable"}</h4>
                                <p>{"Quick processing with high accuracy song matching"}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
