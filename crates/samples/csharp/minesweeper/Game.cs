using System;
using Windows.Foundation.Numerics;
using Windows.UI.Composition;

namespace CSharpMinesweeper;

internal readonly record struct GridSize(int Width, int Height);

internal readonly record struct TileCoordinate(int X, int Y);

internal enum MineState
{
    Empty,
    Flag,
    Question,
    Revealed,
}

internal sealed class MinesweeperGame : IDisposable
{
    private const int Width = 16;
    private const int Height = 16;
    private const int MineCount = 40;
    private const int TileCount = Width * Height;

    private readonly CompositionBoard _ui;
    private readonly MineState[] _mineStates = new MineState[TileCount];
    private readonly bool[] _mines = new bool[TileCount];
    private readonly int[] _neighborCounts = new int[TileCount];
    private readonly int[] _sweepQueue = new int[TileCount];
    private readonly int[] _animationMines = new int[TileCount];
    private readonly int[] _animationRings = new int[32];
    private XorShift64 _rng;
    private bool _minesGenerated;
    private bool _gameOver;
    private TileCoordinate? _lastTile;

    internal MinesweeperGame(
        Compositor compositor,
        ContainerVisual parentVisual,
        Vector2 parentSize)
    {
        _rng = new XorShift64(
            unchecked((ulong)DateTime.UtcNow.Ticks ^ (ulong)Environment.TickCount64));
        _ui = new CompositionBoard(
            compositor,
            parentVisual,
            parentSize,
            new GridSize(Width, Height));
        try
        {
            NewGame();
            OnParentSizeChanged(parentSize);
        }
        catch
        {
            _ui.Dispose();
            throw;
        }
    }

    internal void OnPointerMoved(Vector2 point)
    {
        if (_gameOver || _ui.IsAnimationPlaying)
        {
            return;
        }

        TileCoordinate? selected = null;
        TileCoordinate? hit = _ui.HitTest(point);
        if (hit is TileCoordinate tile)
        {
            _lastTile = tile;
            if (_mineStates[Index(tile.X, tile.Y)] != MineState.Revealed)
            {
                selected = tile;
            }
        }
        else
        {
            _lastTile = null;
        }
        _ui.SelectTile(selected);
    }

    internal void OnParentSizeChanged(Vector2 newSize) => _ui.Resize(newSize);

    internal void OnPointerPressed(bool isRightButton, bool isEraser)
    {
        if (_gameOver)
        {
            NewGame();
        }

        TileCoordinate? selection = _ui.CurrentSelectedTile;
        if (selection is TileCoordinate tile)
        {
            int index = Index(tile.X, tile.Y);
            if (_mineStates[index] == MineState.Revealed)
            {
                return;
            }

            if (isRightButton || isEraser)
            {
                MineState state = Cycle(_mineStates[index]);
                _mineStates[index] = state;
                _ui.UpdateTileWithState(tile, state);
            }
            else if (_mineStates[index] == MineState.Empty && Sweep(tile.X, tile.Y))
            {
                Lose(tile);
            }
            else if (_mineStates[index] == MineState.Empty && CheckIfWon())
            {
                _ui.SelectTile(null);
                _gameOver = true;
            }
        }
        else if (!isRightButton && !isEraser)
        {
            CheckAndClearSatisfied();
        }
    }

    private void NewGame()
    {
        _ui.Reset(new GridSize(Width, Height));
        Array.Fill(_mineStates, MineState.Empty);
        Array.Clear(_mines);
        Array.Clear(_neighborCounts);
        _minesGenerated = false;
        _gameOver = false;
        _lastTile = null;
    }

