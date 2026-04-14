/*-========================================================================-_
 |                               - X3DAUDIO -                               |
 |        Copyright (c) Microsoft Corporation.  All rights reserved.        |
 |~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|
 |PROJECT: X3DAudio                     MODEL:   Unmanaged User-mode        |
 |VERSION: 1.7                          EXCEPT:  No Exceptions              |
 |CLASS:   N / A                        MINREQ:  Win8, Xbox One             |
 |BASE:    N / A                        DIALECT: MSC++ 14.00                |
 |>------------------------------------------------------------------------<|
 | DUTY: Cross-platform stand-alone 3D audio math library                   |
 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
  NOTES:
    1.  Definition of terms:
            LFE: Low Frequency Effect -- always omnidirectional.
            LPF: Low Pass Filter, divided into two classifications:
                 Direct -- Applied to the direct signal path,
                           used for obstruction/occlusion effects.
                 Reverb -- Applied to the reverb signal path,
                           used for occlusion effects only.

    2.  Volume level is expressed as a linear amplitude scaler:
        1.0f represents no attenuation applied to the original signal,
        0.5f denotes an attenuation of 6dB, and 0.0f results in silence.
        Amplification (volume > 1.0f) is also allowed, and is not clamped.

        LPF values range from 1.0f representing all frequencies pass through,
        to 0.0f which results in silence as all frequencies are filtered out.

    3.  X3DAudio uses a left-handed Cartesian coordinate system with values
        on the x-axis increasing from left to right, on the y-axis from
        bottom to top, and on the z-axis from near to far.
        Azimuths are measured clockwise from a given reference direction.

        Distance measurement is with respect to user-defined world units.
        Applications may provide coordinates using any system of measure
        as all non-normalized calculations are scale invariant, with such
        operations natively occurring in user-defined world unit space.
        Metric constants are supplied only as a convenience.
        Distance is calculated using the Euclidean norm formula.

    4.  Only real values are permissible with functions using 32-bit
        float parameters -- NAN and infinite values are not accepted.
        All computation occurs in 32-bit precision mode.                    */

#pragma once

#include <sdkddkver.h>

#if(_WIN32_WINNT < _WIN32_WINNT_WIN8)
#error "This version of XAudio2 is available only in Windows 8 or later. Use the XAudio2 headers and libraries from the DirectX SDK with applications that target Windows 7 and earlier versions."
#endif // (_WIN32_WINNT < _WIN32_WINNT_WIN8)

#include <winapifamily.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_TV_APP | WINAPI_PARTITION_TV_TITLE | WINAPI_PARTITION_GAMES)

//--------------<D-E-F-I-N-I-T-I-O-N-S>-------------------------------------//
#include <windef.h>    // general windows types
#include <DirectXMath.h>

// speaker geometry configuration flags, specifies assignment of channels to speaker positions, defined as per WAVEFORMATEXTENSIBLE.dwChannelMask
#if !defined(_SPEAKER_POSITIONS_)
    #define _SPEAKER_POSITIONS_
    #define SPEAKER_FRONT_LEFT            0x00000001
    #define SPEAKER_FRONT_RIGHT           0x00000002
    #define SPEAKER_FRONT_CENTER          0x00000004
    #define SPEAKER_LOW_FREQUENCY         0x00000008
    #define SPEAKER_BACK_LEFT             0x00000010
    #define SPEAKER_BACK_RIGHT            0x00000020
    #define SPEAKER_FRONT_LEFT_OF_CENTER  0x00000040
    #define SPEAKER_FRONT_RIGHT_OF_CENTER 0x00000080
    #define SPEAKER_BACK_CENTER           0x00000100
    #define SPEAKER_SIDE_LEFT             0x00000200
    #define SPEAKER_SIDE_RIGHT            0x00000400
    #define SPEAKER_TOP_CENTER            0x00000800
    #define SPEAKER_TOP_FRONT_LEFT        0x00001000
    #define SPEAKER_TOP_FRONT_CENTER      0x00002000
    #define SPEAKER_TOP_FRONT_RIGHT       0x00004000
    #define SPEAKER_TOP_BACK_LEFT         0x00008000
    #define SPEAKER_TOP_BACK_CENTER       0x00010000
    #define SPEAKER_TOP_BACK_RIGHT        0x00020000
    #define SPEAKER_RESERVED              0x7FFC0000 // bit mask locations reserved for future use
    #define SPEAKER_ALL                   0x80000000 // used to specify that any possible permutation of speaker configurations
#endif

