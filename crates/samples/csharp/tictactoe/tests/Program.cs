using System;
using System.Collections.Generic;
using CSharpTicTacToe;

internal static class Program
{
    private static int Main()
    {
        NewGame();
        MoveAndAlternation();
        InvalidMoves();
        XRowWin();
        OColumnWin();
        DiagonalWin();
        Draw();
        Reset();
        Console.WriteLine("TICTACTOE_GAME_TESTS ok");
        return 0;
    }

    private static void NewGame()
    {
        TicTacToeGame game = new();
        Equal(Player.X, game.Turn, "new turn");
        Equal(GameStatus.Playing, game.Status, "new status");
        Equal("Turn: X", game.StatusText, "new status text");
        for (int index = 0; index < TicTacToeGame.CellCount; index++)
        {
            Equal(Cell.Empty, game[index], "new cell");
        }
    }

    private static void MoveAndAlternation()
    {
        TicTacToeGame game = new();
        True(game.TryMove(0), "first move");
        Equal(Cell.X, game[0], "first mark");
        Equal(Player.O, game.Turn, "second turn");
        True(game.TryMove(4), "second move");
        Equal(Cell.O, game[4], "second mark");
        Equal(Player.X, game.Turn, "third turn");
    }

    private static void InvalidMoves()
    {
        TicTacToeGame game = new();
        True(!game.TryMove(-1), "negative move");
        True(!game.TryMove(TicTacToeGame.CellCount), "large move");
        True(game.TryMove(0), "occupied setup");
        True(!game.TryMove(0), "occupied move");
    }

    private static void XRowWin()
    {
        TicTacToeGame game = Play(0, 3, 1, 4, 2);
        Equal(GameStatus.XWon, game.Status, "X row win");
        Equal("X wins!", game.StatusText, "X row status");
        True(!game.TryMove(5), "move after win");
    }

    private static void OColumnWin()
    {
        TicTacToeGame game = Play(0, 2, 1, 5, 4, 8);
        Equal(GameStatus.OWon, game.Status, "O column win");
        Equal("O wins!", game.StatusText, "O column status");
    }

    private static void DiagonalWin()
    {
        TicTacToeGame game = Play(0, 1, 4, 2, 8);
        Equal(GameStatus.XWon, game.Status, "diagonal win");
    }

    private static void Draw()
    {
        TicTacToeGame game = Play(0, 1, 2, 4, 3, 5, 7, 6, 8);
        Equal(GameStatus.Draw, game.Status, "draw");
        Equal("It's a draw", game.StatusText, "draw status");
    }

    private static void Reset()
    {
        TicTacToeGame game = Play(0, 3, 1, 4, 2);
        game.Reset();
        Equal(Player.X, game.Turn, "reset turn");
        Equal(GameStatus.Playing, game.Status, "reset status");
        for (int index = 0; index < TicTacToeGame.CellCount; index++)
        {
            Equal(Cell.Empty, game[index], "reset cell");
        }
    }

    private static TicTacToeGame Play(params int[] moves)
    {
        TicTacToeGame game = new();
        foreach (int move in moves)
        {
            True(game.TryMove(move), $"move {move}");
        }
        return game;
    }

    private static void True(bool value, string operation)
    {
        if (!value)
        {
            throw new InvalidOperationException($"Failed: {operation}.");
        }
    }

    private static void Equal<T>(T expected, T actual, string operation)
    {
        if (!EqualityComparer<T>.Default.Equals(actual, expected))
        {
            throw new InvalidOperationException(
                $"Failed: {operation}; expected {expected}, actual {actual}.");
        }
    }
}
