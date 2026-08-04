using System;
using Microsoft.UI.Dispatching;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Windows.Foundation.Collections;
using Windows.UI.Text;
using Win32Apis = Windows.Win32.Apis;

namespace CSharpTicTacToe;

internal sealed class TicTacToeApplication : IDisposable
{
    private readonly bool _smoke;
    private readonly TicTacToeGame _game = new();
    private readonly Button?[] _cellButtons = new Button?[TicTacToeGame.CellCount];
    private readonly TextBlock?[] _cellLabels = new TextBlock?[TicTacToeGame.CellCount];
    private readonly Control?[] _cellControls = new Control?[TicTacToeGame.CellCount];
    private readonly RoutedEventHandler?[] _cellHandlers =
        new RoutedEventHandler?[TicTacToeGame.CellCount];
    private readonly WindowsCsharp.EventRevoker?[] _cellRevokers =
        new WindowsCsharp.EventRevoker?[TicTacToeGame.CellCount];

    private Window? _window;
    private StackPanel? _root;
    private Grid? _board;
    private TextBlock? _status;
    private Button? _newGameButton;
    private RoutedEventHandler? _newGameHandler;
    private WindowsCsharp.EventRevoker? _newGameRevoker;
    private DispatcherQueue? _dispatcher;
    private DispatcherQueueHandler? _launchHandler;
    private DispatcherQueueHandler? _smokeHandler;
    private Exception? _callbackError;
    private bool _disposed;
    private bool _closing;

    internal TicTacToeApplication(bool smoke)
    {
        _smoke = smoke;
        try
        {
            _dispatcher = DispatcherQueue.GetForCurrentThread() ??
                throw new InvalidOperationException(
                    "DispatcherQueue.GetForCurrentThread returned null.");
            _launchHandler = DispatcherQueueHandler.Create(Launch);
            if (!_dispatcher.TryEnqueue(_launchHandler))
            {
                throw new InvalidOperationException("DispatcherQueue.TryEnqueue failed.");
            }
        }
        catch
        {
            Dispose();
            throw;
        }
    }

    private void Launch()
    {
        try
        {
            BuildWindow();
            _window!.Activate();
            if (_smoke)
            {
                QueueSmoke();
            }
        }
        catch (Exception error)
        {
            _callbackError = error;
            Win32Apis.PostQuitMessage(error.HResult);
        }
    }

    private void BuildWindow()
    {
        _window = new Window
        {
            Title = "windows-csharp - tictactoe",
        };

        _root = new StackPanel
        {
            Spacing = 12,
        };
        using (FrameworkElement rootElement = _root.As<FrameworkElement>())
        {
            rootElement.Margin = UniformThickness(16);
            rootElement.HorizontalAlignment = HorizontalAlignment.Center;
        }

        using StackPanel header = BuildHeader();
        _board = BuildBoard();
        using (UIElementCollection children = Require(_root.Children, "StackPanel.Children"))
        using (IVector<UIElement?> vector = children.As<IVector<UIElement?>>())
        {
            vector.Append(header);
            vector.Append(_board);
        }

        using UIElement content = _root.As<UIElement>();
        _window.Content = content;
        UpdateUi();
    }

    private StackPanel BuildHeader()
    {
        StackPanel header = new()
        {
            Spacing = 8,
        };
        try
        {
            using (FrameworkElement headerElement = header.As<FrameworkElement>())
            {
                headerElement.Margin = new Thickness
                {
                    Top = 12,
                    Bottom = 4,
                };
                headerElement.HorizontalAlignment = HorizontalAlignment.Center;
            }

            _status = new TextBlock
            {
                FontSize = 24,
                FontWeight = new FontWeight { Weight = 700 },
            };
            using (FrameworkElement statusElement = _status.As<FrameworkElement>())
            {
                statusElement.HorizontalAlignment = HorizontalAlignment.Center;
            }

            _newGameButton = new Button();
            using TextBlock label = new()
            {
                Text = "New Game",
            };
            _newGameButton.Content = label;
            using (FrameworkElement buttonElement =
                _newGameButton.As<FrameworkElement>())
            {
                buttonElement.HorizontalAlignment = HorizontalAlignment.Center;
            }
            _newGameHandler = RoutedEventHandler.Create(
                (_, _) => HandleResetClick());
            using (ButtonBase buttonBase = _newGameButton.As<ButtonBase>())
            {
                _newGameRevoker = buttonBase.Click(_newGameHandler);
            }

            using UIElementCollection children =
                Require(header.Children, "header StackPanel.Children");
            using IVector<UIElement?> vector = children.As<IVector<UIElement?>>();
            vector.Append(_status);
            vector.Append(_newGameButton);
            return header;
        }
        catch
        {
            header.Dispose();
            throw;
        }
    }

