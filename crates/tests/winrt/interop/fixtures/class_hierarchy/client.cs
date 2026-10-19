using ClassHierarchy;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var compositor = new Compositor();

        var container = compositor.CreateContainerVisual(123);
        Check(container.Children == 123);
        Check(container.Compositor == compositor);

        var sprite = compositor.CreateSpriteVisual(456);
        Check(sprite.Brush == 456);
        Check(sprite.Children == 912);
        Check(sprite.Compositor == compositor);

        // CsWinRT creates the most derived class from the object's runtime class name.
        var visual = compositor.Echo(sprite);
        Check(ReferenceEquals(visual, sprite));
        Check(visual is SpriteVisual { Brush: 456 });
        Check(compositor.Echo(container) is not SpriteVisual);
    }
}