// standard speaker geometry configurations, used with X3DAudioInitialize
#if !defined(SPEAKER_MONO)
    #define SPEAKER_MONO             SPEAKER_FRONT_CENTER
    #define SPEAKER_STEREO           (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT)
    #define SPEAKER_2POINT1          (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_LOW_FREQUENCY)
    #define SPEAKER_SURROUND         (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_FRONT_CENTER | SPEAKER_BACK_CENTER)
    #define SPEAKER_QUAD             (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_BACK_LEFT | SPEAKER_BACK_RIGHT)
    #define SPEAKER_4POINT1          (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_LOW_FREQUENCY | SPEAKER_BACK_LEFT | SPEAKER_BACK_RIGHT)
    #define SPEAKER_5POINT1          (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_FRONT_CENTER | SPEAKER_LOW_FREQUENCY | SPEAKER_BACK_LEFT | SPEAKER_BACK_RIGHT)
    #define SPEAKER_7POINT1          (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_FRONT_CENTER | SPEAKER_LOW_FREQUENCY | SPEAKER_BACK_LEFT | SPEAKER_BACK_RIGHT | SPEAKER_FRONT_LEFT_OF_CENTER | SPEAKER_FRONT_RIGHT_OF_CENTER)
    #define SPEAKER_5POINT1_SURROUND (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_FRONT_CENTER | SPEAKER_LOW_FREQUENCY | SPEAKER_SIDE_LEFT  | SPEAKER_SIDE_RIGHT)
    #define SPEAKER_7POINT1_SURROUND (SPEAKER_FRONT_LEFT | SPEAKER_FRONT_RIGHT | SPEAKER_FRONT_CENTER | SPEAKER_LOW_FREQUENCY | SPEAKER_BACK_LEFT | SPEAKER_BACK_RIGHT | SPEAKER_SIDE_LEFT  | SPEAKER_SIDE_RIGHT)
#endif

// size of instance handle in bytes
#define X3DAUDIO_HANDLE_BYTESIZE 20

// float math constants
#define X3DAUDIO_PI  3.141592654f
#define X3DAUDIO_2PI 6.283185307f

// speed of sound in meters per second for dry air at approximately 20C, used with X3DAudioInitialize
#define X3DAUDIO_SPEED_OF_SOUND 343.5f

// calculation control flags, used with X3DAudioCalculate
#define X3DAUDIO_CALCULATE_MATRIX          0x00000001 // enable matrix coefficient table calculation
#define X3DAUDIO_CALCULATE_DELAY           0x00000002 // enable delay time array calculation (stereo final mix only)
#define X3DAUDIO_CALCULATE_LPF_DIRECT      0x00000004 // enable LPF direct-path coefficient calculation
#define X3DAUDIO_CALCULATE_LPF_REVERB      0x00000008 // enable LPF reverb-path coefficient calculation
#define X3DAUDIO_CALCULATE_REVERB          0x00000010 // enable reverb send level calculation
#define X3DAUDIO_CALCULATE_DOPPLER         0x00000020 // enable doppler shift factor calculation
#define X3DAUDIO_CALCULATE_EMITTER_ANGLE   0x00000040 // enable emitter-to-listener interior angle calculation

#define X3DAUDIO_CALCULATE_ZEROCENTER      0x00010000 // do not position to front center speaker, signal positioned to remaining speakers instead, front center destination channel will be zero in returned matrix coefficient table, valid only for matrix calculations with final mix formats that have a front center channel
#define X3DAUDIO_CALCULATE_REDIRECT_TO_LFE 0x00020000 // apply equal mix of all source channels to LFE destination channel, valid only for matrix calculations with sources that have no LFE channel and final mix formats that have an LFE channel


//--------------<D-A-T-A---T-Y-P-E-S>---------------------------------------//
#pragma pack(push, 1) // set packing alignment to ensure consistency across arbitrary build environments


// primitive types
typedef float FLOAT32; // 32-bit IEEE float
typedef DirectX::XMFLOAT3 X3DAUDIO_VECTOR; // float 3D vector

// instance handle of precalculated constants
typedef BYTE X3DAUDIO_HANDLE[X3DAUDIO_HANDLE_BYTESIZE];


// Distance curve point:
// Defines a DSP setting at a given normalized distance.
typedef struct X3DAUDIO_DISTANCE_CURVE_POINT
{
    FLOAT32 Distance;   // normalized distance, must be within [0.0f, 1.0f]
    FLOAT32 DSPSetting; // DSP setting
} X3DAUDIO_DISTANCE_CURVE_POINT, *LPX3DAUDIO_DISTANCE_CURVE_POINT;

