using System;
using Windows.Foundation.Collections;
using Windows.Foundation.Numerics;
using Windows.UI;
using Windows.UI.Composition;
using WinRTTimeSpan = Windows.Foundation.TimeSpan;

namespace CSharpMinesweeper;

internal sealed class CompositionBoard : IDisposable
{
    private readonly Compositor _compositor;
    private readonly SpriteVisual _root;
    private readonly VisualCollection _parentChildren;
    private readonly VisualCollection _rootChildren;
    private readonly VisualGrid _gameBoard;
    private readonly CompositionAssets _assets;
    private Vector2 _parentSize;
    private bool _disposed;

    internal bool IsAnimationPlaying { get; private set; }

    internal TileCoordinate? CurrentSelectedTile => _gameBoard.CurrentSelection;

    internal CompositionBoard(
        Compositor compositor,
        ContainerVisual parentVisual,
        Vector2 parentSize,
        GridSize gridSize)
    {
        _parentSize = parentSize;
        _compositor = compositor;
        try
        {
            _root = _compositor.CreateSpriteVisual()!;
            _root.RelativeSizeAdjustment = new Vector2 { X = 1, Y = 1 };
            _root.BorderMode = CompositionBorderMode.Hard;
            using (CompositionBrush white = CreateBrush(_compositor, Colors.White))
            {
                _root.Brush = white;
            }

            _parentChildren = parentVisual.Children!;
            _parentChildren.InsertAtTop(_root);
            _rootChildren = _root.Children!;

            Vector2 tileSize = new() { X = 25, Y = 25 };
            _gameBoard = new VisualGrid(
                _compositor,
                gridSize,
                tileSize,
                new Vector2 { X = 2.5f, Y = 2.5f });
            _gameBoard.Root.RelativeOffsetAdjustment = new Vector3
            {
                X = 0.5f,
                Y = 0.5f,
            };
            _gameBoard.Root.AnchorPoint = new Vector2 { X = 0.5f, Y = 0.5f };
            _rootChildren.InsertAtTop(_gameBoard.Root);
            _rootChildren.InsertAtTop(_gameBoard.SelectionVisual);

            _assets = new CompositionAssets(_compositor, tileSize);
        }
        catch
        {
            Dispose();
            throw;
        }
    }

    internal TileCoordinate? HitTest(Vector2 point)
    {
        float scale = ComputeScaleFactor(_parentSize);
        if (scale <= 0)
        {
            return null;
        }
        Vector2 boardSize = _gameBoard.Size;
        Vector2 realBoardSize = Multiply(boardSize, scale);
        Vector2 realOffset = Divide(Subtract(_parentSize, realBoardSize), 2);
        return _gameBoard.HitTest(Divide(Subtract(point, realOffset), scale));
    }

    internal void Resize(Vector2 newSize)
    {
        _parentSize = newSize;
        float scale = ComputeScaleFactor(newSize);
        _gameBoard.Root.Scale = new Vector3 { X = scale, Y = scale, Z = 1 };
    }

    internal void SelectTile(TileCoordinate? tile) => _gameBoard.SelectTile(tile);

    internal void Reset(GridSize gridSize)
    {
        _gameBoard.Reset(gridSize);
        for (int i = 0; i < _gameBoard.TileCount; i++)
        {
            _gameBoard.GetTile(i).Brush = _assets.BrushForState(MineState.Empty);
        }
        Resize(_parentSize);
        IsAnimationPlaying = false;
    }

    internal void UpdateTileWithState(TileCoordinate tile, MineState state) =>
        _gameBoard.GetTile(tile).Brush = _assets.BrushForState(state);

    internal void UpdateTileAsMine(TileCoordinate tile) =>
        _gameBoard.GetTile(tile).Brush = _assets.MineBrush;

    internal void UpdateTileWithMineCount(TileCoordinate tile, int count)
    {
        SpriteVisual visual = _gameBoard.GetTile(tile);
        visual.Brush = _assets.BrushForCount(count);
        if (count == 0)
        {
            return;
        }

        using ShapeVisual shapeVisual = _compositor.CreateShapeVisual()!;
        shapeVisual.RelativeSizeAdjustment = new Vector2 { X = 1, Y = 1 };
        shapeVisual.BorderMode = CompositionBorderMode.Soft;
        using (CompositionShapeCollection shapes = shapeVisual.Shapes!)
        using (IVector<CompositionShape?> vector =
            shapes.As<IVector<CompositionShape?>>())
        {
            vector.Append(_assets.ShapeForCount(count));
        }
        using VisualCollection children = visual.Children!;
        children.InsertAtTop(shapeVisual);
    }

