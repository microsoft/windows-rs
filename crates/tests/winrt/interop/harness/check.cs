using System;
using System.Runtime.CompilerServices;

public static class Harness
{
    // Fails the fixture test with the client's file, line, and expression when `condition` is false.
    public static void Check(
        bool condition,
        [CallerArgumentExpression(nameof(condition))] string expression = "",
        [CallerFilePath] string file = "",
        [CallerLineNumber] int line = 0)
    {
        if (!condition)
        {
            throw new Exception($"{file}({line}): check failed: {expression}");
        }
    }
}
