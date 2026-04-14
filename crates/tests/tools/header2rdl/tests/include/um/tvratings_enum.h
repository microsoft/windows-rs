//
// copyright (c) Microsoft Corp.
//

#ifndef TVRATINGS_ENUMS_H
#define TVRATINGS_ENUMS_H

// !!!! do not #pragma once, we use this file twice(once for native and once for mgd) in managed interop
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "exposeenums2managed.h"

ENUM16  EnTvRat_System
{
    MPAA                    = 0,
    US_TV                   = 1,
    Canadian_English        = 2,
    Canadian_French         = 3,
    Reserved4               = 4,    // filler - not used.
    System5                 = 5,    // maps to system 5 of XDS rating table 19
    System6                 = 6,    // maps to system 6 of XDS rating table 19
    Reserved7               = 7,    // filler - not used.
    PBDA                    = 8,    // PBDA rating system
    AgeBased                = 9,    // age based rating system
    TvRat_kSystems          = 10,   // used for allocating structures           
    TvRat_SystemDontKnow    = 255   // haven't gotten a data value yet... (perhaps change to 0)
} EnTvRat_System;


ENUM16 EnTvRat_GenericLevel     // constraint is that must be in increasing order, and max < 2^8
{
    TvRat_0                 = 0,
    TvRat_1                 = 1,
    TvRat_2                 = 2,
    TvRat_3                 = 3,
    TvRat_4                 = 4,
    TvRat_5                 = 5,
    TvRat_6                 = 6,
    TvRat_7                 = 7,
    TvRat_8                 = 8,
    TvRat_9                 = 9,
    TvRat_10                = 10,
    TvRat_11                = 11,
    TvRat_12                = 12,
    TvRat_13                = 13,
    TvRat_14                = 14,
    TvRat_15                = 15,
    TvRat_16                = 16,
    TvRat_17                = 17,
    TvRat_18                = 18,
    TvRat_19                = 19,
    TvRat_20                = 20,
    TvRat_21                = 21,
    TvRat_kLevels           = 22,
    TvRat_Unblock           = -1,
    TvRat_LevelDontKnow     = 255       // haven't gotten a data value yet... (perhaps change to _7)
} EnTvRat_GenericLevel;


ENUM16 EnTvRat_MPAA
{
    MPAA_NotApplicable      = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_0)),
    MPAA_G                  = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_1)),
    MPAA_PG                 = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_2)),
    MPAA_PG13               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_3)),
    MPAA_R                  = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_4)),
    MPAA_NC17               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_5)),
    MPAA_X                  = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_6)),
    MPAA_NotRated           = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_7))                  
} EnTvRat_MPAA;

ENUM16 EnTvRat_US_TV
{
    US_TV_None              = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_0)),
    US_TV_Y                 = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_1)),
    US_TV_Y7                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_2)),
    US_TV_G                 = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_3)),
    US_TV_PG                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_4)),
    US_TV_14                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_5)),
    US_TV_MA                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_6)),
    US_TV_None7             = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_7))                  
} EnTvRat_US_TV;

ENUM16 EnTvRat_CAE_TV
{
    CAE_TV_Exempt           = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_0)),
    CAE_TV_C                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_1)),
    CAE_TV_C8               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_2)),
    CAE_TV_G                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_3)),
    CAE_TV_PG               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_4)),
    CAE_TV_14               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_5)),
    CAE_TV_18               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_6)),
    CAE_TV_Reserved         = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_7))                  
} EnTvRat_CAE_TV;

ENUM16 EnTvRat_CAF_TV
{
    CAF_TV_Exempt           = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_0)),
    CAF_TV_G                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_1)),
    CAF_TV_8                = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_2)),
    CAF_TV_13               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_3)),
    CAF_TV_16               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_4)),
    CAF_TV_18               = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_5)),
    CAF_TV_Reserved6        = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_6)),
    CAF_TV_Reserved         = EHRECVR_MGD_NAMESPACE(RATLEVEL(TvRat_7))                  
} EnTvRat_CAF_TV;

    
        // -------------------
FLAGS16 BfEnTvRat_GenericAttributes
{
    BfAttrNone                      = 0,    // no bits set (for initialization)         
    BfIsBlocked                     = 1,    // if set, 
    BfIsAttr_1                      = 2,
    BfIsAttr_2                      = 4,    
    BfIsAttr_3                      = 8,
    BfIsAttr_4                      = 16,       
    BfIsAttr_5                      = 32,   // no bits set...
    BfIsAttr_6                      = 64,
    BfIsAttr_7                      = 128,  
    BfValidAttrSubmask              = 255       // IsBlocked is not a valid attribute to display    
} BfEnTvRat_GenericAttributes;

FLAGS16 BfEnTvRat_Attributes_US_TV
{
    US_TV_IsBlocked                     = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsBlocked)),
    US_TV_IsViolent                     = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsAttr_1)),
    US_TV_IsSexualSituation             = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsAttr_2)),
    US_TV_IsAdultLanguage               = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsAttr_3)),
    US_TV_IsSexuallySuggestiveDialog    = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsAttr_4)),
    US_TV_ValidAttrSubmask              = 31    // IsBlocked is not a valid attribute for TV
} BfEnTvRat_Attributes_US_TV;

FLAGS16 BfEnTvRat_Attributes_MPAA
{
    MPAA_IsBlocked                      = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsBlocked)),
    MPAA_ValidAttrSubmask               = 1     // IsBlocked is not a valid attribute 
} BfEnTvRat_Attributes_MPAA;

FLAGS16 BfEnTvRat_Attributes_CAE_TV
{
    CAE_IsBlocked                       = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsBlocked)),
    CAE_ValidAttrSubmask                = 1     // IsBlocked is not a valid attribute 
} BfEnTvRat_Attributes_CAE_TV;

FLAGS16 BfEnTvRat_Attributes_CAF_TV
{
    CAF_IsBlocked                       = EHRECVR_MGD_NAMESPACE(RATATTR(BfIsBlocked)),
    CAF_ValidAttrSubmask                = 1     // IsBlocked is not a valid attribute 
} BfEnTvRat_Attributes_CAF_TV;

#include "unexposeenums2managed.h"


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // ENCDEC_ENUMS_H

// end of file