    internal void PlayMineAnimations(
        ReadOnlySpan<int> mineIndices,
        ReadOnlySpan<int> minesPerRing)
    {
        using CompositionScopedBatch batch =
            _compositor.CreateScopedBatch(CompositionBatchTypes.Animation)!;
        long delay = 0;
        int ring = 0;
        int minesOnCurrentRing = 0;

        for (int i = 0; i < mineIndices.Length; i++)
        {
            PlayMineAnimation(mineIndices[i], delay);
            minesOnCurrentRing++;
            if (minesOnCurrentRing == minesPerRing[ring])
            {
                minesOnCurrentRing = 0;
                ring++;
                delay += 100;
            }
        }

        batch.End();
        IsAnimationPlaying = true;
    }

    private void PlayMineAnimation(int index, long delayMilliseconds)
    {
        SpriteVisual visual = _gameBoard.GetTile(index);
        _gameBoard.Promote(index);
        visual.Brush = _assets.MineBrush;

        using Vector3KeyFrameAnimation animation =
            _compositor.CreateVector3KeyFrameAnimation()!;
        animation.InsertKeyFrame(0, new Vector3 { X = 1, Y = 1, Z = 1 });
        animation.InsertKeyFrame(0.7f, new Vector3 { X = 2, Y = 2, Z = 1 });
        animation.InsertKeyFrame(1, new Vector3 { X = 1, Y = 1, Z = 1 });
        animation.Duration = Duration(600);
        animation.DelayTime = Duration(delayMilliseconds);
        animation.IterationCount = 1;
        visual.StartAnimation("Scale", animation);
    }

    private float ComputeScaleFactor(Vector2 windowSize)
    {
        Vector2 board = Add(
            _gameBoard.Size,
            new Vector2 { X = 100, Y = 100 });
        if (windowSize.X <= 0 || windowSize.Y <= 0 || board.X <= 0 || board.Y <= 0)
        {
            return 0;
        }
        float windowRatio = windowSize.X / windowSize.Y;
        float boardRatio = board.X / board.Y;
        return windowRatio > boardRatio
            ? windowSize.Y / board.Y
            : windowSize.X / board.X;
    }

    private static WinRTTimeSpan Duration(long milliseconds) => new()
    {
        Duration = checked(milliseconds * 10_000),
    };

    private static CompositionBrush CreateBrush(Compositor compositor, Color color)
    {
        using CompositionColorBrush concrete = compositor.CreateColorBrush(color)!;
        return concrete.As<CompositionBrush>();
    }

    private static Vector2 Add(Vector2 left, Vector2 right) => new()
    {
        X = left.X + right.X,
        Y = left.Y + right.Y,
    };

    private static Vector2 Subtract(Vector2 left, Vector2 right) => new()
    {
        X = left.X - right.X,
        Y = left.Y - right.Y,
    };

    private static Vector2 Multiply(Vector2 value, float scale) => new()
    {
        X = value.X * scale,
        Y = value.Y * scale,
    };

    private static Vector2 Divide(Vector2 value, float divisor) => new()
    {
        X = value.X / divisor,
        Y = value.Y / divisor,
    };

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;

        if (_parentChildren is not null && _root is not null)
        {
            _parentChildren.Remove(_root);
        }
        _rootChildren?.RemoveAll();
        _gameBoard?.Dispose();
        _assets?.Dispose();
        _rootChildren?.Dispose();
        _parentChildren?.Dispose();
        _root?.Dispose();
    }
}

internal sealed class VisualGrid : IDisposable
{
    private readonly Compositor _compositor;
    private readonly ContainerVisual _root;
    private readonly VisualCollection _children;
    private readonly SpriteVisual _selectionVisual;
    private SpriteVisual[] _tiles;
    private Visual[] _tileVisuals;
    private GridSize _gridSize;
    private readonly Vector2 _tileSize;
    private readonly Vector2 _margin;
    private int _tileCount;
    private bool _disposed;

    internal ContainerVisual Root => _root;

    internal SpriteVisual SelectionVisual => _selectionVisual;

    internal int TileCount => _tileCount;

    internal Vector2 Size => _root.Size;