    private Grid BuildBoard()
    {
        Grid board = new()
        {
            RowSpacing = 4,
            ColumnSpacing = 4,
        };
        try
        {
            using (FrameworkElement boardElement = board.As<FrameworkElement>())
            {
                boardElement.Width = 360;
                boardElement.Height = 360;
                boardElement.HorizontalAlignment = HorizontalAlignment.Center;
            }

            AddDefinitions(board);
            using UIElementCollection children =
                Require(board.Children, "Grid.Children");
            using IVector<UIElement?> vector = children.As<IVector<UIElement?>>();
            for (int position = 0; position < TicTacToeGame.CellCount; position++)
            {
                Button button = new();
                _cellButtons[position] = button;

                TextBlock label = new()
                {
                    Text = " ",
                    FontSize = 36,
                    FontWeight = new FontWeight { Weight = 600 },
                };
                _cellLabels[position] = label;
                button.Content = label;

                using (FrameworkElement element = button.As<FrameworkElement>())
                {
                    element.HorizontalAlignment = HorizontalAlignment.Stretch;
                    element.VerticalAlignment = VerticalAlignment.Stretch;
                }
                Grid.SetRow(button, position / TicTacToeGame.Size);
                Grid.SetColumn(button, position % TicTacToeGame.Size);

                Control control = button.As<Control>();
                _cellControls[position] = control;

                int capturedPosition = position;
                RoutedEventHandler handler = RoutedEventHandler.Create(
                    (_, _) => HandleCellClick(capturedPosition));
                _cellHandlers[position] = handler;
                using (ButtonBase buttonBase = button.As<ButtonBase>())
                {
                    _cellRevokers[position] = buttonBase.Click(handler);
                }

                vector.Append(button);
            }
            return board;
        }
        catch
        {
            board.Dispose();
            throw;
        }
    }

    private static void AddDefinitions(Grid board)
    {
        GridLength star = new()
        {
            Value = 1,
            GridUnitType = GridUnitType.Star,
        };

        using RowDefinitionCollection rowCollection =
            Require(board.RowDefinitions, "Grid.RowDefinitions");
        using IVector<RowDefinition?> rows =
            rowCollection.As<IVector<RowDefinition?>>();
        for (int index = 0; index < TicTacToeGame.Size; index++)
        {
            using RowDefinition row = new()
            {
                Height = star,
            };
            rows.Append(row);
        }

        using ColumnDefinitionCollection columnCollection =
            Require(board.ColumnDefinitions, "Grid.ColumnDefinitions");
        using IVector<ColumnDefinition?> columns =
            columnCollection.As<IVector<ColumnDefinition?>>();
        for (int index = 0; index < TicTacToeGame.Size; index++)
        {
            using ColumnDefinition column = new()
            {
                Width = star,
            };
            columns.Append(column);
        }
    }

    private void QueueSmoke()
    {
        _smokeHandler = DispatcherQueueHandler.Create(RunSmoke);
        if (!_dispatcher!.TryEnqueue(_smokeHandler))
        {
            throw new InvalidOperationException("DispatcherQueue.TryEnqueue failed.");
        }
    }

    private void HandleCellClick(int position)
    {
        try
        {
            Play(position);
        }
        catch (Exception error)
        {
            FailCallback(error);
        }
    }

    private void HandleResetClick()
    {
        try
        {
            Reset();
        }
        catch (Exception error)
        {
            FailCallback(error);
        }
    }

    private void Play(int position)
    {
        if (_game.TryMove(position))
        {
            _cellLabels[position]!.Text = TicTacToeGame.Label(_game[position]);
            _cellControls[position]!.IsEnabled = false;
            _status!.Text = _game.StatusText;
            if (_game.Status != GameStatus.Playing)
            {
                for (int index = 0; index < TicTacToeGame.CellCount; index++)
                {
                    _cellControls[index]!.IsEnabled = false;
                }
            }
        }
    }

