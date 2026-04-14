//-------------------------------------------------------------------------------------
// DirectXCollision.inl -- C++ Collision Math library
//
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
//
// http://go.microsoft.com/fwlink/?LinkID=615560
//-------------------------------------------------------------------------------------

#pragma once

XMGLOBALCONST XMVECTORF32 g_BoxOffset[8] =
{
    { { { -1.0f, -1.0f,  1.0f, 0.0f } } },
    { { {  1.0f, -1.0f,  1.0f, 0.0f } } },
    { { {  1.0f,  1.0f,  1.0f, 0.0f } } },
    { { { -1.0f,  1.0f,  1.0f, 0.0f } } },
    { { { -1.0f, -1.0f, -1.0f, 0.0f } } },
    { { {  1.0f, -1.0f, -1.0f, 0.0f } } },
    { { {  1.0f,  1.0f, -1.0f, 0.0f } } },
    { { { -1.0f,  1.0f, -1.0f, 0.0f } } },
};

XMGLOBALCONST XMVECTORF32 g_RayEpsilon = { { { 1e-20f, 1e-20f, 1e-20f, 1e-20f } } };
XMGLOBALCONST XMVECTORF32 g_RayNegEpsilon = { { { -1e-20f, -1e-20f, -1e-20f, -1e-20f } } };
XMGLOBALCONST XMVECTORF32 g_FltMin = { { { -FLT_MAX, -FLT_MAX, -FLT_MAX, -FLT_MAX } } };
XMGLOBALCONST XMVECTORF32 g_FltMax = { { { FLT_MAX, FLT_MAX, FLT_MAX, FLT_MAX } } };

namespace Internal
{

    //-----------------------------------------------------------------------------
    // Return true if any of the elements of a 3 vector are equal to 0xffffffff.
    // Slightly more efficient than using XMVector3EqualInt.
    //-----------------------------------------------------------------------------
    inline bool XMVector3AnyTrue(_In_ FXMVECTOR V) noexcept
    {
        // Duplicate the fourth element from the first element.
        XMVECTOR C = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_X>(V);

        return XMComparisonAnyTrue(XMVector4EqualIntR(C, XMVectorTrueInt()));
    }


    //-----------------------------------------------------------------------------
    // Return true if all of the elements of a 3 vector are equal to 0xffffffff.
    // Slightly more efficient than using XMVector3EqualInt.
    //-----------------------------------------------------------------------------
    inline bool XMVector3AllTrue(_In_ FXMVECTOR V) noexcept
    {
        // Duplicate the fourth element from the first element.
        XMVECTOR C = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_X>(V);

        return XMComparisonAllTrue(XMVector4EqualIntR(C, XMVectorTrueInt()));
    }

#if defined(_PREFAST_) || !defined(NDEBUG)

    XMGLOBALCONST XMVECTORF32 g_UnitVectorEpsilon = { { { 1.0e-4f, 1.0e-4f, 1.0e-4f, 1.0e-4f } } };
    XMGLOBALCONST XMVECTORF32 g_UnitQuaternionEpsilon = { { { 1.0e-4f, 1.0e-4f, 1.0e-4f, 1.0e-4f } } };
    XMGLOBALCONST XMVECTORF32 g_UnitPlaneEpsilon = { { { 1.0e-4f, 1.0e-4f, 1.0e-4f, 1.0e-4f } } };

    //-----------------------------------------------------------------------------
    // Return true if the vector is a unit vector (length == 1).
    //-----------------------------------------------------------------------------
    inline bool XMVector3IsUnit(_In_ FXMVECTOR V) noexcept
    {
        XMVECTOR Difference = XMVectorSubtract(XMVector3Length(V), XMVectorSplatOne());
        return XMVector4Less(XMVectorAbs(Difference), g_UnitVectorEpsilon);
    }

    //-----------------------------------------------------------------------------
    // Return true if the quaterion is a unit quaternion.
    //-----------------------------------------------------------------------------
    inline bool XMQuaternionIsUnit(_In_ FXMVECTOR Q) noexcept
    {
        XMVECTOR Difference = XMVectorSubtract(XMVector4Length(Q), XMVectorSplatOne());
        return XMVector4Less(XMVectorAbs(Difference), g_UnitQuaternionEpsilon);
    }

    //-----------------------------------------------------------------------------
    // Return true if the plane is a unit plane.
    //-----------------------------------------------------------------------------
    inline bool XMPlaneIsUnit(_In_ FXMVECTOR Plane) noexcept
    {
        XMVECTOR Difference = XMVectorSubtract(XMVector3Length(Plane), XMVectorSplatOne());
        return XMVector4Less(XMVectorAbs(Difference), g_UnitPlaneEpsilon);
    }

#endif // _PREFAST_ || !NDEBUG

    //-----------------------------------------------------------------------------
    inline XMVECTOR XMPlaneTransform(_In_ FXMVECTOR Plane, _In_ FXMVECTOR Rotation, _In_ FXMVECTOR Translation) noexcept
    {
        XMVECTOR vNormal = XMVector3Rotate(Plane, Rotation);
        XMVECTOR vD = XMVectorSubtract(XMVectorSplatW(Plane), XMVector3Dot(vNormal, Translation));

        return XMVectorInsert<0, 0, 0, 0, 1>(vNormal, vD);
    }

    //-----------------------------------------------------------------------------
    // Return the point on the line segement (S1, S2) nearest the point P.
    //-----------------------------------------------------------------------------
    inline XMVECTOR PointOnLineSegmentNearestPoint(_In_ FXMVECTOR S1, _In_ FXMVECTOR S2, _In_ FXMVECTOR P) noexcept
    {
        XMVECTOR Dir = XMVectorSubtract(S2, S1);
        XMVECTOR Projection = XMVectorSubtract(XMVector3Dot(P, Dir), XMVector3Dot(S1, Dir));
        XMVECTOR LengthSq = XMVector3Dot(Dir, Dir);

        XMVECTOR t = XMVectorMultiply(Projection, XMVectorReciprocal(LengthSq));
        XMVECTOR Point = XMVectorMultiplyAdd(t, Dir, S1);

        // t < 0
        XMVECTOR SelectS1 = XMVectorLess(Projection, XMVectorZero());
        Point = XMVectorSelect(Point, S1, SelectS1);

        // t > 1
        XMVECTOR SelectS2 = XMVectorGreater(Projection, LengthSq);
        Point = XMVectorSelect(Point, S2, SelectS2);

        return Point;
    }

    //-----------------------------------------------------------------------------
    // Test if the point (P) on the plane of the triangle is inside the triangle
    // (V0, V1, V2).
    //-----------------------------------------------------------------------------
    inline XMVECTOR XM_CALLCONV PointOnPlaneInsideTriangle(_In_ FXMVECTOR P, _In_ FXMVECTOR V0, _In_ FXMVECTOR V1, _In_ GXMVECTOR V2) noexcept
    {
        // Compute the triangle normal.
        XMVECTOR N = XMVector3Cross(XMVectorSubtract(V2, V0), XMVectorSubtract(V1, V0));

        // Compute the cross products of the vector from the base of each edge to
        // the point with each edge vector.
        XMVECTOR C0 = XMVector3Cross(XMVectorSubtract(P, V0), XMVectorSubtract(V1, V0));
        XMVECTOR C1 = XMVector3Cross(XMVectorSubtract(P, V1), XMVectorSubtract(V2, V1));
        XMVECTOR C2 = XMVector3Cross(XMVectorSubtract(P, V2), XMVectorSubtract(V0, V2));

        // If the cross product points in the same direction as the normal the the
        // point is inside the edge (it is zero if is on the edge).
        XMVECTOR Zero = XMVectorZero();
        XMVECTOR Inside0 = XMVectorGreaterOrEqual(XMVector3Dot(C0, N), Zero);
        XMVECTOR Inside1 = XMVectorGreaterOrEqual(XMVector3Dot(C1, N), Zero);
        XMVECTOR Inside2 = XMVectorGreaterOrEqual(XMVector3Dot(C2, N), Zero);

        // If the point inside all of the edges it is inside.
        return XMVectorAndInt(XMVectorAndInt(Inside0, Inside1), Inside2);
    }

    //-----------------------------------------------------------------------------
    inline bool SolveCubic(_In_ float e, _In_ float f, _In_ float g, _Out_ float* t, _Out_ float* u, _Out_ float* v) noexcept
    {
        float p, q, h, rc, d, theta, costh3, sinth3;

        p = f - e * e / 3.0f;
        q = g - e * f / 3.0f + e * e * e * 2.0f / 27.0f;
        h = q * q / 4.0f + p * p * p / 27.0f;

        if (h > 0)
        {
            *t = *u = *v = 0.f;
            return false; // only one real root
        }

        if ((h == 0) && (q == 0)) // all the same root
        {
            *t = -e / 3;
            *u = -e / 3;
            *v = -e / 3;

            return true;
        }

        d = sqrtf(q * q / 4.0f - h);
        if (d < 0)
            rc = -powf(-d, 1.0f / 3.0f);
        else
            rc = powf(d, 1.0f / 3.0f);

        theta = XMScalarACos(-q / (2.0f * d));
        costh3 = XMScalarCos(theta / 3.0f);
        sinth3 = sqrtf(3.0f) * XMScalarSin(theta / 3.0f);
        *t = 2.0f * rc * costh3 - e / 3.0f;
        *u = -rc * (costh3 + sinth3) - e / 3.0f;
        *v = -rc * (costh3 - sinth3) - e / 3.0f;

        return true;
    }

    //-----------------------------------------------------------------------------
    inline XMVECTOR CalculateEigenVector(_In_ float m11, _In_ float m12, _In_ float m13,
        _In_ float m22, _In_ float m23, _In_ float m33, _In_ float e) noexcept
    {
        float fTmp[3];
        fTmp[0] = m12 * m23 - m13 * (m22 - e);
        fTmp[1] = m13 * m12 - m23 * (m11 - e);
        fTmp[2] = (m11 - e) * (m22 - e) - m12 * m12;

        XMVECTOR vTmp = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(fTmp));

        if (XMVector3Equal(vTmp, XMVectorZero())) // planar or linear
        {
            float f1, f2, f3;

            // we only have one equation - find a valid one
            if ((m11 - e != 0) || (m12 != 0) || (m13 != 0))
            {
                f1 = m11 - e; f2 = m12; f3 = m13;
            }
            else if ((m12 != 0) || (m22 - e != 0) || (m23 != 0))
            {
                f1 = m12; f2 = m22 - e; f3 = m23;
            }
            else if ((m13 != 0) || (m23 != 0) || (m33 - e != 0))
            {
                f1 = m13; f2 = m23; f3 = m33 - e;
            }
            else
            {
                // error, we'll just make something up - we have NO context
                f1 = 1.0f; f2 = 0.0f; f3 = 0.0f;
            }

            if (f1 == 0)
                vTmp = XMVectorSetX(vTmp, 0.0f);
            else
                vTmp = XMVectorSetX(vTmp, 1.0f);

            if (f2 == 0)
                vTmp = XMVectorSetY(vTmp, 0.0f);
            else
                vTmp = XMVectorSetY(vTmp, 1.0f);

            if (f3 == 0)
            {
                vTmp = XMVectorSetZ(vTmp, 0.0f);
                // recalculate y to make equation work
                if (m12 != 0)
                    vTmp = XMVectorSetY(vTmp, -f1 / f2);
            }
            else
            {
                vTmp = XMVectorSetZ(vTmp, (f2 - f1) / f3);
            }
        }

        if (XMVectorGetX(XMVector3LengthSq(vTmp)) > 1e-5f)
        {
            return XMVector3Normalize(vTmp);
        }
        else
        {
            // Multiply by a value large enough to make the vector non-zero.
            vTmp = XMVectorScale(vTmp, 1e5f);
            return XMVector3Normalize(vTmp);
        }
    }

    //-----------------------------------------------------------------------------
    inline bool CalculateEigenVectors(_In_ float m11, _In_ float m12, _In_ float m13,
        _In_ float m22, _In_ float m23, _In_ float m33,
        _In_ float e1, _In_ float e2, _In_ float e3,
        _Out_ XMVECTOR* pV1, _Out_ XMVECTOR* pV2, _Out_ XMVECTOR* pV3) noexcept
    {
        *pV1 = DirectX::Internal::CalculateEigenVector(m11, m12, m13, m22, m23, m33, e1);
        *pV2 = DirectX::Internal::CalculateEigenVector(m11, m12, m13, m22, m23, m33, e2);
        *pV3 = DirectX::Internal::CalculateEigenVector(m11, m12, m13, m22, m23, m33, e3);

        bool v1z = false;
        bool v2z = false;
        bool v3z = false;

        XMVECTOR Zero = XMVectorZero();

        if (XMVector3Equal(*pV1, Zero))
            v1z = true;

        if (XMVector3Equal(*pV2, Zero))
            v2z = true;

        if (XMVector3Equal(*pV3, Zero))
            v3z = true;

        bool e12 = (fabsf(XMVectorGetX(XMVector3Dot(*pV1, *pV2))) > 0.1f); // check for non-orthogonal vectors
        bool e13 = (fabsf(XMVectorGetX(XMVector3Dot(*pV1, *pV3))) > 0.1f);
        bool e23 = (fabsf(XMVectorGetX(XMVector3Dot(*pV2, *pV3))) > 0.1f);

        if ((v1z && v2z && v3z) || (e12 && e13 && e23) ||
            (e12 && v3z) || (e13 && v2z) || (e23 && v1z)) // all eigenvectors are 0- any basis set
        {
            *pV1 = g_XMIdentityR0.v;
            *pV2 = g_XMIdentityR1.v;
            *pV3 = g_XMIdentityR2.v;
            return true;
        }

        if (v1z && v2z)
        {
            XMVECTOR vTmp = XMVector3Cross(g_XMIdentityR1, *pV3);
            if (XMVectorGetX(XMVector3LengthSq(vTmp)) < 1e-5f)
            {
                vTmp = XMVector3Cross(g_XMIdentityR0, *pV3);
            }
            *pV1 = XMVector3Normalize(vTmp);
            *pV2 = XMVector3Cross(*pV3, *pV1);
            return true;
        }

        if (v3z && v1z)
        {
            XMVECTOR vTmp = XMVector3Cross(g_XMIdentityR1, *pV2);
            if (XMVectorGetX(XMVector3LengthSq(vTmp)) < 1e-5f)
            {
                vTmp = XMVector3Cross(g_XMIdentityR0, *pV2);
            }
            *pV3 = XMVector3Normalize(vTmp);
            *pV1 = XMVector3Cross(*pV2, *pV3);
            return true;
        }

        if (v2z && v3z)
        {
            XMVECTOR vTmp = XMVector3Cross(g_XMIdentityR1, *pV1);
            if (XMVectorGetX(XMVector3LengthSq(vTmp)) < 1e-5f)
            {
                vTmp = XMVector3Cross(g_XMIdentityR0, *pV1);
            }
            *pV2 = XMVector3Normalize(vTmp);
            *pV3 = XMVector3Cross(*pV1, *pV2);
            return true;
        }

        if ((v1z) || e12)
        {
            *pV1 = XMVector3Cross(*pV2, *pV3);
            return true;
        }

        if ((v2z) || e23)
        {
            *pV2 = XMVector3Cross(*pV3, *pV1);
            return true;
        }

        if ((v3z) || e13)
        {
            *pV3 = XMVector3Cross(*pV1, *pV2);
            return true;
        }

        return true;
    }

    //-----------------------------------------------------------------------------
    inline bool CalculateEigenVectorsFromCovarianceMatrix(_In_ float Cxx, _In_ float Cyy, _In_ float Czz,
        _In_ float Cxy, _In_ float Cxz, _In_ float Cyz,
        _Out_ XMVECTOR* pV1, _Out_ XMVECTOR* pV2, _Out_ XMVECTOR* pV3) noexcept
    {
        // Calculate the eigenvalues by solving a cubic equation.
        float e = -(Cxx + Cyy + Czz);
        float f = Cxx * Cyy + Cyy * Czz + Czz * Cxx - Cxy * Cxy - Cxz * Cxz - Cyz * Cyz;
        float g = Cxy * Cxy * Czz + Cxz * Cxz * Cyy + Cyz * Cyz * Cxx - Cxy * Cyz * Cxz * 2.0f - Cxx * Cyy * Czz;

        float ev1, ev2, ev3;
        if (!DirectX::Internal::SolveCubic(e, f, g, &ev1, &ev2, &ev3))
        {
            // set them to arbitrary orthonormal basis set
            *pV1 = g_XMIdentityR0.v;
            *pV2 = g_XMIdentityR1.v;
            *pV3 = g_XMIdentityR2.v;
            return false;
        }

        return DirectX::Internal::CalculateEigenVectors(Cxx, Cxy, Cxz, Cyy, Cyz, Czz, ev1, ev2, ev3, pV1, pV2, pV3);
    }

    //-----------------------------------------------------------------------------
    inline void XM_CALLCONV FastIntersectTrianglePlane(
        FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2,
        GXMVECTOR Plane,
        XMVECTOR& Outside, XMVECTOR& Inside) noexcept
    {
        // Plane0
        XMVECTOR Dist0 = XMVector4Dot(V0, Plane);
        XMVECTOR Dist1 = XMVector4Dot(V1, Plane);
        XMVECTOR Dist2 = XMVector4Dot(V2, Plane);

        XMVECTOR MinDist = XMVectorMin(Dist0, Dist1);
        MinDist = XMVectorMin(MinDist, Dist2);

        XMVECTOR MaxDist = XMVectorMax(Dist0, Dist1);
        MaxDist = XMVectorMax(MaxDist, Dist2);

        XMVECTOR Zero = XMVectorZero();

        // Outside the plane?
        Outside = XMVectorGreater(MinDist, Zero);

        // Fully inside the plane?
        Inside = XMVectorLess(MaxDist, Zero);
    }

    //-----------------------------------------------------------------------------
    inline void FastIntersectSpherePlane(_In_ FXMVECTOR Center, _In_ FXMVECTOR Radius, _In_ FXMVECTOR Plane,
        _Out_ XMVECTOR& Outside, _Out_ XMVECTOR& Inside) noexcept
    {
        XMVECTOR Dist = XMVector4Dot(Center, Plane);

        // Outside the plane?
        Outside = XMVectorGreater(Dist, Radius);

        // Fully inside the plane?
        Inside = XMVectorLess(Dist, XMVectorNegate(Radius));
    }

    //-----------------------------------------------------------------------------
    inline void FastIntersectAxisAlignedBoxPlane(_In_ FXMVECTOR Center, _In_ FXMVECTOR Extents, _In_ FXMVECTOR Plane,
        _Out_ XMVECTOR& Outside, _Out_ XMVECTOR& Inside) noexcept
    {
        // Compute the distance to the center of the box.
        XMVECTOR Dist = XMVector4Dot(Center, Plane);

        // Project the axes of the box onto the normal of the plane.  Half the
        // length of the projection (sometime called the "radius") is equal to
        // h(u) * abs(n dot b(u))) + h(v) * abs(n dot b(v)) + h(w) * abs(n dot b(w))
        // where h(i) are extents of the box, n is the plane normal, and b(i) are the
        // axes of the box. In this case b(i) = [(1,0,0), (0,1,0), (0,0,1)].
        XMVECTOR Radius = XMVector3Dot(Extents, XMVectorAbs(Plane));

        // Outside the plane?
        Outside = XMVectorGreater(Dist, Radius);

        // Fully inside the plane?
        Inside = XMVectorLess(Dist, XMVectorNegate(Radius));
    }

    //-----------------------------------------------------------------------------
    inline void XM_CALLCONV FastIntersectOrientedBoxPlane(
        _In_ FXMVECTOR Center, _In_ FXMVECTOR Extents, _In_ FXMVECTOR Axis0,
        _In_ GXMVECTOR Axis1,
        _In_ HXMVECTOR Axis2, _In_ HXMVECTOR Plane,
        _Out_ XMVECTOR& Outside, _Out_ XMVECTOR& Inside) noexcept
    {
        // Compute the distance to the center of the box.
        XMVECTOR Dist = XMVector4Dot(Center, Plane);

        // Project the axes of the box onto the normal of the plane.  Half the
        // length of the projection (sometime called the "radius") is equal to
        // h(u) * abs(n dot b(u))) + h(v) * abs(n dot b(v)) + h(w) * abs(n dot b(w))
        // where h(i) are extents of the box, n is the plane normal, and b(i) are the
        // axes of the box.
        XMVECTOR Radius = XMVector3Dot(Plane, Axis0);
        Radius = XMVectorInsert<0, 0, 1, 0, 0>(Radius, XMVector3Dot(Plane, Axis1));
        Radius = XMVectorInsert<0, 0, 0, 1, 0>(Radius, XMVector3Dot(Plane, Axis2));
        Radius = XMVector3Dot(Extents, XMVectorAbs(Radius));

        // Outside the plane?
        Outside = XMVectorGreater(Dist, Radius);

        // Fully inside the plane?
        Inside = XMVectorLess(Dist, XMVectorNegate(Radius));
    }

    //-----------------------------------------------------------------------------
    inline void XM_CALLCONV FastIntersectFrustumPlane(
        _In_ FXMVECTOR Point0, _In_ FXMVECTOR Point1, _In_ FXMVECTOR Point2,
        _In_ GXMVECTOR Point3,
        _In_ HXMVECTOR Point4, _In_ HXMVECTOR Point5,
        _In_ CXMVECTOR Point6, _In_ CXMVECTOR Point7, _In_ CXMVECTOR Plane,
        _Out_ XMVECTOR& Outside, _Out_ XMVECTOR& Inside) noexcept
    {
        // Find the min/max projection of the frustum onto the plane normal.
        XMVECTOR Min, Max, Dist;

        Min = Max = XMVector3Dot(Plane, Point0);

        Dist = XMVector3Dot(Plane, Point1);
        Min = XMVectorMin(Min, Dist);
        Max = XMVectorMax(Max, Dist);

        Dist = XMVector3Dot(Plane, Point2);
        Min = XMVectorMin(Min, Dist);
        Max = XMVectorMax(Max, Dist);

        Dist = XMVector3Dot(Plane, Point3);
        Min = XMVectorMin(Min, Dist);
        Max = XMVectorMax(Max, Dist);

        Dist = XMVector3Dot(Plane, Point4);
        Min = XMVectorMin(Min, Dist);
        Max = XMVectorMax(Max, Dist);

        Dist = XMVector3Dot(Plane, Point5);
        Min = XMVectorMin(Min, Dist);
        Max = XMVectorMax(Max, Dist);

        Dist = XMVector3Dot(Plane, Point6);
        Min = XMVectorMin(Min, Dist);
        Max = XMVectorMax(Max, Dist);

        Dist = XMVector3Dot(Plane, Point7);
        Min = XMVectorMin(Min, Dist);
        Max = XMVectorMax(Max, Dist);

        XMVECTOR PlaneDist = XMVectorNegate(XMVectorSplatW(Plane));

        // Outside the plane?
        Outside = XMVectorGreater(Min, PlaneDist);

        // Fully inside the plane?
        Inside = XMVectorLess(Max, PlaneDist);
    }

} // namespace Internal


/****************************************************************************
 *
 * BoundingSphere
 *
 ****************************************************************************/

 //-----------------------------------------------------------------------------
 // Transform a sphere by an angle preserving transform.
 //-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV BoundingSphere::Transform(BoundingSphere& Out, FXMMATRIX M) const noexcept
{
    // Load the center of the sphere.
    XMVECTOR vCenter = XMLoadFloat3(&Center);

    // Transform the center of the sphere.
    XMVECTOR C = XMVector3Transform(vCenter, M);

    XMVECTOR dX = XMVector3Dot(M.r[0], M.r[0]);
    XMVECTOR dY = XMVector3Dot(M.r[1], M.r[1]);
    XMVECTOR dZ = XMVector3Dot(M.r[2], M.r[2]);

    XMVECTOR d = XMVectorMax(dX, XMVectorMax(dY, dZ));

    // Store the center sphere.
    XMStoreFloat3(&Out.Center, C);

    // Scale the radius of the pshere.
    float Scale = sqrtf(XMVectorGetX(d));
    Out.Radius = Radius * Scale;
}

