//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <DirectXMath.h>


#ifdef _WINDOWS_NUMERICS_INTEROP_NAMESPACE_

// Define conversion operators between these C++ structs and a set of WinRT ABI types with matching layouts.
#define _DEFINE_WINDOWS_NUMERICS_INTEROP_(CppName, WinRTName)                                       \
                                                                                                    \
    CppName(_WINDOWS_NUMERICS_INTEROP_NAMESPACE_::WinRTName const& value)                           \
        : CppName(*reinterpret_cast<CppName const*>(&value))                                        \
    { }                                                                                             \
                                                                                                    \
    operator _WINDOWS_NUMERICS_INTEROP_NAMESPACE_::WinRTName() const                                \
    {                                                                                               \
        return *reinterpret_cast<_WINDOWS_NUMERICS_INTEROP_NAMESPACE_::WinRTName const*>(this);     \
    }

#else
#define _DEFINE_WINDOWS_NUMERICS_INTEROP_(CppType, WinRTType)
#endif


// Mark the numerics constructors as constexpr, except when using the C++/CX projection.
#ifdef _WINDOWS_NUMERICS_CX_PROJECTION_
    #define _WINDOWS_NUMERICS_CONSTEXPR_ inline
#else
    #define _WINDOWS_NUMERICS_CONSTEXPR_ constexpr
#endif


