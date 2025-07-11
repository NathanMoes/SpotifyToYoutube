# Improved Spotify API with Automatic Token Refresh

This implementation provides a robust Spotify API client with automatic token refresh capabilities.

## Features

- **Automatic Token Refresh**: Tokens are refreshed automatically 2 minutes before expiry
- **Background Token Management**: Token refresh runs in background without blocking your application
- **Token Persistence**: Save and restore tokens to/from storage
- **Multiple Initialization Methods**: Support for first-time auth, existing tokens, or fresh auth flows
- **Comprehensive Error Handling**: Proper error handling with detailed error messages

## Key Improvements

### 1. Token Management
- `access_token` and `refresh_token` are now `Option<String>` for better null safety
- Added `expires_at` field to track token expiration
- Added `is_token_expired()` method to check token validity

### 2. Automatic Refresh
- `ensure_valid_token()` automatically refreshes tokens when needed
- `start_token_refresh_loop()` runs in background to refresh tokens before expiry
- All API methods now call `ensure_valid_token()` before making requests

### 3. Multiple Initialization Options
- `SpotifyApi::new()` - Basic initialization
- `SpotifyApi::with_tokens()` - Initialize with existing tokens
- `SpotifyApi::from_auth_code()` - Initialize from authorization code

### 4. Token Persistence
- `get_token_state()` - Get current tokens for saving
- `restore_token_state()` - Restore tokens from saved state
- `TokenState` struct for easy serialization

## Usage Examples

### First Time Authentication
```rust
let mut spotify_api = SpotifyApi::from_auth_code(
    client_id,
    client_secret,
    redirect_uri,
    auth_code,
).await?;

// Start background token refresh
let refresh_api = spotify_api.clone();
tokio::spawn(async move {
    refresh_api.start_token_refresh_loop().await;
});

// Use API normally - tokens will be refreshed automatically
let user_profile = spotify_api.fetch_user_profile().await?;
```

### Using Existing Tokens
```rust
let mut spotify_api = SpotifyApi::with_tokens(
    client_id,
    client_secret,
    redirect_uri,
    access_token,
    refresh_token,
    expires_at,
);

// Start background token refresh
let refresh_api = spotify_api.clone();
tokio::spawn(async move {
    refresh_api.start_token_refresh_loop().await;
});

// Use API normally
let playlists = spotify_api.fetch_user_playlists().await?;
```

### Token Persistence
```rust
// Save tokens
if let Some(token_state) = spotify_api.get_token_state() {
    let json = serde_json::to_string_pretty(&token_state)?;
    std::fs::write("tokens.json", json)?;
}

// Load tokens
let json = std::fs::read_to_string("tokens.json")?;
let token_state: TokenState = serde_json::from_str(&json)?;
spotify_api.restore_token_state(token_state);
```

## Environment Variables

Set these environment variables for easy configuration:

```bash
SPOTIFY_CLIENT_ID=your_client_id
SPOTIFY_CLIENT_SECRET=your_client_secret
SPOTIFY_REDIRECT_URI=your_redirect_uri
SPOTIFY_AUTH_CODE=authorization_code_from_callback
```

## Error Handling

The API now provides better error handling with specific error types:
- Invalid data errors for missing tokens/codes
- Permission denied errors for unauthorized access
- Network errors for API communication issues

## Background Token Refresh

The `start_token_refresh_loop()` method:
- Calculates when tokens will expire
- Refreshes tokens 2 minutes before expiry
- Handles refresh failures with retry logic
- Runs indefinitely in the background

This ensures your application never has to deal with expired tokens during normal operation.
