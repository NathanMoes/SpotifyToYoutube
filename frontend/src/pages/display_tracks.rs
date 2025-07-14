use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::api::{ApiService, DatabaseTrack};

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

    // Load tracks from database on component mount
    {
        let tracks = tracks.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                let api_service = ApiService::new();
                
                match api_service.get_tracks_for_conversion(Some(20)).await {
                    Ok(response) => {
                        let mut converted_tracks = response.tracks.iter()
                            .map(database_track_to_track)
                            .collect::<Vec<Track>>();

                        // If no tracks from database, use fallback static tracks
                        if converted_tracks.is_empty() {
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
                        loading.set(false);
                    },
                    Err(err) => {
                        // On error, use fallback static tracks
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
                        error.set(Some(format!("Failed to load tracks from database: {}. Using fallback data.", err)));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

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
                        <p>{"Total Tracks"}</p>
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
            </div>
        </div>
    }
}
