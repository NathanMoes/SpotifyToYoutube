import React, { useState } from 'react';
import { apiService } from '../services/api';
import './PlaylistImport.css';

const PlaylistImport: React.FC = () => {
  const [spotifyUrl, setSpotifyUrl] = useState('');
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState(false);
  const [error, setError] = useState('');

  const extractPlaylistId = (url: string): string | null => {
    const match = url.match(/playlist\/([a-zA-Z0-9]+)/);
    return match ? match[1] : null;
  };

  const handleImport = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!spotifyUrl.trim()) return;

    setLoading(true);
    setError('');
    setSuccess(false);

    try {
      const playlistId = extractPlaylistId(spotifyUrl);
      if (!playlistId) {
        throw new Error('Invalid Spotify playlist URL');
      }

      // Create playlist in database
      const playlistData = {
        external_id: playlistId,
        platform: 'spotify',
        name: 'Imported Playlist', // This would come from Spotify API
        description: 'Imported from Spotify',
        user_id: 'user123', // This would come from auth context
      };

      await apiService.createPlaylist(playlistData);
      setSuccess(true);
      setSpotifyUrl('');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to import playlist');
    } finally {
      setLoading(false);
    }
  };

  const handleAuth = async (platform: 'spotify' | 'youtube') => {
    try {
      if (platform === 'spotify') {
        const response = await apiService.spotifyAuth();
        window.location.href = response.data.auth_url;
      } else {
        const response = await apiService.youtubeAuth();
        window.location.href = response.data.auth_url;
      }
    } catch (err) {
      setError(`Failed to authenticate with ${platform}`);
    }
  };

  return (
    <div className="playlist-import">
      <div className="import-header">
        <h1>Import Spotify Playlist</h1>
        <p>Paste a Spotify playlist URL to import it into your library</p>
      </div>

      <div className="auth-section">
        <h2>Authentication</h2>
        <p>Make sure you're authenticated with both platforms:</p>
        <div className="auth-buttons">
          <button
            className="auth-button spotify"
            onClick={() => handleAuth('spotify')}
          >
            <span className="auth-icon">🎵</span>
            Connect Spotify
          </button>
          <button
            className="auth-button youtube"
            onClick={() => handleAuth('youtube')}
          >
            <span className="auth-icon">📺</span>
            Connect YouTube
          </button>
        </div>
      </div>

      <div className="import-form-section">
        <h2>Import Playlist</h2>
        <form onSubmit={handleImport} className="import-form">
          <div className="form-group">
            <label htmlFor="spotify-url">Spotify Playlist URL</label>
            <input
              id="spotify-url"
              type="text"
              value={spotifyUrl}
              onChange={(e) => setSpotifyUrl(e.target.value)}
              placeholder="https://open.spotify.com/playlist/..."
              className="url-input"
              required
            />
            <div className="url-help">
              Example: https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M
            </div>
          </div>

          <button
            type="submit"
            disabled={loading || !spotifyUrl.trim()}
            className="import-button"
          >
            {loading ? 'Importing...' : 'Import Playlist'}
          </button>
        </form>

        {error && (
          <div className="message error">
            <span className="message-icon">❌</span>
            {error}
          </div>
        )}

        {success && (
          <div className="message success">
            <span className="message-icon">✅</span>
            Playlist imported successfully!
          </div>
        )}
      </div>

      <div className="instructions">
        <h3>How to get a Spotify playlist URL:</h3>
        <ol>
          <li>Open Spotify and navigate to the playlist you want to import</li>
          <li>Click the three dots menu (⋯) next to the playlist name</li>
          <li>Select "Share" → "Copy link to playlist"</li>
          <li>Paste the URL in the field above</li>
        </ol>
      </div>
    </div>
  );
};

export default PlaylistImport;