// Distance curve:
// A piecewise curve made up of linear segments used to
// define DSP behaviour with respect to normalized distance.
//
// Note that curve point distances are normalized within [0.0f, 1.0f].
// X3DAUDIO_EMITTER.CurveDistanceScaler must be used to scale the
// normalized distances to user-defined world units.
// For distances beyond CurveDistanceScaler * 1.0f,
// pPoints[PointCount-1].DSPSetting is used as the DSP setting.
//
// All distance curve spans must be such that:
//      pPoints[k-1].DSPSetting + ((pPoints[k].DSPSetting-pPoints[k-1].DSPSetting) / (pPoints[k].Distance-pPoints[k-1].Distance)) * (pPoints[k].Distance-pPoints[k-1].Distance) != NAN or infinite values
// For all points in the distance curve where 1 <= k < PointCount.
typedef struct X3DAUDIO_DISTANCE_CURVE
{
    X3DAUDIO_DISTANCE_CURVE_POINT* pPoints;    // distance curve point array, must have at least PointCount elements with no duplicates and be sorted in ascending order with respect to Distance
    UINT32                         PointCount; // number of distance curve points, must be >= 2 as all distance curves must have at least two endpoints, defining DSP settings at 0.0f and 1.0f normalized distance
} X3DAUDIO_DISTANCE_CURVE, *LPX3DAUDIO_DISTANCE_CURVE;
static const X3DAUDIO_DISTANCE_CURVE_POINT X3DAudioDefault_LinearCurvePoints[2] = { { 0.0f, 1.0f }, { 1.0f, 0.0f } };
static const X3DAUDIO_DISTANCE_CURVE       X3DAudioDefault_LinearCurve          = { (X3DAUDIO_DISTANCE_CURVE_POINT*)&X3DAudioDefault_LinearCurvePoints[0], 2 };

// Cone:
// Specifies directionality for a listener or single-channel emitter by
// modifying DSP behaviour with respect to its front orientation.
// This is modeled using two sound cones: an inner cone and an outer cone.
// On/within the inner cone, DSP settings are scaled by the inner values.
// On/beyond the outer cone, DSP settings are scaled by the outer values.
// If on both the cones, DSP settings are scaled by the inner values only.
// Between the two cones, the scaler is linearly interpolated between the
// inner and outer values.  Set both cone angles to 0 or X3DAUDIO_2PI for
// omnidirectionality using only the outer or inner values respectively.
typedef struct X3DAUDIO_CONE
{
    FLOAT32 InnerAngle; // inner cone angle in radians, must be within [0.0f, X3DAUDIO_2PI]
    FLOAT32 OuterAngle; // outer cone angle in radians, must be within [InnerAngle, X3DAUDIO_2PI]

    FLOAT32 InnerVolume; // volume level scaler on/within inner cone, used only for matrix calculations, must be within [0.0f, 2.0f] when used
    FLOAT32 OuterVolume; // volume level scaler on/beyond outer cone, used only for matrix calculations, must be within [0.0f, 2.0f] when used
    FLOAT32 InnerLPF;    // LPF (both direct and reverb paths) coefficient subtrahend on/within inner cone, used only for LPF (both direct and reverb paths) calculations, must be within [0.0f, 1.0f] when used
    FLOAT32 OuterLPF;    // LPF (both direct and reverb paths) coefficient subtrahend on/beyond outer cone, used only for LPF (both direct and reverb paths) calculations, must be within [0.0f, 1.0f] when used
    FLOAT32 InnerReverb; // reverb send level scaler on/within inner cone, used only for reverb calculations, must be within [0.0f, 2.0f] when used
    FLOAT32 OuterReverb; // reverb send level scaler on/beyond outer cone, used only for reverb calculations, must be within [0.0f, 2.0f] when used
} X3DAUDIO_CONE, *LPX3DAUDIO_CONE;
static const X3DAUDIO_CONE X3DAudioDefault_DirectionalCone = { X3DAUDIO_PI/2, X3DAUDIO_PI, 1.0f, 0.708f, 0.0f, 0.25f, 0.708f, 1.0f };


