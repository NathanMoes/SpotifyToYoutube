import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { apiService, Playlist, Song } from '../services/api';
import './Dashboard.css';

const Dashboard: React.FC = () => {
  const [playlists, setPlaylists] = useState<Playlist[]>([]);
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [songs, setSongs] = useState<Song[]>([]);
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState({
    totalPlaylists: 0,
    totalSongs: 0,
    spotifyPlaylists: 0,
    youtubePlaylists: 0,
  });

  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    try {
      setLoading(true);
      const [playlistsResponse, songsResponse] = await Promise.all([
        apiService.getPlaylists(),
        apiService.getSongs(),
      ]);

      const playlistData = playlistsResponse.data;
      const songData = songsResponse.data;

      setPlaylists(playlistData);
      setSongs(songData);

      setStats({
        totalPlaylists: playlistData.length,
        totalSongs: songData.length,
        spotifyPlaylists: playlistData.filter(p => p.platform === 'spotify').length,
        youtubePlaylists: playlistData.filter(p => p.platform === 'youtube').length,
      });
    } catch (error) {
      console.error('Error fetching data:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="dashboard">
        <div className="loading">Loading dashboard...</div>
      </div>
    );
  }

  return (
    <div className="dashboard">
      <div className="dashboard-header">
        <h1>Dashboard</h1>
        <p>Manage your Spotify to YouTube playlist conversions</p>
      </div>

      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-number">{stats.totalPlaylists}</div>
          <div className="stat-label">Total Playlists</div>
        </div>
        <div className="stat-card">
          <div className="stat-number">{stats.totalSongs}</div>
          <div className="stat-label">Total Songs</div>
        </div>
        <div className="stat-card spotify">
          <div className="stat-number">{stats.spotifyPlaylists}</div>
          <div className="stat-label">Spotify Playlists</div>
        </div>
        <div className="stat-card youtube">
          <div className="stat-number">{stats.youtubePlaylists}</div>
          <div className="stat-label">YouTube Playlists</div>
        </div>
      </div>

      <div className="quick-actions">
        <h2>Quick Actions</h2>
        <div className="action-buttons">
          <Link to="/import" className="action-button primary">
            <span className="action-icon">📥</span>
            Import Spotify Playlist
          </Link>
          <Link to="/playlists" className="action-button">
            <span className="action-icon">📋</span>
            View All Playlists
          </Link>
          <Link to="/songs" className="action-button">
            <span className="action-icon">🎵</span>
            Manage Songs
          </Link>
        </div>
      </div>

      <div className="recent-section">
        <div className="recent-playlists">
          <h3>Recent Playlists</h3>
          {playlists.slice(0, 5).map(playlist => (
            <div key={playlist.id} className="recent-item">
              <div className="item-info">
                <div className="item-name">{playlist.name}</div>
                <div className="item-meta">
                  {playlist.platform} • {playlist.songs.length} songs
                </div>
              </div>
              <div className={`platform-badge ${playlist.platform}`}>
                {playlist.platform}
              </div>
            </div>
          ))}
          {playlists.length === 0 && (
            <div className="empty-state">No playlists yet. Start by importing one!</div>
          )}
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
