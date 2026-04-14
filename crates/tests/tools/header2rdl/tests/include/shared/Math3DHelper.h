/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    Math3DHelperV2.h

Abstract:

    Internal math library for vector, rotation matrix and quaternion calculation

--*/

#pragma once

#ifdef __cplusplus

#include <cmath>

// Pi
static const float PI               = (3.14159265f);
static const float DegToRadRatio    = (PI / 180.0f);
static const float RadToDegRatio    = (180.0f / PI);
static const float FLOAT_TOLERANCE  = (0.00001f);

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
static __inline float RealModulo (float x, float y)
{
    if (0 == y) return x;
    return x - y * (float)(floor(x/y)); 
} 

static __inline bool FloatEq(float x, float y)
{
    return (fabs(x - y) < FLOAT_TOLERANCE);
}
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */



class CVEC3D : public VEC3D
{
public:
    inline CVEC3D( void ) 
    {
        Clear();
    };
    inline ~CVEC3D( void ) {};
    inline CVEC3D( const VEC3D& _V ) 
    {
        X = _V.X;
        Y = _V.Y;
        Z = _V.Z;
    };
    inline CVEC3D( float _X, float _Y, float _Z)
    {
        X = _X;
        Y = _Y;
        Z = _Z;
    };

    // assignment operators
    inline CVEC3D& operator += ( const CVEC3D& _V)
    {
        *this = *this + _V;
        return *this;
    };

    inline CVEC3D& operator -= ( const CVEC3D& _V)
    {
        *this = *this - _V;
        return *this;
    };

    inline CVEC3D& operator *= (const float _F)
    {
        *this = *this * _F;
        return *this;
    };

    inline CVEC3D& operator /= (const float _F)
    {
        *this = *this / _F;
        return *this;
    };


    // unary
    inline CVEC3D operator - () const
    {
        CVEC3D v3;
        v3.X = -X;
        v3.Y = -Y;
        v3.Z = -Z;
        return v3;
    };

    // binary operators    
    inline CVEC3D operator + ( const CVEC3D& _V) const
    {
        CVEC3D v3;
        v3.X = X + _V.X;
        v3.Y = Y + _V.Y;
        v3.Z = Z + _V.Z;
        return v3;
    };
    
    inline CVEC3D operator - ( const CVEC3D& _V) const
    {
        CVEC3D v3;
        v3.X = X - _V.X;
        v3.Y = Y - _V.Y;
        v3.Z = Z - _V.Z;
        return v3;
    };
    
    inline CVEC3D operator * ( float _F) const
    {
        CVEC3D v3;
        v3.X = X * _F;
        v3.Y = Y * _F;
        v3.Z = Z * _F;
        return v3;
    };
    
    inline CVEC3D operator / ( float _F) const
    {
        CVEC3D v3;
        v3.X = X / _F;
        v3.Y = Y / _F;
        v3.Z = Z / _F;
        return v3;
    };

    inline bool operator == ( const CVEC3D& _V) const
    {
        return ((X == _V.X) && (Y == _V.Y) && (Z == _V.Z));
    };


    inline bool operator != ( const CVEC3D& _V) const
    {
        return !(this == &_V);
    };

    inline CVEC3D& operator = ( const CVEC3D& _V)
    {
        X = _V.X;
        Y = _V.Y;
        Z = _V.Z;
        return *this;
    };

    inline void Clear (void)
    {
        X = 0.0f;
        Y = 0.0f;
        Z = 0.0f;
    };

    inline CVEC3D CrossProduct ( const CVEC3D& _V) const
    {
        CVEC3D v3;
        v3.X = Y * _V.Z - Z * _V.Y;
        v3.Y = Z * _V.X - X * _V.Z;
        v3.Z = X * _V.Y - Y * _V.X;
        return v3;
    };

    inline float DotProduct ( const CVEC3D& _V) const
    {
        return X * _V.X + Y * _V.Y + Z * _V.Z;
    };

    inline float Magnitude (void)
    {
        float fMagnitude;
        fMagnitude = DotProduct (*this);
        fMagnitude = static_cast<float>(sqrt(fMagnitude));
        return fMagnitude;
    };

    inline void Normalize (void)
    {
        float fMagnitude = this->Magnitude();
        if (0.0 != fMagnitude)
        {
            *this /= fMagnitude;
        }
    };