    internal TileCoordinate? CurrentSelection { get; private set; }

    internal VisualGrid(
        Compositor compositor,
        GridSize gridSize,
        Vector2 tileSize,
        Vector2 margin)
    {
        _compositor = compositor;
        _gridSize = gridSize;
        _tileSize = tileSize;
        _margin = margin;
        _tiles = new SpriteVisual[gridSize.Width * gridSize.Height];
        _tileVisuals = new Visual[_tiles.Length];
        try
        {
            _root = compositor.CreateContainerVisual()!;
            _children = _root.Children!;
            _selectionVisual = compositor.CreateSpriteVisual()!;
            using CompositionColorBrush red = compositor.CreateColorBrush(Colors.Red)!;
            using CompositionBrush redBrush = red.As<CompositionBrush>();
            using CompositionNineGridBrush nineGrid = compositor.CreateNineGridBrush()!;
            nineGrid.SetInsets(margin.X, margin.Y, margin.X, margin.Y);
            nineGrid.IsCenterHollow = true;
            nineGrid.Source = redBrush;
            using (CompositionBrush selectionBrush = nineGrid.As<CompositionBrush>())
            {
                _selectionVisual.Brush = selectionBrush;
            }
            _selectionVisual.Offset = new Vector3
            {
                X = -margin.X,
                Y = -margin.Y,
            };
            _selectionVisual.Size = new Vector2
            {
                X = tileSize.X + margin.X * 2,
                Y = tileSize.Y + margin.Y * 2,
            };
            _selectionVisual.IsVisible = false;

            Reset(gridSize);
        }
        catch
        {
            Dispose();
            throw;
        }
    }

    internal void Reset(GridSize gridSize)
    {
        _selectionVisual.ParentForTransform = null;
        _selectionVisual.IsVisible = false;
        CurrentSelection = null;
        _children.RemoveAll();
        DisposeTiles();

        _gridSize = gridSize;
        int count = gridSize.Width * gridSize.Height;
        if (_tiles.Length != count)
        {
            _tiles = new SpriteVisual[count];
            _tileVisuals = new Visual[count];
        }
        _tileCount = count;
        _root.Size = new Vector2
        {
            X = (_tileSize.X + _margin.X) * gridSize.Width,
            Y = (_tileSize.Y + _margin.Y) * gridSize.Height,
        };

        int index = 0;
        for (int x = 0; x < gridSize.Width; x++)
        {
            for (int y = 0; y < gridSize.Height; y++)
            {
                SpriteVisual tile = _compositor.CreateSpriteVisual()!;
                tile.Size = _tileSize;
                tile.CenterPoint = new Vector3
                {
                    X = _tileSize.X / 2,
                    Y = _tileSize.Y / 2,
                };
                tile.Offset = new Vector3
                {
                    X = _margin.X / 2 + (_tileSize.X + _margin.X) * x,
                    Y = _margin.Y / 2 + (_tileSize.Y + _margin.Y) * y,
                };
                Visual tileVisual = tile.As<Visual>();
                _children.InsertAtTop(tileVisual);
                _tiles[index] = tile;
                _tileVisuals[index] = tileVisual;
                index++;
            }
        }
    }

    internal TileCoordinate? HitTest(Vector2 point)
    {
        int x = (int)(point.X / (_tileSize.X + _margin.X));
        int y = (int)(point.Y / (_tileSize.Y + _margin.Y));
        return InBounds(x, y) ? new TileCoordinate(x, y) : null;
    }

    internal void SelectTile(TileCoordinate? tile)
    {
        if (CurrentSelection == tile)
        {
            return;
        }
        CurrentSelection = tile;
        if (tile is TileCoordinate coordinate)
        {
            _selectionVisual.ParentForTransform =
                _tileVisuals[Index(coordinate.X, coordinate.Y)];
            _selectionVisual.IsVisible = true;
        }
        else
        {
            _selectionVisual.ParentForTransform = null;
            _selectionVisual.IsVisible = false;
        }
    }

    internal SpriteVisual GetTile(TileCoordinate tile) => GetTile(Index(tile.X, tile.Y));

    internal SpriteVisual GetTile(int index) => _tiles[index];

    internal void Promote(int index)
    {
        Visual visual = _tileVisuals[index];
        _children.Remove(visual);
        _children.InsertAtTop(visual);
    }

    private int Index(int x, int y) => x * _gridSize.Height + y;

