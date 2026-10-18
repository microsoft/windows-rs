using Test;

public static class Client
{
    public static Widget Create() => new Widget();

    public static int Use(Widget widget)
    {
        widget.Name = "name";
        return widget.Add(1, 2) + widget.Count + widget.Name.Length;
    }
}