    inline CVEC3D AngularDiff ( const CVEC3D& _V) const
    {
        CVEC3D Out;
        float fSine;

        Out = (*this).CrossProduct(_V);
        fSine = Out.Magnitude();
        if (fSine >= 0.001f) // if Sine is very small, Angle=Sin(Angle) - so skip this
        {
            float fCosine = (*this).DotProduct(_V);
            float fAngle = (float)(atan2(fSine, fCosine));  // Angle is in ]-PI..PI] range
            Out = Out * (fAngle / fSine);
        }
        
        return Out;
    };

    inline CVEC3D RotateAngleByAxis ( const float AngleInRad, const VEC3D ReferenceAxis)
    {
        //
        //  Rotate 3D vector pvToRotate by angle AngleRad (in radian), along axis pvAxis,  
        //  using right handed space (positive rotation is counter-clockwize)
        //  This is using Rodrigues' rotation formula which is more efficient than 
        //  converting the axis and angle into a rotation matrix, and using the rotation 
        //  matrix to compute the rotated vector.
        //
        // Rotate vector V by Angle about direction vector W:
        //      Vrot = V*Cos(Angle) + (WxV)*Sin(Angle) + w*(w.v)*(1-Cos(Angle))
        //

        CVEC3D Out, Axis;

        Axis = ReferenceAxis;
        Axis.Normalize();
        Out = (*this) * ((float)(cos(AngleInRad))) +
              Axis.CrossProduct(*this) * ((float)(sin(AngleInRad))) +
              Axis * (Axis.DotProduct(*this)) * (1 - ((float)(cos(AngleInRad))));

        return Out;
    };

    inline bool HasNAN (void)
    {
        return !(X == X && Y == Y && Z == Z);
    };

    inline void LowPassFilter ( const CVEC3D& _V, float Coefficient )
    {
        CVEC3D Temp;
        Temp = _V - *this;
        Temp = Temp * Coefficient;
        Temp = Temp + *this;
        X = Temp.X;
        Y = Temp.Y;
        Z = Temp.Z;
    };

};


class CMATRIX3X3 : public MATRIX3X3
{
public:
    inline CMATRIX3X3( void )
    {
        Clear();
    };        
    inline ~CMATRIX3X3( void ) {};
    inline CMATRIX3X3( const MATRIX3X3& _M ) 
    {
        A11 = _M.A11;
        A12 = _M.A12;
        A13 = _M.A13;
        A21 = _M.A21;
        A22 = _M.A22;
        A23 = _M.A23;
        A31 = _M.A31;
        A32 = _M.A32;
        A33 = _M.A33;
    };
    inline CMATRIX3X3( const VEC3D& _Vx, const VEC3D& _Vy, const VEC3D& _Vz ) 
    {
        V1 = _Vx;
        V2 = _Vy;
        V3 = _Vz;
    };
    inline CMATRIX3X3( float _A11, float _A12, float _A13,
                float _A21, float _A22, float _A23,
                float _A31, float _A32, float _A33)
    {
        A11 = _A11;
        A12 = _A12;
        A13 = _A13;
        A21 = _A21;
        A22 = _A22;
        A23 = _A23;
        A31 = _A31;
        A32 = _A32;
        A33 = _A33;
    };

    // assignment operators
    inline CMATRIX3X3 operator *= ( const CMATRIX3X3& _M ) 
    {
        *this = *this * _M;
        return *this;
    };
    
    inline CMATRIX3X3& operator += ( const CMATRIX3X3& _M)
    {
        *this = *this + _M;
        return *this;
    };

    inline CMATRIX3X3& operator -= ( const CMATRIX3X3& _M)
    {
        *this = *this - _M;
        return *this;
    };

    inline CMATRIX3X3& operator *= ( float _F)
    {
        *this = *this * _F;
        return *this;
    };

    inline CMATRIX3X3& operator /= ( float _F)
    {
        *this = *this / _F;
        return *this;
    };