// Listener:
// Defines a point of 3D audio reception.
//
// The cone is directed by the listener's front orientation.
typedef struct X3DAUDIO_LISTENER
{
    X3DAUDIO_VECTOR OrientFront; // orientation of front direction, used only for matrix and delay calculations or listeners with cones for matrix, LPF (both direct and reverb paths), and reverb calculations, must be normalized when used
    X3DAUDIO_VECTOR OrientTop;   // orientation of top direction, used only for matrix and delay calculations, must be orthonormal with OrientFront when used

    X3DAUDIO_VECTOR Position; // position in user-defined world units, does not affect Velocity
    X3DAUDIO_VECTOR Velocity; // velocity vector in user-defined world units/second, used only for doppler calculations, does not affect Position

    X3DAUDIO_CONE* pCone; // sound cone, used only for matrix, LPF (both direct and reverb paths), and reverb calculations, NULL specifies omnidirectionality
} X3DAUDIO_LISTENER, *LPX3DAUDIO_LISTENER;

// Emitter:
// Defines a 3D audio source, divided into two classifications:
//
// Single-point -- For use with single-channel sounds.
//                 Positioned at the emitter base, i.e. the channel radius
//                 and azimuth are ignored if the number of channels == 1.
//
//                 May be omnidirectional or directional using a cone.
//                 The cone originates from the emitter base position,
//                 and is directed by the emitter's front orientation.
//
// Multi-point  -- For use with multi-channel sounds.
//                 Each non-LFE channel is positioned using an
//                 azimuth along the channel radius with respect to the
//                 front orientation vector in the plane orthogonal to the
//                 top orientation vector.  An azimuth of X3DAUDIO_2PI
//                 specifies a channel is an LFE.  Such channels are
//                 positioned at the emitter base and are calculated
//                 with respect to pLFECurve only, never pVolumeCurve.
//
//                 Multi-point emitters are always omnidirectional,
//                 i.e. the cone is ignored if the number of channels > 1.
//
// Note that many properties are shared among all channel points,
// locking certain behaviour with respect to the emitter base position.
// For example, doppler shift is always calculated with respect to the
// emitter base position and so is constant for all its channel points.
// Distance curve calculations are also with respect to the emitter base
// position, with the curves being calculated independently of each other.
// For instance, volume and LFE calculations do not affect one another.
typedef struct X3DAUDIO_EMITTER
{
    X3DAUDIO_CONE* pCone; // sound cone, used only with single-channel emitters for matrix, LPF (both direct and reverb paths), and reverb calculations, NULL specifies omnidirectionality
    X3DAUDIO_VECTOR OrientFront; // orientation of front direction, used only for emitter angle calculations or with multi-channel emitters for matrix calculations or single-channel emitters with cones for matrix, LPF (both direct and reverb paths), and reverb calculations, must be normalized when used
    X3DAUDIO_VECTOR OrientTop;   // orientation of top direction, used only with multi-channel emitters for matrix calculations, must be orthonormal with OrientFront when used

    X3DAUDIO_VECTOR Position; // position in user-defined world units, does not affect Velocity
    X3DAUDIO_VECTOR Velocity; // velocity vector in user-defined world units/second, used only for doppler calculations, does not affect Position

    FLOAT32 InnerRadius;      // inner radius, must be within [0.0f, FLT_MAX]
    FLOAT32 InnerRadiusAngle; // inner radius angle, must be within [0.0f, X3DAUDIO_PI/4.0)

    UINT32 ChannelCount;       // number of sound channels, must be > 0
    FLOAT32 ChannelRadius;     // channel radius, used only with multi-channel emitters for matrix calculations, must be >= 0.0f when used
    FLOAT32* pChannelAzimuths; // channel azimuth array, used only with multi-channel emitters for matrix calculations, contains positions of each channel expressed in radians along the channel radius with respect to the front orientation vector in the plane orthogonal to the top orientation vector, or X3DAUDIO_2PI to specify an LFE channel, must have at least ChannelCount elements, all within [0.0f, X3DAUDIO_2PI] when used

    X3DAUDIO_DISTANCE_CURVE* pVolumeCurve;    // volume level distance curve, used only for matrix calculations, NULL specifies a default curve that conforms to the inverse square law, calculated in user-defined world units with distances <= CurveDistanceScaler clamped to no attenuation
    X3DAUDIO_DISTANCE_CURVE* pLFECurve;       // LFE level distance curve, used only for matrix calculations, NULL specifies a default curve that conforms to the inverse square law, calculated in user-defined world units with distances <= CurveDistanceScaler clamped to no attenuation
    X3DAUDIO_DISTANCE_CURVE* pLPFDirectCurve; // LPF direct-path coefficient distance curve, used only for LPF direct-path calculations, NULL specifies the default curve: [0.0f,1.0f], [1.0f,0.75f]
    X3DAUDIO_DISTANCE_CURVE* pLPFReverbCurve; // LPF reverb-path coefficient distance curve, used only for LPF reverb-path calculations, NULL specifies the default curve: [0.0f,0.75f], [1.0f,0.75f]
    X3DAUDIO_DISTANCE_CURVE* pReverbCurve;    // reverb send level distance curve, used only for reverb calculations, NULL specifies the default curve: [0.0f,1.0f], [1.0f,0.0f]

    FLOAT32 CurveDistanceScaler; // curve distance scaler, used to scale normalized distance curves to user-defined world units and/or exaggerate their effect, used only for matrix, LPF (both direct and reverb paths), and reverb calculations, must be within [FLT_MIN, FLT_MAX] when used
    FLOAT32 DopplerScaler;       // doppler shift scaler, used to exaggerate doppler shift effect, used only for doppler calculations, must be within [0.0f, FLT_MAX] when used
} X3DAUDIO_EMITTER, *LPX3DAUDIO_EMITTER;


