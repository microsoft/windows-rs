#ifndef EXDISPID_H_
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation. All Rights Reserved.
//
//  File: exdispid.h
//
//--------------------------------------------------------------------------

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Dispatch IDS for IExplorer Dispatch Events.
//
#define DISPID_BEFORENAVIGATE     100   // this is sent before navigation to give a chance to abort
#define DISPID_NAVIGATECOMPLETE   101   // in async, this is sent when we have enough to show
#define DISPID_STATUSTEXTCHANGE   102
#define DISPID_QUIT               103
#define DISPID_DOWNLOADCOMPLETE   104
#define DISPID_COMMANDSTATECHANGE 105
#define DISPID_DOWNLOADBEGIN      106
#define DISPID_NEWWINDOW          107   // sent when a new window should be created
#define DISPID_PROGRESSCHANGE     108   // sent when download progress is updated
#define DISPID_WINDOWMOVE         109   // sent when main window has been moved
#define DISPID_WINDOWRESIZE       110   // sent when main window has been sized
#define DISPID_WINDOWACTIVATE     111   // sent when main window has been activated
#define DISPID_PROPERTYCHANGE     112   // sent when the PutProperty method is called
#define DISPID_TITLECHANGE        113   // sent when the document title changes
#define DISPID_TITLEICONCHANGE    114   // sent when the top level window icon may have changed.

#define DISPID_FRAMEBEFORENAVIGATE    200
#define DISPID_FRAMENAVIGATECOMPLETE  201
#define DISPID_FRAMENEWWINDOW         204

#define DISPID_BEFORENAVIGATE2              250           // hyperlink clicked on
#define DISPID_NEWWINDOW2                   251
#define DISPID_NAVIGATECOMPLETE2            252           // UIActivate new document
#define DISPID_ONQUIT                       253
#define DISPID_ONVISIBLE                    254           // sent when the window goes visible/hidden
#define DISPID_ONTOOLBAR                    255           // sent when the toolbar should be shown/hidden
#define DISPID_ONMENUBAR                    256           // sent when the menubar should be shown/hidden
#define DISPID_ONSTATUSBAR                  257           // sent when the statusbar should be shown/hidden
#define DISPID_ONFULLSCREEN                 258           // sent when kiosk mode should be on/off
#define DISPID_DOCUMENTCOMPLETE             259           // new document goes ReadyState_Complete
#define DISPID_ONTHEATERMODE                260           // sent when theater mode should be on/off
#define DISPID_ONADDRESSBAR                 261           // sent when the address bar should be shown/hidden
#define DISPID_WINDOWSETRESIZABLE           262           // sent to set the style of the host window frame
#define DISPID_WINDOWCLOSING                263           // sent before script window.close closes the window 
#define DISPID_WINDOWSETLEFT                264           // sent when the put_left method is called on the WebOC
#define DISPID_WINDOWSETTOP                 265           // sent when the put_top method is called on the WebOC
#define DISPID_WINDOWSETWIDTH               266           // sent when the put_width method is called on the WebOC
#define DISPID_WINDOWSETHEIGHT              267           // sent when the put_height method is called on the WebOC 
#define DISPID_CLIENTTOHOSTWINDOW           268           // sent during window.open to request conversion of dimensions
#define DISPID_SETSECURELOCKICON            269           // sent to suggest the appropriate security icon to show
#define DISPID_FILEDOWNLOAD                 270           // Fired to indicate the File Download dialog is opening
#define DISPID_NAVIGATEERROR                271           // Fired to indicate the a binding error has occured
#define DISPID_PRIVACYIMPACTEDSTATECHANGE   272           // Fired when the user's browsing experience is impacted
#define DISPID_NEWWINDOW3                   273
#define DISPID_VIEWUPDATE                   281           // Fired when the contents of a shell browser window change
#define DISPID_SETPHISHINGFILTERSTATUS      282           // Fired by the Phishing Filter API to signal what state the analysis is in
#define DISPID_WINDOWSTATECHANGED           283           // Fired to indicate that the browser window's visibility or enabled state has changed
#define DISPID_NEWPROCESS                   284           // Fired when a navigation must be redirected due to Protected Mode
#define DISPID_THIRDPARTYURLBLOCKED         285           // Fired when a third-party url is blocked due to Privacy Advisor   
#define DISPID_REDIRECTXDOMAINBLOCKED       286           // Fired when a x-domain redirect is blocked due to browser nav constant   
#define DISPID_WEBWORKERSTARTED             288
#define DISPID_WEBWORKERFINISHED            289
#define DISPID_BEFORESCRIPTEXECUTE          290           // Fired prior to any of a page's script is executed