    // unary
    inline CMATRIX3X3 operator - () const
    {
        CMATRIX3X3 m33Ret;
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                m33Ret.M [i] [j] = -M [i] [j];
            }
        }
        return m33Ret;
    };

    // binary operators
    inline CMATRIX3X3 operator * ( const CMATRIX3X3& _M) const
    {
        CMATRIX3X3 m33Ret;
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                m33Ret.M [i] [j] = (M [i] [0] * _M.M [0] [j]) + 
                                   (M [i] [1] * _M.M [1] [j]) + 
                                   (M [i] [2] * _M.M [2] [j]);
            }
        }
        return m33Ret;
    };
    
    inline CMATRIX3X3 operator + ( const CMATRIX3X3& _M) const
    {
        CMATRIX3X3 m33Ret;
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                m33Ret.M [i] [j] = M [i] [j] + _M.M [i] [j];
            }
        }
        return m33Ret;
    };
    
    inline CMATRIX3X3 operator - ( const CMATRIX3X3& _M) const
    {
        CMATRIX3X3 m33Ret;
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                m33Ret.M [i] [j] = M [i] [j] - _M.M [i] [j];
            }
        }
        return m33Ret;
    };
    
    inline CMATRIX3X3 operator * ( float _F) const
    {
        CMATRIX3X3 m33Ret;
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                m33Ret.M [i] [j] = M [i] [j] * _F;
            }
        }
        return m33Ret;
    };
    
    inline CMATRIX3X3 operator / ( float _F) const
    {
        CMATRIX3X3 m33Ret;
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                m33Ret.M [i] [j] = M [i] [j] / _F;
            }
        }
        return m33Ret;
    };

    inline bool operator == ( const CMATRIX3X3& _M) const
    {
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                if (M [i] [j] != _M.M [i] [j])
                {
                    return false;
                }
            }
        }
        return true;
    };


    inline bool operator != ( const CMATRIX3X3& _M) const
    {
        return !(this == &_M);
    };

    inline CMATRIX3X3& operator = ( const CMATRIX3X3& _M)
    {
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                M [i] [j] = _M.M [i] [j];
            }
        }
        return *this;
    };

    inline void InitialPosition (void)
    {
        Clear();
        A11 = 1.0f;
        A22 = 1.0f;
        A33 = 1.0f;
    }; 

    inline void Clear (void)
    {
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                M [i] [j] = 0.0f;
            }
        }
    };

    inline void SetToIdentity (void)
    {
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                M [i] [j] = (i == j) ? 1.0f : 0.0f;
            }
        }
    };
    
    inline CMATRIX3X3 Transpose (void) const
    {
        CMATRIX3X3 m33Ret;
        for (int i = 0; i < 3; i++)
        {
            for (int j = 0; j < 3; j++)
            {
                m33Ret.M [j] [i] = M [i] [j];
            }
        }
        return m33Ret;
    };

    inline CVEC3D operator * ( const CVEC3D &_V ) const
    {
        CVEC3D v3Ret (  A11 * _V.X + A12 * _V.Y + A13 * _V.Z,
                         A21 * _V.X + A22 * _V.Y + A23 * _V.Z,
                         A31 * _V.X + A32 * _V.Y + A33 * _V.Z);
        return v3Ret;
    };

    inline QUATERNION ToQuaternion (void)
    {
        QUATERNION quarternion = {0};

        // This method is safer and removes the imprecisions around PI rotation than using the classical:
        // W = Sqrt(1+M11+M22+M33); X = (M32-M23) /4W; Y = (M13-M31)/4W; Z = (M21-M12)/4W
        quarternion.W = (float) sqrt (max (0.0f, 1.0f + A11 + A22 + A33)) / 2.0f; 
        //Get norms:
        quarternion.X = (float) sqrt (max (0.0f, 1.0f + A11 - A22 - A33)) / 2.0f; 
        quarternion.Y = (float) sqrt (max (0.0f, 1.0f - A11 + A22 - A33)) / 2.0f; 
        quarternion.Z = (float) sqrt (max (0.0f, 1.0f - A11 - A22 + A33)) / 2.0f;
        // Get signs
        if ((A32 - A23) < 0.0f)
        { // negate QX:
            quarternion.X = - quarternion.X;
        }
        if ((A13 - A31) < 0.0f)
        { // negate QY:
            quarternion.Y = - quarternion.Y;
        }
        if ((A21 - A12) < 0.0f)
        { // negate QZ:
            quarternion.Z = - quarternion.Z;
        }

        return quarternion;
    };

    inline VEC3D ToEulerAngle (void)
    {
        VEC3D Euler = {0};
        CVEC3D EulerAngle;

        if (!FloatEq(A32, 1.0f) && !FloatEq(A32, -1.0f))
        {
            float cx = 1.0f;

            if (FloatEq (A33, 0.0f))
            {
                EulerAngle.Y = -PI/2; // then A31 = cos X
                EulerAngle.X = (float) (atan2 (A32, A31));
            }
            else
            {
                EulerAngle.Y = (float) (atan (-A31 / A33));
                if (A33 >= 0.0f)
                {
                    EulerAngle.X = (float) (asin (A32));
                }
                else
                {
                    EulerAngle.X = (float) (-asin (A32)) + PI;
                }                
            }

            if (cos (EulerAngle.X) < 0.0f) 
            {
                cx = -1.0f;
            }

            EulerAngle.Z = (float) (atan2 (-A12 * cx, A22 * cx));
        }
        else
        {
            EulerAngle.Y = 0.0f; // can be anything, make if 0

            if (FloatEq (A32, -1.0f))
            {
                EulerAngle.X = -PI/2;
            }
            else
            {
                EulerAngle.X = PI/2;
            }

            EulerAngle.Z = (float) (atan2 (A21, A11));
        }

        // Yaw is in [0, +2PI[
        EulerAngle.Z = RealModulo (EulerAngle.Z, 2*PI);

        // Pitch is in [-PI, +PI[
        EulerAngle.X = RealModulo (EulerAngle.X + PI, 2*PI) - PI;

        // Roll is in [-PI/2, +PI/2[
        EulerAngle.Y = RealModulo (EulerAngle.Y + PI/2, PI) - PI/2;

        // Convert from rad to degree
        Euler = EulerAngle * RadToDegRatio;

        return Euler;
    };

};


