using System;

namespace CSharpTicTacToe;

internal enum Cell
{
    Empty,
    X,
    O,
}

internal enum Player
{
    X,
    O,
}

internal enum GameStatus
{
    Playing,
    XWon,
    OWon,
    Draw,
}

internal sealed class TicTacToeGame
{
    internal const int Size = 3;
    internal const int CellCount = Size * Size;

    private static readonly int[,] s_lines =
    {
        { 0, 1, 2 },
        { 3, 4, 5 },
        { 6, 7, 8 },
        { 0, 3, 6 },
        { 1, 4, 7 },
        { 2, 5, 8 },
        { 0, 4, 8 },
        { 2, 4, 6 },
    };

    private readonly Cell[] _cells = new Cell[CellCount];

    internal Player Turn { get; private set; }

    internal GameStatus Status { get; private set; }

    internal string StatusText => Status switch
    {
        GameStatus.Playing when Turn == Player.X => "Turn: X",
        GameStatus.Playing => "Turn: O",
        GameStatus.XWon => "X wins!",
        GameStatus.OWon => "O wins!",
        _ => "It's a draw",
    };

    internal TicTacToeGame() => Reset();

    internal Cell this[int index] => _cells[index];

    internal void Reset()
    {
        Array.Fill(_cells, Cell.Empty);
        Turn = Player.X;
        Status = GameStatus.Playing;
    }

    internal bool TryMove(int position)
    {
        if ((uint)position >= CellCount ||
            Status != GameStatus.Playing ||
            _cells[position] != Cell.Empty)
        {
            return false;
        }

        Cell cell = Turn == Player.X ? Cell.X : Cell.O;
        _cells[position] = cell;

        if (HasWinner(cell))
        {
            Status = cell == Cell.X ? GameStatus.XWon : GameStatus.OWon;
        }
        else if (IsFull())
        {
            Status = GameStatus.Draw;
        }
        else
        {
            Turn = Turn == Player.X ? Player.O : Player.X;
        }
        return true;
    }

    internal static string Label(Cell cell) => cell switch
    {
        Cell.X => "X",
        Cell.O => "O",
        _ => " ",
    };

    private bool HasWinner(Cell cell)
    {
        for (int line = 0; line < s_lines.GetLength(0); line++)
        {
            if (_cells[s_lines[line, 0]] == cell &&
                _cells[s_lines[line, 1]] == cell &&
                _cells[s_lines[line, 2]] == cell)
            {
                return true;
            }
        }
        return false;
    }

    private bool IsFull()
    {
        for (int index = 0; index < _cells.Length; index++)
        {
            if (_cells[index] == Cell.Empty)
            {
                return false;
            }
        }
        return true;
    }
}
