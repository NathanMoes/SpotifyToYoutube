import React, { useState, useEffect } from 'react';
import { apiService, Playlist, ConversionRequest } from '../services/api';
import './PlaylistView.css';

const PlaylistView: React.FC = () => {
  const [playlists, setPlaylists] = useState<Playlist[]>([]);
  const [loading, setLoading] = useState(true);
  const [convertingId, setConvertingId] = useState<string | null>(null);
  const [filter, setFilter] = useState<'all' | 'spotify' | 'youtube'>('all');

  useEffect(() => {
    fetchPlaylists();
  }, []);

  const fetchPlaylists = async () => {
    try {
      setLoading(true);
      const response = await apiService.getPlaylists();
      setPlaylists(response.data);
    } catch (error) {
      console.error('Error fetching playlists:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleConvert = async (playlist: Playlist) => {
    const targetPlatform = playlist.platform === 'spotify' ? 'youtube' : 'spotify';
    
    try {
      setConvertingId(playlist.id);
      
      const conversion: ConversionRequest = {
        playlist_id: playlist.id,
        source_platform: playlist.platform,
        target_platform: targetPlatform,
      };

      await apiService.convertPlaylist(playlist.id, conversion);
      
      // Refresh playlists after conversion
      await fetchPlaylists();
    } catch (error) {
      console.error('Error converting playlist:', error);
    } finally {
      setConvertingId(null);
    }
  };

  const handleDelete = async (playlistId: string) => {
    if (!window.confirm('Are you sure you want to delete this playlist?')) {
      return;
    }

    try {
      await apiService.deletePlaylist(playlistId);
      await fetchPlaylists();
    } catch (error) {
      console.error('Error deleting playlist:', error);
    }
  };

  const filteredPlaylists = playlists.filter(playlist => {
    if (filter === 'all') return true;
    return playlist.platform === filter;
  });

  const formatDuration = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  if (loading) {
    return (
      <div className="playlist-view">
        <div className="loading">Loading playlists...</div>
      </div>
    );
  }

  return (
    <div className="playlist-view">
      <div className="view-header">
        <h1>Your Playlists</h1>
        <div className="filter-controls">
          <button
            className={`filter-button ${filter === 'all' ? 'active' : ''}`}
            onClick={() => setFilter('all')}
          >
            All ({playlists.length})
          </button>
          <button
            className={`filter-button ${filter === 'spotify' ? 'active' : ''}`}
            onClick={() => setFilter('spotify')}
          >
            Spotify ({playlists.filter(p => p.platform === 'spotify').length})
          </button>
          <button
            className={`filter-button ${filter === 'youtube' ? 'active' : ''}`}
            onClick={() => setFilter('youtube')}
          >
            YouTube ({playlists.filter(p => p.platform === 'youtube').length})
          </button>
        </div>
      </div>

      {filteredPlaylists.length === 0 ? (
        <div className="empty-state">
          <div className="empty-icon">📋</div>
          <h3>No playlists found</h3>
          <p>Start by importing a Spotify playlist or create a new one</p>
        </div>
      ) : (
        <div className="playlists-grid">
          {filteredPlaylists.map(playlist => (
            <div key={playlist.id} className="playlist-card">
              <div className="playlist-header">
                <h3 className="playlist-name">{playlist.name}</h3>
                <div className={`platform-badge ${playlist.platform}`}>
                  {playlist.platform}
                </div>
              </div>
              
              <div className="playlist-info">
                <p className="playlist-description">{playlist.description}</p>
                <div className="playlist-stats">
                  <span className="stat">
                    <span className="stat-icon">🎵</span>
                    {playlist.songs.length} songs
                  </span>
                  <span className="stat">
                    <span className="stat-icon">⏱️</span>
                    {Math.round(playlist.songs.reduce((total, song) => total + song.duration, 0) / 60)} min
                  </span>
                </div>
              </div>

              <div className="playlist-songs">
                <h4>Songs:</h4>
                <div className="songs-list">
                  {playlist.songs.slice(0, 3).map(song => (
                    <div key={song.id} className="song-item">
                      <div className="song-info">
                        <div className="song-title">{song.title}</div>
                        <div className="song-artist">{song.artist}</div>
                      </div>
                      <div className="song-duration">
                        {formatDuration(song.duration)}
                      </div>
                    </div>
                  ))}
                  {playlist.songs.length > 3 && (
                    <div className="more-songs">
                      +{playlist.songs.length - 3} more songs
                    </div>
                  )}
                </div>
              </div>

              <div className="playlist-actions">
                <button
                  className="action-button convert"
                  onClick={() => handleConvert(playlist)}
                  disabled={convertingId === playlist.id}
                >
                  {convertingId === playlist.id ? (
                    'Converting...'
                  ) : (
                    `Convert to ${playlist.platform === 'spotify' ? 'YouTube' : 'Spotify'}`
                  )}
                </button>
                <button
                  className="action-button delete"
                  onClick={() => handleDelete(playlist.id)}
                >
                  Delete
                </button>
              </div>

              <div className="playlist-links">
                {playlist.platform === 'spotify' && (
                  <a
                    href={`https://open.spotify.com/playlist/${playlist.external_id}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="external-link spotify"
                  >
                    <span className="link-icon">🎵</span>
                    Open in Spotify
                  </a>
                )}
                {playlist.platform === 'youtube' && (
                  <a
                    href={`https://www.youtube.com/playlist?list=${playlist.external_id}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="external-link youtube"
                  >
                    <span className="link-icon">📺</span>
                    Open in YouTube
                  </a>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default PlaylistView;
