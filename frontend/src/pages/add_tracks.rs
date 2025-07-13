use yew::prelude::*;

#[function_component(AddTracks)]
pub fn add_tracks() -> Html {
    let track_name = use_state(|| String::new());
    let artist_name = use_state(|| String::new());
    let playlist_url = use_state(|| String::new());

    let on_add_track = {
        let track_name = track_name.clone();
        let artist_name = artist_name.clone();
        Callback::from(move |_| {
            // Here you would handle the form submission
            log::info!("Adding track: {} by {}", *track_name, *artist_name);
            
            // Reset form
            track_name.set(String::new());
            artist_name.set(String::new());
        })
    };

    let on_import_playlist = {
        let playlist_url = playlist_url.clone();
        Callback::from(move |_| {
            log::info!("Importing playlist: {}", *playlist_url);
            
            // Reset form
            playlist_url.set(String::new());
        })
    };

    html! {
        <div class="add-tracks">
            <div class="container">
                <h2>{"Add Tracks"}</h2>
                
                <div class="form-section">
                    <h3>{"Add Individual Track"}</h3>
                    <div class="form-group">
                        <label for="track-name">{"Track Name:"}</label>
                        <input
                            type="text"
                            id="track-name"
                            value={(*track_name).clone()}
                            onchange={
                                let track_name = track_name.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                        track_name.set(input.value());
                                    }
                                })
                            }
                            placeholder="Enter track name"
                            class="form-input"
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="artist-name">{"Artist Name:"}</label>
                        <input
                            type="text"
                            id="artist-name"
                            value={(*artist_name).clone()}
                            onchange={
                                let artist_name = artist_name.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                        artist_name.set(input.value());
                                    }
                                })
                            }
                            placeholder="Enter artist name"
                            class="form-input"
                        />
                    </div>
                    
                    <button type="button" onclick={on_add_track} class="btn btn-primary">{"Add Track"}</button>
                </div>

                <div class="form-section">
                    <h3>{"Import from Spotify Playlist"}</h3>
                    <div class="form-group">
                        <label for="playlist-url">{"Spotify Playlist URL:"}</label>
                        <input
                            type="url"
                            id="playlist-url"
                            value={(*playlist_url).clone()}
                            onchange={
                                let playlist_url = playlist_url.clone();
                                Callback::from(move |e: Event| {
                                    if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                        playlist_url.set(input.value());
                                    }
                                })
                            }
                            placeholder="https://open.spotify.com/playlist/..."
                            class="form-input"
                        />
                    </div>
                    
                    <button type="button" onclick={on_import_playlist} class="btn btn-secondary">{"Import Playlist"}</button>
                </div>
            </div>
        </div>
    }
}
