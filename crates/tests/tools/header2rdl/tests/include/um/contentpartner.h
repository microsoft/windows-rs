

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __contentpartner_h__
#define __contentpartner_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __IWMPContentContainer_FWD_DEFINED__
#define __IWMPContentContainer_FWD_DEFINED__
typedef interface IWMPContentContainer IWMPContentContainer;

#endif 	/* __IWMPContentContainer_FWD_DEFINED__ */


#ifndef __IWMPContentContainerList_FWD_DEFINED__
#define __IWMPContentContainerList_FWD_DEFINED__
typedef interface IWMPContentContainerList IWMPContentContainerList;

#endif 	/* __IWMPContentContainerList_FWD_DEFINED__ */


#ifndef __IWMPContentPartnerCallback_FWD_DEFINED__
#define __IWMPContentPartnerCallback_FWD_DEFINED__
typedef interface IWMPContentPartnerCallback IWMPContentPartnerCallback;

#endif 	/* __IWMPContentPartnerCallback_FWD_DEFINED__ */


#ifndef __IWMPContentPartner_FWD_DEFINED__
#define __IWMPContentPartner_FWD_DEFINED__
typedef interface IWMPContentPartner IWMPContentPartner;

#endif 	/* __IWMPContentPartner_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_contentpartner_0000_0000 */
/* [local] */ 

//=========================================================================
//
// Microsoft Windows Media Technologies
// Copyright (C) Microsoft Corporation. All rights reserved.
//
//=========================================================================
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are used in the IWMPContentPartner::GetContentPartnerInfo API for the bstrInfoName parameter.

static const WCHAR g_szContentPartnerInfo_LoginState[]             = L"LoginState";

// Synopsis: IWMPContentPartner::GetContentPartnerInfo
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BOOL, indicates whether the user is logged into the services

static const WCHAR g_szContentPartnerInfo_MediaPlayerAccountType[] = L"MediaPlayerAccountType";

// Synopsis: IWMPContentPartner::GetContentPartnerInfo
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_UI4,  - Represents a WMPAccountType value, this value is interpreted by the media player

static const WCHAR g_szContentPartnerInfo_AccountType[]            = L"AccountType";

// Synopsis: IWMPContentPartner::GetContentPartnerInfo
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR,  - Represents subscriptions services account type as a string
//                                                    -> This value is not interpreted by the media player, and may be shown to the user

static const WCHAR g_szContentPartnerInfo_HasCachedCredentials[]   = L"HasCachedCredentials";   // VT_BOOL - Returns whether plugin has cached credentials

// Synopsis: IWMPContentPartner::GetContentPartnerInfo
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BOOL, indicates whether the pluggin has cached
//                                                    -> credentials necessary to log in to the service

static const WCHAR g_szContentPartnerInfo_LicenseRefreshAdvanceWarning[] = L"LicenseRefreshAdvanceWarning";

// Synopsis: IWMPContentPartner::GetContentPartnerInfo
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_UI4, representing days  - This value is used by the player to pre-emptively refresh licenses that will expire soon.
//                                                    -> For example, if the plugin can support refreshing a playback license 5 days before it expires, then pData->ulVal should be set to 5.
//                                                    -> This value is a global setting, and cannot be different for different licenses, users, etc.

static const WCHAR g_szContentPartnerInfo_PurchasedTrackRequiresReDownload[] = L"PurchasedTrackRequiresReDownload";
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BOOL, represents whether a purchased track must be redownloaded if the content had been previously downloaded.

static const WCHAR g_szContentPartnerInfo_MaximumTrackPurchasePerPurchase[]  = L"MaximumNumberOfTracksPerPurchase";
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_UI4, indicates the maximum number of tracks that can be handled in a single IWMPContentParnter::Buy call.
//                                                       0 indicates no maximum.

static const WCHAR g_szContentPartnerInfo_AccountBalance[]  = L"AccountBalance";
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR,  - Represents account balance as a string
//                                                    -> This value is not interpreted by the media player, and may be shown to the user

