use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::api::{ApiService, DatabaseTrack};
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub spotify_url: Option<String>,
    pub youtube_url: Option<String>,
    pub status: TrackStatus,
}

#[derive(Clone, PartialEq)]
pub enum TrackStatus {
    Pending,
    Found,
    NotFound,
    Converted,
}

impl std::fmt::Display for TrackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackStatus::Pending => write!(f, "Pending"),
            TrackStatus::Found => write!(f, "Found"),
            TrackStatus::NotFound => write!(f, "Not Found"),
            TrackStatus::Converted => write!(f, "Converted"),
        }
    }
}

// Helper function to convert DatabaseTrack to Track
fn database_track_to_track(db_track: &DatabaseTrack) -> Track {
    let status = if db_track.youtube_url.is_some() {
        TrackStatus::Converted
    } else {
        TrackStatus::Found
    };

    let spotify_url = if !db_track.spotify_uri.is_empty() {
        Some(format!("https://open.spotify.com/track/{}", 
            db_track.spotify_uri.strip_prefix("spotify:track:").unwrap_or(&db_track.id)))
    } else {
        None
    };

    Track {
        id: db_track.id.clone(),
        name: db_track.name.clone(),
        artist: "Various Artists".to_string(), // We'll need to fetch artist info separately
        spotify_url,
        youtube_url: db_track.youtube_url.clone(),
        status,
    }
}

// Helper function to extract artist name from external URLs (fallback)
fn extract_artist_from_external_urls(_external_urls: &str) -> String {
    // This is a simplified extraction - in real implementation you might parse JSON
    "Various Artists".to_string()
}