// Printing events
#define DISPID_PRINTTEMPLATEINSTANTIATION   225           // Fired to indicate that a print template is instantiated
#define DISPID_PRINTTEMPLATETEARDOWN        226           // Fired to indicate that a print templete is completely gone 
#define DISPID_UPDATEPAGESTATUS             227           // Fired to indicate that the spooling status has changed

// define the events for the shell window list
#define DISPID_WINDOWREGISTERED             200           // Window registered
#define DISPID_WINDOWREVOKED                201           // Window Revoked

#define DISPID_RESETFIRSTBOOTMODE       1
#define DISPID_RESETSAFEMODE            2
#define DISPID_REFRESHOFFLINEDESKTOP    3
#define DISPID_ADDFAVORITE              4
#define DISPID_ADDCHANNEL               5
#define DISPID_ADDDESKTOPCOMPONENT      6
#define DISPID_ISSUBSCRIBED             7
#define DISPID_NAVIGATEANDFIND          8
#define DISPID_IMPORTEXPORTFAVORITES    9
#define DISPID_AUTOCOMPLETESAVEFORM     10
#define DISPID_AUTOSCAN                 11
#define DISPID_AUTOCOMPLETEATTACH       12
#define DISPID_SHOWBROWSERUI            13
#define DISPID_ADDSEARCHPROVIDER        14
#define DISPID_RUNONCESHOWN             15
#define DISPID_SKIPRUNONCE              16
#define DISPID_CUSTOMIZESETTINGS        17
#define DISPID_SQMENABLED               18
#define DISPID_PHISHINGENABLED          19
#define DISPID_BRANDIMAGEURI            20
#define DISPID_SKIPTABSWELCOME          21
#define DISPID_DIAGNOSECONNECTION       22
#define DISPID_CUSTOMIZECLEARTYPE       23
#define DISPID_ISSEARCHPROVIDERINSTALLED 24
#define DISPID_ISSEARCHMIGRATED         25
#define DISPID_DEFAULTSEARCHPROVIDER    26
#define DISPID_RUNONCEREQUIREDSETTINGSCOMPLETE 27
#define DISPID_RUNONCEHASSHOWN          28
#define DISPID_SEARCHGUIDEURL           29
#define DISPID_ADDSERVICE               30
#define DISPID_ISSERVICEINSTALLED       31
#define DISPID_ADDTOFAVORITESBAR        32
#define DISPID_BUILDNEWTABPAGE          33
#define DISPID_SETRECENTLYCLOSEDVISIBLE 34
#define DISPID_SETACTIVITIESVISIBLE     35
#define DISPID_CONTENTDISCOVERYRESET    36
#define DISPID_INPRIVATEFILTERINGENABLED    37
#define DISPID_SUGGESTEDSITESENABLED    38
#define DISPID_ENABLESUGGESTEDSITES     39
#define DISPID_NAVIGATETOSUGGESTEDSITES 40
#define DISPID_SHOWTABSHELP             41
#define DISPID_SHOWINPRIVATEHELP        42
#define DISPID_ISSITEMODE               43
#define DISPID_SETSITEMODEICONOVERLAY   44
#define DISPID_CLEARSITEMODEICONOVERLAY 45
#define DISPID_UPDATETHUMBNAILBUTTON    46
#define DISPID_SETTHUMBNAILBUTTONS      47 
#define DISPID_ADDTHUMBNAILBUTTONS      48
#define DISPID_ADDSITEMODE              49
#define DISPID_SETSITEMODEPROPERTIES    50
#define DISPID_SITEMODECREATEJUMPLIST   51
#define DISPID_SITEMODEADDJUMPLISTITEM  52
#define DISPID_SITEMODECLEARJUMPLIST    53
#define DISPID_SITEMODEADDBUTTONSTYLE   54 
#define DISPID_SITEMODESHOWBUTTONSTYLE  55
#define DISPID_SITEMODESHOWJUMPLIST     56
#define DISPID_ADDTRACKINGPROTECTIONLIST    57
#define DISPID_SITEMODEACTIVATE         58
#define DISPID_ISSITEMODEFIRSTRUN       59
#define DISPID_TRACKINGPROTECTIONENABLED 60
#define DISPID_ACTIVEXFILTERINGENABLED  61
#define DISPID_PROVISIONNETWORKS        62
#define DISPID_REPORTSAFEURL            63
#define DISPID_SITEMODEREFRESHBADGE     64
#define DISPID_SITEMODECLEARBADGE       65
#define DISPID_DIAGNOSECONNECTIONUILESS 66
#define DISPID_LAUNCHNETWORKCLIENTHELP  67
#define DISPID_CHANGEDEFAULTBROWSER     68
#define DISPID_STOPPERIODICUPDATE       69
#define DISPID_STARTPERIODICUPDATE      70
#define DISPID_CLEARNOTIFICATION        71
#define DISPID_ENABLENOTIFICATIONQUEUE  72
#define DISPID_PINNEDSITESTATE          73
#define DISPID_LAUNCHINTERNETOPTIONS   74
#define DISPID_STARTPERIODICUPDATEBATCH 75
#define DISPID_ENABLENOTIFICATIONQUEUESQUARE   76
#define DISPID_ENABLENOTIFICATIONQUEUEWIDE     77
#define DISPID_ENABLENOTIFICATIONQUEUELARGE    78
#define DISPID_SCHEDULEDTILENOTIFICATION       79
#define DISPID_REMOVESCHEDULEDTILENOTIFICATION 80
#define DISPID_STARTBADGEUPDATE         81
#define DISPID_STOPBADGEUPDATE          82
#define DISPID_ISMETAREFERRERAVAILABLE  83
#define DISPID_SETEXPERIMENTALFLAG      84
#define DISPID_GETEXPERIMENTALFLAG      85
#define DISPID_SETEXPERIMENTALVALUE     86
#define DISPID_GETEXPERIMENTALVALUE     87
#define DISPID_HASNEEDIEAUTOLAUNCHFLAG  88
#define DISPID_GETNEEDIEAUTOLAUNCHFLAG  89
#define DISPID_SETNEEDIEAUTOLAUNCHFLAG  90
#define DISPID_LAUNCHIE                 91
#define DISPID_RESETEXPERIMENTALFLAGS   92
#define DISPID_GETCVLISTDATA            93
#define DISPID_GETCVLISTLOCALDATA       94
#define DISPID_GETEMIELISTDATA          95
#define DISPID_GETEMIELISTLOCALDATA     96
#define DISPID_OPENFAVORITESPANE        97
#define DISPID_OPENFAVORITESSETTINGS    98
#define DISPID_LAUNCHINHVSI             99
#define DISPID_GETNEEDHVSIAUTOLAUNCHFLAG 100
#define DISPID_SETNEEDHVSIAUTOLAUNCHFLAG 101
#define DISPID_HASNEEDHVSIAUTOLAUNCHFLAG 102
#define DISPID_GETOSSKU                  103
#define DISPID_SETMSDEFAULTS             104
#define DISPID_SHELLUIHELPERLAST         105

