import React, { useState, useEffect } from 'react';
import { apiService, Song } from '../services/api';
import './SongManager.css';

const SongManager: React.FC = () => {
  const [songs, setSongs] = useState<Song[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [filteredSongs, setFilteredSongs] = useState<Song[]>([]);
  const [selectedSong, setSelectedSong] = useState<Song | null>(null);
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    fetchSongs();
  }, []);

  useEffect(() => {
    if (searchQuery.trim()) {
      const filtered = songs.filter(song =>
        song.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        song.artist.toLowerCase().includes(searchQuery.toLowerCase()) ||
        song.album.toLowerCase().includes(searchQuery.toLowerCase())
      );
      setFilteredSongs(filtered);
    } else {
      setFilteredSongs(songs);
    }
  }, [songs, searchQuery]);

  const fetchSongs = async () => {
    try {
      setLoading(true);
      const response = await apiService.getSongs();
      setSongs(response.data);
    } catch (error) {
      console.error('Error fetching songs:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSongClick = (song: Song) => {
    setSelectedSong(song);
    setShowModal(true);
  };

  const handleDelete = async (songId: string) => {
    if (!window.confirm('Are you sure you want to delete this song?')) {
      return;
    }

    try {
      await apiService.deleteSong(songId);
      await fetchSongs();
    } catch (error) {
      console.error('Error deleting song:', error);
    }
  };

  const formatDuration = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const closeModal = () => {
    setShowModal(false);
    setSelectedSong(null);
  };

  if (loading) {
    return (
      <div className="song-manager">
        <div className="loading">Loading songs...</div>
      </div>
    );
  }

  return (
    <div className="song-manager">
      <div className="manager-header">
        <h1>Song Manager</h1>
        <div className="search-section">
          <input
            type="text"
            placeholder="Search songs, artists, or albums..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="search-input"
          />
          <div className="search-stats">
            Showing {filteredSongs.length} of {songs.length} songs
          </div>
        </div>
      </div>

      {filteredSongs.length === 0 ? (
        <div className="empty-state">
          <div className="empty-icon">🎵</div>
          <h3>No songs found</h3>
          <p>
            {searchQuery
              ? 'Try adjusting your search criteria'
              : 'Songs will appear here when you import playlists'}
          </p>
        </div>
      ) : (
        <div className="songs-table">
          <div className="table-header">
            <div className="col-title">Title</div>
            <div className="col-artist">Artist</div>
            <div className="col-album">Album</div>
            <div className="col-duration">Duration</div>
            <div className="col-platforms">Platforms</div>
            <div className="col-actions">Actions</div>
          </div>
          
          <div className="table-body">
            {filteredSongs.map(song => (
              <div key={song.id} className="table-row" onClick={() => handleSongClick(song)}>
                <div className="col-title">
                  <div className="song-title">{song.title}</div>
                </div>
                <div className="col-artist">{song.artist}</div>
                <div className="col-album">{song.album}</div>
                <div className="col-duration">{formatDuration(song.duration)}</div>
                <div className="col-platforms">
                  <div className="platform-indicators">
                    {song.spotify_id && (
                      <span className="platform-indicator spotify" title="Available on Spotify">
                        🎵
                      </span>
                    )}
                    {song.youtube_id && (
                      <span className="platform-indicator youtube" title="Available on YouTube">
                        📺
                      </span>
                    )}
                  </div>
                </div>
                <div className="col-actions">
                  <button
                    className="action-button delete"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleDelete(song.id);
                    }}
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {showModal && selectedSong && (
        <div className="modal-overlay" onClick={closeModal}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>Song Details</h2>
              <button className="close-button" onClick={closeModal}>×</button>
            </div>
            
            <div className="modal-body">
              <div className="song-details">
                <h3>{selectedSong.title}</h3>
                <p className="song-artist">{selectedSong.artist}</p>
                <p className="song-album">{selectedSong.album}</p>
                <p className="song-duration">Duration: {formatDuration(selectedSong.duration)}</p>
              </div>

              <div className="platform-links">
                <h4>Platform Links:</h4>
                <div className="links-grid">
                  {selectedSong.spotify_url && (
                    <a
                      href={selectedSong.spotify_url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="platform-link spotify"
                    >
                      <span className="link-icon">🎵</span>
                      Open in Spotify
                    </a>
                  )}
                  {selectedSong.youtube_url && (
                    <a
                      href={selectedSong.youtube_url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="platform-link youtube"
                    >
                      <span className="link-icon">📺</span>
                      Open in YouTube
                    </a>
                  )}
                </div>
              </div>

              <div className="song-metadata">
                <h4>Metadata:</h4>
                <div className="metadata-grid">
                  <div className="metadata-item">
                    <span className="metadata-label">Spotify ID:</span>
                    <span className="metadata-value">{selectedSong.spotify_id || 'Not available'}</span>
                  </div>
                  <div className="metadata-item">
                    <span className="metadata-label">YouTube ID:</span>
                    <span className="metadata-value">{selectedSong.youtube_id || 'Not available'}</span>
                  </div>
                  <div className="metadata-item">
                    <span className="metadata-label">Created:</span>
                    <span className="metadata-value">
                      {new Date(selectedSong.created_at).toLocaleDateString()}
                    </span>
                  </div>
                  <div className="metadata-item">
                    <span className="metadata-label">Updated:</span>
                    <span className="metadata-value">
                      {new Date(selectedSong.updated_at).toLocaleDateString()}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default SongManager;
