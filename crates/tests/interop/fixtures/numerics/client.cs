using System.Numerics;
using Numerics;
using static Harness;

public static class Client
{
    public static void Test()
    {
        var widget = new Widget();
        Check(widget.NegateVector2(new Vector2(1.25f, -2.5f)) == new Vector2(-1.25f, 2.5f));
        Check(widget.NegateVector3(new Vector3(1.25f, -2.5f, 3.75f)) == new Vector3(-1.25f, 2.5f, -3.75f));
        Check(widget.NegateVector4(new Vector4(1.25f, -2.5f, 3.75f, -4.5f)) == new Vector4(-1.25f, 2.5f, -3.75f, 4.5f));
        Check(widget.NegateMatrix3x2(new Matrix3x2(1, -2, 3, -4, 5, -6)) == new Matrix3x2(-1, 2, -3, 4, -5, 6));
        Check(widget.NegateMatrix4x4(new Matrix4x4(
            1, -2, 3, -4,
            5, -6, 7, -8,
            9, -10, 11, -12,
            13, -14, 15, -16)) == new Matrix4x4(
            -1, 2, -3, 4,
            -5, 6, -7, 8,
            -9, 10, -11, 12,
            -13, 14, -15, 16));
        Check(widget.NegateQuaternion(new Quaternion(1.25f, -2.5f, 3.75f, -4.5f)) == new Quaternion(-1.25f, 2.5f, -3.75f, 4.5f));
        Check(widget.NegatePlane(new Plane(new Vector3(1.25f, -2.5f, 3.75f), -4.5f)) == new Plane(new Vector3(-1.25f, 2.5f, -3.75f), 4.5f));
    }
}