#define DISPID_ADVANCEERROR             10
#define DISPID_RETREATERROR             11
#define DISPID_CANADVANCEERROR          12
#define DISPID_CANRETREATERROR          13
#define DISPID_GETERRORLINE             14
#define DISPID_GETERRORCHAR             15
#define DISPID_GETERRORCODE             16
#define DISPID_GETERRORMSG              17
#define DISPID_GETERRORURL              18
#define DISPID_GETDETAILSSTATE          19
#define DISPID_SETDETAILSSTATE          20
#define DISPID_GETPERERRSTATE           21
#define DISPID_SETPERERRSTATE           22
#define DISPID_GETALWAYSSHOWLOCKSTATE   23

// Dispatch IDS for ShellFavoritesNameSpace Dispatch Events.
//
#define DISPID_FAVSELECTIONCHANGE       1
#define DISPID_SELECTIONCHANGE          2
#define DISPID_DOUBLECLICK              3
#define DISPID_INITIALIZED              4

#define DISPID_MOVESELECTIONUP          1
#define DISPID_MOVESELECTIONDOWN        2
#define DISPID_RESETSORT                3
#define DISPID_NEWFOLDER                4
#define DISPID_SYNCHRONIZE              5
#define DISPID_IMPORT                   6
#define DISPID_EXPORT                   7
#define DISPID_INVOKECONTEXTMENU        8
#define DISPID_MOVESELECTIONTO          9
#define DISPID_SUBSCRIPTIONSENABLED     10
#define DISPID_CREATESUBSCRIPTION       11
#define DISPID_DELETESUBSCRIPTION       12
#define DISPID_SETROOT                  13
#define DISPID_ENUMOPTIONS              14
#define DISPID_SELECTEDITEM             15
#define DISPID_ROOT                     16
#define DISPID_DEPTH                    17
#define DISPID_MODE                     18
#define DISPID_FLAGS                    19
#define DISPID_TVFLAGS                  20
#define DISPID_NSCOLUMNS                21
#define DISPID_COUNTVIEWTYPES           22
#define DISPID_SETVIEWTYPE              23
#define DISPID_SELECTEDITEMS            24
#define DISPID_EXPAND                   25
#define DISPID_UNSELECTALL              26

#define EXDISPID_H_

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EXDISPID_H_