    private bool Sweep(int x, int y)
    {
        if (!_minesGenerated)
        {
            GenerateMines(x, y);
            _minesGenerated = true;
        }

        int head = 0;
        int tail = 0;
        int first = Index(x, y);
        _sweepQueue[tail++] = first;
        Reveal(first);

        while (head < tail)
        {
            int index = _sweepQueue[head++];
            int currentX = XFromIndex(index);
            int currentY = YFromIndex(index);
            if (_mines[index])
            {
                return true;
            }

            if (_neighborCounts[index] == 0)
            {
                PushIfUnmarked(currentX - 1, currentY - 1, ref tail);
                PushIfUnmarked(currentX, currentY - 1, ref tail);
                PushIfUnmarked(currentX + 1, currentY - 1, ref tail);
                PushIfUnmarked(currentX + 1, currentY, ref tail);
                PushIfUnmarked(currentX + 1, currentY + 1, ref tail);
                PushIfUnmarked(currentX, currentY + 1, ref tail);
                PushIfUnmarked(currentX - 1, currentY + 1, ref tail);
                PushIfUnmarked(currentX - 1, currentY, ref tail);
            }
        }
        return false;
    }

    private void PushIfUnmarked(int x, int y, ref int tail)
    {
        if (!InBounds(x, y))
        {
            return;
        }
        int index = Index(x, y);
        if (_mineStates[index] != MineState.Empty)
        {
            return;
        }
        Reveal(index);
        _sweepQueue[tail++] = index;
    }

    private void Reveal(int index)
    {
        TileCoordinate tile = new(XFromIndex(index), YFromIndex(index));
        if (_mines[index])
        {
            _ui.UpdateTileAsMine(tile);
        }
        else
        {
            _ui.UpdateTileWithMineCount(tile, _neighborCounts[index]);
        }
        _mineStates[index] = MineState.Revealed;
    }

    private void GenerateMines(int excludeX, int excludeY)
    {
        Array.Clear(_mines);
        int exclude = Index(excludeX, excludeY);
        for (int count = 0; count < MineCount; count++)
        {
            while (true)
            {
                int index = _rng.Next(TileCount);
                if (index != exclude && !_mines[index])
                {
                    _mines[index] = true;
                    break;
                }
            }
        }

        for (int index = 0; index < TileCount; index++)
        {
            _neighborCounts[index] = _mines[index]
                ? -1
                : SurroundingMineCount(XFromIndex(index), YFromIndex(index));
        }
    }

    private int SurroundingMineCount(int x, int y)
    {
        int count = 0;
        count += IsMine(x + 1, y) ? 1 : 0;
        count += IsMine(x - 1, y) ? 1 : 0;
        count += IsMine(x, y + 1) ? 1 : 0;
        count += IsMine(x, y - 1) ? 1 : 0;
        count += IsMine(x + 1, y + 1) ? 1 : 0;
        count += IsMine(x - 1, y - 1) ? 1 : 0;
        count += IsMine(x - 1, y + 1) ? 1 : 0;
        count += IsMine(x + 1, y - 1) ? 1 : 0;
        return count;
    }

    private bool IsMine(int x, int y) => InBounds(x, y) && _mines[Index(x, y)];

    private void CheckAndClearSatisfied()
    {
        if (_lastTile is not TileCoordinate current)
        {
            return;
        }

        int currentIndex = Index(current.X, current.Y);
        if (_neighborCounts[currentIndex] < 1 ||
            _mineStates[currentIndex] != MineState.Revealed)
        {
            return;
        }

        Span<TileCoordinate> neighbors = stackalloc TileCoordinate[8];
        int neighborCount = 0;
        AddNeighbor(current.X - 1, current.Y - 1, neighbors, ref neighborCount);
        AddNeighbor(current.X, current.Y - 1, neighbors, ref neighborCount);
        AddNeighbor(current.X + 1, current.Y - 1, neighbors, ref neighborCount);
        AddNeighbor(current.X - 1, current.Y, neighbors, ref neighborCount);
        AddNeighbor(current.X + 1, current.Y, neighbors, ref neighborCount);
        AddNeighbor(current.X - 1, current.Y + 1, neighbors, ref neighborCount);
        AddNeighbor(current.X, current.Y + 1, neighbors, ref neighborCount);
        AddNeighbor(current.X + 1, current.Y + 1, neighbors, ref neighborCount);

        int flags = 0;
        for (int i = 0; i < neighborCount; i++)
        {
            TileCoordinate tile = neighbors[i];
            if (_mineStates[Index(tile.X, tile.Y)] == MineState.Flag)
            {
                flags++;
            }
        }
        if (flags != _neighborCounts[currentIndex])
        {
            return;
        }

        for (int i = 0; i < neighborCount; i++)
        {
            TileCoordinate tile = neighbors[i];
            int index = Index(tile.X, tile.Y);
            if (_mineStates[index] == MineState.Empty && Sweep(tile.X, tile.Y))
            {
                Lose(tile);
                return;
            }
        }

        if (CheckIfWon())
        {
            _ui.SelectTile(null);
            _gameOver = true;
        }
    }