_WINDOWS_NUMERICS_BEGIN_NAMESPACE_
{
#ifndef _WINDOWS_NUMERICS_CX_PROJECTION_

    struct float2;
    struct float3;
    struct float4;
    struct float3x2;
    struct float4x4;
    struct plane;
    struct quaternion;


    struct float2
    {
        float x, y;

        // Constructors.
        float2() = default;
        _WINDOWS_NUMERICS_CONSTEXPR_ float2(float x, float y);
        _WINDOWS_NUMERICS_CONSTEXPR_ explicit float2(float value);

        // Conversion operators.
        _DEFINE_WINDOWS_NUMERICS_INTEROP_(float2, Vector2)

#ifdef _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_
        float2(_WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point const& value);
        float2(_WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size const& value);

        operator _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point() const;
        operator _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size() const;
#endif

        // Common values.
        _WINDOWS_NUMERICS_CONSTEXPR_ static float2 zero();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float2 one();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float2 unit_x();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float2 unit_y();
    };

#endif  // !_WINDOWS_NUMERICS_CX_PROJECTION_


    // Operators.
    float2 operator +(float2 const& value1, float2 const& value2);
    float2 operator -(float2 const& value1, float2 const& value2);
    float2 operator *(float2 const& value1, float2 const& value2);
    float2 operator *(float2 const& value1, float value2);
    float2 operator *(float value1, float2 const& value2);
    float2 operator /(float2 const& value1, float2 const& value2);
    float2 operator /(float2 const& value1, float value2);
    float2 operator -(float2 const& value);
    float2& operator +=(float2& value1, float2 const& value2);
    float2& operator -=(float2& value1, float2 const& value2);
    float2& operator *=(float2& value1, float2 const& value2);
    float2& operator *=(float2& value1, float value2);
    float2& operator /=(float2& value1, float2 const& value2);
    float2& operator /=(float2& value1, float value2);
    bool operator ==(float2 const& value1, float2 const& value2);
    bool operator !=(float2 const& value1, float2 const& value2);

    // Functions.
    float length(float2 const& value);
    float length_squared(float2 const& value);
    float distance(float2 const& value1, float2 const& value2);
    float distance_squared(float2 const& value1, float2 const& value2);
    float dot(float2 const& value1, float2 const& value2);
    float2 normalize(float2 const& value);
    float2 reflect(float2 const& vector, float2 const& normal);
    float2 (min)(float2 const& value1, float2 const& value2);
    float2 (max)(float2 const& value1, float2 const& value2);
    float2 clamp(float2 const& value1, float2 const& min, float2 const& max);
    float2 lerp(float2 const& value1, float2 const& value2, float amount);
    float2 transform(float2 const& position, float3x2 const& matrix);
    float2 transform(float2 const& position, float4x4 const& matrix);
    float2 transform_normal(float2 const& normal, float3x2 const& matrix);
    float2 transform_normal(float2 const& normal, float4x4 const& matrix);
    float2 transform(float2 const& value, quaternion const& rotation);


#ifndef _WINDOWS_NUMERICS_CX_PROJECTION_

    struct float3
    {
        float x, y, z;

        // Constructors.
        float3() = default;
        _WINDOWS_NUMERICS_CONSTEXPR_ float3(float x, float y, float z);
        _WINDOWS_NUMERICS_CONSTEXPR_ float3(float2 value, float z);
        _WINDOWS_NUMERICS_CONSTEXPR_ explicit float3(float value);

        _DEFINE_WINDOWS_NUMERICS_INTEROP_(float3, Vector3)

        // Common values.
        _WINDOWS_NUMERICS_CONSTEXPR_ static float3 zero();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float3 one();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float3 unit_x();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float3 unit_y();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float3 unit_z();
    };

#endif  // !_WINDOWS_NUMERICS_CX_PROJECTION_


    // Operators.
    float3 operator +(float3 const& value1, float3 const& value2);
    float3 operator -(float3 const& value1, float3 const& value2);
    float3 operator *(float3 const& value1, float3 const& value2);
    float3 operator *(float3 const& value1, float value2);
    float3 operator *(float value1, float3 const& value2);
    float3 operator /(float3 const& value1, float3 const& value2);
    float3 operator /(float3 const& value1, float value2);
    float3 operator -(float3 const& value);
    float3& operator +=(float3& value1, float3 const& value2);
    float3& operator -=(float3& value1, float3 const& value2);
    float3& operator *=(float3& value1, float3 const& value2);
    float3& operator *=(float3& value1, float value2);
    float3& operator /=(float3& value1, float3 const& value2);
    float3& operator /=(float3& value1, float value2);
    bool operator ==(float3 const& value1, float3 const& value2);
    bool operator !=(float3 const& value1, float3 const& value2);

    // Functions.
    float length(float3 const& value);
    float length_squared(float3 const& value);
    float distance(float3 const& value1, float3 const& value2);
    float distance_squared(float3 const& value1, float3 const& value2);
    float dot(float3 const& vector1, float3 const& vector2);
    float3 normalize(float3 const& value);
    float3 cross(float3 const& vector1, float3 const& vector2);
    float3 reflect(float3 const& vector, float3 const& normal);
    float3 (min)(float3 const& value1, float3 const& value2);
    float3 (max)(float3 const& value1, float3 const& value2);
    float3 clamp(float3 const& value1, float3 const& min, float3 const& max);
    float3 lerp(float3 const& value1, float3 const& value2, float amount);
    float3 transform(float3 const& position, float4x4 const& matrix);
    float3 transform_normal(float3 const& normal, float4x4 const& matrix);
    float3 transform(float3 const& value, quaternion const& rotation);


#ifndef _WINDOWS_NUMERICS_CX_PROJECTION_

    struct float4
    {
        float x, y, z, w;

        // Constructors.
        float4() = default;
        _WINDOWS_NUMERICS_CONSTEXPR_ float4(float x, float y, float z, float w);
        _WINDOWS_NUMERICS_CONSTEXPR_ float4(float2 value, float z, float w);
        _WINDOWS_NUMERICS_CONSTEXPR_ float4(float3 value, float w);
        _WINDOWS_NUMERICS_CONSTEXPR_ explicit float4(float value);

        _DEFINE_WINDOWS_NUMERICS_INTEROP_(float4, Vector4)

        // Common values.
        _WINDOWS_NUMERICS_CONSTEXPR_ static float4 zero();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float4 one();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float4 unit_x();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float4 unit_y();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float4 unit_z();
        _WINDOWS_NUMERICS_CONSTEXPR_ static float4 unit_w();
    };

#endif  // !_WINDOWS_NUMERICS_CX_PROJECTION_


    // Operators.
    float4 operator +(float4 const& value1, float4 const& value2);
    float4 operator -(float4 const& value1, float4 const& value2);
    float4 operator *(float4 const& value1, float4 const& value2);
    float4 operator *(float4 const& value1, float value2);
    float4 operator *(float value1, float4 const& value2);
    float4 operator /(float4 const& value1, float4 const& value2);
    float4 operator /(float4 const& value1, float value2);
    float4 operator -(float4 const& value);
    float4& operator +=(float4& value1, float4 const& value2);
    float4& operator -=(float4& value1, float4 const& value2);
    float4& operator *=(float4& value1, float4 const& value2);
    float4& operator *=(float4& value1, float value2);
    float4& operator /=(float4& value1, float4 const& value2);
    float4& operator /=(float4& value1, float value2);
    bool operator ==(float4 const& value1, float4 const& value2);
    bool operator !=(float4 const& value1, float4 const& value2);

    // Functions.
    float length(float4 const& value);
    float length_squared(float4 const& value);
    float distance(float4 const& value1, float4 const& value2);
    float distance_squared(float4 const& value1, float4 const& value2);
    float dot(float4 const& vector1, float4 const& vector2);
    float4 normalize(float4 const& value);
    float4 (min)(float4 const& value1, float4 const& value2);
    float4 (max)(float4 const& value1, float4 const& value2);
    float4 clamp(float4 const& value1, float4 const& min, float4 const& max);
    float4 lerp(float4 const& value1, float4 const& value2, float amount);
    float4 transform(float4 const& vector, float4x4 const& matrix);
    float4 transform4(float3 const& position, float4x4 const& matrix);
    float4 transform4(float2 const& position, float4x4 const& matrix);
    float4 transform(float4 const& value, quaternion const& rotation);
    float4 transform4(float3 const& value, quaternion const& rotation);
    float4 transform4(float2 const& value, quaternion const& rotation);


#ifndef _WINDOWS_NUMERICS_CX_PROJECTION_

    struct float3x2
    {
        float m11, m12;
        float m21, m22;
        float m31, m32;

        // Constructors.
        float3x2() = default;
        _WINDOWS_NUMERICS_CONSTEXPR_ float3x2(float m11, float m12, float m21, float m22, float m31, float m32);

        _DEFINE_WINDOWS_NUMERICS_INTEROP_(float3x2, Matrix3x2)

        // Common values.
        _WINDOWS_NUMERICS_CONSTEXPR_ static float3x2 identity();
    };

#endif  // !_WINDOWS_NUMERICS_CX_PROJECTION_


    // Factory functions.
    float3x2 make_float3x2_translation(float2 const& position);
    float3x2 make_float3x2_translation(float xPosition, float yPosition);
    float3x2 make_float3x2_scale(float xScale, float yScale);
    float3x2 make_float3x2_scale(float xScale, float yScale, float2 const& centerPoint);
    float3x2 make_float3x2_scale(float2 const& scales);
    float3x2 make_float3x2_scale(float2 const& scales, float2 const& centerPoint);
    float3x2 make_float3x2_scale(float scale);
    float3x2 make_float3x2_scale(float scale, float2 const& centerPoint);
    float3x2 make_float3x2_skew(float radiansX, float radiansY);
    float3x2 make_float3x2_skew(float radiansX, float radiansY, float2 const& centerPoint);
    float3x2 make_float3x2_rotation(float radians);
    float3x2 make_float3x2_rotation(float radians, float2 const& centerPoint);

    // Operators.
    float3x2 operator +(float3x2 const& value1, float3x2 const& value2);
    float3x2 operator -(float3x2 const& value1, float3x2 const& value2);
    float3x2 operator *(float3x2 const& value1, float3x2 const& value2);
    float3x2 operator *(float3x2 const& value1, float value2);
    float3x2 operator -(float3x2 const& value);
    float3x2& operator +=(float3x2& value1, float3x2 const& value2);
    float3x2& operator -=(float3x2& value1, float3x2 const& value2);
    float3x2& operator *=(float3x2& value1, float3x2 const& value2);
    float3x2& operator *=(float3x2& value1, float value2);
    bool operator ==(float3x2 const& value1, float3x2 const& value2);
    bool operator !=(float3x2 const& value1, float3x2 const& value2);

    // Functions.
    bool is_identity(float3x2 const& value);
    float determinant(float3x2 const& value);
    float2 translation(float3x2 const& value);
    bool invert(float3x2 const& matrix, _Out_ float3x2* result);
    float3x2 lerp(float3x2 const& matrix1, float3x2 const& matrix2, float amount);


#ifndef _WINDOWS_NUMERICS_CX_PROJECTION_

    struct float4x4
    {
        float m11, m12, m13, m14;
        float m21, m22, m23, m24;
        float m31, m32, m33, m34;
        float m41, m42, m43, m44;

        // Constructors.
        float4x4() = default;
        _WINDOWS_NUMERICS_CONSTEXPR_ float4x4(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44);
        _WINDOWS_NUMERICS_CONSTEXPR_ explicit float4x4(float3x2 value);
        
        _DEFINE_WINDOWS_NUMERICS_INTEROP_(float4x4, Matrix4x4)

        // Common values.
        _WINDOWS_NUMERICS_CONSTEXPR_ static float4x4 identity();
    };

#endif  // !_WINDOWS_NUMERICS_CX_PROJECTION_


    // Factory functions.
    float4x4 make_float4x4_billboard(float3 const& objectPosition, float3 const& cameraPosition, float3 const& cameraUpVector, float3 const& cameraForwardVector);
    float4x4 make_float4x4_constrained_billboard(float3 const& objectPosition, float3 const& cameraPosition, float3 const& rotateAxis, float3 const& cameraForwardVector, float3 const& objectForwardVector);
    float4x4 make_float4x4_translation(float3 const& position);
    float4x4 make_float4x4_translation(float xPosition, float yPosition, float zPosition);
    float4x4 make_float4x4_scale(float xScale, float yScale, float zScale);
    float4x4 make_float4x4_scale(float xScale, float yScale, float zScale, float3 const& centerPoint);
    float4x4 make_float4x4_scale(float3 const& scales);
    float4x4 make_float4x4_scale(float3 const& scales, float3 const& centerPoint);
    float4x4 make_float4x4_scale(float scale);
    float4x4 make_float4x4_scale(float scale, float3 const& centerPoint);
    float4x4 make_float4x4_rotation_x(float radians);
    float4x4 make_float4x4_rotation_x(float radians, float3 const& centerPoint);
    float4x4 make_float4x4_rotation_y(float radians);
    float4x4 make_float4x4_rotation_y(float radians, float3 const& centerPoint);
    float4x4 make_float4x4_rotation_z(float radians);
    float4x4 make_float4x4_rotation_z(float radians, float3 const& centerPoint);
    float4x4 make_float4x4_from_axis_angle(float3 const& axis, float angle);
    float4x4 make_float4x4_perspective_field_of_view(float fieldOfView, float aspectRatio, float nearPlaneDistance, float farPlaneDistance);
    float4x4 make_float4x4_perspective(float width, float height, float nearPlaneDistance, float farPlaneDistance);
    float4x4 make_float4x4_perspective_off_center(float left, float right, float bottom, float top, float nearPlaneDistance, float farPlaneDistance);
    float4x4 make_float4x4_orthographic(float width, float height, float zNearPlane, float zFarPlane);
    float4x4 make_float4x4_orthographic_off_center(float left, float right, float bottom, float top, float zNearPlane, float zFarPlane);
    float4x4 make_float4x4_look_at(float3 const& cameraPosition, float3 const& cameraTarget, float3 const& cameraUpVector);
    float4x4 make_float4x4_world(float3 const& position, float3 const& forward, float3 const& up);
    float4x4 make_float4x4_from_quaternion(quaternion const& quaternion);
    float4x4 make_float4x4_from_yaw_pitch_roll(float yaw, float pitch, float roll);
    float4x4 make_float4x4_shadow(float3 const& lightDirection, plane const& plane);
    float4x4 make_float4x4_reflection(plane const& value);

    // Operators.
    float4x4 operator +(float4x4 const& value1, float4x4 const& value2);
    float4x4 operator -(float4x4 const& value1, float4x4 const& value2);
    float4x4 operator *(float4x4 const& value1, float4x4 const& value2);
    float4x4 operator *(float4x4 const& value1, float value2);
    float4x4 operator -(float4x4 const& value);
    float4x4& operator +=(float4x4& value1, float4x4 const& value2);
    float4x4& operator -=(float4x4& value1, float4x4 const& value2);
    float4x4& operator *=(float4x4& value1, float4x4 const& value2);
    float4x4& operator *=(float4x4& value1, float value2);
    bool operator ==(float4x4 const& value1, float4x4 const& value2);
    bool operator !=(float4x4 const& value1, float4x4 const& value2);

    // Functions.
    bool is_identity(float4x4 const& value);
    float determinant(float4x4 const& value);
    float3 translation(float4x4 const& value);
    bool invert(float4x4 const& matrix, _Out_ float4x4* result);
    bool decompose(float4x4 const& matrix, _Out_ float3* scale, _Out_ quaternion* rotation, _Out_ float3* translation);
    float4x4 transform(float4x4 const& value, quaternion const& rotation);
    float4x4 transpose(float4x4 const& matrix);
    float4x4 lerp(float4x4 const& matrix1, float4x4 const& matrix2, float amount);


#ifndef _WINDOWS_NUMERICS_CX_PROJECTION_

    struct plane
    {
        float3 normal;
        float d;

        // Constructors.
        plane() = default;
        _WINDOWS_NUMERICS_CONSTEXPR_ plane(float x, float y, float z, float d);
        _WINDOWS_NUMERICS_CONSTEXPR_ plane(float3 normal, float d);
        _WINDOWS_NUMERICS_CONSTEXPR_ explicit plane(float4 value);

        _DEFINE_WINDOWS_NUMERICS_INTEROP_(plane, Plane)
    };

#endif  // !_WINDOWS_NUMERICS_CX_PROJECTION_


    // Factory functions.
    plane make_plane_from_vertices(float3 const& point1, float3 const& point2, float3 const& point3);

    // Operators.
    bool operator ==(plane const& value1, plane const& value2);
    bool operator !=(plane const& value1, plane const& value2);

    // Functions.
    plane normalize(plane const& value);
    plane transform(plane const& plane, float4x4 const& matrix);
    plane transform(plane const& plane, quaternion const& rotation);
    float dot(plane const& plane, float4 const& value);
    float dot_coordinate(plane const& plane, float3 const& value);
    float dot_normal(plane const& plane, float3 const& value);


#ifndef _WINDOWS_NUMERICS_CX_PROJECTION_

    struct quaternion
    {
        float x, y, z, w;

        // Constructors.
        quaternion() = default;
        _WINDOWS_NUMERICS_CONSTEXPR_ quaternion(float x, float y, float z, float w);
        _WINDOWS_NUMERICS_CONSTEXPR_ quaternion(float3 vectorPart, float scalarPart);

        _DEFINE_WINDOWS_NUMERICS_INTEROP_(quaternion, Quaternion)

        // Common values.
        _WINDOWS_NUMERICS_CONSTEXPR_ static quaternion identity();
    };

#endif  // !_WINDOWS_NUMERICS_CX_PROJECTION_


    // Factory functions.
    quaternion make_quaternion_from_axis_angle(float3 const& axis, float angle);
    quaternion make_quaternion_from_yaw_pitch_roll(float yaw, float pitch, float roll);
    quaternion make_quaternion_from_rotation_matrix(float4x4 const& matrix);

    // Operators.
    quaternion operator +(quaternion const& value1, quaternion const& value2);
    quaternion operator -(quaternion const& value1, quaternion const& value2);
    quaternion operator *(quaternion const& value1, quaternion const& value2);
    quaternion operator *(quaternion const& value1, float value2);
    quaternion operator /(quaternion const& value1, quaternion const& value2);
    quaternion operator -(quaternion const& value);
    quaternion& operator +=(quaternion& value1, quaternion const& value2);
    quaternion& operator -=(quaternion& value1, quaternion const& value2);
    quaternion& operator *=(quaternion& value1, quaternion const& value2);
    quaternion& operator *=(quaternion& value1, float value2);
    quaternion& operator /=(quaternion& value1, quaternion const& value2);
    bool operator ==(quaternion const& value1, quaternion const& value2);
    bool operator !=(quaternion const& value1, quaternion const& value2);

    // Functions.
    bool is_identity(quaternion const& value);
    float length(quaternion const& value);
    float length_squared(quaternion const& value);
    float dot(quaternion const& quaternion1, quaternion const& quaternion2);
    quaternion normalize(quaternion const& value);
    quaternion conjugate(quaternion const& value);
    quaternion inverse(quaternion const& value);
    quaternion slerp(quaternion const& quaternion1, quaternion const& quaternion2, float amount);
    quaternion lerp(quaternion const& quaternion1, quaternion const& quaternion2, float amount);
    quaternion concatenate(quaternion const& value1, quaternion const& value2);
}
_WINDOWS_NUMERICS_END_NAMESPACE_


