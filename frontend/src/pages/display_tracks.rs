use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Track {
    pub id: u32,
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

#[function_component(DisplayTracks)]
pub fn display_tracks() -> Html {
    // Mock data for demonstration
    let tracks = use_state(|| vec![
        Track {
            id: 1,
            name: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            spotify_url: Some("https://open.spotify.com/track/4u7EnebtmKWzUH433cf5Qv".to_string()),
            youtube_url: Some("https://www.youtube.com/watch?v=fJ9rUzIMcZQ".to_string()),
            status: TrackStatus::Converted,
        },
        Track {
            id: 2,
            name: "Imagine".to_string(),
            artist: "John Lennon".to_string(),
            spotify_url: Some("https://open.spotify.com/track/7pKfPomDEeI4TPT6EOYjn9".to_string()),
            youtube_url: None,
            status: TrackStatus::Found,
        },
        Track {
            id: 3,
            name: "Hotel California".to_string(),
            artist: "Eagles".to_string(),
            spotify_url: Some("https://open.spotify.com/track/40riOy7x9W7GXjyGp4pjAv".to_string()),
            youtube_url: None,
            status: TrackStatus::Pending,
        },
    ]);

    let convert_track = {
        let tracks = tracks.clone();
        Callback::from(move |track_id: u32| {
            let mut updated_tracks = (*tracks).clone();
            if let Some(track) = updated_tracks.iter_mut().find(|t| t.id == track_id) {
                track.status = TrackStatus::Converted;
                track.youtube_url = Some(format!("https://www.youtube.com/watch?v=mock_{}", track_id));
            }
            tracks.set(updated_tracks);
        })
    };

    html! {
        <div class="display-tracks">
            <div class="container">
                <h2>{"Your Tracks"}</h2>
                
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
                        let track_id = track.id;
                        let on_convert = {
                            let convert_track = convert_track.clone();
                            Callback::from(move |_| convert_track.emit(track_id))
                        };

                        html! {
                            <div class="track-card" key={track.id}>
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