class CQUATERNION : public QUATERNION
{
public:
    inline CQUATERNION( void )
    {
        Clear();
    };        
    inline ~CQUATERNION( void ) {};
    inline CQUATERNION( const QUATERNION& _Q ) 
    {
        W = _Q.W;
        X = _Q.X;
        Y = _Q.Y;
        Z = _Q.Z;
    };
    inline CQUATERNION( float _W, float _X, float _Y, float _Z ) 
    {
        W = _W;
        X = _X;
        Y = _Y;
        Z = _Z;
    };
    inline CQUATERNION( float AngleInRad, VEC3D Axis ) 
    {
        W = (float)(cos(AngleInRad/2));
        X = Axis.X * (float)(sin(AngleInRad/2));
        Y = Axis.Y * (float)(sin(AngleInRad/2));
        Z = Axis.Z * (float)(sin(AngleInRad/2));
    };

    // assignment operators
    inline CQUATERNION operator *= ( const CQUATERNION& _Q ) 
    {
        *this = *this * _Q;
        return *this;
    };

    inline CQUATERNION& operator *= (const float _F)
    {
        *this = *this * _F;
        return *this;
    };

    inline CQUATERNION& operator /= (const float _F)
    {
        *this = *this / _F;
        return *this;
    };
    
    inline CQUATERNION& operator += ( const CQUATERNION& _Q)
    {
        *this = *this + _Q;
        return *this;
    };

    inline CQUATERNION& operator -= ( const CQUATERNION& _Q)
    {
        *this = *this - _Q;
        return *this;
    };

    // unary
    inline CQUATERNION operator - () const
    {
        CQUATERNION QuatRet;
        
        QuatRet.W = -W;
        QuatRet.X = -X;
        QuatRet.Y = -Y;
        QuatRet.Z = -Z;

        return QuatRet;
    };

    // binary operators
    inline CQUATERNION operator * ( const CQUATERNION& _Q) const
    {
        CQUATERNION QuatRet;
        
        QuatRet.W = W*_Q.W - X*_Q.X - Y*_Q.Y - Z*_Q.Z;
        QuatRet.X = X*_Q.W + W*_Q.X + Y*_Q.Z - Z*_Q.Y;
        QuatRet.Y = W*_Q.Y - X*_Q.Z + Y*_Q.W + Z*_Q.X;
        QuatRet.Z = W*_Q.Z + X*_Q.Y - Y*_Q.X + Z*_Q.W;

        return QuatRet;
    };

    inline CQUATERNION operator * ( float _F) const
    {
        CQUATERNION QuatRet;

        QuatRet.W = W * _F;
        QuatRet.X = X * _F;
        QuatRet.Y = Y * _F;
        QuatRet.Z = Z * _F;

        return QuatRet;
    };

    inline CQUATERNION operator / ( float _F) const
    {
        CQUATERNION QuatRet;

        QuatRet.W = W / _F;
        QuatRet.X = X / _F;
        QuatRet.Y = Y / _F;
        QuatRet.Z = Z / _F;

        return QuatRet;
    };
    