// DSP settings:
// Receives results from a call to X3DAudioCalculate to be sent
// to the low-level audio rendering API for 3D signal processing.
//
// The user is responsible for allocating the matrix coefficient table,
// delay time array, and initializing the channel counts when used.
typedef struct X3DAUDIO_DSP_SETTINGS
{
    FLOAT32* pMatrixCoefficients; // [inout] matrix coefficient table, receives an array representing the volume level used to send from source channel S to destination channel D, stored as pMatrixCoefficients[SrcChannelCount * D + S], must have at least SrcChannelCount*DstChannelCount elements
    FLOAT32* pDelayTimes;         // [inout] delay time array, receives delays for each destination channel in milliseconds, must have at least DstChannelCount elements (stereo final mix only)
    UINT32 SrcChannelCount;       // [in] number of source channels, must equal number of channels in respective emitter
    UINT32 DstChannelCount;       // [in] number of destination channels, must equal number of channels of the final mix

    FLOAT32 LPFDirectCoefficient; // [out] LPF direct-path coefficient
    FLOAT32 LPFReverbCoefficient; // [out] LPF reverb-path coefficient
    FLOAT32 ReverbLevel; // [out] reverb send level
    FLOAT32 DopplerFactor; // [out] doppler shift factor, scales resampler ratio for doppler shift effect, where the effective frequency = DopplerFactor * original frequency
    FLOAT32 EmitterToListenerAngle; // [out] emitter-to-listener interior angle, expressed in radians with respect to the emitter's front orientation

    FLOAT32 EmitterToListenerDistance; // [out] distance in user-defined world units from the emitter base to listener position, always calculated
    FLOAT32 EmitterVelocityComponent; // [out] component of emitter velocity vector projected onto emitter->listener vector in user-defined world units/second, calculated only for doppler
    FLOAT32 ListenerVelocityComponent; // [out] component of listener velocity vector projected onto emitter->listener vector in user-defined world units/second, calculated only for doppler
} X3DAUDIO_DSP_SETTINGS, *LPX3DAUDIO_DSP_SETTINGS;


//--------------<M-A-C-R-O-S>-----------------------------------------------//
// function storage-class attribute and calltype
#if defined(X3DEXPORT)
#define X3DAUDIO_API_(type) EXTERN_C __declspec(dllexport) type STDAPIVCALLTYPE
#else
#define X3DAUDIO_API_(type) EXTERN_C __declspec(dllimport) type STDAPIVCALLTYPE
#endif

//--------------<F-U-N-C-T-I-O-N-S>-----------------------------------------//
// initializes instance handle
X3DAUDIO_API_(HRESULT) X3DAudioInitialize (UINT32 SpeakerChannelMask, FLOAT32 SpeedOfSound, _Out_writes_bytes_(X3DAUDIO_HANDLE_BYTESIZE) X3DAUDIO_HANDLE Instance);

// calculates DSP settings with respect to 3D parameters
X3DAUDIO_API_(void) X3DAudioCalculate (_In_reads_bytes_(X3DAUDIO_HANDLE_BYTESIZE) const X3DAUDIO_HANDLE Instance, _In_ const X3DAUDIO_LISTENER* pListener, _In_ const X3DAUDIO_EMITTER* pEmitter, UINT32 Flags, _Inout_ X3DAUDIO_DSP_SETTINGS* pDSPSettings);


#pragma pack(pop) // revert packing alignment

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_TV_APP | WINAPI_PARTITION_TV_TITLE | WINAPI_PARTITION_GAMES) */
#pragma endregion
//---------------------------------<-EOF->----------------------------------//