#[function_component(DisplayTracks)]
pub fn display_tracks() -> Html {
    let tracks = use_state(|| Vec::<Track>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let current_page = use_state(|| 1usize);
    let total_pages = use_state(|| 1usize);
    let search_query = use_state(|| String::new());
    let search_mode = use_state(|| false);

    const TRACKS_PER_PAGE: i64 = 10;

    // Load tracks function
    let load_tracks = {
        let tracks = tracks.clone();
        let loading = loading.clone();
        let error = error.clone();
        let current_page = current_page.clone();
        let total_pages = total_pages.clone();
        let search_query = search_query.clone();
        let search_mode = search_mode.clone();

        Callback::from(move |page: usize| {
            let tracks = tracks.clone();
            let loading = loading.clone();
            let error = error.clone();
            let current_page = current_page.clone();
            let total_pages = total_pages.clone();
            let search_query = search_query.clone();
            let search_mode = search_mode.clone();

            spawn_local(async move {
                loading.set(true);
                let api_service = ApiService::new();
                
                let result = if *search_mode && !search_query.is_empty() {
                    api_service.search_tracks(&search_query, Some(TRACKS_PER_PAGE)).await
                        .map(|response| (response.tracks, response.count))
                } else {
                    let offset = ((page - 1) * TRACKS_PER_PAGE as usize) as i64;
                    api_service.get_tracks_for_conversion(Some(TRACKS_PER_PAGE), Some(offset)).await
                        .map(|response| (response.tracks, response.count))
                };

                match result {
                    Ok((db_tracks, total_count)) => {
                        let mut converted_tracks = db_tracks.iter()
                            .map(database_track_to_track)
                            .collect::<Vec<Track>>();

                        // If no tracks from database, use fallback static tracks only on page 1
                        if converted_tracks.is_empty() && page == 1 && !*search_mode {
                            converted_tracks = vec![
                                Track {
                                    id: "1".to_string(),
                                    name: "Bohemian Rhapsody".to_string(),
                                    artist: "Queen".to_string(),
                                    spotify_url: Some("https://open.spotify.com/track/4u7EnebtmKWzUH433cf5Qv".to_string()),
                                    youtube_url: Some("https://www.youtube.com/watch?v=fJ9rUzIMcZQ".to_string()),
                                    status: TrackStatus::Converted,
                                },
                                Track {
                                    id: "2".to_string(),
                                    name: "Imagine".to_string(),
                                    artist: "John Lennon".to_string(),
                                    spotify_url: Some("https://open.spotify.com/track/7pKfPomDEeI4TPT6EOYjn9".to_string()),
                                    youtube_url: None,
                                    status: TrackStatus::Found,
                                },
                                Track {
                                    id: "3".to_string(),
                                    name: "Hotel California".to_string(),
                                    artist: "Eagles".to_string(),
                                    spotify_url: Some("https://open.spotify.com/track/40riOy7x9W7GXjyGp4pjAv".to_string()),
                                    youtube_url: None,
                                    status: TrackStatus::Pending,
                                },
                            ];
                        }

                        tracks.set(converted_tracks);
                        current_page.set(page);
                        
                        if *search_mode {
                            total_pages.set(1); // Search results are limited, show only one page
                        } else {
                            let calculated_total_pages = (total_count as f64 / TRACKS_PER_PAGE as f64).ceil() as usize;
                            total_pages.set(calculated_total_pages.max(1));
                        }
                        
                        error.set(None);
                        loading.set(false);
                    },
                    Err(err) => {
                        // On error, use fallback static tracks only for page 1
                        if page == 1 && !*search_mode {
                            let fallback_tracks = vec![
                                Track {
                                    id: "1".to_string(),
                                    name: "Bohemian Rhapsody".to_string(),
                                    artist: "Queen".to_string(),
                                    spotify_url: Some("https://open.spotify.com/track/4u7EnebtmKWzUH433cf5Qv".to_string()),
                                    youtube_url: Some("https://www.youtube.com/watch?v=fJ9rUzIMcZQ".to_string()),
                                    status: TrackStatus::Converted,
                                },
                                Track {
                                    id: "2".to_string(),
                                    name: "Imagine".to_string(),
                                    artist: "John Lennon".to_string(),
                                    spotify_url: Some("https://open.spotify.com/track/7pKfPomDEeI4TPT6EOYjn9".to_string()),
                                    youtube_url: None,
                                    status: TrackStatus::Found,
                                },
                                Track {
                                    id: "3".to_string(),
                                    name: "Hotel California".to_string(),
                                    artist: "Eagles".to_string(),
                                    spotify_url: Some("https://open.spotify.com/track/40riOy7x9W7GXjyGp4pjAv".to_string()),
                                    youtube_url: None,
                                    status: TrackStatus::Pending,
                                },
                            ];
                            
                            tracks.set(fallback_tracks);
                            current_page.set(1);
                            total_pages.set(1);
                            error.set(Some(format!("Failed to load tracks from database: {}. Using fallback data.", err)));
                        } else {
                            tracks.set(vec![]);
                            error.set(Some(format!("Failed to load tracks: {}", err)));
                        }
                        loading.set(false);
                    }
                }
            });
        })
    };

    // Load tracks on component mount
    {
        let load_tracks = load_tracks.clone();
        use_effect_with((), move |_| {
            load_tracks.emit(1);
            || ()
        });
    }

    // Search functionality
    let on_search = {
        let search_query = search_query.clone();
        let search_mode = search_mode.clone();
        let load_tracks = load_tracks.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if search_query.is_empty() {
                search_mode.set(false);
                load_tracks.emit(1);
            } else {
                search_mode.set(true);
                load_tracks.emit(1);
            }
        })
    };

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let clear_search = {
        let search_query = search_query.clone();
        let search_mode = search_mode.clone();
        let load_tracks = load_tracks.clone();

        Callback::from(move |_| {
            search_query.set(String::new());
            search_mode.set(false);
            load_tracks.emit(1);
        })
    };

    // Pagination controls
    let go_to_page = {
        let load_tracks = load_tracks.clone();
        Callback::from(move |page: usize| {
            load_tracks.emit(page);
        })
    };

    let convert_track = {
        let tracks = tracks.clone();
        Callback::from(move |track_id: String| {
            let mut updated_tracks = (*tracks).clone();
            if let Some(track) = updated_tracks.iter_mut().find(|t| t.id == track_id) {
                track.status = TrackStatus::Converted;
                track.youtube_url = Some(format!("https://www.youtube.com/watch?v=mock_{}", track_id));
            }
            tracks.set(updated_tracks);
        })
    };

    if *loading {
        return html! {
            <div class="display-tracks">
                <div class="container">
                    <h2>{"Loading tracks..."}</h2>
                </div>
            </div>
        };
    }

    html! {
        <div class="display-tracks">
            <div class="container">
                <h2>{"Your Tracks"}</h2>
                
                // Search form
                <div class="search-section">
                    <form onsubmit={on_search} class="search-form">
                        <div class="search-input-group">
                            <input
                                type="text"
                                placeholder="Search by artist name..."
                                value={(*search_query).clone()}
                                oninput={on_search_input}
                                class="search-input"
                            />
                            <button type="submit" class="btn btn-primary search-btn">
                                {"Search"}
                            </button>
                            {if *search_mode {
                                html! {
                                    <button type="button" onclick={clear_search} class="btn btn-secondary">
                                        {"Clear"}
                                    </button>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    </form>
                    
                    {if *search_mode {
                        html! {
                            <p class="search-info">
                                {format!("Showing search results for: \"{}\"", *search_query)}
                            </p>
                        }
                    } else {
                        html! {}
                    }}
                </div>
                
                {if let Some(err) = (*error).as_ref() {
                    html! {
                        <div class="error-message" style="background: #fee; color: #c00; padding: 10px; margin-bottom: 20px; border-radius: 4px;">
                            {err}
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                <div class="tracks-summary">
                    <div class="summary-card">
                        <h3>{tracks.len()}</h3>
                        <p>{if *search_mode { "Search Results" } else { "Tracks on Page" }}</p>
                    </div>
                    <div class="summary-card">
                        <h3>{tracks.iter().filter(|t| t.status == TrackStatus::Converted).count()}</h3>
                        <p>{"Converted"}</p>
                    </div>
                    <div class="summary-card">
                        <h3>{tracks.iter().filter(|t| t.status == TrackStatus::Pending).count()}</h3>
                        <p>{"Pending"}</p>
                    </div>
                </div>

                <div class="tracks-list">
                    {for tracks.iter().map(|track| {
                        let track_id = track.id.clone();
                        let on_convert = {
                            let convert_track = convert_track.clone();
                            Callback::from(move |_| convert_track.emit(track_id.clone()))
                        };

                        html! {
                            <div class="track-card" key={track.id.clone()}>
                                <div class="track-info">
                                    <h4>{&track.name}</h4>
                                    <p class="artist">{&track.artist}</p>
                                    <span class={format!("status status-{}", 
                                        match track.status {
                                            TrackStatus::Pending => "pending",
                                            TrackStatus::Found => "found", 
                                            TrackStatus::NotFound => "not-found",
                                            TrackStatus::Converted => "converted",
                                        }
                                    )}>
                                        {track.status.to_string()}
                                    </span>
                                </div>
                                
                                <div class="track-actions">
                                    {if let Some(spotify_url) = &track.spotify_url {
                                        html! {
                                            <a href={spotify_url.clone()} target="_blank" class="btn btn-spotify">
                                                {"Spotify"}
                                            </a>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                    
                                    {if let Some(youtube_url) = &track.youtube_url {
                                        html! {
                                            <a href={youtube_url.clone()} target="_blank" class="btn btn-youtube">
                                                {"YouTube"}
                                            </a>
                                        }
                                    } else if track.status == TrackStatus::Found {
                                        html! {
                                            <button onclick={on_convert} class="btn btn-primary">
                                                {"Convert"}
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            </div>
                        }
                    })}
                </div>

                // Pagination controls
                {if !*search_mode && *total_pages > 1 {
                    html! {
                        <div class="pagination">
                            <div class="pagination-info">
                                {format!("Page {} of {}", *current_page, *total_pages)}
                            </div>
                            <div class="pagination-controls">
                                {if *current_page > 1 {
                                    html! {
                                        <button onclick={{
                                            let go_to_page = go_to_page.clone();
                                            let page = *current_page - 1;
                                            Callback::from(move |_| go_to_page.emit(page))
                                        }} class="btn btn-secondary">
                                            {"Previous"}
                                        </button>
                                    }
                                } else {
                                    html! {}
                                }}
                                
                                // Page numbers (show up to 5 pages around current)
                                {for (1..=*total_pages).filter(|&page| {
                                    page == 1 || page == *total_pages || 
                                    (page >= current_page.saturating_sub(2) && page <= *current_page + 2)
                                }).map(|page| {
                                    let is_current = page == *current_page;
                                    let go_to_page = go_to_page.clone();
                                    
                                    html! {
                                        <button 
                                            onclick={Callback::from(move |_| go_to_page.emit(page))}
                                            class={if is_current { "btn btn-primary page-btn current" } else { "btn btn-secondary page-btn" }}
                                            disabled={is_current}
                                        >
                                            {page}
                                        </button>
                                    }
                                })}

                                {if *current_page < *total_pages {
                                    html! {
                                        <button onclick={{
                                            let go_to_page = go_to_page.clone();
                                            let page = *current_page + 1;
                                            Callback::from(move |_| go_to_page.emit(page))
                                        }} class="btn btn-secondary">
                                            {"Next"}
                                        </button>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}