    inline CQUATERNION operator + ( const CQUATERNION& _Q) const
    {
        CQUATERNION QuatRet;
        
        QuatRet.W = W + _Q.W;
        QuatRet.X = X + _Q.X;
        QuatRet.Y = Y + _Q.Y;
        QuatRet.Z = Z + _Q.Z;

        return QuatRet;
    };
    
    inline CQUATERNION operator - ( const CQUATERNION& _Q) const
    {
        CQUATERNION QuatRet;
        
        QuatRet.W = W - _Q.W;
        QuatRet.X = X - _Q.X;
        QuatRet.Y = Y - _Q.Y;
        QuatRet.Z = Z - _Q.Z;

        return QuatRet;
    };
    
    inline bool operator == ( const CQUATERNION& _Q) const
    {
        return ((W == _Q.W) && (X == _Q.X) && (Y == _Q.Y) && (Z == _Q.Z));
    };


    inline bool operator != ( const CQUATERNION& _Q) const
    {
        return !(this == &_Q);
    };

    inline CQUATERNION& operator = ( const CQUATERNION& _Q)
    {
        W = _Q.W;
        X = _Q.X;
        Y = _Q.Y;
        Z = _Q.Z;
        return *this;
    };

    inline CQUATERNION Conjugate (void) const
    {
        CQUATERNION QuatRet;
        
        QuatRet.W = W;
        QuatRet.X = -X;
        QuatRet.Y = -Y;
        QuatRet.Z = -Z;

        return QuatRet;
    };

    inline float Magnitude (void)
    {
        return (static_cast<float>(sqrt(W*W+X*X+Y*Y+Z*Z)));
    };

    inline void Normalize (void)
    {
        float fMagnitude = this->Magnitude();
        if (0.0 != fMagnitude)
        {
            *this /= fMagnitude;
        }
    };

    inline void InitialPosition (void)
    {
        W = 1.0f;
        X = 0.0f;
        Y = 0.0f;
        Z = 0.0f;
    }; 

    inline void Clear (void)
    {
        W = 0.0f;
        X = 0.0f;
        Y = 0.0f;
        Z = 0.0f;
    };

    inline float ToAngleAxis ( PVEC3D pAxis)
    {
        float Angle = 2 * acos(W);

        if (pAxis != nullptr)
        {
            float Temp = sqrt(1 - W*W);
            if (Temp != 0)
            {
                pAxis->X = X / Temp;
                pAxis->Y = Y / Temp;
                pAxis->Z = Z / Temp;
            }
        }

        return Angle;
    };

    inline MATRIX3X3 ToMatrix (void)
    {
        MATRIX3X3 Matrix = {0};

        Matrix.A11 = 1 - 2*Y*Y - 2*Z*Z;
        Matrix.A12 = 2*X*Y - 2*Z*W;
        Matrix.A13 = 2*X*Z + 2*Y*W;
        Matrix.A21 = 2*X*Y + 2*Z*W;
        Matrix.A22 = 1 - 2*X*X - 2*Z*Z;
        Matrix.A23 = 2*Y*Z - 2*X*W;
        Matrix.A31 = 2*X*Z - 2*Y*W;
        Matrix.A32 = 2*Y*Z + 2*X*W;
        Matrix.A33 = 1 - 2*X*X - 2*Y*Y;

        return Matrix;
    };

    inline CQUATERNION RotateAngleByAxis ( const float AngleInRad, const VEC3D Axis)
    {
        CQUATERNION QuatRet = *this;

        if (!FloatEq(AngleInRad, 0))
        {
            CQUATERNION Rotation(AngleInRad, Axis);

            QuatRet = Rotation * *this;
        }

        return QuatRet;
    };

    inline CQUATERNION RotateAngleByGravity ( const float AngleInRad )
    {
        CQUATERNION QuatRet = *this;

        if (!FloatEq(AngleInRad, 0))
        {
            VEC3D Axis = {0};
            Axis.X = -1.0f * (2*X*Z - 2*Y*W);
            Axis.Y = -1.0f * (2*Y*Z + 2*X*W);
            Axis.Z = -1.0f * (1 - 2*X*X - 2*Y*Y);

            CQUATERNION Rotation(AngleInRad, Axis);

            QuatRet = Rotation * *this;
        }

        return QuatRet;
    };

};

#endif