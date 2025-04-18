mod Windows {
    mod Foundation {
        mod Diagnostics {
        }
        mod Collections {
        }
        mod Numerics {
            struct Quaternion {
                X: f32,
                Y: f32,
                Z: f32,
                W: f32,
            }
            struct Plane {
                Normal: Windows::Foundation::Numerics::Vector3,
                D: f32,
            }
            struct Vector3 {
                X: f32,
                Y: f32,
                Z: f32,
            }
            struct Rational {
                Numerator: u32,
                Denominator: u32,
            }
            struct Matrix4x4 {
                M11: f32,
                M12: f32,
                M13: f32,
                M14: f32,
                M21: f32,
                M22: f32,
                M23: f32,
                M24: f32,
                M31: f32,
                M32: f32,
                M33: f32,
                M34: f32,
                M41: f32,
                M42: f32,
                M43: f32,
                M44: f32,
            }
            struct Vector4 {
                X: f32,
                Y: f32,
                Z: f32,
                W: f32,
            }
            struct Matrix3x2 {
                M11: f32,
                M12: f32,
                M21: f32,
                M22: f32,
                M31: f32,
                M32: f32,
            }
            struct Vector2 {
                X: f32,
                Y: f32,
            }
        }
        struct Size {
            Width: f32,
            Height: f32,
        }
        struct Rect {
            X: f32,
            Y: f32,
            Width: f32,
            Height: f32,
        }
        struct UniversalApiContract {
        }
        struct FoundationContract {
        }
        struct Point {
            X: f32,
            Y: f32,
        }
        struct TimeSpan {
            Duration: i64,
        }
        struct DateTime {
            UniversalTime: i64,
        }
        struct HResult {
            Value: i32,
        }
        struct EventRegistrationToken {
            Value: i64,
        }
        mod Metadata {
        }
    }
}
