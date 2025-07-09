import React from 'react';
import { Link } from 'react-router-dom';
import './Header.css';

const Header: React.FC = () => {
  return (
    <header className="header">
      <div className="header-container">
        <Link to="/" className="logo">
          <h1>Spotify → YouTube</h1>
        </Link>
        <nav className="nav">
          <Link to="/" className="nav-link">Dashboard</Link>
          <Link to="/import" className="nav-link">Import Playlist</Link>
          <Link to="/playlists" className="nav-link">Playlists</Link>
          <Link to="/songs" className="nav-link">Songs</Link>
        </nav>
      </div>
    </header>
  );
};

export default Header;