    private bool InBounds(int x, int y) =>
        x >= 0 && x < _gridSize.Width && y >= 0 && y < _gridSize.Height;

    private void DisposeTiles()
    {
        for (int i = 0; i < _tileCount; i++)
        {
            _tileVisuals[i]?.Dispose();
            _tiles[i]?.Dispose();
        }
        _tileCount = 0;
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;

        if (_selectionVisual is not null)
        {
            _selectionVisual.ParentForTransform = null;
        }
        _children?.RemoveAll();
        DisposeTiles();
        _selectionVisual?.Dispose();
        _children?.Dispose();
        _root?.Dispose();
    }
}

internal sealed class CompositionAssets : IDisposable
{
    private readonly CompositionBrush _mineBrush;
    private readonly CompositionBrush[] _stateBrushes = new CompositionBrush[3];
    private readonly CompositionBrush[] _countBrushes = new CompositionBrush[9];
    private readonly CompositionContainerShape?[] _countShapes =
        new CompositionContainerShape?[9];
    private bool _disposed;

    internal CompositionBrush MineBrush => _mineBrush;

    internal CompositionAssets(Compositor compositor, Vector2 tileSize)
    {
        try
        {
            _mineBrush = CreateBrush(compositor, Colors.Red);
            _stateBrushes[(int)MineState.Empty] = CreateBrush(compositor, Colors.Blue);
            _stateBrushes[(int)MineState.Flag] = CreateBrush(compositor, Colors.Orange);
            _stateBrushes[(int)MineState.Question] =
                CreateBrush(compositor, Colors.LimeGreen);

            _countBrushes[0] = CreateBrush(compositor, Colors.WhiteSmoke);
            _countBrushes[1] = CreateBrush(compositor, Colors.LightBlue);
            _countBrushes[2] = CreateBrush(compositor, Colors.LightGreen);
            _countBrushes[3] = CreateBrush(compositor, Colors.LightSalmon);
            _countBrushes[4] = CreateBrush(compositor, Colors.LightSteelBlue);
            _countBrushes[5] = CreateBrush(compositor, Colors.MediumPurple);
            _countBrushes[6] = CreateBrush(compositor, Colors.LightCyan);
            _countBrushes[7] = CreateBrush(compositor, Colors.Maroon);
            _countBrushes[8] = CreateBrush(compositor, Colors.DarkSeaGreen);

            using CompositionEllipseGeometry circle = compositor.CreateEllipseGeometry()!;
            circle.Radius = new Vector2
            {
                X = tileSize.X / 12,
                Y = tileSize.Y / 12,
            };
            using CompositionBrush dotBrush = CreateBrush(compositor, Colors.Black);
            for (int count = 1; count <= 8; count++)
            {
                CompositionContainerShape? container = null;
                try
                {
                    container = compositor.CreateContainerShape()!;
                    using CompositionShapeCollection shapes = container.Shapes!;
                    using IVector<CompositionShape?> vector =
                        shapes.As<IVector<CompositionShape?>>();
                    AddDots(compositor, circle, dotBrush, vector, tileSize, count);
                    _countShapes[count] = container;
                    container = null;
                }
                finally
                {
                    container?.Dispose();
                }
            }
        }
        catch
        {
            Dispose();
            throw;
        }
    }

    internal CompositionBrush BrushForState(MineState state) => _stateBrushes[(int)state];

    internal CompositionBrush BrushForCount(int count) => _countBrushes[count];

    internal CompositionContainerShape ShapeForCount(int count) => _countShapes[count]!;

