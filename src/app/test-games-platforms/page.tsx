import { Game, useGames } from '@/hooks/use-games';
import { Platform, usePlatforms } from '@/hooks/use-platforms';
import { useCallback, useEffect, useState } from 'react';

export default function TestGamesPlatformsPage() {
  const { createGame, getGames, getGamesByPlatform, updateGame, deleteGame } =
    useGames();
  const { createPlatform, getPlatforms, updatePlatform, deletePlatform } =
    usePlatforms();

  const [platformName, setPlatformName] = useState('Test Platform');
  const [platformDesc, setPlatformDesc] = useState('Test Description');
  const [platformResult, setPlatformResult] = useState('');
  const [platforms, setPlatforms] = useState<Platform[]>([]);

  const [gameName, setGameName] = useState('Test Game');
  const [gamePlatformId, setGamePlatformId] = useState('');
  const [gameDesc, setGameDesc] = useState('Test Game Description');
  const [gameResult, setGameResult] = useState('');
  const [games, setGames] = useState<Game[]>([]);

  const loadPlatforms = useCallback(async () => {
    try {
      const plats = await getPlatforms();
      setPlatforms(plats);
    } catch (error) {
      console.error('Failed to load platforms:', error);
    }
  }, [getPlatforms, setPlatforms]);

  const loadGames = useCallback(async () => {
    try {
      const gms = await getGames();
      setGames(gms);
    } catch (error) {
      console.error('Failed to load games:', error);
    }
  }, [getGames, setGames]);

  // Automatic tests on mount
  useEffect(() => {
    const runTests = async () => {
      console.log('Running automatic games and platforms tests...');

      try {
        // Create test platform
        const platformId = await createPlatform({
          name: 'Auto Test Platform',
          description: 'Auto created',
        });
        console.log('Platform created with ID:', platformId);

        // Create test game
        const gameId = await createGame({
          name: 'Auto Test Game',
          platform_id: platformId,
          description: 'Auto created game',
        });
        console.log('Game created with ID:', gameId);

        // Get all platforms
        const platforms = await getPlatforms();
        console.log('Platforms retrieved:', platforms.length);

        // Get all games
        const games = await getGames();
        console.log('Games retrieved:', games.length);

        // Get games by platform
        const gamesByPlatform = await getGamesByPlatform(platformId);
        console.log('Games by platform:', gamesByPlatform.length);

        // Update platform
        await updatePlatform({
          id: platformId,
          name: 'Updated Platform',
          description: 'Updated desc',
        });
        console.log('Platform updated');

        // Update game
        await updateGame({
          id: gameId,
          name: 'Updated Game',
          platform_id: platformId,
          description: 'Updated game desc',
        });
        console.log('Game updated');

        // Delete game
        await deleteGame(gameId);
        console.log('Game deleted');

        // Delete platform
        await deletePlatform(platformId);
        console.log('Platform deleted');

        setPlatformResult('Auto tests completed successfully');
        setGameResult('Auto tests completed successfully');
      } catch (error) {
        console.error('Auto tests failed:', error);
        setPlatformResult(`Auto tests failed: ${error}`);
        setGameResult(`Auto tests failed: ${error}`);
      }

      // Load remaining data
      loadPlatforms();
      loadGames();
    };

    runTests();
  }, [
    createPlatform,
    createGame,
    getPlatforms,
    getGames,
    getGamesByPlatform,
    updatePlatform,
    updateGame,
    deleteGame,
    deletePlatform,
    loadPlatforms,
    loadGames,
  ]);

  const testCreatePlatform = async () => {
    try {
      const id = await createPlatform({
        name: platformName,
        description: platformDesc,
      });
      setPlatformResult(`Created platform with ID: ${id}`);
      loadPlatforms();
    } catch (error) {
      setPlatformResult(`Error: ${error}`);
    }
  };

  const testUpdatePlatform = async () => {
    if (platforms.length === 0) return;
    try {
      await updatePlatform({
        id: platforms[0].id,
        name: platformName + ' Updated',
        description: platformDesc,
      });
      setPlatformResult('Updated first platform');
      loadPlatforms();
    } catch (error) {
      setPlatformResult(`Error: ${error}`);
    }
  };

  const testDeletePlatform = async () => {
    if (platforms.length === 0) return;
    try {
      await deletePlatform(platforms[platforms.length - 1].id);
      setPlatformResult('Deleted last platform');
      loadPlatforms();
    } catch (error) {
      setPlatformResult(`Error: ${error}`);
    }
  };

  const testCreateGame = async () => {
    if (!gamePlatformId) return;
    try {
      const id = await createGame({
        name: gameName,
        platform_id: parseInt(gamePlatformId),
        description: gameDesc,
      });
      setGameResult(`Created game with ID: ${id}`);
      loadGames();
    } catch (error) {
      setGameResult(`Error: ${error}`);
    }
  };

  const testUpdateGame = async () => {
    if (games.length === 0) return;
    try {
      await updateGame({
        id: games[0].id,
        name: gameName + ' Updated',
        platform_id: games[0].platform_id,
        description: gameDesc,
      });
      setGameResult('Updated first game');
      loadGames();
    } catch (error) {
      setGameResult(`Error: ${error}`);
    }
  };

  const testDeleteGame = async () => {
    if (games.length === 0) return;
    try {
      await deleteGame(games[games.length - 1].id);
      setGameResult('Deleted last game');
      loadGames();
    } catch (error) {
      setGameResult(`Error: ${error}`);
    }
  };

  const testGetGamesByPlatform = async () => {
    if (!gamePlatformId) return;
    try {
      const gms = await getGamesByPlatform(parseInt(gamePlatformId));
      setGameResult(`Found ${gms.length} games for platform ${gamePlatformId}`);
      setGames(gms);
    } catch (error) {
      setGameResult(`Error: ${error}`);
    }
  };

  return (
    <div className='p-6 space-y-8'>
      <h1 className='text-2xl font-bold'>Games and Platforms Test Page</h1>

      <div className='space-y-4'>
        <h2 className='text-xl font-semibold'>Platforms</h2>
        <div className='flex gap-2'>
          <input
            type='text'
            value={platformName}
            onChange={(e) => setPlatformName(e.target.value)}
            placeholder='Name'
            className='border p-2'
          />
          <input
            type='text'
            value={platformDesc}
            onChange={(e) => setPlatformDesc(e.target.value)}
            placeholder='Description'
            className='border p-2'
          />
          <button
            onClick={testCreatePlatform}
            className='bg-blue-500 text-white px-4 py-2'
          >
            Create
          </button>
          <button
            onClick={testUpdatePlatform}
            className='bg-green-500 text-white px-4 py-2'
          >
            Update First
          </button>
          <button
            onClick={testDeletePlatform}
            className='bg-red-500 text-white px-4 py-2'
          >
            Delete Last
          </button>
        </div>
        <div className='text-sm text-gray-600'>{platformResult}</div>
        <div className='text-sm'>
          <strong>Platforms:</strong>
          <ul>
            {platforms.map((p) => (
              <li key={p.id}>
                {p.id}: {p.name} - {p.description}
              </li>
            ))}
          </ul>
        </div>
      </div>

      <div className='space-y-4'>
        <h2 className='text-xl font-semibold'>Games</h2>
        <div className='flex gap-2'>
          <input
            type='text'
            value={gameName}
            onChange={(e) => setGameName(e.target.value)}
            placeholder='Name'
            className='border p-2'
          />
          <select
            value={gamePlatformId}
            onChange={(e) => setGamePlatformId(e.target.value)}
            className='border p-2'
          >
            <option value=''>Select Platform</option>
            {platforms.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name}
              </option>
            ))}
          </select>
          <input
            type='text'
            value={gameDesc}
            onChange={(e) => setGameDesc(e.target.value)}
            placeholder='Description'
            className='border p-2'
          />
          <button
            onClick={testCreateGame}
            className='bg-blue-500 text-white px-4 py-2'
          >
            Create
          </button>
          <button
            onClick={testUpdateGame}
            className='bg-green-500 text-white px-4 py-2'
          >
            Update First
          </button>
          <button
            onClick={testDeleteGame}
            className='bg-red-500 text-white px-4 py-2'
          >
            Delete Last
          </button>
          <button
            onClick={testGetGamesByPlatform}
            className='bg-purple-500 text-white px-4 py-2'
          >
            Get by Platform
          </button>
        </div>
        <div className='text-sm text-gray-600'>{gameResult}</div>
        <div className='text-sm'>
          <strong>Games:</strong>
          <ul>
            {games.map((g) => (
              <li key={g.id}>
                {g.id}: {g.name} (Platform {g.platform_id}) - {g.description}
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}
