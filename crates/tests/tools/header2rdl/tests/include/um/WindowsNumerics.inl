//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//

#pragma warning(push)
#pragma warning(disable: 4723) // potential divide by 0
#pragma warning(disable: 4756) // overflow in constant arithmetic


#ifdef _CPPUNWIND

// If C++ exception handling is enabled, we can throw exceptions and use STL to access NaN.
#include <stdexcept>
#include <limits>

#define _WINDOWS_NUMERICS_THROW_(e) throw e
#define _WINDOWS_NUMERICS_NAN_      std::numeric_limits<float>::quiet_NaN()

#else

// Fallback for environments with exception handling disabled.
#define _WINDOWS_NUMERICS_THROW_(e) (void)0
#define _WINDOWS_NUMERICS_NAN_      ::DirectX::XMVectorGetX(::DirectX::XMVectorSplatQNaN())

#endif


// Implementing some operations via the SIMD DirectXMath API is a performance
// win for SSE CPU architectures (x86 and x64), but not for ARM NEON.
#if defined _M_ARM && !defined WINDOWS_NUMERICS_DISABLE_SIMD
#define WINDOWS_NUMERICS_DISABLE_SIMD
#endif


namespace DirectX
{
    inline XMVECTOR XM_CALLCONV XMLoadFloat2(_In_ _WINDOWS_NUMERICS_NAMESPACE_::float2 const* pSource)
    {
        return XMLoadFloat2(reinterpret_cast<XMFLOAT2 const*>(pSource));
    }


    inline XMVECTOR XM_CALLCONV XMLoadFloat3(_In_ _WINDOWS_NUMERICS_NAMESPACE_::float3 const* pSource)
    {
        return XMLoadFloat3(reinterpret_cast<XMFLOAT3 const*>(pSource));
    }


    inline XMVECTOR XM_CALLCONV XMLoadFloat4(_In_ _WINDOWS_NUMERICS_NAMESPACE_::float4 const* pSource)
    {
        return XMLoadFloat4(reinterpret_cast<XMFLOAT4 const*>(pSource));
    }


    inline XMMATRIX XM_CALLCONV XMLoadFloat3x2(_In_ _WINDOWS_NUMERICS_NAMESPACE_::float3x2 const* pSource)
    {
        XMVECTOR abcd = XMLoadFloat4(reinterpret_cast<XMFLOAT4 const*>(&pSource->m11));
        XMVECTOR ef = XMLoadFloat2(reinterpret_cast<XMFLOAT2 const*>(&pSource->m31));

        XMMATRIX m;

        m.r[0] = XMVectorPermute<0, 1, 4, 5>(abcd, g_XMZero);
        m.r[1] = XMVectorPermute<2, 3, 4, 5>(abcd, g_XMZero);
        m.r[2] = g_XMIdentityR2;
        m.r[3] = XMVectorPermute<0, 1, 6, 7>(ef, g_XMIdentityR3);

        return m;
    }


    inline XMMATRIX XM_CALLCONV XMLoadFloat4x4(_In_ _WINDOWS_NUMERICS_NAMESPACE_::float4x4 const* pSource)
    {
        return XMLoadFloat4x4(reinterpret_cast<XMFLOAT4X4 const*>(pSource));
    }


    inline XMVECTOR XM_CALLCONV XMLoadPlane(_In_ _WINDOWS_NUMERICS_NAMESPACE_::plane const* pSource)
    {
        return XMLoadFloat4(reinterpret_cast<XMFLOAT4 const*>(pSource));
    }


    inline XMVECTOR XM_CALLCONV XMLoadQuaternion(_In_ _WINDOWS_NUMERICS_NAMESPACE_::quaternion const* pSource)
    {
        return XMLoadFloat4(reinterpret_cast<XMFLOAT4 const*>(pSource));
    }


    inline void XM_CALLCONV XMStoreFloat2(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float2* pDestination, _In_ FXMVECTOR value)
    {
        XMStoreFloat2(reinterpret_cast<XMFLOAT2*>(pDestination), value);
    }


    inline void XM_CALLCONV XMStoreFloat3(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float3* pDestination, _In_ FXMVECTOR value)
    {
        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pDestination), value);
    }


    inline void XM_CALLCONV XMStoreFloat4(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float4* pDestination, _In_ FXMVECTOR value)
    {
        XMStoreFloat4(reinterpret_cast<XMFLOAT4*>(pDestination), value);
    }


    inline void XM_CALLCONV XMStoreFloat3x2(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float3x2* pDestination, _In_ FXMMATRIX value)
    {
        XMVECTOR abcd = XMVectorPermute<0, 1, 4, 5>(value.r[0], value.r[1]);
        XMVECTOR ef = value.r[3];

        XMStoreFloat4(reinterpret_cast<XMFLOAT4*>(&pDestination->m11), abcd);
        XMStoreFloat2(reinterpret_cast<XMFLOAT2*>(&pDestination->m31), ef);
    }


    inline void XM_CALLCONV XMStoreFloat4x4(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::float4x4* pDestination, _In_ FXMMATRIX value)
    {
        XMStoreFloat4x4(reinterpret_cast<XMFLOAT4X4*>(pDestination), value);
    }


    inline void XM_CALLCONV XMStorePlane(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::plane* pDestination, _In_ FXMVECTOR value)
    {
        XMStoreFloat4(reinterpret_cast<XMFLOAT4*>(pDestination), value);
    }


    inline void XM_CALLCONV XMStoreQuaternion(_Out_ _WINDOWS_NUMERICS_NAMESPACE_::quaternion* pDestination, _In_ FXMVECTOR value)
    {
        XMStoreFloat4(reinterpret_cast<XMFLOAT4*>(pDestination), value);
    }
}


