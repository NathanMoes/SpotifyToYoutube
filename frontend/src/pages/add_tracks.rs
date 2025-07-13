use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::api::ApiService;

#[function_component(AddTracks)]
pub fn add_tracks() -> Html {
    let track_name = use_state(|| String::new());
    let artist_name = use_state(|| String::new());
    let playlist_url = use_state(|| String::new());
    let loading = use_state(|| false);
    let message = use_state(|| None::<String>);

    let on_add_track = {
        let track_name = track_name.clone();
        let artist_name = artist_name.clone();
        let loading = loading.clone();
        let message = message.clone();
        Callback::from(move |_| {
            let track_name_value = (*track_name).clone();
            let artist_name_value = (*artist_name).clone();
            
            if track_name_value.trim().is_empty() || artist_name_value.trim().is_empty() {
                message.set(Some("Please enter both track name and artist name".to_string()));
                return;
            }
            
            loading.set(true);
            message.set(None);
            
            let track_name_clone = track_name.clone();
            let artist_name_clone = artist_name.clone();
            let loading_clone = loading.clone();
            let message_clone = message.clone();
            let api_service = ApiService::new();
            
            spawn_local(async move {
                match api_service.add_track(track_name_value, artist_name_value).await {
                    Ok(response) => {
                        message_clone.set(Some(response.message));
                        if response.status == "success" {
                            track_name_clone.set(String::new());
                            artist_name_clone.set(String::new());
                        }
                    }
                    Err(error) => {
                        message_clone.set(Some(format!("Error: {}", error)));
                    }
                }
                loading_clone.set(false);
            });
        })
    };

    let on_import_playlist = {
        let playlist_url = playlist_url.clone();
        let loading = loading.clone();
        let message = message.clone();
        Callback::from(move |_| {
            let url_value = (*playlist_url).clone();
            
            if url_value.trim().is_empty() {
                message.set(Some("Please enter a Spotify playlist URL".to_string()));
                return;
            }
            
            loading.set(true);
            message.set(None);
            
            let playlist_url_clone = playlist_url.clone();
            let loading_clone = loading.clone();
            let message_clone = message.clone();
            let api_service = ApiService::new();
            
            spawn_local(async move {
                match api_service.import_playlist(url_value).await {
                    Ok(response) => {
                        let msg = if let Some(count) = response.tracks_count {
                            format!("{} ({} tracks imported)", response.message, count)
                        } else {
                            response.message
                        };
                        message_clone.set(Some(msg));
                        if response.status == "success" {
                            playlist_url_clone.set(String::new());
                        }
                    }
                    Err(error) => {
                        message_clone.set(Some(format!("Error: {}", error)));
                    }
                }
                loading_clone.set(false);
            });
        })
    };

    html! {
        <div class="add-tracks">
            <div class="container">
                <h2>{"Add Tracks"}</h2>
                
                // Show loading state and messages
                {if *loading {
                    html! { <div class="loading">{"Processing..."}</div> }
                } else {
                    html! {}
                }}
                
                {if let Some(msg) = (*message).as_ref() {
                    html! { 
                        <div class="message" style="padding: 10px; margin: 10px 0; border-radius: 4px; background-color: #f0f9ff; border: 1px solid #0ea5e9;">
                            {msg}
                        </div> 
                    }
                } else {
                    html! {}
                }}
                
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
                            disabled={*loading}
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
                            disabled={*loading}
                        />
                    </div>
                    
                    <button 
                        type="button" 
                        onclick={on_add_track} 
                        class="btn btn-primary"
                        disabled={*loading}
                    >
                        {if *loading { "Adding..." } else { "Add Track" }}
                    </button>
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
                            disabled={*loading}
                        />
                    </div>
                    
                    <button 
                        type="button" 
                        onclick={on_import_playlist} 
                        class="btn btn-secondary"
                        disabled={*loading}
                    >
                        {if *loading { "Importing..." } else { "Import Playlist" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
