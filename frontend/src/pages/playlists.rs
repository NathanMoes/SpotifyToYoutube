use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::api::{ApiService, DatabasePlaylist, DatabaseTrack};

#[derive(Clone, PartialEq)]
pub struct PlaylistWithStats {
    pub playlist: DatabasePlaylist,
    pub converted_count: usize,
    pub total_count: usize,
}

#[function_component(Playlists)]
pub fn playlists() -> Html {
    let playlists = use_state(|| Vec::<PlaylistWithStats>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let selected_playlist = use_state(|| Option::<String>::None);
    let playlist_tracks = use_state(|| Vec::<DatabaseTrack>::new());
    let tracks_loading = use_state(|| false);

    // Load playlists on component mount
    {
        let playlists = playlists.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                let api_service = ApiService::new();
                
                match api_service.get_playlists(Some(50), Some(0)).await {
                    Ok(response) => {
                        // For now, create mock stats since we don't have an endpoint for playlist stats
                        let playlists_with_stats = response.playlists.into_iter().map(|playlist| {
                            PlaylistWithStats {
                                converted_count: (playlist.total_tracks as f32 * 0.7) as usize, // Mock 70% conversion rate
                                total_count: playlist.total_tracks as usize,
                                playlist,
                            }
                        }).collect();

                        playlists.set(playlists_with_stats);
                        error.set(None);
                        loading.set(false);
                    },
                    Err(err) => {
                        // Create fallback playlists
                        let fallback_playlists = vec![
                            PlaylistWithStats {
                                playlist: DatabasePlaylist {
                                    id: "1".to_string(),
                                    name: "My Favorites".to_string(),
                                    description: Some("A collection of my favorite tracks".to_string()),
                                    spotify_uri: "spotify:playlist:example1".to_string(),
                                    owner_id: "user123".to_string(),
                                    owner_display_name: "John Doe".to_string(),
                                    public: true,
                                    collaborative: false,
                                    snapshot_id: "snapshot1".to_string(),
                                    total_tracks: 25,
                                },
                                converted_count: 18,
                                total_count: 25,
                            },
                            PlaylistWithStats {
                                playlist: DatabasePlaylist {
                                    id: "2".to_string(),
                                    name: "Road Trip Mix".to_string(),
                                    description: Some("Perfect songs for a long drive".to_string()),
                                    spotify_uri: "spotify:playlist:example2".to_string(),
                                    owner_id: "user123".to_string(),
                                    owner_display_name: "John Doe".to_string(),
                                    public: false,
                                    collaborative: true,
                                    snapshot_id: "snapshot2".to_string(),
                                    total_tracks: 42,
                                },
                                converted_count: 30,
                                total_count: 42,
                            },
                            PlaylistWithStats {
                                playlist: DatabasePlaylist {
                                    id: "3".to_string(),
                                    name: "Workout Beats".to_string(),
                                    description: Some("High energy tracks for exercise".to_string()),
                                    spotify_uri: "spotify:playlist:example3".to_string(),
                                    owner_id: "user123".to_string(),
                                    owner_display_name: "John Doe".to_string(),
                                    public: true,
                                    collaborative: false,
                                    snapshot_id: "snapshot3".to_string(),
                                    total_tracks: 33,
                                },
                                converted_count: 15,
                                total_count: 33,
                            },
                        ];
                        
                        playlists.set(fallback_playlists);
                        error.set(Some(format!("Failed to load playlists from database: {}. Using fallback data.", err)));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Load tracks for selected playlist
    let load_playlist_tracks = {
        let playlist_tracks = playlist_tracks.clone();
        let tracks_loading = tracks_loading.clone();
        let error = error.clone();

        Callback::from(move |playlist_id: String| {
            let playlist_tracks = playlist_tracks.clone();
            let tracks_loading = tracks_loading.clone();
            let error = error.clone();

            spawn_local(async move {
                tracks_loading.set(true);
                let api_service = ApiService::new();
                
                match api_service.get_playlist_tracks(&playlist_id, Some(50), Some(0)).await {
                    Ok(response) => {
                        playlist_tracks.set(response.tracks);
                        tracks_loading.set(false);
                    },
                    Err(err) => {
                        // Fallback tracks for demo
                        let fallback_tracks = vec![
                            DatabaseTrack {
                                id: "track1".to_string(),
                                name: "Bohemian Rhapsody".to_string(),
                                spotify_uri: "spotify:track:4u7EnebtmKWzUH433cf5Qv".to_string(),
                                duration_ms: 355000,
                                explicit: false,
                                popularity: 95,
                                preview_url: None,
                                external_urls: "{}".to_string(),
                                youtube_url: Some("https://www.youtube.com/watch?v=fJ9rUzIMcZQ".to_string()),
                                isrc: Some("GBUM71505078".to_string()),
                            },
                            DatabaseTrack {
                                id: "track2".to_string(),
                                name: "Imagine".to_string(),
                                spotify_uri: "spotify:track:7pKfPomDEeI4TPT6EOYjn9".to_string(),
                                duration_ms: 183000,
                                explicit: false,
                                popularity: 88,
                                preview_url: None,
                                external_urls: "{}".to_string(),
                                youtube_url: None,
                                isrc: Some("USQX91100207".to_string()),
                            },
                        ];
                        
                        playlist_tracks.set(fallback_tracks);
                        error.set(Some(format!("Failed to load playlist tracks: {}. Using fallback data.", err)));
                        tracks_loading.set(false);
                    }
                }
            });
        })
    };

    let on_playlist_click = {
        let selected_playlist = selected_playlist.clone();
        let load_playlist_tracks = load_playlist_tracks.clone();

        Callback::from(move |playlist_id: String| {
            selected_playlist.set(Some(playlist_id.clone()));
            load_playlist_tracks.emit(playlist_id);
        })
    };

    let close_playlist_details = {
        let selected_playlist = selected_playlist.clone();
        let playlist_tracks = playlist_tracks.clone();

        Callback::from(move |_| {
            selected_playlist.set(None);
            playlist_tracks.set(vec![]);
        })
    };

    if *loading {
        return html! {
            <div class="playlists">
                <div class="container">
                    <h2>{"Loading playlists..."}</h2>
                </div>
            </div>
        };
    }

    html! {
        <div class="playlists">
            <div class="container">
                <h2>{"Your Playlists"}</h2>
                
                {if let Some(err) = (*error).as_ref() {
                    html! {
                        <div class="error-message" style="background: #fee; color: #c00; padding: 10px; margin-bottom: 20px; border-radius: 4px;">
                            {err}
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                <div class="playlists-summary">
                    <div class="summary-card">
                        <h3>{playlists.len()}</h3>
                        <p>{"Total Playlists"}</p>
                    </div>
                    <div class="summary-card">
                        <h3>{playlists.iter().map(|p| p.total_count).sum::<usize>()}</h3>
                        <p>{"Total Tracks"}</p>
                    </div>
                    <div class="summary-card">
                        <h3>{playlists.iter().map(|p| p.converted_count).sum::<usize>()}</h3>
                        <p>{"Converted Tracks"}</p>
                    </div>
                    <div class="summary-card">
                        <h3>{format!("{:.1}%", 
                            if playlists.iter().map(|p| p.total_count).sum::<usize>() > 0 {
                                (playlists.iter().map(|p| p.converted_count).sum::<usize>() as f32 / 
                                 playlists.iter().map(|p| p.total_count).sum::<usize>() as f32) * 100.0
                            } else { 0.0 }
                        )}</h3>
                        <p>{"Conversion Rate"}</p>
                    </div>
                </div>

                <div class="playlists-grid">
                    {for playlists.iter().map(|playlist_with_stats| {
                        let playlist = &playlist_with_stats.playlist;
                        let playlist_id = playlist.id.clone();
                        let on_click = {
                            let on_playlist_click = on_playlist_click.clone();
                            Callback::from(move |_| on_playlist_click.emit(playlist_id.clone()))
                        };

                        let conversion_percentage = if playlist_with_stats.total_count > 0 {
                            (playlist_with_stats.converted_count as f32 / playlist_with_stats.total_count as f32) * 100.0
                        } else { 0.0 };

                        html! {
                            <div class="playlist-card" key={playlist.id.clone()}>
                                <div class="playlist-header">
                                    <h3>{&playlist.name}</h3>
                                    <div class="playlist-meta">
                                        <span class="owner">{"by "}{&playlist.owner_display_name}</span>
                                        <div class="playlist-badges">
                                            {if playlist.public {
                                                html! { <span class="badge public">{"Public"}</span> }
                                            } else {
                                                html! { <span class="badge private">{"Private"}</span> }
                                            }}
                                            {if playlist.collaborative {
                                                html! { <span class="badge collaborative">{"Collaborative"}</span> }
                                            } else {
                                                html! {}
                                            }}
                                        </div>
                                    </div>
                                </div>
                                
                                {if let Some(description) = &playlist.description {
                                    html! {
                                        <p class="playlist-description">{description}</p>
                                    }
                                } else {
                                    html! {}
                                }}
                                
                                <div class="playlist-stats">
                                    <div class="stat">
                                        <span class="stat-number">{playlist_with_stats.total_count}</span>
                                        <span class="stat-label">{"tracks"}</span>
                                    </div>
                                    <div class="stat">
                                        <span class="stat-number">{playlist_with_stats.converted_count}</span>
                                        <span class="stat-label">{"converted"}</span>
                                    </div>
                                    <div class="conversion-bar">
                                        <div class="conversion-progress" 
                                             style={format!("width: {}%", conversion_percentage)}>
                                        </div>
                                    </div>
                                    <span class="conversion-percentage">{format!("{:.1}%", conversion_percentage)}</span>
                                </div>
                                
                                <div class="playlist-actions">
                                    <button onclick={on_click} class="btn btn-primary">
                                        {"View Tracks"}
                                    </button>
                                    <a href={format!("https://open.spotify.com/playlist/{}", 
                                        playlist.spotify_uri.strip_prefix("spotify:playlist:").unwrap_or(&playlist.id))}
                                       target="_blank" class="btn btn-spotify">
                                        {"Open in Spotify"}
                                    </a>
                                </div>
                            </div>
                        }
                    })}
                </div>
                
                // Playlist details modal
                {if let Some(playlist_id) = (*selected_playlist).as_ref() {
                    let selected_playlist_data = playlists.iter()
                        .find(|p| &p.playlist.id == playlist_id)
                        .map(|p| &p.playlist);
                        
                    html! {
                        <div class="modal-overlay" onclick={close_playlist_details.clone()}>
                            <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                <div class="modal-header">
                                    <h2>{
                                        if let Some(playlist) = selected_playlist_data {
                                            format!("{} - Tracks", playlist.name)
                                        } else {
                                            "Playlist Tracks".to_string()
                                        }
                                    }</h2>
                                    <button onclick={close_playlist_details} class="btn btn-secondary close-btn">
                                        {"×"}
                                    </button>
                                </div>
                                
                                <div class="modal-body">
                                    {if *tracks_loading {
                                        html! {
                                            <div class="loading">{"Loading tracks..."}</div>
                                        }
                                    } else {
                                        html! {
                                            <div class="playlist-tracks-list">
                                                {for playlist_tracks.iter().enumerate().map(|(index, track)| {
                                                    let track_number = index + 1;
                                                    let duration_minutes = track.duration_ms / 60000;
                                                    let duration_seconds = (track.duration_ms % 60000) / 1000;
                                                    
                                                    html! {
                                                        <div class="track-row" key={track.id.clone()}>
                                                            <div class="track-number">{track_number}</div>
                                                            <div class="track-info">
                                                                <div class="track-name">{&track.name}</div>
                                                                <div class="track-duration">
                                                                    {format!("{}:{:02}", duration_minutes, duration_seconds)}
                                                                </div>
                                                            </div>
                                                            <div class="track-status">
                                                                {if track.youtube_url.is_some() {
                                                                    html! {
                                                                        <span class="status-badge converted">{"✓ Converted"}</span>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <span class="status-badge pending">{"⏳ Pending"}</span>
                                                                    }
                                                                }}
                                                            </div>
                                                            <div class="track-actions">
                                                                <a href={format!("https://open.spotify.com/track/{}", 
                                                                    track.spotify_uri.strip_prefix("spotify:track:").unwrap_or(&track.id))}
                                                                   target="_blank" class="btn btn-small btn-spotify">
                                                                    {"Spotify"}
                                                                </a>
                                                                {if let Some(youtube_url) = &track.youtube_url {
                                                                    html! {
                                                                        <a href={youtube_url.clone()} target="_blank" 
                                                                           class="btn btn-small btn-youtube">
                                                                            {"YouTube"}
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }}
                                                            </div>
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        }
                                    }}
                                </div>
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