_WINDOWS_NUMERICS_BEGIN_NAMESPACE_
{
    _WINDOWS_NUMERICS_CONSTEXPR_ float2::float2(float x, float y)
        : x(x), y(y)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float2::float2(float value)
        : x(value), y(value)
    { }


#ifdef _WINDOWS_NUMERICS_CX_PROJECTION_


    inline float2::operator float2(_WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point value)
    {
        return float2(value.X, value.Y);
    }


    inline float2::operator float2(_WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size value)
    {
        return float2(value.Width, value.Height);
    }


    inline float2::operator _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point(float2 value)
    {
        return _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point(value.x, value.y);
    }


    inline float2::operator _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size(float2 value)
    {
        return _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size(value.x, value.y);
    }


#elif defined _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_ // && !_WINDOWS_NUMERICS_CX_PROJECTION_


    inline float2::float2(_WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point const& value)
        : x(value.X), y(value.Y)
    { }


    inline float2::float2(_WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size const& value)
        : x(value.Width), y(value.Height)
    { }


    inline float2::operator _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point() const
    {
        return _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Point(x, y);
    }


    inline float2::operator _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size() const
    {
        return _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_::Size(x, y);
    }


#endif  // _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_ && !_WINDOWS_NUMERICS_CX_PROJECTION_


    _WINDOWS_NUMERICS_CONSTEXPR_ float2 float2::zero()
    {
        return float2(0, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float2 float2::one()
    {
        return float2(1, 1);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float2 float2::unit_x()
    {
        return float2(1, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float2 float2::unit_y()
    {
        return float2(0, 1);
    }


    inline float2 operator +(float2 const& value1, float2 const& value2)
    {
        return float2(value1.x + value2.x,
                      value1.y + value2.y);
    }


    inline float2 operator -(float2 const& value1, float2 const& value2)
    {
        return float2(value1.x - value2.x,
                      value1.y - value2.y);
    }


    inline float2 operator *(float2 const& value1, float2 const& value2)
    {
        return float2(value1.x * value2.x,
                      value1.y * value2.y);
    }


    inline float2 operator *(float2 const& value1, float value2)
    {
        return float2(value1.x * value2,
                      value1.y * value2);
    }


    inline float2 operator *(float value1, float2 const& value2)
    {
        return value2 * value1;
    }


    inline float2 operator /(float2 const& value1, float2 const& value2)
    {
        return float2(value1.x / value2.x,
                      value1.y / value2.y);
    }


    inline float2 operator /(float2 const& value1, float value2)
    {
        return value1 * (1.0f / value2);
    }


    inline float2 operator -(float2 const& value)
    {
        return float2(-value.x,
                      -value.y);
    }


    inline float2& operator +=(float2& value1, float2 const& value2)
    {
        value1 = value1 + value2;

        return value1;
    }


    inline float2& operator -=(float2& value1, float2 const& value2)
    {
        value1 = value1 - value2;

        return value1;
    }


    inline float2& operator *=(float2& value1, float2 const& value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float2& operator *=(float2& value1, float value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float2& operator /=(float2& value1, float2 const& value2)
    {
        value1 = value1 / value2;

        return value1;
    }


    inline float2& operator /=(float2& value1, float value2)
    {
        value1 = value1 / value2;

        return value1;
    }


    inline bool operator ==(float2 const& value1, float2 const& value2)
    {
        return value1.x == value2.x &&
               value1.y == value2.y;
    }


    inline bool operator !=(float2 const& value1, float2 const& value2)
    {
        return value1.x != value2.x ||
               value1.y != value2.y;
    }


    inline float length(float2 const& value)
    {
        return sqrtf(length_squared(value));
    }


    inline float length_squared(float2 const& value)
    {
        return dot(value, value);
    }


    inline float distance(float2 const& value1, float2 const& value2)
    {
        return length(value1 - value2);
    }


    inline float distance_squared(float2 const& value1, float2 const& value2)
    {
        return length_squared(value1 - value2);
    }


    inline float dot(float2 const& value1, float2 const& value2)
    {
        return value1.x * value2.x +
               value1.y * value2.y;
    }


    inline float2 normalize(float2 const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return value / length(value);
#else
        using namespace ::DirectX;

        float2 result;
        XMVECTOR v = XMLoadFloat2(&value);
        XMStoreFloat2(&result, XMVectorDivide(v, XMVector2Length(v)));
        return result;
#endif
    }


    inline float2 reflect(float2 const& vector, float2 const& normal)
    {
        return vector - 2.0f * dot(vector, normal) * normal;
    }


    inline float2 (min)(float2 const& value1, float2 const& value2)
    {
        return float2((value1.x < value2.x) ? value1.x : value2.x,
                      (value1.y < value2.y) ? value1.y : value2.y);
    }


    inline float2 (max)(float2 const& value1, float2 const& value2)
    {
        return float2((value1.x > value2.x) ? value1.x : value2.x,
                      (value1.y > value2.y) ? value1.y : value2.y);
    }


    inline float2 clamp(float2 const& value1, float2 const& minValue, float2 const& maxValue)
    {
        return (max)((min)(value1, maxValue), minValue);
    }


    inline float2 lerp(float2 const& value1, float2 const& value2, float amount)
    {
        return value1 + (value2 - value1) * amount;
    }


    inline float2 transform(float2 const& position, float3x2 const& matrix)
    {
        return float2(position.x * matrix.m11 + position.y * matrix.m21 + matrix.m31,
                      position.x * matrix.m12 + position.y * matrix.m22 + matrix.m32);
    }


    inline float2 transform(float2 const& position, float4x4 const& matrix)
    {
        return float2(position.x * matrix.m11 + position.y * matrix.m21 + matrix.m41,
                      position.x * matrix.m12 + position.y * matrix.m22 + matrix.m42);
    }


    inline float2 transform_normal(float2 const& normal, float3x2 const& matrix)
    {
        return float2(normal.x * matrix.m11 + normal.y * matrix.m21,
                      normal.x * matrix.m12 + normal.y * matrix.m22);
    }


    inline float2 transform_normal(float2 const& normal, float4x4 const& matrix)
    {
        return float2(normal.x * matrix.m11 + normal.y * matrix.m21,
                      normal.x * matrix.m12 + normal.y * matrix.m22);
    }


    inline float2 transform(float2 const& value, quaternion const& rotation)
    {
        float x2 = rotation.x + rotation.x;
        float y2 = rotation.y + rotation.y;
        float z2 = rotation.z + rotation.z;

        float wz2 = rotation.w * z2;
        float xx2 = rotation.x * x2;
        float xy2 = rotation.x * y2;
        float yy2 = rotation.y * y2;
        float zz2 = rotation.z * z2;

        return float2(value.x * (1.0f - yy2 - zz2) + value.y * (xy2 - wz2),
                      value.x * (xy2 + wz2) + value.y * (1.0f - xx2 - zz2));
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3::float3(float x, float y, float z)
        : x(x), y(y), z(z)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3::float3(float2 value, float z)
        : x(value.x), y(value.y), z(z)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3::float3(float value)
        : x(value), y(value), z(value)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3 float3::zero()
    {
        return float3(0, 0, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3 float3::one()
    {
        return float3(1, 1, 1);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3 float3::unit_x()
    {
        return float3(1, 0, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3 float3::unit_y()
    {
        return float3(0, 1, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3 float3::unit_z()
    {
        return float3(0, 0, 1);
    }


    inline float3 operator +(float3 const& value1, float3 const& value2)
    {
        return float3(value1.x + value2.x,
                      value1.y + value2.y,
                      value1.z + value2.z);
    }


    inline float3 operator -(float3 const& value1, float3 const& value2)
    {
        return float3(value1.x - value2.x,
                      value1.y - value2.y,
                      value1.z - value2.z);
    }


    inline float3 operator *(float3 const& value1, float3 const& value2)
    {
        return float3(value1.x * value2.x,
                      value1.y * value2.y,
                      value1.z * value2.z);
    }


    inline float3 operator *(float3 const& value1, float value2)
    {
        return float3(value1.x * value2,
                      value1.y * value2,
                      value1.z * value2);
    }


    inline float3 operator *(float value1, float3 const& value2)
    {
        return value2 * value1;
    }


    inline float3 operator /(float3 const& value1, float3 const& value2)
    {
        return float3(value1.x / value2.x,
                      value1.y / value2.y,
                      value1.z / value2.z);
    }


    inline float3 operator /(float3 const& value1, float value2)
    {
        return value1 * (1.0f / value2);
    }


    inline float3 operator -(float3 const& value)
    {
        return float3(-value.x,
                      -value.y,
                      -value.z);
    }


    inline float3& operator +=(float3& value1, float3 const& value2)
    {
        value1 = value1 + value2;

        return value1;
    }


    inline float3& operator -=(float3& value1, float3 const& value2)
    {
        value1 = value1 - value2;

        return value1;
    }


    inline float3& operator *=(float3& value1, float3 const& value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float3& operator *=(float3& value1, float value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float3& operator /=(float3& value1, float3 const& value2)
    {
        value1 = value1 / value2;

        return value1;
    }


    inline float3& operator /=(float3& value1, float value2)
    {
        value1 = value1 / value2;

        return value1;
    }


    inline bool operator ==(float3 const& value1, float3 const& value2)
    {
        return value1.x == value2.x &&
               value1.y == value2.y &&
               value1.z == value2.z;
    }


    inline bool operator !=(float3 const& value1, float3 const& value2)
    {
        return value1.x != value2.x ||
               value1.y != value2.y ||
               value1.z != value2.z;
    }


    inline float length(float3 const& value)
    {
        return sqrtf(length_squared(value));
    }


    inline float length_squared(float3 const& value)
    {
        return dot(value, value);
    }


    inline float distance(float3 const& value1, float3 const& value2)
    {
        return length(value1 - value2);
    }


    inline float distance_squared(float3 const& value1, float3 const& value2)
    {
        return length_squared(value1 - value2);
    }


    inline float dot(float3 const& vector1, float3 const& vector2)
    {
        return vector1.x * vector2.x +
               vector1.y * vector2.y +
               vector1.z * vector2.z;
    }


    inline float3 normalize(float3 const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return value / length(value);
#else
        using namespace ::DirectX;

        float3 result;
        XMVECTOR v = XMLoadFloat3(&value);
        XMStoreFloat3(&result, XMVectorDivide(v, XMVector3Length(v)));
        return result;
#endif
    }


    inline float3 cross(float3 const& vector1, float3 const& vector2)
    {
        return float3(vector1.y * vector2.z - vector1.z * vector2.y,
                      vector1.z * vector2.x - vector1.x * vector2.z,
                      vector1.x * vector2.y - vector1.y * vector2.x);
    }


    inline float3 reflect(float3 const& vector, float3 const& normal)
    {
        return vector - 2.0f * dot(vector, normal) * normal;
    }


    inline float3 (min)(float3 const& value1, float3 const& value2)
    {
        return float3((value1.x < value2.x) ? value1.x : value2.x,
                      (value1.y < value2.y) ? value1.y : value2.y,
                      (value1.z < value2.z) ? value1.z : value2.z);
    }


    inline float3 (max)(float3 const& value1, float3 const& value2)
    {
        return float3((value1.x > value2.x) ? value1.x : value2.x,
                      (value1.y > value2.y) ? value1.y : value2.y,
                      (value1.z > value2.z) ? value1.z : value2.z);
    }


    inline float3 clamp(float3 const& value1, float3 const& minValue, float3 const& maxValue)
    {
        return (max)((min)(value1, maxValue), minValue);
    }


    inline float3 lerp(float3 const& value1, float3 const& value2, float amount)
    {
        return value1 + (value2 - value1) * amount;
    }


    inline float3 transform(float3 const& position, float4x4 const& matrix)
    {
        return float3(position.x * matrix.m11 + position.y * matrix.m21 + position.z * matrix.m31 + matrix.m41,
                      position.x * matrix.m12 + position.y * matrix.m22 + position.z * matrix.m32 + matrix.m42,
                      position.x * matrix.m13 + position.y * matrix.m23 + position.z * matrix.m33 + matrix.m43);
    }


    inline float3 transform_normal(float3 const& normal, float4x4 const& matrix)
    {
        return float3(normal.x * matrix.m11 + normal.y * matrix.m21 + normal.z * matrix.m31,
                      normal.x * matrix.m12 + normal.y * matrix.m22 + normal.z * matrix.m32,
                      normal.x * matrix.m13 + normal.y * matrix.m23 + normal.z * matrix.m33);
    }


    inline float3 transform(float3 const& value, quaternion const& rotation)
    {
        float x2 = rotation.x + rotation.x;
        float y2 = rotation.y + rotation.y;
        float z2 = rotation.z + rotation.z;

        float wx2 = rotation.w * x2;
        float wy2 = rotation.w * y2;
        float wz2 = rotation.w * z2;
        float xx2 = rotation.x * x2;
        float xy2 = rotation.x * y2;
        float xz2 = rotation.x * z2;
        float yy2 = rotation.y * y2;
        float yz2 = rotation.y * z2;
        float zz2 = rotation.z * z2;

        return float3(value.x * (1.0f - yy2 - zz2) + value.y * (xy2 - wz2) + value.z * (xz2 + wy2),
                      value.x * (xy2 + wz2) + value.y * (1.0f - xx2 - zz2) + value.z * (yz2 - wx2),
                      value.x * (xz2 - wy2) + value.y * (yz2 + wx2) + value.z * (1.0f - xx2 - yy2));
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4::float4(float x, float y, float z, float w)
        : x(x), y(y), z(z), w(w)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4::float4(float2 value, float z, float w)
        : x(value.x), y(value.y), z(z), w(w)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4::float4(float3 value, float w)
        : x(value.x), y(value.y), z(value.z), w(w)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4::float4(float value)
        : x(value), y(value), z(value), w(value)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4 float4::zero()
    {
        return float4(0, 0, 0, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4 float4::one()
    {
        return float4(1, 1, 1, 1);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4 float4::unit_x()
    {
        return float4(1, 0, 0, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4 float4::unit_y()
    {
        return float4(0, 1, 0, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4 float4::unit_z()
    {
        return float4(0, 0, 1, 0);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4 float4::unit_w()
    {
        return float4(0, 0, 0, 1);
    }


    inline float4 operator +(float4 const& value1, float4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4(value1.x + value2.x,
                      value1.y + value2.y,
                      value1.z + value2.z,
                      value1.w + value2.w);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorAdd(XMLoadFloat4(&value1), XMLoadFloat4(&value2)));
        return result;
#endif
    }


    inline float4 operator -(float4 const& value1, float4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4(value1.x - value2.x,
                      value1.y - value2.y,
                      value1.z - value2.z,
                      value1.w - value2.w);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorSubtract(XMLoadFloat4(&value1), XMLoadFloat4(&value2)));
        return result;
#endif
    }


    inline float4 operator *(float4 const& value1, float4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4(value1.x * value2.x,
                      value1.y * value2.y,
                      value1.z * value2.z,
                      value1.w * value2.w);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorMultiply(XMLoadFloat4(&value1), XMLoadFloat4(&value2)));
        return result;
#endif
    }


    inline float4 operator *(float4 const& value1, float value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4(value1.x * value2,
                      value1.y * value2,
                      value1.z * value2,
                      value1.w * value2);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorScale(XMLoadFloat4(&value1), value2));
        return result;
#endif
    }


    inline float4 operator *(float value1, float4 const& value2)
    {
        return value2 * value1;
    }


    inline float4 operator /(float4 const& value1, float4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4(value1.x / value2.x,
                      value1.y / value2.y,
                      value1.z / value2.z,
                      value1.w / value2.w);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorDivide(XMLoadFloat4(&value1), XMLoadFloat4(&value2)));
        return result;
#endif
    }


    inline float4 operator /(float4 const& value1, float value2)
    {
        return value1 * (1.0f / value2);
    }


    inline float4 operator -(float4 const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4(-value.x,
                      -value.y,
                      -value.z,
                      -value.w);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorNegate(XMLoadFloat4(&value)));
        return result;
#endif
    }


    inline float4& operator +=(float4& value1, float4 const& value2)
    {
        value1 = value1 + value2;

        return value1;
    }


    inline float4& operator -=(float4& value1, float4 const& value2)
    {
        value1 = value1 - value2;

        return value1;
    }


    inline float4& operator *=(float4& value1, float4 const& value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float4& operator *=(float4& value1, float value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float4& operator /=(float4& value1, float4 const& value2)
    {
        value1 = value1 / value2;

        return value1;
    }


    inline float4& operator /=(float4& value1, float value2)
    {
        value1 = value1 / value2;

        return value1;
    }


    inline bool operator ==(float4 const& value1, float4 const& value2)
    {
        return value1.x == value2.x &&
               value1.y == value2.y &&
               value1.z == value2.z &&
               value1.w == value2.w;
    }


    inline bool operator !=(float4 const& value1, float4 const& value2)
    {
        return value1.x != value2.x ||
               value1.y != value2.y ||
               value1.z != value2.z ||
               value1.w != value2.w;
    }


    inline float length(float4 const& value)
    {
        return sqrtf(length_squared(value));
    }


    inline float length_squared(float4 const& value)
    {
        return dot(value, value);
    }


    inline float distance(float4 const& value1, float4 const& value2)
    {
        return length(value1 - value2);
    }


    inline float distance_squared(float4 const& value1, float4 const& value2)
    {
        return length_squared(value1 - value2);
    }


    inline float dot(float4 const& vector1, float4 const& vector2)
    {
        return vector1.x * vector2.x + 
               vector1.y * vector2.y + 
               vector1.z * vector2.z + 
               vector1.w * vector2.w;
    }


    inline float4 normalize(float4 const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return value / length(value);
#else
        using namespace ::DirectX;

        float4 result;
        XMVECTOR v = XMLoadFloat4(&value);
        XMStoreFloat4(&result, XMVectorDivide(v, XMVector4Length(v)));
        return result;
#endif
    }


    inline float4 (min)(float4 const& value1, float4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4((value1.x < value2.x) ? value1.x : value2.x,
                      (value1.y < value2.y) ? value1.y : value2.y,
                      (value1.z < value2.z) ? value1.z : value2.z,
                      (value1.w < value2.w) ? value1.w : value2.w);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorMin(XMLoadFloat4(&value1), XMLoadFloat4(&value2)));
        return result;
#endif
    }


    inline float4 (max)(float4 const& value1, float4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4((value1.x > value2.x) ? value1.x : value2.x,
                      (value1.y > value2.y) ? value1.y : value2.y,
                      (value1.z > value2.z) ? value1.z : value2.z,
                      (value1.w > value2.w) ? value1.w : value2.w);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVectorMax(XMLoadFloat4(&value1), XMLoadFloat4(&value2)));
        return result;
#endif
    }


    inline float4 clamp(float4 const& value1, float4 const& minValue, float4 const& maxValue)
    {
        return (max)((min)(value1, maxValue), minValue);
    }


    inline float4 lerp(float4 const& value1, float4 const& value2, float amount)
    {
        return value1 + (value2 - value1) * amount;
    }


    inline float4 transform(float4 const& vector, float4x4 const& matrix)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4(vector.x * matrix.m11 + vector.y * matrix.m21 + vector.z * matrix.m31 + vector.w * matrix.m41,
                      vector.x * matrix.m12 + vector.y * matrix.m22 + vector.z * matrix.m32 + vector.w * matrix.m42,
                      vector.x * matrix.m13 + vector.y * matrix.m23 + vector.z * matrix.m33 + vector.w * matrix.m43,
                      vector.x * matrix.m14 + vector.y * matrix.m24 + vector.z * matrix.m34 + vector.w * matrix.m44);
#else
        using namespace ::DirectX;

        float4 result;
        XMStoreFloat4(&result, XMVector4Transform(XMLoadFloat4(&vector), XMLoadFloat4x4(&matrix)));
        return result;
#endif
    }


    inline float4 transform4(float3 const& position, float4x4 const& matrix)
    {
        return float4(position.x * matrix.m11 + position.y * matrix.m21 + position.z * matrix.m31 + matrix.m41,
                      position.x * matrix.m12 + position.y * matrix.m22 + position.z * matrix.m32 + matrix.m42,
                      position.x * matrix.m13 + position.y * matrix.m23 + position.z * matrix.m33 + matrix.m43,
                      position.x * matrix.m14 + position.y * matrix.m24 + position.z * matrix.m34 + matrix.m44);
    }


    inline float4 transform4(float2 const& position, float4x4 const& matrix)
    {
        return float4(position.x * matrix.m11 + position.y * matrix.m21 + matrix.m41,
                      position.x * matrix.m12 + position.y * matrix.m22 + matrix.m42,
                      position.x * matrix.m13 + position.y * matrix.m23 + matrix.m43,
                      position.x * matrix.m14 + position.y * matrix.m24 + matrix.m44);
    }


    inline float4 transform(float4 const& value, quaternion const& rotation)
    {
        float x2 = rotation.x + rotation.x;
        float y2 = rotation.y + rotation.y;
        float z2 = rotation.z + rotation.z;

        float wx2 = rotation.w * x2;
        float wy2 = rotation.w * y2;
        float wz2 = rotation.w * z2;
        float xx2 = rotation.x * x2;
        float xy2 = rotation.x * y2;
        float xz2 = rotation.x * z2;
        float yy2 = rotation.y * y2;
        float yz2 = rotation.y * z2;
        float zz2 = rotation.z * z2;

        return float4(value.x * (1.0f - yy2 - zz2) + value.y * (       xy2 - wz2) + value.z * (       xz2 + wy2),
                      value.x * (       xy2 + wz2) + value.y * (1.0f - xx2 - zz2) + value.z * (       yz2 - wx2),
                      value.x * (       xz2 - wy2) + value.y * (       yz2 + wx2) + value.z * (1.0f - xx2 - yy2),
                      value.w);
    }


    inline float4 transform4(float3 const& value, quaternion const& rotation)
    {
        float x2 = rotation.x + rotation.x;
        float y2 = rotation.y + rotation.y;
        float z2 = rotation.z + rotation.z;

        float wx2 = rotation.w * x2;
        float wy2 = rotation.w * y2;
        float wz2 = rotation.w * z2;
        float xx2 = rotation.x * x2;
        float xy2 = rotation.x * y2;
        float xz2 = rotation.x * z2;
        float yy2 = rotation.y * y2;
        float yz2 = rotation.y * z2;
        float zz2 = rotation.z * z2;

        return float4(value.x * (1.0f - yy2 - zz2) + value.y * (       xy2 - wz2) + value.z * (       xz2 + wy2),
                      value.x * (       xy2 + wz2) + value.y * (1.0f - xx2 - zz2) + value.z * (       yz2 - wx2),
                      value.x * (       xz2 - wy2) + value.y * (       yz2 + wx2) + value.z * (1.0f - xx2 - yy2),
                      1.0f);
    }


    inline float4 transform4(float2 const& value, quaternion const& rotation)
    {
        float x2 = rotation.x + rotation.x;
        float y2 = rotation.y + rotation.y;
        float z2 = rotation.z + rotation.z;

        float wx2 = rotation.w * x2;
        float wy2 = rotation.w * y2;
        float wz2 = rotation.w * z2;
        float xx2 = rotation.x * x2;
        float xy2 = rotation.x * y2;
        float xz2 = rotation.x * z2;
        float yy2 = rotation.y * y2;
        float yz2 = rotation.y * z2;
        float zz2 = rotation.z * z2;

        return float4(value.x * (1.0f - yy2 - zz2) + value.y * (       xy2 - wz2),
                      value.x * (       xy2 + wz2) + value.y * (1.0f - xx2 - zz2),
                      value.x * (       xz2 - wy2) + value.y * (       yz2 + wx2),
                      1.0f);
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3x2::float3x2(float m11, float m12, float m21, float m22, float m31, float m32)
        : m11(m11), m12(m12), m21(m21), m22(m22), m31(m31), m32(m32)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float3x2 float3x2::identity()
    {
        return float3x2(1, 0,
                        0, 1,
                        0, 0);
    }


    inline float3x2 make_float3x2_translation(float2 const& position)
    {
        return float3x2(1, 0,
                        0, 1,
                        position.x, position.y);
    }


    inline float3x2 make_float3x2_translation(float xPosition, float yPosition)
    {
        return float3x2(1, 0,
                        0, 1,
                        xPosition, yPosition);
    }


    inline float3x2 make_float3x2_scale(float xScale, float yScale)
    {
        return float3x2(xScale, 0,
                        0,      yScale,
                        0,      0);
    }


    inline float3x2 make_float3x2_scale(float xScale, float yScale, float2 const& centerPoint)
    {
        float tx = centerPoint.x * (1 - xScale);
        float ty = centerPoint.y * (1 - yScale);

        return float3x2(xScale, 0,
                        0,      yScale,
                        tx,     ty);
    }


    inline float3x2 make_float3x2_scale(float2 const& scales)
    {
        return float3x2(scales.x, 0,
                        0,        scales.y,
                        0,        0);
    }


    inline float3x2 make_float3x2_scale(float2 const& scales, float2 const& centerPoint)
    {
        float2 t = centerPoint * (float2::one() - scales);

        return float3x2(scales.x, 0,
                        0,        scales.y,
                        t.x,      t.y);
    }


    inline float3x2 make_float3x2_scale(float scale)
    {
        return float3x2(scale, 0,
                        0,     scale,
                        0,     0);
    }


    inline float3x2 make_float3x2_scale(float scale, float2 const& centerPoint)
    {
        float2 t = centerPoint * (1 - scale);

        return float3x2(scale, 0,
                        0,     scale,
                        t.x,   t.y);
    }


    inline float3x2 make_float3x2_skew(float radiansX, float radiansY)
    {
        float xTan = tanf(radiansX);
        float yTan = tanf(radiansY);

        return float3x2(1,    yTan,
                        xTan, 1,
                        0,    0);
    }


    inline float3x2 make_float3x2_skew(float radiansX, float radiansY, float2 const& centerPoint)
    {
        float xTan = tanf(radiansX);
        float yTan = tanf(radiansY);

        float tx = -centerPoint.y * xTan;
        float ty = -centerPoint.x * yTan;

        return float3x2(1,    yTan,
                        xTan, 1,
                        tx,   ty);
    }


    inline float3x2 make_float3x2_rotation(float radians)
    {
        return make_float3x2_rotation(radians, float2::zero());
    }


    inline float3x2 make_float3x2_rotation(float radians, float2 const& centerPoint)
    {
        radians = fmodf(radians, ::DirectX::XM_2PI);

        if (radians < 0)
            radians += ::DirectX::XM_2PI;

        float c, s;

        const float epsilon = 0.001f * ::DirectX::XM_PI / 180.0f;     // 0.1% of a degree

        if (radians < epsilon || radians > ::DirectX::XM_2PI - epsilon)
        {
            // Exact case for zero rotation.
            c = 1;
            s = 0;
        }
        else if (radians > ::DirectX::XM_PIDIV2 - epsilon && radians < ::DirectX::XM_PIDIV2 + epsilon)
        {
            // Exact case for 90 degree rotation.
            c = 0;
            s = 1;
        }
        else if (radians > ::DirectX::XM_PI - epsilon && radians < ::DirectX::XM_PI + epsilon)
        {
            // Exact case for 180 degree rotation.
            c = -1;
            s = 0;
        }
        else if (radians > ::DirectX::XM_PI + ::DirectX::XM_PIDIV2 - epsilon && radians < ::DirectX::XM_PI + ::DirectX::XM_PIDIV2 + epsilon)
        {
            // Exact case for 270 degree rotation.
            c = 0;
            s = -1;
        }
        else
        {
            // Arbitrary rotation.
            c = cosf(radians);
            s = sinf(radians);
        }

        float x = centerPoint.x * (1 - c) + centerPoint.y * s;
        float y = centerPoint.y * (1 - c) - centerPoint.x * s;

        return float3x2( c, s,
                        -s, c,
                         x, y);
    }


    inline float3x2 operator +(float3x2 const& value1, float3x2 const& value2)
    {
        return float3x2(value1.m11 + value2.m11,  value1.m12 + value2.m12,
                        value1.m21 + value2.m21,  value1.m22 + value2.m22,
                        value1.m31 + value2.m31,  value1.m32 + value2.m32);
    }


    inline float3x2 operator -(float3x2 const& value1, float3x2 const& value2)
    {
        return float3x2(value1.m11 - value2.m11,  value1.m12 - value2.m12,
                        value1.m21 - value2.m21,  value1.m22 - value2.m22,
                        value1.m31 - value2.m31,  value1.m32 - value2.m32);
    }


    inline float3x2 operator *(float3x2 const& value1, float3x2 const& value2)
    {
        return float3x2
        (
            // First row
            value1.m11 * value2.m11 + value1.m12 * value2.m21,
            value1.m11 * value2.m12 + value1.m12 * value2.m22,
            
            // Second row
            value1.m21 * value2.m11 + value1.m22 * value2.m21,
            value1.m21 * value2.m12 + value1.m22 * value2.m22,
            
            // Third row
            value1.m31 * value2.m11 + value1.m32 * value2.m21 + value2.m31,
            value1.m31 * value2.m12 + value1.m32 * value2.m22 + value2.m32
        );
    }


    inline float3x2 operator *(float3x2 const& value1, float value2)
    {
        return float3x2(value1.m11 * value2,  value1.m12 * value2,
                        value1.m21 * value2,  value1.m22 * value2,
                        value1.m31 * value2,  value1.m32 * value2);
    }


    inline float3x2 operator -(float3x2 const& value)
    {
        return float3x2(-value.m11, -value.m12,
                        -value.m21, -value.m22,
                        -value.m31, -value.m32);
    }


    inline float3x2& operator +=(float3x2& value1, float3x2 const& value2)
    {
        value1 = value1 + value2;

        return value1;
    }


    inline float3x2& operator -=(float3x2& value1, float3x2 const& value2)
    {
        value1 = value1 - value2;

        return value1;
    }


    inline float3x2& operator *=(float3x2& value1, float3x2 const& value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float3x2& operator *=(float3x2& value1, float value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline bool operator ==(float3x2 const& value1, float3x2 const& value2)
    {
        return value1.m11 == value2.m11 && value1.m22 == value2.m22 && // Check diagonal element first for early out.
                                           value1.m12 == value2.m12 &&
               value1.m21 == value2.m21                             &&
               value1.m31 == value2.m31 && value1.m32 == value2.m32;
    }


    inline bool operator !=(float3x2 const& value1, float3x2 const& value2)
    {
        return value1.m11 != value2.m11 || value1.m12 != value2.m12 ||
               value1.m21 != value2.m21 || value1.m22 != value2.m22 ||
               value1.m31 != value2.m31 || value1.m32 != value2.m32;
    }


    inline bool is_identity(float3x2 const& value)
    {
        return value.m11 == 1 && value.m22 == 1 && // Check diagonal element first for early out.
                                 value.m12 == 0 &&
               value.m21 == 0                   &&
               value.m31 == 0 && value.m32 == 0;
    }


    inline float determinant(float3x2 const& value)
    {
        return (value.m11 * value.m22) - (value.m21 * value.m12);
    }


    inline float2 translation(float3x2 const& value)
    {
        return float2(value.m31, value.m32);
    }


    inline bool invert(float3x2 const& matrix, _Out_ float3x2* result)
    {
        float det = determinant(matrix);

        // NaN safe
        if (!(fabs(det) >= FLT_EPSILON))
        {
            const float nan = _WINDOWS_NUMERICS_NAN_;
            *result = float3x2(nan, nan, nan, nan, nan, nan);
            return false;
        }

        float invDet = 1.0f / det;

        *result = float3x2
        (
            // First row
            matrix.m22 * invDet,
            -matrix.m12 * invDet,

            // Second row
            -matrix.m21 * invDet,
            matrix.m11 * invDet,

            // Third row
            (matrix.m21 * matrix.m32 - matrix.m31 * matrix.m22) * invDet,
            (matrix.m31 * matrix.m12 - matrix.m11 * matrix.m32) * invDet
        );

        return true;
    }


    inline float3x2 lerp(float3x2 const& matrix1, float3x2 const& matrix2, float amount)
    {
        return matrix1 + (matrix2 - matrix1) * amount;
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4x4::float4x4(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44)
        : m11(m11), m12(m12), m13(m13), m14(m14),
          m21(m21), m22(m22), m23(m23), m24(m24),
          m31(m31), m32(m32), m33(m33), m34(m34),
          m41(m41), m42(m42), m43(m43), m44(m44)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4x4::float4x4(float3x2 value)
        : m11(value.m11), m12(value.m12), m13(0), m14(0),
          m21(value.m21), m22(value.m22), m23(0), m24(0),
          m31(0),         m32(0),         m33(1), m34(0),
          m41(value.m31), m42(value.m32), m43(0), m44(1)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ float4x4 float4x4::identity()
    {
        return float4x4(1, 0, 0, 0,
                        0, 1, 0, 0,
                        0, 0, 1, 0,
                        0, 0, 0, 1);
    }


    inline float4x4 make_float4x4_billboard(float3 const& objectPosition, float3 const& cameraPosition, float3 const& cameraUpVector, float3 const& cameraForwardVector)
    {
        const float epsilon = 1e-4f;

        float3 zaxis = objectPosition - cameraPosition;

        float norm = length_squared(zaxis);

        if (norm < epsilon)
        {
            zaxis = -cameraForwardVector;
        }
        else
        {
            zaxis = zaxis / sqrtf(norm);
        }

        float3 xaxis = normalize(cross(cameraUpVector, zaxis));

        float3 yaxis = cross(zaxis, xaxis);

        return float4x4(xaxis.x,          xaxis.y,          xaxis.z,          0,
                        yaxis.x,          yaxis.y,          yaxis.z,          0,
                        zaxis.x,          zaxis.y,          zaxis.z,          0,
                        objectPosition.x, objectPosition.y, objectPosition.z, 1);
    }


    inline float4x4 make_float4x4_constrained_billboard(float3 const& objectPosition, float3 const& cameraPosition, float3 const& rotateAxis, float3 const& cameraForwardVector, float3 const& objectForwardVector)
    {
        const float epsilon = 1e-4f;
        const float minAngle = 1.0f - (0.1f * (::DirectX::XM_PI / 180.0f)); // 0.1 degrees

        // Treat the case when object and camera positions are too close.
        float3 faceDir = objectPosition - cameraPosition;

        float norm = length_squared(faceDir);

        if (norm < epsilon)
        {
            faceDir = -cameraForwardVector;
        }
        else
        {
            faceDir = faceDir / sqrtf(norm);
        }

        float3 yaxis = rotateAxis;
        float3 xaxis;
        float3 zaxis;

        // Treat the case when angle between faceDir and rotateAxis is too close to 0.
        if (fabs(dot(rotateAxis, faceDir)) > minAngle)
        {
            zaxis = objectForwardVector;

            if (fabs(dot(rotateAxis, zaxis)) > minAngle)
            {
                zaxis = (fabs(rotateAxis.z) > minAngle) ? float3(1, 0, 0) : float3(0, 0, -1);
            }

            xaxis = normalize(cross(rotateAxis, zaxis));
            zaxis = normalize(cross(xaxis, rotateAxis));
        }
        else
        {
            xaxis = normalize(cross(rotateAxis, faceDir));
            zaxis = normalize(cross(xaxis, yaxis));
        }

        return float4x4(xaxis.x,          xaxis.y,          xaxis.z,          0,
                        yaxis.x,          yaxis.y,          yaxis.z,          0,
                        zaxis.x,          zaxis.y,          zaxis.z,          0,
                        objectPosition.x, objectPosition.y, objectPosition.z, 1);
    }


    inline float4x4 make_float4x4_translation(float3 const& position)
    {
        return float4x4(1, 0, 0, 0,
                        0, 1, 0, 0,
                        0, 0, 1, 0,
                        position.x, position.y, position.z, 1);
    }


    inline float4x4 make_float4x4_translation(float xPosition, float yPosition, float zPosition)
    {
        return float4x4(1, 0, 0, 0,
                        0, 1, 0, 0,
                        0, 0, 1, 0,
                        xPosition, yPosition, zPosition, 1);
    }


    inline float4x4 make_float4x4_scale(float xScale, float yScale, float zScale)
    {
        return float4x4(xScale, 0,      0,      0,
                        0,      yScale, 0,      0,
                        0,      0,      zScale, 0,
                        0,      0,      0,      1);
    }


    inline float4x4 make_float4x4_scale(float xScale, float yScale, float zScale, float3 const& centerPoint)
    { 
        float tx = centerPoint.x * (1 - xScale);
        float ty = centerPoint.y * (1 - yScale);
        float tz = centerPoint.z * (1 - zScale);

        return float4x4(xScale, 0,      0,      0,
                        0,      yScale, 0,      0,
                        0,      0,      zScale, 0,
                        tx,     ty,     tz,     1);
    }


    inline float4x4 make_float4x4_scale(float3 const& scales)
    {
        return float4x4(scales.x, 0,        0,        0,
                        0,        scales.y, 0,        0,
                        0,        0,        scales.z, 0,
                        0,        0,        0,        1);
    }


    inline float4x4 make_float4x4_scale(float3 const& scales, float3 const& centerPoint)
    {
        float3 t = centerPoint * (float3::one() - scales);

        return float4x4(scales.x, 0,        0,        0,
                        0,        scales.y, 0,        0,
                        0,        0,        scales.z, 0,
                        t.x,      t.y,      t.z,      1);
    }


    inline float4x4 make_float4x4_scale(float scale)
    {
        return float4x4(scale, 0,     0,     0,
                        0,     scale, 0,     0,
                        0,     0,     scale, 0,
                        0,     0,     0,     1);
    }


    inline float4x4 make_float4x4_scale(float scale, float3 const& centerPoint)
    {
        float3 t = centerPoint * (1 - scale);

        return float4x4(scale, 0,     0,     0,
                        0,     scale, 0,     0,
                        0,     0,     scale, 0,
                        t.x,   t.y,   t.z,   1);
    }


    inline float4x4 make_float4x4_rotation_x(float radians)
    {
        float c = cosf(radians);
        float s = sinf(radians);

        return float4x4(1,  0, 0, 0,
                        0,  c, s, 0,
                        0, -s, c, 0,
                        0,  0, 0, 1);
    }


    inline float4x4 make_float4x4_rotation_x(float radians, float3 const& centerPoint)
    {
        float c = cosf(radians);
        float s = sinf(radians);

        float y = centerPoint.y * (1 - c) + centerPoint.z * s;
        float z = centerPoint.z * (1 - c) - centerPoint.y * s;

        return float4x4(1,  0, 0, 0,
                        0,  c, s, 0,
                        0, -s, c, 0,
                        0,  y, z, 1);
    }


    inline float4x4 make_float4x4_rotation_y(float radians)
    {
        float c = cosf(radians);
        float s = sinf(radians);

        return float4x4(c, 0, -s, 0,
                        0, 1,  0, 0,
                        s, 0,  c, 0,
                        0, 0,  0, 1);
    }


    inline float4x4 make_float4x4_rotation_y(float radians, float3 const& centerPoint)
    {
        float c = cosf(radians);
        float s = sinf(radians);

        float x = centerPoint.x * (1 - c) - centerPoint.z * s;
        float z = centerPoint.z * (1 - c) + centerPoint.x * s;

        return float4x4(c, 0, -s, 0,
                        0, 1,  0, 0,
                        s, 0,  c, 0,
                        x, 0,  z, 1);
    }


    inline float4x4 make_float4x4_rotation_z(float radians)
    {
        float c = cosf(radians);
        float s = sinf(radians);

        return float4x4( c, s, 0, 0,
                        -s, c, 0, 0,
                         0, 0, 1, 0,
                         0, 0, 0, 1);
    }


    inline float4x4 make_float4x4_rotation_z(float radians, float3 const& centerPoint)
    {
        float c = cosf(radians);
        float s = sinf(radians);

        float x = centerPoint.x * (1 - c) + centerPoint.y * s;
        float y = centerPoint.y * (1 - c) - centerPoint.x * s;

        return float4x4( c, s, 0, 0,
                        -s, c, 0, 0,
                         0, 0, 1, 0,
                         x, y, 0, 1);
    }


    inline float4x4 make_float4x4_from_axis_angle(float3 const& axis, float angle)
    {
        // a: angle
        // x, y, z: unit vector for axis.
        //
        // Rotation matrix M can be computed by below equation.
        //
        //  M = uu + (cos a)(I-uu) + (sin a)S
        //
        // Where:
        //
        //  u = ( x, y, z )
        //
        //      [  0 -z  y ]
        //  S = [  z  0 -x ]
        //      [ -y  x  0 ]
        //
        //      [ 1 0 0 ]
        //  I = [ 0 1 0 ]
        //      [ 0 0 1 ]

        float x = axis.x, y = axis.y, z = axis.z;
        float sa = sinf(angle), ca = cosf(angle);
        float xx = x * x, yy = y * y, zz = z * z;
        float xy = x * y, xz = x * z, yz = y * z;

        return float4x4(xx + ca * (1 - xx),     xy - ca * xy + sa * z,  xz - ca * xz - sa * y,  0,
                        xy - ca * xy - sa * z,  yy + ca * (1 - yy),     yz - ca * yz + sa * x,  0,
                        xz - ca * xz + sa * y,  yz - ca * yz - sa * x,  zz + ca * (1 - zz),     0,
                        0,                      0,                      0,                      1);
    }


    inline float4x4 make_float4x4_perspective_field_of_view(float fieldOfView, float aspectRatio, float nearPlaneDistance, float farPlaneDistance)
    {
        if (fieldOfView <= 0.0f || fieldOfView >= ::DirectX::XM_PI)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("fieldOfView"));

        if (nearPlaneDistance <= 0.0f)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("nearPlaneDistance"));

        if (farPlaneDistance <= 0.0f)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("farPlaneDistance"));

        if (nearPlaneDistance >= farPlaneDistance )
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("nearPlaneDistance"));

        float yScale = 1.0f / tanf(fieldOfView * 0.5f);
        float xScale = yScale / aspectRatio;
        float clipDist = nearPlaneDistance - farPlaneDistance;

        return float4x4(xScale, 0,      0,                                                0,
                        0,      yScale, 0,                                                0,
                        0,      0,      farPlaneDistance / clipDist,                     -1,
                        0,      0,      nearPlaneDistance * farPlaneDistance / clipDist,  0);
    }


    inline float4x4 make_float4x4_perspective(float width, float height, float nearPlaneDistance, float farPlaneDistance)
    {
        if (nearPlaneDistance <= 0.0f)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("nearPlaneDistance"));

        if (farPlaneDistance <= 0.0f)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("farPlaneDistance"));

        if (nearPlaneDistance >= farPlaneDistance)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("nearPlaneDistance"));

        float clipDist = nearPlaneDistance - farPlaneDistance;

        return float4x4(2 * nearPlaneDistance / width, 0,                              0,                                                0,
                        0,                             2 * nearPlaneDistance / height, 0,                                                0,
                        0,                             0,                              farPlaneDistance / clipDist,                     -1,
                        0,                             0,                              nearPlaneDistance * farPlaneDistance / clipDist,  0);
    }


    inline float4x4 make_float4x4_perspective_off_center(float left, float right, float bottom, float top, float nearPlaneDistance, float farPlaneDistance)
    {
        if (nearPlaneDistance <= 0.0f)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("nearPlaneDistance"));

        if (farPlaneDistance <= 0.0f)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("farPlaneDistance"));

        if (nearPlaneDistance >= farPlaneDistance)
            _WINDOWS_NUMERICS_THROW_(std::invalid_argument("nearPlaneDistance"));

        float clipDist = nearPlaneDistance - farPlaneDistance;

        return float4x4(2 * nearPlaneDistance / (right - left), 0,                                      0,                                                0,
                        0,                                      2 * nearPlaneDistance / (top - bottom), 0,                                                0,
                        (left + right) / (right - left),        (top + bottom) / (top - bottom),        farPlaneDistance / clipDist,                     -1,
                        0,                                      0,                                      nearPlaneDistance * farPlaneDistance / clipDist,  0);
    }


    inline float4x4 make_float4x4_orthographic(float width, float height, float zNearPlane, float zFarPlane)
    {
        float clipDist = zNearPlane - zFarPlane;

        return float4x4(2 / width, 0,          0,                     0,
                        0,         2 / height, 0,                     0,
                        0,         0,          1 / clipDist,          0,
                        0,         0,          zNearPlane / clipDist, 1);
    }


    inline float4x4 make_float4x4_orthographic_off_center(float left, float right, float bottom, float top, float zNearPlane, float zFarPlane)
    {
        float clipDist = zNearPlane - zFarPlane;

        return float4x4(2 / (right - left),              0,                               0,                     0,
                        0,                               2 / (top - bottom),              0,                     0,
                        0,                               0,                               1 / clipDist,          0,
                        (left + right) / (left - right), (top + bottom) / (bottom - top), zNearPlane / clipDist, 1);
    }


    inline float4x4 make_float4x4_look_at(float3 const& cameraPosition, float3 const& cameraTarget, float3 const& cameraUpVector)
    {
        float3 zaxis = normalize(cameraPosition - cameraTarget);
        float3 xaxis = normalize(cross(cameraUpVector, zaxis));
        float3 yaxis = cross(zaxis, xaxis);

        return float4x4(xaxis.x,                     yaxis.x,                     zaxis.x,                     0,
                        xaxis.y,                     yaxis.y,                     zaxis.y,                     0,
                        xaxis.z,                     yaxis.z,                     zaxis.z,                     0,
                        -dot(xaxis, cameraPosition), -dot(yaxis, cameraPosition), -dot(zaxis, cameraPosition), 1);
    }


    inline float4x4 make_float4x4_world(float3 const& position, float3 const& forward, float3 const& up)
    {
        float3 zaxis = normalize(-forward);
        float3 xaxis = normalize(cross(up, zaxis));
        float3 yaxis = cross(zaxis, xaxis);

        return float4x4(xaxis.x,    xaxis.y,    xaxis.z,    0,
                        yaxis.x,    yaxis.y,    yaxis.z,    0,
                        zaxis.x,    zaxis.y,    zaxis.z,    0,
                        position.x, position.y, position.z, 1);
    }


    inline float4x4 make_float4x4_from_quaternion(quaternion const& quaternion)
    {
        float xx = quaternion.x * quaternion.x;
        float yy = quaternion.y * quaternion.y;
        float zz = quaternion.z * quaternion.z;

        float xy = quaternion.x * quaternion.y;
        float wz = quaternion.z * quaternion.w;
        float xz = quaternion.z * quaternion.x;
        float wy = quaternion.y * quaternion.w;
        float yz = quaternion.y * quaternion.z;
        float wx = quaternion.x * quaternion.w;

        return float4x4(1 - 2 * (yy + zz),  2 * (xy + wz),      2 * (xz - wy),      0,
                        2 * (xy - wz),      1 - 2 * (zz + xx),  2 * (yz + wx),      0,
                        2 * (xz + wy),      2 * (yz - wx),      1 - 2 * (yy + xx),  0,
                        0,                  0,                  0,                  1);
    }


    inline float4x4 make_float4x4_from_yaw_pitch_roll(float yaw, float pitch, float roll)
    {
        quaternion q = make_quaternion_from_yaw_pitch_roll(yaw, pitch, roll);

        return make_float4x4_from_quaternion(q);
    }


    inline float4x4 make_float4x4_shadow(float3 const& lightDirection, plane const& plane)
    {
        auto p = normalize(plane);

        float dot = dot_normal(p, lightDirection);

        float a = -p.normal.x;
        float b = -p.normal.y;
        float c = -p.normal.z;
        float d = -p.d;

        return float4x4(a * lightDirection.x + dot,  a * lightDirection.y,        a * lightDirection.z,        0,
                        b * lightDirection.x,        b * lightDirection.y + dot,  b * lightDirection.z,        0,
                        c * lightDirection.x,        c * lightDirection.y,        c * lightDirection.z + dot,  0,
                        d * lightDirection.x,        d * lightDirection.y,        d * lightDirection.z,        dot);
    }


    inline float4x4 make_float4x4_reflection(plane const& value)
    {
        plane v = normalize(value);

        float a = v.normal.x;
        float b = v.normal.y;
        float c = v.normal.z;

        float fa = -2 * a;
        float fb = -2 * b;
        float fc = -2 * c;

        return float4x4(fa * a + 1,  fb * a,      fc * a,      0,
                        fa * b,      fb * b + 1,  fc * b,      0,
                        fa * c,      fb * c,      fc * c + 1,  0,
                        fa * v.d,    fb * v.d,    fc * v.d,    1);
    }


    inline float4x4 operator +(float4x4 const& value1, float4x4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4x4(value1.m11 + value2.m11,  value1.m12 + value2.m12,  value1.m13 + value2.m13,  value1.m14 + value2.m14,
                        value1.m21 + value2.m21,  value1.m22 + value2.m22,  value1.m23 + value2.m23,  value1.m24 + value2.m24,
                        value1.m31 + value2.m31,  value1.m32 + value2.m32,  value1.m33 + value2.m33,  value1.m34 + value2.m34,
                        value1.m41 + value2.m41,  value1.m42 + value2.m42,  value1.m43 + value2.m43,  value1.m44 + value2.m44);
#else
        using namespace ::DirectX;

        float4x4 result;
        XMStoreFloat4x4(&result, XMLoadFloat4x4(&value1) + XMLoadFloat4x4(&value2));
        return result;
#endif
    }


    inline float4x4 operator -(float4x4 const& value1, float4x4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4x4(value1.m11 - value2.m11,  value1.m12 - value2.m12,  value1.m13 - value2.m13,  value1.m14 - value2.m14,
                        value1.m21 - value2.m21,  value1.m22 - value2.m22,  value1.m23 - value2.m23,  value1.m24 - value2.m24,
                        value1.m31 - value2.m31,  value1.m32 - value2.m32,  value1.m33 - value2.m33,  value1.m34 - value2.m34,
                        value1.m41 - value2.m41,  value1.m42 - value2.m42,  value1.m43 - value2.m43,  value1.m44 - value2.m44);
#else
        using namespace ::DirectX;

        float4x4 result;
        XMStoreFloat4x4(&result, XMLoadFloat4x4(&value1) - XMLoadFloat4x4(&value2));
        return result;
#endif
    }


    inline float4x4 operator *(float4x4 const& value1, float4x4 const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4x4
        (
            // First row
            value1.m11 * value2.m11 + value1.m12 * value2.m21 + value1.m13 * value2.m31 + value1.m14 * value2.m41,
            value1.m11 * value2.m12 + value1.m12 * value2.m22 + value1.m13 * value2.m32 + value1.m14 * value2.m42,
            value1.m11 * value2.m13 + value1.m12 * value2.m23 + value1.m13 * value2.m33 + value1.m14 * value2.m43,
            value1.m11 * value2.m14 + value1.m12 * value2.m24 + value1.m13 * value2.m34 + value1.m14 * value2.m44,

            // Second row
            value1.m21 * value2.m11 + value1.m22 * value2.m21 + value1.m23 * value2.m31 + value1.m24 * value2.m41,
            value1.m21 * value2.m12 + value1.m22 * value2.m22 + value1.m23 * value2.m32 + value1.m24 * value2.m42,
            value1.m21 * value2.m13 + value1.m22 * value2.m23 + value1.m23 * value2.m33 + value1.m24 * value2.m43,
            value1.m21 * value2.m14 + value1.m22 * value2.m24 + value1.m23 * value2.m34 + value1.m24 * value2.m44,

            // Third row
            value1.m31 * value2.m11 + value1.m32 * value2.m21 + value1.m33 * value2.m31 + value1.m34 * value2.m41,
            value1.m31 * value2.m12 + value1.m32 * value2.m22 + value1.m33 * value2.m32 + value1.m34 * value2.m42,
            value1.m31 * value2.m13 + value1.m32 * value2.m23 + value1.m33 * value2.m33 + value1.m34 * value2.m43,
            value1.m31 * value2.m14 + value1.m32 * value2.m24 + value1.m33 * value2.m34 + value1.m34 * value2.m44,

            // Fourth row
            value1.m41 * value2.m11 + value1.m42 * value2.m21 + value1.m43 * value2.m31 + value1.m44 * value2.m41,
            value1.m41 * value2.m12 + value1.m42 * value2.m22 + value1.m43 * value2.m32 + value1.m44 * value2.m42,
            value1.m41 * value2.m13 + value1.m42 * value2.m23 + value1.m43 * value2.m33 + value1.m44 * value2.m43,
            value1.m41 * value2.m14 + value1.m42 * value2.m24 + value1.m43 * value2.m34 + value1.m44 * value2.m44
        );
#else
        using namespace ::DirectX;

        float4x4 result;
        XMStoreFloat4x4(&result, XMMatrixMultiply(XMLoadFloat4x4(&value1), XMLoadFloat4x4(&value2)));
        return result;
#endif
    }


    inline float4x4 operator *(float4x4 const& value1, float value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4x4(value1.m11 * value2,  value1.m12 * value2,  value1.m13 * value2,  value1.m14 * value2,
                        value1.m21 * value2,  value1.m22 * value2,  value1.m23 * value2,  value1.m24 * value2,
                        value1.m31 * value2,  value1.m32 * value2,  value1.m33 * value2,  value1.m34 * value2,
                        value1.m41 * value2,  value1.m42 * value2,  value1.m43 * value2,  value1.m44 * value2);
#else
        using namespace ::DirectX;

        float4x4 result;
        XMStoreFloat4x4(&result, XMLoadFloat4x4(&value1) * value2);
        return result;
#endif
    }


    inline float4x4 operator -(float4x4 const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4x4(-value.m11, -value.m12, -value.m13, -value.m14,
                        -value.m21, -value.m22, -value.m23, -value.m24,
                        -value.m31, -value.m32, -value.m33, -value.m34,
                        -value.m41, -value.m42, -value.m43, -value.m44);
#else
        using namespace ::DirectX;

        float4x4 result;
        XMStoreFloat4x4(&result, -XMLoadFloat4x4(&value));
        return result;
#endif
    }


    inline float4x4& operator +=(float4x4& value1, float4x4 const& value2)
    {
        value1 = value1 + value2;

        return value1;
    }


    inline float4x4& operator -=(float4x4& value1, float4x4 const& value2)
    {
        value1 = value1 - value2;

        return value1;
    }


    inline float4x4& operator *=(float4x4& value1, float4x4 const& value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline float4x4& operator *=(float4x4& value1, float value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline bool operator ==(float4x4 const& value1, float4x4 const& value2)
    {
        return value1.m11 == value2.m11 && value1.m22 == value2.m22 && value1.m33 == value2.m33 && value1.m44 == value2.m44 && // Check diagonal element first for early out.
                                           value1.m12 == value2.m12 && value1.m13 == value2.m13 && value1.m14 == value2.m14 &&
               value1.m21 == value2.m21                             && value1.m23 == value2.m23 && value1.m24 == value2.m24 &&
               value1.m31 == value2.m31 && value1.m32 == value2.m32                             && value1.m34 == value2.m34 &&
               value1.m41 == value2.m41 && value1.m42 == value2.m42 && value1.m43 == value2.m43;
    }


    inline bool operator !=(float4x4 const& value1, float4x4 const& value2)
    {
        return value1.m11 != value2.m11 || value1.m12 != value2.m12 || value1.m13 != value2.m13 || value1.m14 != value2.m14 ||
               value1.m21 != value2.m21 || value1.m22 != value2.m22 || value1.m23 != value2.m23 || value1.m24 != value2.m24 ||
               value1.m31 != value2.m31 || value1.m32 != value2.m32 || value1.m33 != value2.m33 || value1.m34 != value2.m34 ||
               value1.m41 != value2.m41 || value1.m42 != value2.m42 || value1.m43 != value2.m43 || value1.m44 != value2.m44;
    }


    inline bool is_identity(float4x4 const& value)
    {
        return value.m11 == 1 && value.m22 == 1 && value.m33 == 1 && value.m44 == 1 && // Check diagonal element first for early out.
                                 value.m12 == 0 && value.m13 == 0 && value.m14 == 0 &&
               value.m21 == 0                   && value.m23 == 0 && value.m24 == 0 &&
               value.m31 == 0 && value.m32 == 0                   && value.m34 == 0 &&
               value.m41 == 0 && value.m42 == 0 && value.m43 == 0;
    }


    inline float determinant(float4x4 const& value)
    {
        // | a b c d |     | f g h |     | e g h |     | e f h |     | e f g |
        // | e f g h | = a | j k l | - b | i k l | + c | i j l | - d | i j k |
        // | i j k l |     | n o p |     | m o p |     | m n p |     | m n o |
        // | m n o p |
        //
        //   | f g h |
        // a | j k l | = a ( f ( kp - lo ) - g ( jp - ln ) + h ( jo - kn ) )
        //   | n o p |
        //
        //   | e g h |     
        // b | i k l | = b ( e ( kp - lo ) - g ( ip - lm ) + h ( io - km ) )
        //   | m o p |     
        //
        //   | e f h |
        // c | i j l | = c ( e ( jp - ln ) - f ( ip - lm ) + h ( in - jm ) )
        //   | m n p |
        //
        //   | e f g |
        // d | i j k | = d ( e ( jo - kn ) - f ( io - km ) + g ( in - jm ) )
        //   | m n o |

        float a = value.m11, b = value.m12, c = value.m13, d = value.m14;
        float e = value.m21, f = value.m22, g = value.m23, h = value.m24;
        float i = value.m31, j = value.m32, k = value.m33, l = value.m34;
        float m = value.m41, n = value.m42, o = value.m43, p = value.m44;

        float kp_lo = k * p - l * o;
        float jp_ln = j * p - l * n;
        float jo_kn = j * o - k * n;
        float ip_lm = i * p - l * m;
        float io_km = i * o - k * m;
        float in_jm = i * n - j * m;

        return a * (f * kp_lo - g * jp_ln + h * jo_kn) -
               b * (e * kp_lo - g * ip_lm + h * io_km) +
               c * (e * jp_ln - f * ip_lm + h * in_jm) -
               d * (e * jo_kn - f * io_km + g * in_jm);
    }


    inline float3 translation(float4x4 const& value)
    {
        return float3(value.m41, value.m42, value.m43);
    }


    inline bool invert(float4x4 const& matrix, _Out_ float4x4* result)
    {
        //                                       -1
        // If you have matrix M, inverse Matrix M   can compute
        //
        //     -1       1      
        //    M   = --------- A
        //            det(M)
        //
        // A is adjugate (adjoint) of M, where,
        //
        //      T
        // A = C
        //
        // C is Cofactor matrix of M, where,
        //           i + j
        // C   = (-1)      * det(M  )
        //  ij                    ij
        //
        //     [ a b c d ]
        // M = [ e f g h ]
        //     [ i j k l ]
        //     [ m n o p ]
        //
        // First Row
        //           2 | f g h |
        // C   = (-1)  | j k l | = + ( f ( kp - lo ) - g ( jp - ln ) + h ( jo - kn ) )
        //  11         | n o p |
        //
        //           3 | e g h |
        // C   = (-1)  | i k l | = - ( e ( kp - lo ) - g ( ip - lm ) + h ( io - km ) )
        //  12         | m o p |
        //
        //           4 | e f h |
        // C   = (-1)  | i j l | = + ( e ( jp - ln ) - f ( ip - lm ) + h ( in - jm ) )
        //  13         | m n p |
        //
        //           5 | e f g |
        // C   = (-1)  | i j k | = - ( e ( jo - kn ) - f ( io - km ) + g ( in - jm ) )
        //  14         | m n o |
        //
        // Second Row
        //           3 | b c d |
        // C   = (-1)  | j k l | = - ( b ( kp - lo ) - c ( jp - ln ) + d ( jo - kn ) )
        //  21         | n o p |
        //
        //           4 | a c d |
        // C   = (-1)  | i k l | = + ( a ( kp - lo ) - c ( ip - lm ) + d ( io - km ) )
        //  22         | m o p |
        //
        //           5 | a b d |
        // C   = (-1)  | i j l | = - ( a ( jp - ln ) - b ( ip - lm ) + d ( in - jm ) )
        //  23         | m n p |
        //
        //           6 | a b c |
        // C   = (-1)  | i j k | = + ( a ( jo - kn ) - b ( io - km ) + c ( in - jm ) )
        //  24         | m n o |
        //
        // Third Row
        //           4 | b c d |
        // C   = (-1)  | f g h | = + ( b ( gp - ho ) - c ( fp - hn ) + d ( fo - gn ) )
        //  31         | n o p |
        //
        //           5 | a c d |
        // C   = (-1)  | e g h | = - ( a ( gp - ho ) - c ( ep - hm ) + d ( eo - gm ) )
        //  32         | m o p |
        //
        //           6 | a b d |
        // C   = (-1)  | e f h | = + ( a ( fp - hn ) - b ( ep - hm ) + d ( en - fm ) )
        //  33         | m n p |
        //
        //           7 | a b c |
        // C   = (-1)  | e f g | = - ( a ( fo - gn ) - b ( eo - gm ) + c ( en - fm ) )
        //  34         | m n o |
        //
        // Fourth Row
        //           5 | b c d |
        // C   = (-1)  | f g h | = - ( b ( gl - hk ) - c ( fl - hj ) + d ( fk - gj ) )
        //  41         | j k l |
        //
        //           6 | a c d |
        // C   = (-1)  | e g h | = + ( a ( gl - hk ) - c ( el - hi ) + d ( ek - gi ) )
        //  42         | i k l |
        //
        //           7 | a b d |
        // C   = (-1)  | e f h | = - ( a ( fl - hj ) - b ( el - hi ) + d ( ej - fi ) )
        //  43         | i j l |
        //
        //           8 | a b c |
        // C   = (-1)  | e f g | = + ( a ( fk - gj ) - b ( ek - gi ) + c ( ej - fi ) )
        //  44         | i j k |
        //
        float a = matrix.m11, b = matrix.m12, c = matrix.m13, d = matrix.m14;
        float e = matrix.m21, f = matrix.m22, g = matrix.m23, h = matrix.m24;
        float i = matrix.m31, j = matrix.m32, k = matrix.m33, l = matrix.m34;
        float m = matrix.m41, n = matrix.m42, o = matrix.m43, p = matrix.m44;

        float kp_lo = k * p - l * o;
        float jp_ln = j * p - l * n;
        float jo_kn = j * o - k * n;
        float ip_lm = i * p - l * m;
        float io_km = i * o - k * m;
        float in_jm = i * n - j * m;

        float a11 = +(f * kp_lo - g * jp_ln + h * jo_kn);
        float a12 = -(e * kp_lo - g * ip_lm + h * io_km);
        float a13 = +(e * jp_ln - f * ip_lm + h * in_jm);
        float a14 = -(e * jo_kn - f * io_km + g * in_jm);

        float det = a * a11 + b * a12 + c * a13 + d * a14;

        // NaN safe
        if (!(fabs(det) >= FLT_EPSILON))
        {
            const float nan = _WINDOWS_NUMERICS_NAN_;

            *result = float4x4(nan, nan, nan, nan,
                               nan, nan, nan, nan,
                               nan, nan, nan, nan,
                               nan, nan, nan, nan);
            return false;
        }

        float invDet = 1.0f / det;

        result->m11 = a11 * invDet;
        result->m21 = a12 * invDet;
        result->m31 = a13 * invDet;
        result->m41 = a14 * invDet;

        result->m12 = -(b * kp_lo - c * jp_ln + d * jo_kn) * invDet;
        result->m22 = +(a * kp_lo - c * ip_lm + d * io_km) * invDet;
        result->m32 = -(a * jp_ln - b * ip_lm + d * in_jm) * invDet;
        result->m42 = +(a * jo_kn - b * io_km + c * in_jm) * invDet;

        float gp_ho = g * p - h * o;
        float fp_hn = f * p - h * n;
        float fo_gn = f * o - g * n;
        float ep_hm = e * p - h * m;
        float eo_gm = e * o - g * m;
        float en_fm = e * n - f * m;

        result->m13 = +(b * gp_ho - c * fp_hn + d * fo_gn) * invDet;
        result->m23 = -(a * gp_ho - c * ep_hm + d * eo_gm) * invDet;
        result->m33 = +(a * fp_hn - b * ep_hm + d * en_fm) * invDet;
        result->m43 = -(a * fo_gn - b * eo_gm + c * en_fm) * invDet;

        float gl_hk = g * l - h * k;
        float fl_hj = f * l - h * j;
        float fk_gj = f * k - g * j;
        float el_hi = e * l - h * i;
        float ek_gi = e * k - g * i;
        float ej_fi = e * j - f * i;

        result->m14 = -(b * gl_hk - c * fl_hj + d * fk_gj) * invDet;
        result->m24 = +(a * gl_hk - c * el_hi + d * ek_gi) * invDet;
        result->m34 = -(a * fl_hj - b * el_hi + d * ej_fi) * invDet;
        result->m44 = +(a * fk_gj - b * ek_gi + c * ej_fi) * invDet;

        return true;
    }


    _Success_(return) inline bool decompose(_In_ float4x4 const& matrix, _Inout_ float3* scale, _Inout_ quaternion* rotation, _Inout_ float3* translation)
    {
        using namespace ::DirectX;

        XMVECTOR s, r, t;

        if (!XMMatrixDecompose(&s, &r, &t, XMLoadFloat4x4(&matrix)))
        {
            return false;
        }

        XMStoreFloat3(scale, s);
        XMStoreQuaternion(rotation, r);
        XMStoreFloat3(translation, t);

        return true;
    }


    inline float4x4 transform(float4x4 const& value, quaternion const& rotation)
    {
        // Compute rotation matrix.
        float x2 = rotation.x + rotation.x;
        float y2 = rotation.y + rotation.y;
        float z2 = rotation.z + rotation.z;

        float wx2 = rotation.w * x2;
        float wy2 = rotation.w * y2;
        float wz2 = rotation.w * z2;
        float xx2 = rotation.x * x2;
        float xy2 = rotation.x * y2;
        float xz2 = rotation.x * z2;
        float yy2 = rotation.y * y2;
        float yz2 = rotation.y * z2;
        float zz2 = rotation.z * z2;

        float q11 = 1 - yy2 - zz2;
        float q21 = xy2 - wz2;
        float q31 = xz2 + wy2;

        float q12 = xy2 + wz2;
        float q22 = 1 - xx2 - zz2;
        float q32 = yz2 - wx2;

        float q13 = xz2 - wy2;
        float q23 = yz2 + wx2;
        float q33 = 1 - xx2 - yy2;

        return float4x4
        (
            // First row
            value.m11 * q11 + value.m12 * q21 + value.m13 * q31,
            value.m11 * q12 + value.m12 * q22 + value.m13 * q32,
            value.m11 * q13 + value.m12 * q23 + value.m13 * q33,
            value.m14,

            // Second row
            value.m21 * q11 + value.m22 * q21 + value.m23 * q31,
            value.m21 * q12 + value.m22 * q22 + value.m23 * q32,
            value.m21 * q13 + value.m22 * q23 + value.m23 * q33,
            value.m24,

            // Third row
            value.m31 * q11 + value.m32 * q21 + value.m33 * q31,
            value.m31 * q12 + value.m32 * q22 + value.m33 * q32,
            value.m31 * q13 + value.m32 * q23 + value.m33 * q33,
            value.m34,

            // Fourth row
            value.m41 * q11 + value.m42 * q21 + value.m43 * q31,
            value.m41 * q12 + value.m42 * q22 + value.m43 * q32,
            value.m41 * q13 + value.m42 * q23 + value.m43 * q33,
            value.m44
        );
    }


    inline float4x4 transpose(float4x4 const& matrix)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return float4x4(matrix.m11, matrix.m21, matrix.m31, matrix.m41,
                        matrix.m12, matrix.m22, matrix.m32, matrix.m42,
                        matrix.m13, matrix.m23, matrix.m33, matrix.m43,
                        matrix.m14, matrix.m24, matrix.m34, matrix.m44);
#else
        using namespace ::DirectX;

        float4x4 result;
        XMStoreFloat4x4(&result, XMMatrixTranspose(XMLoadFloat4x4(&matrix)));
        return result;
#endif
    }


    inline float4x4 lerp(float4x4 const& matrix1, float4x4 const& matrix2, float amount)
    {
        return matrix1 + (matrix2 - matrix1) * amount;
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ plane::plane(float x, float y, float z, float d)
        : normal(x, y, z), d(d)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ plane::plane(float3 normal, float d)
        : normal(normal), d(d)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ plane::plane(float4 value)
        : normal(value.x, value.y, value.z), d(value.w)
    { }


    inline plane make_plane_from_vertices(float3 const& point1, float3 const& point2, float3 const& point3)
    {
        float3 a = point2 - point1;
        float3 b = point3 - point1;

        float3 normal = normalize(cross(a, b));
        float d = -dot(normal, point1);

        return plane(normal, d);
    }


    inline bool operator ==(plane const& value1, plane const& value2)
    {
        return value1.normal.x == value2.normal.x &&
               value1.normal.y == value2.normal.y &&
               value1.normal.z == value2.normal.z &&
               value1.d == value2.d;
    }


    inline bool operator !=(plane const& value1, plane const& value2)
    {
        return value1.normal.x != value2.normal.x ||
               value1.normal.y != value2.normal.y || 
               value1.normal.z != value2.normal.z ||
               value1.d != value2.d;
    }


    inline plane normalize(plane const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        float f = length_squared(value.normal);

        if (fabs(f - 1.0f) < FLT_EPSILON)
        {
            return value;
        }

        float fInv = 1.0f / sqrtf(f);

        return plane(value.normal * fInv, value.d * fInv);
#else
        using namespace ::DirectX;

        plane result;
        XMStorePlane(&result, XMPlaneNormalize(XMLoadPlane(&value)));
        return result;
#endif
    }


    inline plane transform(plane const& value, float4x4 const& matrix)
    {
        float4x4 inverseMatrix;
        invert(matrix, &inverseMatrix);

#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        float4 planeAsVector(value.normal, value.d);

        return plane(transform(planeAsVector, transpose(inverseMatrix)));
#else
        using namespace ::DirectX;

        plane result;
        XMStorePlane(&result, XMPlaneTransform(XMLoadPlane(&value), XMMatrixTranspose(XMLoadFloat4x4(&inverseMatrix))));
        return result;
#endif
    }


    inline plane transform(plane const& value, quaternion const& rotation)
    {
        float4 planeAsVector(value.normal, value.d);

        return plane(transform(planeAsVector, rotation));
    }


    inline float dot(plane const& plane, float4 const& value)
    {
        return plane.normal.x * value.x +
               plane.normal.y * value.y + 
               plane.normal.z * value.z + 
               plane.d * value.w;
    }


    inline float dot_coordinate(plane const& plane, float3 const& value)
    {
        return plane.normal.x * value.x +
               plane.normal.y * value.y + 
               plane.normal.z * value.z + 
               plane.d;
    }


    inline float dot_normal(plane const& plane, float3 const& value)
    {
        return plane.normal.x * value.x +
               plane.normal.y * value.y + 
               plane.normal.z * value.z;
    }


    _WINDOWS_NUMERICS_CONSTEXPR_ quaternion::quaternion(float x, float y, float z, float w)
        : x(x), y(y), z(z), w(w)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ quaternion::quaternion(float3 vectorPart, float scalarPart)
        : x(vectorPart.x), y(vectorPart.y), z(vectorPart.z), w(scalarPart)
    { }


    _WINDOWS_NUMERICS_CONSTEXPR_ quaternion quaternion::identity()
    {
        return quaternion(0, 0, 0, 1);
    }


    inline quaternion make_quaternion_from_axis_angle(float3 const& axis, float angle)
    {
        float halfAngle = angle * 0.5f;
        float s = sinf(halfAngle);
        float c = cosf(halfAngle);

        return quaternion(axis * s, c);
    }


    inline quaternion make_quaternion_from_yaw_pitch_roll(float yaw, float pitch, float roll)
    {
        // Roll first, about axis the object is facing, then
        // pitch upward, then yaw to face into the new heading
        float sr, cr, sp, cp, sy, cy;

        float halfRoll = roll * 0.5f;
        sr = sinf(halfRoll); cr = cosf(halfRoll);

        float halfPitch = pitch * 0.5f;
        sp = sinf(halfPitch); cp = cosf(halfPitch);

        float halfYaw = yaw * 0.5f;
        sy = sinf(halfYaw); cy = cosf(halfYaw);

        return quaternion(cy * sp * cr + sy * cp * sr,
                          sy * cp * cr - cy * sp * sr,
                          cy * cp * sr - sy * sp * cr,
                          cy * cp * cr + sy * sp * sr);
    }


    inline quaternion make_quaternion_from_rotation_matrix(float4x4 const& matrix)
    {
        if (matrix.m11 + matrix.m22 + matrix.m33 > 0.0f)
        {
            float s = sqrtf(1.0f + matrix.m11 + matrix.m22 + matrix.m33);
            float invS = 0.5f / s;

            return quaternion((matrix.m23 - matrix.m32) * invS,
                              (matrix.m31 - matrix.m13) * invS,
                              (matrix.m12 - matrix.m21) * invS,
                              s * 0.5f);
        }
        else if (matrix.m11 >= matrix.m22 && matrix.m11 >= matrix.m33)
        {
            float s = sqrtf(1.0f + matrix.m11 - matrix.m22 - matrix.m33);
            float invS = 0.5f / s;

            return quaternion(0.5f * s,
                              (matrix.m12 + matrix.m21) * invS,
                              (matrix.m13 + matrix.m31) * invS,
                              (matrix.m23 - matrix.m32) * invS);
        }
        else if (matrix.m22 > matrix.m33)
        {
            float s = sqrtf(1.0f + matrix.m22 - matrix.m11 - matrix.m33);
            float invS = 0.5f / s;

            return quaternion((matrix.m21 + matrix.m12) * invS,
                              0.5f * s,
                              (matrix.m32 + matrix.m23) * invS,
                              (matrix.m31 - matrix.m13) * invS);
        }
        else
        {
            float s = sqrtf(1.0f + matrix.m33 - matrix.m11 - matrix.m22);
            float invS = 0.5f / s;

            return quaternion((matrix.m31 + matrix.m13) * invS,
                              (matrix.m32 + matrix.m23) * invS,
                              0.5f * s,
                              (matrix.m12 - matrix.m21) * invS);
        }
    }


    inline quaternion operator +(quaternion const& value1, quaternion const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return quaternion(value1.x + value2.x,
                          value1.y + value2.y,
                          value1.z + value2.z,
                          value1.w + value2.w);
#else
        using namespace ::DirectX;

        quaternion result;
        XMStoreQuaternion(&result, XMVectorAdd(XMLoadQuaternion(&value1), XMLoadQuaternion(&value2)));
        return result;
#endif
    }


    inline quaternion operator -(quaternion const& value1, quaternion const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return quaternion(value1.x - value2.x,
                          value1.y - value2.y,
                          value1.z - value2.z,
                          value1.w - value2.w);
#else
        using namespace ::DirectX;

        quaternion result;
        XMStoreQuaternion(&result, XMVectorSubtract(XMLoadQuaternion(&value1), XMLoadQuaternion(&value2)));
        return result;
#endif
    }


    inline quaternion operator *(quaternion const& value1, quaternion const& value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        float3 q1(value1.x, value1.y, value1.z);
        float3 q2(value2.x, value2.y, value2.z);

        float3 c = cross(q1, q2);
        float d = dot(q1, q2);

        return quaternion(q1 * value2.w + q2 * value1.w + c,
                          value1.w * value2.w - d);
#else
        using namespace ::DirectX;

        quaternion result;
        XMStoreQuaternion(&result, XMQuaternionMultiply(XMLoadQuaternion(&value2), XMLoadQuaternion(&value1)));
        return result;
#endif
    }


    inline quaternion operator *(quaternion const& value1, float value2)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return quaternion(value1.x * value2,
                          value1.y * value2,
                          value1.z * value2,
                          value1.w * value2);
#else
        using namespace ::DirectX;

        quaternion result;
        XMStoreQuaternion(&result, XMVectorScale(XMLoadQuaternion(&value1), value2));
        return result;
#endif
    }


    inline quaternion operator /(quaternion const& value1, quaternion const& value2)
    {
        return value1 * inverse(value2);
    }


    inline quaternion operator -(quaternion const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return quaternion(-value.x,
                          -value.y,
                          -value.z,
                          -value.w);
#else
        using namespace ::DirectX;

        quaternion result;
        XMStoreQuaternion(&result, XMVectorNegate(XMLoadQuaternion(&value)));
        return result;
#endif
    }


    inline quaternion& operator +=(quaternion& value1, quaternion const& value2)
    {
        value1 = value1 + value2;

        return value1;
    }


    inline quaternion& operator -=(quaternion& value1, quaternion const& value2)
    {
        value1 = value1 - value2;

        return value1;
    }


    inline quaternion& operator *=(quaternion& value1, quaternion const& value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline quaternion& operator *=(quaternion& value1, float value2)
    {
        value1 = value1 * value2;

        return value1;
    }


    inline quaternion& operator /=(quaternion& value1, quaternion const& value2)
    {
        value1 = value1 / value2;

        return value1;
    }


    inline bool operator ==(quaternion const& value1, quaternion const& value2)
    {
        return value1.x == value2.x &&
               value1.y == value2.y &&
               value1.z == value2.z &&
               value1.w == value2.w;
    }


    inline bool operator !=(quaternion const& value1, quaternion const& value2)
    {
        return value1.x != value2.x ||
               value1.y != value2.y ||
               value1.z != value2.z ||
               value1.w != value2.w;
    }


    inline bool is_identity(quaternion const& value)
    {
        return value.x == 0 &&
               value.y == 0 &&
               value.z == 0 &&
               value.w == 1;
    }


    inline float length(quaternion const& value)
    {
        return sqrtf(length_squared(value));
    }


    inline float length_squared(quaternion const& value)
    {
        return dot(value, value);
    }


    inline float dot(quaternion const& quaternion1, quaternion const& quaternion2)
    {
        return quaternion1.x * quaternion2.x +
               quaternion1.y * quaternion2.y + 
               quaternion1.z * quaternion2.z + 
               quaternion1.w * quaternion2.w;
    }


    inline quaternion normalize(quaternion const& value)
    {
        return value * (1.0f / length(value));
    }


    inline quaternion conjugate(quaternion const& value)
    {
#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return quaternion(-value.x,
                          -value.y,
                          -value.z,
                           value.w);
#else
        using namespace ::DirectX;

        quaternion result;
        XMStoreQuaternion(&result, XMQuaternionConjugate(XMLoadQuaternion(&value)));
        return result;
#endif
    }


    inline quaternion inverse(quaternion const& value)
    {
        return conjugate(value * (1.0f / length_squared(value)));
    }


    inline quaternion slerp(quaternion const& quaternion1, quaternion const& quaternion2, float amount)
    {
        const float epsilon = 1e-6f;

        float t = amount;
        float cosOmega = dot(quaternion1, quaternion2);
        bool flip = false;
        
        if (cosOmega < 0.0f)
        {
            flip = true;
            cosOmega = -cosOmega;
        }

        float s1, s2;
        
        if (cosOmega > (1.0f - epsilon))
        {
            // Too close, do straight linear interpolation.
            s1 = 1.0f - t;
            s2 = flip ? -t : t;
        }
        else
        {
            float omega = acosf(cosOmega);
            float invSinOmega = 1.0f / sinf(omega);

            s1 = sinf((1.0f - t) * omega) * invSinOmega;
            s2 = flip ? -sinf(t * omega) * invSinOmega
                      :  sinf(t * omega) * invSinOmega;
        }

#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return quaternion(s1 * quaternion1.x + s2 * quaternion2.x,
                          s1 * quaternion1.y + s2 * quaternion2.y,
                          s1 * quaternion1.z + s2 * quaternion2.z,
                          s1 * quaternion1.w + s2 * quaternion2.w);
#else
        using namespace ::DirectX;

        XMVECTOR q1 = XMVectorScale(XMLoadQuaternion(&quaternion1), s1);
        XMVECTOR q2 = XMVectorScale(XMLoadQuaternion(&quaternion2), s2);

        quaternion result;
        XMStoreQuaternion(&result, XMVectorAdd(q1, q2));
        return result;
#endif
    }


    inline quaternion lerp(quaternion const& quaternion1, quaternion const& quaternion2, float amount)
    {
        float t2 = amount;
        float t1 = 1.0f - amount;

        if (dot(quaternion1, quaternion2) < 0.0f)
        {
            t2 = -t2;
        }

#ifdef WINDOWS_NUMERICS_DISABLE_SIMD
        return normalize(quaternion(t1 * quaternion1.x + t2 * quaternion2.x,
                                    t1 * quaternion1.y + t2 * quaternion2.y,
                                    t1 * quaternion1.z + t2 * quaternion2.z,
                                    t1 * quaternion1.w + t2 * quaternion2.w));
#else
        using namespace ::DirectX;

        XMVECTOR q1 = XMVectorScale(XMLoadQuaternion(&quaternion1), t1);
        XMVECTOR q2 = XMVectorScale(XMLoadQuaternion(&quaternion2), t2);

        quaternion result;
        XMStoreQuaternion(&result, XMQuaternionNormalize(XMVectorAdd(q1, q2)));
        return result;
#endif
    }


    inline quaternion concatenate(quaternion const& value1, quaternion const& value2)
    {
        return value2 * value1;
    }
}
_WINDOWS_NUMERICS_END_NAMESPACE_


#undef _WINDOWS_NUMERICS_THROW_
#undef _WINDOWS_NUMERICS_NAN_

#pragma warning(pop)