    private static void AddNeighbor(
        int x,
        int y,
        Span<TileCoordinate> neighbors,
        ref int count)
    {
        if (InBounds(x, y))
        {
            neighbors[count++] = new TileCoordinate(x, y);
        }
    }

    private void Lose(TileCoordinate hit)
    {
        _ui.SelectTile(null);
        PlayAnimationOnAllMines(hit.X, hit.Y);
        _gameOver = true;
    }

    private void PlayAnimationOnAllMines(int centerX, int centerY)
    {
        int mineCount = 0;
        int ringCount = 0;
        int visited = 0;
        int level = 0;

        while (visited < TileCount)
        {
            if (level == 0)
            {
                _animationMines[mineCount++] = Index(centerX, centerY);
                _animationRings[ringCount++] = 1;
                visited++;
            }
            else
            {
                int minesInRing = 0;
                for (int x = centerX - level; x <= centerX + level; x++)
                {
                    CheckAnimationTile(
                        x,
                        centerY - level,
                        ref mineCount,
                        ref visited,
                        ref minesInRing);
                }
                for (int y = centerY - level + 1; y <= centerY + level; y++)
                {
                    CheckAnimationTile(
                        centerX + level,
                        y,
                        ref mineCount,
                        ref visited,
                        ref minesInRing);
                }
                for (int x = centerX - level; x < centerX + level; x++)
                {
                    CheckAnimationTile(
                        x,
                        centerY + level,
                        ref mineCount,
                        ref visited,
                        ref minesInRing);
                }
                for (int y = centerY - level + 1; y < centerY + level; y++)
                {
                    CheckAnimationTile(
                        centerX - level,
                        y,
                        ref mineCount,
                        ref visited,
                        ref minesInRing);
                }
                if (minesInRing > 0)
                {
                    _animationRings[ringCount++] = minesInRing;
                }
            }
            level++;
        }

        _ui.PlayMineAnimations(
            _animationMines.AsSpan(0, mineCount),
            _animationRings.AsSpan(0, ringCount));
    }

    private void CheckAnimationTile(
        int x,
        int y,
        ref int mineCount,
        ref int visited,
        ref int minesInRing)
    {
        if (!InBounds(x, y))
        {
            return;
        }
        int index = Index(x, y);
        if (_mines[index])
        {
            _animationMines[mineCount++] = index;
            minesInRing++;
        }
        visited++;
    }

    private bool CheckIfWon()
    {
        int unrevealed = 0;
        for (int i = 0; i < TileCount; i++)
        {
            if (_mineStates[i] != MineState.Revealed)
            {
                unrevealed++;
            }
        }
        return unrevealed == MineCount;
    }

    private static MineState Cycle(MineState state) => state switch
    {
        MineState.Empty => MineState.Flag,
        MineState.Flag => MineState.Question,
        MineState.Question => MineState.Empty,
        _ => throw new InvalidOperationException("A revealed tile cannot be marked."),
    };

    private static int Index(int x, int y) => x * Height + y;

    private static int XFromIndex(int index) => index / Height;

    private static int YFromIndex(int index) => index % Height;

    private static bool InBounds(int x, int y) =>
        x >= 0 && x < Width && y >= 0 && y < Height;

    public void Dispose() => _ui.Dispose();
}

internal struct XorShift64
{
    private ulong _state;

    internal XorShift64(ulong seed)
    {
        _state = seed == 0 ? 1 : seed;
    }

    internal int Next(int bound)
    {
        ulong value = _state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        _state = value;
        return (int)(value % (uint)bound);
    }
}
