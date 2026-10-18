using Test;

public static class Client
{
    public static void Use(Widget widget)
    {
        ChangedHandler handler = (Widget sender, int value) => { };
        widget.Changed += handler;
        widget.Changed -= handler;
    }
}
