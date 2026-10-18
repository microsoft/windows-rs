using System;

try
{
    Client.Test();
    return 0;
}
catch (Exception e)
{
    Console.WriteLine(e);
    return 1;
}