static const WCHAR g_szContentPartnerInfo_UserName[]  = L"UserName";
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR,  - Represents user's account friendly name as a string
//                                                    -> This value is not interpreted by the media player, and may be shown to the user


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are returned to the HTML templates hosted in the player (returned from IWMPContentPartner::GetTemplate
// from the window.external.task API

static const WCHAR g_szMediaPlayerTask_Burn[]                    = L"Burn";
static const WCHAR g_szMediaPlayerTask_Browse[]                  = L"Browse";
static const WCHAR g_szMediaPlayerTask_Sync[]                    = L"Sync";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are used in the IWMPContentPartner::GetItemInfo API for the bstrInfoName parameter

static const WCHAR g_szItemInfo_PopupURL[]                       = L"Popup";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_I4, indicates index of popup URL to return
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to host in HTML based dialog

static const WCHAR g_szItemInfo_AuthenticationSuccessURL[]       = L"AuthenticationSuccessURL";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_I4, indicates index of Authentication URL to return
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to navigate if authentication is successful

static const WCHAR g_szItemInfo_LoginFailureURL[]                = L"LoginFailureURL";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, indicates index of LoginFailureURL to query.  This value was passed to the player via Notify( wmpcnLoginStateChange, <value
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to navigate when loginstatechange 

static const WCHAR g_szItemInfo_HTMLViewURL[]                = L"HTMLViewURL";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_BSTR, this is the string specified in the ASX file.  For example: <param name="HTMLFLINK" value="foo" \> 
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to use for the HTMLView 

static const WCHAR g_szItemInfo_PopupCaption[]                   = L"PopupCaption";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_I4, indicates index of popup URL to return
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates caption for the HTML based dialog

static const WCHAR g_szItemInfo_ALTLoginURL[]              = L"ALTLoginURL";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_EMPTY, contains nothing
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL for the alternative login

static const WCHAR g_szItemInfo_ALTLoginCaption[]              = L"ALTLoginCaption";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_EMPTY, contains nothing
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates caption for the alternative login

static const WCHAR g_szItemInfo_ForgetPasswordURL[]              = L"ForgotPassword";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_EMPTY, contains nothing
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to navigate the service plane
//                                                    -> in the event the user has forgotten their password

static const WCHAR g_szItemInfo_CreateAccountURL[]               = L"CreateAccount";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_EMPTY, contains nothing
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to navigate the service plane
//                                                    -> which the user can manage their account

static const WCHAR g_szItemInfo_ArtistArtURL[]                   = L"ArtistArt";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, containing the aritst ID from the catalog
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to retrieve artist art

static const WCHAR g_szItemInfo_AlbumArtURL[]                    = L"AlbumArt";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, containing the album ID from the catalog
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to retrieve album art

static const WCHAR g_szItemInfo_ListArtURL[]                     = L"ListArt";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, containing the list ID from the catalog
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to retrieve list art

static const WCHAR g_szItemInfo_GenreArtURL[]                    = L"GenreArt";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, containing the genre ID from the catalog
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to retrieve genre art

static const WCHAR g_szItemInfo_SubGenreArtURL[]                 = L"SubGenreArt";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, containing the sub-genre ID from the catalog
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to retrieve sub-genre art

static const WCHAR g_szItemInfo_RadioArtURL[]                 = L"RadioArt";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, containing the radio ID from the catalog
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to retrieve radio art

static const WCHAR g_szItemInfo_TreeListIconURL[]                 = L"CPListIDIcon";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_UI4, containing the list ID from the catalog
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, indicates URL to retrieve list icon art

static const WCHAR g_szItemInfo_ErrorDescription[]                = L"CPErrorDescription";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_ERROR, containing the error code
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, text describing the error.
//                                                       Fail this call if you do not understand error code

static const WCHAR g_szItemInfo_ErrorURL[]                        = L"CPErrorURL";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_ERROR, containing the error code
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, url that the service pane will be navigated
//                                                       to when then user clicks the resolve link

static const WCHAR g_szItemInfo_ErrorURLLinkText[]                    = L"CPErrorURLLinkText";

// Synopsis: IWMPContentPartner::GetItemInfo
//              /*input parameter*/ VARIANT *pContext -> Type is VT_ERROR, containing the error code
//              /*out parameter*/   VARIANT *pData    -> Type must be VT_BSTR, text that will be used for the hyperlink
//                                                       text for the URL returned from g_szItemInfo_ErrorURL


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are used in the all the IWMPContentPartner and IWMPContentPartnerCallback APIs
// that deal with some sort of location or object type.

static const WCHAR g_szUnknownLocation[]                         = L"UnknownLocation";

// Passed when the media player cannot determine a valid location to pass to the plugin.
// should rarely happen

static const WCHAR g_szRootLocation[]                            = L"RootLocation";           // Passed when the user is on the 'root' tree node of the service

// Passed when the action is taking place on the root content-partner node in the media player's lbrary tree

static const WCHAR g_szFlyoutMenu[]                            = L"FlyoutMenu";                 // Passed when the user selects the flyout menu of the service

// Passed when the user selects the flyout menu of the service

static const WCHAR g_szOnlineStore[]                           = L"OnlineStore";               // Used to navigate the player to the online stores page in the call to IWMPContentPartnerCallback::ChangeView

static const WCHAR g_szVideoRecent[]                           = L"VideoRecent";               // Used to navigate the player to the online stores page in the call to IWMPContentPartnerCallback::ChangeView

static const WCHAR g_szVideoRoot[]                           = L"VideoRoot";               // Used to navigate the player to the online stores page in the call to IWMPContentPartnerCallback::ChangeView

// These constants are used in the all the IWMPContentPartner and IWMPContentPartnerCallback APIs
// They indicate either a location in the library (see IWMPContentPartner::GetTemplate) or
// a type being returned via a callback (see IWMPContentPartner::GetListContents and
// IWMPContentPartnerCallback::AddListContents

static const WCHAR g_szCPListID[]                                   = L"CPListID"; 
static const WCHAR g_szAllCPListIDs[]                               = L"AllCPListIDs"; 
static const WCHAR g_szCPTrackID[]                                  = L"CPTrackID"; 
static const WCHAR g_szAllCPTrackIDs[]                              = L"AllCPTrackIDs"; 
static const WCHAR g_szCPArtistID[]                                 = L"CPArtistID"; 
static const WCHAR g_szAllCPArtistIDs[]                             = L"AllCPArtistIDs"; 
static const WCHAR g_szCPAlbumID[]                                  = L"CPAlbumID"; 
static const WCHAR g_szAllCPAlbumIDs[]                              = L"AllCPAlbumIDs"; 
static const WCHAR g_szCPGenreID[]                                  = L"CPGenreID"; 
static const WCHAR g_szAllCPGenreIDs[]                              = L"AllCPGenreIDs"; 
static const WCHAR g_szCPAlbumSubGenreID[]                          = L"CPAlbumSubGenreID"; 
static const WCHAR g_szAllCPAlbumSubGenreIDs[]                      = L"AllCPAlbumSubGenreIDs"; 
static const WCHAR g_szReleaseDateYear[]                            = L"ReleaseDateYear"; 
static const WCHAR g_szAllReleaseDateYears[]                        = L"AllReleaseDateYears"; 
static const WCHAR g_szCPRadioID[]                                  = L"CPRadioID"; 
static const WCHAR g_szAllCPRadioIDs[]                              = L"AllCPRadioIDs"; 
static const WCHAR g_szAuthor[]                                     = L"Author"; 
static const WCHAR g_szAllAuthors[]                                 = L"AllAuthors"; 
static const WCHAR g_szWMParentalRating[]                           = L"WMParentalRating"; 
static const WCHAR g_szAllWMParentalRatings[]                       = L"AllWMParentalRatings"; 
static const WCHAR g_szAllUserEffectiveRatingStarss[]               = L"AllUserEffectiveRatingStarss"; 
static const WCHAR g_szUserEffectiveRatingStars[]                   = L"UserEffectiveRatingStars"; 
static const WCHAR g_szUserPlaylist[]                               = L"UserPlaylist"; 
static ULONG g_knReservedCPTrackID_NotFound                         = (DWORD)-1; 

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are to be used by the window.external.changeViewOnlineList for the bstrViewMode parameter

static const WCHAR g_szViewMode_Report[]                          = L"ViewModeReport";         // Display dynamic list contents in report mode
static const WCHAR g_szViewMode_Details[]                         = L"ViewModeDetails";        // Display dynamic list contents in details mode
static const WCHAR g_szViewMode_Icon[]                            = L"ViewModeIcon";           // Display dynamic list contents in icon mode
static const WCHAR g_szViewMode_Tile[]                            = L"ViewModeTile";           // Display dynamic list contents in tile mode
static const WCHAR g_szViewMode_OrderedList[]                     = L"ViewModeOrderedList";    // Display dynamic list contents in ordered list mode

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are used in the IWMPContentContainer APIs for detecting fixed pricing

static const WCHAR g_szContentPrice_Unknown[]                     = L"PriceUnknown";           // The price of the content is unknown
static const WCHAR g_szContentPrice_CannotBuy[]                   = L"PriceCannotBuy";         // The content cannot be bought
static const WCHAR g_szContentPrice_Free[]                        = L"PriceFree";              // The content is free
//////////////////////////////////////////////////////////////////////
//          SERVICE CAPABILITIES THAT APPLY TO CONTENT-PARTNER-PLUGINS
//
// NOTE: If SUBSCRIPTION_CAP_IS_CONTENTPARTNER is not present, the 
// content partner plugin will NOT load
//
// These values are written in the registry as a DWORD value called 'Capabilities'
// under HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\MediaPlayer\Subscriptions\<ContentPartnerPluginKey>
// See the MSDN section on Online Stores Programming Reference in the Windows Media SDK
//////////////////////////////////////////////////////////////////////
#ifndef SUBSCRIPTION_CAP_DEVICEAVAILABLE
#define SUBSCRIPTION_CAP_DEVICEAVAILABLE        0x00000010
#endif
#ifndef SUBSCRIPTION_CAP_BACKGROUNDPROCESSING
#define SUBSCRIPTION_CAP_BACKGROUNDPROCESSING   0x00000008
#endif
#ifndef SUBSCRIPTION_CAP_IS_CONTENTPARTNER
#define SUBSCRIPTION_CAP_IS_CONTENTPARTNER      0x00000040
#endif
#ifndef SUBSCRIPTION_CAP_ALTLOGIN
#define SUBSCRIPTION_CAP_ALTLOGIN      0x00000080
#endif

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPPartnerNotification enum values are passed to the IWMPContentPartner::Notify API

// Synopsis - IWMPContentPartner::Notify( wmpsnBackgroundProcessingBegin 
//                                           /*input*/ VARIANT *pContenxt, type must be VT_EMPTY
// Remarks: the plugin can start background processing

// Synopsis - IWMPContentPartner::Notify( wmpsnBackgroundProcessingEnd 
//                                           /*input*/ VARIANT *pContenxt, type must be VT_EMPTY
// Remarks: the plugin should stop background processing, this enables the player to use more CPU for playback

// Synopsis - IWMPContentPartner::Notify( wmpsnCatalogDownloadFailure 
//                                           /*input*/ VARIANT *pContenxt, type must be VT_ERROR
// Remarks: indicates a failure while downloading the catalog

// Synopsis - IWMPContentPartner::Notify( wmpsnCatalogDownloadComplete 
//                                           /*input*/ VARIANT *pContenxt, type must be VT_EMPTY
// Remarks: indicates the catalog was downloaded successfully


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are used in the IWMPContentPartner::RefreshLicesnse APIs for the reason

static const WCHAR g_szRefreshLicensePlay[]                     = L"RefreshForPlay";
static const WCHAR g_szRefreshLicenseBurn[]                     = L"RefreshForBurn";
static const WCHAR g_szRefreshLicenseSync[]                     = L"RefreshForSync";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are used in the IWMPContentPartner::VerifyPermission APIs

static const WCHAR g_szVerifyPermissionSync[]                     = L"VerifyPermissionSync";
#ifndef __WMPNotifySubscriptionPluginAddRemove
#define __WMPNotifySubscriptionPluginAddRemove
__inline BOOL WMPNotifySubscriptionPluginAddRemove()
{
    BOOL fRet = FALSE;
    UINT  msg = RegisterWindowMessageA( "WMPlayer_PluginAddRemove" );
    if( 0 != msg ) 
     {
        fRet = PostMessage( HWND_BROADCAST, msg, 1, 0 );
     }
     return fRet;
}
#endif
typedef /* [public] */ 
enum WMPPartnerNotification
    {
        wmpsnBackgroundProcessingBegin	= 1,
        wmpsnBackgroundProcessingEnd	= 2,
        wmpsnCatalogDownloadFailure	= 3,
        wmpsnCatalogDownloadComplete	= 4
    } 	WMPPartnerNotification;


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPCallbackNotification enum values are passed to the IWMPContentPartner::Notify API

// Synopsis - IWMPContentPartnerCallback::Notify( wmpcnLoginStateChange 
//                                           /*input*/ VARIANT *pContext, type must be VT_BOOL or VT_UI4
// Remarks: indicates a new login state. VT_BOOL indicates a normal success or failure, while setting the type to VT_UI4 indicates
// a special condition (expired or canceled account, etc) in which case the player will query the plugin via
// GetItemInfo( g_szItemInfo_LoginFailureURL, <VT_UI4 value returned with wmpcnLoginStateChange>, <VARIANT * to receive the URL as VT_BSTR> )

// Synopsis - IWMPContentPartnerCallback::Notify( wmpcnAuthResult 
//                                           /*input*/ VARIANT *pContext, type must be VT_BOOL
// Remarks: indicates a successful auth

// Synopsis - IWMPContentPartnerCallback::Notify( wmpcnLicenseUpdated 
//                                           /*input*/ VARIANT *pContext, type must be VT_UI4
// Remarks: represents the content ID whose license has been update

// Synopsis - IWMPContentPartnerCallback::Notify( wmpcnNewCatalogAvailable 
//                                           /*input*/ VARIANT *pContext, type must be VT_EMPTY
// Remarks: Instructs the player to download a new catalog

// Synopsis - IWMPContentPartnerCallback::CallbackNotify( wmpcnNewPluginAvailable 
//                                           /*input*/ VARIANT *pContext, type must be VT_BOOL
// Remarks: Instructs the player to download a new plugin, if pContext is VARIANT_TRUE, this indicates
//          that an upgrade is required

// Synopsis - IWMPContentPartnerCallback::CallbackNotify( wmpcnDisableRadioSkipping 
//                                           /*input*/ VARIANT *pContext, type must be VT_EMPTY
// Remarks: Instructs the player to not allow any skipping of items in a radio playlist
typedef /* [public] */ 
enum WMPCallbackNotification
    {
        wmpcnLoginStateChange	= 1,
        wmpcnAuthResult	= 2,
        wmpcnLicenseUpdated	= 3,
        wmpcnNewCatalogAvailable	= 4,
        wmpcnNewPluginAvailable	= 5,
        wmpcnDisableRadioSkipping	= 6
    } 	WMPCallbackNotification;


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPTaskType enum values are passed to the IWMPContentPartner::GetTemplate API

// Synopsis - IWMPContentPartner::GetTemplate( wmpttBrowse 
// Remarks: Indicates that the player is in the Library task

// Synopsis - IWMPContentPartner::GetTemplate( wmpttSync 
// Remarks: Indicates that the player is in the Sync task

// Synopsis - IWMPContentPartner::GetTemplate( wmpttBurn 
// Remarks: Indicates that the player is in the Burn task

// Synopsis - IWMPContentPartner::GetTemplate( wmpttCurrent 
// Remarks: Indicates that the player is in something other than the Library, Sync or Burn task

typedef /* [public] */ 
enum WMPTaskType
    {
        wmpttBrowse	= 1,
        wmpttSync	= 2,
        wmpttBurn	= 3,
        wmpttCurrent	= 4
    } 	WMPTaskType;


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPContextMenuInfo is returned from the IWMPContentPartner::GetCommands API

// Synopsis - struct WMPContextMenuInfo

// Member - dwID -> Indicates the value for the command from the plugin. A value of zero will
//                  cause the media player to insert a separator in the context menu

// Member - bstrMenuText -> Indicates the text that will be show in the context menu

// Member - bstrHelpText -> Indicates help text that could be shown when the user is
//                           highlighting this menu option

typedef /* [public] */ struct WMPContextMenuInfo
    {
    DWORD dwID;
    BSTR bstrMenuText;
    BSTR bstrHelpText;
    } 	WMPContextMenuInfo;


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// These constants are to be used in the IWMPContentPartner::StationEvent

static const WCHAR g_szStationEvent_Started[]                          = L"TrackStarted";  
static const WCHAR g_szStationEvent_Complete[]                         = L"TrackComplete"; 
static const WCHAR g_szStationEvent_Skipped[]                          = L"TrackSkipped";  


extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0000_v0_0_s_ifspec;

#ifndef __IWMPContentContainer_INTERFACE_DEFINED__
#define __IWMPContentContainer_INTERFACE_DEFINED__

/* interface IWMPContentContainer */
/* [unique][uuid][object] */ 


// Synopsis - GetID-> This function returns the ID for this container


// Synopsis - GetPrice-> This function returns the price for this container


// Synopsis - GetType -> This returns the type of this container. Can return g_szCPAlbumID, g_szCPListID, or g_szUnknownLocation


// Synopsis - GetContentCount -> Returns the content count held in this container.


// Synopsis - GetContentPrice -> Returns the price for a piece of content.
//               /*input*/ idxContent -> index of content to get price for. Should be less than value returned by GetContentCount
//               /*output*/ pbstrPrice -> string content the price or g_szContentPrice_Unknown, g_szContentPrice_Free, or g_szContentPrice_CannotBuy


// Synopsis - GetContentID-> Returns the identifier for a piece of content.
//               /*input*/ idxContent -> index of content to indentifier for. Should be less than value returned by GetContentCount
//               /*output*/ pContentID -> indentifier of the content


EXTERN_C const IID IID_IWMPContentContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ad7f4d9c-1a9f-4ed2-9815-ecc0b58cb616")
    IWMPContentContainer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetID( 
            /* [out] */ __RPC__out ULONG *pContentID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrice( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPrice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentCount( 
            /* [out] */ __RPC__out ULONG *pcContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentPrice( 
            /* [in] */ ULONG idxContent,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPrice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentID( 
            /* [in] */ ULONG idxContent,
            /* [out] */ __RPC__out ULONG *pContentID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPContentContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMPContentContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMPContentContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMPContentContainer * This);
        
        DECLSPEC_XFGVIRT(IWMPContentContainer, GetID)
        HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in IWMPContentContainer * This,
            /* [out] */ __RPC__out ULONG *pContentID);
        
        DECLSPEC_XFGVIRT(IWMPContentContainer, GetPrice)
        HRESULT ( STDMETHODCALLTYPE *GetPrice )( 
            __RPC__in IWMPContentContainer * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPrice);
        
        DECLSPEC_XFGVIRT(IWMPContentContainer, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IWMPContentContainer * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(IWMPContentContainer, GetContentCount)
        HRESULT ( STDMETHODCALLTYPE *GetContentCount )( 
            __RPC__in IWMPContentContainer * This,
            /* [out] */ __RPC__out ULONG *pcContent);
        
        DECLSPEC_XFGVIRT(IWMPContentContainer, GetContentPrice)
        HRESULT ( STDMETHODCALLTYPE *GetContentPrice )( 
            __RPC__in IWMPContentContainer * This,
            /* [in] */ ULONG idxContent,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPrice);
        
        DECLSPEC_XFGVIRT(IWMPContentContainer, GetContentID)
        HRESULT ( STDMETHODCALLTYPE *GetContentID )( 
            __RPC__in IWMPContentContainer * This,
            /* [in] */ ULONG idxContent,
            /* [out] */ __RPC__out ULONG *pContentID);
        
        END_INTERFACE
    } IWMPContentContainerVtbl;

    interface IWMPContentContainer
    {
        CONST_VTBL struct IWMPContentContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPContentContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPContentContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPContentContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPContentContainer_GetID(This,pContentID)	\
    ( (This)->lpVtbl -> GetID(This,pContentID) ) 

#define IWMPContentContainer_GetPrice(This,pbstrPrice)	\
    ( (This)->lpVtbl -> GetPrice(This,pbstrPrice) ) 

#define IWMPContentContainer_GetType(This,pbstrType)	\
    ( (This)->lpVtbl -> GetType(This,pbstrType) ) 

#define IWMPContentContainer_GetContentCount(This,pcContent)	\
    ( (This)->lpVtbl -> GetContentCount(This,pcContent) ) 

#define IWMPContentContainer_GetContentPrice(This,idxContent,pbstrPrice)	\
    ( (This)->lpVtbl -> GetContentPrice(This,idxContent,pbstrPrice) ) 

#define IWMPContentContainer_GetContentID(This,idxContent,pContentID)	\
    ( (This)->lpVtbl -> GetContentID(This,idxContent,pContentID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPContentContainer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contentpartner_0000_0001 */
/* [local] */ 


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPTransactionType enum values are passed to the IWMPContentContainerList::GetTransactionType API

// Synopsis - wmpttNoTransaction
// Remarks: Undefined transaction, not currently used

// Synopsis - wmpttDownload
// Remarks: Indicates that this IWMPContentContainerList is to be used for a download transaction

// Synopsis - wmpttBuy
// Remarks: Indicates that this IWMPContentContainerList is to be used for a buy transaction

typedef /* [public] */ 
enum WMPTransactionType
    {
        wmpttNoTransaction	= 0,
        wmpttDownload	= 1,
        wmpttBuy	= 2
    } 	WMPTransactionType;



extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0001_v0_0_s_ifspec;

#ifndef __IWMPContentContainerList_INTERFACE_DEFINED__
#define __IWMPContentContainerList_INTERFACE_DEFINED__

/* interface IWMPContentContainerList */
/* [unique][uuid][object] */ 


// Synopsis - GetTransactionType-> Returns the transaction type.
//               /*output*/ WMPTransactionType *pwmptt -> indicates whether this IWMPContentContainerList will be used to a buy or download


// Synopsis - GetContainerCount-> Returns the number of IWMPContentContainer(s) present in this container list.
//               /*output*/ ULONG *pcContainer -> returns the number of containers


// Synopsis - GetContainer-> Returns the IWMPContentContainer given an index.
//               /*input*/ ULONG idxContainer -> the index of the desired container, should be less than the number returned
//                                               by IWMPContentContainerList::GetContainerCount
//               /*output*/ IWMPContentContainer **ppContent -> will containe the requested IWMPContentContainer on success


EXTERN_C const IID IID_IWMPContentContainerList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a9937f78-0802-4af8-8b8d-e3f045bc8ab5")
    IWMPContentContainerList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTransactionType( 
            /* [out] */ __RPC__out WMPTransactionType *pwmptt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContainerCount( 
            /* [out] */ __RPC__out ULONG *pcContainer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContainer( 
            /* [in] */ ULONG idxContainer,
            /* [out] */ __RPC__deref_out_opt IWMPContentContainer **ppContent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPContentContainerListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMPContentContainerList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMPContentContainerList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMPContentContainerList * This);
        
        DECLSPEC_XFGVIRT(IWMPContentContainerList, GetTransactionType)
        HRESULT ( STDMETHODCALLTYPE *GetTransactionType )( 
            __RPC__in IWMPContentContainerList * This,
            /* [out] */ __RPC__out WMPTransactionType *pwmptt);
        
        DECLSPEC_XFGVIRT(IWMPContentContainerList, GetContainerCount)
        HRESULT ( STDMETHODCALLTYPE *GetContainerCount )( 
            __RPC__in IWMPContentContainerList * This,
            /* [out] */ __RPC__out ULONG *pcContainer);
        
        DECLSPEC_XFGVIRT(IWMPContentContainerList, GetContainer)
        HRESULT ( STDMETHODCALLTYPE *GetContainer )( 
            __RPC__in IWMPContentContainerList * This,
            /* [in] */ ULONG idxContainer,
            /* [out] */ __RPC__deref_out_opt IWMPContentContainer **ppContent);
        
        END_INTERFACE
    } IWMPContentContainerListVtbl;

    interface IWMPContentContainerList
    {
        CONST_VTBL struct IWMPContentContainerListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPContentContainerList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPContentContainerList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPContentContainerList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPContentContainerList_GetTransactionType(This,pwmptt)	\
    ( (This)->lpVtbl -> GetTransactionType(This,pwmptt) ) 

#define IWMPContentContainerList_GetContainerCount(This,pcContainer)	\
    ( (This)->lpVtbl -> GetContainerCount(This,pcContainer) ) 

#define IWMPContentContainerList_GetContainer(This,idxContainer,ppContent)	\
    ( (This)->lpVtbl -> GetContainer(This,idxContainer,ppContent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPContentContainerList_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contentpartner_0000_0002 */
/* [local] */ 


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPTemplateSize enum values are returned from the IWMPContentPartner::GetTemplate API

// Synopsis - wmptsSmall
// Remarks: indicates the media player will allocate 100 pixels in height for the template

// Synopsis - wmptsMedium
// Remarks: indicates the media player will allocate 250 pixels in height for the template

// Synopsis - wmptsLarge
// Remarks: indicates the media player will allocate all of the room expect for the necessary space
//          for the list-control to display a small set of items for the template

typedef /* [public] */ 
enum WMPTemplateSize
    {
        wmptsSmall	= 0,
        wmptsMedium	= ( wmptsSmall + 1 ) ,
        wmptsLarge	= ( wmptsMedium + 1 ) 
    } 	WMPTemplateSize;


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPStreamingType enum values are passed to the returned from the IWMPContentPartner::GetStreamingULR API

// Synopsis - wmpstMusic
// Remarks: indicates the plugin should return an URL for music content

// Synopsis - wmpstVideo
// Remarks: indicates the plugin should return an URL for video content

// Synopsis - wmpstRadio
// Remarks: indicates the plugin should return an URL for radio content

typedef /* [public] */ 
enum WMPStreamingType
    {
        wmpstUnknown	= 0,
        wmpstMusic	= 1,
        wmpstVideo	= 2,
        wmpstRadio	= 3
    } 	WMPStreamingType;


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// The WMPAccountType enum values are returned from the IWMPContentPartner::GetPartnerInfo when API
// the string is g_szContentPartnerInfo_MediaPlayerAccountType passed

// Synopsis - wmpatBuyOnly
// Remarks: indicates the user only has the option to buy content

// Synopsis - wmpatSubscription
// Remarks: indicates the user has a subcription account, which indicates buy is also required for sync to Janus device

// Synopsis - wmpatJanus
// Remarks: indicates the user has a subcirption account, and the ability to sync to Janus devices without having to buy

typedef /* [public] */ 
enum WMPAccountType
    {
        wmpatBuyOnly	= 1,
        wmpatSubscription	= 2,
        wmpatJanus	= 3
    } 	WMPAccountType;



extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0002_v0_0_s_ifspec;

#ifndef __IWMPContentPartnerCallback_INTERFACE_DEFINED__
#define __IWMPContentPartnerCallback_INTERFACE_DEFINED__

/* interface IWMPContentPartnerCallback */
/* [unique][helpstring][uuid][object] */ 


// Synopsis - Notify-> This function is called to notify the media player of plugin events
//            /*input*/  WMPCallbackNotification type - Event that the plugin is notifying the player of
//            /*input*/  VARIANT * pContext - The context depends on the notification being sent. See
//                           remakrs for WMPCallbackNotification


// Synopsis - BuyComplete -> This function is called to notify the media player of that a purchase has completed
//            /*input*/  HRESULT hrResult - The success or error code of the purchase operation.
//            /*input*/  DWORD dwBuyCookie - The cookie passed to IWMPContentPartner::Buy.

// Remarks: This method should ONLY be called on SUCCESS AFTER all licenses have been delivered.

// Synopsis - DownloadTrack -> This function is called in response to a IWMPContentPartner::Download
//            /*input*/  DWORD dwCookie - The cookie passed to IWMPContentPartner::Download.
//            /*input*/  BSTR bstrTrackURL - The URL to download.
//            /*input*/  DWORD dwServiceTrackID - The content identifier of the content to download.
//            /*input*/  BSTR bstrDownloadParams - A string containing data to be passed to
//                                    IWMPContentPartnerCallback::DownloadTrackComplete.
//            /*input*/  HRESULT hrDownload - The success or error code of download operation.

// Remarks: This method should ONLY be called on SUCCESS AFTER all licenses have been delivered.
// Remarks: The tracks will NOT be download if hrDownload is not a success code.

// Synopsis - GetCatalogVersion -> This function is called to retrieve the current catalog version.
//            /*output*/  DWORD *pdwVersion- The major version of the catalog being used.
//            /*output*/  DWORD *pdwSchemaVersion - The schema of the catalog being used.
//            /*output*/  LCID *plcid- The language of the catalog being used.


// Synopsis - UpdateDeviceComplete -> This function is called to indicate that IWMPContentPartner::UpdateDevice has completed.
//            /*input*/  BSTR bstrDeviceName - The device name passed to IWMPContentPartner::UpdateDevice.


// Synopsis - ChangeView-> This function is called to navigate the media player UI
//            /*input*/  BSTR bstrType - a content partner string ID such as g_szCPArtistID, or g_szOnlineStore
//            /*input*/  BSTR bstrID -   The identifier to navigate to
//            /*input*/  BSTR bstrFilter - A filter to place in the 'word wheel' after navigating

// Remarks: When using g_szOnlineStore for bstrType, the bstrID ss a full URL to navigate the browser to
// Remarks: Use 'Back' or 'Forward' in bstrType to simulate pressing the applications back and forward navigation buttons
// Remarks: Use g_szRootLocation in bstrType to navigate to the service's root node in the libary UI


// Synopsis - AddListContents-> This function is called to add content identifiers to a list as a rsult of the 
//                               IWMPContentPartner::GetListContents API being called
//            /*input*/  DWORD dwListCookie - Cookie sent during IWMPContentParnter::GetListContents
//            /*input*/  DWORD cItems - The number of content identifiers being passed in the prgItems parameter
//            /*input*/  DWORD *prgItems - An array of content identifiers


// Synopsis - ListContentsComplete-> This function is called to indicate that the plugin has completed a IWMPContentPartner::GetListContents call
//            /*input*/  DWORD dwListCookie - Cookie sent during IWMPContentParnter::GetListContents 
//            /*input*/  HRESULT hrSuccess - The success or error code of the GetListContents operation


// Synopsis - SendMessageComplete-> This function is called to indicate that the plugin has completed a IWMPContentPartner::SendMessage call
//            /*input*/  BSTR bstrMsg - Message text sent in IWMPContentPartner::SendMessage
//            /*input*/  BSTR bstrParam - Parameter text sent in IWMPContentPartner::SendMessage
//            /*input*/  BSTR bstrResult - Result text


// Synopsis - GetContentIDsInLibrary-> This function is called to retrieve the content identifiers that are in the users
//                                     library that have been purchased or downloaded.
//            /*output*/  ULONG pcContentIDs - The number of content identifiers returned.
//            /*output*/  ULONG **pprgIDs    - An array of ULONGs containing the content identifiers. Free this array with CoTaskMemFree.


// Synopsis - RefreshLicenseComplete -> This function is called to indicate that a license refresh call has completed
//            /*input*/   DWORD dwCookie - This should be the same cookie passed to IWMPContentPartner::RefreshLicense.
//            /*input*/   ULOING contentID - This should be the same content identifier passed to IWMPContentPartner::RefreshLicense.
//            /*input*/   HRESULT hrRefresh - The success or error code of the refresh operation.


// Synopsis - ShowPopup -> This function is called to have the player show a popup dialog. Usually this happens 
//                          as a result of IWMPContentPartner::InvokeCommand being called.
//            /*input*/   long lIndex - The URL index to request from the IWMPContentPartner::GetItemInfo( g_szItemInfo_PopupURL, ... ) API.
//            /*input*/   BSTR bstrParameters - The parameters that are concatenated onto the URL returned by IWMPContentPartner::GetItemInfo( g_szItemInfo_PopupURL, ... ) API


// Synopsis - VerifyPermission-> This function is called to indicate that permission verification is complete
//            /*input*/ BSTR bstrPermission - Indicates action for which permission was requested.
//            /*input*/   VARIANT * pContext - If sync, this is the canonical name of the device. This is the same thing that was passed in as pContext in VerifyPermission.
//            /*input*/   HRESULT hrPermission - Result of permission verification.


EXTERN_C const IID IID_IWMPContentPartnerCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9E8F7DA2-0695-403c-B697-DA10FAFAA676")
    IWMPContentPartnerCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ WMPCallbackNotification type,
            /* [in] */ __RPC__in VARIANT *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BuyComplete( 
            /* [in] */ HRESULT hrResult,
            /* [in] */ DWORD dwBuyCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadTrack( 
            /* [in] */ DWORD cookie,
            /* [in] */ __RPC__in BSTR bstrTrackURL,
            /* [in] */ DWORD dwServiceTrackID,
            /* [in] */ __RPC__in BSTR bstrDownloadParams,
            /* [in] */ HRESULT hrDownload) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCatalogVersion( 
            /* [out] */ __RPC__out DWORD *pdwVersion,
            /* [out] */ __RPC__out DWORD *pdwSchemaVersion,
            /* [out] */ __RPC__out LCID *plcid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateDeviceComplete( 
            /* [in] */ __RPC__in BSTR bstrDeviceName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChangeView( 
            /* [in] */ __RPC__in BSTR bstrType,
            /* [in] */ __RPC__in BSTR bstrID,
            /* [in] */ __RPC__in BSTR bstrFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddListContents( 
            /* [in] */ DWORD dwListCookie,
            /* [in] */ DWORD cItems,
            /* [size_is][in] */ __RPC__in_ecount_full(cItems) DWORD *prgItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ListContentsComplete( 
            /* [in] */ DWORD dwListCookie,
            /* [in] */ HRESULT hrSuccess) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendMessageComplete( 
            /* [in] */ __RPC__in BSTR bstrMsg,
            /* [in] */ __RPC__in BSTR bstrParam,
            /* [in] */ __RPC__in BSTR bstrResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentIDsInLibrary( 
            /* [out] */ __RPC__out ULONG *pcContentIDs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcContentIDs) ULONG **pprgIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RefreshLicenseComplete( 
            /* [in] */ DWORD dwCookie,
            /* [in] */ ULONG contentID,
            /* [in] */ HRESULT hrRefresh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowPopup( 
            /* [in] */ long lIndex,
            /* [in] */ __RPC__in BSTR bstrParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE VerifyPermissionComplete( 
            /* [in] */ __RPC__in BSTR bstrPermission,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [in] */ HRESULT hrPermission) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPContentPartnerCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMPContentPartnerCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMPContentPartnerCallback * This);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ WMPCallbackNotification type,
            /* [in] */ __RPC__in VARIANT *pContext);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, BuyComplete)
        HRESULT ( STDMETHODCALLTYPE *BuyComplete )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ HRESULT hrResult,
            /* [in] */ DWORD dwBuyCookie);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, DownloadTrack)
        HRESULT ( STDMETHODCALLTYPE *DownloadTrack )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ DWORD cookie,
            /* [in] */ __RPC__in BSTR bstrTrackURL,
            /* [in] */ DWORD dwServiceTrackID,
            /* [in] */ __RPC__in BSTR bstrDownloadParams,
            /* [in] */ HRESULT hrDownload);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, GetCatalogVersion)
        HRESULT ( STDMETHODCALLTYPE *GetCatalogVersion )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [out] */ __RPC__out DWORD *pdwVersion,
            /* [out] */ __RPC__out DWORD *pdwSchemaVersion,
            /* [out] */ __RPC__out LCID *plcid);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, UpdateDeviceComplete)
        HRESULT ( STDMETHODCALLTYPE *UpdateDeviceComplete )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ __RPC__in BSTR bstrDeviceName);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, ChangeView)
        HRESULT ( STDMETHODCALLTYPE *ChangeView )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ __RPC__in BSTR bstrType,
            /* [in] */ __RPC__in BSTR bstrID,
            /* [in] */ __RPC__in BSTR bstrFilter);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, AddListContents)
        HRESULT ( STDMETHODCALLTYPE *AddListContents )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ DWORD dwListCookie,
            /* [in] */ DWORD cItems,
            /* [size_is][in] */ __RPC__in_ecount_full(cItems) DWORD *prgItems);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, ListContentsComplete)
        HRESULT ( STDMETHODCALLTYPE *ListContentsComplete )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ DWORD dwListCookie,
            /* [in] */ HRESULT hrSuccess);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, SendMessageComplete)
        HRESULT ( STDMETHODCALLTYPE *SendMessageComplete )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ __RPC__in BSTR bstrMsg,
            /* [in] */ __RPC__in BSTR bstrParam,
            /* [in] */ __RPC__in BSTR bstrResult);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, GetContentIDsInLibrary)
        HRESULT ( STDMETHODCALLTYPE *GetContentIDsInLibrary )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [out] */ __RPC__out ULONG *pcContentIDs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcContentIDs) ULONG **pprgIDs);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, RefreshLicenseComplete)
        HRESULT ( STDMETHODCALLTYPE *RefreshLicenseComplete )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ DWORD dwCookie,
            /* [in] */ ULONG contentID,
            /* [in] */ HRESULT hrRefresh);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, ShowPopup)
        HRESULT ( STDMETHODCALLTYPE *ShowPopup )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ long lIndex,
            /* [in] */ __RPC__in BSTR bstrParameters);
        
        DECLSPEC_XFGVIRT(IWMPContentPartnerCallback, VerifyPermissionComplete)
        HRESULT ( STDMETHODCALLTYPE *VerifyPermissionComplete )( 
            __RPC__in IWMPContentPartnerCallback * This,
            /* [in] */ __RPC__in BSTR bstrPermission,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [in] */ HRESULT hrPermission);
        
        END_INTERFACE
    } IWMPContentPartnerCallbackVtbl;

    interface IWMPContentPartnerCallback
    {
        CONST_VTBL struct IWMPContentPartnerCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPContentPartnerCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPContentPartnerCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPContentPartnerCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPContentPartnerCallback_Notify(This,type,pContext)	\
    ( (This)->lpVtbl -> Notify(This,type,pContext) ) 

#define IWMPContentPartnerCallback_BuyComplete(This,hrResult,dwBuyCookie)	\
    ( (This)->lpVtbl -> BuyComplete(This,hrResult,dwBuyCookie) ) 

#define IWMPContentPartnerCallback_DownloadTrack(This,cookie,bstrTrackURL,dwServiceTrackID,bstrDownloadParams,hrDownload)	\
    ( (This)->lpVtbl -> DownloadTrack(This,cookie,bstrTrackURL,dwServiceTrackID,bstrDownloadParams,hrDownload) ) 

#define IWMPContentPartnerCallback_GetCatalogVersion(This,pdwVersion,pdwSchemaVersion,plcid)	\
    ( (This)->lpVtbl -> GetCatalogVersion(This,pdwVersion,pdwSchemaVersion,plcid) ) 

#define IWMPContentPartnerCallback_UpdateDeviceComplete(This,bstrDeviceName)	\
    ( (This)->lpVtbl -> UpdateDeviceComplete(This,bstrDeviceName) ) 

#define IWMPContentPartnerCallback_ChangeView(This,bstrType,bstrID,bstrFilter)	\
    ( (This)->lpVtbl -> ChangeView(This,bstrType,bstrID,bstrFilter) ) 

#define IWMPContentPartnerCallback_AddListContents(This,dwListCookie,cItems,prgItems)	\
    ( (This)->lpVtbl -> AddListContents(This,dwListCookie,cItems,prgItems) ) 

#define IWMPContentPartnerCallback_ListContentsComplete(This,dwListCookie,hrSuccess)	\
    ( (This)->lpVtbl -> ListContentsComplete(This,dwListCookie,hrSuccess) ) 

#define IWMPContentPartnerCallback_SendMessageComplete(This,bstrMsg,bstrParam,bstrResult)	\
    ( (This)->lpVtbl -> SendMessageComplete(This,bstrMsg,bstrParam,bstrResult) ) 

#define IWMPContentPartnerCallback_GetContentIDsInLibrary(This,pcContentIDs,pprgIDs)	\
    ( (This)->lpVtbl -> GetContentIDsInLibrary(This,pcContentIDs,pprgIDs) ) 

#define IWMPContentPartnerCallback_RefreshLicenseComplete(This,dwCookie,contentID,hrRefresh)	\
    ( (This)->lpVtbl -> RefreshLicenseComplete(This,dwCookie,contentID,hrRefresh) ) 

#define IWMPContentPartnerCallback_ShowPopup(This,lIndex,bstrParameters)	\
    ( (This)->lpVtbl -> ShowPopup(This,lIndex,bstrParameters) ) 

#define IWMPContentPartnerCallback_VerifyPermissionComplete(This,bstrPermission,pContext,hrPermission)	\
    ( (This)->lpVtbl -> VerifyPermissionComplete(This,bstrPermission,pContext,hrPermission) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPContentPartnerCallback_INTERFACE_DEFINED__ */


#ifndef __IWMPContentPartner_INTERFACE_DEFINED__
#define __IWMPContentPartner_INTERFACE_DEFINED__

/* interface IWMPContentPartner */
/* [unique][helpstring][uuid][object] */ 


// Synopsis - SetCallback-> This function allows gives the pugin the ability to store a callback pointer
//                          that will be necessary to communicate with the player
//             /*input*/ pCallback -> interface pointer to the media player's implementation of the IWMPContentPartnerCallback
//
// Remarks: The player will call this with a null value during its shutdown process. The plugin can use


// Synopsis - Notify-> This function is called by the player to notify the plugin of certain events
//            /*input*/ WMPPartnerNotification type - see comments for WMPPartnerNotification enum
//            /*input*/ VARIANT *pContext - see comments for WMPPartnerNotification enum


// Synopsis - GetItemInfo-> This function is called to retrieve different properties about content
//            /*input*/ VARIANT *pContext - see comments above for the strings passed into this API
//            /*out*/   VARIANT *pData - see comments abouve for the strings passed into this API


// Synopsis - GetContentPartnerInfo-> This function is called to retrieve different properties about the content partner
//            /*input*/ BSTR bstrInfoName - see comments above for the strings passed into this API 
//            /*out*/   VARIANT *pData - see comments above for the strings passed into this API


// Synopsis - GetCommands-> This function allows the plugin to return commands to be displayed in context menus
//             /*input*/  BSTR location -> location in the library (such as g_szCPArtistID)
//             /*input*/  VARIANT *pLocationContext -> location in the library (such as VT_UI4, 1231231)
//             /*input*/  BSTR itemLocation -> item type selected in library (such as g_szCPTrackID)
//             /*input*/  ULONG cItemIDs -> the number of selected items of type itemLocation
//             /*input*/  ULONG prgItemIDs -> the array of content identifiers
//             /*output*/ ULONG *pcItemIDs -> the number of context menu commands returned in pprgItems
//             /*output*/ WMPContextMenuInfo **pprgItems -> Array of WMPContenxtMenuInfo items allocated by CoTaskMemAlloc

//             Remarks: See comments for WMPContextMenuInfo for more information


// Synopsis - InvokeCommand-> This function is called when the user selects an option fro a context menu command
//                            returned by IWMPContentPartner::GetCommands
//             /*input*/  DWORD dwCommandID -> Command ID from the select WMPContextMenuInfo
//             /*input*/  The remaining parameters are the same as those passed to IWMPContentPartner::GetCommands

//             Remarks: See comments for WMPContextMenuInfo and IWMPContentPartner::GetCommands for more information


// Synopsis - GetCatalogURL -> This function is called by the player to retrieve the streaming URL
//             /*input*/  WMPStreamingType -> indicates whether the streaming type is for radio\music\video
//             /*input*/  VARIANT *pStreamContext -> VT_UI4, contains the content identifier
//             /*ouptut*/ BSTR *pbstrURL-> recieves the streaming URL of the content

//             Remarks: The streaming URL for Radio and Video must be contained in an ASX


// Synopsis - GetCatalogURL -> This function is called by the player to retrieve the catalog download URL
//             /*input*/  DWORD dwCatalogVersion -> current catalog version, or zero if none exists
//             /*input*/  DWORD dwCatalogSchemaVesrion -> current catalog schema
//             /*input*/  LCID catalogLCID -> current catalog version
//             /*output*/ DWORD *pdwNewCatalogVersion -> version of catalog that URL represents
//             /*ouptut*/ BSTR *pbstrCatalogURL-> recieves URL that the player will use to download catalog
//             /*output*/ VARIANT *pExpirationData-> Must be of type VT_DATE. This is the date at which the player will
//                           will call IWMPContentPartner::GetCatalogURL to retrieve a new catalog.


// Synopsis - UpdateDevice -> This function is called during sync to allow the plugin to do device updates.
//             /*input*/ BSTR bstrDeviceName-> name of device to update
//
// Remarks: This call should function asynchronously. Call IWMPContentPartnerCallback::UpdateDeviceComplete when finsihed.


// Synopsis - Login-> This function indicates to the plugin to begin logging in to the service
//             /*input*/ BLOB userInfo, an encrypted blob containing the user's login name
//             /*input*/ BLOB pwdInfo,  an encrypted blob containing the user's login password
//             /*input*/ VARIANT_BOOL fUsedCachedCreds, indicates to the plugin to attempt to use cached credentials
//                                                       for login if present.
//             /*input*/ VARIANT_BOOL fOkToCache, indicates that it is acceptable for the plugin to cache
//                                                 the users credential.
//
// Remarks: The plugin is responsible for calling IWMPContentPartner::Notify to indicate login state change
//
// Remarks: To unencrypt the user and password name, use 
//   CryptUnprotectData( blob, NULL, NULL, NULL, NULL, CRYPTPROTECT_UI_FORBIDDEN, &outBlob)


// Synopsis - Authenticate-> This function indicates to the plugin to Authenticate the credentials with the service
//             /*input*/ BLOB userInfo, an encrypted blob containing the user's login name
//             /*input*/ BLOB pwdInfo,  an encrypted blob containing the user's login password
//
// Remarks: To unencrypt the user and password name, use 
//   CryptUnprotectData( blob, NULL, NULL, NULL, NULL, CRYPTPROTECT_UI_FORBIDDEN, &outBlob)


// Synopsis - Logout-> This function indicates to the plugin to begin logging out from the service
// 
// Remarks: The plugin is responsible for calling IWMPContentPartner::Notify to indicate login state change
//
// Remarks: The plugin should delete any cached credentials when this API is called


// Synopsis - SendMessage -> This function allows HTML templates returned from IWMPContentPartner::GetTemplate
//                          to communicate to the plugin
//             /*input*/ bstrMsg -> value send directly from the template returned by IWMPContentPartner::GetTemplate
//                               -> via the window.external.sendMessage API
//             /*input*/ bstrParam -> parameter for the message, also sent from the window.external.sendMessage API
//
//Remarks: The plugin is reponsible for calling IWMPContentPartnerCallback::OnSendMessageComplete in order for
//         the HTML template to know that processing of the message occurs. This will be sent to the template
//         via the window.external.OnSendMessageComplete notificaiton


// Synopsis - CompareContainerListPrices-> This function is called to preform a comparison operation on tow IWMPContentContainerLists
//            /*input*/   IWMPContentContainerList *pListBase - 
//            /*input*/   IWMPContentContainerList *pListCompare - 
//            /*output*/  long *pResult* - Return less than 0 when pListBase is less than pListCompare, 0 for equal, and greather than 0 when pListBase is more than pListCompare


// Synopsis - VerifyPermission-> This function is called to get permission to perform an action
//            /*input*/ BSTR bstrPermission - Indicates action for which permission is requested
//            /*input*/   VARIANT * pContext - If sync, this is the canonical name of the device. This needs to be returned in VerifyPermissionComplete.


EXTERN_C const IID IID_IWMPContentPartner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55455073-41B5-4e75-87B8-F13BDB291D08")
    IWMPContentPartner : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCallback( 
            /* [in] */ __RPC__in_opt IWMPContentPartnerCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ WMPPartnerNotification type,
            /* [in] */ __RPC__in VARIANT *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemInfo( 
            /* [in] */ __RPC__in BSTR bstrInfoName,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [out] */ __RPC__out VARIANT *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentPartnerInfo( 
            /* [in] */ __RPC__in BSTR bstrInfoName,
            /* [out] */ __RPC__out VARIANT *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCommands( 
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pLocationContext,
            /* [in] */ __RPC__in BSTR itemLocation,
            /* [in] */ ULONG cItemIDs,
            /* [size_is][in] */ __RPC__in_ecount_full(cItemIDs) ULONG *prgItemIDs,
            /* [out] */ __RPC__out ULONG *pcItemIDs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcItemIDs) WMPContextMenuInfo **pprgItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InvokeCommand( 
            /* [in] */ DWORD dwCommandID,
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pLocationContext,
            /* [in] */ __RPC__in BSTR itemLocation,
            /* [in] */ ULONG cItemIDs,
            /* [size_is][in] */ __RPC__in_ecount_full(cItemIDs) ULONG *rgItemIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanBuySilent( 
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pInfo,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrTotalPrice,
            /* [out] */ __RPC__out VARIANT_BOOL *pSilentOK) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Buy( 
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pInfo,
            /* [in] */ DWORD cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamingURL( 
            /* [in] */ WMPStreamingType st,
            /* [in] */ __RPC__in VARIANT *pStreamContext,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Download( 
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pInfo,
            /* [in] */ DWORD cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DownloadTrackComplete( 
            /* [in] */ HRESULT hrResult,
            /* [in] */ ULONG contentID,
            /* [in] */ __RPC__in BSTR downloadTrackParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RefreshLicense( 
            /* [in] */ DWORD dwCookie,
            /* [in] */ VARIANT_BOOL fLocal,
            /* [unique][in] */ __RPC__in_opt BSTR bstrURL,
            /* [in] */ WMPStreamingType type,
            /* [in] */ ULONG contentID,
            /* [in] */ __RPC__in BSTR bstrRefreshReason,
            /* [in] */ __RPC__in VARIANT *pReasonContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCatalogURL( 
            /* [in] */ DWORD dwCatalogVersion,
            /* [in] */ DWORD dwCatalogSchemaVersion,
            /* [in] */ LCID catalogLCID,
            /* [out] */ __RPC__out DWORD *pdwNewCatalogVersion,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCatalogURL,
            /* [out] */ __RPC__out VARIANT *pExpirationDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTemplate( 
            /* [in] */ WMPTaskType task,
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [in] */ __RPC__in BSTR clickLocation,
            /* [in] */ __RPC__in VARIANT *pClickContext,
            /* [in] */ __RPC__in BSTR bstrFilter,
            /* [in] */ __RPC__in BSTR bstrViewParams,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrTemplateURL,
            /* [out] */ __RPC__out WMPTemplateSize *pTemplateSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateDevice( 
            /* [in] */ __RPC__in BSTR bstrDeviceName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetListContents( 
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [in] */ __RPC__in BSTR bstrListType,
            /* [in] */ __RPC__in BSTR bstrParams,
            /* [in] */ DWORD dwListCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Login( 
            /* [in] */ BLOB userInfo,
            /* [in] */ BLOB pwdInfo,
            /* [in] */ VARIANT_BOOL fUsedCachedCreds,
            /* [in] */ VARIANT_BOOL fOkToCache) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Authenticate( 
            /* [in] */ BLOB userInfo,
            /* [in] */ BLOB pwdInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Logout( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendMessage( 
            /* [in] */ __RPC__in BSTR bstrMsg,
            /* [in] */ __RPC__in BSTR bstrParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StationEvent( 
            /* [in] */ __RPC__in BSTR bstrStationEventType,
            /* [in] */ ULONG StationId,
            /* [in] */ ULONG PlaylistIndex,
            /* [in] */ ULONG TrackID,
            /* [in] */ __RPC__in BSTR TrackData,
            /* [in] */ DWORD dwSecondsPlayed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompareContainerListPrices( 
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pListBase,
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pListCompare,
            /* [out] */ __RPC__out long *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE VerifyPermission( 
            /* [in] */ __RPC__in BSTR bstrPermission,
            /* [in] */ __RPC__in VARIANT *pContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPContentPartnerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMPContentPartner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMPContentPartner * This);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, SetCallback)
        HRESULT ( STDMETHODCALLTYPE *SetCallback )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in_opt IWMPContentPartnerCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ WMPPartnerNotification type,
            /* [in] */ __RPC__in VARIANT *pContext);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, GetItemInfo)
        HRESULT ( STDMETHODCALLTYPE *GetItemInfo )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR bstrInfoName,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [out] */ __RPC__out VARIANT *pData);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, GetContentPartnerInfo)
        HRESULT ( STDMETHODCALLTYPE *GetContentPartnerInfo )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR bstrInfoName,
            /* [out] */ __RPC__out VARIANT *pData);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, GetCommands)
        HRESULT ( STDMETHODCALLTYPE *GetCommands )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pLocationContext,
            /* [in] */ __RPC__in BSTR itemLocation,
            /* [in] */ ULONG cItemIDs,
            /* [size_is][in] */ __RPC__in_ecount_full(cItemIDs) ULONG *prgItemIDs,
            /* [out] */ __RPC__out ULONG *pcItemIDs,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcItemIDs) WMPContextMenuInfo **pprgItems);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, InvokeCommand)
        HRESULT ( STDMETHODCALLTYPE *InvokeCommand )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ DWORD dwCommandID,
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pLocationContext,
            /* [in] */ __RPC__in BSTR itemLocation,
            /* [in] */ ULONG cItemIDs,
            /* [size_is][in] */ __RPC__in_ecount_full(cItemIDs) ULONG *rgItemIDs);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, CanBuySilent)
        HRESULT ( STDMETHODCALLTYPE *CanBuySilent )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pInfo,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrTotalPrice,
            /* [out] */ __RPC__out VARIANT_BOOL *pSilentOK);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, Buy)
        HRESULT ( STDMETHODCALLTYPE *Buy )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pInfo,
            /* [in] */ DWORD cookie);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, GetStreamingURL)
        HRESULT ( STDMETHODCALLTYPE *GetStreamingURL )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ WMPStreamingType st,
            /* [in] */ __RPC__in VARIANT *pStreamContext,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, Download)
        HRESULT ( STDMETHODCALLTYPE *Download )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pInfo,
            /* [in] */ DWORD cookie);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, DownloadTrackComplete)
        HRESULT ( STDMETHODCALLTYPE *DownloadTrackComplete )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ HRESULT hrResult,
            /* [in] */ ULONG contentID,
            /* [in] */ __RPC__in BSTR downloadTrackParam);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, RefreshLicense)
        HRESULT ( STDMETHODCALLTYPE *RefreshLicense )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ DWORD dwCookie,
            /* [in] */ VARIANT_BOOL fLocal,
            /* [unique][in] */ __RPC__in_opt BSTR bstrURL,
            /* [in] */ WMPStreamingType type,
            /* [in] */ ULONG contentID,
            /* [in] */ __RPC__in BSTR bstrRefreshReason,
            /* [in] */ __RPC__in VARIANT *pReasonContext);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, GetCatalogURL)
        HRESULT ( STDMETHODCALLTYPE *GetCatalogURL )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ DWORD dwCatalogVersion,
            /* [in] */ DWORD dwCatalogSchemaVersion,
            /* [in] */ LCID catalogLCID,
            /* [out] */ __RPC__out DWORD *pdwNewCatalogVersion,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCatalogURL,
            /* [out] */ __RPC__out VARIANT *pExpirationDate);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, GetTemplate)
        HRESULT ( STDMETHODCALLTYPE *GetTemplate )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ WMPTaskType task,
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [in] */ __RPC__in BSTR clickLocation,
            /* [in] */ __RPC__in VARIANT *pClickContext,
            /* [in] */ __RPC__in BSTR bstrFilter,
            /* [in] */ __RPC__in BSTR bstrViewParams,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrTemplateURL,
            /* [out] */ __RPC__out WMPTemplateSize *pTemplateSize);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, UpdateDevice)
        HRESULT ( STDMETHODCALLTYPE *UpdateDevice )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR bstrDeviceName);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, GetListContents)
        HRESULT ( STDMETHODCALLTYPE *GetListContents )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR location,
            /* [in] */ __RPC__in VARIANT *pContext,
            /* [in] */ __RPC__in BSTR bstrListType,
            /* [in] */ __RPC__in BSTR bstrParams,
            /* [in] */ DWORD dwListCookie);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, Login)
        HRESULT ( STDMETHODCALLTYPE *Login )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ BLOB userInfo,
            /* [in] */ BLOB pwdInfo,
            /* [in] */ VARIANT_BOOL fUsedCachedCreds,
            /* [in] */ VARIANT_BOOL fOkToCache);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, Authenticate)
        HRESULT ( STDMETHODCALLTYPE *Authenticate )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ BLOB userInfo,
            /* [in] */ BLOB pwdInfo);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, Logout)
        HRESULT ( STDMETHODCALLTYPE *Logout )( 
            __RPC__in IWMPContentPartner * This);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, SendMessage)
        HRESULT ( STDMETHODCALLTYPE *SendMessage )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR bstrMsg,
            /* [in] */ __RPC__in BSTR bstrParam);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, StationEvent)
        HRESULT ( STDMETHODCALLTYPE *StationEvent )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR bstrStationEventType,
            /* [in] */ ULONG StationId,
            /* [in] */ ULONG PlaylistIndex,
            /* [in] */ ULONG TrackID,
            /* [in] */ __RPC__in BSTR TrackData,
            /* [in] */ DWORD dwSecondsPlayed);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, CompareContainerListPrices)
        HRESULT ( STDMETHODCALLTYPE *CompareContainerListPrices )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pListBase,
            /* [in] */ __RPC__in_opt IWMPContentContainerList *pListCompare,
            /* [out] */ __RPC__out long *pResult);
        
        DECLSPEC_XFGVIRT(IWMPContentPartner, VerifyPermission)
        HRESULT ( STDMETHODCALLTYPE *VerifyPermission )( 
            __RPC__in IWMPContentPartner * This,
            /* [in] */ __RPC__in BSTR bstrPermission,
            /* [in] */ __RPC__in VARIANT *pContext);
        
        END_INTERFACE
    } IWMPContentPartnerVtbl;

    interface IWMPContentPartner
    {
        CONST_VTBL struct IWMPContentPartnerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPContentPartner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPContentPartner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPContentPartner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPContentPartner_SetCallback(This,pCallback)	\
    ( (This)->lpVtbl -> SetCallback(This,pCallback) ) 

#define IWMPContentPartner_Notify(This,type,pContext)	\
    ( (This)->lpVtbl -> Notify(This,type,pContext) ) 

#define IWMPContentPartner_GetItemInfo(This,bstrInfoName,pContext,pData)	\
    ( (This)->lpVtbl -> GetItemInfo(This,bstrInfoName,pContext,pData) ) 

#define IWMPContentPartner_GetContentPartnerInfo(This,bstrInfoName,pData)	\
    ( (This)->lpVtbl -> GetContentPartnerInfo(This,bstrInfoName,pData) ) 

#define IWMPContentPartner_GetCommands(This,location,pLocationContext,itemLocation,cItemIDs,prgItemIDs,pcItemIDs,pprgItems)	\
    ( (This)->lpVtbl -> GetCommands(This,location,pLocationContext,itemLocation,cItemIDs,prgItemIDs,pcItemIDs,pprgItems) ) 

#define IWMPContentPartner_InvokeCommand(This,dwCommandID,location,pLocationContext,itemLocation,cItemIDs,rgItemIDs)	\
    ( (This)->lpVtbl -> InvokeCommand(This,dwCommandID,location,pLocationContext,itemLocation,cItemIDs,rgItemIDs) ) 

#define IWMPContentPartner_CanBuySilent(This,pInfo,pbstrTotalPrice,pSilentOK)	\
    ( (This)->lpVtbl -> CanBuySilent(This,pInfo,pbstrTotalPrice,pSilentOK) ) 

#define IWMPContentPartner_Buy(This,pInfo,cookie)	\
    ( (This)->lpVtbl -> Buy(This,pInfo,cookie) ) 

#define IWMPContentPartner_GetStreamingURL(This,st,pStreamContext,pbstrURL)	\
    ( (This)->lpVtbl -> GetStreamingURL(This,st,pStreamContext,pbstrURL) ) 

#define IWMPContentPartner_Download(This,pInfo,cookie)	\
    ( (This)->lpVtbl -> Download(This,pInfo,cookie) ) 

#define IWMPContentPartner_DownloadTrackComplete(This,hrResult,contentID,downloadTrackParam)	\
    ( (This)->lpVtbl -> DownloadTrackComplete(This,hrResult,contentID,downloadTrackParam) ) 

#define IWMPContentPartner_RefreshLicense(This,dwCookie,fLocal,bstrURL,type,contentID,bstrRefreshReason,pReasonContext)	\
    ( (This)->lpVtbl -> RefreshLicense(This,dwCookie,fLocal,bstrURL,type,contentID,bstrRefreshReason,pReasonContext) ) 

#define IWMPContentPartner_GetCatalogURL(This,dwCatalogVersion,dwCatalogSchemaVersion,catalogLCID,pdwNewCatalogVersion,pbstrCatalogURL,pExpirationDate)	\
    ( (This)->lpVtbl -> GetCatalogURL(This,dwCatalogVersion,dwCatalogSchemaVersion,catalogLCID,pdwNewCatalogVersion,pbstrCatalogURL,pExpirationDate) ) 

#define IWMPContentPartner_GetTemplate(This,task,location,pContext,clickLocation,pClickContext,bstrFilter,bstrViewParams,pbstrTemplateURL,pTemplateSize)	\
    ( (This)->lpVtbl -> GetTemplate(This,task,location,pContext,clickLocation,pClickContext,bstrFilter,bstrViewParams,pbstrTemplateURL,pTemplateSize) ) 

#define IWMPContentPartner_UpdateDevice(This,bstrDeviceName)	\
    ( (This)->lpVtbl -> UpdateDevice(This,bstrDeviceName) ) 

#define IWMPContentPartner_GetListContents(This,location,pContext,bstrListType,bstrParams,dwListCookie)	\
    ( (This)->lpVtbl -> GetListContents(This,location,pContext,bstrListType,bstrParams,dwListCookie) ) 

#define IWMPContentPartner_Login(This,userInfo,pwdInfo,fUsedCachedCreds,fOkToCache)	\
    ( (This)->lpVtbl -> Login(This,userInfo,pwdInfo,fUsedCachedCreds,fOkToCache) ) 

#define IWMPContentPartner_Authenticate(This,userInfo,pwdInfo)	\
    ( (This)->lpVtbl -> Authenticate(This,userInfo,pwdInfo) ) 

#define IWMPContentPartner_Logout(This)	\
    ( (This)->lpVtbl -> Logout(This) ) 

#define IWMPContentPartner_SendMessage(This,bstrMsg,bstrParam)	\
    ( (This)->lpVtbl -> SendMessage(This,bstrMsg,bstrParam) ) 

#define IWMPContentPartner_StationEvent(This,bstrStationEventType,StationId,PlaylistIndex,TrackID,TrackData,dwSecondsPlayed)	\
    ( (This)->lpVtbl -> StationEvent(This,bstrStationEventType,StationId,PlaylistIndex,TrackID,TrackData,dwSecondsPlayed) ) 

#define IWMPContentPartner_CompareContainerListPrices(This,pListBase,pListCompare,pResult)	\
    ( (This)->lpVtbl -> CompareContainerListPrices(This,pListBase,pListCompare,pResult) ) 

#define IWMPContentPartner_VerifyPermission(This,bstrPermission,pContext)	\
    ( (This)->lpVtbl -> VerifyPermission(This,bstrPermission,pContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPContentPartner_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contentpartner_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contentpartner_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


