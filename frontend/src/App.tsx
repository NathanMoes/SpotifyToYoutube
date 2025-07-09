import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import './App.css';
import Header from './components/Header';
import Dashboard from './components/Dashboard';
import PlaylistImport from './components/PlaylistImport';
import PlaylistView from './components/PlaylistView';
import SongManager from './components/SongManager';

function App() {
  return (
    <Router>
      <div className="App">
        <Header />
        <main className="main-content">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/import" element={<PlaylistImport />} />
            <Route path="/playlists" element={<PlaylistView />} />
            <Route path="/songs" element={<SongManager />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