    private void Reset()
    {
        _game.Reset();
        UpdateUi();
    }

    private void UpdateUi()
    {
        _status!.Text = _game.StatusText;
        bool playing = _game.Status == GameStatus.Playing;
        for (int index = 0; index < TicTacToeGame.CellCount; index++)
        {
            _cellLabels[index]!.Text = TicTacToeGame.Label(_game[index]);
            _cellControls[index]!.IsEnabled =
                playing && _game[index] == Cell.Empty;
        }
    }

    private void RunSmoke()
    {
        try
        {
            AssertSmoke(_game.StatusText == "Turn: X", "initial status");
            InvokeCell(0);
            InvokeCell(3);
            InvokeCell(1);
            InvokeCell(4);
            InvokeCell(2);
            AssertSmoke(_game.Status == GameStatus.XWon, "X win");
            AssertSmoke(_status!.Text == "X wins!", "win status text");
            for (int index = 0; index < TicTacToeGame.CellCount; index++)
            {
                AssertSmoke(!_cellControls[index]!.IsEnabled, "completed cell disabled");
            }

            InvokeReset();
            AssertSmoke(_game.Status == GameStatus.Playing, "reset status");
            AssertSmoke(_game.Turn == Player.X, "reset turn");
            for (int index = 0; index < TicTacToeGame.CellCount; index++)
            {
                AssertSmoke(_cellControls[index]!.IsEnabled, "reset cell enabled");
            }

            ReadOnlySpan<int> drawMoves = [0, 1, 2, 4, 3, 5, 7, 6, 8];
            foreach (int position in drawMoves)
            {
                InvokeCell(position);
            }
            AssertSmoke(_game.Status == GameStatus.Draw, "draw");
            InvokeReset();
        }
        catch (Exception error)
        {
            _callbackError = error;
        }
        finally
        {
            Close();
        }
    }

    private void InvokeCell(int position)
    {
        _cellHandlers[position]!.Invoke(_cellButtons[position], null);
        if (_callbackError is not null)
        {
            throw _callbackError;
        }
    }

    private void InvokeReset()
    {
        _newGameHandler!.Invoke(_newGameButton, null);
        if (_callbackError is not null)
        {
            throw _callbackError;
        }
    }

    private static void AssertSmoke(bool condition, string operation)
    {
        if (!condition)
        {
            throw new InvalidOperationException(
                $"Tic-Tac-Toe smoke assertion failed: {operation}.");
        }
    }

    private void FailCallback(Exception error)
    {
        _callbackError ??= error;
        Close();
    }

    private void Close()
    {
        if (_closing)
        {
            return;
        }
        _closing = true;
        try
        {
            _window?.Close();
        }
        catch (Exception error)
        {
            _callbackError ??= error;
            Win32Apis.PostQuitMessage(error.HResult);
        }
    }

    internal void ThrowIfCallbackFailed()
    {
        if (_callbackError is not null)
        {
            throw new InvalidOperationException(
                "A WinUI callback failed.",
                _callbackError);
        }
    }

    private static Thickness UniformThickness(double value) => new()
    {
        Left = value,
        Top = value,
        Right = value,
        Bottom = value,
    };

    private static T Require<T>(T? value, string name) where T : class =>
        value ?? throw new InvalidOperationException($"{name} returned null.");

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;
        _smokeHandler?.Dispose();
        _launchHandler?.Dispose();
        _launchHandler?.Dispose();

        for (int index = _cellRevokers.Length - 1; index >= 0; index--)
        {
            _cellRevokers[index]?.Dispose();
        }
        _newGameRevoker?.Dispose();

        for (int index = _cellHandlers.Length - 1; index >= 0; index--)
        {
            _cellHandlers[index]?.Dispose();
            _cellControls[index]?.Dispose();
            _cellLabels[index]?.Dispose();
            _cellButtons[index]?.Dispose();
        }

        _newGameHandler?.Dispose();
        _newGameButton?.Dispose();
        _status?.Dispose();
        _board?.Dispose();
        _root?.Dispose();
        _window?.Dispose();
        _dispatcher?.Dispose();
    }
}
