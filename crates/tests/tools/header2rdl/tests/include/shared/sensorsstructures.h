/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    SensorsStructures.h

Abstract:

    Sensor structures header file

--*/

//#if _MSC_VER > 1000
#pragma once
//#endif

// The following flag is required to prevent structure redefinition with WpSensor.h
// we keep the #pragma above to benefit from the speed advantages this pragma brings.
#ifndef _SENSORS_STRUCTURES
#define _SENSORS_STRUCTURES

#ifdef __cplusplus
extern "C" {
#endif

typedef struct
{
    float X;
    float Y;
    float Z;
} VEC3D, *PVEC3D;

#pragma warning(push)
#pragma warning(disable:4201) // warning C4201: nonstandard extension used : nameless struct/union

typedef struct
{
    union
    {
        struct
        {
            float A11;
            float A12;
            float A13;
            float A21;
            float A22;
            float A23;
            float A31;
            float A32;
            float A33;
        };
        struct
        {
            VEC3D V1;
            VEC3D V2;
            VEC3D V3;
        };
        float M[3][3];
    };
} MATRIX3X3, *PMATRIX3X3;


#pragma warning(pop)

// Simple structure representing a 4 dimensional vector used for simple 3D rotation operation. 
// The rotation is done around the axis formed by the vector v= [X, Y, Z] and is of angle ?, and we have:
// W=cos(theta/2)
// |v|=sin(theta/2)
typedef struct
{
    float X;     // x component of rotation axis vector * sin(theta/2) also i (imaginary) coefficient
    float Y;     // y component of rotation axis vector * sin(theta/2) also j coefficient
    float Z;     // z component of rotation axis vector * sin(theta/2) also k coefficient
    float W;     // real coefficient = cos(theta/2)
} QUATERNION, *PQUATERNION;


typedef enum
{
    AXIS_X = 0,
    AXIS_Y,
    AXIS_Z,
    AXIS_MAX
} AXIS, *PAXIS;


#ifdef __cplusplus
}
#endif

#endif // _SENSORS_STRUCTURES