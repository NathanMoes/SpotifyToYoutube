import axios from 'axios';

const API_BASE_URL = 'http://localhost:8080/api/v1';

const api = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types based on backend models
export interface Song {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: number;
  spotify_id?: string;
  youtube_id?: string;
  spotify_url?: string;
  youtube_url?: string;
  created_at: string;
  updated_at: string;
}

export interface Playlist {
  id: string;
  name: string;
  description: string;
  user_id: string;
  platform: string;
  external_id: string;
  songs: Song[];
  created_at: string;
  updated_at: string;
}

export interface ConversionRequest {
  playlist_id: string;
  source_platform: string;
  target_platform: string;
}

// API methods
export const apiService = {
  // Health check
  healthCheck: () => api.get('/health'),

  // Authentication
  spotifyAuth: () => api.get('/auth/spotify'),
  youtubeAuth: () => api.get('/auth/youtube'),

  // Playlists
  getPlaylists: () => api.get<Playlist[]>('/playlists'),
  getPlaylist: (id: string) => api.get<Playlist>(`/playlists/${id}`),
  createPlaylist: (playlist: Partial<Playlist>) => api.post<Playlist>('/playlists', playlist),
  updatePlaylist: (id: string, playlist: Partial<Playlist>) => api.put<Playlist>(`/playlists/${id}`, playlist),
  deletePlaylist: (id: string) => api.delete(`/playlists/${id}`),
  convertPlaylist: (id: string, conversion: ConversionRequest) => api.post(`/playlists/${id}/convert`, conversion),
  syncPlaylist: (id: string) => api.post(`/playlists/${id}/sync`),

  // Songs
  getSongs: () => api.get<Song[]>('/songs'),
  getSong: (id: string) => api.get<Song>(`/songs/${id}`),
  createSong: (song: Partial<Song>) => api.post<Song>('/songs', song),
  updateSong: (id: string, song: Partial<Song>) => api.put<Song>(`/songs/${id}`, song),
  deleteSong: (id: string) => api.delete(`/songs/${id}`),
  searchSongs: (query: any) => api.post<Song[]>('/songs/search', query),
};

export default api;
