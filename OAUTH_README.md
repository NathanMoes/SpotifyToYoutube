# Automated Spotify OAuth Flow

This application now includes an automated OAuth flow that eliminates the need to manually copy and paste authorization codes.

## How it works

1. **Temporary OAuth Server**: When authentication is needed, the application starts a temporary web server on port 8080 to handle the OAuth callback.

2. **Automatic Browser Opening**: The application attempts to automatically open your default browser with the Spotify authorization URL.

3. **Callback Handling**: After you authorize the application in your browser, Spotify redirects to `http://localhost:8080/callback` with the authorization code.

4. **Automatic Processing**: The temporary server captures the authorization code and automatically processes it to obtain access tokens.

5. **Token Storage**: The tokens are automatically saved to `tokens.json` for future use.

## Setup

1. **Environment Variables**: Make sure your `.env` file contains:
   ```
   SPOTIFY_CLIENT_ID=your_spotify_client_id
   SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
   SPOTIFY_REDIRECT_URI=http://localhost:8080/callback
   ```

2. **Spotify App Configuration**: In your Spotify app settings at https://developer.spotify.com/dashboard, make sure to add `http://localhost:8080/callback` as a redirect URI.

## Usage

1. **First Run**: When you run the application for the first time or when tokens expire:
   ```bash
   cargo run
   ```

2. **Follow the prompts**: The application will:
   - Display the authorization URL
   - Try to open your browser automatically
   - Wait for you to authorize the application
   - Automatically process the callback and save tokens

3. **Subsequent Runs**: Once tokens are saved, the application will use them automatically without requiring re-authorization (unless they expire).

## Troubleshooting

- **Browser doesn't open automatically**: If the browser doesn't open, copy the displayed URL and paste it into your browser manually.
- **Port 8080 in use**: If port 8080 is already in use, you can modify the `oauth_port` variable in `main.rs` and update your Spotify app's redirect URI accordingly.
- **Timeout**: The OAuth flow has a 5-minute timeout. If you don't complete authorization within this time, the process will fail and you'll need to restart the application.

## Security Notes

- The temporary OAuth server only runs during the authentication process and shuts down immediately after receiving the authorization code.
- Tokens are stored locally in `tokens.json` and should be kept secure.
- The application includes automatic token refresh functionality to maintain valid access tokens.
