using Test;
using Windows.Foundation;

public static class Client
{
    public static void Use(Widget widget)
    {
        TypedEventHandler<Widget, int> handler = (sender, value) => { };
        widget.Changed += handler;
        widget.Changed -= handler;
    }
}
