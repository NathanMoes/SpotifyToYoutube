# Spotify to YouTube Frontend

A React-based web application for managing Spotify to YouTube playlist conversions.

## Features

### 🎵 Dashboard
- Overview of all playlists and songs
- Statistics showing total playlists, songs, and platform distribution
- Quick access to main features
- Recent playlists display

### 📥 Playlist Import
- Import Spotify playlists using playlist URLs
- Authentication with Spotify and YouTube
- Easy-to-use interface with URL validation
- Step-by-step instructions for obtaining playlist URLs

### 📋 Playlist Management
- View all imported playlists
- Filter by platform (Spotify, YouTube, or All)
- Convert playlists between Spotify and YouTube
- Delete playlists
- View playlist details including songs and duration
- Direct links to open playlists in their respective platforms

### 🎶 Song Manager
- Comprehensive song database view
- Search functionality across titles, artists, and albums
- Detailed song information including platform availability
- Modal view for detailed song metadata
- Platform-specific links (Spotify and YouTube URLs)

## Getting Started

### Prerequisites
- Node.js (version 16 or higher)
- npm or yarn
- Backend server running on port 8080

### Installation

1. Install dependencies:
   ```bash
   npm install
   ```

2. Start the development server:
   ```bash
   npm start
   ```

3. Open [http://localhost:3000](http://localhost:3000) in your browser

### Environment Setup
Make sure your backend server is running on `http://localhost:8080` with the following endpoints available:
- `/api/v1/auth/spotify` - Spotify authentication
- `/api/v1/auth/youtube` - YouTube authentication
- `/api/v1/playlists` - Playlist CRUD operations
- `/api/v1/songs` - Song management

## Usage

### Importing a Playlist
1. Navigate to the "Import Playlist" page
2. Authenticate with Spotify and YouTube if needed
3. Paste a Spotify playlist URL (e.g., `https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M`)
4. Click "Import Playlist"

### Converting Playlists
1. Go to the "Playlists" page
2. Find the playlist you want to convert
3. Click "Convert to YouTube" (or "Convert to Spotify")
4. The system will create a corresponding playlist on the target platform

### Managing Songs
1. Visit the "Songs" page
2. Use the search bar to find specific songs
3. Click on any song to view detailed information
4. Access direct links to the song on Spotify or YouTube

## API Integration

The frontend communicates with the backend through a RESTful API. Key endpoints include:

- **Authentication**: Handle OAuth flows for Spotify and YouTube
- **Playlists**: CRUD operations for playlist management
- **Songs**: Song database operations and search functionality
- **Conversion**: Convert playlists between platforms

## Technologies Used

- **React** - Frontend framework
- **TypeScript** - Type safety
- **React Router** - Navigation
- **Axios** - HTTP client
- **CSS3** - Styling with modern design principles

## Development

### Available Scripts

- `npm start` - Start development server
- `npm build` - Build for production
- `npm test` - Run tests
- `npm run eject` - Eject from Create React App

### Code Structure

```
src/
├── components/          # React components
│   ├── Dashboard.tsx    # Main dashboard
│   ├── Header.tsx       # Navigation header
│   ├── PlaylistImport.tsx # Playlist import form
│   ├── PlaylistView.tsx # Playlist listing and management
│   └── SongManager.tsx  # Song database management
├── services/           # API service layer
│   └── api.ts         # API client and types
├── App.tsx            # Main application component
└── App.css            # Global styles
```

## Features Overview

### 🎯 Key Functionality
- **Playlist Import**: Seamlessly import Spotify playlists
- **Cross-Platform Conversion**: Convert playlists between Spotify and YouTube
- **Song Management**: Comprehensive song database with search capabilities
- **Platform Integration**: Direct links and authentication with both platforms
- **Modern UI**: Clean, responsive design with platform-specific styling

### 🔧 Technical Features
- **Type Safety**: Full TypeScript implementation
- **Error Handling**: Comprehensive error handling throughout the application
- **Responsive Design**: Mobile-friendly interface
- **Real-time Updates**: Live data synchronization with backend
- **Authentication**: OAuth integration for both platforms

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License.
