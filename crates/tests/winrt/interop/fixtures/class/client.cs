using Class;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.Add(1, 2) == 3);
        Check(widget.Name == "");
        Check(widget.Count == 0);

        widget.Name = "name";
        Check(widget.Name == "name");
        Check(widget.Count == 1);
    }
}