// Interop between these numerics types and DirectXMath.
namespace DirectX
{
    XMVECTOR XM_CALLCONV XMLoadFloat2    (_In_ _WINDOWS_NUMERICS_NAMESPACE_::float2     const* pSource);
    XMVECTOR XM_CALLCONV XMLoadFloat3    (_In_ _WINDOWS_NUMERICS_NAMESPACE_::float3     const* pSource);
    XMVECTOR XM_CALLCONV XMLoadFloat4    (_In_ _WINDOWS_NUMERICS_NAMESPACE_::float4     const* pSource);
    XMMATRIX XM_CALLCONV XMLoadFloat3x2  (_In_ _WINDOWS_NUMERICS_NAMESPACE_::float3x2   const* pSource);
    XMMATRIX XM_CALLCONV XMLoadFloat4x4  (_In_ _WINDOWS_NUMERICS_NAMESPACE_::float4x4   const* pSource);
    XMVECTOR XM_CALLCONV XMLoadPlane     (_In_ _WINDOWS_NUMERICS_NAMESPACE_::plane      const* pSource);
    XMVECTOR XM_CALLCONV XMLoadQuaternion(_In_ _WINDOWS_NUMERICS_NAMESPACE_::quaternion const* pSource);

    void XM_CALLCONV XMStoreFloat2    (_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float2*     pDestination, _In_ FXMVECTOR value);
    void XM_CALLCONV XMStoreFloat3    (_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float3*     pDestination, _In_ FXMVECTOR value);
    void XM_CALLCONV XMStoreFloat4    (_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float4*     pDestination, _In_ FXMVECTOR value);
    void XM_CALLCONV XMStoreFloat3x2  (_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float3x2*   pDestination, _In_ FXMMATRIX value);
    void XM_CALLCONV XMStoreFloat4x4  (_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float4x4*   pDestination, _In_ FXMMATRIX value);
    void XM_CALLCONV XMStorePlane     (_Out_ _WINDOWS_NUMERICS_NAMESPACE_::plane*      pDestination, _In_ FXMVECTOR value);
    void XM_CALLCONV XMStoreQuaternion(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::quaternion* pDestination, _In_ FXMVECTOR value);
}


#include "WindowsNumerics.inl"


#undef _DEFINE_WINDOWS_NUMERICS_INTEROP_
#undef _WINDOWS_NUMERICS_CONSTEXPR_