    private static void AddDots(
        Compositor compositor,
        CompositionEllipseGeometry geometry,
        CompositionBrush brush,
        IVector<CompositionShape?> shapes,
        Vector2 size,
        int count)
    {
        float thirdX = size.X / 3;
        float thirdY = size.Y / 3;
        float fourthX = size.X / 4;
        float fourthY = size.Y / 4;
        float halfX = size.X / 2;
        float halfY = size.Y / 2;

        switch (count)
        {
            case 1:
                AddDot(compositor, geometry, brush, shapes, halfX, halfY);
                break;
            case 2:
                AddDot(compositor, geometry, brush, shapes, thirdX, halfY);
                AddDot(compositor, geometry, brush, shapes, thirdX * 2, halfY);
                break;
            case 3:
                AddDot(compositor, geometry, brush, shapes, halfX, halfY);
                AddDot(compositor, geometry, brush, shapes, fourthX, fourthY * 3);
                AddDot(compositor, geometry, brush, shapes, fourthX * 3, fourthY);
                break;
            case 4:
                AddDot(compositor, geometry, brush, shapes, thirdX, thirdY);
                AddDot(compositor, geometry, brush, shapes, thirdX * 2, thirdY);
                AddDot(compositor, geometry, brush, shapes, thirdX, thirdY * 2);
                AddDot(compositor, geometry, brush, shapes, thirdX * 2, thirdY * 2);
                break;
            case 5:
                AddDot(compositor, geometry, brush, shapes, halfX, halfY);
                AddDot(compositor, geometry, brush, shapes, fourthX, fourthY * 3);
                AddDot(compositor, geometry, brush, shapes, fourthX * 3, fourthY);
                AddDot(compositor, geometry, brush, shapes, fourthX, fourthY);
                AddDot(
                    compositor,
                    geometry,
                    brush,
                    shapes,
                    fourthX * 3,
                    fourthY * 3);
                break;
            case 6:
                AddDot(compositor, geometry, brush, shapes, fourthX, fourthY * 2);
                AddDot(compositor, geometry, brush, shapes, fourthX, fourthY * 3);
                AddDot(compositor, geometry, brush, shapes, fourthX * 3, fourthY);
                AddDot(compositor, geometry, brush, shapes, fourthX, fourthY);
                AddDot(
                    compositor,
                    geometry,
                    brush,
                    shapes,
                    fourthX * 3,
                    fourthY * 3);
                AddDot(
                    compositor,
                    geometry,
                    brush,
                    shapes,
                    fourthX * 3,
                    fourthY * 2);
                break;
            case 7:
                AddDots(compositor, geometry, brush, shapes, size, 6);
                AddDot(compositor, geometry, brush, shapes, halfX, halfY);
                break;
            case 8:
                AddDots(compositor, geometry, brush, shapes, size, 6);
                AddDot(compositor, geometry, brush, shapes, halfX, thirdY);
                AddDot(compositor, geometry, brush, shapes, halfX, thirdY * 2);
                break;
        }
    }

    private static void AddDot(
        Compositor compositor,
        CompositionEllipseGeometry geometry,
        CompositionBrush brush,
        IVector<CompositionShape?> shapes,
        float x,
        float y)
    {
        using CompositionSpriteShape dot = compositor.CreateSpriteShape(geometry)!;
        dot.FillBrush = brush;
        dot.Offset = new Vector2 { X = x, Y = y };
        shapes.Append(dot);
    }

    private static CompositionBrush CreateBrush(Compositor compositor, Color color)
    {
        using CompositionColorBrush concrete = compositor.CreateColorBrush(color)!;
        return concrete.As<CompositionBrush>();
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;

        for (int i = 1; i < _countShapes.Length; i++)
        {
            _countShapes[i]?.Dispose();
        }
        for (int i = 0; i < _countBrushes.Length; i++)
        {
            _countBrushes[i]?.Dispose();
        }
        for (int i = 0; i < _stateBrushes.Length; i++)
        {
            _stateBrushes[i]?.Dispose();
        }
        _mineBrush?.Dispose();
    }
}

internal static class Colors
{
    internal static Color Red => Rgb(255, 0, 0);
    internal static Color White => Rgb(255, 255, 255);
    internal static Color Blue => Rgb(0, 0, 255);
    internal static Color Orange => Rgb(255, 165, 0);
    internal static Color LimeGreen => Rgb(50, 205, 50);
    internal static Color LightBlue => Rgb(173, 216, 230);
    internal static Color LightGreen => Rgb(144, 238, 144);
    internal static Color LightSalmon => Rgb(255, 160, 122);
    internal static Color LightSteelBlue => Rgb(176, 196, 222);
    internal static Color MediumPurple => Rgb(147, 112, 219);
    internal static Color LightCyan => Rgb(224, 255, 255);
    internal static Color Maroon => Rgb(128, 0, 0);
    internal static Color DarkSeaGreen => Rgb(143, 188, 143);
    internal static Color WhiteSmoke => Rgb(245, 245, 245);
    internal static Color Black => Rgb(0, 0, 0);

    private static Color Rgb(byte red, byte green, byte blue) => new()
    {
        A = 255,
        R = red,
        G = green,
        B = blue,
    };
}