_Use_decl_annotations_
inline void XM_CALLCONV BoundingSphere::Transform(BoundingSphere& Out, float Scale, FXMVECTOR Rotation, FXMVECTOR Translation) const noexcept
{
    // Load the center of the sphere.
    XMVECTOR vCenter = XMLoadFloat3(&Center);

    // Transform the center of the sphere.
    vCenter = XMVectorAdd(XMVector3Rotate(XMVectorScale(vCenter, Scale), Rotation), Translation);

    // Store the center sphere.
    XMStoreFloat3(&Out.Center, vCenter);

    // Scale the radius of the pshere.
    Out.Radius = Radius * Scale;
}


//-----------------------------------------------------------------------------
// Point in sphere test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingSphere::Contains(FXMVECTOR Point) const noexcept
{
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);

    XMVECTOR DistanceSquared = XMVector3LengthSq(XMVectorSubtract(Point, vCenter));
    XMVECTOR RadiusSquared = XMVectorMultiply(vRadius, vRadius);

    return XMVector3LessOrEqual(DistanceSquared, RadiusSquared) ? CONTAINS : DISJOINT;
}


//-----------------------------------------------------------------------------
// Triangle in sphere test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingSphere::Contains(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    if (!Intersects(V0, V1, V2))
        return DISJOINT;

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);
    XMVECTOR RadiusSquared = XMVectorMultiply(vRadius, vRadius);

    XMVECTOR DistanceSquared = XMVector3LengthSq(XMVectorSubtract(V0, vCenter));
    XMVECTOR Inside = XMVectorLessOrEqual(DistanceSquared, RadiusSquared);

    DistanceSquared = XMVector3LengthSq(XMVectorSubtract(V1, vCenter));
    Inside = XMVectorAndInt(Inside, XMVectorLessOrEqual(DistanceSquared, RadiusSquared));

    DistanceSquared = XMVector3LengthSq(XMVectorSubtract(V2, vCenter));
    Inside = XMVectorAndInt(Inside, XMVectorLessOrEqual(DistanceSquared, RadiusSquared));

    return (XMVector3EqualInt(Inside, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Sphere in sphere test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingSphere::Contains(const BoundingSphere& sh) const noexcept
{
    XMVECTOR Center1 = XMLoadFloat3(&Center);
    float r1 = Radius;

    XMVECTOR Center2 = XMLoadFloat3(&sh.Center);
    float r2 = sh.Radius;

    XMVECTOR V = XMVectorSubtract(Center2, Center1);

    XMVECTOR Dist = XMVector3Length(V);

    float d = XMVectorGetX(Dist);

    return (r1 + r2 >= d) ? ((r1 - r2 >= d) ? CONTAINS : INTERSECTS) : DISJOINT;
}


//-----------------------------------------------------------------------------
// Axis-aligned box in sphere test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingSphere::Contains(const BoundingBox& box) const noexcept
{
    if (!box.Intersects(*this))
        return DISJOINT;

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);
    XMVECTOR RadiusSq = XMVectorMultiply(vRadius, vRadius);

    XMVECTOR boxCenter = XMLoadFloat3(&box.Center);
    XMVECTOR boxExtents = XMLoadFloat3(&box.Extents);

    XMVECTOR InsideAll = XMVectorTrueInt();

    XMVECTOR offset = XMVectorSubtract(boxCenter, vCenter);

    for (size_t i = 0; i < BoundingBox::CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVectorMultiplyAdd(boxExtents, g_BoxOffset[i], offset);
        XMVECTOR d = XMVector3LengthSq(C);
        InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(d, RadiusSq));
    }

    return (XMVector3EqualInt(InsideAll, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Oriented box in sphere test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingSphere::Contains(const BoundingOrientedBox& box) const noexcept
{
    if (!box.Intersects(*this))
        return DISJOINT;

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);
    XMVECTOR RadiusSq = XMVectorMultiply(vRadius, vRadius);

    XMVECTOR boxCenter = XMLoadFloat3(&box.Center);
    XMVECTOR boxExtents = XMLoadFloat3(&box.Extents);
    XMVECTOR boxOrientation = XMLoadFloat4(&box.Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(boxOrientation));

    XMVECTOR InsideAll = XMVectorTrueInt();

    for (size_t i = 0; i < BoundingOrientedBox::CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVectorAdd(XMVector3Rotate(XMVectorMultiply(boxExtents, g_BoxOffset[i]), boxOrientation), boxCenter);
        XMVECTOR d = XMVector3LengthSq(XMVectorSubtract(vCenter, C));
        InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(d, RadiusSq));
    }

    return (XMVector3EqualInt(InsideAll, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;

}


//-----------------------------------------------------------------------------
// Frustum in sphere test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingSphere::Contains(const BoundingFrustum& fr) const noexcept
{
    if (!fr.Intersects(*this))
        return DISJOINT;

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);
    XMVECTOR RadiusSq = XMVectorMultiply(vRadius, vRadius);

    XMVECTOR vOrigin = XMLoadFloat3(&fr.Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&fr.Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Build the corners of the frustum.
    XMVECTOR vRightTop = XMVectorSet(fr.RightSlope, fr.TopSlope, 1.0f, 0.0f);
    XMVECTOR vRightBottom = XMVectorSet(fr.RightSlope, fr.BottomSlope, 1.0f, 0.0f);
    XMVECTOR vLeftTop = XMVectorSet(fr.LeftSlope, fr.TopSlope, 1.0f, 0.0f);
    XMVECTOR vLeftBottom = XMVectorSet(fr.LeftSlope, fr.BottomSlope, 1.0f, 0.0f);
    XMVECTOR vNear = XMVectorReplicatePtr(&fr.Near);
    XMVECTOR vFar = XMVectorReplicatePtr(&fr.Far);

    XMVECTOR Corners[BoundingFrustum::CORNER_COUNT];
    Corners[0] = XMVectorMultiply(vRightTop, vNear);
    Corners[1] = XMVectorMultiply(vRightBottom, vNear);
    Corners[2] = XMVectorMultiply(vLeftTop, vNear);
    Corners[3] = XMVectorMultiply(vLeftBottom, vNear);
    Corners[4] = XMVectorMultiply(vRightTop, vFar);
    Corners[5] = XMVectorMultiply(vRightBottom, vFar);
    Corners[6] = XMVectorMultiply(vLeftTop, vFar);
    Corners[7] = XMVectorMultiply(vLeftBottom, vFar);

    XMVECTOR InsideAll = XMVectorTrueInt();
    for (size_t i = 0; i < BoundingFrustum::CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVectorAdd(XMVector3Rotate(Corners[i], vOrientation), vOrigin);
        XMVECTOR d = XMVector3LengthSq(XMVectorSubtract(vCenter, C));
        InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(d, RadiusSq));
    }

    return (XMVector3EqualInt(InsideAll, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Sphere vs. sphere test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingSphere::Intersects(const BoundingSphere& sh) const noexcept
{
    // Load A.
    XMVECTOR vCenterA = XMLoadFloat3(&Center);
    XMVECTOR vRadiusA = XMVectorReplicatePtr(&Radius);

    // Load B.
    XMVECTOR vCenterB = XMLoadFloat3(&sh.Center);
    XMVECTOR vRadiusB = XMVectorReplicatePtr(&sh.Radius);

    // Distance squared between centers.
    XMVECTOR Delta = XMVectorSubtract(vCenterB, vCenterA);
    XMVECTOR DistanceSquared = XMVector3LengthSq(Delta);

    // Sum of the radii squared.
    XMVECTOR RadiusSquared = XMVectorAdd(vRadiusA, vRadiusB);
    RadiusSquared = XMVectorMultiply(RadiusSquared, RadiusSquared);

    return XMVector3LessOrEqual(DistanceSquared, RadiusSquared);
}


//-----------------------------------------------------------------------------
// Box vs. sphere test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingSphere::Intersects(const BoundingBox& box) const noexcept
{
    return box.Intersects(*this);
}

_Use_decl_annotations_
inline bool BoundingSphere::Intersects(const BoundingOrientedBox& box) const noexcept
{
    return box.Intersects(*this);
}


//-----------------------------------------------------------------------------
// Frustum vs. sphere test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingSphere::Intersects(const BoundingFrustum& fr) const noexcept
{
    return fr.Intersects(*this);
}


//-----------------------------------------------------------------------------
// Triangle vs sphere test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingSphere::Intersects(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    // Load the sphere.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);

    // Compute the plane of the triangle (has to be normalized).
    XMVECTOR N = XMVector3Normalize(XMVector3Cross(XMVectorSubtract(V1, V0), XMVectorSubtract(V2, V0)));

    // Assert that the triangle is not degenerate.
    assert(!XMVector3Equal(N, XMVectorZero()));

    // Find the nearest feature on the triangle to the sphere.
    XMVECTOR Dist = XMVector3Dot(XMVectorSubtract(vCenter, V0), N);

    // If the center of the sphere is farther from the plane of the triangle than
    // the radius of the sphere, then there cannot be an intersection.
    XMVECTOR NoIntersection = XMVectorLess(Dist, XMVectorNegate(vRadius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Dist, vRadius));

    // Project the center of the sphere onto the plane of the triangle.
    XMVECTOR Point = XMVectorNegativeMultiplySubtract(N, Dist, vCenter);

    // Is it inside all the edges? If so we intersect because the distance
    // to the plane is less than the radius.
    XMVECTOR Intersection = DirectX::Internal::PointOnPlaneInsideTriangle(Point, V0, V1, V2);

    // Find the nearest point on each edge.
    XMVECTOR RadiusSq = XMVectorMultiply(vRadius, vRadius);

    // Edge 0,1
    Point = DirectX::Internal::PointOnLineSegmentNearestPoint(V0, V1, vCenter);

    // If the distance to the center of the sphere to the point is less than
    // the radius of the sphere then it must intersect.
    Intersection = XMVectorOrInt(Intersection, XMVectorLessOrEqual(XMVector3LengthSq(XMVectorSubtract(vCenter, Point)), RadiusSq));

    // Edge 1,2
    Point = DirectX::Internal::PointOnLineSegmentNearestPoint(V1, V2, vCenter);

    // If the distance to the center of the sphere to the point is less than
    // the radius of the sphere then it must intersect.
    Intersection = XMVectorOrInt(Intersection, XMVectorLessOrEqual(XMVector3LengthSq(XMVectorSubtract(vCenter, Point)), RadiusSq));

    // Edge 2,0
    Point = DirectX::Internal::PointOnLineSegmentNearestPoint(V2, V0, vCenter);

    // If the distance to the center of the sphere to the point is less than
    // the radius of the sphere then it must intersect.
    Intersection = XMVectorOrInt(Intersection, XMVectorLessOrEqual(XMVector3LengthSq(XMVectorSubtract(vCenter, Point)), RadiusSq));

    return XMVector4EqualInt(XMVectorAndCInt(Intersection, NoIntersection), XMVectorTrueInt());
}


//-----------------------------------------------------------------------------
// Sphere-plane intersection
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline PlaneIntersectionType XM_CALLCONV BoundingSphere::Intersects(FXMVECTOR Plane) const noexcept
{
    assert(DirectX::Internal::XMPlaneIsUnit(Plane));

    // Load the sphere.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);

    // Set w of the center to one so we can dot4 with a plane.
    vCenter = XMVectorInsert<0, 0, 0, 0, 1>(vCenter, XMVectorSplatOne());

    XMVECTOR Outside, Inside;
    DirectX::Internal::FastIntersectSpherePlane(vCenter, vRadius, Plane, Outside, Inside);

    // If the sphere is outside any plane it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return FRONT;

    // If the sphere is inside all planes it is inside.
    if (XMVector4EqualInt(Inside, XMVectorTrueInt()))
        return BACK;

    // The sphere is not inside all planes or outside a plane it intersects.
    return INTERSECTING;
}


//-----------------------------------------------------------------------------
// Compute the intersection of a ray (Origin, Direction) with a sphere.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingSphere::Intersects(FXMVECTOR Origin, FXMVECTOR Direction, float& Dist) const noexcept
{
    assert(DirectX::Internal::XMVector3IsUnit(Direction));

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);

    // l is the vector from the ray origin to the center of the sphere.
    XMVECTOR l = XMVectorSubtract(vCenter, Origin);

    // s is the projection of the l onto the ray direction.
    XMVECTOR s = XMVector3Dot(l, Direction);

    XMVECTOR l2 = XMVector3Dot(l, l);

    XMVECTOR r2 = XMVectorMultiply(vRadius, vRadius);

    // m2 is squared distance from the center of the sphere to the projection.
    XMVECTOR m2 = XMVectorNegativeMultiplySubtract(s, s, l2);

    XMVECTOR NoIntersection;

    // If the ray origin is outside the sphere and the center of the sphere is
    // behind the ray origin there is no intersection.
    NoIntersection = XMVectorAndInt(XMVectorLess(s, XMVectorZero()), XMVectorGreater(l2, r2));

    // If the squared distance from the center of the sphere to the projection
    // is greater than the radius squared the ray will miss the sphere.
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(m2, r2));

    // The ray hits the sphere, compute the nearest intersection point.
    XMVECTOR q = XMVectorSqrt(XMVectorSubtract(r2, m2));
    XMVECTOR t1 = XMVectorSubtract(s, q);
    XMVECTOR t2 = XMVectorAdd(s, q);

    XMVECTOR OriginInside = XMVectorLessOrEqual(l2, r2);
    XMVECTOR t = XMVectorSelect(t1, t2, OriginInside);

    if (XMVector4NotEqualInt(NoIntersection, XMVectorTrueInt()))
    {
        // Store the x-component to *pDist.
        XMStoreFloat(&Dist, t);
        return true;
    }

    Dist = 0.f;
    return false;
}


//-----------------------------------------------------------------------------
// Test a sphere vs 6 planes (typically forming a frustum).
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingSphere::ContainedBy(
    FXMVECTOR Plane0, FXMVECTOR Plane1, FXMVECTOR Plane2,
    GXMVECTOR Plane3,
    HXMVECTOR Plane4, HXMVECTOR Plane5) const noexcept
{
    // Load the sphere.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&Radius);

    // Set w of the center to one so we can dot4 with a plane.
    vCenter = XMVectorInsert<0, 0, 0, 0, 1>(vCenter, XMVectorSplatOne());

    XMVECTOR Outside, Inside;

    // Test against each plane.
    DirectX::Internal::FastIntersectSpherePlane(vCenter, vRadius, Plane0, Outside, Inside);

    XMVECTOR AnyOutside = Outside;
    XMVECTOR AllInside = Inside;

    DirectX::Internal::FastIntersectSpherePlane(vCenter, vRadius, Plane1, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectSpherePlane(vCenter, vRadius, Plane2, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectSpherePlane(vCenter, vRadius, Plane3, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectSpherePlane(vCenter, vRadius, Plane4, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectSpherePlane(vCenter, vRadius, Plane5, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    // If the sphere is outside any plane it is outside.
    if (XMVector4EqualInt(AnyOutside, XMVectorTrueInt()))
        return DISJOINT;

    // If the sphere is inside all planes it is inside.
    if (XMVector4EqualInt(AllInside, XMVectorTrueInt()))
        return CONTAINS;

    // The sphere is not inside all planes or outside a plane, it may intersect.
    return INTERSECTS;
}


//-----------------------------------------------------------------------------
// Creates a bounding sphere that contains two other bounding spheres
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingSphere::CreateMerged(BoundingSphere& Out, const BoundingSphere& S1, const BoundingSphere& S2) noexcept
{
    XMVECTOR Center1 = XMLoadFloat3(&S1.Center);
    float r1 = S1.Radius;

    XMVECTOR Center2 = XMLoadFloat3(&S2.Center);
    float r2 = S2.Radius;

    XMVECTOR V = XMVectorSubtract(Center2, Center1);

    XMVECTOR Dist = XMVector3Length(V);

    float d = XMVectorGetX(Dist);

    if (r1 + r2 >= d)
    {
        if (r1 - r2 >= d)
        {
            Out = S1;
            return;
        }
        else if (r2 - r1 >= d)
        {
            Out = S2;
            return;
        }
    }

    XMVECTOR N = XMVectorDivide(V, Dist);

    float t1 = XMMin(-r1, d - r2);
    float t2 = XMMax(r1, d + r2);
    float t_5 = (t2 - t1) * 0.5f;

    XMVECTOR NCenter = XMVectorAdd(Center1, XMVectorMultiply(N, XMVectorReplicate(t_5 + t1)));

    XMStoreFloat3(&Out.Center, NCenter);
    Out.Radius = t_5;
}


//-----------------------------------------------------------------------------
// Create sphere enscribing bounding box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingSphere::CreateFromBoundingBox(BoundingSphere& Out, const BoundingBox& box) noexcept
{
    Out.Center = box.Center;
    XMVECTOR vExtents = XMLoadFloat3(&box.Extents);
    Out.Radius = XMVectorGetX(XMVector3Length(vExtents));
}

_Use_decl_annotations_
inline void BoundingSphere::CreateFromBoundingBox(BoundingSphere& Out, const BoundingOrientedBox& box) noexcept
{
    // Bounding box orientation is irrelevant because a sphere is rotationally invariant
    Out.Center = box.Center;
    XMVECTOR vExtents = XMLoadFloat3(&box.Extents);
    Out.Radius = XMVectorGetX(XMVector3Length(vExtents));
}


//-----------------------------------------------------------------------------
// Find the approximate smallest enclosing bounding sphere for a set of
// points. Exact computation of the smallest enclosing bounding sphere is
// possible but is slower and requires a more complex algorithm.
// The algorithm is based on  Jack Ritter, "An Efficient Bounding Sphere",
// Graphics Gems.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingSphere::CreateFromPoints(BoundingSphere& Out, size_t Count, const XMFLOAT3* pPoints, size_t Stride) noexcept
{
    assert(Count > 0);
    assert(pPoints);

    // Find the points with minimum and maximum x, y, and z
    XMVECTOR MinX, MaxX, MinY, MaxY, MinZ, MaxZ;

    MinX = MaxX = MinY = MaxY = MinZ = MaxZ = XMLoadFloat3(pPoints);

    for (size_t i = 1; i < Count; ++i)
    {
        XMVECTOR Point = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(reinterpret_cast<const uint8_t*>(pPoints) + i * Stride));

        float px = XMVectorGetX(Point);
        float py = XMVectorGetY(Point);
        float pz = XMVectorGetZ(Point);

        if (px < XMVectorGetX(MinX))
            MinX = Point;

        if (px > XMVectorGetX(MaxX))
            MaxX = Point;

        if (py < XMVectorGetY(MinY))
            MinY = Point;

        if (py > XMVectorGetY(MaxY))
            MaxY = Point;

        if (pz < XMVectorGetZ(MinZ))
            MinZ = Point;

        if (pz > XMVectorGetZ(MaxZ))
            MaxZ = Point;
    }

    // Use the min/max pair that are farthest apart to form the initial sphere.
    XMVECTOR DeltaX = XMVectorSubtract(MaxX, MinX);
    XMVECTOR DistX = XMVector3Length(DeltaX);

    XMVECTOR DeltaY = XMVectorSubtract(MaxY, MinY);
    XMVECTOR DistY = XMVector3Length(DeltaY);

    XMVECTOR DeltaZ = XMVectorSubtract(MaxZ, MinZ);
    XMVECTOR DistZ = XMVector3Length(DeltaZ);

    XMVECTOR vCenter;
    XMVECTOR vRadius;

    if (XMVector3Greater(DistX, DistY))
    {
        if (XMVector3Greater(DistX, DistZ))
        {
            // Use min/max x.
            vCenter = XMVectorLerp(MaxX, MinX, 0.5f);
            vRadius = XMVectorScale(DistX, 0.5f);
        }
        else
        {
            // Use min/max z.
            vCenter = XMVectorLerp(MaxZ, MinZ, 0.5f);
            vRadius = XMVectorScale(DistZ, 0.5f);
        }
    }
    else // Y >= X
    {
        if (XMVector3Greater(DistY, DistZ))
        {
            // Use min/max y.
            vCenter = XMVectorLerp(MaxY, MinY, 0.5f);
            vRadius = XMVectorScale(DistY, 0.5f);
        }
        else
        {
            // Use min/max z.
            vCenter = XMVectorLerp(MaxZ, MinZ, 0.5f);
            vRadius = XMVectorScale(DistZ, 0.5f);
        }
    }

    // Add any points not inside the sphere.
    for (size_t i = 0; i < Count; ++i)
    {
        XMVECTOR Point = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(reinterpret_cast<const uint8_t*>(pPoints) + i * Stride));

        XMVECTOR Delta = XMVectorSubtract(Point, vCenter);

        XMVECTOR Dist = XMVector3Length(Delta);

        if (XMVector3Greater(Dist, vRadius))
        {
            // Adjust sphere to include the new point.
            vRadius = XMVectorScale(XMVectorAdd(vRadius, Dist), 0.5f);
            vCenter = XMVectorAdd(vCenter, XMVectorMultiply(XMVectorSubtract(XMVectorReplicate(1.0f), XMVectorDivide(vRadius, Dist)), Delta));
        }
    }

    XMStoreFloat3(&Out.Center, vCenter);
    XMStoreFloat(&Out.Radius, vRadius);
}


//-----------------------------------------------------------------------------
// Create sphere containing frustum
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingSphere::CreateFromFrustum(BoundingSphere& Out, const BoundingFrustum& fr) noexcept
{
    XMFLOAT3 Corners[BoundingFrustum::CORNER_COUNT];
    fr.GetCorners(Corners);
    CreateFromPoints(Out, BoundingFrustum::CORNER_COUNT, Corners, sizeof(XMFLOAT3));
}


/****************************************************************************
 *
 * BoundingBox
 *
 ****************************************************************************/

 //-----------------------------------------------------------------------------
 // Transform an axis aligned box by an angle preserving transform.
 //-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV BoundingBox::Transform(BoundingBox& Out, FXMMATRIX M) const noexcept
{
    // Load center and extents.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    // Compute and transform the corners and find new min/max bounds.
    XMVECTOR Corner = XMVectorMultiplyAdd(vExtents, g_BoxOffset[0], vCenter);
    Corner = XMVector3Transform(Corner, M);

    XMVECTOR Min, Max;
    Min = Max = Corner;

    for (size_t i = 1; i < CORNER_COUNT; ++i)
    {
        Corner = XMVectorMultiplyAdd(vExtents, g_BoxOffset[i], vCenter);
        Corner = XMVector3Transform(Corner, M);

        Min = XMVectorMin(Min, Corner);
        Max = XMVectorMax(Max, Corner);
    }

    // Store center and extents.
    XMStoreFloat3(&Out.Center, XMVectorScale(XMVectorAdd(Min, Max), 0.5f));
    XMStoreFloat3(&Out.Extents, XMVectorScale(XMVectorSubtract(Max, Min), 0.5f));
}

_Use_decl_annotations_
inline void XM_CALLCONV BoundingBox::Transform(BoundingBox& Out, float Scale, FXMVECTOR Rotation, FXMVECTOR Translation) const noexcept
{
    assert(DirectX::Internal::XMQuaternionIsUnit(Rotation));

    // Load center and extents.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    XMVECTOR VectorScale = XMVectorReplicate(Scale);

    // Compute and transform the corners and find new min/max bounds.
    XMVECTOR Corner = XMVectorMultiplyAdd(vExtents, g_BoxOffset[0], vCenter);
    Corner = XMVectorAdd(XMVector3Rotate(XMVectorMultiply(Corner, VectorScale), Rotation), Translation);

    XMVECTOR Min, Max;
    Min = Max = Corner;

    for (size_t i = 1; i < CORNER_COUNT; ++i)
    {
        Corner = XMVectorMultiplyAdd(vExtents, g_BoxOffset[i], vCenter);
        Corner = XMVectorAdd(XMVector3Rotate(XMVectorMultiply(Corner, VectorScale), Rotation), Translation);

        Min = XMVectorMin(Min, Corner);
        Max = XMVectorMax(Max, Corner);
    }

    // Store center and extents.
    XMStoreFloat3(&Out.Center, XMVectorScale(XMVectorAdd(Min, Max), 0.5f));
    XMStoreFloat3(&Out.Extents, XMVectorScale(XMVectorSubtract(Max, Min), 0.5f));
}


//-----------------------------------------------------------------------------
// Get the corner points of the box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingBox::GetCorners(XMFLOAT3* Corners) const noexcept
{
    assert(Corners != nullptr);

    // Load the box
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    for (size_t i = 0; i < CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVectorMultiplyAdd(vExtents, g_BoxOffset[i], vCenter);
        XMStoreFloat3(&Corners[i], C);
    }
}


//-----------------------------------------------------------------------------
// Point in axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingBox::Contains(FXMVECTOR Point) const noexcept
{
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    return XMVector3InBounds(XMVectorSubtract(Point, vCenter), vExtents) ? CONTAINS : DISJOINT;
}


//-----------------------------------------------------------------------------
// Triangle in axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingBox::Contains(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    if (!Intersects(V0, V1, V2))
        return DISJOINT;

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    XMVECTOR d = XMVectorAbs(XMVectorSubtract(V0, vCenter));
    XMVECTOR Inside = XMVectorLessOrEqual(d, vExtents);

    d = XMVectorAbs(XMVectorSubtract(V1, vCenter));
    Inside = XMVectorAndInt(Inside, XMVectorLessOrEqual(d, vExtents));

    d = XMVectorAbs(XMVectorSubtract(V2, vCenter));
    Inside = XMVectorAndInt(Inside, XMVectorLessOrEqual(d, vExtents));

    return (XMVector3EqualInt(Inside, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Sphere in axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingBox::Contains(const BoundingSphere& sh) const noexcept
{
    XMVECTOR SphereCenter = XMLoadFloat3(&sh.Center);
    XMVECTOR SphereRadius = XMVectorReplicatePtr(&sh.Radius);

    XMVECTOR BoxCenter = XMLoadFloat3(&Center);
    XMVECTOR BoxExtents = XMLoadFloat3(&Extents);

    XMVECTOR BoxMin = XMVectorSubtract(BoxCenter, BoxExtents);
    XMVECTOR BoxMax = XMVectorAdd(BoxCenter, BoxExtents);

    // Find the distance to the nearest point on the box.
    // for each i in (x, y, z)
    // if (SphereCenter(i) < BoxMin(i)) d2 += (SphereCenter(i) - BoxMin(i)) ^ 2
    // else if (SphereCenter(i) > BoxMax(i)) d2 += (SphereCenter(i) - BoxMax(i)) ^ 2

    XMVECTOR d = XMVectorZero();

    // Compute d for each dimension.
    XMVECTOR LessThanMin = XMVectorLess(SphereCenter, BoxMin);
    XMVECTOR GreaterThanMax = XMVectorGreater(SphereCenter, BoxMax);

    XMVECTOR MinDelta = XMVectorSubtract(SphereCenter, BoxMin);
    XMVECTOR MaxDelta = XMVectorSubtract(SphereCenter, BoxMax);

    // Choose value for each dimension based on the comparison.
    d = XMVectorSelect(d, MinDelta, LessThanMin);
    d = XMVectorSelect(d, MaxDelta, GreaterThanMax);

    // Use a dot-product to square them and sum them together.
    XMVECTOR d2 = XMVector3Dot(d, d);

    if (XMVector3Greater(d2, XMVectorMultiply(SphereRadius, SphereRadius)))
        return DISJOINT;

    XMVECTOR InsideAll = XMVectorLessOrEqual(XMVectorAdd(BoxMin, SphereRadius), SphereCenter);
    InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(SphereCenter, XMVectorSubtract(BoxMax, SphereRadius)));
    InsideAll = XMVectorAndInt(InsideAll, XMVectorGreater(XMVectorSubtract(BoxMax, BoxMin), SphereRadius));

    return (XMVector3EqualInt(InsideAll, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Axis-aligned box in axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingBox::Contains(const BoundingBox& box) const noexcept
{
    XMVECTOR CenterA = XMLoadFloat3(&Center);
    XMVECTOR ExtentsA = XMLoadFloat3(&Extents);

    XMVECTOR CenterB = XMLoadFloat3(&box.Center);
    XMVECTOR ExtentsB = XMLoadFloat3(&box.Extents);

    XMVECTOR MinA = XMVectorSubtract(CenterA, ExtentsA);
    XMVECTOR MaxA = XMVectorAdd(CenterA, ExtentsA);

    XMVECTOR MinB = XMVectorSubtract(CenterB, ExtentsB);
    XMVECTOR MaxB = XMVectorAdd(CenterB, ExtentsB);

    // for each i in (x, y, z) if a_min(i) > b_max(i) or b_min(i) > a_max(i) then return false
    XMVECTOR Disjoint = XMVectorOrInt(XMVectorGreater(MinA, MaxB), XMVectorGreater(MinB, MaxA));

    if (DirectX::Internal::XMVector3AnyTrue(Disjoint))
        return DISJOINT;

    // for each i in (x, y, z) if a_min(i) <= b_min(i) and b_max(i) <= a_max(i) then A contains B
    XMVECTOR Inside = XMVectorAndInt(XMVectorLessOrEqual(MinA, MinB), XMVectorLessOrEqual(MaxB, MaxA));

    return DirectX::Internal::XMVector3AllTrue(Inside) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Oriented box in axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingBox::Contains(const BoundingOrientedBox& box) const noexcept
{
    if (!box.Intersects(*this))
        return DISJOINT;

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    // Subtract off the AABB center to remove a subtract below
    XMVECTOR oCenter = XMVectorSubtract(XMLoadFloat3(&box.Center), vCenter);

    XMVECTOR oExtents = XMLoadFloat3(&box.Extents);
    XMVECTOR oOrientation = XMLoadFloat4(&box.Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(oOrientation));

    XMVECTOR Inside = XMVectorTrueInt();

    for (size_t i = 0; i < BoundingOrientedBox::CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVectorAdd(XMVector3Rotate(XMVectorMultiply(oExtents, g_BoxOffset[i]), oOrientation), oCenter);
        XMVECTOR d = XMVectorAbs(C);
        Inside = XMVectorAndInt(Inside, XMVectorLessOrEqual(d, vExtents));
    }

    return (XMVector3EqualInt(Inside, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Frustum in axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingBox::Contains(const BoundingFrustum& fr) const noexcept
{
    if (!fr.Intersects(*this))
        return DISJOINT;

    XMFLOAT3 Corners[BoundingFrustum::CORNER_COUNT];
    fr.GetCorners(Corners);

    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    XMVECTOR Inside = XMVectorTrueInt();

    for (size_t i = 0; i < BoundingFrustum::CORNER_COUNT; ++i)
    {
        XMVECTOR Point = XMLoadFloat3(&Corners[i]);
        XMVECTOR d = XMVectorAbs(XMVectorSubtract(Point, vCenter));
        Inside = XMVectorAndInt(Inside, XMVectorLessOrEqual(d, vExtents));
    }

    return (XMVector3EqualInt(Inside, XMVectorTrueInt())) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Sphere vs axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingBox::Intersects(const BoundingSphere& sh) const noexcept
{
    XMVECTOR SphereCenter = XMLoadFloat3(&sh.Center);
    XMVECTOR SphereRadius = XMVectorReplicatePtr(&sh.Radius);

    XMVECTOR BoxCenter = XMLoadFloat3(&Center);
    XMVECTOR BoxExtents = XMLoadFloat3(&Extents);

    XMVECTOR BoxMin = XMVectorSubtract(BoxCenter, BoxExtents);
    XMVECTOR BoxMax = XMVectorAdd(BoxCenter, BoxExtents);

    // Find the distance to the nearest point on the box.
    // for each i in (x, y, z)
    // if (SphereCenter(i) < BoxMin(i)) d2 += (SphereCenter(i) - BoxMin(i)) ^ 2
    // else if (SphereCenter(i) > BoxMax(i)) d2 += (SphereCenter(i) - BoxMax(i)) ^ 2

    XMVECTOR d = XMVectorZero();

    // Compute d for each dimension.
    XMVECTOR LessThanMin = XMVectorLess(SphereCenter, BoxMin);
    XMVECTOR GreaterThanMax = XMVectorGreater(SphereCenter, BoxMax);

    XMVECTOR MinDelta = XMVectorSubtract(SphereCenter, BoxMin);
    XMVECTOR MaxDelta = XMVectorSubtract(SphereCenter, BoxMax);

    // Choose value for each dimension based on the comparison.
    d = XMVectorSelect(d, MinDelta, LessThanMin);
    d = XMVectorSelect(d, MaxDelta, GreaterThanMax);

    // Use a dot-product to square them and sum them together.
    XMVECTOR d2 = XMVector3Dot(d, d);

    return XMVector3LessOrEqual(d2, XMVectorMultiply(SphereRadius, SphereRadius));
}


//-----------------------------------------------------------------------------
// Axis-aligned box vs. axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingBox::Intersects(const BoundingBox& box) const noexcept
{
    XMVECTOR CenterA = XMLoadFloat3(&Center);
    XMVECTOR ExtentsA = XMLoadFloat3(&Extents);

    XMVECTOR CenterB = XMLoadFloat3(&box.Center);
    XMVECTOR ExtentsB = XMLoadFloat3(&box.Extents);

    XMVECTOR MinA = XMVectorSubtract(CenterA, ExtentsA);
    XMVECTOR MaxA = XMVectorAdd(CenterA, ExtentsA);

    XMVECTOR MinB = XMVectorSubtract(CenterB, ExtentsB);
    XMVECTOR MaxB = XMVectorAdd(CenterB, ExtentsB);

    // for each i in (x, y, z) if a_min(i) > b_max(i) or b_min(i) > a_max(i) then return false
    XMVECTOR Disjoint = XMVectorOrInt(XMVectorGreater(MinA, MaxB), XMVectorGreater(MinB, MaxA));

    return !DirectX::Internal::XMVector3AnyTrue(Disjoint);
}


//-----------------------------------------------------------------------------
// Oriented box vs. axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingBox::Intersects(const BoundingOrientedBox& box) const noexcept
{
    return box.Intersects(*this);
}


//-----------------------------------------------------------------------------
// Frustum vs. axis-aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingBox::Intersects(const BoundingFrustum& fr) const noexcept
{
    return fr.Intersects(*this);
}


//-----------------------------------------------------------------------------
// Triangle vs. axis aligned box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingBox::Intersects(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    XMVECTOR Zero = XMVectorZero();

    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    XMVECTOR BoxMin = XMVectorSubtract(vCenter, vExtents);
    XMVECTOR BoxMax = XMVectorAdd(vCenter, vExtents);

    // Test the axes of the box (in effect test the AAB against the minimal AAB
    // around the triangle).
    XMVECTOR TriMin = XMVectorMin(XMVectorMin(V0, V1), V2);
    XMVECTOR TriMax = XMVectorMax(XMVectorMax(V0, V1), V2);

    // for each i in (x, y, z) if a_min(i) > b_max(i) or b_min(i) > a_max(i) then disjoint
    XMVECTOR Disjoint = XMVectorOrInt(XMVectorGreater(TriMin, BoxMax), XMVectorGreater(BoxMin, TriMax));
    if (DirectX::Internal::XMVector3AnyTrue(Disjoint))
        return false;

    // Test the plane of the triangle.
    XMVECTOR Normal = XMVector3Cross(XMVectorSubtract(V1, V0), XMVectorSubtract(V2, V0));
    XMVECTOR Dist = XMVector3Dot(Normal, V0);

    // Assert that the triangle is not degenerate.
    assert(!XMVector3Equal(Normal, Zero));

    // for each i in (x, y, z) if n(i) >= 0 then v_min(i)=b_min(i), v_max(i)=b_max(i)
    // else v_min(i)=b_max(i), v_max(i)=b_min(i)
    XMVECTOR NormalSelect = XMVectorGreater(Normal, Zero);
    XMVECTOR V_Min = XMVectorSelect(BoxMax, BoxMin, NormalSelect);
    XMVECTOR V_Max = XMVectorSelect(BoxMin, BoxMax, NormalSelect);

    // if n dot v_min + d > 0 || n dot v_max + d < 0 then disjoint
    XMVECTOR MinDist = XMVector3Dot(V_Min, Normal);
    XMVECTOR MaxDist = XMVector3Dot(V_Max, Normal);

    XMVECTOR NoIntersection = XMVectorGreater(MinDist, Dist);
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(MaxDist, Dist));

    // Move the box center to zero to simplify the following tests.
    XMVECTOR TV0 = XMVectorSubtract(V0, vCenter);
    XMVECTOR TV1 = XMVectorSubtract(V1, vCenter);
    XMVECTOR TV2 = XMVectorSubtract(V2, vCenter);

    // Test the edge/edge axes (3*3).
    XMVECTOR e0 = XMVectorSubtract(TV1, TV0);
    XMVECTOR e1 = XMVectorSubtract(TV2, TV1);
    XMVECTOR e2 = XMVectorSubtract(TV0, TV2);

    // Make w zero.
    e0 = XMVectorInsert<0, 0, 0, 0, 1>(e0, Zero);
    e1 = XMVectorInsert<0, 0, 0, 0, 1>(e1, Zero);
    e2 = XMVectorInsert<0, 0, 0, 0, 1>(e2, Zero);

    XMVECTOR Axis;
    XMVECTOR p0, p1, p2;
    XMVECTOR Min, Max;
    XMVECTOR Radius;

    // Axis == (1,0,0) x e0 = (0, -e0.z, e0.y)
    Axis = XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_0X>(e0, XMVectorNegate(e0));
    p0 = XMVector3Dot(TV0, Axis);
    // p1 = XMVector3Dot( V1, Axis ); // p1 = p0;
    p2 = XMVector3Dot(TV2, Axis);
    Min = XMVectorMin(p0, p2);
    Max = XMVectorMax(p0, p2);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (1,0,0) x e1 = (0, -e1.z, e1.y)
    Axis = XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_0X>(e1, XMVectorNegate(e1));
    p0 = XMVector3Dot(TV0, Axis);
    p1 = XMVector3Dot(TV1, Axis);
    // p2 = XMVector3Dot( V2, Axis ); // p2 = p1;
    Min = XMVectorMin(p0, p1);
    Max = XMVectorMax(p0, p1);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (1,0,0) x e2 = (0, -e2.z, e2.y)
    Axis = XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_0X>(e2, XMVectorNegate(e2));
    p0 = XMVector3Dot(TV0, Axis);
    p1 = XMVector3Dot(TV1, Axis);
    // p2 = XMVector3Dot( V2, Axis ); // p2 = p0;
    Min = XMVectorMin(p0, p1);
    Max = XMVectorMax(p0, p1);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (0,1,0) x e0 = (e0.z, 0, -e0.x)
    Axis = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0W, XM_PERMUTE_1X, XM_PERMUTE_0Y>(e0, XMVectorNegate(e0));
    p0 = XMVector3Dot(TV0, Axis);
    // p1 = XMVector3Dot( V1, Axis ); // p1 = p0;
    p2 = XMVector3Dot(TV2, Axis);
    Min = XMVectorMin(p0, p2);
    Max = XMVectorMax(p0, p2);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (0,1,0) x e1 = (e1.z, 0, -e1.x)
    Axis = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0W, XM_PERMUTE_1X, XM_PERMUTE_0Y>(e1, XMVectorNegate(e1));
    p0 = XMVector3Dot(TV0, Axis);
    p1 = XMVector3Dot(TV1, Axis);
    // p2 = XMVector3Dot( V2, Axis ); // p2 = p1;
    Min = XMVectorMin(p0, p1);
    Max = XMVectorMax(p0, p1);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (0,0,1) x e2 = (e2.z, 0, -e2.x)
    Axis = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0W, XM_PERMUTE_1X, XM_PERMUTE_0Y>(e2, XMVectorNegate(e2));
    p0 = XMVector3Dot(TV0, Axis);
    p1 = XMVector3Dot(TV1, Axis);
    // p2 = XMVector3Dot( V2, Axis ); // p2 = p0;
    Min = XMVectorMin(p0, p1);
    Max = XMVectorMax(p0, p1);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (0,0,1) x e0 = (-e0.y, e0.x, 0)
    Axis = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_0Z>(e0, XMVectorNegate(e0));
    p0 = XMVector3Dot(TV0, Axis);
    // p1 = XMVector3Dot( V1, Axis ); // p1 = p0;
    p2 = XMVector3Dot(TV2, Axis);
    Min = XMVectorMin(p0, p2);
    Max = XMVectorMax(p0, p2);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (0,0,1) x e1 = (-e1.y, e1.x, 0)
    Axis = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_0Z>(e1, XMVectorNegate(e1));
    p0 = XMVector3Dot(TV0, Axis);
    p1 = XMVector3Dot(TV1, Axis);
    // p2 = XMVector3Dot( V2, Axis ); // p2 = p1;
    Min = XMVectorMin(p0, p1);
    Max = XMVectorMax(p0, p1);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    // Axis == (0,0,1) x e2 = (-e2.y, e2.x, 0)
    Axis = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_0Z>(e2, XMVectorNegate(e2));
    p0 = XMVector3Dot(TV0, Axis);
    p1 = XMVector3Dot(TV1, Axis);
    // p2 = XMVector3Dot( V2, Axis ); // p2 = p0;
    Min = XMVectorMin(p0, p1);
    Max = XMVectorMax(p0, p1);
    Radius = XMVector3Dot(vExtents, XMVectorAbs(Axis));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(Min, Radius));
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(Max, XMVectorNegate(Radius)));

    return XMVector4NotEqualInt(NoIntersection, XMVectorTrueInt());
}


//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline PlaneIntersectionType XM_CALLCONV BoundingBox::Intersects(FXMVECTOR Plane) const noexcept
{
    assert(DirectX::Internal::XMPlaneIsUnit(Plane));

    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    // Set w of the center to one so we can dot4 with a plane.
    vCenter = XMVectorInsert<0, 0, 0, 0, 1>(vCenter, XMVectorSplatOne());

    XMVECTOR Outside, Inside;
    DirectX::Internal::FastIntersectAxisAlignedBoxPlane(vCenter, vExtents, Plane, Outside, Inside);

    // If the box is outside any plane it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return FRONT;

    // If the box is inside all planes it is inside.
    if (XMVector4EqualInt(Inside, XMVectorTrueInt()))
        return BACK;

    // The box is not inside all planes or outside a plane it intersects.
    return INTERSECTING;
}


//-----------------------------------------------------------------------------
// Compute the intersection of a ray (Origin, Direction) with an axis aligned
// box using the slabs method.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingBox::Intersects(FXMVECTOR Origin, FXMVECTOR Direction, float& Dist) const noexcept
{
    assert(DirectX::Internal::XMVector3IsUnit(Direction));

    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    // Adjust ray origin to be relative to center of the box.
    XMVECTOR TOrigin = XMVectorSubtract(vCenter, Origin);

    // Compute the dot product againt each axis of the box.
    // Since the axii are (1,0,0), (0,1,0), (0,0,1) no computation is necessary.
    XMVECTOR AxisDotOrigin = TOrigin;
    XMVECTOR AxisDotDirection = Direction;

    // if (fabs(AxisDotDirection) <= Epsilon) the ray is nearly parallel to the slab.
    XMVECTOR IsParallel = XMVectorLessOrEqual(XMVectorAbs(AxisDotDirection), g_RayEpsilon);

    // Test against all three axii simultaneously.
    XMVECTOR InverseAxisDotDirection = XMVectorReciprocal(AxisDotDirection);
    XMVECTOR t1 = XMVectorMultiply(XMVectorSubtract(AxisDotOrigin, vExtents), InverseAxisDotDirection);
    XMVECTOR t2 = XMVectorMultiply(XMVectorAdd(AxisDotOrigin, vExtents), InverseAxisDotDirection);

    // Compute the max of min(t1,t2) and the min of max(t1,t2) ensuring we don't
    // use the results from any directions parallel to the slab.
    XMVECTOR t_min = XMVectorSelect(XMVectorMin(t1, t2), g_FltMin, IsParallel);
    XMVECTOR t_max = XMVectorSelect(XMVectorMax(t1, t2), g_FltMax, IsParallel);

    // t_min.x = maximum( t_min.x, t_min.y, t_min.z );
    // t_max.x = minimum( t_max.x, t_max.y, t_max.z );
    t_min = XMVectorMax(t_min, XMVectorSplatY(t_min));  // x = max(x,y)
    t_min = XMVectorMax(t_min, XMVectorSplatZ(t_min));  // x = max(max(x,y),z)
    t_max = XMVectorMin(t_max, XMVectorSplatY(t_max));  // x = min(x,y)
    t_max = XMVectorMin(t_max, XMVectorSplatZ(t_max));  // x = min(min(x,y),z)

    // if ( t_min > t_max ) return false;
    XMVECTOR NoIntersection = XMVectorGreater(XMVectorSplatX(t_min), XMVectorSplatX(t_max));

    // if ( t_max < 0.0f ) return false;
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(XMVectorSplatX(t_max), XMVectorZero()));

    // if (IsParallel && (-Extents > AxisDotOrigin || Extents < AxisDotOrigin)) return false;
    XMVECTOR ParallelOverlap = XMVectorInBounds(AxisDotOrigin, vExtents);
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorAndCInt(IsParallel, ParallelOverlap));

    if (!DirectX::Internal::XMVector3AnyTrue(NoIntersection))
    {
        // Store the x-component to *pDist
        XMStoreFloat(&Dist, t_min);
        return true;
    }

    Dist = 0.f;
    return false;
}


//-----------------------------------------------------------------------------
// Test an axis alinged box vs 6 planes (typically forming a frustum).
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingBox::ContainedBy(
    FXMVECTOR Plane0, FXMVECTOR Plane1, FXMVECTOR Plane2,
    GXMVECTOR Plane3,
    HXMVECTOR Plane4, HXMVECTOR Plane5) const noexcept
{
    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);

    // Set w of the center to one so we can dot4 with a plane.
    vCenter = XMVectorInsert<0, 0, 0, 0, 1>(vCenter, XMVectorSplatOne());

    XMVECTOR Outside, Inside;

    // Test against each plane.
    DirectX::Internal::FastIntersectAxisAlignedBoxPlane(vCenter, vExtents, Plane0, Outside, Inside);

    XMVECTOR AnyOutside = Outside;
    XMVECTOR AllInside = Inside;

    DirectX::Internal::FastIntersectAxisAlignedBoxPlane(vCenter, vExtents, Plane1, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectAxisAlignedBoxPlane(vCenter, vExtents, Plane2, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectAxisAlignedBoxPlane(vCenter, vExtents, Plane3, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectAxisAlignedBoxPlane(vCenter, vExtents, Plane4, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectAxisAlignedBoxPlane(vCenter, vExtents, Plane5, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    // If the box is outside any plane it is outside.
    if (XMVector4EqualInt(AnyOutside, XMVectorTrueInt()))
        return DISJOINT;

    // If the box is inside all planes it is inside.
    if (XMVector4EqualInt(AllInside, XMVectorTrueInt()))
        return CONTAINS;

    // The box is not inside all planes or outside a plane, it may intersect.
    return INTERSECTS;
}


//-----------------------------------------------------------------------------
// Create axis-aligned box that contains two other bounding boxes
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingBox::CreateMerged(BoundingBox& Out, const BoundingBox& b1, const BoundingBox& b2) noexcept
{
    XMVECTOR b1Center = XMLoadFloat3(&b1.Center);
    XMVECTOR b1Extents = XMLoadFloat3(&b1.Extents);

    XMVECTOR b2Center = XMLoadFloat3(&b2.Center);
    XMVECTOR b2Extents = XMLoadFloat3(&b2.Extents);

    XMVECTOR Min = XMVectorSubtract(b1Center, b1Extents);
    Min = XMVectorMin(Min, XMVectorSubtract(b2Center, b2Extents));

    XMVECTOR Max = XMVectorAdd(b1Center, b1Extents);
    Max = XMVectorMax(Max, XMVectorAdd(b2Center, b2Extents));

    assert(XMVector3LessOrEqual(Min, Max));

    XMStoreFloat3(&Out.Center, XMVectorScale(XMVectorAdd(Min, Max), 0.5f));
    XMStoreFloat3(&Out.Extents, XMVectorScale(XMVectorSubtract(Max, Min), 0.5f));
}


//-----------------------------------------------------------------------------
// Create axis-aligned box that contains a bounding sphere
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingBox::CreateFromSphere(BoundingBox& Out, const BoundingSphere& sh) noexcept
{
    XMVECTOR spCenter = XMLoadFloat3(&sh.Center);
    XMVECTOR shRadius = XMVectorReplicatePtr(&sh.Radius);

    XMVECTOR Min = XMVectorSubtract(spCenter, shRadius);
    XMVECTOR Max = XMVectorAdd(spCenter, shRadius);

    assert(XMVector3LessOrEqual(Min, Max));

    XMStoreFloat3(&Out.Center, XMVectorScale(XMVectorAdd(Min, Max), 0.5f));
    XMStoreFloat3(&Out.Extents, XMVectorScale(XMVectorSubtract(Max, Min), 0.5f));
}


//-----------------------------------------------------------------------------
// Create axis-aligned box from min/max points
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV BoundingBox::CreateFromPoints(BoundingBox& Out, FXMVECTOR pt1, FXMVECTOR pt2) noexcept
{
    XMVECTOR Min = XMVectorMin(pt1, pt2);
    XMVECTOR Max = XMVectorMax(pt1, pt2);

    // Store center and extents.
    XMStoreFloat3(&Out.Center, XMVectorScale(XMVectorAdd(Min, Max), 0.5f));
    XMStoreFloat3(&Out.Extents, XMVectorScale(XMVectorSubtract(Max, Min), 0.5f));
}


//-----------------------------------------------------------------------------
// Find the minimum axis aligned bounding box containing a set of points.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingBox::CreateFromPoints(BoundingBox& Out, size_t Count, const XMFLOAT3* pPoints, size_t Stride) noexcept
{
    assert(Count > 0);
    assert(pPoints);

    // Find the minimum and maximum x, y, and z
    XMVECTOR vMin, vMax;

    vMin = vMax = XMLoadFloat3(pPoints);

    for (size_t i = 1; i < Count; ++i)
    {
        XMVECTOR Point = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(reinterpret_cast<const uint8_t*>(pPoints) + i * Stride));

        vMin = XMVectorMin(vMin, Point);
        vMax = XMVectorMax(vMax, Point);
    }

    // Store center and extents.
    XMStoreFloat3(&Out.Center, XMVectorScale(XMVectorAdd(vMin, vMax), 0.5f));
    XMStoreFloat3(&Out.Extents, XMVectorScale(XMVectorSubtract(vMax, vMin), 0.5f));
}


/****************************************************************************
 *
 * BoundingOrientedBox
 *
 ****************************************************************************/

 //-----------------------------------------------------------------------------
 // Transform an oriented box by an angle preserving transform.
 //-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV BoundingOrientedBox::Transform(BoundingOrientedBox& Out, FXMMATRIX M) const noexcept
{
    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Composite the box rotation and the transform rotation.
    XMMATRIX nM;
    nM.r[0] = XMVector3Normalize(M.r[0]);
    nM.r[1] = XMVector3Normalize(M.r[1]);
    nM.r[2] = XMVector3Normalize(M.r[2]);
    nM.r[3] = g_XMIdentityR3;
    XMVECTOR Rotation = XMQuaternionRotationMatrix(nM);
    vOrientation = XMQuaternionMultiply(vOrientation, Rotation);

    // Transform the center.
    vCenter = XMVector3Transform(vCenter, M);

    // Scale the box extents.
    XMVECTOR dX = XMVector3Length(M.r[0]);
    XMVECTOR dY = XMVector3Length(M.r[1]);
    XMVECTOR dZ = XMVector3Length(M.r[2]);

    XMVECTOR VectorScale = XMVectorSelect(dY, dX, g_XMSelect1000);
    VectorScale = XMVectorSelect(dZ, VectorScale, g_XMSelect1100);
    vExtents = XMVectorMultiply(vExtents, VectorScale);

    // Store the box.
    XMStoreFloat3(&Out.Center, vCenter);
    XMStoreFloat3(&Out.Extents, vExtents);
    XMStoreFloat4(&Out.Orientation, vOrientation);
}

_Use_decl_annotations_
inline void XM_CALLCONV BoundingOrientedBox::Transform(BoundingOrientedBox& Out, float Scale, FXMVECTOR Rotation, FXMVECTOR Translation) const noexcept
{
    assert(DirectX::Internal::XMQuaternionIsUnit(Rotation));

    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Composite the box rotation and the transform rotation.
    vOrientation = XMQuaternionMultiply(vOrientation, Rotation);

    // Transform the center.
    XMVECTOR VectorScale = XMVectorReplicate(Scale);
    vCenter = XMVectorAdd(XMVector3Rotate(XMVectorMultiply(vCenter, VectorScale), Rotation), Translation);

    // Scale the box extents.
    vExtents = XMVectorMultiply(vExtents, VectorScale);

    // Store the box.
    XMStoreFloat3(&Out.Center, vCenter);
    XMStoreFloat3(&Out.Extents, vExtents);
    XMStoreFloat4(&Out.Orientation, vOrientation);
}


//-----------------------------------------------------------------------------
// Get the corner points of the box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingOrientedBox::GetCorners(XMFLOAT3* Corners) const noexcept
{
    assert(Corners != nullptr);

    // Load the box
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    for (size_t i = 0; i < CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVectorAdd(XMVector3Rotate(XMVectorMultiply(vExtents, g_BoxOffset[i]), vOrientation), vCenter);
        XMStoreFloat3(&Corners[i], C);
    }
}


//-----------------------------------------------------------------------------
// Point in oriented box test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingOrientedBox::Contains(FXMVECTOR Point) const noexcept
{
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Transform the point to be local to the box.
    XMVECTOR TPoint = XMVector3InverseRotate(XMVectorSubtract(Point, vCenter), vOrientation);

    return XMVector3InBounds(TPoint, vExtents) ? CONTAINS : DISJOINT;
}


//-----------------------------------------------------------------------------
// Triangle in oriented bounding box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingOrientedBox::Contains(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    // Load the box center & orientation.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Transform the triangle vertices into the space of the box.
    XMVECTOR TV0 = XMVector3InverseRotate(XMVectorSubtract(V0, vCenter), vOrientation);
    XMVECTOR TV1 = XMVector3InverseRotate(XMVectorSubtract(V1, vCenter), vOrientation);
    XMVECTOR TV2 = XMVector3InverseRotate(XMVectorSubtract(V2, vCenter), vOrientation);

    BoundingBox box;
    box.Center = XMFLOAT3(0.0f, 0.0f, 0.0f);
    box.Extents = Extents;

    // Use the triangle vs axis aligned box intersection routine.
    return box.Contains(TV0, TV1, TV2);
}


//-----------------------------------------------------------------------------
// Sphere in oriented bounding box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingOrientedBox::Contains(const BoundingSphere& sh) const noexcept
{
    XMVECTOR SphereCenter = XMLoadFloat3(&sh.Center);
    XMVECTOR SphereRadius = XMVectorReplicatePtr(&sh.Radius);

    XMVECTOR BoxCenter = XMLoadFloat3(&Center);
    XMVECTOR BoxExtents = XMLoadFloat3(&Extents);
    XMVECTOR BoxOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(BoxOrientation));

    // Transform the center of the sphere to be local to the box.
    // BoxMin = -BoxExtents
    // BoxMax = +BoxExtents
    SphereCenter = XMVector3InverseRotate(XMVectorSubtract(SphereCenter, BoxCenter), BoxOrientation);

    // Find the distance to the nearest point on the box.
    // for each i in (x, y, z)
    // if (SphereCenter(i) < BoxMin(i)) d2 += (SphereCenter(i) - BoxMin(i)) ^ 2
    // else if (SphereCenter(i) > BoxMax(i)) d2 += (SphereCenter(i) - BoxMax(i)) ^ 2

    XMVECTOR d = XMVectorZero();

    // Compute d for each dimension.
    XMVECTOR LessThanMin = XMVectorLess(SphereCenter, XMVectorNegate(BoxExtents));
    XMVECTOR GreaterThanMax = XMVectorGreater(SphereCenter, BoxExtents);

    XMVECTOR MinDelta = XMVectorAdd(SphereCenter, BoxExtents);
    XMVECTOR MaxDelta = XMVectorSubtract(SphereCenter, BoxExtents);

    // Choose value for each dimension based on the comparison.
    d = XMVectorSelect(d, MinDelta, LessThanMin);
    d = XMVectorSelect(d, MaxDelta, GreaterThanMax);

    // Use a dot-product to square them and sum them together.
    XMVECTOR d2 = XMVector3Dot(d, d);
    XMVECTOR SphereRadiusSq = XMVectorMultiply(SphereRadius, SphereRadius);

    if (XMVector4Greater(d2, SphereRadiusSq))
        return DISJOINT;

    // See if we are completely inside the box
    XMVECTOR SMin = XMVectorSubtract(SphereCenter, SphereRadius);
    XMVECTOR SMax = XMVectorAdd(SphereCenter, SphereRadius);

    return (XMVector3InBounds(SMin, BoxExtents) && XMVector3InBounds(SMax, BoxExtents)) ? CONTAINS : INTERSECTS;
}


//-----------------------------------------------------------------------------
// Axis aligned box vs. oriented box. Constructs an oriented box and uses
// the oriented box vs. oriented box test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingOrientedBox::Contains(const BoundingBox& box) const noexcept
{
    // Make the axis aligned box oriented and do an OBB vs OBB test.
    BoundingOrientedBox obox(box.Center, box.Extents, XMFLOAT4(0.f, 0.f, 0.f, 1.f));
    return Contains(obox);
}


//-----------------------------------------------------------------------------
// Oriented bounding box in oriented bounding box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingOrientedBox::Contains(const BoundingOrientedBox& box) const noexcept
{
    if (!Intersects(box))
        return DISJOINT;

    // Load the boxes
    XMVECTOR aCenter = XMLoadFloat3(&Center);
    XMVECTOR aExtents = XMLoadFloat3(&Extents);
    XMVECTOR aOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(aOrientation));

    XMVECTOR bCenter = XMLoadFloat3(&box.Center);
    XMVECTOR bExtents = XMLoadFloat3(&box.Extents);
    XMVECTOR bOrientation = XMLoadFloat4(&box.Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(bOrientation));

    XMVECTOR offset = XMVectorSubtract(bCenter, aCenter);

    for (size_t i = 0; i < CORNER_COUNT; ++i)
    {
        // Cb = rotate( bExtents * corneroffset[i], bOrientation ) + bcenter
        // Ca = invrotate( Cb - aCenter, aOrientation )

        XMVECTOR C = XMVectorAdd(XMVector3Rotate(XMVectorMultiply(bExtents, g_BoxOffset[i]), bOrientation), offset);
        C = XMVector3InverseRotate(C, aOrientation);

        if (!XMVector3InBounds(C, aExtents))
            return INTERSECTS;
    }

    return CONTAINS;
}


//-----------------------------------------------------------------------------
// Frustum in oriented bounding box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingOrientedBox::Contains(const BoundingFrustum& fr) const noexcept
{
    if (!fr.Intersects(*this))
        return DISJOINT;

    XMFLOAT3 Corners[BoundingFrustum::CORNER_COUNT];
    fr.GetCorners(Corners);

    // Load the box
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    for (size_t i = 0; i < BoundingFrustum::CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVector3InverseRotate(XMVectorSubtract(XMLoadFloat3(&Corners[i]), vCenter), vOrientation);

        if (!XMVector3InBounds(C, vExtents))
            return INTERSECTS;
    }

    return CONTAINS;
}


//-----------------------------------------------------------------------------
// Sphere vs. oriented box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingOrientedBox::Intersects(const BoundingSphere& sh) const noexcept
{
    XMVECTOR SphereCenter = XMLoadFloat3(&sh.Center);
    XMVECTOR SphereRadius = XMVectorReplicatePtr(&sh.Radius);

    XMVECTOR BoxCenter = XMLoadFloat3(&Center);
    XMVECTOR BoxExtents = XMLoadFloat3(&Extents);
    XMVECTOR BoxOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(BoxOrientation));

    // Transform the center of the sphere to be local to the box.
    // BoxMin = -BoxExtents
    // BoxMax = +BoxExtents
    SphereCenter = XMVector3InverseRotate(XMVectorSubtract(SphereCenter, BoxCenter), BoxOrientation);

    // Find the distance to the nearest point on the box.
    // for each i in (x, y, z)
    // if (SphereCenter(i) < BoxMin(i)) d2 += (SphereCenter(i) - BoxMin(i)) ^ 2
    // else if (SphereCenter(i) > BoxMax(i)) d2 += (SphereCenter(i) - BoxMax(i)) ^ 2

    XMVECTOR d = XMVectorZero();

    // Compute d for each dimension.
    XMVECTOR LessThanMin = XMVectorLess(SphereCenter, XMVectorNegate(BoxExtents));
    XMVECTOR GreaterThanMax = XMVectorGreater(SphereCenter, BoxExtents);

    XMVECTOR MinDelta = XMVectorAdd(SphereCenter, BoxExtents);
    XMVECTOR MaxDelta = XMVectorSubtract(SphereCenter, BoxExtents);

    // Choose value for each dimension based on the comparison.
    d = XMVectorSelect(d, MinDelta, LessThanMin);
    d = XMVectorSelect(d, MaxDelta, GreaterThanMax);

    // Use a dot-product to square them and sum them together.
    XMVECTOR d2 = XMVector3Dot(d, d);

    return XMVector4LessOrEqual(d2, XMVectorMultiply(SphereRadius, SphereRadius)) ? true : false;
}


//-----------------------------------------------------------------------------
// Axis aligned box vs. oriented box. Constructs an oriented box and uses
// the oriented box vs. oriented box test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingOrientedBox::Intersects(const BoundingBox& box) const noexcept
{
    // Make the axis aligned box oriented and do an OBB vs OBB test.
    BoundingOrientedBox obox(box.Center, box.Extents, XMFLOAT4(0.f, 0.f, 0.f, 1.f));
    return Intersects(obox);
}


//-----------------------------------------------------------------------------
// Fast oriented box / oriented box intersection test using the separating axis
// theorem.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingOrientedBox::Intersects(const BoundingOrientedBox& box) const noexcept
{
    // Build the 3x3 rotation matrix that defines the orientation of B relative to A.
    XMVECTOR A_quat = XMLoadFloat4(&Orientation);
    XMVECTOR B_quat = XMLoadFloat4(&box.Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(A_quat));
    assert(DirectX::Internal::XMQuaternionIsUnit(B_quat));

    XMVECTOR Q = XMQuaternionMultiply(A_quat, XMQuaternionConjugate(B_quat));
    XMMATRIX R = XMMatrixRotationQuaternion(Q);

    // Compute the translation of B relative to A.
    XMVECTOR A_cent = XMLoadFloat3(&Center);
    XMVECTOR B_cent = XMLoadFloat3(&box.Center);
    XMVECTOR t = XMVector3InverseRotate(XMVectorSubtract(B_cent, A_cent), A_quat);

    //
    // h(A) = extents of A.
    // h(B) = extents of B.
    //
    // a(u) = axes of A = (1,0,0), (0,1,0), (0,0,1)
    // b(u) = axes of B relative to A = (r00,r10,r20), (r01,r11,r21), (r02,r12,r22)
    //
    // For each possible separating axis l:
    //   d(A) = sum (for i = u,v,w) h(A)(i) * abs( a(i) dot l )
    //   d(B) = sum (for i = u,v,w) h(B)(i) * abs( b(i) dot l )
    //   if abs( t dot l ) > d(A) + d(B) then disjoint
    //

    // Load extents of A and B.
    XMVECTOR h_A = XMLoadFloat3(&Extents);
    XMVECTOR h_B = XMLoadFloat3(&box.Extents);

    // Rows. Note R[0,1,2]X.w = 0.
    XMVECTOR R0X = R.r[0];
    XMVECTOR R1X = R.r[1];
    XMVECTOR R2X = R.r[2];

    R = XMMatrixTranspose(R);

    // Columns. Note RX[0,1,2].w = 0.
    XMVECTOR RX0 = R.r[0];
    XMVECTOR RX1 = R.r[1];
    XMVECTOR RX2 = R.r[2];

    // Absolute value of rows.
    XMVECTOR AR0X = XMVectorAbs(R0X);
    XMVECTOR AR1X = XMVectorAbs(R1X);
    XMVECTOR AR2X = XMVectorAbs(R2X);

    // Absolute value of columns.
    XMVECTOR ARX0 = XMVectorAbs(RX0);
    XMVECTOR ARX1 = XMVectorAbs(RX1);
    XMVECTOR ARX2 = XMVectorAbs(RX2);

    // Test each of the 15 possible seperating axii.
    XMVECTOR d, d_A, d_B;

    // l = a(u) = (1, 0, 0)
    // t dot l = t.x
    // d(A) = h(A).x
    // d(B) = h(B) dot abs(r00, r01, r02)
    d = XMVectorSplatX(t);
    d_A = XMVectorSplatX(h_A);
    d_B = XMVector3Dot(h_B, AR0X);
    XMVECTOR NoIntersection = XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B));

    // l = a(v) = (0, 1, 0)
    // t dot l = t.y
    // d(A) = h(A).y
    // d(B) = h(B) dot abs(r10, r11, r12)
    d = XMVectorSplatY(t);
    d_A = XMVectorSplatY(h_A);
    d_B = XMVector3Dot(h_B, AR1X);
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(w) = (0, 0, 1)
    // t dot l = t.z
    // d(A) = h(A).z
    // d(B) = h(B) dot abs(r20, r21, r22)
    d = XMVectorSplatZ(t);
    d_A = XMVectorSplatZ(h_A);
    d_B = XMVector3Dot(h_B, AR2X);
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = b(u) = (r00, r10, r20)
    // d(A) = h(A) dot abs(r00, r10, r20)
    // d(B) = h(B).x
    d = XMVector3Dot(t, RX0);
    d_A = XMVector3Dot(h_A, ARX0);
    d_B = XMVectorSplatX(h_B);
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = b(v) = (r01, r11, r21)
    // d(A) = h(A) dot abs(r01, r11, r21)
    // d(B) = h(B).y
    d = XMVector3Dot(t, RX1);
    d_A = XMVector3Dot(h_A, ARX1);
    d_B = XMVectorSplatY(h_B);
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = b(w) = (r02, r12, r22)
    // d(A) = h(A) dot abs(r02, r12, r22)
    // d(B) = h(B).z
    d = XMVector3Dot(t, RX2);
    d_A = XMVector3Dot(h_A, ARX2);
    d_B = XMVectorSplatZ(h_B);
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(u) x b(u) = (0, -r20, r10)
    // d(A) = h(A) dot abs(0, r20, r10)
    // d(B) = h(B) dot abs(0, r02, r01)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_0X>(RX0, XMVectorNegate(RX0)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_X>(ARX0));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_X>(AR0X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(u) x b(v) = (0, -r21, r11)
    // d(A) = h(A) dot abs(0, r21, r11)
    // d(B) = h(B) dot abs(r02, 0, r00)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_0X>(RX1, XMVectorNegate(RX1)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_X>(ARX1));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Y>(AR0X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(u) x b(w) = (0, -r22, r12)
    // d(A) = h(A) dot abs(0, r22, r12)
    // d(B) = h(B) dot abs(r01, r00, 0)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_0X>(RX2, XMVectorNegate(RX2)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_X>(ARX2));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_Z>(AR0X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(v) x b(u) = (r20, 0, -r00)
    // d(A) = h(A) dot abs(r20, 0, r00)
    // d(B) = h(B) dot abs(0, r12, r11)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0W, XM_PERMUTE_1X, XM_PERMUTE_0Y>(RX0, XMVectorNegate(RX0)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Y>(ARX0));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_X>(AR1X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(v) x b(v) = (r21, 0, -r01)
    // d(A) = h(A) dot abs(r21, 0, r01)
    // d(B) = h(B) dot abs(r12, 0, r10)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0W, XM_PERMUTE_1X, XM_PERMUTE_0Y>(RX1, XMVectorNegate(RX1)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Y>(ARX1));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Y>(AR1X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(v) x b(w) = (r22, 0, -r02)
    // d(A) = h(A) dot abs(r22, 0, r02)
    // d(B) = h(B) dot abs(r11, r10, 0)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0W, XM_PERMUTE_1X, XM_PERMUTE_0Y>(RX2, XMVectorNegate(RX2)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Y>(ARX2));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_Z>(AR1X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(w) x b(u) = (-r10, r00, 0)
    // d(A) = h(A) dot abs(r10, r00, 0)
    // d(B) = h(B) dot abs(0, r22, r21)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_0Z>(RX0, XMVectorNegate(RX0)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_Z>(ARX0));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_X>(AR2X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(w) x b(v) = (-r11, r01, 0)
    // d(A) = h(A) dot abs(r11, r01, 0)
    // d(B) = h(B) dot abs(r22, 0, r20)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_0Z>(RX1, XMVectorNegate(RX1)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_Z>(ARX1));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Y>(AR2X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // l = a(w) x b(w) = (-r12, r02, 0)
    // d(A) = h(A) dot abs(r12, r02, 0)
    // d(B) = h(B) dot abs(r21, r20, 0)
    d = XMVector3Dot(t, XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_0Z>(RX2, XMVectorNegate(RX2)));
    d_A = XMVector3Dot(h_A, XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_Z>(ARX2));
    d_B = XMVector3Dot(h_B, XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_Z>(AR2X));
    NoIntersection = XMVectorOrInt(NoIntersection,
        XMVectorGreater(XMVectorAbs(d), XMVectorAdd(d_A, d_B)));

    // No seperating axis found, boxes must intersect.
    return XMVector4NotEqualInt(NoIntersection, XMVectorTrueInt()) ? true : false;
}


//-----------------------------------------------------------------------------
// Frustum vs. oriented box test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingOrientedBox::Intersects(const BoundingFrustum& fr) const noexcept
{
    return fr.Intersects(*this);
}


//-----------------------------------------------------------------------------
// Triangle vs. oriented box test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingOrientedBox::Intersects(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    // Load the box center & orientation.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Transform the triangle vertices into the space of the box.
    XMVECTOR TV0 = XMVector3InverseRotate(XMVectorSubtract(V0, vCenter), vOrientation);
    XMVECTOR TV1 = XMVector3InverseRotate(XMVectorSubtract(V1, vCenter), vOrientation);
    XMVECTOR TV2 = XMVector3InverseRotate(XMVectorSubtract(V2, vCenter), vOrientation);

    BoundingBox box;
    box.Center = XMFLOAT3(0.0f, 0.0f, 0.0f);
    box.Extents = Extents;

    // Use the triangle vs axis aligned box intersection routine.
    return box.Intersects(TV0, TV1, TV2);
}


//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline PlaneIntersectionType XM_CALLCONV BoundingOrientedBox::Intersects(FXMVECTOR Plane) const noexcept
{
    assert(DirectX::Internal::XMPlaneIsUnit(Plane));

    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR BoxOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(BoxOrientation));

    // Set w of the center to one so we can dot4 with a plane.
    vCenter = XMVectorInsert<0, 0, 0, 0, 1>(vCenter, XMVectorSplatOne());

    // Build the 3x3 rotation matrix that defines the box axes.
    XMMATRIX R = XMMatrixRotationQuaternion(BoxOrientation);

    XMVECTOR Outside, Inside;
    DirectX::Internal::FastIntersectOrientedBoxPlane(vCenter, vExtents, R.r[0], R.r[1], R.r[2], Plane, Outside, Inside);

    // If the box is outside any plane it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return FRONT;

    // If the box is inside all planes it is inside.
    if (XMVector4EqualInt(Inside, XMVectorTrueInt()))
        return BACK;

    // The box is not inside all planes or outside a plane it intersects.
    return INTERSECTING;
}


//-----------------------------------------------------------------------------
// Compute the intersection of a ray (Origin, Direction) with an oriented box
// using the slabs method.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingOrientedBox::Intersects(FXMVECTOR Origin, FXMVECTOR Direction, float& Dist) const noexcept
{
    assert(DirectX::Internal::XMVector3IsUnit(Direction));

    static const XMVECTORU32 SelectY = { { { XM_SELECT_0, XM_SELECT_1, XM_SELECT_0, XM_SELECT_0 } } };
    static const XMVECTORU32 SelectZ = { { { XM_SELECT_0, XM_SELECT_0, XM_SELECT_1, XM_SELECT_0 } } };

    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Get the boxes normalized side directions.
    XMMATRIX R = XMMatrixRotationQuaternion(vOrientation);

    // Adjust ray origin to be relative to center of the box.
    XMVECTOR TOrigin = XMVectorSubtract(vCenter, Origin);

    // Compute the dot product againt each axis of the box.
    XMVECTOR AxisDotOrigin = XMVector3Dot(R.r[0], TOrigin);
    AxisDotOrigin = XMVectorSelect(AxisDotOrigin, XMVector3Dot(R.r[1], TOrigin), SelectY);
    AxisDotOrigin = XMVectorSelect(AxisDotOrigin, XMVector3Dot(R.r[2], TOrigin), SelectZ);

    XMVECTOR AxisDotDirection = XMVector3Dot(R.r[0], Direction);
    AxisDotDirection = XMVectorSelect(AxisDotDirection, XMVector3Dot(R.r[1], Direction), SelectY);
    AxisDotDirection = XMVectorSelect(AxisDotDirection, XMVector3Dot(R.r[2], Direction), SelectZ);

    // if (fabs(AxisDotDirection) <= Epsilon) the ray is nearly parallel to the slab.
    XMVECTOR IsParallel = XMVectorLessOrEqual(XMVectorAbs(AxisDotDirection), g_RayEpsilon);

    // Test against all three axes simultaneously.
    XMVECTOR InverseAxisDotDirection = XMVectorReciprocal(AxisDotDirection);
    XMVECTOR t1 = XMVectorMultiply(XMVectorSubtract(AxisDotOrigin, vExtents), InverseAxisDotDirection);
    XMVECTOR t2 = XMVectorMultiply(XMVectorAdd(AxisDotOrigin, vExtents), InverseAxisDotDirection);

    // Compute the max of min(t1,t2) and the min of max(t1,t2) ensuring we don't
    // use the results from any directions parallel to the slab.
    XMVECTOR t_min = XMVectorSelect(XMVectorMin(t1, t2), g_FltMin, IsParallel);
    XMVECTOR t_max = XMVectorSelect(XMVectorMax(t1, t2), g_FltMax, IsParallel);

    // t_min.x = maximum( t_min.x, t_min.y, t_min.z );
    // t_max.x = minimum( t_max.x, t_max.y, t_max.z );
    t_min = XMVectorMax(t_min, XMVectorSplatY(t_min));  // x = max(x,y)
    t_min = XMVectorMax(t_min, XMVectorSplatZ(t_min));  // x = max(max(x,y),z)
    t_max = XMVectorMin(t_max, XMVectorSplatY(t_max));  // x = min(x,y)
    t_max = XMVectorMin(t_max, XMVectorSplatZ(t_max));  // x = min(min(x,y),z)

    // if ( t_min > t_max ) return false;
    XMVECTOR NoIntersection = XMVectorGreater(XMVectorSplatX(t_min), XMVectorSplatX(t_max));

    // if ( t_max < 0.0f ) return false;
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(XMVectorSplatX(t_max), XMVectorZero()));

    // if (IsParallel && (-Extents > AxisDotOrigin || Extents < AxisDotOrigin)) return false;
    XMVECTOR ParallelOverlap = XMVectorInBounds(AxisDotOrigin, vExtents);
    NoIntersection = XMVectorOrInt(NoIntersection, XMVectorAndCInt(IsParallel, ParallelOverlap));

    if (!DirectX::Internal::XMVector3AnyTrue(NoIntersection))
    {
        // Store the x-component to *pDist
        XMStoreFloat(&Dist, t_min);
        return true;
    }

    Dist = 0.f;
    return false;
}


//-----------------------------------------------------------------------------
// Test an oriented box vs 6 planes (typically forming a frustum).
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingOrientedBox::ContainedBy(
    FXMVECTOR Plane0, FXMVECTOR Plane1, FXMVECTOR Plane2,
    GXMVECTOR Plane3,
    HXMVECTOR Plane4, HXMVECTOR Plane5) const noexcept
{
    // Load the box.
    XMVECTOR vCenter = XMLoadFloat3(&Center);
    XMVECTOR vExtents = XMLoadFloat3(&Extents);
    XMVECTOR BoxOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(BoxOrientation));

    // Set w of the center to one so we can dot4 with a plane.
    vCenter = XMVectorInsert<0, 0, 0, 0, 1>(vCenter, XMVectorSplatOne());

    // Build the 3x3 rotation matrix that defines the box axes.
    XMMATRIX R = XMMatrixRotationQuaternion(BoxOrientation);

    XMVECTOR Outside, Inside;

    // Test against each plane.
    DirectX::Internal::FastIntersectOrientedBoxPlane(vCenter, vExtents, R.r[0], R.r[1], R.r[2], Plane0, Outside, Inside);

    XMVECTOR AnyOutside = Outside;
    XMVECTOR AllInside = Inside;

    DirectX::Internal::FastIntersectOrientedBoxPlane(vCenter, vExtents, R.r[0], R.r[1], R.r[2], Plane1, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectOrientedBoxPlane(vCenter, vExtents, R.r[0], R.r[1], R.r[2], Plane2, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectOrientedBoxPlane(vCenter, vExtents, R.r[0], R.r[1], R.r[2], Plane3, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectOrientedBoxPlane(vCenter, vExtents, R.r[0], R.r[1], R.r[2], Plane4, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectOrientedBoxPlane(vCenter, vExtents, R.r[0], R.r[1], R.r[2], Plane5, Outside, Inside);
    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    // If the box is outside any plane it is outside.
    if (XMVector4EqualInt(AnyOutside, XMVectorTrueInt()))
        return DISJOINT;

    // If the box is inside all planes it is inside.
    if (XMVector4EqualInt(AllInside, XMVectorTrueInt()))
        return CONTAINS;

    // The box is not inside all planes or outside a plane, it may intersect.
    return INTERSECTS;
}


//-----------------------------------------------------------------------------
// Create oriented bounding box from axis-aligned bounding box
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingOrientedBox::CreateFromBoundingBox(BoundingOrientedBox& Out, const BoundingBox& box) noexcept
{
    Out.Center = box.Center;
    Out.Extents = box.Extents;
    Out.Orientation = XMFLOAT4(0.f, 0.f, 0.f, 1.f);
}


//-----------------------------------------------------------------------------
// Find the approximate minimum oriented bounding box containing a set of
// points.  Exact computation of minimum oriented bounding box is possible but
// is slower and requires a more complex algorithm.
// The algorithm works by computing the inertia tensor of the points and then
// using the eigenvectors of the intertia tensor as the axes of the box.
// Computing the intertia tensor of the convex hull of the points will usually
// result in better bounding box but the computation is more complex.
// Exact computation of the minimum oriented bounding box is possible but the
// best know algorithm is O(N^3) and is significanly more complex to implement.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingOrientedBox::CreateFromPoints(BoundingOrientedBox& Out, size_t Count, const XMFLOAT3* pPoints, size_t Stride) noexcept
{
    assert(Count > 0);
    assert(pPoints != nullptr);

    XMVECTOR CenterOfMass = XMVectorZero();

    // Compute the center of mass and inertia tensor of the points.
    for (size_t i = 0; i < Count; ++i)
    {
        XMVECTOR Point = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(reinterpret_cast<const uint8_t*>(pPoints) + i * Stride));

        CenterOfMass = XMVectorAdd(CenterOfMass, Point);
    }

    CenterOfMass = XMVectorMultiply(CenterOfMass, XMVectorReciprocal(XMVectorReplicate(float(Count))));

    // Compute the inertia tensor of the points around the center of mass.
    // Using the center of mass is not strictly necessary, but will hopefully
    // improve the stability of finding the eigenvectors.
    XMVECTOR XX_YY_ZZ = XMVectorZero();
    XMVECTOR XY_XZ_YZ = XMVectorZero();

    for (size_t i = 0; i < Count; ++i)
    {
        XMVECTOR Point = XMVectorSubtract(XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(reinterpret_cast<const uint8_t*>(pPoints) + i * Stride)), CenterOfMass);

        XX_YY_ZZ = XMVectorAdd(XX_YY_ZZ, XMVectorMultiply(Point, Point));

        XMVECTOR XXY = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_W>(Point);
        XMVECTOR YZZ = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_Z, XM_SWIZZLE_W>(Point);

        XY_XZ_YZ = XMVectorAdd(XY_XZ_YZ, XMVectorMultiply(XXY, YZZ));
    }

    XMVECTOR v1, v2, v3;

    // Compute the eigenvectors of the inertia tensor.
    DirectX::Internal::CalculateEigenVectorsFromCovarianceMatrix(XMVectorGetX(XX_YY_ZZ), XMVectorGetY(XX_YY_ZZ),
        XMVectorGetZ(XX_YY_ZZ),
        XMVectorGetX(XY_XZ_YZ), XMVectorGetY(XY_XZ_YZ),
        XMVectorGetZ(XY_XZ_YZ),
        &v1, &v2, &v3);

    // Put them in a matrix.
    XMMATRIX R;

    R.r[0] = XMVectorSetW(v1, 0.f);
    R.r[1] = XMVectorSetW(v2, 0.f);
    R.r[2] = XMVectorSetW(v3, 0.f);
    R.r[3] = g_XMIdentityR3.v;

    // Multiply by -1 to convert the matrix into a right handed coordinate
    // system (Det ~= 1) in case the eigenvectors form a left handed
    // coordinate system (Det ~= -1) because XMQuaternionRotationMatrix only
    // works on right handed matrices.
    XMVECTOR Det = XMMatrixDeterminant(R);

    if (XMVector4Less(Det, XMVectorZero()))
    {
        R.r[0] = XMVectorMultiply(R.r[0], g_XMNegativeOne.v);
        R.r[1] = XMVectorMultiply(R.r[1], g_XMNegativeOne.v);
        R.r[2] = XMVectorMultiply(R.r[2], g_XMNegativeOne.v);
    }

    // Get the rotation quaternion from the matrix.
    XMVECTOR vOrientation = XMQuaternionRotationMatrix(R);

    // Make sure it is normal (in case the vectors are slightly non-orthogonal).
    vOrientation = XMQuaternionNormalize(vOrientation);

    // Rebuild the rotation matrix from the quaternion.
    R = XMMatrixRotationQuaternion(vOrientation);

    // Build the rotation into the rotated space.
    XMMATRIX InverseR = XMMatrixTranspose(R);

    // Find the minimum OBB using the eigenvectors as the axes.
    XMVECTOR vMin, vMax;

    vMin = vMax = XMVector3TransformNormal(XMLoadFloat3(pPoints), InverseR);

    for (size_t i = 1; i < Count; ++i)
    {
        XMVECTOR Point = XMVector3TransformNormal(XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(reinterpret_cast<const uint8_t*>(pPoints) + i * Stride)),
            InverseR);

        vMin = XMVectorMin(vMin, Point);
        vMax = XMVectorMax(vMax, Point);
    }

    // Rotate the center into world space.
    XMVECTOR vCenter = XMVectorScale(XMVectorAdd(vMin, vMax), 0.5f);
    vCenter = XMVector3TransformNormal(vCenter, R);

    // Store center, extents, and orientation.
    XMStoreFloat3(&Out.Center, vCenter);
    XMStoreFloat3(&Out.Extents, XMVectorScale(XMVectorSubtract(vMax, vMin), 0.5f));
    XMStoreFloat4(&Out.Orientation, vOrientation);
}


/****************************************************************************
 *
 * BoundingFrustum
 *
 ****************************************************************************/

_Use_decl_annotations_
inline BoundingFrustum::BoundingFrustum(CXMMATRIX Projection, bool rhcoords) noexcept
{
    CreateFromMatrix(*this, Projection, rhcoords);
}


//-----------------------------------------------------------------------------
// Transform a frustum by an angle preserving transform.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV BoundingFrustum::Transform(BoundingFrustum& Out, FXMMATRIX M) const noexcept
{
    // Load the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Composite the frustum rotation and the transform rotation
    XMMATRIX nM;
    nM.r[0] = XMVector3Normalize(M.r[0]);
    nM.r[1] = XMVector3Normalize(M.r[1]);
    nM.r[2] = XMVector3Normalize(M.r[2]);
    nM.r[3] = g_XMIdentityR3;
    XMVECTOR Rotation = XMQuaternionRotationMatrix(nM);
    vOrientation = XMQuaternionMultiply(vOrientation, Rotation);

    // Transform the center.
    vOrigin = XMVector3Transform(vOrigin, M);

    // Store the frustum.
    XMStoreFloat3(&Out.Origin, vOrigin);
    XMStoreFloat4(&Out.Orientation, vOrientation);

    // Scale the near and far distances (the slopes remain the same).
    XMVECTOR dX = XMVector3Dot(M.r[0], M.r[0]);
    XMVECTOR dY = XMVector3Dot(M.r[1], M.r[1]);
    XMVECTOR dZ = XMVector3Dot(M.r[2], M.r[2]);

    XMVECTOR d = XMVectorMax(dX, XMVectorMax(dY, dZ));
    float Scale = sqrtf(XMVectorGetX(d));

    Out.Near = Near * Scale;
    Out.Far = Far * Scale;

    // Copy the slopes.
    Out.RightSlope = RightSlope;
    Out.LeftSlope = LeftSlope;
    Out.TopSlope = TopSlope;
    Out.BottomSlope = BottomSlope;
}

_Use_decl_annotations_
inline void XM_CALLCONV BoundingFrustum::Transform(BoundingFrustum& Out, float Scale, FXMVECTOR Rotation, FXMVECTOR Translation) const noexcept
{
    assert(DirectX::Internal::XMQuaternionIsUnit(Rotation));

    // Load the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Composite the frustum rotation and the transform rotation.
    vOrientation = XMQuaternionMultiply(vOrientation, Rotation);

    // Transform the origin.
    vOrigin = XMVectorAdd(XMVector3Rotate(XMVectorScale(vOrigin, Scale), Rotation), Translation);

    // Store the frustum.
    XMStoreFloat3(&Out.Origin, vOrigin);
    XMStoreFloat4(&Out.Orientation, vOrientation);

    // Scale the near and far distances (the slopes remain the same).
    Out.Near = Near * Scale;
    Out.Far = Far * Scale;

    // Copy the slopes.
    Out.RightSlope = RightSlope;
    Out.LeftSlope = LeftSlope;
    Out.TopSlope = TopSlope;
    Out.BottomSlope = BottomSlope;
}


//-----------------------------------------------------------------------------
// Get the corner points of the frustum
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingFrustum::GetCorners(XMFLOAT3* Corners) const noexcept
{
    assert(Corners != nullptr);

    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Build the corners of the frustum.
    XMVECTOR vRightTop = XMVectorSet(RightSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vRightBottom = XMVectorSet(RightSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vLeftTop = XMVectorSet(LeftSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vLeftBottom = XMVectorSet(LeftSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vNear = XMVectorReplicatePtr(&Near);
    XMVECTOR vFar = XMVectorReplicatePtr(&Far);

    // Returns 8 corners position of bounding frustum.
    //     Near    Far
    //    0----1  4----5
    //    |    |  |    |
    //    |    |  |    |
    //    3----2  7----6

    XMVECTOR vCorners[CORNER_COUNT];
    vCorners[0] = XMVectorMultiply(vLeftTop, vNear);
    vCorners[1] = XMVectorMultiply(vRightTop, vNear);
    vCorners[2] = XMVectorMultiply(vRightBottom, vNear);
    vCorners[3] = XMVectorMultiply(vLeftBottom, vNear);
    vCorners[4] = XMVectorMultiply(vLeftTop, vFar);
    vCorners[5] = XMVectorMultiply(vRightTop, vFar);
    vCorners[6] = XMVectorMultiply(vRightBottom, vFar);
    vCorners[7] = XMVectorMultiply(vLeftBottom, vFar);

    for (size_t i = 0; i < CORNER_COUNT; ++i)
    {
        XMVECTOR C = XMVectorAdd(XMVector3Rotate(vCorners[i], vOrientation), vOrigin);
        XMStoreFloat3(&Corners[i], C);
    }
}


//-----------------------------------------------------------------------------
// Point in frustum test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingFrustum::Contains(FXMVECTOR Point) const noexcept
{
    // Build frustum planes.
    XMVECTOR Planes[6];
    Planes[0] = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    Planes[1] = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    Planes[2] = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    Planes[3] = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    Planes[4] = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    Planes[5] = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);

    // Load origin and orientation.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Transform point into local space of frustum.
    XMVECTOR TPoint = XMVector3InverseRotate(XMVectorSubtract(Point, vOrigin), vOrientation);

    // Set w to one.
    TPoint = XMVectorInsert<0, 0, 0, 0, 1>(TPoint, XMVectorSplatOne());

    XMVECTOR Zero = XMVectorZero();
    XMVECTOR Outside = Zero;

    // Test point against each plane of the frustum.
    for (size_t i = 0; i < 6; ++i)
    {
        XMVECTOR Dot = XMVector4Dot(TPoint, Planes[i]);
        Outside = XMVectorOrInt(Outside, XMVectorGreater(Dot, Zero));
    }

    return XMVector4NotEqualInt(Outside, XMVectorTrueInt()) ? CONTAINS : DISJOINT;
}


//-----------------------------------------------------------------------------
// Triangle vs frustum test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingFrustum::Contains(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Create 6 planes (do it inline to encourage use of registers)
    XMVECTOR NearPlane = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    NearPlane = DirectX::Internal::XMPlaneTransform(NearPlane, vOrientation, vOrigin);
    NearPlane = XMPlaneNormalize(NearPlane);

    XMVECTOR FarPlane = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    FarPlane = DirectX::Internal::XMPlaneTransform(FarPlane, vOrientation, vOrigin);
    FarPlane = XMPlaneNormalize(FarPlane);

    XMVECTOR RightPlane = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    RightPlane = DirectX::Internal::XMPlaneTransform(RightPlane, vOrientation, vOrigin);
    RightPlane = XMPlaneNormalize(RightPlane);

    XMVECTOR LeftPlane = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    LeftPlane = DirectX::Internal::XMPlaneTransform(LeftPlane, vOrientation, vOrigin);
    LeftPlane = XMPlaneNormalize(LeftPlane);

    XMVECTOR TopPlane = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    TopPlane = DirectX::Internal::XMPlaneTransform(TopPlane, vOrientation, vOrigin);
    TopPlane = XMPlaneNormalize(TopPlane);

    XMVECTOR BottomPlane = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);
    BottomPlane = DirectX::Internal::XMPlaneTransform(BottomPlane, vOrientation, vOrigin);
    BottomPlane = XMPlaneNormalize(BottomPlane);

    return TriangleTests::ContainedBy(V0, V1, V2, NearPlane, FarPlane, RightPlane, LeftPlane, TopPlane, BottomPlane);
}


//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingFrustum::Contains(const BoundingSphere& sh) const noexcept
{
    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Create 6 planes (do it inline to encourage use of registers)
    XMVECTOR NearPlane = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    NearPlane = DirectX::Internal::XMPlaneTransform(NearPlane, vOrientation, vOrigin);
    NearPlane = XMPlaneNormalize(NearPlane);

    XMVECTOR FarPlane = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    FarPlane = DirectX::Internal::XMPlaneTransform(FarPlane, vOrientation, vOrigin);
    FarPlane = XMPlaneNormalize(FarPlane);

    XMVECTOR RightPlane = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    RightPlane = DirectX::Internal::XMPlaneTransform(RightPlane, vOrientation, vOrigin);
    RightPlane = XMPlaneNormalize(RightPlane);

    XMVECTOR LeftPlane = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    LeftPlane = DirectX::Internal::XMPlaneTransform(LeftPlane, vOrientation, vOrigin);
    LeftPlane = XMPlaneNormalize(LeftPlane);

    XMVECTOR TopPlane = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    TopPlane = DirectX::Internal::XMPlaneTransform(TopPlane, vOrientation, vOrigin);
    TopPlane = XMPlaneNormalize(TopPlane);

    XMVECTOR BottomPlane = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);
    BottomPlane = DirectX::Internal::XMPlaneTransform(BottomPlane, vOrientation, vOrigin);
    BottomPlane = XMPlaneNormalize(BottomPlane);

    return sh.ContainedBy(NearPlane, FarPlane, RightPlane, LeftPlane, TopPlane, BottomPlane);
}


//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingFrustum::Contains(const BoundingBox& box) const noexcept
{
    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Create 6 planes (do it inline to encourage use of registers)
    XMVECTOR NearPlane = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    NearPlane = DirectX::Internal::XMPlaneTransform(NearPlane, vOrientation, vOrigin);
    NearPlane = XMPlaneNormalize(NearPlane);

    XMVECTOR FarPlane = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    FarPlane = DirectX::Internal::XMPlaneTransform(FarPlane, vOrientation, vOrigin);
    FarPlane = XMPlaneNormalize(FarPlane);

    XMVECTOR RightPlane = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    RightPlane = DirectX::Internal::XMPlaneTransform(RightPlane, vOrientation, vOrigin);
    RightPlane = XMPlaneNormalize(RightPlane);

    XMVECTOR LeftPlane = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    LeftPlane = DirectX::Internal::XMPlaneTransform(LeftPlane, vOrientation, vOrigin);
    LeftPlane = XMPlaneNormalize(LeftPlane);

    XMVECTOR TopPlane = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    TopPlane = DirectX::Internal::XMPlaneTransform(TopPlane, vOrientation, vOrigin);
    TopPlane = XMPlaneNormalize(TopPlane);

    XMVECTOR BottomPlane = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);
    BottomPlane = DirectX::Internal::XMPlaneTransform(BottomPlane, vOrientation, vOrigin);
    BottomPlane = XMPlaneNormalize(BottomPlane);

    return box.ContainedBy(NearPlane, FarPlane, RightPlane, LeftPlane, TopPlane, BottomPlane);
}


//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingFrustum::Contains(const BoundingOrientedBox& box) const noexcept
{
    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Create 6 planes (do it inline to encourage use of registers)
    XMVECTOR NearPlane = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    NearPlane = DirectX::Internal::XMPlaneTransform(NearPlane, vOrientation, vOrigin);
    NearPlane = XMPlaneNormalize(NearPlane);

    XMVECTOR FarPlane = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    FarPlane = DirectX::Internal::XMPlaneTransform(FarPlane, vOrientation, vOrigin);
    FarPlane = XMPlaneNormalize(FarPlane);

    XMVECTOR RightPlane = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    RightPlane = DirectX::Internal::XMPlaneTransform(RightPlane, vOrientation, vOrigin);
    RightPlane = XMPlaneNormalize(RightPlane);

    XMVECTOR LeftPlane = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    LeftPlane = DirectX::Internal::XMPlaneTransform(LeftPlane, vOrientation, vOrigin);
    LeftPlane = XMPlaneNormalize(LeftPlane);

    XMVECTOR TopPlane = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    TopPlane = DirectX::Internal::XMPlaneTransform(TopPlane, vOrientation, vOrigin);
    TopPlane = XMPlaneNormalize(TopPlane);

    XMVECTOR BottomPlane = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);
    BottomPlane = DirectX::Internal::XMPlaneTransform(BottomPlane, vOrientation, vOrigin);
    BottomPlane = XMPlaneNormalize(BottomPlane);

    return box.ContainedBy(NearPlane, FarPlane, RightPlane, LeftPlane, TopPlane, BottomPlane);
}


