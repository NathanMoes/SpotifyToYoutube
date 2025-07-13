# Logging and Tracing Documentation

This backend includes comprehensive logging and tracing capabilities to help monitor request/response patterns, performance, and troubleshoot issues.

## Features

### 🔍 Request/Response Logging
- **HTTP Request Tracking**: Every incoming request is logged with method, URI, user agent, and unique request ID
- **Response Metrics**: All responses include status code and response time in milliseconds
- **Slow Request Detection**: Requests taking longer than 1 second are automatically flagged as slow
- **Error Context**: Failed requests include detailed error information

### 📊 Performance Monitoring
- **Response Time Tracking**: All requests are timed automatically
- **Operation Timing**: Individual operations within handlers are tracked
- **Database Query Performance**: Database operations are instrumented for performance analysis

### 🎯 Structured Logging
- **JSON Output**: Production-ready structured logging with configurable format
- **Context Fields**: Each log entry includes relevant context (track IDs, playlist IDs, etc.)
- **Correlation IDs**: Request tracing with unique identifiers for following requests across services

## Configuration

### Environment Variables

Configure logging through environment variables in your `.env` file:

```bash
# Log Level - Controls verbosity
RUST_LOG=info,spotify_to_youtube_backend=debug

# Log Format - "json" for production, empty for development
LOG_FORMAT=json
```

### Log Levels

- **`trace`**: Very detailed debugging information
- **`debug`**: Detailed information for diagnosing problems
- **`info`**: General information about application operation (default)
- **`warn`**: Warning messages for potentially harmful situations
- **`error`**: Error events that might allow the application to continue

### Module-Specific Logging

You can configure different log levels for different parts of the application:

```bash
# More verbose logging for the main application
RUST_LOG=info,spotify_to_youtube_backend=debug

# Enable trace logging for database operations
RUST_LOG=info,spotify_to_youtube_backend::database=trace

# Quiet external libraries but verbose for our code
RUST_LOG=warn,spotify_to_youtube_backend=debug
```

## Log Output Examples

### Development Format (Human-Readable)
```
2024-07-13T10:30:15.123Z INFO  spotify_to_youtube_backend: Application starting port="3000" rust_version="1.70.0" app_version="0.1.0"
2024-07-13T10:30:15.456Z INFO  spotify_to_youtube_backend: HTTP request started method="POST" uri="/api/playlists/123/store"
2024-07-13T10:30:15.789Z INFO  spotify_to_youtube_backend: Successfully stored playlist in database playlist_id="123"
2024-07-13T10:30:15.790Z INFO  spotify_to_youtube_backend: HTTP request completed status="200" latency_ms="334"
```

### Production Format (JSON)
```json
{
  "timestamp": "2024-07-13T10:30:15.123Z",
  "level": "INFO",
  "target": "spotify_to_youtube_backend",
  "message": "HTTP request started",
  "fields": {
    "method": "POST",
    "uri": "/api/playlists/123/store",
    "request_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479"
  }
}
```

## Request Tracing

Each HTTP request gets a unique request ID that appears in all related log entries:

```
request_id="f47ac10b-58cc-4372-a567-0e02b2c3d479"
```

This allows you to trace a single request through the entire system, making debugging much easier.

## Performance Monitoring

### Response Time Tracking
All requests are automatically timed and logged:
```
latency_ms="156" status="200" "HTTP request completed"
```

### Slow Request Detection
Requests taking longer than 1 second trigger a warning:
```
WARN latency_ms="1234" status="200" "Slow request detected"
```

### Operation Timing
You can use the timing macros in your code:
```rust
use crate::time_operation;

let result = time_operation!(
    expensive_database_query().await,
    operation = "playlist_fetch",
    playlist_id = %playlist_id
);
```

## Error Logging

Errors are logged with full context:

```rust
use crate::log_error_context;

if let Err(e) = risky_operation().await {
    log_error_context!(e, "Failed to process playlist", playlist_id = %playlist_id);
}
```

## API Endpoint Monitoring

Each API endpoint is instrumented to log:

- **Request Parameters**: Extracted path and query parameters
- **Request Body**: For POST/PUT requests (sensitive data is filtered)
- **Processing Time**: How long each operation takes
- **Success/Failure**: With detailed error context on failure

### Example API Logs

```
INFO  Storing playlist in database playlist_id="441K4rF3u0qfg9m4X1WSQJ"
INFO  Successfully stored playlist in database
INFO  HTTP request completed status="200" latency_ms="1247"
```

## Log Analysis

### Finding Slow Requests
```bash
grep "Slow request detected" app.log
```

### Tracking Errors
```bash
grep "ERROR" app.log | grep "playlist"
```

### Performance Analysis
```bash
grep "latency_ms" app.log | sort -k3 -n
```

### Following a Specific Request
```bash
grep "f47ac10b-58cc-4372-a567-0e02b2c3d479" app.log
```

## Production Deployment

For production environments:

1. **Set JSON Logging**:
   ```bash
   LOG_FORMAT=json
   ```

2. **Use Appropriate Log Level**:
   ```bash
   RUST_LOG=info,spotify_to_youtube_backend=info
   ```

3. **Log Aggregation**: Use tools like:
   - **ELK Stack** (Elasticsearch, Logstash, Kibana)
   - **Grafana Loki**
   - **Fluentd**
   - **Datadog**
   - **New Relic**

4. **Log Rotation**: Configure log rotation to prevent disk space issues

## Troubleshooting

### Common Issues

1. **No Logs Appearing**:
   - Check `RUST_LOG` environment variable
   - Ensure logging is initialized in main.rs

2. **Too Verbose**:
   - Reduce log level: `RUST_LOG=warn`
   - Filter specific modules: `RUST_LOG=warn,spotify_to_youtube_backend=info`

3. **Performance Impact**:
   - Avoid `trace` level in production
   - Use async logging for high-throughput scenarios

### Debug Mode

For maximum verbosity during development:
```bash
RUST_LOG=trace,spotify_to_youtube_backend=trace
```

This will show everything including:
- HTTP request/response headers
- Database query details
- All function entry/exit points
- Variable values and state changes
