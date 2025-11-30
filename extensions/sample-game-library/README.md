# Sample Game Library Extension

A sample extension for Arcadia App that demonstrates how game library extensions work. This extension provides stub implementations for game scanning, data provision, and database integration.

## Features

- **Game Scanning**: Automatically scans for installed games (stub implementation)
- **Game Data Provision**: Provides game metadata and information
- **Database Integration**: Demonstrates storing and retrieving game data
- **Hook System**: Responds to application lifecycle events
- **API System**: Provides callable APIs for game management

## Installation

1. **Copy the Extension**: Copy the entire `sample-game-library` directory to your Arcadia App extensions folder:

   ```
   ~/.arcadia/extensions/
   ```

2. **Install via App**: Open Arcadia App and go to Extensions > Install Extension, then select the `manifest.json` file from this directory.

3. **Grant Permissions**: When prompted, grant the following permissions:
   - `filesystem`: For scanning game directories
   - `database`: For storing game metadata

## Usage

### Automatic Features

Once installed and enabled, the extension will:

- **On App Startup**: Initialize and prepare for game scanning
- **Game Scanning**: Periodically scan for new games (triggered by hooks)

### API Usage

The extension provides several APIs that can be called from the main application:

#### Scan for Games

```typescript
import { callExtensionApi } from '@/lib/extensions';

// Trigger a game scan
const result = await callExtensionApi(
  'sample-game-library-id',
  'scan_games',
  {}
);
console.log(`Found ${result.scanned} games`);
```

#### Get Games List

```typescript
// Get paginated list of games
const games = await callExtensionApi('sample-game-library-id', 'get_games', {
  limit: 20,
  offset: 0,
});
console.log(`Total games: ${games.total}`);
```

#### Get Game Details

```typescript
// Get detailed information for a specific game
const gameDetails = await callExtensionApi(
  'sample-game-library-id',
  'get_game_details',
  {
    game_id: 'game1',
  }
);
console.log(`Game: ${gameDetails.name}, Platform: ${gameDetails.platform}`);
```

#### Launch Game

```typescript
// Launch a game
const launchResult = await callExtensionApi(
  'sample-game-library-id',
  'launch_game',
  {
    game_id: 'game1',
  }
);
console.log(launchResult.message);
```

### Hook Responses

The extension responds to the following hooks:

#### `on_startup`

Triggered when the application starts. The extension initializes and reports readiness.

#### `on_game_scan`

Triggered when a game scan is requested. The extension would perform a scan operation.

## Sample Data

The extension includes sample game data for demonstration:

- **Sample Game 1**: A PC game with cover art and playtime
- **Sample Game 2**: Another PC game with different metadata

## Database Integration

The extension demonstrates database integration by:

1. **Storing Games**: When games are scanned, they're stored in the app's database
2. **Retrieving Games**: Game data can be queried from the database
3. **Metadata Management**: Game metadata is managed through database operations

## Extension Structure

```
sample-game-library/
├── manifest.json          # Extension metadata and configuration
├── src/
│   └── lib.rs            # Rust implementation
└── README.md             # This file
```

## Manifest Configuration

The `manifest.json` defines:

- **Metadata**: Name, version, author, description
- **Type**: `game_library` (identifies this as a game library extension)
- **Permissions**: `filesystem` and `database` access
- **Hooks**: `on_startup` and `on_game_scan` event handlers
- **APIs**: Provided APIs (`scan_games`, `get_games`, etc.) and required APIs

## Development Notes

This is a sample extension demonstrating the extension system capabilities. In a real game library extension, you would:

- Implement actual filesystem scanning for game installations
- Connect to real game platform APIs (Steam, Epic Games, etc.)
- Use proper database queries instead of stub implementations
- Handle actual game launching processes
- Implement proper error handling and logging

## Troubleshooting

- **Extension Not Loading**: Check that the manifest.json is valid JSON and all required fields are present
- **Permissions Denied**: Ensure the extension has been granted the necessary permissions
- **API Calls Failing**: Verify the extension ID and API method names are correct
- **Database Errors**: Check that the app's database is properly initialized

## License

This sample extension is provided as-is for demonstration purposes.