//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType BoundingFrustum::Contains(const BoundingFrustum& fr) const noexcept
{
    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    // Create 6 planes (do it inline to encourage use of registers)
    XMVECTOR NearPlane = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    NearPlane = DirectX::Internal::XMPlaneTransform(NearPlane, vOrientation, vOrigin);
    NearPlane = XMPlaneNormalize(NearPlane);

    XMVECTOR FarPlane = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    FarPlane = DirectX::Internal::XMPlaneTransform(FarPlane, vOrientation, vOrigin);
    FarPlane = XMPlaneNormalize(FarPlane);

    XMVECTOR RightPlane = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    RightPlane = DirectX::Internal::XMPlaneTransform(RightPlane, vOrientation, vOrigin);
    RightPlane = XMPlaneNormalize(RightPlane);

    XMVECTOR LeftPlane = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    LeftPlane = DirectX::Internal::XMPlaneTransform(LeftPlane, vOrientation, vOrigin);
    LeftPlane = XMPlaneNormalize(LeftPlane);

    XMVECTOR TopPlane = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    TopPlane = DirectX::Internal::XMPlaneTransform(TopPlane, vOrientation, vOrigin);
    TopPlane = XMPlaneNormalize(TopPlane);

    XMVECTOR BottomPlane = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);
    BottomPlane = DirectX::Internal::XMPlaneTransform(BottomPlane, vOrientation, vOrigin);
    BottomPlane = XMPlaneNormalize(BottomPlane);

    return fr.ContainedBy(NearPlane, FarPlane, RightPlane, LeftPlane, TopPlane, BottomPlane);
}


//-----------------------------------------------------------------------------
// Exact sphere vs frustum test.  The algorithm first checks the sphere against
// the planes of the frustum, then if the plane checks were indeterminate finds
// the nearest feature (plane, line, point) on the frustum to the center of the
// sphere and compares the distance to the nearest feature to the radius of the
// sphere
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingFrustum::Intersects(const BoundingSphere& sh) const noexcept
{
    XMVECTOR Zero = XMVectorZero();

    // Build the frustum planes.
    XMVECTOR Planes[6];
    Planes[0] = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    Planes[1] = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    Planes[2] = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    Planes[3] = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    Planes[4] = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    Planes[5] = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);

    // Normalize the planes so we can compare to the sphere radius.
    Planes[2] = XMVector3Normalize(Planes[2]);
    Planes[3] = XMVector3Normalize(Planes[3]);
    Planes[4] = XMVector3Normalize(Planes[4]);
    Planes[5] = XMVector3Normalize(Planes[5]);

    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Load the sphere.
    XMVECTOR vCenter = XMLoadFloat3(&sh.Center);
    XMVECTOR vRadius = XMVectorReplicatePtr(&sh.Radius);

    // Transform the center of the sphere into the local space of frustum.
    vCenter = XMVector3InverseRotate(XMVectorSubtract(vCenter, vOrigin), vOrientation);

    // Set w of the center to one so we can dot4 with the plane.
    vCenter = XMVectorInsert<0, 0, 0, 0, 1>(vCenter, XMVectorSplatOne());

    // Check against each plane of the frustum.
    XMVECTOR Outside = XMVectorFalseInt();
    XMVECTOR InsideAll = XMVectorTrueInt();
    XMVECTOR CenterInsideAll = XMVectorTrueInt();

    XMVECTOR Dist[6];

    for (size_t i = 0; i < 6; ++i)
    {
        Dist[i] = XMVector4Dot(vCenter, Planes[i]);

        // Outside the plane?
        Outside = XMVectorOrInt(Outside, XMVectorGreater(Dist[i], vRadius));

        // Fully inside the plane?
        InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(Dist[i], XMVectorNegate(vRadius)));

        // Check if the center is inside the plane.
        CenterInsideAll = XMVectorAndInt(CenterInsideAll, XMVectorLessOrEqual(Dist[i], Zero));
    }

    // If the sphere is outside any of the planes it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // If the sphere is inside all planes it is fully inside.
    if (XMVector4EqualInt(InsideAll, XMVectorTrueInt()))
        return true;

    // If the center of the sphere is inside all planes and the sphere intersects
    // one or more planes then it must intersect.
    if (XMVector4EqualInt(CenterInsideAll, XMVectorTrueInt()))
        return true;

    // The sphere may be outside the frustum or intersecting the frustum.
    // Find the nearest feature (face, edge, or corner) on the frustum
    // to the sphere.

    // The faces adjacent to each face are:
    static const size_t adjacent_faces[6][4] =
    {
        { 2, 3, 4, 5 },    // 0
        { 2, 3, 4, 5 },    // 1
        { 0, 1, 4, 5 },    // 2
        { 0, 1, 4, 5 },    // 3
        { 0, 1, 2, 3 },    // 4
        { 0, 1, 2, 3 }
    };  // 5

    XMVECTOR Intersects = XMVectorFalseInt();

    // Check to see if the nearest feature is one of the planes.
    for (size_t i = 0; i < 6; ++i)
    {
        // Find the nearest point on the plane to the center of the sphere.
        XMVECTOR Point = XMVectorNegativeMultiplySubtract(Planes[i], Dist[i], vCenter);

        // Set w of the point to one.
        Point = XMVectorInsert<0, 0, 0, 0, 1>(Point, XMVectorSplatOne());

        // If the point is inside the face (inside the adjacent planes) then
        // this plane is the nearest feature.
        XMVECTOR InsideFace = XMVectorTrueInt();

        for (size_t j = 0; j < 4; j++)
        {
            size_t plane_index = adjacent_faces[i][j];

            InsideFace = XMVectorAndInt(InsideFace,
                XMVectorLessOrEqual(XMVector4Dot(Point, Planes[plane_index]), Zero));
        }

        // Since we have already checked distance from the plane we know that the
        // sphere must intersect if this plane is the nearest feature.
        Intersects = XMVectorOrInt(Intersects,
            XMVectorAndInt(XMVectorGreater(Dist[i], Zero), InsideFace));
    }

    if (XMVector4EqualInt(Intersects, XMVectorTrueInt()))
        return true;

    // Build the corners of the frustum.
    XMVECTOR vRightTop = XMVectorSet(RightSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vRightBottom = XMVectorSet(RightSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vLeftTop = XMVectorSet(LeftSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vLeftBottom = XMVectorSet(LeftSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vNear = XMVectorReplicatePtr(&Near);
    XMVECTOR vFar = XMVectorReplicatePtr(&Far);

    XMVECTOR Corners[CORNER_COUNT];
    Corners[0] = XMVectorMultiply(vRightTop, vNear);
    Corners[1] = XMVectorMultiply(vRightBottom, vNear);
    Corners[2] = XMVectorMultiply(vLeftTop, vNear);
    Corners[3] = XMVectorMultiply(vLeftBottom, vNear);
    Corners[4] = XMVectorMultiply(vRightTop, vFar);
    Corners[5] = XMVectorMultiply(vRightBottom, vFar);
    Corners[6] = XMVectorMultiply(vLeftTop, vFar);
    Corners[7] = XMVectorMultiply(vLeftBottom, vFar);

    // The Edges are:
    static const size_t edges[12][2] =
    {
        { 0, 1 }, { 2, 3 }, { 0, 2 }, { 1, 3 },    // Near plane
        { 4, 5 }, { 6, 7 }, { 4, 6 }, { 5, 7 },    // Far plane
        { 0, 4 }, { 1, 5 }, { 2, 6 }, { 3, 7 },
    }; // Near to far

    XMVECTOR RadiusSq = XMVectorMultiply(vRadius, vRadius);

    // Check to see if the nearest feature is one of the edges (or corners).
    for (size_t i = 0; i < 12; ++i)
    {
        size_t ei0 = edges[i][0];
        size_t ei1 = edges[i][1];

        // Find the nearest point on the edge to the center of the sphere.
        // The corners of the frustum are included as the endpoints of the edges.
        XMVECTOR Point = DirectX::Internal::PointOnLineSegmentNearestPoint(Corners[ei0], Corners[ei1], vCenter);

        XMVECTOR Delta = XMVectorSubtract(vCenter, Point);

        XMVECTOR DistSq = XMVector3Dot(Delta, Delta);

        // If the distance to the center of the sphere to the point is less than
        // the radius of the sphere then it must intersect.
        Intersects = XMVectorOrInt(Intersects, XMVectorLessOrEqual(DistSq, RadiusSq));
    }

    if (XMVector4EqualInt(Intersects, XMVectorTrueInt()))
        return true;

    // The sphere must be outside the frustum.
    return false;
}


//-----------------------------------------------------------------------------
// Exact axis aligned box vs frustum test.  Constructs an oriented box and uses
// the oriented box vs frustum test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingFrustum::Intersects(const BoundingBox& box) const noexcept
{
    // Make the axis aligned box oriented and do an OBB vs frustum test.
    BoundingOrientedBox obox(box.Center, box.Extents, XMFLOAT4(0.f, 0.f, 0.f, 1.f));
    return Intersects(obox);
}


//-----------------------------------------------------------------------------
// Exact oriented box vs frustum test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingFrustum::Intersects(const BoundingOrientedBox& box) const noexcept
{
    static const XMVECTORU32 SelectY = { { { XM_SELECT_0, XM_SELECT_1, XM_SELECT_0, XM_SELECT_0 } } };
    static const XMVECTORU32 SelectZ = { { { XM_SELECT_0, XM_SELECT_0, XM_SELECT_1, XM_SELECT_0 } } };

    XMVECTOR Zero = XMVectorZero();

    // Build the frustum planes.
    XMVECTOR Planes[6];
    Planes[0] = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    Planes[1] = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    Planes[2] = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    Planes[3] = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    Planes[4] = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    Planes[5] = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);

    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR FrustumOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(FrustumOrientation));

    // Load the box.
    XMVECTOR Center = XMLoadFloat3(&box.Center);
    XMVECTOR Extents = XMLoadFloat3(&box.Extents);
    XMVECTOR BoxOrientation = XMLoadFloat4(&box.Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(BoxOrientation));

    // Transform the oriented box into the space of the frustum in order to
    // minimize the number of transforms we have to do.
    Center = XMVector3InverseRotate(XMVectorSubtract(Center, vOrigin), FrustumOrientation);
    BoxOrientation = XMQuaternionMultiply(BoxOrientation, XMQuaternionConjugate(FrustumOrientation));

    // Set w of the center to one so we can dot4 with the plane.
    Center = XMVectorInsert<0, 0, 0, 0, 1>(Center, XMVectorSplatOne());

    // Build the 3x3 rotation matrix that defines the box axes.
    XMMATRIX R = XMMatrixRotationQuaternion(BoxOrientation);

    // Check against each plane of the frustum.
    XMVECTOR Outside = XMVectorFalseInt();
    XMVECTOR InsideAll = XMVectorTrueInt();
    XMVECTOR CenterInsideAll = XMVectorTrueInt();

    for (size_t i = 0; i < 6; ++i)
    {
        // Compute the distance to the center of the box.
        XMVECTOR Dist = XMVector4Dot(Center, Planes[i]);

        // Project the axes of the box onto the normal of the plane.  Half the
        // length of the projection (sometime called the "radius") is equal to
        // h(u) * abs(n dot b(u))) + h(v) * abs(n dot b(v)) + h(w) * abs(n dot b(w))
        // where h(i) are extents of the box, n is the plane normal, and b(i) are the
        // axes of the box.
        XMVECTOR Radius = XMVector3Dot(Planes[i], R.r[0]);
        Radius = XMVectorSelect(Radius, XMVector3Dot(Planes[i], R.r[1]), SelectY);
        Radius = XMVectorSelect(Radius, XMVector3Dot(Planes[i], R.r[2]), SelectZ);
        Radius = XMVector3Dot(Extents, XMVectorAbs(Radius));

        // Outside the plane?
        Outside = XMVectorOrInt(Outside, XMVectorGreater(Dist, Radius));

        // Fully inside the plane?
        InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(Dist, XMVectorNegate(Radius)));

        // Check if the center is inside the plane.
        CenterInsideAll = XMVectorAndInt(CenterInsideAll, XMVectorLessOrEqual(Dist, Zero));
    }

    // If the box is outside any of the planes it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // If the box is inside all planes it is fully inside.
    if (XMVector4EqualInt(InsideAll, XMVectorTrueInt()))
        return true;

    // If the center of the box is inside all planes and the box intersects
    // one or more planes then it must intersect.
    if (XMVector4EqualInt(CenterInsideAll, XMVectorTrueInt()))
        return true;

    // Build the corners of the frustum.
    XMVECTOR vRightTop = XMVectorSet(RightSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vRightBottom = XMVectorSet(RightSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vLeftTop = XMVectorSet(LeftSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vLeftBottom = XMVectorSet(LeftSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vNear = XMVectorReplicatePtr(&Near);
    XMVECTOR vFar = XMVectorReplicatePtr(&Far);

    XMVECTOR Corners[CORNER_COUNT];
    Corners[0] = XMVectorMultiply(vRightTop, vNear);
    Corners[1] = XMVectorMultiply(vRightBottom, vNear);
    Corners[2] = XMVectorMultiply(vLeftTop, vNear);
    Corners[3] = XMVectorMultiply(vLeftBottom, vNear);
    Corners[4] = XMVectorMultiply(vRightTop, vFar);
    Corners[5] = XMVectorMultiply(vRightBottom, vFar);
    Corners[6] = XMVectorMultiply(vLeftTop, vFar);
    Corners[7] = XMVectorMultiply(vLeftBottom, vFar);

    // Test against box axes (3)
    {
        // Find the min/max values of the projection of the frustum onto each axis.
        XMVECTOR FrustumMin, FrustumMax;

        FrustumMin = XMVector3Dot(Corners[0], R.r[0]);
        FrustumMin = XMVectorSelect(FrustumMin, XMVector3Dot(Corners[0], R.r[1]), SelectY);
        FrustumMin = XMVectorSelect(FrustumMin, XMVector3Dot(Corners[0], R.r[2]), SelectZ);
        FrustumMax = FrustumMin;

        for (size_t i = 1; i < BoundingOrientedBox::CORNER_COUNT; ++i)
        {
            XMVECTOR Temp = XMVector3Dot(Corners[i], R.r[0]);
            Temp = XMVectorSelect(Temp, XMVector3Dot(Corners[i], R.r[1]), SelectY);
            Temp = XMVectorSelect(Temp, XMVector3Dot(Corners[i], R.r[2]), SelectZ);

            FrustumMin = XMVectorMin(FrustumMin, Temp);
            FrustumMax = XMVectorMax(FrustumMax, Temp);
        }

        // Project the center of the box onto the axes.
        XMVECTOR BoxDist = XMVector3Dot(Center, R.r[0]);
        BoxDist = XMVectorSelect(BoxDist, XMVector3Dot(Center, R.r[1]), SelectY);
        BoxDist = XMVectorSelect(BoxDist, XMVector3Dot(Center, R.r[2]), SelectZ);

        // The projection of the box onto the axis is just its Center and Extents.
        // if (min > box_max || max < box_min) reject;
        XMVECTOR Result = XMVectorOrInt(XMVectorGreater(FrustumMin, XMVectorAdd(BoxDist, Extents)),
            XMVectorLess(FrustumMax, XMVectorSubtract(BoxDist, Extents)));

        if (DirectX::Internal::XMVector3AnyTrue(Result))
            return false;
    }

    // Test against edge/edge axes (3*6).
    XMVECTOR FrustumEdgeAxis[6];

    FrustumEdgeAxis[0] = vRightTop;
    FrustumEdgeAxis[1] = vRightBottom;
    FrustumEdgeAxis[2] = vLeftTop;
    FrustumEdgeAxis[3] = vLeftBottom;
    FrustumEdgeAxis[4] = XMVectorSubtract(vRightTop, vLeftTop);
    FrustumEdgeAxis[5] = XMVectorSubtract(vLeftBottom, vLeftTop);

    for (size_t i = 0; i < 3; ++i)
    {
        for (size_t j = 0; j < 6; j++)
        {
            // Compute the axis we are going to test.
            XMVECTOR Axis = XMVector3Cross(R.r[i], FrustumEdgeAxis[j]);

            // Find the min/max values of the projection of the frustum onto the axis.
            XMVECTOR FrustumMin, FrustumMax;

            FrustumMin = FrustumMax = XMVector3Dot(Axis, Corners[0]);

            for (size_t k = 1; k < CORNER_COUNT; k++)
            {
                XMVECTOR Temp = XMVector3Dot(Axis, Corners[k]);
                FrustumMin = XMVectorMin(FrustumMin, Temp);
                FrustumMax = XMVectorMax(FrustumMax, Temp);
            }

            // Project the center of the box onto the axis.
            XMVECTOR Dist = XMVector3Dot(Center, Axis);

            // Project the axes of the box onto the axis to find the "radius" of the box.
            XMVECTOR Radius = XMVector3Dot(Axis, R.r[0]);
            Radius = XMVectorSelect(Radius, XMVector3Dot(Axis, R.r[1]), SelectY);
            Radius = XMVectorSelect(Radius, XMVector3Dot(Axis, R.r[2]), SelectZ);
            Radius = XMVector3Dot(Extents, XMVectorAbs(Radius));

            // if (center > max + radius || center < min - radius) reject;
            Outside = XMVectorOrInt(Outside, XMVectorGreater(Dist, XMVectorAdd(FrustumMax, Radius)));
            Outside = XMVectorOrInt(Outside, XMVectorLess(Dist, XMVectorSubtract(FrustumMin, Radius)));
        }
    }

    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // If we did not find a separating plane then the box must intersect the frustum.
    return true;
}


//-----------------------------------------------------------------------------
// Exact frustum vs frustum test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool BoundingFrustum::Intersects(const BoundingFrustum& fr) const noexcept
{
    // Load origin and orientation of frustum B.
    XMVECTOR OriginB = XMLoadFloat3(&Origin);
    XMVECTOR OrientationB = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(OrientationB));

    // Build the planes of frustum B.
    XMVECTOR AxisB[6];
    AxisB[0] = XMVectorSet(0.0f, 0.0f, -1.0f, 0.0f);
    AxisB[1] = XMVectorSet(0.0f, 0.0f, 1.0f, 0.0f);
    AxisB[2] = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    AxisB[3] = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    AxisB[4] = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    AxisB[5] = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);

    XMVECTOR PlaneDistB[6];
    PlaneDistB[0] = XMVectorNegate(XMVectorReplicatePtr(&Near));
    PlaneDistB[1] = XMVectorReplicatePtr(&Far);
    PlaneDistB[2] = XMVectorZero();
    PlaneDistB[3] = XMVectorZero();
    PlaneDistB[4] = XMVectorZero();
    PlaneDistB[5] = XMVectorZero();

    // Load origin and orientation of frustum A.
    XMVECTOR OriginA = XMLoadFloat3(&fr.Origin);
    XMVECTOR OrientationA = XMLoadFloat4(&fr.Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(OrientationA));

    // Transform frustum A into the space of the frustum B in order to
    // minimize the number of transforms we have to do.
    OriginA = XMVector3InverseRotate(XMVectorSubtract(OriginA, OriginB), OrientationB);
    OrientationA = XMQuaternionMultiply(OrientationA, XMQuaternionConjugate(OrientationB));

    // Build the corners of frustum A (in the local space of B).
    XMVECTOR RightTopA = XMVectorSet(fr.RightSlope, fr.TopSlope, 1.0f, 0.0f);
    XMVECTOR RightBottomA = XMVectorSet(fr.RightSlope, fr.BottomSlope, 1.0f, 0.0f);
    XMVECTOR LeftTopA = XMVectorSet(fr.LeftSlope, fr.TopSlope, 1.0f, 0.0f);
    XMVECTOR LeftBottomA = XMVectorSet(fr.LeftSlope, fr.BottomSlope, 1.0f, 0.0f);
    XMVECTOR NearA = XMVectorReplicatePtr(&fr.Near);
    XMVECTOR FarA = XMVectorReplicatePtr(&fr.Far);

    RightTopA = XMVector3Rotate(RightTopA, OrientationA);
    RightBottomA = XMVector3Rotate(RightBottomA, OrientationA);
    LeftTopA = XMVector3Rotate(LeftTopA, OrientationA);
    LeftBottomA = XMVector3Rotate(LeftBottomA, OrientationA);

    XMVECTOR CornersA[CORNER_COUNT];
    CornersA[0] = XMVectorMultiplyAdd(RightTopA, NearA, OriginA);
    CornersA[1] = XMVectorMultiplyAdd(RightBottomA, NearA, OriginA);
    CornersA[2] = XMVectorMultiplyAdd(LeftTopA, NearA, OriginA);
    CornersA[3] = XMVectorMultiplyAdd(LeftBottomA, NearA, OriginA);
    CornersA[4] = XMVectorMultiplyAdd(RightTopA, FarA, OriginA);
    CornersA[5] = XMVectorMultiplyAdd(RightBottomA, FarA, OriginA);
    CornersA[6] = XMVectorMultiplyAdd(LeftTopA, FarA, OriginA);
    CornersA[7] = XMVectorMultiplyAdd(LeftBottomA, FarA, OriginA);

    // Check frustum A against each plane of frustum B.
    XMVECTOR Outside = XMVectorFalseInt();
    XMVECTOR InsideAll = XMVectorTrueInt();

    for (size_t i = 0; i < 6; ++i)
    {
        // Find the min/max projection of the frustum onto the plane normal.
        XMVECTOR Min, Max;

        Min = Max = XMVector3Dot(AxisB[i], CornersA[0]);

        for (size_t j = 1; j < CORNER_COUNT; j++)
        {
            XMVECTOR Temp = XMVector3Dot(AxisB[i], CornersA[j]);
            Min = XMVectorMin(Min, Temp);
            Max = XMVectorMax(Max, Temp);
        }

        // Outside the plane?
        Outside = XMVectorOrInt(Outside, XMVectorGreater(Min, PlaneDistB[i]));

        // Fully inside the plane?
        InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(Max, PlaneDistB[i]));
    }

    // If the frustum A is outside any of the planes of frustum B it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // If frustum A is inside all planes of frustum B it is fully inside.
    if (XMVector4EqualInt(InsideAll, XMVectorTrueInt()))
        return true;

    // Build the corners of frustum B.
    XMVECTOR RightTopB = XMVectorSet(RightSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR RightBottomB = XMVectorSet(RightSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR LeftTopB = XMVectorSet(LeftSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR LeftBottomB = XMVectorSet(LeftSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR NearB = XMVectorReplicatePtr(&Near);
    XMVECTOR FarB = XMVectorReplicatePtr(&Far);

    XMVECTOR CornersB[BoundingFrustum::CORNER_COUNT];
    CornersB[0] = XMVectorMultiply(RightTopB, NearB);
    CornersB[1] = XMVectorMultiply(RightBottomB, NearB);
    CornersB[2] = XMVectorMultiply(LeftTopB, NearB);
    CornersB[3] = XMVectorMultiply(LeftBottomB, NearB);
    CornersB[4] = XMVectorMultiply(RightTopB, FarB);
    CornersB[5] = XMVectorMultiply(RightBottomB, FarB);
    CornersB[6] = XMVectorMultiply(LeftTopB, FarB);
    CornersB[7] = XMVectorMultiply(LeftBottomB, FarB);

    // Build the planes of frustum A (in the local space of B).
    XMVECTOR AxisA[6];
    XMVECTOR PlaneDistA[6];

    AxisA[0] = XMVectorSet(0.0f, 0.0f, -1.0f, 0.0f);
    AxisA[1] = XMVectorSet(0.0f, 0.0f, 1.0f, 0.0f);
    AxisA[2] = XMVectorSet(1.0f, 0.0f, -fr.RightSlope, 0.0f);
    AxisA[3] = XMVectorSet(-1.0f, 0.0f, fr.LeftSlope, 0.0f);
    AxisA[4] = XMVectorSet(0.0f, 1.0f, -fr.TopSlope, 0.0f);
    AxisA[5] = XMVectorSet(0.0f, -1.0f, fr.BottomSlope, 0.0f);

    AxisA[0] = XMVector3Rotate(AxisA[0], OrientationA);
    AxisA[1] = XMVectorNegate(AxisA[0]);
    AxisA[2] = XMVector3Rotate(AxisA[2], OrientationA);
    AxisA[3] = XMVector3Rotate(AxisA[3], OrientationA);
    AxisA[4] = XMVector3Rotate(AxisA[4], OrientationA);
    AxisA[5] = XMVector3Rotate(AxisA[5], OrientationA);

    PlaneDistA[0] = XMVector3Dot(AxisA[0], CornersA[0]);  // Re-use corner on near plane.
    PlaneDistA[1] = XMVector3Dot(AxisA[1], CornersA[4]);  // Re-use corner on far plane.
    PlaneDistA[2] = XMVector3Dot(AxisA[2], OriginA);
    PlaneDistA[3] = XMVector3Dot(AxisA[3], OriginA);
    PlaneDistA[4] = XMVector3Dot(AxisA[4], OriginA);
    PlaneDistA[5] = XMVector3Dot(AxisA[5], OriginA);

    // Check each axis of frustum A for a seperating plane (5).
    for (size_t i = 0; i < 6; ++i)
    {
        // Find the minimum projection of the frustum onto the plane normal.
        XMVECTOR Min;

        Min = XMVector3Dot(AxisA[i], CornersB[0]);

        for (size_t j = 1; j < CORNER_COUNT; j++)
        {
            XMVECTOR Temp = XMVector3Dot(AxisA[i], CornersB[j]);
            Min = XMVectorMin(Min, Temp);
        }

        // Outside the plane?
        Outside = XMVectorOrInt(Outside, XMVectorGreater(Min, PlaneDistA[i]));
    }

    // If the frustum B is outside any of the planes of frustum A it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // Check edge/edge axes (6 * 6).
    XMVECTOR FrustumEdgeAxisA[6];
    FrustumEdgeAxisA[0] = RightTopA;
    FrustumEdgeAxisA[1] = RightBottomA;
    FrustumEdgeAxisA[2] = LeftTopA;
    FrustumEdgeAxisA[3] = LeftBottomA;
    FrustumEdgeAxisA[4] = XMVectorSubtract(RightTopA, LeftTopA);
    FrustumEdgeAxisA[5] = XMVectorSubtract(LeftBottomA, LeftTopA);

    XMVECTOR FrustumEdgeAxisB[6];
    FrustumEdgeAxisB[0] = RightTopB;
    FrustumEdgeAxisB[1] = RightBottomB;
    FrustumEdgeAxisB[2] = LeftTopB;
    FrustumEdgeAxisB[3] = LeftBottomB;
    FrustumEdgeAxisB[4] = XMVectorSubtract(RightTopB, LeftTopB);
    FrustumEdgeAxisB[5] = XMVectorSubtract(LeftBottomB, LeftTopB);

    for (size_t i = 0; i < 6; ++i)
    {
        for (size_t j = 0; j < 6; j++)
        {
            // Compute the axis we are going to test.
            XMVECTOR Axis = XMVector3Cross(FrustumEdgeAxisA[i], FrustumEdgeAxisB[j]);

            // Find the min/max values of the projection of both frustums onto the axis.
            XMVECTOR MinA, MaxA;
            XMVECTOR MinB, MaxB;

            MinA = MaxA = XMVector3Dot(Axis, CornersA[0]);
            MinB = MaxB = XMVector3Dot(Axis, CornersB[0]);

            for (size_t k = 1; k < CORNER_COUNT; k++)
            {
                XMVECTOR TempA = XMVector3Dot(Axis, CornersA[k]);
                MinA = XMVectorMin(MinA, TempA);
                MaxA = XMVectorMax(MaxA, TempA);

                XMVECTOR TempB = XMVector3Dot(Axis, CornersB[k]);
                MinB = XMVectorMin(MinB, TempB);
                MaxB = XMVectorMax(MaxB, TempB);
            }

            // if (MinA > MaxB || MinB > MaxA) reject
            Outside = XMVectorOrInt(Outside, XMVectorGreater(MinA, MaxB));
            Outside = XMVectorOrInt(Outside, XMVectorGreater(MinB, MaxA));
        }
    }

    // If there is a seperating plane, then the frustums do not intersect.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // If we did not find a separating plane then the frustums intersect.
    return true;
}


//-----------------------------------------------------------------------------
// Triangle vs frustum test.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingFrustum::Intersects(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2) const noexcept
{
    // Build the frustum planes (NOTE: D is negated from the usual).
    XMVECTOR Planes[6];
    Planes[0] = XMVectorSet(0.0f, 0.0f, -1.0f, -Near);
    Planes[1] = XMVectorSet(0.0f, 0.0f, 1.0f, Far);
    Planes[2] = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    Planes[3] = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    Planes[4] = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    Planes[5] = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);

    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Transform triangle into the local space of frustum.
    XMVECTOR TV0 = XMVector3InverseRotate(XMVectorSubtract(V0, vOrigin), vOrientation);
    XMVECTOR TV1 = XMVector3InverseRotate(XMVectorSubtract(V1, vOrigin), vOrientation);
    XMVECTOR TV2 = XMVector3InverseRotate(XMVectorSubtract(V2, vOrigin), vOrientation);

    // Test each vertex of the triangle against the frustum planes.
    XMVECTOR Outside = XMVectorFalseInt();
    XMVECTOR InsideAll = XMVectorTrueInt();

    for (size_t i = 0; i < 6; ++i)
    {
        XMVECTOR Dist0 = XMVector3Dot(TV0, Planes[i]);
        XMVECTOR Dist1 = XMVector3Dot(TV1, Planes[i]);
        XMVECTOR Dist2 = XMVector3Dot(TV2, Planes[i]);

        XMVECTOR MinDist = XMVectorMin(Dist0, Dist1);
        MinDist = XMVectorMin(MinDist, Dist2);
        XMVECTOR MaxDist = XMVectorMax(Dist0, Dist1);
        MaxDist = XMVectorMax(MaxDist, Dist2);

        XMVECTOR PlaneDist = XMVectorSplatW(Planes[i]);

        // Outside the plane?
        Outside = XMVectorOrInt(Outside, XMVectorGreater(MinDist, PlaneDist));

        // Fully inside the plane?
        InsideAll = XMVectorAndInt(InsideAll, XMVectorLessOrEqual(MaxDist, PlaneDist));
    }

    // If the triangle is outside any of the planes it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // If the triangle is inside all planes it is fully inside.
    if (XMVector4EqualInt(InsideAll, XMVectorTrueInt()))
        return true;

    // Build the corners of the frustum.
    XMVECTOR vRightTop = XMVectorSet(RightSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vRightBottom = XMVectorSet(RightSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vLeftTop = XMVectorSet(LeftSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR vLeftBottom = XMVectorSet(LeftSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vNear = XMVectorReplicatePtr(&Near);
    XMVECTOR vFar = XMVectorReplicatePtr(&Far);

    XMVECTOR Corners[CORNER_COUNT];
    Corners[0] = XMVectorMultiply(vRightTop, vNear);
    Corners[1] = XMVectorMultiply(vRightBottom, vNear);
    Corners[2] = XMVectorMultiply(vLeftTop, vNear);
    Corners[3] = XMVectorMultiply(vLeftBottom, vNear);
    Corners[4] = XMVectorMultiply(vRightTop, vFar);
    Corners[5] = XMVectorMultiply(vRightBottom, vFar);
    Corners[6] = XMVectorMultiply(vLeftTop, vFar);
    Corners[7] = XMVectorMultiply(vLeftBottom, vFar);

    // Test the plane of the triangle.
    XMVECTOR Normal = XMVector3Cross(XMVectorSubtract(V1, V0), XMVectorSubtract(V2, V0));
    XMVECTOR Dist = XMVector3Dot(Normal, V0);

    XMVECTOR MinDist, MaxDist;
    MinDist = MaxDist = XMVector3Dot(Corners[0], Normal);
    for (size_t i = 1; i < CORNER_COUNT; ++i)
    {
        XMVECTOR Temp = XMVector3Dot(Corners[i], Normal);
        MinDist = XMVectorMin(MinDist, Temp);
        MaxDist = XMVectorMax(MaxDist, Temp);
    }

    Outside = XMVectorOrInt(XMVectorGreater(MinDist, Dist), XMVectorLess(MaxDist, Dist));
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // Check the edge/edge axes (3*6).
    XMVECTOR TriangleEdgeAxis[3];
    TriangleEdgeAxis[0] = XMVectorSubtract(V1, V0);
    TriangleEdgeAxis[1] = XMVectorSubtract(V2, V1);
    TriangleEdgeAxis[2] = XMVectorSubtract(V0, V2);

    XMVECTOR FrustumEdgeAxis[6];
    FrustumEdgeAxis[0] = vRightTop;
    FrustumEdgeAxis[1] = vRightBottom;
    FrustumEdgeAxis[2] = vLeftTop;
    FrustumEdgeAxis[3] = vLeftBottom;
    FrustumEdgeAxis[4] = XMVectorSubtract(vRightTop, vLeftTop);
    FrustumEdgeAxis[5] = XMVectorSubtract(vLeftBottom, vLeftTop);

    for (size_t i = 0; i < 3; ++i)
    {
        for (size_t j = 0; j < 6; j++)
        {
            // Compute the axis we are going to test.
            XMVECTOR Axis = XMVector3Cross(TriangleEdgeAxis[i], FrustumEdgeAxis[j]);

            // Find the min/max of the projection of the triangle onto the axis.
            XMVECTOR MinA, MaxA;

            XMVECTOR Dist0 = XMVector3Dot(V0, Axis);
            XMVECTOR Dist1 = XMVector3Dot(V1, Axis);
            XMVECTOR Dist2 = XMVector3Dot(V2, Axis);

            MinA = XMVectorMin(Dist0, Dist1);
            MinA = XMVectorMin(MinA, Dist2);
            MaxA = XMVectorMax(Dist0, Dist1);
            MaxA = XMVectorMax(MaxA, Dist2);

            // Find the min/max of the projection of the frustum onto the axis.
            XMVECTOR MinB, MaxB;

            MinB = MaxB = XMVector3Dot(Axis, Corners[0]);

            for (size_t k = 1; k < CORNER_COUNT; k++)
            {
                XMVECTOR Temp = XMVector3Dot(Axis, Corners[k]);
                MinB = XMVectorMin(MinB, Temp);
                MaxB = XMVectorMax(MaxB, Temp);
            }

            // if (MinA > MaxB || MinB > MaxA) reject;
            Outside = XMVectorOrInt(Outside, XMVectorGreater(MinA, MaxB));
            Outside = XMVectorOrInt(Outside, XMVectorGreater(MinB, MaxA));
        }
    }

    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return false;

    // If we did not find a separating plane then the triangle must intersect the frustum.
    return true;
}


//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline PlaneIntersectionType XM_CALLCONV BoundingFrustum::Intersects(FXMVECTOR Plane) const noexcept
{
    assert(DirectX::Internal::XMPlaneIsUnit(Plane));

    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Set w of the origin to one so we can dot4 with a plane.
    vOrigin = XMVectorInsert<0, 0, 0, 0, 1>(vOrigin, XMVectorSplatOne());

    // Build the corners of the frustum (in world space).
    XMVECTOR RightTop = XMVectorSet(RightSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR RightBottom = XMVectorSet(RightSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR LeftTop = XMVectorSet(LeftSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR LeftBottom = XMVectorSet(LeftSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vNear = XMVectorReplicatePtr(&Near);
    XMVECTOR vFar = XMVectorReplicatePtr(&Far);

    RightTop = XMVector3Rotate(RightTop, vOrientation);
    RightBottom = XMVector3Rotate(RightBottom, vOrientation);
    LeftTop = XMVector3Rotate(LeftTop, vOrientation);
    LeftBottom = XMVector3Rotate(LeftBottom, vOrientation);

    XMVECTOR Corners0 = XMVectorMultiplyAdd(RightTop, vNear, vOrigin);
    XMVECTOR Corners1 = XMVectorMultiplyAdd(RightBottom, vNear, vOrigin);
    XMVECTOR Corners2 = XMVectorMultiplyAdd(LeftTop, vNear, vOrigin);
    XMVECTOR Corners3 = XMVectorMultiplyAdd(LeftBottom, vNear, vOrigin);
    XMVECTOR Corners4 = XMVectorMultiplyAdd(RightTop, vFar, vOrigin);
    XMVECTOR Corners5 = XMVectorMultiplyAdd(RightBottom, vFar, vOrigin);
    XMVECTOR Corners6 = XMVectorMultiplyAdd(LeftTop, vFar, vOrigin);
    XMVECTOR Corners7 = XMVectorMultiplyAdd(LeftBottom, vFar, vOrigin);

    XMVECTOR Outside, Inside;
    DirectX::Internal::FastIntersectFrustumPlane(Corners0, Corners1, Corners2, Corners3,
        Corners4, Corners5, Corners6, Corners7,
        Plane, Outside, Inside);

    // If the frustum is outside any plane it is outside.
    if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
        return FRONT;

    // If the frustum is inside all planes it is inside.
    if (XMVector4EqualInt(Inside, XMVectorTrueInt()))
        return BACK;

    // The frustum is not inside all planes or outside a plane it intersects.
    return INTERSECTING;
}


//-----------------------------------------------------------------------------
// Ray vs. frustum test
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline bool XM_CALLCONV BoundingFrustum::Intersects(FXMVECTOR rayOrigin, FXMVECTOR Direction, float& Dist) const noexcept
{
    // If ray starts inside the frustum, return a distance of 0 for the hit
    if (Contains(rayOrigin) == CONTAINS)
    {
        Dist = 0.0f;
        return true;
    }

    // Build the frustum planes.
    XMVECTOR Planes[6];
    Planes[0] = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
    Planes[1] = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
    Planes[2] = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
    Planes[3] = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
    Planes[4] = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
    Planes[5] = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);

    // Load origin and orientation of the frustum.
    XMVECTOR frOrigin = XMLoadFloat3(&Origin);
    XMVECTOR frOrientation = XMLoadFloat4(&Orientation);

    // This algorithm based on "Fast Ray-Convex Polyhedron Intersectin," in James Arvo, ed., Graphics Gems II pp. 247-250
    float tnear = -FLT_MAX;
    float tfar = FLT_MAX;

    for (size_t i = 0; i < 6; ++i)
    {
        XMVECTOR Plane = DirectX::Internal::XMPlaneTransform(Planes[i], frOrientation, frOrigin);
        Plane = XMPlaneNormalize(Plane);

        XMVECTOR AxisDotOrigin = XMPlaneDotCoord(Plane, rayOrigin);
        XMVECTOR AxisDotDirection = XMVector3Dot(Plane, Direction);

        if (XMVector3LessOrEqual(XMVectorAbs(AxisDotDirection), g_RayEpsilon))
        {
            // Ray is parallel to plane - check if ray origin is inside plane's
            if (XMVector3Greater(AxisDotOrigin, g_XMZero))
            {
                // Ray origin is outside half-space.
                Dist = 0.f;
                return false;
            }
        }
        else
        {
            // Ray not parallel - get distance to plane.
            float vd = XMVectorGetX(AxisDotDirection);
            float vn = XMVectorGetX(AxisDotOrigin);
            float t = -vn / vd;
            if (vd < 0.0f)
            {
                // Front face - T is a near point.
                if (t > tfar)
                {
                    Dist = 0.f;
                    return false;
                }
                if (t > tnear)
                {
                    // Hit near face.
                    tnear = t;
                }
            }
            else
            {
                // back face - T is far point.
                if (t < tnear)
                {
                    Dist = 0.f;
                    return false;
                }
                if (t < tfar)
                {
                    // Hit far face.
                    tfar = t;
                }
            }
        }
    }

    // Survived all tests.
    // Note: if ray originates on polyhedron, may want to change 0.0f to some
    // epsilon to avoid intersecting the originating face.
    float distance = (tnear >= 0.0f) ? tnear : tfar;
    if (distance >= 0.0f)
    {
        Dist = distance;
        return true;
    }

    Dist = 0.f;
    return false;
}


//-----------------------------------------------------------------------------
// Test a frustum vs 6 planes (typically forming another frustum).
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline ContainmentType XM_CALLCONV BoundingFrustum::ContainedBy(
    FXMVECTOR Plane0, FXMVECTOR Plane1, FXMVECTOR Plane2,
    GXMVECTOR Plane3,
    HXMVECTOR Plane4, HXMVECTOR Plane5) const noexcept
{
    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    assert(DirectX::Internal::XMQuaternionIsUnit(vOrientation));

    // Set w of the origin to one so we can dot4 with a plane.
    vOrigin = XMVectorInsert<0, 0, 0, 0, 1>(vOrigin, XMVectorSplatOne());

    // Build the corners of the frustum (in world space).
    XMVECTOR RightTop = XMVectorSet(RightSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR RightBottom = XMVectorSet(RightSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR LeftTop = XMVectorSet(LeftSlope, TopSlope, 1.0f, 0.0f);
    XMVECTOR LeftBottom = XMVectorSet(LeftSlope, BottomSlope, 1.0f, 0.0f);
    XMVECTOR vNear = XMVectorReplicatePtr(&Near);
    XMVECTOR vFar = XMVectorReplicatePtr(&Far);

    RightTop = XMVector3Rotate(RightTop, vOrientation);
    RightBottom = XMVector3Rotate(RightBottom, vOrientation);
    LeftTop = XMVector3Rotate(LeftTop, vOrientation);
    LeftBottom = XMVector3Rotate(LeftBottom, vOrientation);

    XMVECTOR Corners0 = XMVectorMultiplyAdd(RightTop, vNear, vOrigin);
    XMVECTOR Corners1 = XMVectorMultiplyAdd(RightBottom, vNear, vOrigin);
    XMVECTOR Corners2 = XMVectorMultiplyAdd(LeftTop, vNear, vOrigin);
    XMVECTOR Corners3 = XMVectorMultiplyAdd(LeftBottom, vNear, vOrigin);
    XMVECTOR Corners4 = XMVectorMultiplyAdd(RightTop, vFar, vOrigin);
    XMVECTOR Corners5 = XMVectorMultiplyAdd(RightBottom, vFar, vOrigin);
    XMVECTOR Corners6 = XMVectorMultiplyAdd(LeftTop, vFar, vOrigin);
    XMVECTOR Corners7 = XMVectorMultiplyAdd(LeftBottom, vFar, vOrigin);

    XMVECTOR Outside, Inside;

    // Test against each plane.
    DirectX::Internal::FastIntersectFrustumPlane(Corners0, Corners1, Corners2, Corners3,
        Corners4, Corners5, Corners6, Corners7,
        Plane0, Outside, Inside);

    XMVECTOR AnyOutside = Outside;
    XMVECTOR AllInside = Inside;

    DirectX::Internal::FastIntersectFrustumPlane(Corners0, Corners1, Corners2, Corners3,
        Corners4, Corners5, Corners6, Corners7,
        Plane1, Outside, Inside);

    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectFrustumPlane(Corners0, Corners1, Corners2, Corners3,
        Corners4, Corners5, Corners6, Corners7,
        Plane2, Outside, Inside);

    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectFrustumPlane(Corners0, Corners1, Corners2, Corners3,
        Corners4, Corners5, Corners6, Corners7,
        Plane3, Outside, Inside);

    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectFrustumPlane(Corners0, Corners1, Corners2, Corners3,
        Corners4, Corners5, Corners6, Corners7,
        Plane4, Outside, Inside);

    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    DirectX::Internal::FastIntersectFrustumPlane(Corners0, Corners1, Corners2, Corners3,
        Corners4, Corners5, Corners6, Corners7,
        Plane5, Outside, Inside);

    AnyOutside = XMVectorOrInt(AnyOutside, Outside);
    AllInside = XMVectorAndInt(AllInside, Inside);

    // If the frustum is outside any plane it is outside.
    if (XMVector4EqualInt(AnyOutside, XMVectorTrueInt()))
        return DISJOINT;

    // If the frustum is inside all planes it is inside.
    if (XMVector4EqualInt(AllInside, XMVectorTrueInt()))
        return CONTAINS;

    // The frustum is not inside all planes or outside a plane, it may intersect.
    return INTERSECTS;
}


//-----------------------------------------------------------------------------
// Build the 6 frustum planes from a frustum.
//
// The intended use for these routines is for fast culling to a view frustum.
// When the volume being tested against a view frustum is small relative to the
// view frustum it is usually either inside all six planes of the frustum
// (CONTAINS) or outside one of the planes of the frustum (DISJOINT). If neither
// of these cases is true then it may or may not be intersecting the frustum
// (INTERSECTS)
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void BoundingFrustum::GetPlanes(XMVECTOR* NearPlane, XMVECTOR* FarPlane, XMVECTOR* RightPlane,
    XMVECTOR* LeftPlane, XMVECTOR* TopPlane, XMVECTOR* BottomPlane) const noexcept
{
    // Load origin and orientation of the frustum.
    XMVECTOR vOrigin = XMLoadFloat3(&Origin);
    XMVECTOR vOrientation = XMLoadFloat4(&Orientation);

    if (NearPlane)
    {
        XMVECTOR vNearPlane = XMVectorSet(0.0f, 0.0f, -1.0f, Near);
        vNearPlane = DirectX::Internal::XMPlaneTransform(vNearPlane, vOrientation, vOrigin);
        *NearPlane = XMPlaneNormalize(vNearPlane);
    }

    if (FarPlane)
    {
        XMVECTOR vFarPlane = XMVectorSet(0.0f, 0.0f, 1.0f, -Far);
        vFarPlane = DirectX::Internal::XMPlaneTransform(vFarPlane, vOrientation, vOrigin);
        *FarPlane = XMPlaneNormalize(vFarPlane);
    }

    if (RightPlane)
    {
        XMVECTOR vRightPlane = XMVectorSet(1.0f, 0.0f, -RightSlope, 0.0f);
        vRightPlane = DirectX::Internal::XMPlaneTransform(vRightPlane, vOrientation, vOrigin);
        *RightPlane = XMPlaneNormalize(vRightPlane);
    }

    if (LeftPlane)
    {
        XMVECTOR vLeftPlane = XMVectorSet(-1.0f, 0.0f, LeftSlope, 0.0f);
        vLeftPlane = DirectX::Internal::XMPlaneTransform(vLeftPlane, vOrientation, vOrigin);
        *LeftPlane = XMPlaneNormalize(vLeftPlane);
    }

    if (TopPlane)
    {
        XMVECTOR vTopPlane = XMVectorSet(0.0f, 1.0f, -TopSlope, 0.0f);
        vTopPlane = DirectX::Internal::XMPlaneTransform(vTopPlane, vOrientation, vOrigin);
        *TopPlane = XMPlaneNormalize(vTopPlane);
    }

    if (BottomPlane)
    {
        XMVECTOR vBottomPlane = XMVectorSet(0.0f, -1.0f, BottomSlope, 0.0f);
        vBottomPlane = DirectX::Internal::XMPlaneTransform(vBottomPlane, vOrientation, vOrigin);
        *BottomPlane = XMPlaneNormalize(vBottomPlane);
    }
}


//-----------------------------------------------------------------------------
// Build a frustum from a persepective projection matrix.  The matrix may only
// contain a projection; any rotation, translation or scale will cause the
// constructed frustum to be incorrect.
//-----------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV BoundingFrustum::CreateFromMatrix(BoundingFrustum& Out, FXMMATRIX Projection, bool rhcoords) noexcept
{
    // Corners of the projection frustum in NDC space.
    static XMVECTORF32 NDCPoints[6] =
    {
        { { {  1.0f,  0.0f, 1.0f, 1.0f } } },   // right (at far plane)
        { { { -1.0f,  0.0f, 1.0f, 1.0f } } },   // left
        { { {  0.0f,  1.0f, 1.0f, 1.0f } } },   // top
        { { {  0.0f, -1.0f, 1.0f, 1.0f } } },   // bottom

        { { { 0.0f, 0.0f, 0.0f, 1.0f } } },     // near
        { { { 0.0f, 0.0f, 1.0f, 1.0f } } }      // far
    };

    XMVECTOR Determinant;
    XMMATRIX matInverse = XMMatrixInverse(&Determinant, Projection);

    // Compute the frustum corners in world space.
    XMVECTOR Points[6];

    for (size_t i = 0; i < 6; ++i)
    {
        // Transform point.
        Points[i] = XMVector4Transform(NDCPoints[i], matInverse);
    }

    Out.Origin = XMFLOAT3(0.0f, 0.0f, 0.0f);
    Out.Orientation = XMFLOAT4(0.0f, 0.0f, 0.0f, 1.0f);

    // Compute the slopes.
    Points[0] = XMVectorMultiply(Points[0], XMVectorReciprocal(XMVectorSplatZ(Points[0])));
    Points[1] = XMVectorMultiply(Points[1], XMVectorReciprocal(XMVectorSplatZ(Points[1])));
    Points[2] = XMVectorMultiply(Points[2], XMVectorReciprocal(XMVectorSplatZ(Points[2])));
    Points[3] = XMVectorMultiply(Points[3], XMVectorReciprocal(XMVectorSplatZ(Points[3])));

    Out.RightSlope = XMVectorGetX(Points[0]);
    Out.LeftSlope = XMVectorGetX(Points[1]);
    Out.TopSlope = XMVectorGetY(Points[2]);
    Out.BottomSlope = XMVectorGetY(Points[3]);

    // Compute near and far.
    Points[4] = XMVectorMultiply(Points[4], XMVectorReciprocal(XMVectorSplatW(Points[4])));
    Points[5] = XMVectorMultiply(Points[5], XMVectorReciprocal(XMVectorSplatW(Points[5])));

    if (rhcoords)
    {
        Out.Near = XMVectorGetZ(Points[5]);
        Out.Far = XMVectorGetZ(Points[4]);
    }
    else
    {
        Out.Near = XMVectorGetZ(Points[4]);
        Out.Far = XMVectorGetZ(Points[5]);
    }
}


/****************************************************************************
 *
 * TriangleTests
 *
 ****************************************************************************/

namespace TriangleTests
{

    //-----------------------------------------------------------------------------
    // Compute the intersection of a ray (Origin, Direction) with a triangle
    // (V0, V1, V2).  Return true if there is an intersection and also set *pDist
    // to the distance along the ray to the intersection.
    //
    // The algorithm is based on Moller, Tomas and Trumbore, "Fast, Minimum Storage
    // Ray-Triangle Intersection", Journal of Graphics Tools, vol. 2, no. 1,
    // pp 21-28, 1997.
    //-----------------------------------------------------------------------------
    _Use_decl_annotations_
        inline bool XM_CALLCONV Intersects(
            FXMVECTOR Origin, FXMVECTOR Direction, FXMVECTOR V0,
            GXMVECTOR V1,
            HXMVECTOR V2, float& Dist) noexcept
    {
        assert(DirectX::Internal::XMVector3IsUnit(Direction));

        XMVECTOR Zero = XMVectorZero();

        XMVECTOR e1 = XMVectorSubtract(V1, V0);
        XMVECTOR e2 = XMVectorSubtract(V2, V0);

        // p = Direction ^ e2;
        XMVECTOR p = XMVector3Cross(Direction, e2);

        // det = e1 * p;
        XMVECTOR det = XMVector3Dot(e1, p);

        XMVECTOR u, v, t;

        if (XMVector3GreaterOrEqual(det, g_RayEpsilon))
        {
            // Determinate is positive (front side of the triangle).
            XMVECTOR s = XMVectorSubtract(Origin, V0);

            // u = s * p;
            u = XMVector3Dot(s, p);

            XMVECTOR NoIntersection = XMVectorLess(u, Zero);
            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(u, det));

            // q = s ^ e1;
            XMVECTOR q = XMVector3Cross(s, e1);

            // v = Direction * q;
            v = XMVector3Dot(Direction, q);

            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(v, Zero));
            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(XMVectorAdd(u, v), det));

            // t = e2 * q;
            t = XMVector3Dot(e2, q);

            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(t, Zero));

            if (XMVector4EqualInt(NoIntersection, XMVectorTrueInt()))
            {
                Dist = 0.f;
                return false;
            }
        }
        else if (XMVector3LessOrEqual(det, g_RayNegEpsilon))
        {
            // Determinate is negative (back side of the triangle).
            XMVECTOR s = XMVectorSubtract(Origin, V0);

            // u = s * p;
            u = XMVector3Dot(s, p);

            XMVECTOR NoIntersection = XMVectorGreater(u, Zero);
            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(u, det));

            // q = s ^ e1;
            XMVECTOR q = XMVector3Cross(s, e1);

            // v = Direction * q;
            v = XMVector3Dot(Direction, q);

            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(v, Zero));
            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorLess(XMVectorAdd(u, v), det));

            // t = e2 * q;
            t = XMVector3Dot(e2, q);

            NoIntersection = XMVectorOrInt(NoIntersection, XMVectorGreater(t, Zero));

            if (XMVector4EqualInt(NoIntersection, XMVectorTrueInt()))
            {
                Dist = 0.f;
                return false;
            }
        }
        else
        {
            // Parallel ray.
            Dist = 0.f;
            return false;
        }

        t = XMVectorDivide(t, det);

        // (u / det) and (v / dev) are the barycentric cooridinates of the intersection.

        // Store the x-component to *pDist
        XMStoreFloat(&Dist, t);

        return true;
    }


    //-----------------------------------------------------------------------------
    // Test if two triangles intersect.
    //
    // The final test of algorithm is based on Shen, Heng, and Tang, "A Fast
    // Triangle-Triangle Overlap Test Using Signed Distances", Journal of Graphics
    // Tools, vol. 8, no. 1, pp 17-23, 2003 and Guigue and Devillers, "Fast and
    // Robust Triangle-Triangle Overlap Test Using Orientation Predicates", Journal
    // of Graphics Tools, vol. 8, no. 1, pp 25-32, 2003.
    //
    // The final test could be considered an edge-edge separating plane test with
    // the 9 possible cases narrowed down to the only two pairs of edges that can
    // actaully result in a seperation.
    //-----------------------------------------------------------------------------
    _Use_decl_annotations_
        inline bool XM_CALLCONV Intersects(FXMVECTOR A0, FXMVECTOR A1, FXMVECTOR A2, GXMVECTOR B0, HXMVECTOR B1, HXMVECTOR B2) noexcept
    {
        static const XMVECTORU32 SelectY = { { { XM_SELECT_0, XM_SELECT_1, XM_SELECT_0, XM_SELECT_0 } } };
        static const XMVECTORU32 SelectZ = { { { XM_SELECT_0, XM_SELECT_0, XM_SELECT_1, XM_SELECT_0 } } };
        static const XMVECTORU32 Select0111 = { { { XM_SELECT_0, XM_SELECT_1, XM_SELECT_1, XM_SELECT_1 } } };
        static const XMVECTORU32 Select1011 = { { { XM_SELECT_1, XM_SELECT_0, XM_SELECT_1, XM_SELECT_1 } } };
        static const XMVECTORU32 Select1101 = { { { XM_SELECT_1, XM_SELECT_1, XM_SELECT_0, XM_SELECT_1 } } };

        XMVECTOR Zero = XMVectorZero();

        // Compute the normal of triangle A.
        XMVECTOR N1 = XMVector3Cross(XMVectorSubtract(A1, A0), XMVectorSubtract(A2, A0));

        // Assert that the triangle is not degenerate.
        assert(!XMVector3Equal(N1, Zero));

        // Test points of B against the plane of A.
        XMVECTOR BDist = XMVector3Dot(N1, XMVectorSubtract(B0, A0));
        BDist = XMVectorSelect(BDist, XMVector3Dot(N1, XMVectorSubtract(B1, A0)), SelectY);
        BDist = XMVectorSelect(BDist, XMVector3Dot(N1, XMVectorSubtract(B2, A0)), SelectZ);

        // Ensure robustness with co-planar triangles by zeroing small distances.
        uint32_t BDistIsZeroCR;
        XMVECTOR BDistIsZero = XMVectorGreaterR(&BDistIsZeroCR, g_RayEpsilon, XMVectorAbs(BDist));
        BDist = XMVectorSelect(BDist, Zero, BDistIsZero);

        uint32_t BDistIsLessCR;
        XMVECTOR BDistIsLess = XMVectorGreaterR(&BDistIsLessCR, Zero, BDist);

        uint32_t BDistIsGreaterCR;
        XMVECTOR BDistIsGreater = XMVectorGreaterR(&BDistIsGreaterCR, BDist, Zero);

        // If all the points are on the same side we don't intersect.
        if (XMComparisonAllTrue(BDistIsLessCR) || XMComparisonAllTrue(BDistIsGreaterCR))
            return false;

        // Compute the normal of triangle B.
        XMVECTOR N2 = XMVector3Cross(XMVectorSubtract(B1, B0), XMVectorSubtract(B2, B0));

        // Assert that the triangle is not degenerate.
        assert(!XMVector3Equal(N2, Zero));

        // Test points of A against the plane of B.
        XMVECTOR ADist = XMVector3Dot(N2, XMVectorSubtract(A0, B0));
        ADist = XMVectorSelect(ADist, XMVector3Dot(N2, XMVectorSubtract(A1, B0)), SelectY);
        ADist = XMVectorSelect(ADist, XMVector3Dot(N2, XMVectorSubtract(A2, B0)), SelectZ);

        // Ensure robustness with co-planar triangles by zeroing small distances.
        uint32_t ADistIsZeroCR;
        XMVECTOR ADistIsZero = XMVectorGreaterR(&ADistIsZeroCR, g_RayEpsilon, XMVectorAbs(BDist));
        ADist = XMVectorSelect(ADist, Zero, ADistIsZero);

        uint32_t ADistIsLessCR;
        XMVECTOR ADistIsLess = XMVectorGreaterR(&ADistIsLessCR, Zero, ADist);

        uint32_t ADistIsGreaterCR;
        XMVECTOR ADistIsGreater = XMVectorGreaterR(&ADistIsGreaterCR, ADist, Zero);

        // If all the points are on the same side we don't intersect.
        if (XMComparisonAllTrue(ADistIsLessCR) || XMComparisonAllTrue(ADistIsGreaterCR))
            return false;

        // Special case for co-planar triangles.
        if (XMComparisonAllTrue(ADistIsZeroCR) || XMComparisonAllTrue(BDistIsZeroCR))
        {
            XMVECTOR Axis, Dist, MinDist;

            // Compute an axis perpindicular to the edge (points out).
            Axis = XMVector3Cross(N1, XMVectorSubtract(A1, A0));
            Dist = XMVector3Dot(Axis, A0);

            // Test points of B against the axis.
            MinDist = XMVector3Dot(B0, Axis);
            MinDist = XMVectorMin(MinDist, XMVector3Dot(B1, Axis));
            MinDist = XMVectorMin(MinDist, XMVector3Dot(B2, Axis));
            if (XMVector4GreaterOrEqual(MinDist, Dist))
                return false;

            // Edge (A1, A2)
            Axis = XMVector3Cross(N1, XMVectorSubtract(A2, A1));
            Dist = XMVector3Dot(Axis, A1);

            MinDist = XMVector3Dot(B0, Axis);
            MinDist = XMVectorMin(MinDist, XMVector3Dot(B1, Axis));
            MinDist = XMVectorMin(MinDist, XMVector3Dot(B2, Axis));
            if (XMVector4GreaterOrEqual(MinDist, Dist))
                return false;

            // Edge (A2, A0)
            Axis = XMVector3Cross(N1, XMVectorSubtract(A0, A2));
            Dist = XMVector3Dot(Axis, A2);

            MinDist = XMVector3Dot(B0, Axis);
            MinDist = XMVectorMin(MinDist, XMVector3Dot(B1, Axis));
            MinDist = XMVectorMin(MinDist, XMVector3Dot(B2, Axis));
            if (XMVector4GreaterOrEqual(MinDist, Dist))
                return false;

            // Edge (B0, B1)
            Axis = XMVector3Cross(N2, XMVectorSubtract(B1, B0));
            Dist = XMVector3Dot(Axis, B0);

            MinDist = XMVector3Dot(A0, Axis);
            MinDist = XMVectorMin(MinDist, XMVector3Dot(A1, Axis));
            MinDist = XMVectorMin(MinDist, XMVector3Dot(A2, Axis));
            if (XMVector4GreaterOrEqual(MinDist, Dist))
                return false;

            // Edge (B1, B2)
            Axis = XMVector3Cross(N2, XMVectorSubtract(B2, B1));
            Dist = XMVector3Dot(Axis, B1);

            MinDist = XMVector3Dot(A0, Axis);
            MinDist = XMVectorMin(MinDist, XMVector3Dot(A1, Axis));
            MinDist = XMVectorMin(MinDist, XMVector3Dot(A2, Axis));
            if (XMVector4GreaterOrEqual(MinDist, Dist))
                return false;

            // Edge (B2,B0)
            Axis = XMVector3Cross(N2, XMVectorSubtract(B0, B2));
            Dist = XMVector3Dot(Axis, B2);

            MinDist = XMVector3Dot(A0, Axis);
            MinDist = XMVectorMin(MinDist, XMVector3Dot(A1, Axis));
            MinDist = XMVectorMin(MinDist, XMVector3Dot(A2, Axis));
            if (XMVector4GreaterOrEqual(MinDist, Dist))
                return false;

            return true;
        }

        //
        // Find the single vertex of A and B (ie the vertex on the opposite side
        // of the plane from the other two) and reorder the edges so we can compute
        // the signed edge/edge distances.
        //
        // if ( (V0 >= 0 && V1 <  0 && V2 <  0) ||
        //      (V0 >  0 && V1 <= 0 && V2 <= 0) ||
        //      (V0 <= 0 && V1 >  0 && V2 >  0) ||
        //      (V0 <  0 && V1 >= 0 && V2 >= 0) ) then V0 is singular;
        //
        // If our singular vertex is not on the positive side of the plane we reverse
        // the triangle winding so that the overlap comparisons will compare the
        // correct edges with the correct signs.
        //
        XMVECTOR ADistIsLessEqual = XMVectorOrInt(ADistIsLess, ADistIsZero);
        XMVECTOR ADistIsGreaterEqual = XMVectorOrInt(ADistIsGreater, ADistIsZero);

        XMVECTOR AA0, AA1, AA2;
        bool bPositiveA;

        if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsGreaterEqual, ADistIsLess, Select0111)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsGreater, ADistIsLessEqual, Select0111)))
        {
            // A0 is singular, crossing from positive to negative.
            AA0 = A0; AA1 = A1; AA2 = A2;
            bPositiveA = true;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsLessEqual, ADistIsGreater, Select0111)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsLess, ADistIsGreaterEqual, Select0111)))
        {
            // A0 is singular, crossing from negative to positive.
            AA0 = A0; AA1 = A2; AA2 = A1;
            bPositiveA = false;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsGreaterEqual, ADistIsLess, Select1011)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsGreater, ADistIsLessEqual, Select1011)))
        {
            // A1 is singular, crossing from positive to negative.
            AA0 = A1; AA1 = A2; AA2 = A0;
            bPositiveA = true;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsLessEqual, ADistIsGreater, Select1011)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsLess, ADistIsGreaterEqual, Select1011)))
        {
            // A1 is singular, crossing from negative to positive.
            AA0 = A1; AA1 = A0; AA2 = A2;
            bPositiveA = false;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsGreaterEqual, ADistIsLess, Select1101)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsGreater, ADistIsLessEqual, Select1101)))
        {
            // A2 is singular, crossing from positive to negative.
            AA0 = A2; AA1 = A0; AA2 = A1;
            bPositiveA = true;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsLessEqual, ADistIsGreater, Select1101)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(ADistIsLess, ADistIsGreaterEqual, Select1101)))
        {
            // A2 is singular, crossing from negative to positive.
            AA0 = A2; AA1 = A1; AA2 = A0;
            bPositiveA = false;
        }
        else
        {
            assert(false);
            return false;
        }

        XMVECTOR BDistIsLessEqual = XMVectorOrInt(BDistIsLess, BDistIsZero);
        XMVECTOR BDistIsGreaterEqual = XMVectorOrInt(BDistIsGreater, BDistIsZero);

        XMVECTOR BB0, BB1, BB2;
        bool bPositiveB;

        if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsGreaterEqual, BDistIsLess, Select0111)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsGreater, BDistIsLessEqual, Select0111)))
        {
            // B0 is singular, crossing from positive to negative.
            BB0 = B0; BB1 = B1; BB2 = B2;
            bPositiveB = true;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsLessEqual, BDistIsGreater, Select0111)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsLess, BDistIsGreaterEqual, Select0111)))
        {
            // B0 is singular, crossing from negative to positive.
            BB0 = B0; BB1 = B2; BB2 = B1;
            bPositiveB = false;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsGreaterEqual, BDistIsLess, Select1011)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsGreater, BDistIsLessEqual, Select1011)))
        {
            // B1 is singular, crossing from positive to negative.
            BB0 = B1; BB1 = B2; BB2 = B0;
            bPositiveB = true;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsLessEqual, BDistIsGreater, Select1011)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsLess, BDistIsGreaterEqual, Select1011)))
        {
            // B1 is singular, crossing from negative to positive.
            BB0 = B1; BB1 = B0; BB2 = B2;
            bPositiveB = false;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsGreaterEqual, BDistIsLess, Select1101)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsGreater, BDistIsLessEqual, Select1101)))
        {
            // B2 is singular, crossing from positive to negative.
            BB0 = B2; BB1 = B0; BB2 = B1;
            bPositiveB = true;
        }
        else if (DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsLessEqual, BDistIsGreater, Select1101)) ||
            DirectX::Internal::XMVector3AllTrue(XMVectorSelect(BDistIsLess, BDistIsGreaterEqual, Select1101)))
        {
            // B2 is singular, crossing from negative to positive.
            BB0 = B2; BB1 = B1; BB2 = B0;
            bPositiveB = false;
        }
        else
        {
            assert(false);
            return false;
        }

        XMVECTOR Delta0, Delta1;

        // Reverse the direction of the test depending on whether the singular vertices are
        // the same sign or different signs.
        if (bPositiveA ^ bPositiveB)
        {
            Delta0 = XMVectorSubtract(BB0, AA0);
            Delta1 = XMVectorSubtract(AA0, BB0);
        }
        else
        {
            Delta0 = XMVectorSubtract(AA0, BB0);
            Delta1 = XMVectorSubtract(BB0, AA0);
        }

        // Check if the triangles overlap on the line of intersection between the
        // planes of the two triangles by finding the signed line distances.
        XMVECTOR Dist0 = XMVector3Dot(Delta0, XMVector3Cross(XMVectorSubtract(BB2, BB0), XMVectorSubtract(AA2, AA0)));
        if (XMVector4Greater(Dist0, Zero))
            return false;

        XMVECTOR Dist1 = XMVector3Dot(Delta1, XMVector3Cross(XMVectorSubtract(BB1, BB0), XMVectorSubtract(AA1, AA0)));
        if (XMVector4Greater(Dist1, Zero))
            return false;

        return true;
    }


    //-----------------------------------------------------------------------------
    // Ray-triangle test
    //-----------------------------------------------------------------------------
    _Use_decl_annotations_
        inline PlaneIntersectionType XM_CALLCONV Intersects(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2, GXMVECTOR Plane) noexcept
    {
        XMVECTOR One = XMVectorSplatOne();

        assert(DirectX::Internal::XMPlaneIsUnit(Plane));

        // Set w of the points to one so we can dot4 with a plane.
        XMVECTOR TV0 = XMVectorInsert<0, 0, 0, 0, 1>(V0, One);
        XMVECTOR TV1 = XMVectorInsert<0, 0, 0, 0, 1>(V1, One);
        XMVECTOR TV2 = XMVectorInsert<0, 0, 0, 0, 1>(V2, One);

        XMVECTOR Outside, Inside;
        DirectX::Internal::FastIntersectTrianglePlane(TV0, TV1, TV2, Plane, Outside, Inside);

        // If the triangle is outside any plane it is outside.
        if (XMVector4EqualInt(Outside, XMVectorTrueInt()))
            return FRONT;

        // If the triangle is inside all planes it is inside.
        if (XMVector4EqualInt(Inside, XMVectorTrueInt()))
            return BACK;

        // The triangle is not inside all planes or outside a plane it intersects.
        return INTERSECTING;
    }


    //-----------------------------------------------------------------------------
    // Test a triangle vs 6 planes (typically forming a frustum).
    //-----------------------------------------------------------------------------
    _Use_decl_annotations_
        inline ContainmentType XM_CALLCONV ContainedBy(
            FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR V2,
            GXMVECTOR Plane0,
            HXMVECTOR Plane1, HXMVECTOR Plane2,
            CXMVECTOR Plane3, CXMVECTOR Plane4, CXMVECTOR Plane5) noexcept
    {
        XMVECTOR One = XMVectorSplatOne();

        // Set w of the points to one so we can dot4 with a plane.
        XMVECTOR TV0 = XMVectorInsert<0, 0, 0, 0, 1>(V0, One);
        XMVECTOR TV1 = XMVectorInsert<0, 0, 0, 0, 1>(V1, One);
        XMVECTOR TV2 = XMVectorInsert<0, 0, 0, 0, 1>(V2, One);

        XMVECTOR Outside, Inside;

        // Test against each plane.
        DirectX::Internal::FastIntersectTrianglePlane(TV0, TV1, TV2, Plane0, Outside, Inside);

        XMVECTOR AnyOutside = Outside;
        XMVECTOR AllInside = Inside;

        DirectX::Internal::FastIntersectTrianglePlane(TV0, TV1, TV2, Plane1, Outside, Inside);
        AnyOutside = XMVectorOrInt(AnyOutside, Outside);
        AllInside = XMVectorAndInt(AllInside, Inside);

        DirectX::Internal::FastIntersectTrianglePlane(TV0, TV1, TV2, Plane2, Outside, Inside);
        AnyOutside = XMVectorOrInt(AnyOutside, Outside);
        AllInside = XMVectorAndInt(AllInside, Inside);

        DirectX::Internal::FastIntersectTrianglePlane(TV0, TV1, TV2, Plane3, Outside, Inside);
        AnyOutside = XMVectorOrInt(AnyOutside, Outside);
        AllInside = XMVectorAndInt(AllInside, Inside);

        DirectX::Internal::FastIntersectTrianglePlane(TV0, TV1, TV2, Plane4, Outside, Inside);
        AnyOutside = XMVectorOrInt(AnyOutside, Outside);
        AllInside = XMVectorAndInt(AllInside, Inside);

        DirectX::Internal::FastIntersectTrianglePlane(TV0, TV1, TV2, Plane5, Outside, Inside);
        AnyOutside = XMVectorOrInt(AnyOutside, Outside);
        AllInside = XMVectorAndInt(AllInside, Inside);

        // If the triangle is outside any plane it is outside.
        if (XMVector4EqualInt(AnyOutside, XMVectorTrueInt()))
            return DISJOINT;

        // If the triangle is inside all planes it is inside.
        if (XMVector4EqualInt(AllInside, XMVectorTrueInt()))
            return CONTAINS;

        // The triangle is not inside all planes or outside a plane, it may intersect.
        return INTERSECTS;
    }

} // namespace TriangleTests

