

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

#ifndef __mmc_h__
#define __mmc_h__

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

#ifndef __IComponentData_FWD_DEFINED__
#define __IComponentData_FWD_DEFINED__
typedef interface IComponentData IComponentData;

#endif 	/* __IComponentData_FWD_DEFINED__ */


#ifndef __IComponent_FWD_DEFINED__
#define __IComponent_FWD_DEFINED__
typedef interface IComponent IComponent;

#endif 	/* __IComponent_FWD_DEFINED__ */


#ifndef __IResultDataCompare_FWD_DEFINED__
#define __IResultDataCompare_FWD_DEFINED__
typedef interface IResultDataCompare IResultDataCompare;

#endif 	/* __IResultDataCompare_FWD_DEFINED__ */


#ifndef __IResultOwnerData_FWD_DEFINED__
#define __IResultOwnerData_FWD_DEFINED__
typedef interface IResultOwnerData IResultOwnerData;

#endif 	/* __IResultOwnerData_FWD_DEFINED__ */


#ifndef __IConsole_FWD_DEFINED__
#define __IConsole_FWD_DEFINED__
typedef interface IConsole IConsole;

#endif 	/* __IConsole_FWD_DEFINED__ */


#ifndef __IHeaderCtrl_FWD_DEFINED__
#define __IHeaderCtrl_FWD_DEFINED__
typedef interface IHeaderCtrl IHeaderCtrl;

#endif 	/* __IHeaderCtrl_FWD_DEFINED__ */


#ifndef __IContextMenuCallback_FWD_DEFINED__
#define __IContextMenuCallback_FWD_DEFINED__
typedef interface IContextMenuCallback IContextMenuCallback;

#endif 	/* __IContextMenuCallback_FWD_DEFINED__ */


#ifndef __IContextMenuProvider_FWD_DEFINED__
#define __IContextMenuProvider_FWD_DEFINED__
typedef interface IContextMenuProvider IContextMenuProvider;

#endif 	/* __IContextMenuProvider_FWD_DEFINED__ */


#ifndef __IExtendContextMenu_FWD_DEFINED__
#define __IExtendContextMenu_FWD_DEFINED__
typedef interface IExtendContextMenu IExtendContextMenu;

#endif 	/* __IExtendContextMenu_FWD_DEFINED__ */


#ifndef __IImageList_FWD_DEFINED__
#define __IImageList_FWD_DEFINED__
typedef interface IImageList IImageList;

#endif 	/* __IImageList_FWD_DEFINED__ */


#ifndef __IResultData_FWD_DEFINED__
#define __IResultData_FWD_DEFINED__
typedef interface IResultData IResultData;

#endif 	/* __IResultData_FWD_DEFINED__ */


#ifndef __IConsoleNameSpace_FWD_DEFINED__
#define __IConsoleNameSpace_FWD_DEFINED__
typedef interface IConsoleNameSpace IConsoleNameSpace;

#endif 	/* __IConsoleNameSpace_FWD_DEFINED__ */


#ifndef __IConsoleNameSpace2_FWD_DEFINED__
#define __IConsoleNameSpace2_FWD_DEFINED__
typedef interface IConsoleNameSpace2 IConsoleNameSpace2;

#endif 	/* __IConsoleNameSpace2_FWD_DEFINED__ */


#ifndef __IPropertySheetCallback_FWD_DEFINED__
#define __IPropertySheetCallback_FWD_DEFINED__
typedef interface IPropertySheetCallback IPropertySheetCallback;

#endif 	/* __IPropertySheetCallback_FWD_DEFINED__ */


#ifndef __IPropertySheetProvider_FWD_DEFINED__
#define __IPropertySheetProvider_FWD_DEFINED__
typedef interface IPropertySheetProvider IPropertySheetProvider;

#endif 	/* __IPropertySheetProvider_FWD_DEFINED__ */


#ifndef __IExtendPropertySheet_FWD_DEFINED__
#define __IExtendPropertySheet_FWD_DEFINED__
typedef interface IExtendPropertySheet IExtendPropertySheet;

#endif 	/* __IExtendPropertySheet_FWD_DEFINED__ */


#ifndef __IControlbar_FWD_DEFINED__
#define __IControlbar_FWD_DEFINED__
typedef interface IControlbar IControlbar;

#endif 	/* __IControlbar_FWD_DEFINED__ */


#ifndef __IExtendControlbar_FWD_DEFINED__
#define __IExtendControlbar_FWD_DEFINED__
typedef interface IExtendControlbar IExtendControlbar;

#endif 	/* __IExtendControlbar_FWD_DEFINED__ */


#ifndef __IToolbar_FWD_DEFINED__
#define __IToolbar_FWD_DEFINED__
typedef interface IToolbar IToolbar;

#endif 	/* __IToolbar_FWD_DEFINED__ */


#ifndef __IConsoleVerb_FWD_DEFINED__
#define __IConsoleVerb_FWD_DEFINED__
typedef interface IConsoleVerb IConsoleVerb;

#endif 	/* __IConsoleVerb_FWD_DEFINED__ */


#ifndef __ISnapinAbout_FWD_DEFINED__
#define __ISnapinAbout_FWD_DEFINED__
typedef interface ISnapinAbout ISnapinAbout;

#endif 	/* __ISnapinAbout_FWD_DEFINED__ */


#ifndef __IMenuButton_FWD_DEFINED__
#define __IMenuButton_FWD_DEFINED__
typedef interface IMenuButton IMenuButton;

#endif 	/* __IMenuButton_FWD_DEFINED__ */


#ifndef __ISnapinHelp_FWD_DEFINED__
#define __ISnapinHelp_FWD_DEFINED__
typedef interface ISnapinHelp ISnapinHelp;

#endif 	/* __ISnapinHelp_FWD_DEFINED__ */


#ifndef __IExtendPropertySheet2_FWD_DEFINED__
#define __IExtendPropertySheet2_FWD_DEFINED__
typedef interface IExtendPropertySheet2 IExtendPropertySheet2;

#endif 	/* __IExtendPropertySheet2_FWD_DEFINED__ */


#ifndef __IHeaderCtrl2_FWD_DEFINED__
#define __IHeaderCtrl2_FWD_DEFINED__
typedef interface IHeaderCtrl2 IHeaderCtrl2;

#endif 	/* __IHeaderCtrl2_FWD_DEFINED__ */


#ifndef __ISnapinHelp2_FWD_DEFINED__
#define __ISnapinHelp2_FWD_DEFINED__
typedef interface ISnapinHelp2 ISnapinHelp2;

#endif 	/* __ISnapinHelp2_FWD_DEFINED__ */


#ifndef __IEnumTASK_FWD_DEFINED__
#define __IEnumTASK_FWD_DEFINED__
typedef interface IEnumTASK IEnumTASK;

#endif 	/* __IEnumTASK_FWD_DEFINED__ */


#ifndef __IExtendTaskPad_FWD_DEFINED__
#define __IExtendTaskPad_FWD_DEFINED__
typedef interface IExtendTaskPad IExtendTaskPad;

#endif 	/* __IExtendTaskPad_FWD_DEFINED__ */


#ifndef __IConsole2_FWD_DEFINED__
#define __IConsole2_FWD_DEFINED__
typedef interface IConsole2 IConsole2;

#endif 	/* __IConsole2_FWD_DEFINED__ */


#ifndef __IDisplayHelp_FWD_DEFINED__
#define __IDisplayHelp_FWD_DEFINED__
typedef interface IDisplayHelp IDisplayHelp;

#endif 	/* __IDisplayHelp_FWD_DEFINED__ */


#ifndef __IRequiredExtensions_FWD_DEFINED__
#define __IRequiredExtensions_FWD_DEFINED__
typedef interface IRequiredExtensions IRequiredExtensions;

#endif 	/* __IRequiredExtensions_FWD_DEFINED__ */


#ifndef __IStringTable_FWD_DEFINED__
#define __IStringTable_FWD_DEFINED__
typedef interface IStringTable IStringTable;

#endif 	/* __IStringTable_FWD_DEFINED__ */


#ifndef __IColumnData_FWD_DEFINED__
#define __IColumnData_FWD_DEFINED__
typedef interface IColumnData IColumnData;

#endif 	/* __IColumnData_FWD_DEFINED__ */


#ifndef __IMessageView_FWD_DEFINED__
#define __IMessageView_FWD_DEFINED__
typedef interface IMessageView IMessageView;

#endif 	/* __IMessageView_FWD_DEFINED__ */


#ifndef __IResultDataCompareEx_FWD_DEFINED__
#define __IResultDataCompareEx_FWD_DEFINED__
typedef interface IResultDataCompareEx IResultDataCompareEx;

#endif 	/* __IResultDataCompareEx_FWD_DEFINED__ */


#ifndef __IComponentData2_FWD_DEFINED__
#define __IComponentData2_FWD_DEFINED__
typedef interface IComponentData2 IComponentData2;

#endif 	/* __IComponentData2_FWD_DEFINED__ */


#ifndef __IComponent2_FWD_DEFINED__
#define __IComponent2_FWD_DEFINED__
typedef interface IComponent2 IComponent2;

#endif 	/* __IComponent2_FWD_DEFINED__ */


#ifndef __IContextMenuCallback2_FWD_DEFINED__
#define __IContextMenuCallback2_FWD_DEFINED__
typedef interface IContextMenuCallback2 IContextMenuCallback2;

#endif 	/* __IContextMenuCallback2_FWD_DEFINED__ */


#ifndef __IMMCVersionInfo_FWD_DEFINED__
#define __IMMCVersionInfo_FWD_DEFINED__
typedef interface IMMCVersionInfo IMMCVersionInfo;

#endif 	/* __IMMCVersionInfo_FWD_DEFINED__ */


#ifndef __MMCVersionInfo_FWD_DEFINED__
#define __MMCVersionInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class MMCVersionInfo MMCVersionInfo;
#else
typedef struct MMCVersionInfo MMCVersionInfo;
#endif /* __cplusplus */

#endif 	/* __MMCVersionInfo_FWD_DEFINED__ */


#ifndef __ConsolePower_FWD_DEFINED__
#define __ConsolePower_FWD_DEFINED__

#ifdef __cplusplus
typedef class ConsolePower ConsolePower;
#else
typedef struct ConsolePower ConsolePower;
#endif /* __cplusplus */

#endif 	/* __ConsolePower_FWD_DEFINED__ */


#ifndef __IExtendView_FWD_DEFINED__
#define __IExtendView_FWD_DEFINED__
typedef interface IExtendView IExtendView;

#endif 	/* __IExtendView_FWD_DEFINED__ */


#ifndef __IViewExtensionCallback_FWD_DEFINED__
#define __IViewExtensionCallback_FWD_DEFINED__
typedef interface IViewExtensionCallback IViewExtensionCallback;

#endif 	/* __IViewExtensionCallback_FWD_DEFINED__ */


#ifndef __IConsolePower_FWD_DEFINED__
#define __IConsolePower_FWD_DEFINED__
typedef interface IConsolePower IConsolePower;

#endif 	/* __IConsolePower_FWD_DEFINED__ */


#ifndef __IConsolePowerSink_FWD_DEFINED__
#define __IConsolePowerSink_FWD_DEFINED__
typedef interface IConsolePowerSink IConsolePowerSink;

#endif 	/* __IConsolePowerSink_FWD_DEFINED__ */


#ifndef __INodeProperties_FWD_DEFINED__
#define __INodeProperties_FWD_DEFINED__
typedef interface INodeProperties INodeProperties;

#endif 	/* __INodeProperties_FWD_DEFINED__ */


#ifndef __IConsole3_FWD_DEFINED__
#define __IConsole3_FWD_DEFINED__
typedef interface IConsole3 IConsole3;

#endif 	/* __IConsole3_FWD_DEFINED__ */


#ifndef __IResultData2_FWD_DEFINED__
#define __IResultData2_FWD_DEFINED__
typedef interface IResultData2 IResultData2;

#endif 	/* __IResultData2_FWD_DEFINED__ */


/* header files for imported files */
#include "basetsd.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mmc_0000_0000 */
/* [local] */ 

#ifndef MMC_VER
#define MMC_VER 0x0210
#endif
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)













#if (MMC_VER >= 0x0110)





#endif // MMC_VER >= 0x0110
#if (MMC_VER >= 0x0120)

#endif // MMC_VER >= 0x0120
#if (MMC_VER >= 0x0200)




#endif // MMC_VER >= 0x0200









#if (MMC_VER >= 0x0110)





#endif // MMC_VER >= 0x0110
#if (MMC_VER >= 0x0120)

#endif // MMC_VER >= 0x0120
#if (MMC_VER >= 0x0200)








#endif // MMC_VER >= 0x0200
typedef IConsole *LPCONSOLE;

typedef IHeaderCtrl *LPHEADERCTRL;

typedef IToolbar *LPTOOLBAR;

typedef IImageList *LPIMAGELIST;

typedef IResultData *LPRESULTDATA;

typedef IConsoleNameSpace *LPCONSOLENAMESPACE;

typedef IPropertySheetProvider *LPPROPERTYSHEETPROVIDER;

typedef IPropertySheetCallback *LPPROPERTYSHEETCALLBACK;

typedef IContextMenuProvider *LPCONTEXTMENUPROVIDER;

typedef IContextMenuCallback *LPCONTEXTMENUCALLBACK;

typedef IControlbar *LPCONTROLBAR;

typedef IConsoleVerb *LPCONSOLEVERB;

typedef IMenuButton *LPMENUBUTTON;

#if (MMC_VER >= 0x0110)
typedef IConsole2 *LPCONSOLE2;

typedef IHeaderCtrl2 *LPHEADERCTRL2;

typedef IConsoleNameSpace2 *LPCONSOLENAMESPACE2;

typedef IDisplayHelp *LPDISPLAYHELP;

typedef IStringTable *LPSTRINGTABLE;

#endif // MMC_VER >= 0x0110
#if (MMC_VER >= 0x0120)
typedef IColumnData *LPCOLUMNDATA;

typedef IResultDataCompareEx *LPRESULTDATACOMPAREEX;

#endif // MMC_VER >= 0x0120
typedef IComponent *LPCOMPONENT;

typedef IComponentData *LPCOMPONENTDATA;

typedef IExtendPropertySheet *LPEXTENDPROPERTYSHEET;

typedef IExtendContextMenu *LPEXTENDCONTEXTMENU;

typedef IExtendControlbar *LPEXTENDCONTROLBAR;

typedef IResultDataCompare *LPRESULTDATACOMPARE;

typedef IResultOwnerData *LPRESULTOWNERDATA;

typedef ISnapinAbout *LPSNAPABOUT;

typedef ISnapinAbout *LPSNAPINABOUT;

typedef ISnapinHelp *LPSNAPHELP;

typedef ISnapinHelp *LPSNAPINHELP;

#if (MMC_VER >= 0x0110)
typedef IEnumTASK *LPENUMTASK;

typedef IExtendPropertySheet2 *LPEXTENDPROPERTYSHEET2;

typedef ISnapinHelp2 *LPSNAPINHELP2;

typedef IExtendTaskPad *LPEXTENDTASKPAD;

typedef IRequiredExtensions *LPREQUIREDEXTENSIONS;

#endif // MMC_VER >= 0x0110
#if (MMC_VER >= 0x0200)
typedef IComponent2 *LPCOMPONENT2;

typedef IComponentData2 *LPCOMPONENTDATA2;

typedef IExtendView *LPEXTENDVIEW;

typedef IViewExtensionCallback *LPVIEWEXTENSIONCALLBACK;

typedef IConsolePower *LPCONSOLEPOWER;

typedef IConsolePowerSink *LPCONSOLEPOWERSINK;

typedef IConsole3 *LPCONSOLE3;

typedef INodeProperties *LPNODEPROPERTIES;

typedef IResultData2 *LPRESULTDATA2;

#endif // MMC_VER >= 0x0200
typedef BSTR *PBSTR;

#define	MMCLV_AUTO	( -1 )

#define	MMCLV_NOPARAM	( -2 )

#define	MMCLV_NOICON	( -1 )

#define	MMCLV_VIEWSTYLE_ICON	( 0 )

#define	MMCLV_VIEWSTYLE_SMALLICON	( 0x2 )

#define	MMCLV_VIEWSTYLE_LIST	( 0x3 )

#define	MMCLV_VIEWSTYLE_REPORT	( 0x1 )

#define	MMCLV_VIEWSTYLE_FILTERED	( 0x4 )

#define	MMCLV_NOPTR	( 0 )

#define	MMCLV_UPDATE_NOINVALIDATEALL	( 0x1 )

#define	MMCLV_UPDATE_NOSCROLL	( 0x2 )

static LPOLESTR MMC_CALLBACK	=	( LPOLESTR  )-1;

#if (MMC_VER >= 0x0120)
#define MMC_IMAGECALLBACK (-1)
#define MMC_TEXTCALLBACK  MMC_CALLBACK
#endif // MMC_VER >= 0x0120
typedef LONG_PTR HSCOPEITEM;

typedef long COMPONENTID;

typedef LONG_PTR HRESULTITEM;

#define	RDI_STR	( 0x2 )

#define	RDI_IMAGE	( 0x4 )

#define	RDI_STATE	( 0x8 )

#define	RDI_PARAM	( 0x10 )

#define	RDI_INDEX	( 0x20 )

#define	RDI_INDENT	( 0x40 )

typedef enum _MMC_RESULT_VIEW_STYLE      
{                                        
    MMC_SINGLESEL           = 0x0001,    
    MMC_SHOWSELALWAYS       = 0x0002,    
    MMC_NOSORTHEADER        = 0x0004,    
#if (MMC_VER >= 0x0120)                  
    MMC_ENSUREFOCUSVISIBLE  = 0x0008     
#endif // MMC_VER >= 0x0120              
} MMC_RESULT_VIEW_STYLE;                 
#if 0
typedef 
enum _MMC_RESULT_VIEW_STYLE
    {
        _MMC_VIEW_STYLE__dummy_	= 0
    } 	MMC_RESULT_VIEW_STYLE;

#endif
#define	MMC_VIEW_OPTIONS_NONE	( 0 )

#define	MMC_VIEW_OPTIONS_NOLISTVIEWS	( 0x1 )

#define	MMC_VIEW_OPTIONS_MULTISELECT	( 0x2 )

#define	MMC_VIEW_OPTIONS_OWNERDATALIST	( 0x4 )

#define	MMC_VIEW_OPTIONS_FILTERED	( 0x8 )

#define	MMC_VIEW_OPTIONS_CREATENEW	( 0x10 )

#if (MMC_VER >= 0x0110)
#define	MMC_VIEW_OPTIONS_USEFONTLINKING	( 0x20 )

#endif // MMC_VER >= 0x0110
#if (MMC_VER >= 0x0120)
#define	MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST	( 0x40 )

#define	MMC_VIEW_OPTIONS_LEXICAL_SORT	( 0x80 )

#endif // MMC_VER >= 0x0120
#define	MMC_PSO_NOAPPLYNOW	( 0x1 )

#define	MMC_PSO_HASHELP	( 0x2 )

#define	MMC_PSO_NEWWIZARDTYPE	( 0x4 )

#define	MMC_PSO_NO_PROPTITLE	( 0x8 )

typedef 
enum _MMC_CONTROL_TYPE
    {
        TOOLBAR	= 0,
        MENUBUTTON	= ( TOOLBAR + 1 ) ,
        COMBOBOXBAR	= ( MENUBUTTON + 1 ) 
    } 	MMC_CONTROL_TYPE;

typedef enum _MMC_CONSOLE_VERB                               
{                                                            
    MMC_VERB_NONE            = 0x0000,                       
    MMC_VERB_OPEN            = 0x8000,                       
    MMC_VERB_COPY            = 0x8001,                       
    MMC_VERB_PASTE           = 0x8002,                       
    MMC_VERB_DELETE          = 0x8003,                       
    MMC_VERB_PROPERTIES      = 0x8004,                       
    MMC_VERB_RENAME          = 0x8005,                       
    MMC_VERB_REFRESH         = 0x8006,                       
    MMC_VERB_PRINT           = 0x8007,                       
#if (MMC_VER >= 0x0110)                                      
    MMC_VERB_CUT             = 0x8008,  // Used only to explicitly disable/hide
                                        // the cut verb, when copy & paste are enabled.
                                                             
    // must be last                                          
    MMC_VERB_MAX,                                            
    MMC_VERB_FIRST           = MMC_VERB_OPEN,                
    MMC_VERB_LAST            = MMC_VERB_MAX - 1              
#endif // MMC_VER >= 0x0110                                  
} MMC_CONSOLE_VERB;                                          
#if 0
typedef 
enum _MMC_CONSOLE_VERB
    {
        MMC_VERB__dummy_	= 0
    } 	MMC_CONSOLE_VERB;

#endif
#include <pshpack8.h>
typedef struct _MMCButton
    {
    int nBitmap;
    int idCommand;
    BYTE fsState;
    BYTE fsType;
    LPOLESTR lpButtonText;
    LPOLESTR lpTooltipText;
    } 	MMCBUTTON;

#include <poppack.h>
typedef MMCBUTTON *LPMMCBUTTON;

typedef 
enum _MMC_BUTTON_STATE
    {
        ENABLED	= 0x1,
        CHECKED	= 0x2,
        HIDDEN	= 0x4,
        INDETERMINATE	= 0x8,
        BUTTONPRESSED	= 0x10
    } 	MMC_BUTTON_STATE;

typedef struct _RESULTDATAITEM
    {
    DWORD mask;
    BOOL bScopeItem;
    HRESULTITEM itemID;
    int nIndex;
    int nCol;
    LPOLESTR str;
    int nImage;
    UINT nState;
    LPARAM lParam;
    int iIndent;
    } 	RESULTDATAITEM;

typedef RESULTDATAITEM *LPRESULTDATAITEM;

#define	RFI_PARTIAL	( 0x1 )

#define	RFI_WRAP	( 0x2 )

typedef struct _RESULTFINDINFO
    {
    LPOLESTR psz;
    int nStart;
    DWORD dwOptions;
    } 	RESULTFINDINFO;

typedef RESULTFINDINFO *LPRESULTFINDINFO;

#define	RSI_DESCENDING	( 0x1 )

#define	RSI_NOSORTICON	( 0x2 )

#define	SDI_STR	( 0x2 )

#define	SDI_IMAGE	( 0x4 )

#define	SDI_OPENIMAGE	( 0x8 )

#define	SDI_STATE	( 0x10 )

#define	SDI_PARAM	( 0x20 )

#define	SDI_CHILDREN	( 0x40 )

#define	SDI_PARENT	( 0 )

#define	SDI_PREVIOUS	( 0x10000000 )

#define	SDI_NEXT	( 0x20000000 )

#define	SDI_FIRST	( 0x8000000 )

typedef struct _SCOPEDATAITEM
    {
    DWORD mask;
    LPOLESTR displayname;
    int nImage;
    int nOpenImage;
    UINT nState;
    int cChildren;
    LPARAM lParam;
    HSCOPEITEM relativeID;
    HSCOPEITEM ID;
    } 	SCOPEDATAITEM;

typedef SCOPEDATAITEM *LPSCOPEDATAITEM;

typedef 
enum _MMC_SCOPE_ITEM_STATE
    {
        MMC_SCOPE_ITEM_STATE_NORMAL	= 0x1,
        MMC_SCOPE_ITEM_STATE_BOLD	= 0x2,
        MMC_SCOPE_ITEM_STATE_EXPANDEDONCE	= 0x3
    } 	MMC_SCOPE_ITEM_STATE;

typedef struct _CONTEXTMENUITEM
    {
    LPWSTR strName;
    LPWSTR strStatusBarText;
    LONG lCommandID;
    LONG lInsertionPointID;
    LONG fFlags;
    LONG fSpecialFlags;
    } 	CONTEXTMENUITEM;

typedef CONTEXTMENUITEM *LPCONTEXTMENUITEM;

typedef 
enum _MMC_MENU_COMMAND_IDS
    {
        MMCC_STANDARD_VIEW_SELECT	= -1
    } 	MMC_MENU_COMMAND_IDS;

typedef struct _MENUBUTTONDATA
    {
    int idCommand;
    int x;
    int y;
    } 	MENUBUTTONDATA;

typedef MENUBUTTONDATA *LPMENUBUTTONDATA;

typedef LONG_PTR MMC_COOKIE;

#define	MMC_MULTI_SELECT_COOKIE	( -2 )

#define	MMC_WINDOW_COOKIE	( -3 )

#if (MMC_VER >= 0x0110)
#define	SPECIAL_COOKIE_MIN	( -10 )

#define	SPECIAL_COOKIE_MAX	( -1 )

typedef 
enum _MMC_FILTER_TYPE
    {
        MMC_STRING_FILTER	= 0,
        MMC_INT_FILTER	= 0x1,
        MMC_FILTER_NOVALUE	= 0x8000
    } 	MMC_FILTER_TYPE;

typedef struct _MMC_FILTERDATA
    {
    LPOLESTR pszText;
    INT cchTextMax;
    LONG lValue;
    } 	MMC_FILTERDATA;

typedef 
enum _MMC_FILTER_CHANGE_CODE
    {
        MFCC_DISABLE	= 0,
        MFCC_ENABLE	= 1,
        MFCC_VALUE_CHANGE	= 2
    } 	MMC_FILTER_CHANGE_CODE;

typedef struct _MMC_RESTORE_VIEW
    {
    DWORD dwSize;
    MMC_COOKIE cookie;
    LPOLESTR pViewType;
    long lViewOptions;
    } 	MMC_RESTORE_VIEW;

typedef struct _MMC_EXPANDSYNC_STRUCT
    {
    BOOL bHandled;
    BOOL bExpanding;
    HSCOPEITEM hItem;
    } 	MMC_EXPANDSYNC_STRUCT;

#endif // MMC_VER >= 0x0110
#if (MMC_VER >= 0x0120)
typedef struct _MMC_VISIBLE_COLUMNS
    {
    INT nVisibleColumns;
    INT rgVisibleCols[ 1 ];
    } 	MMC_VISIBLE_COLUMNS;

#endif // MMC_VER >= 0x0120
typedef enum _MMC_NOTIFY_TYPE                                
{                                                            
    MMCN_ACTIVATE           = 0x8001,                        
    MMCN_ADD_IMAGES         = 0x8002,                        
    MMCN_BTN_CLICK          = 0x8003,                        
    MMCN_CLICK              = 0x8004,   // NOT USED          
    MMCN_COLUMN_CLICK       = 0x8005,                        
    MMCN_CONTEXTMENU        = 0x8006,   // NOT USED          
    MMCN_CUTORMOVE          = 0x8007,                        
    MMCN_DBLCLICK           = 0x8008,                        
    MMCN_DELETE             = 0x8009,                        
    MMCN_DESELECT_ALL       = 0x800A,                        
    MMCN_EXPAND             = 0x800B,                        
    MMCN_HELP               = 0x800C,   // NOT USED          
    MMCN_MENU_BTNCLICK      = 0x800D,                        
    MMCN_MINIMIZED          = 0x800E,                        
    MMCN_PASTE              = 0x800F,                        
    MMCN_PROPERTY_CHANGE    = 0x8010,                        
    MMCN_QUERY_PASTE        = 0x8011,                        
    MMCN_REFRESH            = 0x8012,                        
    MMCN_REMOVE_CHILDREN    = 0x8013,                        
    MMCN_RENAME             = 0x8014,                        
    MMCN_SELECT             = 0x8015,                        
    MMCN_SHOW               = 0x8016,                        
    MMCN_VIEW_CHANGE        = 0x8017,                        
    MMCN_SNAPINHELP         = 0x8018,                        
    MMCN_CONTEXTHELP        = 0x8019,                        
    MMCN_INITOCX            = 0x801A,                        
#if (MMC_VER >= 0x0110)                                      
    MMCN_FILTER_CHANGE      = 0x801B,                        
    MMCN_FILTERBTN_CLICK    = 0x801C,                        
    MMCN_RESTORE_VIEW       = 0x801D,                        
    MMCN_PRINT              = 0x801E,                        
    MMCN_PRELOAD            = 0x801F,                        
    MMCN_LISTPAD            = 0x8020,                        
    MMCN_EXPANDSYNC         = 0x8021,                        
#if (MMC_VER >= 0x0120)                                      
    MMCN_COLUMNS_CHANGED    = 0x8022,                        
#if (MMC_VER >= 0x0200)                                      
    MMCN_CANPASTE_OUTOFPROC = 0x8023,                        
#endif // MMC_VER >= 0x0200                                  
#endif // MMC_VER >= 0x0120                                  
#endif // MMC_VER >= 0x0110                                  
} MMC_NOTIFY_TYPE;                                           
#if 0
typedef 
enum _MMC_NOTIFY_TYPE
    {
        MMCN__dummy_	= 0
    } 	MMC_NOTIFY_TYPE;

#endif
typedef 
enum _DATA_OBJECT_TYPES
    {
        CCT_SCOPE	= 0x8000,
        CCT_RESULT	= 0x8001,
        CCT_SNAPIN_MANAGER	= 0x8002,
        CCT_UNINITIALIZED	= 0xffff
    } 	DATA_OBJECT_TYPES;

#define	MMC_NW_OPTION_NONE	( 0 )

#define	MMC_NW_OPTION_NOSCOPEPANE	( 0x1 )

#define	MMC_NW_OPTION_NOTOOLBARS	( 0x2 )

#define	MMC_NW_OPTION_SHORTTITLE	( 0x4 )

#define	MMC_NW_OPTION_CUSTOMTITLE	( 0x8 )

#define	MMC_NW_OPTION_NOPERSIST	( 0x10 )

#define	MMC_NW_OPTION_NOACTIONPANE	( 0x20 )

#define	CCF_NODETYPE	( L"CCF_NODETYPE" )

#define	CCF_SZNODETYPE	( L"CCF_SZNODETYPE" )

#define	CCF_DISPLAY_NAME	( L"CCF_DISPLAY_NAME" )

#define	CCF_SNAPIN_CLASSID	( L"CCF_SNAPIN_CLASSID" )

#if (MMC_VER >= 0x0210)
#define	CCF_SNAPIN_CLASS	( L"CCF_SNAPIN_CLASS" )

#endif // MMC_VER >= 0x0210
#define	CCF_WINDOW_TITLE	( L"CCF_WINDOW_TITLE" )

#define	CCF_MMC_MULTISELECT_DATAOBJECT	( L"CCF_MMC_MULTISELECT_DATAOBJECT" )

typedef struct _SMMCDataObjects
    {
    DWORD count;
    LPDATAOBJECT lpDataObject[ 1 ];
    } 	SMMCDataObjects;

#define	CCF_MULTI_SELECT_SNAPINS	( L"CCF_MULTI_SELECT_SNAPINS" )

typedef struct _SMMCObjectTypes
    {
    DWORD count;
    GUID guid[ 1 ];
    } 	SMMCObjectTypes;

#define	CCF_OBJECT_TYPES_IN_MULTI_SELECT	( L"CCF_OBJECT_TYPES_IN_MULTI_SELECT" )

#if (MMC_VER >= 0x0110)
typedef SMMCObjectTypes SMMCDynamicExtensions;

#define	CCF_MMC_DYNAMIC_EXTENSIONS	( L"CCF_MMC_DYNAMIC_EXTENSIONS" )

#define	CCF_SNAPIN_PRELOADS	( L"CCF_SNAPIN_PRELOADS" )

typedef struct _SNodeID
    {
    DWORD cBytes;
    BYTE id[ 1 ];
    } 	SNodeID;

#if (MMC_VER >= 0x0120)
typedef struct _SNodeID2
    {
    DWORD dwFlags;
    DWORD cBytes;
    BYTE id[ 1 ];
    } 	SNodeID2;

#define	MMC_NODEID_SLOW_RETRIEVAL	( 0x1 )

#define	CCF_NODEID2	( L"CCF_NODEID2" )

#endif // MMC_VER >= 0x0120
#define	CCF_NODEID	( L"CCF_NODEID" )

#if (MMC_VER >= 0x0120)
typedef struct _SColumnSetID
    {
    DWORD dwFlags;
    DWORD cBytes;
    BYTE id[ 1 ];
    } 	SColumnSetID;

#define	CCF_COLUMN_SET_ID	( L"CCF_COLUMN_SET_ID" )

#endif // MMC_VER >= 0x0120
#endif // MMC_VER >= 0x0110
STDAPI MMCPropertyChangeNotify(LONG_PTR lNotifyHandle, LPARAM param);
#if (MMC_VER >= 0x0110)
#include <specstrings.h>
STDAPI MMCPropertyHelp(_In_ LPOLESTR pszHelpTopic);
#endif // MMC_VER >= 0x0110
STDAPI MMCFreeNotifyHandle(LONG_PTR lNotifyHandle);
// MMCPropPageCallback is only available in mmc.lib. See MMCAfxPropPageCallback if linking against mmcutil.lib.
STDAPI MMCPropPageCallback(void* vpsp);
EXTERN_C const CLSID CLSID_NodeManager;
#if (MMC_VER >= 0x0120)
EXTERN_C const CLSID CLSID_MessageView;
#endif // MMC_VER >= 0x0120
#if (MMC_VER >= 0x0210)
// Snap-in implemented callback responsible for calling AFX_MANAGE_STATE() for the snapin-in's module.
typedef void (STDAPICALLTYPE *PFNAfxManageStateCallback)();
// Sets up a PROPSHEETPAGE structure for calling the snap-in's PFNAfxManageStateCallback.
// MMCAfxPropPageCallback is available in both mmc.lib and mmcutil.lib.
// Returns:
// S_OK - success
// E_POINTER - vpsp or pfnCallback is null
// E_INVALIDARG - vpsp->pfnCallback must be the same for all property pages
// E_INVALIDARG - pfnManageStateCallback must be the same for all property pages
// E_UNEXPECTED - vpsp has already been passed to MMCAfxPropPageCallback
STDAPI MMCAfxPropPageCallback(void* vpsp, PFNAfxManageStateCallback pfnManageStateCallback);
#endif // MMC_VER >= 0x0210
#define DOBJ_NULL        (LPDATAOBJECT)   0
#define DOBJ_CUSTOMOCX   (LPDATAOBJECT)  -1
#define DOBJ_CUSTOMWEB   (LPDATAOBJECT)  -2
#if (MMC_VER >= 0x0110)
#if (MMC_VER >= 0x0120)
#define DOBJ_NOCONSOLE   (LPDATAOBJECT)  -3
#endif // MMC_VER >= 0x0120
#define SPECIAL_DOBJ_MIN                -10
#define SPECIAL_DOBJ_MAX                  0
#endif // MMC_VER >= 0x0110
#define IS_SPECIAL_DATAOBJECT(d) (((LONG_PTR)(d) >= SPECIAL_DOBJ_MIN)   && ((LONG_PTR)(d) <= SPECIAL_DOBJ_MAX))
#define IS_SPECIAL_COOKIE(c)     (((c)          >= SPECIAL_COOKIE_MIN) && ((c)          <= SPECIAL_COOKIE_MAX))


extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0000_v0_0_s_ifspec;

#ifndef __IComponentData_INTERFACE_DEFINED__
#define __IComponentData_INTERFACE_DEFINED__

/* interface IComponentData */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IComponentData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("955AB28A-5218-11D0-A985-00C04FD8D565")
    IComponentData : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt LPUNKNOWN pUnknown) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateComponent( 
            /* [out] */ __RPC__deref_out_opt LPCOMPONENT *ppComponent) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Destroy( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryDataObject( 
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDATAOBJECT *ppDataObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDisplayInfo( 
            /* [out][in] */ __RPC__inout SCOPEDATAITEM *pScopeDataItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CompareObjects( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectA,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectB) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponentData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponentData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponentData * This);
        
        DECLSPEC_XFGVIRT(IComponentData, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IComponentData * This,
            /* [in] */ __RPC__in_opt LPUNKNOWN pUnknown);
        
        DECLSPEC_XFGVIRT(IComponentData, CreateComponent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateComponent )( 
            __RPC__in IComponentData * This,
            /* [out] */ __RPC__deref_out_opt LPCOMPONENT *ppComponent);
        
        DECLSPEC_XFGVIRT(IComponentData, Notify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IComponentData * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param);
        
        DECLSPEC_XFGVIRT(IComponentData, Destroy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            __RPC__in IComponentData * This);
        
        DECLSPEC_XFGVIRT(IComponentData, QueryDataObject)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDataObject )( 
            __RPC__in IComponentData * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDATAOBJECT *ppDataObject);
        
        DECLSPEC_XFGVIRT(IComponentData, GetDisplayInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDisplayInfo )( 
            __RPC__in IComponentData * This,
            /* [out][in] */ __RPC__inout SCOPEDATAITEM *pScopeDataItem);
        
        DECLSPEC_XFGVIRT(IComponentData, CompareObjects)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CompareObjects )( 
            __RPC__in IComponentData * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectA,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectB);
        
        END_INTERFACE
    } IComponentDataVtbl;

    interface IComponentData
    {
        CONST_VTBL struct IComponentDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponentData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponentData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponentData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponentData_Initialize(This,pUnknown)	\
    ( (This)->lpVtbl -> Initialize(This,pUnknown) ) 

#define IComponentData_CreateComponent(This,ppComponent)	\
    ( (This)->lpVtbl -> CreateComponent(This,ppComponent) ) 

#define IComponentData_Notify(This,lpDataObject,event,arg,param)	\
    ( (This)->lpVtbl -> Notify(This,lpDataObject,event,arg,param) ) 

#define IComponentData_Destroy(This)	\
    ( (This)->lpVtbl -> Destroy(This) ) 

#define IComponentData_QueryDataObject(This,cookie,type,ppDataObject)	\
    ( (This)->lpVtbl -> QueryDataObject(This,cookie,type,ppDataObject) ) 

#define IComponentData_GetDisplayInfo(This,pScopeDataItem)	\
    ( (This)->lpVtbl -> GetDisplayInfo(This,pScopeDataItem) ) 

#define IComponentData_CompareObjects(This,lpDataObjectA,lpDataObjectB)	\
    ( (This)->lpVtbl -> CompareObjects(This,lpDataObjectA,lpDataObjectB) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponentData_INTERFACE_DEFINED__ */


#ifndef __IComponent_INTERFACE_DEFINED__
#define __IComponent_INTERFACE_DEFINED__

/* interface IComponent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IComponent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43136EB2-D36C-11CF-ADBC-00AA00A80033")
    IComponent : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt LPCONSOLE lpConsole) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Destroy( 
            /* [in] */ MMC_COOKIE cookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryDataObject( 
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDATAOBJECT *ppDataObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetResultViewType( 
            /* [in] */ MMC_COOKIE cookie,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *ppViewType,
            /* [out] */ __RPC__out long *pViewOptions) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDisplayInfo( 
            /* [out][in] */ __RPC__inout RESULTDATAITEM *pResultDataItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CompareObjects( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectA,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectB) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponent * This);
        
        DECLSPEC_XFGVIRT(IComponent, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in_opt LPCONSOLE lpConsole);
        
        DECLSPEC_XFGVIRT(IComponent, Notify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param);
        
        DECLSPEC_XFGVIRT(IComponent, Destroy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            __RPC__in IComponent * This,
            /* [in] */ MMC_COOKIE cookie);
        
        DECLSPEC_XFGVIRT(IComponent, QueryDataObject)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDataObject )( 
            __RPC__in IComponent * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDATAOBJECT *ppDataObject);
        
        DECLSPEC_XFGVIRT(IComponent, GetResultViewType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetResultViewType )( 
            __RPC__in IComponent * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *ppViewType,
            /* [out] */ __RPC__out long *pViewOptions);
        
        DECLSPEC_XFGVIRT(IComponent, GetDisplayInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDisplayInfo )( 
            __RPC__in IComponent * This,
            /* [out][in] */ __RPC__inout RESULTDATAITEM *pResultDataItem);
        
        DECLSPEC_XFGVIRT(IComponent, CompareObjects)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CompareObjects )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectA,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectB);
        
        END_INTERFACE
    } IComponentVtbl;

    interface IComponent
    {
        CONST_VTBL struct IComponentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponent_Initialize(This,lpConsole)	\
    ( (This)->lpVtbl -> Initialize(This,lpConsole) ) 

#define IComponent_Notify(This,lpDataObject,event,arg,param)	\
    ( (This)->lpVtbl -> Notify(This,lpDataObject,event,arg,param) ) 

#define IComponent_Destroy(This,cookie)	\
    ( (This)->lpVtbl -> Destroy(This,cookie) ) 

#define IComponent_QueryDataObject(This,cookie,type,ppDataObject)	\
    ( (This)->lpVtbl -> QueryDataObject(This,cookie,type,ppDataObject) ) 

#define IComponent_GetResultViewType(This,cookie,ppViewType,pViewOptions)	\
    ( (This)->lpVtbl -> GetResultViewType(This,cookie,ppViewType,pViewOptions) ) 

#define IComponent_GetDisplayInfo(This,pResultDataItem)	\
    ( (This)->lpVtbl -> GetDisplayInfo(This,pResultDataItem) ) 

#define IComponent_CompareObjects(This,lpDataObjectA,lpDataObjectB)	\
    ( (This)->lpVtbl -> CompareObjects(This,lpDataObjectA,lpDataObjectB) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponent_INTERFACE_DEFINED__ */


#ifndef __IResultDataCompare_INTERFACE_DEFINED__
#define __IResultDataCompare_INTERFACE_DEFINED__

/* interface IResultDataCompare */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IResultDataCompare;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E8315A52-7A1A-11D0-A2D2-00C04FD909DD")
    IResultDataCompare : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Compare( 
            /* [in] */ LPARAM lUserParam,
            /* [in] */ MMC_COOKIE cookieA,
            /* [in] */ MMC_COOKIE cookieB,
            /* [out][in] */ __RPC__inout int *pnResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResultDataCompareVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResultDataCompare * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResultDataCompare * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResultDataCompare * This);
        
        DECLSPEC_XFGVIRT(IResultDataCompare, Compare)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IResultDataCompare * This,
            /* [in] */ LPARAM lUserParam,
            /* [in] */ MMC_COOKIE cookieA,
            /* [in] */ MMC_COOKIE cookieB,
            /* [out][in] */ __RPC__inout int *pnResult);
        
        END_INTERFACE
    } IResultDataCompareVtbl;

    interface IResultDataCompare
    {
        CONST_VTBL struct IResultDataCompareVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResultDataCompare_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResultDataCompare_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResultDataCompare_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResultDataCompare_Compare(This,lUserParam,cookieA,cookieB,pnResult)	\
    ( (This)->lpVtbl -> Compare(This,lUserParam,cookieA,cookieB,pnResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResultDataCompare_INTERFACE_DEFINED__ */


#ifndef __IResultOwnerData_INTERFACE_DEFINED__
#define __IResultOwnerData_INTERFACE_DEFINED__

/* interface IResultOwnerData */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IResultOwnerData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9CB396D8-EA83-11d0-AEF1-00C04FB6DD2C")
    IResultOwnerData : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindItem( 
            /* [in] */ __RPC__in LPRESULTFINDINFO pFindInfo,
            /* [out] */ __RPC__out int *pnFoundIndex) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CacheHint( 
            /* [in] */ int nStartIndex,
            /* [in] */ int nEndIndex) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SortItems( 
            /* [in] */ int nColumn,
            /* [in] */ DWORD dwSortOptions,
            /* [in] */ LPARAM lUserParam) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResultOwnerDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResultOwnerData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResultOwnerData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResultOwnerData * This);
        
        DECLSPEC_XFGVIRT(IResultOwnerData, FindItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindItem )( 
            __RPC__in IResultOwnerData * This,
            /* [in] */ __RPC__in LPRESULTFINDINFO pFindInfo,
            /* [out] */ __RPC__out int *pnFoundIndex);
        
        DECLSPEC_XFGVIRT(IResultOwnerData, CacheHint)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CacheHint )( 
            __RPC__in IResultOwnerData * This,
            /* [in] */ int nStartIndex,
            /* [in] */ int nEndIndex);
        
        DECLSPEC_XFGVIRT(IResultOwnerData, SortItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SortItems )( 
            __RPC__in IResultOwnerData * This,
            /* [in] */ int nColumn,
            /* [in] */ DWORD dwSortOptions,
            /* [in] */ LPARAM lUserParam);
        
        END_INTERFACE
    } IResultOwnerDataVtbl;

    interface IResultOwnerData
    {
        CONST_VTBL struct IResultOwnerDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResultOwnerData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResultOwnerData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResultOwnerData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResultOwnerData_FindItem(This,pFindInfo,pnFoundIndex)	\
    ( (This)->lpVtbl -> FindItem(This,pFindInfo,pnFoundIndex) ) 

#define IResultOwnerData_CacheHint(This,nStartIndex,nEndIndex)	\
    ( (This)->lpVtbl -> CacheHint(This,nStartIndex,nEndIndex) ) 

#define IResultOwnerData_SortItems(This,nColumn,dwSortOptions,lUserParam)	\
    ( (This)->lpVtbl -> SortItems(This,nColumn,dwSortOptions,lUserParam) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResultOwnerData_INTERFACE_DEFINED__ */


#ifndef __IConsole_INTERFACE_DEFINED__
#define __IConsole_INTERFACE_DEFINED__

/* interface IConsole */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConsole;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43136EB1-D36C-11CF-ADBC-00AA00A80033")
    IConsole : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetHeader( 
            /* [in] */ __RPC__in_opt LPHEADERCTRL pHeader) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetToolbar( 
            /* [in] */ __RPC__in_opt LPTOOLBAR pToolbar) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryResultView( 
            /* [out] */ __RPC__deref_out_opt LPUNKNOWN *pUnknown) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryScopeImageList( 
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryResultImageList( 
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UpdateAllViews( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ LPARAM data,
            /* [in] */ LONG_PTR hint) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE MessageBox( 
            /* [in] */ __RPC__in LPCWSTR lpszText,
            /* [in] */ __RPC__in LPCWSTR lpszTitle,
            /* [in] */ UINT fuStyle,
            /* [out] */ __RPC__out int *piRetval) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryConsoleVerb( 
            /* [out] */ __RPC__deref_out_opt LPCONSOLEVERB *ppConsoleVerb) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SelectScopeItem( 
            /* [in] */ HSCOPEITEM hScopeItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMainWindow( 
            /* [out] */ __RPC__deref_out_opt HWND *phwnd) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE NewWindow( 
            /* [in] */ HSCOPEITEM hScopeItem,
            /* [in] */ unsigned long lOptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsoleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsole * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsole * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsole * This);
        
        DECLSPEC_XFGVIRT(IConsole, SetHeader)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetHeader )( 
            __RPC__in IConsole * This,
            /* [in] */ __RPC__in_opt LPHEADERCTRL pHeader);
        
        DECLSPEC_XFGVIRT(IConsole, SetToolbar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetToolbar )( 
            __RPC__in IConsole * This,
            /* [in] */ __RPC__in_opt LPTOOLBAR pToolbar);
        
        DECLSPEC_XFGVIRT(IConsole, QueryResultView)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryResultView )( 
            __RPC__in IConsole * This,
            /* [out] */ __RPC__deref_out_opt LPUNKNOWN *pUnknown);
        
        DECLSPEC_XFGVIRT(IConsole, QueryScopeImageList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryScopeImageList )( 
            __RPC__in IConsole * This,
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList);
        
        DECLSPEC_XFGVIRT(IConsole, QueryResultImageList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryResultImageList )( 
            __RPC__in IConsole * This,
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList);
        
        DECLSPEC_XFGVIRT(IConsole, UpdateAllViews)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateAllViews )( 
            __RPC__in IConsole * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ LPARAM data,
            /* [in] */ LONG_PTR hint);
        
        DECLSPEC_XFGVIRT(IConsole, MessageBox)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MessageBox )( 
            __RPC__in IConsole * This,
            /* [in] */ __RPC__in LPCWSTR lpszText,
            /* [in] */ __RPC__in LPCWSTR lpszTitle,
            /* [in] */ UINT fuStyle,
            /* [out] */ __RPC__out int *piRetval);
        
        DECLSPEC_XFGVIRT(IConsole, QueryConsoleVerb)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryConsoleVerb )( 
            __RPC__in IConsole * This,
            /* [out] */ __RPC__deref_out_opt LPCONSOLEVERB *ppConsoleVerb);
        
        DECLSPEC_XFGVIRT(IConsole, SelectScopeItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectScopeItem )( 
            __RPC__in IConsole * This,
            /* [in] */ HSCOPEITEM hScopeItem);
        
        DECLSPEC_XFGVIRT(IConsole, GetMainWindow)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMainWindow )( 
            __RPC__in IConsole * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IConsole, NewWindow)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NewWindow )( 
            __RPC__in IConsole * This,
            /* [in] */ HSCOPEITEM hScopeItem,
            /* [in] */ unsigned long lOptions);
        
        END_INTERFACE
    } IConsoleVtbl;

    interface IConsole
    {
        CONST_VTBL struct IConsoleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsole_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsole_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsole_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsole_SetHeader(This,pHeader)	\
    ( (This)->lpVtbl -> SetHeader(This,pHeader) ) 

#define IConsole_SetToolbar(This,pToolbar)	\
    ( (This)->lpVtbl -> SetToolbar(This,pToolbar) ) 

#define IConsole_QueryResultView(This,pUnknown)	\
    ( (This)->lpVtbl -> QueryResultView(This,pUnknown) ) 

#define IConsole_QueryScopeImageList(This,ppImageList)	\
    ( (This)->lpVtbl -> QueryScopeImageList(This,ppImageList) ) 

#define IConsole_QueryResultImageList(This,ppImageList)	\
    ( (This)->lpVtbl -> QueryResultImageList(This,ppImageList) ) 

#define IConsole_UpdateAllViews(This,lpDataObject,data,hint)	\
    ( (This)->lpVtbl -> UpdateAllViews(This,lpDataObject,data,hint) ) 

#define IConsole_MessageBox(This,lpszText,lpszTitle,fuStyle,piRetval)	\
    ( (This)->lpVtbl -> MessageBox(This,lpszText,lpszTitle,fuStyle,piRetval) ) 

#define IConsole_QueryConsoleVerb(This,ppConsoleVerb)	\
    ( (This)->lpVtbl -> QueryConsoleVerb(This,ppConsoleVerb) ) 

#define IConsole_SelectScopeItem(This,hScopeItem)	\
    ( (This)->lpVtbl -> SelectScopeItem(This,hScopeItem) ) 

#define IConsole_GetMainWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetMainWindow(This,phwnd) ) 

#define IConsole_NewWindow(This,hScopeItem,lOptions)	\
    ( (This)->lpVtbl -> NewWindow(This,hScopeItem,lOptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsole_INTERFACE_DEFINED__ */


#ifndef __IHeaderCtrl_INTERFACE_DEFINED__
#define __IHeaderCtrl_INTERFACE_DEFINED__

/* interface IHeaderCtrl */
/* [unique][helpstring][uuid][object] */ 

#define	AUTO_WIDTH	( -1 )

#if (MMC_VER >= 0x0120)
#define	HIDE_COLUMN	( -4 )

#endif // MMC_VER >= 0x0120

EXTERN_C const IID IID_IHeaderCtrl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43136EB3-D36C-11CF-ADBC-00AA00A80033")
    IHeaderCtrl : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InsertColumn( 
            /* [in] */ int nCol,
            /* [in] */ __RPC__in LPCWSTR title,
            /* [in] */ int nFormat,
            /* [in] */ int nWidth) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteColumn( 
            /* [in] */ int nCol) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetColumnText( 
            /* [in] */ int nCol,
            /* [in] */ __RPC__in LPCWSTR title) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetColumnText( 
            /* [in] */ int nCol,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *pText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetColumnWidth( 
            /* [in] */ int nCol,
            /* [in] */ int nWidth) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetColumnWidth( 
            /* [in] */ int nCol,
            /* [out] */ __RPC__out int *pWidth) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHeaderCtrlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHeaderCtrl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHeaderCtrl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHeaderCtrl * This);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, InsertColumn)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertColumn )( 
            __RPC__in IHeaderCtrl * This,
            /* [in] */ int nCol,
            /* [in] */ __RPC__in LPCWSTR title,
            /* [in] */ int nFormat,
            /* [in] */ int nWidth);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, DeleteColumn)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteColumn )( 
            __RPC__in IHeaderCtrl * This,
            /* [in] */ int nCol);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, SetColumnText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetColumnText )( 
            __RPC__in IHeaderCtrl * This,
            /* [in] */ int nCol,
            /* [in] */ __RPC__in LPCWSTR title);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, GetColumnText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumnText )( 
            __RPC__in IHeaderCtrl * This,
            /* [in] */ int nCol,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *pText);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, SetColumnWidth)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetColumnWidth )( 
            __RPC__in IHeaderCtrl * This,
            /* [in] */ int nCol,
            /* [in] */ int nWidth);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, GetColumnWidth)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumnWidth )( 
            __RPC__in IHeaderCtrl * This,
            /* [in] */ int nCol,
            /* [out] */ __RPC__out int *pWidth);
        
        END_INTERFACE
    } IHeaderCtrlVtbl;

    interface IHeaderCtrl
    {
        CONST_VTBL struct IHeaderCtrlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHeaderCtrl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHeaderCtrl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHeaderCtrl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHeaderCtrl_InsertColumn(This,nCol,title,nFormat,nWidth)	\
    ( (This)->lpVtbl -> InsertColumn(This,nCol,title,nFormat,nWidth) ) 

#define IHeaderCtrl_DeleteColumn(This,nCol)	\
    ( (This)->lpVtbl -> DeleteColumn(This,nCol) ) 

#define IHeaderCtrl_SetColumnText(This,nCol,title)	\
    ( (This)->lpVtbl -> SetColumnText(This,nCol,title) ) 

#define IHeaderCtrl_GetColumnText(This,nCol,pText)	\
    ( (This)->lpVtbl -> GetColumnText(This,nCol,pText) ) 

#define IHeaderCtrl_SetColumnWidth(This,nCol,nWidth)	\
    ( (This)->lpVtbl -> SetColumnWidth(This,nCol,nWidth) ) 

#define IHeaderCtrl_GetColumnWidth(This,nCol,pWidth)	\
    ( (This)->lpVtbl -> GetColumnWidth(This,nCol,pWidth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHeaderCtrl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0006 */
/* [local] */ 


enum __MIDL___MIDL_itf_mmc_0000_0006_0001
    {
        CCM_INSERTIONPOINTID_MASK_SPECIAL	= 0xffff0000,
        CCM_INSERTIONPOINTID_MASK_SHARED	= 0x80000000,
        CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY	= 0x40000000,
        CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY	= 0x20000000,
        CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY	= 0x10000000,
        CCM_INSERTIONPOINTID_MASK_RESERVED	= 0xfff0000,
        CCM_INSERTIONPOINTID_MASK_FLAGINDEX	= 0x1f,
        CCM_INSERTIONPOINTID_PRIMARY_TOP	= 0xa0000000,
        CCM_INSERTIONPOINTID_PRIMARY_NEW	= 0xa0000001,
        CCM_INSERTIONPOINTID_PRIMARY_TASK	= 0xa0000002,
        CCM_INSERTIONPOINTID_PRIMARY_VIEW	= 0xa0000003,
        CCM_INSERTIONPOINTID_PRIMARY_HELP	= 0xa0000004,
        CCM_INSERTIONPOINTID_3RDPARTY_NEW	= 0x90000001,
        CCM_INSERTIONPOINTID_3RDPARTY_TASK	= 0x90000002,
        CCM_INSERTIONPOINTID_ROOT_MENU	= 0x80000000
    } ;

enum __MIDL___MIDL_itf_mmc_0000_0006_0002
    {
        CCM_INSERTIONALLOWED_TOP	= ( 1L << (( CCM_INSERTIONPOINTID_PRIMARY_TOP & CCM_INSERTIONPOINTID_MASK_FLAGINDEX ) ) ) ,
        CCM_INSERTIONALLOWED_NEW	= ( 1L << (( CCM_INSERTIONPOINTID_PRIMARY_NEW & CCM_INSERTIONPOINTID_MASK_FLAGINDEX ) ) ) ,
        CCM_INSERTIONALLOWED_TASK	= ( 1L << (( CCM_INSERTIONPOINTID_PRIMARY_TASK & CCM_INSERTIONPOINTID_MASK_FLAGINDEX ) ) ) ,
        CCM_INSERTIONALLOWED_VIEW	= ( 1L << (( CCM_INSERTIONPOINTID_PRIMARY_VIEW & CCM_INSERTIONPOINTID_MASK_FLAGINDEX ) ) ) 
    } ;

enum __MIDL___MIDL_itf_mmc_0000_0006_0003
    {
        CCM_COMMANDID_MASK_RESERVED	= 0xffff0000
    } ;

enum __MIDL___MIDL_itf_mmc_0000_0006_0004
    {
        CCM_SPECIAL_SEPARATOR	= 0x1,
        CCM_SPECIAL_SUBMENU	= 0x2,
        CCM_SPECIAL_DEFAULT_ITEM	= 0x4,
        CCM_SPECIAL_INSERTION_POINT	= 0x8,
        CCM_SPECIAL_TESTONLY	= 0x10,
        CCM_SPECIAL_ELEVATION_ICON	= 0x20
    } ;


extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0006_v0_0_s_ifspec;

#ifndef __IContextMenuCallback_INTERFACE_DEFINED__
#define __IContextMenuCallback_INTERFACE_DEFINED__

/* interface IContextMenuCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IContextMenuCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43136EB7-D36C-11CF-ADBC-00AA00A80033")
    IContextMenuCallback : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in CONTEXTMENUITEM *pItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextMenuCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContextMenuCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContextMenuCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContextMenuCallback * This);
        
        DECLSPEC_XFGVIRT(IContextMenuCallback, AddItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in IContextMenuCallback * This,
            /* [in] */ __RPC__in CONTEXTMENUITEM *pItem);
        
        END_INTERFACE
    } IContextMenuCallbackVtbl;

    interface IContextMenuCallback
    {
        CONST_VTBL struct IContextMenuCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContextMenuCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContextMenuCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContextMenuCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContextMenuCallback_AddItem(This,pItem)	\
    ( (This)->lpVtbl -> AddItem(This,pItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContextMenuCallback_INTERFACE_DEFINED__ */


#ifndef __IContextMenuProvider_INTERFACE_DEFINED__
#define __IContextMenuProvider_INTERFACE_DEFINED__

/* interface IContextMenuProvider */
/* [unique][helpstring][object][uuid][object] */ 


EXTERN_C const IID IID_IContextMenuProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43136EB6-D36C-11CF-ADBC-00AA00A80033")
    IContextMenuProvider : public IContextMenuCallback
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EmptyMenuList( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddPrimaryExtensionItems( 
            /* [in] */ __RPC__in_opt LPUNKNOWN piExtension,
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddThirdPartyExtensionItems( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ShowContextMenu( 
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ long xPos,
            /* [in] */ long yPos,
            /* [retval][out] */ __RPC__out long *plSelected) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextMenuProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContextMenuProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContextMenuProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContextMenuProvider * This);
        
        DECLSPEC_XFGVIRT(IContextMenuCallback, AddItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in IContextMenuProvider * This,
            /* [in] */ __RPC__in CONTEXTMENUITEM *pItem);
        
        DECLSPEC_XFGVIRT(IContextMenuProvider, EmptyMenuList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EmptyMenuList )( 
            __RPC__in IContextMenuProvider * This);
        
        DECLSPEC_XFGVIRT(IContextMenuProvider, AddPrimaryExtensionItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddPrimaryExtensionItems )( 
            __RPC__in IContextMenuProvider * This,
            /* [in] */ __RPC__in_opt LPUNKNOWN piExtension,
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject);
        
        DECLSPEC_XFGVIRT(IContextMenuProvider, AddThirdPartyExtensionItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddThirdPartyExtensionItems )( 
            __RPC__in IContextMenuProvider * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject);
        
        DECLSPEC_XFGVIRT(IContextMenuProvider, ShowContextMenu)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShowContextMenu )( 
            __RPC__in IContextMenuProvider * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ long xPos,
            /* [in] */ long yPos,
            /* [retval][out] */ __RPC__out long *plSelected);
        
        END_INTERFACE
    } IContextMenuProviderVtbl;

    interface IContextMenuProvider
    {
        CONST_VTBL struct IContextMenuProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContextMenuProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContextMenuProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContextMenuProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContextMenuProvider_AddItem(This,pItem)	\
    ( (This)->lpVtbl -> AddItem(This,pItem) ) 


#define IContextMenuProvider_EmptyMenuList(This)	\
    ( (This)->lpVtbl -> EmptyMenuList(This) ) 

#define IContextMenuProvider_AddPrimaryExtensionItems(This,piExtension,piDataObject)	\
    ( (This)->lpVtbl -> AddPrimaryExtensionItems(This,piExtension,piDataObject) ) 

#define IContextMenuProvider_AddThirdPartyExtensionItems(This,piDataObject)	\
    ( (This)->lpVtbl -> AddThirdPartyExtensionItems(This,piDataObject) ) 

#define IContextMenuProvider_ShowContextMenu(This,hwndParent,xPos,yPos,plSelected)	\
    ( (This)->lpVtbl -> ShowContextMenu(This,hwndParent,xPos,yPos,plSelected) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContextMenuProvider_INTERFACE_DEFINED__ */


#ifndef __IExtendContextMenu_INTERFACE_DEFINED__
#define __IExtendContextMenu_INTERFACE_DEFINED__

/* interface IExtendContextMenu */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IExtendContextMenu;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4F3B7A4F-CFAC-11CF-B8E3-00C04FD8D5B0")
    IExtendContextMenu : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddMenuItems( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject,
            /* [in] */ __RPC__in_opt LPCONTEXTMENUCALLBACK piCallback,
            /* [out][in] */ __RPC__inout long *pInsertionAllowed) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Command( 
            /* [in] */ long lCommandID,
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExtendContextMenuVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IExtendContextMenu * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IExtendContextMenu * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IExtendContextMenu * This);
        
        DECLSPEC_XFGVIRT(IExtendContextMenu, AddMenuItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddMenuItems )( 
            __RPC__in IExtendContextMenu * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject,
            /* [in] */ __RPC__in_opt LPCONTEXTMENUCALLBACK piCallback,
            /* [out][in] */ __RPC__inout long *pInsertionAllowed);
        
        DECLSPEC_XFGVIRT(IExtendContextMenu, Command)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Command )( 
            __RPC__in IExtendContextMenu * This,
            /* [in] */ long lCommandID,
            /* [in] */ __RPC__in_opt LPDATAOBJECT piDataObject);
        
        END_INTERFACE
    } IExtendContextMenuVtbl;

    interface IExtendContextMenu
    {
        CONST_VTBL struct IExtendContextMenuVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExtendContextMenu_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExtendContextMenu_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExtendContextMenu_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExtendContextMenu_AddMenuItems(This,piDataObject,piCallback,pInsertionAllowed)	\
    ( (This)->lpVtbl -> AddMenuItems(This,piDataObject,piCallback,pInsertionAllowed) ) 

#define IExtendContextMenu_Command(This,lCommandID,piDataObject)	\
    ( (This)->lpVtbl -> Command(This,lCommandID,piDataObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExtendContextMenu_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0009 */
/* [local] */ 

#if (MMC_VER >= 0x0120)
#define ILSIF_LEAVE_LARGE_ICON  0x40000000
#define ILSIF_LEAVE_SMALL_ICON  0x20000000
#define ILSIF_LEAVE_MASK        (ILSIF_LEAVE_LARGE_ICON | ILSIF_LEAVE_SMALL_ICON)
#define ILSI_LARGE_ICON(nLoc)   (nLoc | ILSIF_LEAVE_SMALL_ICON)
#define ILSI_SMALL_ICON(nLoc)   (nLoc | ILSIF_LEAVE_LARGE_ICON)
#endif // MMC_VER >= 0x0120


extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0009_v0_0_s_ifspec;

#ifndef __IImageList_INTERFACE_DEFINED__
#define __IImageList_INTERFACE_DEFINED__

/* interface IImageList */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IImageList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43136EB8-D36C-11CF-ADBC-00AA00A80033")
    IImageList : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ImageListSetIcon( 
            /* [in] */ __RPC__in LONG_PTR *pIcon,
            /* [in] */ long nLoc) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ImageListSetStrip( 
            /* [in] */ __RPC__in LONG_PTR *pBMapSm,
            /* [in] */ __RPC__in LONG_PTR *pBMapLg,
            /* [in] */ long nStartLoc,
            /* [in] */ COLORREF cMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IImageListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IImageList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IImageList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IImageList * This);
        
        DECLSPEC_XFGVIRT(IImageList, ImageListSetIcon)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ImageListSetIcon )( 
            __RPC__in IImageList * This,
            /* [in] */ __RPC__in LONG_PTR *pIcon,
            /* [in] */ long nLoc);
        
        DECLSPEC_XFGVIRT(IImageList, ImageListSetStrip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ImageListSetStrip )( 
            __RPC__in IImageList * This,
            /* [in] */ __RPC__in LONG_PTR *pBMapSm,
            /* [in] */ __RPC__in LONG_PTR *pBMapLg,
            /* [in] */ long nStartLoc,
            /* [in] */ COLORREF cMask);
        
        END_INTERFACE
    } IImageListVtbl;

    interface IImageList
    {
        CONST_VTBL struct IImageListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IImageList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IImageList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IImageList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IImageList_ImageListSetIcon(This,pIcon,nLoc)	\
    ( (This)->lpVtbl -> ImageListSetIcon(This,pIcon,nLoc) ) 

#define IImageList_ImageListSetStrip(This,pBMapSm,pBMapLg,nStartLoc,cMask)	\
    ( (This)->lpVtbl -> ImageListSetStrip(This,pBMapSm,pBMapLg,nStartLoc,cMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IImageList_INTERFACE_DEFINED__ */


#ifndef __IResultData_INTERFACE_DEFINED__
#define __IResultData_INTERFACE_DEFINED__

/* interface IResultData */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IResultData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31DA5FA0-E0EB-11cf-9F21-00AA003CA9F6")
    IResultData : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InsertItem( 
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ HRESULTITEM itemID,
            /* [in] */ int nCol) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindItemByLParam( 
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out HRESULTITEM *pItemID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteAllRsltItems( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetItem( 
            /* [in] */ __RPC__in LPRESULTDATAITEM item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetItem( 
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNextItem( 
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ModifyItemState( 
            /* [in] */ int nIndex,
            /* [in] */ HRESULTITEM itemID,
            /* [in] */ UINT uAdd,
            /* [in] */ UINT uRemove) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ModifyViewStyle( 
            /* [in] */ MMC_RESULT_VIEW_STYLE add,
            /* [in] */ MMC_RESULT_VIEW_STYLE remove) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetViewMode( 
            /* [in] */ long lViewMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetViewMode( 
            /* [out] */ __RPC__out long *lViewMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UpdateItem( 
            /* [in] */ HRESULTITEM itemID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Sort( 
            /* [in] */ int nColumn,
            /* [in] */ DWORD dwSortOptions,
            /* [in] */ LPARAM lUserParam) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetDescBarText( 
            /* [in] */ __RPC__in LPOLESTR DescText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetItemCount( 
            /* [in] */ int nItemCount,
            /* [in] */ DWORD dwOptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResultDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResultData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResultData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResultData * This);
        
        DECLSPEC_XFGVIRT(IResultData, InsertItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertItem )( 
            __RPC__in IResultData * This,
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, DeleteItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IResultData * This,
            /* [in] */ HRESULTITEM itemID,
            /* [in] */ int nCol);
        
        DECLSPEC_XFGVIRT(IResultData, FindItemByLParam)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindItemByLParam )( 
            __RPC__in IResultData * This,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out HRESULTITEM *pItemID);
        
        DECLSPEC_XFGVIRT(IResultData, DeleteAllRsltItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteAllRsltItems )( 
            __RPC__in IResultData * This);
        
        DECLSPEC_XFGVIRT(IResultData, SetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IResultData * This,
            /* [in] */ __RPC__in LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, GetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IResultData * This,
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, GetNextItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNextItem )( 
            __RPC__in IResultData * This,
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, ModifyItemState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ModifyItemState )( 
            __RPC__in IResultData * This,
            /* [in] */ int nIndex,
            /* [in] */ HRESULTITEM itemID,
            /* [in] */ UINT uAdd,
            /* [in] */ UINT uRemove);
        
        DECLSPEC_XFGVIRT(IResultData, ModifyViewStyle)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ModifyViewStyle )( 
            __RPC__in IResultData * This,
            /* [in] */ MMC_RESULT_VIEW_STYLE add,
            /* [in] */ MMC_RESULT_VIEW_STYLE remove);
        
        DECLSPEC_XFGVIRT(IResultData, SetViewMode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetViewMode )( 
            __RPC__in IResultData * This,
            /* [in] */ long lViewMode);
        
        DECLSPEC_XFGVIRT(IResultData, GetViewMode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetViewMode )( 
            __RPC__in IResultData * This,
            /* [out] */ __RPC__out long *lViewMode);
        
        DECLSPEC_XFGVIRT(IResultData, UpdateItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateItem )( 
            __RPC__in IResultData * This,
            /* [in] */ HRESULTITEM itemID);
        
        DECLSPEC_XFGVIRT(IResultData, Sort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Sort )( 
            __RPC__in IResultData * This,
            /* [in] */ int nColumn,
            /* [in] */ DWORD dwSortOptions,
            /* [in] */ LPARAM lUserParam);
        
        DECLSPEC_XFGVIRT(IResultData, SetDescBarText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetDescBarText )( 
            __RPC__in IResultData * This,
            /* [in] */ __RPC__in LPOLESTR DescText);
        
        DECLSPEC_XFGVIRT(IResultData, SetItemCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetItemCount )( 
            __RPC__in IResultData * This,
            /* [in] */ int nItemCount,
            /* [in] */ DWORD dwOptions);
        
        END_INTERFACE
    } IResultDataVtbl;

    interface IResultData
    {
        CONST_VTBL struct IResultDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResultData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResultData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResultData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResultData_InsertItem(This,item)	\
    ( (This)->lpVtbl -> InsertItem(This,item) ) 

#define IResultData_DeleteItem(This,itemID,nCol)	\
    ( (This)->lpVtbl -> DeleteItem(This,itemID,nCol) ) 

#define IResultData_FindItemByLParam(This,lParam,pItemID)	\
    ( (This)->lpVtbl -> FindItemByLParam(This,lParam,pItemID) ) 

#define IResultData_DeleteAllRsltItems(This)	\
    ( (This)->lpVtbl -> DeleteAllRsltItems(This) ) 

#define IResultData_SetItem(This,item)	\
    ( (This)->lpVtbl -> SetItem(This,item) ) 

#define IResultData_GetItem(This,item)	\
    ( (This)->lpVtbl -> GetItem(This,item) ) 

#define IResultData_GetNextItem(This,item)	\
    ( (This)->lpVtbl -> GetNextItem(This,item) ) 

#define IResultData_ModifyItemState(This,nIndex,itemID,uAdd,uRemove)	\
    ( (This)->lpVtbl -> ModifyItemState(This,nIndex,itemID,uAdd,uRemove) ) 

#define IResultData_ModifyViewStyle(This,add,remove)	\
    ( (This)->lpVtbl -> ModifyViewStyle(This,add,remove) ) 

#define IResultData_SetViewMode(This,lViewMode)	\
    ( (This)->lpVtbl -> SetViewMode(This,lViewMode) ) 

#define IResultData_GetViewMode(This,lViewMode)	\
    ( (This)->lpVtbl -> GetViewMode(This,lViewMode) ) 

#define IResultData_UpdateItem(This,itemID)	\
    ( (This)->lpVtbl -> UpdateItem(This,itemID) ) 

#define IResultData_Sort(This,nColumn,dwSortOptions,lUserParam)	\
    ( (This)->lpVtbl -> Sort(This,nColumn,dwSortOptions,lUserParam) ) 

#define IResultData_SetDescBarText(This,DescText)	\
    ( (This)->lpVtbl -> SetDescBarText(This,DescText) ) 

#define IResultData_SetItemCount(This,nItemCount,dwOptions)	\
    ( (This)->lpVtbl -> SetItemCount(This,nItemCount,dwOptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResultData_INTERFACE_DEFINED__ */


#ifndef __IConsoleNameSpace_INTERFACE_DEFINED__
#define __IConsoleNameSpace_INTERFACE_DEFINED__

/* interface IConsoleNameSpace */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConsoleNameSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEDEB620-F24D-11cf-8AFC-00AA003CA9F6")
    IConsoleNameSpace : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InsertItem( 
            /* [out][in] */ __RPC__inout LPSCOPEDATAITEM item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ long fDeleteThis) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetItem( 
            /* [in] */ __RPC__in LPSCOPEDATAITEM item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetItem( 
            /* [out][in] */ __RPC__inout LPSCOPEDATAITEM item) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetChildItem( 
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemChild,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNextItem( 
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemNext,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetParentItem( 
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemParent,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsoleNameSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsoleNameSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsoleNameSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsoleNameSpace * This);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, InsertItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertItem )( 
            __RPC__in IConsoleNameSpace * This,
            /* [out][in] */ __RPC__inout LPSCOPEDATAITEM item);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, DeleteItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IConsoleNameSpace * This,
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ long fDeleteThis);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, SetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IConsoleNameSpace * This,
            /* [in] */ __RPC__in LPSCOPEDATAITEM item);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IConsoleNameSpace * This,
            /* [out][in] */ __RPC__inout LPSCOPEDATAITEM item);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetChildItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChildItem )( 
            __RPC__in IConsoleNameSpace * This,
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemChild,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetNextItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNextItem )( 
            __RPC__in IConsoleNameSpace * This,
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemNext,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetParentItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetParentItem )( 
            __RPC__in IConsoleNameSpace * This,
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemParent,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie);
        
        END_INTERFACE
    } IConsoleNameSpaceVtbl;

    interface IConsoleNameSpace
    {
        CONST_VTBL struct IConsoleNameSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsoleNameSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsoleNameSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsoleNameSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsoleNameSpace_InsertItem(This,item)	\
    ( (This)->lpVtbl -> InsertItem(This,item) ) 

#define IConsoleNameSpace_DeleteItem(This,hItem,fDeleteThis)	\
    ( (This)->lpVtbl -> DeleteItem(This,hItem,fDeleteThis) ) 

#define IConsoleNameSpace_SetItem(This,item)	\
    ( (This)->lpVtbl -> SetItem(This,item) ) 

#define IConsoleNameSpace_GetItem(This,item)	\
    ( (This)->lpVtbl -> GetItem(This,item) ) 

#define IConsoleNameSpace_GetChildItem(This,item,pItemChild,pCookie)	\
    ( (This)->lpVtbl -> GetChildItem(This,item,pItemChild,pCookie) ) 

#define IConsoleNameSpace_GetNextItem(This,item,pItemNext,pCookie)	\
    ( (This)->lpVtbl -> GetNextItem(This,item,pItemNext,pCookie) ) 

#define IConsoleNameSpace_GetParentItem(This,item,pItemParent,pCookie)	\
    ( (This)->lpVtbl -> GetParentItem(This,item,pItemParent,pCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsoleNameSpace_INTERFACE_DEFINED__ */


#ifndef __IConsoleNameSpace2_INTERFACE_DEFINED__
#define __IConsoleNameSpace2_INTERFACE_DEFINED__

/* interface IConsoleNameSpace2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConsoleNameSpace2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("255F18CC-65DB-11D1-A7DC-00C04FD8D565")
    IConsoleNameSpace2 : public IConsoleNameSpace
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Expand( 
            /* [in] */ HSCOPEITEM hItem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddExtension( 
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ __RPC__in LPCLSID lpClsid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsoleNameSpace2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsoleNameSpace2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsoleNameSpace2 * This);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, InsertItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertItem )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [out][in] */ __RPC__inout LPSCOPEDATAITEM item);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, DeleteItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ long fDeleteThis);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, SetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ __RPC__in LPSCOPEDATAITEM item);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [out][in] */ __RPC__inout LPSCOPEDATAITEM item);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetChildItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChildItem )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemChild,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetNextItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNextItem )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemNext,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace, GetParentItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetParentItem )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ HSCOPEITEM item,
            /* [out] */ __RPC__out HSCOPEITEM *pItemParent,
            /* [out] */ __RPC__out MMC_COOKIE *pCookie);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace2, Expand)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ HSCOPEITEM hItem);
        
        DECLSPEC_XFGVIRT(IConsoleNameSpace2, AddExtension)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddExtension )( 
            __RPC__in IConsoleNameSpace2 * This,
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ __RPC__in LPCLSID lpClsid);
        
        END_INTERFACE
    } IConsoleNameSpace2Vtbl;

    interface IConsoleNameSpace2
    {
        CONST_VTBL struct IConsoleNameSpace2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsoleNameSpace2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsoleNameSpace2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsoleNameSpace2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsoleNameSpace2_InsertItem(This,item)	\
    ( (This)->lpVtbl -> InsertItem(This,item) ) 

#define IConsoleNameSpace2_DeleteItem(This,hItem,fDeleteThis)	\
    ( (This)->lpVtbl -> DeleteItem(This,hItem,fDeleteThis) ) 

#define IConsoleNameSpace2_SetItem(This,item)	\
    ( (This)->lpVtbl -> SetItem(This,item) ) 

#define IConsoleNameSpace2_GetItem(This,item)	\
    ( (This)->lpVtbl -> GetItem(This,item) ) 

#define IConsoleNameSpace2_GetChildItem(This,item,pItemChild,pCookie)	\
    ( (This)->lpVtbl -> GetChildItem(This,item,pItemChild,pCookie) ) 

#define IConsoleNameSpace2_GetNextItem(This,item,pItemNext,pCookie)	\
    ( (This)->lpVtbl -> GetNextItem(This,item,pItemNext,pCookie) ) 

#define IConsoleNameSpace2_GetParentItem(This,item,pItemParent,pCookie)	\
    ( (This)->lpVtbl -> GetParentItem(This,item,pItemParent,pCookie) ) 


#define IConsoleNameSpace2_Expand(This,hItem)	\
    ( (This)->lpVtbl -> Expand(This,hItem) ) 

#define IConsoleNameSpace2_AddExtension(This,hItem,lpClsid)	\
    ( (This)->lpVtbl -> AddExtension(This,hItem,lpClsid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsoleNameSpace2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0013 */
/* [local] */ 


typedef struct _PSP *HPROPSHEETPAGE;



extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0013_v0_0_s_ifspec;

#ifndef __IPropertySheetCallback_INTERFACE_DEFINED__
#define __IPropertySheetCallback_INTERFACE_DEFINED__

/* interface IPropertySheetCallback */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IPropertySheetCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85DE64DD-EF21-11cf-A285-00C04FD8DBE6")
    IPropertySheetCallback : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddPage( 
            /* [in] */ HPROPSHEETPAGE hPage) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemovePage( 
            /* [in] */ HPROPSHEETPAGE hPage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertySheetCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPropertySheetCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPropertySheetCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPropertySheetCallback * This);
        
        DECLSPEC_XFGVIRT(IPropertySheetCallback, AddPage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddPage )( 
            IPropertySheetCallback * This,
            /* [in] */ HPROPSHEETPAGE hPage);
        
        DECLSPEC_XFGVIRT(IPropertySheetCallback, RemovePage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemovePage )( 
            IPropertySheetCallback * This,
            /* [in] */ HPROPSHEETPAGE hPage);
        
        END_INTERFACE
    } IPropertySheetCallbackVtbl;

    interface IPropertySheetCallback
    {
        CONST_VTBL struct IPropertySheetCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertySheetCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertySheetCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertySheetCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertySheetCallback_AddPage(This,hPage)	\
    ( (This)->lpVtbl -> AddPage(This,hPage) ) 

#define IPropertySheetCallback_RemovePage(This,hPage)	\
    ( (This)->lpVtbl -> RemovePage(This,hPage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertySheetCallback_INTERFACE_DEFINED__ */


#ifndef __IPropertySheetProvider_INTERFACE_DEFINED__
#define __IPropertySheetProvider_INTERFACE_DEFINED__

/* interface IPropertySheetProvider */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPropertySheetProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85DE64DE-EF21-11cf-A285-00C04FD8DBE6")
    IPropertySheetProvider : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreatePropertySheet( 
            /* [in] */ __RPC__in LPCWSTR title,
            /* [in] */ boolean type,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ __RPC__in_opt LPDATAOBJECT pIDataObjectm,
            /* [in] */ DWORD dwOptions) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindPropertySheet( 
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ __RPC__in_opt LPCOMPONENT lpComponent,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddPrimaryPages( 
            __RPC__in_opt LPUNKNOWN lpUnknown,
            BOOL bCreateHandle,
            __RPC__in HWND hNotifyWindow,
            BOOL bScopePane) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddExtensionPages( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Show( 
            /* [in] */ LONG_PTR window,
            /* [in] */ int page) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertySheetProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertySheetProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertySheetProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertySheetProvider * This);
        
        DECLSPEC_XFGVIRT(IPropertySheetProvider, CreatePropertySheet)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreatePropertySheet )( 
            __RPC__in IPropertySheetProvider * This,
            /* [in] */ __RPC__in LPCWSTR title,
            /* [in] */ boolean type,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ __RPC__in_opt LPDATAOBJECT pIDataObjectm,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IPropertySheetProvider, FindPropertySheet)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindPropertySheet )( 
            __RPC__in IPropertySheetProvider * This,
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ __RPC__in_opt LPCOMPONENT lpComponent,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject);
        
        DECLSPEC_XFGVIRT(IPropertySheetProvider, AddPrimaryPages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddPrimaryPages )( 
            __RPC__in IPropertySheetProvider * This,
            __RPC__in_opt LPUNKNOWN lpUnknown,
            BOOL bCreateHandle,
            __RPC__in HWND hNotifyWindow,
            BOOL bScopePane);
        
        DECLSPEC_XFGVIRT(IPropertySheetProvider, AddExtensionPages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddExtensionPages )( 
            __RPC__in IPropertySheetProvider * This);
        
        DECLSPEC_XFGVIRT(IPropertySheetProvider, Show)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in IPropertySheetProvider * This,
            /* [in] */ LONG_PTR window,
            /* [in] */ int page);
        
        END_INTERFACE
    } IPropertySheetProviderVtbl;

    interface IPropertySheetProvider
    {
        CONST_VTBL struct IPropertySheetProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertySheetProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertySheetProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertySheetProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertySheetProvider_CreatePropertySheet(This,title,type,cookie,pIDataObjectm,dwOptions)	\
    ( (This)->lpVtbl -> CreatePropertySheet(This,title,type,cookie,pIDataObjectm,dwOptions) ) 

#define IPropertySheetProvider_FindPropertySheet(This,hItem,lpComponent,lpDataObject)	\
    ( (This)->lpVtbl -> FindPropertySheet(This,hItem,lpComponent,lpDataObject) ) 

#define IPropertySheetProvider_AddPrimaryPages(This,lpUnknown,bCreateHandle,hNotifyWindow,bScopePane)	\
    ( (This)->lpVtbl -> AddPrimaryPages(This,lpUnknown,bCreateHandle,hNotifyWindow,bScopePane) ) 

#define IPropertySheetProvider_AddExtensionPages(This)	\
    ( (This)->lpVtbl -> AddExtensionPages(This) ) 

#define IPropertySheetProvider_Show(This,window,page)	\
    ( (This)->lpVtbl -> Show(This,window,page) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPropertySheetProvider_INTERFACE_DEFINED__ */


#ifndef __IExtendPropertySheet_INTERFACE_DEFINED__
#define __IExtendPropertySheet_INTERFACE_DEFINED__

/* interface IExtendPropertySheet */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IExtendPropertySheet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85DE64DC-EF21-11cf-A285-00C04FD8DBE6")
    IExtendPropertySheet : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreatePropertyPages( 
            /* [in] */ __RPC__in_opt LPPROPERTYSHEETCALLBACK lpProvider,
            /* [in] */ LONG_PTR handle,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpIDataObject) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryPagesFor( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExtendPropertySheetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IExtendPropertySheet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IExtendPropertySheet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IExtendPropertySheet * This);
        
        DECLSPEC_XFGVIRT(IExtendPropertySheet, CreatePropertyPages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreatePropertyPages )( 
            __RPC__in IExtendPropertySheet * This,
            /* [in] */ __RPC__in_opt LPPROPERTYSHEETCALLBACK lpProvider,
            /* [in] */ LONG_PTR handle,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpIDataObject);
        
        DECLSPEC_XFGVIRT(IExtendPropertySheet, QueryPagesFor)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryPagesFor )( 
            __RPC__in IExtendPropertySheet * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject);
        
        END_INTERFACE
    } IExtendPropertySheetVtbl;

    interface IExtendPropertySheet
    {
        CONST_VTBL struct IExtendPropertySheetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExtendPropertySheet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExtendPropertySheet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExtendPropertySheet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExtendPropertySheet_CreatePropertyPages(This,lpProvider,handle,lpIDataObject)	\
    ( (This)->lpVtbl -> CreatePropertyPages(This,lpProvider,handle,lpIDataObject) ) 

#define IExtendPropertySheet_QueryPagesFor(This,lpDataObject)	\
    ( (This)->lpVtbl -> QueryPagesFor(This,lpDataObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExtendPropertySheet_INTERFACE_DEFINED__ */


#ifndef __IControlbar_INTERFACE_DEFINED__
#define __IControlbar_INTERFACE_DEFINED__

/* interface IControlbar */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IControlbar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("69FB811E-6C1C-11D0-A2CB-00C04FD909DD")
    IControlbar : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ MMC_CONTROL_TYPE nType,
            /* [in] */ __RPC__in_opt LPEXTENDCONTROLBAR pExtendControlbar,
            /* [out] */ __RPC__deref_out_opt LPUNKNOWN *ppUnknown) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Attach( 
            /* [in] */ MMC_CONTROL_TYPE nType,
            /* [in] */ __RPC__in_opt LPUNKNOWN lpUnknown) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Detach( 
            /* [in] */ __RPC__in_opt LPUNKNOWN lpUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IControlbarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IControlbar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IControlbar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IControlbar * This);
        
        DECLSPEC_XFGVIRT(IControlbar, Create)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IControlbar * This,
            /* [in] */ MMC_CONTROL_TYPE nType,
            /* [in] */ __RPC__in_opt LPEXTENDCONTROLBAR pExtendControlbar,
            /* [out] */ __RPC__deref_out_opt LPUNKNOWN *ppUnknown);
        
        DECLSPEC_XFGVIRT(IControlbar, Attach)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Attach )( 
            __RPC__in IControlbar * This,
            /* [in] */ MMC_CONTROL_TYPE nType,
            /* [in] */ __RPC__in_opt LPUNKNOWN lpUnknown);
        
        DECLSPEC_XFGVIRT(IControlbar, Detach)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Detach )( 
            __RPC__in IControlbar * This,
            /* [in] */ __RPC__in_opt LPUNKNOWN lpUnknown);
        
        END_INTERFACE
    } IControlbarVtbl;

    interface IControlbar
    {
        CONST_VTBL struct IControlbarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IControlbar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IControlbar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IControlbar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IControlbar_Create(This,nType,pExtendControlbar,ppUnknown)	\
    ( (This)->lpVtbl -> Create(This,nType,pExtendControlbar,ppUnknown) ) 

#define IControlbar_Attach(This,nType,lpUnknown)	\
    ( (This)->lpVtbl -> Attach(This,nType,lpUnknown) ) 

#define IControlbar_Detach(This,lpUnknown)	\
    ( (This)->lpVtbl -> Detach(This,lpUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IControlbar_INTERFACE_DEFINED__ */


#ifndef __IExtendControlbar_INTERFACE_DEFINED__
#define __IExtendControlbar_INTERFACE_DEFINED__

/* interface IExtendControlbar */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IExtendControlbar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49506520-6F40-11D0-A98B-00C04FD8D565")
    IExtendControlbar : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetControlbar( 
            /* [in] */ __RPC__in_opt LPCONTROLBAR pControlbar) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ControlbarNotify( 
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExtendControlbarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IExtendControlbar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IExtendControlbar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IExtendControlbar * This);
        
        DECLSPEC_XFGVIRT(IExtendControlbar, SetControlbar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetControlbar )( 
            __RPC__in IExtendControlbar * This,
            /* [in] */ __RPC__in_opt LPCONTROLBAR pControlbar);
        
        DECLSPEC_XFGVIRT(IExtendControlbar, ControlbarNotify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ControlbarNotify )( 
            __RPC__in IExtendControlbar * This,
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param);
        
        END_INTERFACE
    } IExtendControlbarVtbl;

    interface IExtendControlbar
    {
        CONST_VTBL struct IExtendControlbarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExtendControlbar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExtendControlbar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExtendControlbar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExtendControlbar_SetControlbar(This,pControlbar)	\
    ( (This)->lpVtbl -> SetControlbar(This,pControlbar) ) 

#define IExtendControlbar_ControlbarNotify(This,event,arg,param)	\
    ( (This)->lpVtbl -> ControlbarNotify(This,event,arg,param) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExtendControlbar_INTERFACE_DEFINED__ */


#ifndef __IToolbar_INTERFACE_DEFINED__
#define __IToolbar_INTERFACE_DEFINED__

/* interface IToolbar */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IToolbar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43136EB9-D36C-11CF-ADBC-00AA00A80033")
    IToolbar : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddBitmap( 
            /* [in] */ int nImages,
            /* [in] */ __RPC__in HBITMAP hbmp,
            /* [in] */ int cxSize,
            /* [in] */ int cySize,
            /* [in] */ COLORREF crMask) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddButtons( 
            /* [in] */ int nButtons,
            /* [in] */ __RPC__in LPMMCBUTTON lpButtons) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InsertButton( 
            /* [in] */ int nIndex,
            /* [in] */ __RPC__in LPMMCBUTTON lpButton) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteButton( 
            /* [in] */ int nIndex) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetButtonState( 
            /* [in] */ int idCommand,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [out] */ __RPC__out BOOL *pState) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetButtonState( 
            /* [in] */ int idCommand,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [in] */ BOOL bState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IToolbarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IToolbar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IToolbar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IToolbar * This);
        
        DECLSPEC_XFGVIRT(IToolbar, AddBitmap)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddBitmap )( 
            __RPC__in IToolbar * This,
            /* [in] */ int nImages,
            /* [in] */ __RPC__in HBITMAP hbmp,
            /* [in] */ int cxSize,
            /* [in] */ int cySize,
            /* [in] */ COLORREF crMask);
        
        DECLSPEC_XFGVIRT(IToolbar, AddButtons)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddButtons )( 
            __RPC__in IToolbar * This,
            /* [in] */ int nButtons,
            /* [in] */ __RPC__in LPMMCBUTTON lpButtons);
        
        DECLSPEC_XFGVIRT(IToolbar, InsertButton)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertButton )( 
            __RPC__in IToolbar * This,
            /* [in] */ int nIndex,
            /* [in] */ __RPC__in LPMMCBUTTON lpButton);
        
        DECLSPEC_XFGVIRT(IToolbar, DeleteButton)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteButton )( 
            __RPC__in IToolbar * This,
            /* [in] */ int nIndex);
        
        DECLSPEC_XFGVIRT(IToolbar, GetButtonState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetButtonState )( 
            __RPC__in IToolbar * This,
            /* [in] */ int idCommand,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [out] */ __RPC__out BOOL *pState);
        
        DECLSPEC_XFGVIRT(IToolbar, SetButtonState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetButtonState )( 
            __RPC__in IToolbar * This,
            /* [in] */ int idCommand,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [in] */ BOOL bState);
        
        END_INTERFACE
    } IToolbarVtbl;

    interface IToolbar
    {
        CONST_VTBL struct IToolbarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IToolbar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IToolbar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IToolbar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IToolbar_AddBitmap(This,nImages,hbmp,cxSize,cySize,crMask)	\
    ( (This)->lpVtbl -> AddBitmap(This,nImages,hbmp,cxSize,cySize,crMask) ) 

#define IToolbar_AddButtons(This,nButtons,lpButtons)	\
    ( (This)->lpVtbl -> AddButtons(This,nButtons,lpButtons) ) 

#define IToolbar_InsertButton(This,nIndex,lpButton)	\
    ( (This)->lpVtbl -> InsertButton(This,nIndex,lpButton) ) 

#define IToolbar_DeleteButton(This,nIndex)	\
    ( (This)->lpVtbl -> DeleteButton(This,nIndex) ) 

#define IToolbar_GetButtonState(This,idCommand,nState,pState)	\
    ( (This)->lpVtbl -> GetButtonState(This,idCommand,nState,pState) ) 

#define IToolbar_SetButtonState(This,idCommand,nState,bState)	\
    ( (This)->lpVtbl -> SetButtonState(This,idCommand,nState,bState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IToolbar_INTERFACE_DEFINED__ */


#ifndef __IConsoleVerb_INTERFACE_DEFINED__
#define __IConsoleVerb_INTERFACE_DEFINED__

/* interface IConsoleVerb */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IConsoleVerb;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E49F7A60-74AF-11D0-A286-00C04FD8FE93")
    IConsoleVerb : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetVerbState( 
            /* [in] */ MMC_CONSOLE_VERB eCmdID,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [out] */ __RPC__out BOOL *pState) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetVerbState( 
            /* [in] */ MMC_CONSOLE_VERB eCmdID,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [in] */ BOOL bState) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetDefaultVerb( 
            /* [in] */ MMC_CONSOLE_VERB eCmdID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDefaultVerb( 
            /* [out] */ __RPC__out MMC_CONSOLE_VERB *peCmdID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsoleVerbVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsoleVerb * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsoleVerb * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsoleVerb * This);
        
        DECLSPEC_XFGVIRT(IConsoleVerb, GetVerbState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVerbState )( 
            __RPC__in IConsoleVerb * This,
            /* [in] */ MMC_CONSOLE_VERB eCmdID,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [out] */ __RPC__out BOOL *pState);
        
        DECLSPEC_XFGVIRT(IConsoleVerb, SetVerbState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetVerbState )( 
            __RPC__in IConsoleVerb * This,
            /* [in] */ MMC_CONSOLE_VERB eCmdID,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [in] */ BOOL bState);
        
        DECLSPEC_XFGVIRT(IConsoleVerb, SetDefaultVerb)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultVerb )( 
            __RPC__in IConsoleVerb * This,
            /* [in] */ MMC_CONSOLE_VERB eCmdID);
        
        DECLSPEC_XFGVIRT(IConsoleVerb, GetDefaultVerb)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultVerb )( 
            __RPC__in IConsoleVerb * This,
            /* [out] */ __RPC__out MMC_CONSOLE_VERB *peCmdID);
        
        END_INTERFACE
    } IConsoleVerbVtbl;

    interface IConsoleVerb
    {
        CONST_VTBL struct IConsoleVerbVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsoleVerb_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsoleVerb_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsoleVerb_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsoleVerb_GetVerbState(This,eCmdID,nState,pState)	\
    ( (This)->lpVtbl -> GetVerbState(This,eCmdID,nState,pState) ) 

#define IConsoleVerb_SetVerbState(This,eCmdID,nState,bState)	\
    ( (This)->lpVtbl -> SetVerbState(This,eCmdID,nState,bState) ) 

#define IConsoleVerb_SetDefaultVerb(This,eCmdID)	\
    ( (This)->lpVtbl -> SetDefaultVerb(This,eCmdID) ) 

#define IConsoleVerb_GetDefaultVerb(This,peCmdID)	\
    ( (This)->lpVtbl -> GetDefaultVerb(This,peCmdID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsoleVerb_INTERFACE_DEFINED__ */


#ifndef __ISnapinAbout_INTERFACE_DEFINED__
#define __ISnapinAbout_INTERFACE_DEFINED__

/* interface ISnapinAbout */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISnapinAbout;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1245208C-A151-11D0-A7D7-00C04FD909DD")
    ISnapinAbout : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSnapinDescription( 
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpDescription) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProvider( 
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSnapinVersion( 
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpVersion) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSnapinImage( 
            /* [out] */ __RPC__deref_out_opt HICON *hAppIcon) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStaticFolderImage( 
            /* [out] */ __RPC__deref_out_opt HBITMAP *hSmallImage,
            /* [out] */ __RPC__deref_out_opt HBITMAP *hSmallImageOpen,
            /* [out] */ __RPC__deref_out_opt HBITMAP *hLargeImage,
            /* [out] */ __RPC__out COLORREF *cMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISnapinAboutVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISnapinAbout * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISnapinAbout * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISnapinAbout * This);
        
        DECLSPEC_XFGVIRT(ISnapinAbout, GetSnapinDescription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSnapinDescription )( 
            __RPC__in ISnapinAbout * This,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpDescription);
        
        DECLSPEC_XFGVIRT(ISnapinAbout, GetProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProvider )( 
            __RPC__in ISnapinAbout * This,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpName);
        
        DECLSPEC_XFGVIRT(ISnapinAbout, GetSnapinVersion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSnapinVersion )( 
            __RPC__in ISnapinAbout * This,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpVersion);
        
        DECLSPEC_XFGVIRT(ISnapinAbout, GetSnapinImage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSnapinImage )( 
            __RPC__in ISnapinAbout * This,
            /* [out] */ __RPC__deref_out_opt HICON *hAppIcon);
        
        DECLSPEC_XFGVIRT(ISnapinAbout, GetStaticFolderImage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStaticFolderImage )( 
            __RPC__in ISnapinAbout * This,
            /* [out] */ __RPC__deref_out_opt HBITMAP *hSmallImage,
            /* [out] */ __RPC__deref_out_opt HBITMAP *hSmallImageOpen,
            /* [out] */ __RPC__deref_out_opt HBITMAP *hLargeImage,
            /* [out] */ __RPC__out COLORREF *cMask);
        
        END_INTERFACE
    } ISnapinAboutVtbl;

    interface ISnapinAbout
    {
        CONST_VTBL struct ISnapinAboutVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISnapinAbout_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISnapinAbout_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISnapinAbout_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISnapinAbout_GetSnapinDescription(This,lpDescription)	\
    ( (This)->lpVtbl -> GetSnapinDescription(This,lpDescription) ) 

#define ISnapinAbout_GetProvider(This,lpName)	\
    ( (This)->lpVtbl -> GetProvider(This,lpName) ) 

#define ISnapinAbout_GetSnapinVersion(This,lpVersion)	\
    ( (This)->lpVtbl -> GetSnapinVersion(This,lpVersion) ) 

#define ISnapinAbout_GetSnapinImage(This,hAppIcon)	\
    ( (This)->lpVtbl -> GetSnapinImage(This,hAppIcon) ) 

#define ISnapinAbout_GetStaticFolderImage(This,hSmallImage,hSmallImageOpen,hLargeImage,cMask)	\
    ( (This)->lpVtbl -> GetStaticFolderImage(This,hSmallImage,hSmallImageOpen,hLargeImage,cMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISnapinAbout_INTERFACE_DEFINED__ */


#ifndef __IMenuButton_INTERFACE_DEFINED__
#define __IMenuButton_INTERFACE_DEFINED__

/* interface IMenuButton */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMenuButton;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("951ED750-D080-11d0-B197-000000000000")
    IMenuButton : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddButton( 
            /* [in] */ int idCommand,
            /* [in] */ __RPC__in LPOLESTR lpButtonText,
            /* [in] */ __RPC__in LPOLESTR lpTooltipText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetButton( 
            /* [in] */ int idCommand,
            /* [in] */ __RPC__in LPOLESTR lpButtonText,
            /* [in] */ __RPC__in LPOLESTR lpTooltipText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetButtonState( 
            /* [in] */ int idCommand,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [in] */ BOOL bState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMenuButtonVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMenuButton * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMenuButton * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMenuButton * This);
        
        DECLSPEC_XFGVIRT(IMenuButton, AddButton)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddButton )( 
            __RPC__in IMenuButton * This,
            /* [in] */ int idCommand,
            /* [in] */ __RPC__in LPOLESTR lpButtonText,
            /* [in] */ __RPC__in LPOLESTR lpTooltipText);
        
        DECLSPEC_XFGVIRT(IMenuButton, SetButton)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetButton )( 
            __RPC__in IMenuButton * This,
            /* [in] */ int idCommand,
            /* [in] */ __RPC__in LPOLESTR lpButtonText,
            /* [in] */ __RPC__in LPOLESTR lpTooltipText);
        
        DECLSPEC_XFGVIRT(IMenuButton, SetButtonState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetButtonState )( 
            __RPC__in IMenuButton * This,
            /* [in] */ int idCommand,
            /* [in] */ MMC_BUTTON_STATE nState,
            /* [in] */ BOOL bState);
        
        END_INTERFACE
    } IMenuButtonVtbl;

    interface IMenuButton
    {
        CONST_VTBL struct IMenuButtonVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMenuButton_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMenuButton_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMenuButton_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMenuButton_AddButton(This,idCommand,lpButtonText,lpTooltipText)	\
    ( (This)->lpVtbl -> AddButton(This,idCommand,lpButtonText,lpTooltipText) ) 

#define IMenuButton_SetButton(This,idCommand,lpButtonText,lpTooltipText)	\
    ( (This)->lpVtbl -> SetButton(This,idCommand,lpButtonText,lpTooltipText) ) 

#define IMenuButton_SetButtonState(This,idCommand,nState,bState)	\
    ( (This)->lpVtbl -> SetButtonState(This,idCommand,nState,bState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMenuButton_INTERFACE_DEFINED__ */


#ifndef __ISnapinHelp_INTERFACE_DEFINED__
#define __ISnapinHelp_INTERFACE_DEFINED__

/* interface ISnapinHelp */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISnapinHelp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A6B15ACE-DF59-11D0-A7DD-00C04FD909DD")
    ISnapinHelp : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetHelpTopic( 
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpCompiledHelpFile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISnapinHelpVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISnapinHelp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISnapinHelp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISnapinHelp * This);
        
        DECLSPEC_XFGVIRT(ISnapinHelp, GetHelpTopic)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHelpTopic )( 
            __RPC__in ISnapinHelp * This,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpCompiledHelpFile);
        
        END_INTERFACE
    } ISnapinHelpVtbl;

    interface ISnapinHelp
    {
        CONST_VTBL struct ISnapinHelpVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISnapinHelp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISnapinHelp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISnapinHelp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISnapinHelp_GetHelpTopic(This,lpCompiledHelpFile)	\
    ( (This)->lpVtbl -> GetHelpTopic(This,lpCompiledHelpFile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISnapinHelp_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0023 */
/* [local] */ 

#if (MMC_VER >= 0x0110)


extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0023_v0_0_s_ifspec;

#ifndef __IExtendPropertySheet2_INTERFACE_DEFINED__
#define __IExtendPropertySheet2_INTERFACE_DEFINED__

/* interface IExtendPropertySheet2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IExtendPropertySheet2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B7A87232-4A51-11D1-A7EA-00C04FD909DD")
    IExtendPropertySheet2 : public IExtendPropertySheet
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetWatermarks( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpIDataObject,
            /* [out] */ __RPC__deref_out_opt HBITMAP *lphWatermark,
            /* [out] */ __RPC__deref_out_opt HBITMAP *lphHeader,
            /* [out] */ __RPC__deref_out_opt HPALETTE *lphPalette,
            /* [out] */ __RPC__out BOOL *bStretch) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExtendPropertySheet2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IExtendPropertySheet2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IExtendPropertySheet2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IExtendPropertySheet2 * This);
        
        DECLSPEC_XFGVIRT(IExtendPropertySheet, CreatePropertyPages)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreatePropertyPages )( 
            __RPC__in IExtendPropertySheet2 * This,
            /* [in] */ __RPC__in_opt LPPROPERTYSHEETCALLBACK lpProvider,
            /* [in] */ LONG_PTR handle,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpIDataObject);
        
        DECLSPEC_XFGVIRT(IExtendPropertySheet, QueryPagesFor)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryPagesFor )( 
            __RPC__in IExtendPropertySheet2 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject);
        
        DECLSPEC_XFGVIRT(IExtendPropertySheet2, GetWatermarks)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetWatermarks )( 
            __RPC__in IExtendPropertySheet2 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpIDataObject,
            /* [out] */ __RPC__deref_out_opt HBITMAP *lphWatermark,
            /* [out] */ __RPC__deref_out_opt HBITMAP *lphHeader,
            /* [out] */ __RPC__deref_out_opt HPALETTE *lphPalette,
            /* [out] */ __RPC__out BOOL *bStretch);
        
        END_INTERFACE
    } IExtendPropertySheet2Vtbl;

    interface IExtendPropertySheet2
    {
        CONST_VTBL struct IExtendPropertySheet2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExtendPropertySheet2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExtendPropertySheet2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExtendPropertySheet2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExtendPropertySheet2_CreatePropertyPages(This,lpProvider,handle,lpIDataObject)	\
    ( (This)->lpVtbl -> CreatePropertyPages(This,lpProvider,handle,lpIDataObject) ) 

#define IExtendPropertySheet2_QueryPagesFor(This,lpDataObject)	\
    ( (This)->lpVtbl -> QueryPagesFor(This,lpDataObject) ) 


#define IExtendPropertySheet2_GetWatermarks(This,lpIDataObject,lphWatermark,lphHeader,lphPalette,bStretch)	\
    ( (This)->lpVtbl -> GetWatermarks(This,lpIDataObject,lphWatermark,lphHeader,lphPalette,bStretch) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExtendPropertySheet2_INTERFACE_DEFINED__ */


#ifndef __IHeaderCtrl2_INTERFACE_DEFINED__
#define __IHeaderCtrl2_INTERFACE_DEFINED__

/* interface IHeaderCtrl2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IHeaderCtrl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9757abb8-1b32-11d1-a7ce-00c04fd8d565")
    IHeaderCtrl2 : public IHeaderCtrl
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetChangeTimeOut( 
            /* [in] */ unsigned long uTimeout) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetColumnFilter( 
            /* [in] */ UINT nColumn,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in MMC_FILTERDATA *pFilterData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetColumnFilter( 
            /* [in] */ UINT nColumn,
            /* [out][in] */ __RPC__inout LPDWORD pdwType,
            /* [out][in] */ __RPC__inout MMC_FILTERDATA *pFilterData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHeaderCtrl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHeaderCtrl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHeaderCtrl2 * This);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, InsertColumn)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertColumn )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ int nCol,
            /* [in] */ __RPC__in LPCWSTR title,
            /* [in] */ int nFormat,
            /* [in] */ int nWidth);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, DeleteColumn)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteColumn )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ int nCol);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, SetColumnText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetColumnText )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ int nCol,
            /* [in] */ __RPC__in LPCWSTR title);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, GetColumnText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumnText )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ int nCol,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *pText);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, SetColumnWidth)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetColumnWidth )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ int nCol,
            /* [in] */ int nWidth);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl, GetColumnWidth)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumnWidth )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ int nCol,
            /* [out] */ __RPC__out int *pWidth);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl2, SetChangeTimeOut)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetChangeTimeOut )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ unsigned long uTimeout);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl2, SetColumnFilter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetColumnFilter )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ UINT nColumn,
            /* [in] */ DWORD dwType,
            /* [in] */ __RPC__in MMC_FILTERDATA *pFilterData);
        
        DECLSPEC_XFGVIRT(IHeaderCtrl2, GetColumnFilter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumnFilter )( 
            __RPC__in IHeaderCtrl2 * This,
            /* [in] */ UINT nColumn,
            /* [out][in] */ __RPC__inout LPDWORD pdwType,
            /* [out][in] */ __RPC__inout MMC_FILTERDATA *pFilterData);
        
        END_INTERFACE
    } IHeaderCtrl2Vtbl;

    interface IHeaderCtrl2
    {
        CONST_VTBL struct IHeaderCtrl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHeaderCtrl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHeaderCtrl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHeaderCtrl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHeaderCtrl2_InsertColumn(This,nCol,title,nFormat,nWidth)	\
    ( (This)->lpVtbl -> InsertColumn(This,nCol,title,nFormat,nWidth) ) 

#define IHeaderCtrl2_DeleteColumn(This,nCol)	\
    ( (This)->lpVtbl -> DeleteColumn(This,nCol) ) 

#define IHeaderCtrl2_SetColumnText(This,nCol,title)	\
    ( (This)->lpVtbl -> SetColumnText(This,nCol,title) ) 

#define IHeaderCtrl2_GetColumnText(This,nCol,pText)	\
    ( (This)->lpVtbl -> GetColumnText(This,nCol,pText) ) 

#define IHeaderCtrl2_SetColumnWidth(This,nCol,nWidth)	\
    ( (This)->lpVtbl -> SetColumnWidth(This,nCol,nWidth) ) 

#define IHeaderCtrl2_GetColumnWidth(This,nCol,pWidth)	\
    ( (This)->lpVtbl -> GetColumnWidth(This,nCol,pWidth) ) 


#define IHeaderCtrl2_SetChangeTimeOut(This,uTimeout)	\
    ( (This)->lpVtbl -> SetChangeTimeOut(This,uTimeout) ) 

#define IHeaderCtrl2_SetColumnFilter(This,nColumn,dwType,pFilterData)	\
    ( (This)->lpVtbl -> SetColumnFilter(This,nColumn,dwType,pFilterData) ) 

#define IHeaderCtrl2_GetColumnFilter(This,nColumn,pdwType,pFilterData)	\
    ( (This)->lpVtbl -> GetColumnFilter(This,nColumn,pdwType,pFilterData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHeaderCtrl2_INTERFACE_DEFINED__ */


#ifndef __ISnapinHelp2_INTERFACE_DEFINED__
#define __ISnapinHelp2_INTERFACE_DEFINED__

/* interface ISnapinHelp2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISnapinHelp2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4861A010-20F9-11d2-A510-00C04FB6DD2C")
    ISnapinHelp2 : public ISnapinHelp
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLinkedTopics( 
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpCompiledHelpFiles) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISnapinHelp2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISnapinHelp2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISnapinHelp2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISnapinHelp2 * This);
        
        DECLSPEC_XFGVIRT(ISnapinHelp, GetHelpTopic)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHelpTopic )( 
            __RPC__in ISnapinHelp2 * This,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpCompiledHelpFile);
        
        DECLSPEC_XFGVIRT(ISnapinHelp2, GetLinkedTopics)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLinkedTopics )( 
            __RPC__in ISnapinHelp2 * This,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *lpCompiledHelpFiles);
        
        END_INTERFACE
    } ISnapinHelp2Vtbl;

    interface ISnapinHelp2
    {
        CONST_VTBL struct ISnapinHelp2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISnapinHelp2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISnapinHelp2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISnapinHelp2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISnapinHelp2_GetHelpTopic(This,lpCompiledHelpFile)	\
    ( (This)->lpVtbl -> GetHelpTopic(This,lpCompiledHelpFile) ) 


#define ISnapinHelp2_GetLinkedTopics(This,lpCompiledHelpFiles)	\
    ( (This)->lpVtbl -> GetLinkedTopics(This,lpCompiledHelpFiles) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISnapinHelp2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0026 */
/* [local] */ 

typedef 
enum _MMC_TASK_DISPLAY_TYPE
    {
        MMC_TASK_DISPLAY_UNINITIALIZED	= 0,
        MMC_TASK_DISPLAY_TYPE_SYMBOL	= ( MMC_TASK_DISPLAY_UNINITIALIZED + 1 ) ,
        MMC_TASK_DISPLAY_TYPE_VANILLA_GIF	= ( MMC_TASK_DISPLAY_TYPE_SYMBOL + 1 ) ,
        MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF	= ( MMC_TASK_DISPLAY_TYPE_VANILLA_GIF + 1 ) ,
        MMC_TASK_DISPLAY_TYPE_BITMAP	= ( MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF + 1 ) 
    } 	MMC_TASK_DISPLAY_TYPE;

typedef struct _MMC_TASK_DISPLAY_SYMBOL
    {
    LPOLESTR szFontFamilyName;
    LPOLESTR szURLtoEOT;
    LPOLESTR szSymbolString;
    } 	MMC_TASK_DISPLAY_SYMBOL;

typedef struct _MMC_TASK_DISPLAY_BITMAP
    {
    LPOLESTR szMouseOverBitmap;
    LPOLESTR szMouseOffBitmap;
    } 	MMC_TASK_DISPLAY_BITMAP;

typedef struct _MMC_TASK_DISPLAY_OBJECT
    {
    MMC_TASK_DISPLAY_TYPE eDisplayType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ MMC_TASK_DISPLAY_BITMAP uBitmap;
        /* [case()] */ MMC_TASK_DISPLAY_SYMBOL uSymbol;
        /* [default] */  /* Empty union arm */ 
        } 	;
    } 	MMC_TASK_DISPLAY_OBJECT;

typedef 
enum _MMC_ACTION_TYPE
    {
        MMC_ACTION_UNINITIALIZED	= -1,
        MMC_ACTION_ID	= ( MMC_ACTION_UNINITIALIZED + 1 ) ,
        MMC_ACTION_LINK	= ( MMC_ACTION_ID + 1 ) ,
        MMC_ACTION_SCRIPT	= ( MMC_ACTION_LINK + 1 ) 
    } 	MMC_ACTION_TYPE;

typedef struct _MMC_TASK
    {
    MMC_TASK_DISPLAY_OBJECT sDisplayObject;
    LPOLESTR szText;
    LPOLESTR szHelpString;
    MMC_ACTION_TYPE eActionType;
    union 
        {
        LONG_PTR nCommandID;
        LPOLESTR szActionURL;
        LPOLESTR szScript;
        } 	;
    } 	MMC_TASK;

typedef struct _MMC_LISTPAD_INFO
    {
    LPOLESTR szTitle;
    LPOLESTR szButtonText;
    LONG_PTR nCommandID;
    } 	MMC_LISTPAD_INFO;

typedef DWORD MMC_STRING_ID;



extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0026_v0_0_s_ifspec;

#ifndef __IEnumTASK_INTERFACE_DEFINED__
#define __IEnumTASK_INTERFACE_DEFINED__

/* interface IEnumTASK */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumTASK;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("338698b1-5a02-11d1-9fec-00600832db4a")
    IEnumTASK : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ MMC_TASK *rgelt,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IEnumTASK **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTASKVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumTASK * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumTASK * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumTASK * This);
        
        DECLSPEC_XFGVIRT(IEnumTASK, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumTASK * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ MMC_TASK *rgelt,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumTASK, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumTASK * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumTASK, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumTASK * This);
        
        DECLSPEC_XFGVIRT(IEnumTASK, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumTASK * This,
            /* [out] */ IEnumTASK **ppenum);
        
        END_INTERFACE
    } IEnumTASKVtbl;

    interface IEnumTASK
    {
        CONST_VTBL struct IEnumTASKVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTASK_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTASK_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTASK_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTASK_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumTASK_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumTASK_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTASK_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTASK_INTERFACE_DEFINED__ */


#ifndef __IExtendTaskPad_INTERFACE_DEFINED__
#define __IExtendTaskPad_INTERFACE_DEFINED__

/* interface IExtendTaskPad */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IExtendTaskPad;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8dee6511-554d-11d1-9fea-00600832db4a")
    IExtendTaskPad : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TaskNotify( 
            /* [in] */ IDataObject *pdo,
            /* [in] */ VARIANT *arg,
            /* [in] */ VARIANT *param) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumTasks( 
            /* [in] */ IDataObject *pdo,
            /* [string][in] */ LPOLESTR szTaskGroup,
            /* [out] */ IEnumTASK **ppEnumTASK) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTitle( 
            /* [string][in] */ LPOLESTR pszGroup,
            /* [string][out] */ LPOLESTR *pszTitle) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDescriptiveText( 
            /* [string][in] */ LPOLESTR pszGroup,
            /* [string][out] */ LPOLESTR *pszDescriptiveText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetBackground( 
            /* [string][in] */ LPOLESTR pszGroup,
            /* [out] */ MMC_TASK_DISPLAY_OBJECT *pTDO) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetListPadInfo( 
            /* [string][in] */ LPOLESTR pszGroup,
            /* [out] */ MMC_LISTPAD_INFO *lpListPadInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExtendTaskPadVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IExtendTaskPad * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IExtendTaskPad * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IExtendTaskPad * This);
        
        DECLSPEC_XFGVIRT(IExtendTaskPad, TaskNotify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TaskNotify )( 
            IExtendTaskPad * This,
            /* [in] */ IDataObject *pdo,
            /* [in] */ VARIANT *arg,
            /* [in] */ VARIANT *param);
        
        DECLSPEC_XFGVIRT(IExtendTaskPad, EnumTasks)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumTasks )( 
            IExtendTaskPad * This,
            /* [in] */ IDataObject *pdo,
            /* [string][in] */ LPOLESTR szTaskGroup,
            /* [out] */ IEnumTASK **ppEnumTASK);
        
        DECLSPEC_XFGVIRT(IExtendTaskPad, GetTitle)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            IExtendTaskPad * This,
            /* [string][in] */ LPOLESTR pszGroup,
            /* [string][out] */ LPOLESTR *pszTitle);
        
        DECLSPEC_XFGVIRT(IExtendTaskPad, GetDescriptiveText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDescriptiveText )( 
            IExtendTaskPad * This,
            /* [string][in] */ LPOLESTR pszGroup,
            /* [string][out] */ LPOLESTR *pszDescriptiveText);
        
        DECLSPEC_XFGVIRT(IExtendTaskPad, GetBackground)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetBackground )( 
            IExtendTaskPad * This,
            /* [string][in] */ LPOLESTR pszGroup,
            /* [out] */ MMC_TASK_DISPLAY_OBJECT *pTDO);
        
        DECLSPEC_XFGVIRT(IExtendTaskPad, GetListPadInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetListPadInfo )( 
            IExtendTaskPad * This,
            /* [string][in] */ LPOLESTR pszGroup,
            /* [out] */ MMC_LISTPAD_INFO *lpListPadInfo);
        
        END_INTERFACE
    } IExtendTaskPadVtbl;

    interface IExtendTaskPad
    {
        CONST_VTBL struct IExtendTaskPadVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExtendTaskPad_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExtendTaskPad_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExtendTaskPad_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExtendTaskPad_TaskNotify(This,pdo,arg,param)	\
    ( (This)->lpVtbl -> TaskNotify(This,pdo,arg,param) ) 

#define IExtendTaskPad_EnumTasks(This,pdo,szTaskGroup,ppEnumTASK)	\
    ( (This)->lpVtbl -> EnumTasks(This,pdo,szTaskGroup,ppEnumTASK) ) 

#define IExtendTaskPad_GetTitle(This,pszGroup,pszTitle)	\
    ( (This)->lpVtbl -> GetTitle(This,pszGroup,pszTitle) ) 

#define IExtendTaskPad_GetDescriptiveText(This,pszGroup,pszDescriptiveText)	\
    ( (This)->lpVtbl -> GetDescriptiveText(This,pszGroup,pszDescriptiveText) ) 

#define IExtendTaskPad_GetBackground(This,pszGroup,pTDO)	\
    ( (This)->lpVtbl -> GetBackground(This,pszGroup,pTDO) ) 

#define IExtendTaskPad_GetListPadInfo(This,pszGroup,lpListPadInfo)	\
    ( (This)->lpVtbl -> GetListPadInfo(This,pszGroup,lpListPadInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExtendTaskPad_INTERFACE_DEFINED__ */


#ifndef __IConsole2_INTERFACE_DEFINED__
#define __IConsole2_INTERFACE_DEFINED__

/* interface IConsole2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConsole2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("103D842A-AA63-11D1-A7E1-00C04FD8D565")
    IConsole2 : public IConsole
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Expand( 
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ BOOL bExpand) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE IsTaskpadViewPreferred( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetStatusText( 
            /* [string][in] */ __RPC__in_string LPOLESTR pszStatusText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsole2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsole2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsole2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsole2 * This);
        
        DECLSPEC_XFGVIRT(IConsole, SetHeader)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetHeader )( 
            __RPC__in IConsole2 * This,
            /* [in] */ __RPC__in_opt LPHEADERCTRL pHeader);
        
        DECLSPEC_XFGVIRT(IConsole, SetToolbar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetToolbar )( 
            __RPC__in IConsole2 * This,
            /* [in] */ __RPC__in_opt LPTOOLBAR pToolbar);
        
        DECLSPEC_XFGVIRT(IConsole, QueryResultView)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryResultView )( 
            __RPC__in IConsole2 * This,
            /* [out] */ __RPC__deref_out_opt LPUNKNOWN *pUnknown);
        
        DECLSPEC_XFGVIRT(IConsole, QueryScopeImageList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryScopeImageList )( 
            __RPC__in IConsole2 * This,
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList);
        
        DECLSPEC_XFGVIRT(IConsole, QueryResultImageList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryResultImageList )( 
            __RPC__in IConsole2 * This,
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList);
        
        DECLSPEC_XFGVIRT(IConsole, UpdateAllViews)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateAllViews )( 
            __RPC__in IConsole2 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ LPARAM data,
            /* [in] */ LONG_PTR hint);
        
        DECLSPEC_XFGVIRT(IConsole, MessageBox)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MessageBox )( 
            __RPC__in IConsole2 * This,
            /* [in] */ __RPC__in LPCWSTR lpszText,
            /* [in] */ __RPC__in LPCWSTR lpszTitle,
            /* [in] */ UINT fuStyle,
            /* [out] */ __RPC__out int *piRetval);
        
        DECLSPEC_XFGVIRT(IConsole, QueryConsoleVerb)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryConsoleVerb )( 
            __RPC__in IConsole2 * This,
            /* [out] */ __RPC__deref_out_opt LPCONSOLEVERB *ppConsoleVerb);
        
        DECLSPEC_XFGVIRT(IConsole, SelectScopeItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectScopeItem )( 
            __RPC__in IConsole2 * This,
            /* [in] */ HSCOPEITEM hScopeItem);
        
        DECLSPEC_XFGVIRT(IConsole, GetMainWindow)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMainWindow )( 
            __RPC__in IConsole2 * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IConsole, NewWindow)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NewWindow )( 
            __RPC__in IConsole2 * This,
            /* [in] */ HSCOPEITEM hScopeItem,
            /* [in] */ unsigned long lOptions);
        
        DECLSPEC_XFGVIRT(IConsole2, Expand)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in IConsole2 * This,
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ BOOL bExpand);
        
        DECLSPEC_XFGVIRT(IConsole2, IsTaskpadViewPreferred)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsTaskpadViewPreferred )( 
            __RPC__in IConsole2 * This);
        
        DECLSPEC_XFGVIRT(IConsole2, SetStatusText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatusText )( 
            __RPC__in IConsole2 * This,
            /* [string][in] */ __RPC__in_string LPOLESTR pszStatusText);
        
        END_INTERFACE
    } IConsole2Vtbl;

    interface IConsole2
    {
        CONST_VTBL struct IConsole2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsole2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsole2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsole2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsole2_SetHeader(This,pHeader)	\
    ( (This)->lpVtbl -> SetHeader(This,pHeader) ) 

#define IConsole2_SetToolbar(This,pToolbar)	\
    ( (This)->lpVtbl -> SetToolbar(This,pToolbar) ) 

#define IConsole2_QueryResultView(This,pUnknown)	\
    ( (This)->lpVtbl -> QueryResultView(This,pUnknown) ) 

#define IConsole2_QueryScopeImageList(This,ppImageList)	\
    ( (This)->lpVtbl -> QueryScopeImageList(This,ppImageList) ) 

#define IConsole2_QueryResultImageList(This,ppImageList)	\
    ( (This)->lpVtbl -> QueryResultImageList(This,ppImageList) ) 

#define IConsole2_UpdateAllViews(This,lpDataObject,data,hint)	\
    ( (This)->lpVtbl -> UpdateAllViews(This,lpDataObject,data,hint) ) 

#define IConsole2_MessageBox(This,lpszText,lpszTitle,fuStyle,piRetval)	\
    ( (This)->lpVtbl -> MessageBox(This,lpszText,lpszTitle,fuStyle,piRetval) ) 

#define IConsole2_QueryConsoleVerb(This,ppConsoleVerb)	\
    ( (This)->lpVtbl -> QueryConsoleVerb(This,ppConsoleVerb) ) 

#define IConsole2_SelectScopeItem(This,hScopeItem)	\
    ( (This)->lpVtbl -> SelectScopeItem(This,hScopeItem) ) 

#define IConsole2_GetMainWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetMainWindow(This,phwnd) ) 

#define IConsole2_NewWindow(This,hScopeItem,lOptions)	\
    ( (This)->lpVtbl -> NewWindow(This,hScopeItem,lOptions) ) 


#define IConsole2_Expand(This,hItem,bExpand)	\
    ( (This)->lpVtbl -> Expand(This,hItem,bExpand) ) 

#define IConsole2_IsTaskpadViewPreferred(This)	\
    ( (This)->lpVtbl -> IsTaskpadViewPreferred(This) ) 

#define IConsole2_SetStatusText(This,pszStatusText)	\
    ( (This)->lpVtbl -> SetStatusText(This,pszStatusText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsole2_INTERFACE_DEFINED__ */


#ifndef __IDisplayHelp_INTERFACE_DEFINED__
#define __IDisplayHelp_INTERFACE_DEFINED__

/* interface IDisplayHelp */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDisplayHelp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cc593830-b926-11d1-8063-0000f875a9ce")
    IDisplayHelp : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ShowTopic( 
            /* [in] */ __RPC__in LPOLESTR pszHelpTopic) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDisplayHelpVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDisplayHelp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDisplayHelp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDisplayHelp * This);
        
        DECLSPEC_XFGVIRT(IDisplayHelp, ShowTopic)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ShowTopic )( 
            __RPC__in IDisplayHelp * This,
            /* [in] */ __RPC__in LPOLESTR pszHelpTopic);
        
        END_INTERFACE
    } IDisplayHelpVtbl;

    interface IDisplayHelp
    {
        CONST_VTBL struct IDisplayHelpVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDisplayHelp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDisplayHelp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDisplayHelp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDisplayHelp_ShowTopic(This,pszHelpTopic)	\
    ( (This)->lpVtbl -> ShowTopic(This,pszHelpTopic) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDisplayHelp_INTERFACE_DEFINED__ */


#ifndef __IRequiredExtensions_INTERFACE_DEFINED__
#define __IRequiredExtensions_INTERFACE_DEFINED__

/* interface IRequiredExtensions */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IRequiredExtensions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72782D7A-A4A0-11d1-AF0F-00C04FB6DD2C")
    IRequiredExtensions : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnableAllExtensions( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetFirstExtension( 
            /* [out] */ LPCLSID pExtCLSID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNextExtension( 
            /* [out] */ LPCLSID pExtCLSID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRequiredExtensionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRequiredExtensions * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRequiredExtensions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRequiredExtensions * This);
        
        DECLSPEC_XFGVIRT(IRequiredExtensions, EnableAllExtensions)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnableAllExtensions )( 
            IRequiredExtensions * This);
        
        DECLSPEC_XFGVIRT(IRequiredExtensions, GetFirstExtension)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFirstExtension )( 
            IRequiredExtensions * This,
            /* [out] */ LPCLSID pExtCLSID);
        
        DECLSPEC_XFGVIRT(IRequiredExtensions, GetNextExtension)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNextExtension )( 
            IRequiredExtensions * This,
            /* [out] */ LPCLSID pExtCLSID);
        
        END_INTERFACE
    } IRequiredExtensionsVtbl;

    interface IRequiredExtensions
    {
        CONST_VTBL struct IRequiredExtensionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRequiredExtensions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRequiredExtensions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRequiredExtensions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRequiredExtensions_EnableAllExtensions(This)	\
    ( (This)->lpVtbl -> EnableAllExtensions(This) ) 

#define IRequiredExtensions_GetFirstExtension(This,pExtCLSID)	\
    ( (This)->lpVtbl -> GetFirstExtension(This,pExtCLSID) ) 

#define IRequiredExtensions_GetNextExtension(This,pExtCLSID)	\
    ( (This)->lpVtbl -> GetNextExtension(This,pExtCLSID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRequiredExtensions_INTERFACE_DEFINED__ */


#ifndef __IStringTable_INTERFACE_DEFINED__
#define __IStringTable_INTERFACE_DEFINED__

/* interface IStringTable */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IStringTable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DE40B7A4-0F65-11d2-8E25-00C04F8ECD78")
    IStringTable : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddString( 
            /* [in] */ __RPC__in LPCOLESTR pszAdd,
            /* [out] */ __RPC__out MMC_STRING_ID *pStringID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetString( 
            /* [in] */ MMC_STRING_ID StringID,
            /* [in] */ ULONG cchBuffer,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBuffer) LPOLESTR lpBuffer,
            /* [out] */ __RPC__out ULONG *pcchOut) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStringLength( 
            /* [in] */ MMC_STRING_ID StringID,
            /* [out] */ __RPC__out ULONG *pcchString) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteString( 
            /* [in] */ MMC_STRING_ID StringID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DeleteAllStrings( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindString( 
            /* [in] */ __RPC__in LPCOLESTR pszFind,
            /* [out] */ __RPC__out MMC_STRING_ID *pStringID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Enumerate( 
            /* [out] */ __RPC__deref_out_opt IEnumString **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStringTableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStringTable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStringTable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStringTable * This);
        
        DECLSPEC_XFGVIRT(IStringTable, AddString)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddString )( 
            __RPC__in IStringTable * This,
            /* [in] */ __RPC__in LPCOLESTR pszAdd,
            /* [out] */ __RPC__out MMC_STRING_ID *pStringID);
        
        DECLSPEC_XFGVIRT(IStringTable, GetString)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IStringTable * This,
            /* [in] */ MMC_STRING_ID StringID,
            /* [in] */ ULONG cchBuffer,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBuffer) LPOLESTR lpBuffer,
            /* [out] */ __RPC__out ULONG *pcchOut);
        
        DECLSPEC_XFGVIRT(IStringTable, GetStringLength)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IStringTable * This,
            /* [in] */ MMC_STRING_ID StringID,
            /* [out] */ __RPC__out ULONG *pcchString);
        
        DECLSPEC_XFGVIRT(IStringTable, DeleteString)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteString )( 
            __RPC__in IStringTable * This,
            /* [in] */ MMC_STRING_ID StringID);
        
        DECLSPEC_XFGVIRT(IStringTable, DeleteAllStrings)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteAllStrings )( 
            __RPC__in IStringTable * This);
        
        DECLSPEC_XFGVIRT(IStringTable, FindString)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindString )( 
            __RPC__in IStringTable * This,
            /* [in] */ __RPC__in LPCOLESTR pszFind,
            /* [out] */ __RPC__out MMC_STRING_ID *pStringID);
        
        DECLSPEC_XFGVIRT(IStringTable, Enumerate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Enumerate )( 
            __RPC__in IStringTable * This,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppEnum);
        
        END_INTERFACE
    } IStringTableVtbl;

    interface IStringTable
    {
        CONST_VTBL struct IStringTableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStringTable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStringTable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStringTable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStringTable_AddString(This,pszAdd,pStringID)	\
    ( (This)->lpVtbl -> AddString(This,pszAdd,pStringID) ) 

#define IStringTable_GetString(This,StringID,cchBuffer,lpBuffer,pcchOut)	\
    ( (This)->lpVtbl -> GetString(This,StringID,cchBuffer,lpBuffer,pcchOut) ) 

#define IStringTable_GetStringLength(This,StringID,pcchString)	\
    ( (This)->lpVtbl -> GetStringLength(This,StringID,pcchString) ) 

#define IStringTable_DeleteString(This,StringID)	\
    ( (This)->lpVtbl -> DeleteString(This,StringID) ) 

#define IStringTable_DeleteAllStrings(This)	\
    ( (This)->lpVtbl -> DeleteAllStrings(This) ) 

#define IStringTable_FindString(This,pszFind,pStringID)	\
    ( (This)->lpVtbl -> FindString(This,pszFind,pStringID) ) 

#define IStringTable_Enumerate(This,ppEnum)	\
    ( (This)->lpVtbl -> Enumerate(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStringTable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0032 */
/* [local] */ 

#endif // MMC_VER >= 0x0110
#if (MMC_VER >= 0x0120)
#define	HDI_HIDDEN	( 0x1 )

typedef struct _MMC_COLUMN_DATA
    {
    int nColIndex;
    DWORD dwFlags;
    int nWidth;
    ULONG_PTR ulReserved;
    } 	MMC_COLUMN_DATA;

typedef struct _MMC_COLUMN_SET_DATA
    {
    int cbSize;
    int nNumCols;
    MMC_COLUMN_DATA *pColData;
    } 	MMC_COLUMN_SET_DATA;

typedef struct _MMC_SORT_DATA
    {
    int nColIndex;
    DWORD dwSortOptions;
    ULONG_PTR ulReserved;
    } 	MMC_SORT_DATA;

typedef struct _MMC_SORT_SET_DATA
    {
    int cbSize;
    int nNumItems;
    MMC_SORT_DATA *pSortData;
    } 	MMC_SORT_SET_DATA;



extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0032_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0032_v0_0_s_ifspec;

#ifndef __IColumnData_INTERFACE_DEFINED__
#define __IColumnData_INTERFACE_DEFINED__

/* interface IColumnData */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IColumnData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("547C1354-024D-11d3-A707-00C04F8EF4CB")
    IColumnData : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetColumnConfigData( 
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [in] */ __RPC__in MMC_COLUMN_SET_DATA *pColSetData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetColumnConfigData( 
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [out] */ __RPC__deref_out_opt MMC_COLUMN_SET_DATA **ppColSetData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetColumnSortData( 
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [in] */ __RPC__in MMC_SORT_SET_DATA *pColSortData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetColumnSortData( 
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [out] */ __RPC__deref_out_opt MMC_SORT_SET_DATA **ppColSortData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IColumnDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IColumnData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IColumnData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IColumnData * This);
        
        DECLSPEC_XFGVIRT(IColumnData, SetColumnConfigData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetColumnConfigData )( 
            __RPC__in IColumnData * This,
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [in] */ __RPC__in MMC_COLUMN_SET_DATA *pColSetData);
        
        DECLSPEC_XFGVIRT(IColumnData, GetColumnConfigData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumnConfigData )( 
            __RPC__in IColumnData * This,
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [out] */ __RPC__deref_out_opt MMC_COLUMN_SET_DATA **ppColSetData);
        
        DECLSPEC_XFGVIRT(IColumnData, SetColumnSortData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetColumnSortData )( 
            __RPC__in IColumnData * This,
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [in] */ __RPC__in MMC_SORT_SET_DATA *pColSortData);
        
        DECLSPEC_XFGVIRT(IColumnData, GetColumnSortData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetColumnSortData )( 
            __RPC__in IColumnData * This,
            /* [in] */ __RPC__in SColumnSetID *pColID,
            /* [out] */ __RPC__deref_out_opt MMC_SORT_SET_DATA **ppColSortData);
        
        END_INTERFACE
    } IColumnDataVtbl;

    interface IColumnData
    {
        CONST_VTBL struct IColumnDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IColumnData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IColumnData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IColumnData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IColumnData_SetColumnConfigData(This,pColID,pColSetData)	\
    ( (This)->lpVtbl -> SetColumnConfigData(This,pColID,pColSetData) ) 

#define IColumnData_GetColumnConfigData(This,pColID,ppColSetData)	\
    ( (This)->lpVtbl -> GetColumnConfigData(This,pColID,ppColSetData) ) 

#define IColumnData_SetColumnSortData(This,pColID,pColSortData)	\
    ( (This)->lpVtbl -> SetColumnSortData(This,pColID,pColSortData) ) 

#define IColumnData_GetColumnSortData(This,pColID,ppColSortData)	\
    ( (This)->lpVtbl -> GetColumnSortData(This,pColID,ppColSortData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IColumnData_INTERFACE_DEFINED__ */


#ifndef __IMessageView_INTERFACE_DEFINED__
#define __IMessageView_INTERFACE_DEFINED__

/* interface IMessageView */
/* [unique][helpstring][uuid][object] */ 

typedef 
enum tagIconIdentifier
    {
        Icon_None	= 0,
        Icon_Error	= 32513,
        Icon_Question	= 32514,
        Icon_Warning	= 32515,
        Icon_Information	= 32516,
        Icon_First	= Icon_Error,
        Icon_Last	= Icon_Information
    } 	IconIdentifier;


EXTERN_C const IID IID_IMessageView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("80F94174-FCCC-11d2-B991-00C04F8ECD78")
    IMessageView : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTitleText( 
            /* [in] */ __RPC__in LPCOLESTR pszTitleText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetBodyText( 
            /* [in] */ __RPC__in LPCOLESTR pszBodyText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetIcon( 
            /* [in] */ IconIdentifier id) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMessageViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMessageView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMessageView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMessageView * This);
        
        DECLSPEC_XFGVIRT(IMessageView, SetTitleText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTitleText )( 
            __RPC__in IMessageView * This,
            /* [in] */ __RPC__in LPCOLESTR pszTitleText);
        
        DECLSPEC_XFGVIRT(IMessageView, SetBodyText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetBodyText )( 
            __RPC__in IMessageView * This,
            /* [in] */ __RPC__in LPCOLESTR pszBodyText);
        
        DECLSPEC_XFGVIRT(IMessageView, SetIcon)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIcon )( 
            __RPC__in IMessageView * This,
            /* [in] */ IconIdentifier id);
        
        DECLSPEC_XFGVIRT(IMessageView, Clear)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IMessageView * This);
        
        END_INTERFACE
    } IMessageViewVtbl;

    interface IMessageView
    {
        CONST_VTBL struct IMessageViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMessageView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMessageView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMessageView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMessageView_SetTitleText(This,pszTitleText)	\
    ( (This)->lpVtbl -> SetTitleText(This,pszTitleText) ) 

#define IMessageView_SetBodyText(This,pszBodyText)	\
    ( (This)->lpVtbl -> SetBodyText(This,pszBodyText) ) 

#define IMessageView_SetIcon(This,id)	\
    ( (This)->lpVtbl -> SetIcon(This,id) ) 

#define IMessageView_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMessageView_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0034 */
/* [local] */ 

typedef struct _RDCITEMHDR
    {
    DWORD dwFlags;
    MMC_COOKIE cookie;
    LPARAM lpReserved;
    } 	RDITEMHDR;

#define	RDCI_ScopeItem	( 0x80000000 )

typedef struct _RDCOMPARE
    {
    DWORD cbSize;
    DWORD dwFlags;
    int nColumn;
    LPARAM lUserParam;
    RDITEMHDR *prdch1;
    RDITEMHDR *prdch2;
    } 	RDCOMPARE;



extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0034_v0_0_s_ifspec;

#ifndef __IResultDataCompareEx_INTERFACE_DEFINED__
#define __IResultDataCompareEx_INTERFACE_DEFINED__

/* interface IResultDataCompareEx */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IResultDataCompareEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("96933476-0251-11d3-AEB0-00C04F8ECD78")
    IResultDataCompareEx : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Compare( 
            /* [in] */ __RPC__in RDCOMPARE *prdc,
            /* [out] */ __RPC__out int *pnResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResultDataCompareExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResultDataCompareEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResultDataCompareEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResultDataCompareEx * This);
        
        DECLSPEC_XFGVIRT(IResultDataCompareEx, Compare)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IResultDataCompareEx * This,
            /* [in] */ __RPC__in RDCOMPARE *prdc,
            /* [out] */ __RPC__out int *pnResult);
        
        END_INTERFACE
    } IResultDataCompareExVtbl;

    interface IResultDataCompareEx
    {
        CONST_VTBL struct IResultDataCompareExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResultDataCompareEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResultDataCompareEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResultDataCompareEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResultDataCompareEx_Compare(This,prdc,pnResult)	\
    ( (This)->lpVtbl -> Compare(This,prdc,pnResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResultDataCompareEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0035 */
/* [local] */ 

#endif // MMC_VER >= 0x0120
#if (MMC_VER >= 0x0200)
typedef 
enum _MMC_VIEW_TYPE
    {
        MMC_VIEW_TYPE_LIST	= 0,
        MMC_VIEW_TYPE_HTML	= ( MMC_VIEW_TYPE_LIST + 1 ) ,
        MMC_VIEW_TYPE_OCX	= ( MMC_VIEW_TYPE_HTML + 1 ) 
    } 	MMC_VIEW_TYPE;

#define	RVTI_MISC_OPTIONS_NOLISTVIEWS	( 0x1 )

#define	RVTI_LIST_OPTIONS_NONE	( 0 )

#define	RVTI_LIST_OPTIONS_OWNERDATALIST	( 0x2 )

#define	RVTI_LIST_OPTIONS_MULTISELECT	( 0x4 )

#define	RVTI_LIST_OPTIONS_FILTERED	( 0x8 )

#define	RVTI_LIST_OPTIONS_USEFONTLINKING	( 0x20 )

#define	RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST	( 0x40 )

#define	RVTI_LIST_OPTIONS_LEXICAL_SORT	( 0x80 )

#define	RVTI_LIST_OPTIONS_ALLOWPASTE	( 0x100 )

#define	RVTI_HTML_OPTIONS_NONE	( 0 )

#define	RVTI_HTML_OPTIONS_NOLISTVIEW	( 0x1 )

#define	RVTI_OCX_OPTIONS_NONE	( 0 )

#define	RVTI_OCX_OPTIONS_NOLISTVIEW	( 0x1 )

#define	RVTI_OCX_OPTIONS_CACHE_OCX	( 0x2 )

typedef struct _RESULT_VIEW_TYPE_INFO
    {
    LPOLESTR pstrPersistableViewDescription;
    MMC_VIEW_TYPE eViewType;
    DWORD dwMiscOptions;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ DWORD dwListOptions;
        /* [case()] */ struct 
            {
            DWORD dwHTMLOptions;
            LPOLESTR pstrURL;
            } 	;
        /* [case()] */ struct 
            {
            DWORD dwOCXOptions;
            LPUNKNOWN pUnkControl;
            } 	;
        /* [default] */  /* Empty union arm */ 
        } 	;
    } 	RESULT_VIEW_TYPE_INFO;

typedef struct _RESULT_VIEW_TYPE_INFO *PRESULT_VIEW_TYPE_INFO;

#define	CCF_DESCRIPTION	( L"CCF_DESCRIPTION" )

#define	CCF_HTML_DETAILS	( L"CCF_HTML_DETAILS" )

typedef struct _CONTEXTMENUITEM2
    {
    LPWSTR strName;
    LPWSTR strStatusBarText;
    LONG lCommandID;
    LONG lInsertionPointID;
    LONG fFlags;
    LONG fSpecialFlags;
    LPWSTR strLanguageIndependentName;
    } 	CONTEXTMENUITEM2;

typedef CONTEXTMENUITEM2 *LPCONTEXTMENUITEM2;

typedef struct _MMC_EXT_VIEW_DATA
    {
    GUID viewID;
    LPCOLESTR pszURL;
    LPCOLESTR pszViewTitle;
    LPCOLESTR pszTooltipText;
    BOOL bReplacesDefaultView;
    } 	MMC_EXT_VIEW_DATA;

typedef struct _MMC_EXT_VIEW_DATA *PMMC_EXT_VIEW_DATA;

#define	MMC_DEFAULT_OPERATION_COPY	( 0x1 )



extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0035_v0_0_s_ifspec;

#ifndef __IComponentData2_INTERFACE_DEFINED__
#define __IComponentData2_INTERFACE_DEFINED__

/* interface IComponentData2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IComponentData2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CCA0F2D2-82DE-41B5-BF47-3B2076273D5C")
    IComponentData2 : public IComponentData
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryDispatch( 
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDISPATCH *ppDispatch) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentData2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponentData2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponentData2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponentData2 * This);
        
        DECLSPEC_XFGVIRT(IComponentData, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IComponentData2 * This,
            /* [in] */ __RPC__in_opt LPUNKNOWN pUnknown);
        
        DECLSPEC_XFGVIRT(IComponentData, CreateComponent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateComponent )( 
            __RPC__in IComponentData2 * This,
            /* [out] */ __RPC__deref_out_opt LPCOMPONENT *ppComponent);
        
        DECLSPEC_XFGVIRT(IComponentData, Notify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IComponentData2 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param);
        
        DECLSPEC_XFGVIRT(IComponentData, Destroy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            __RPC__in IComponentData2 * This);
        
        DECLSPEC_XFGVIRT(IComponentData, QueryDataObject)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDataObject )( 
            __RPC__in IComponentData2 * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDATAOBJECT *ppDataObject);
        
        DECLSPEC_XFGVIRT(IComponentData, GetDisplayInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDisplayInfo )( 
            __RPC__in IComponentData2 * This,
            /* [out][in] */ __RPC__inout SCOPEDATAITEM *pScopeDataItem);
        
        DECLSPEC_XFGVIRT(IComponentData, CompareObjects)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CompareObjects )( 
            __RPC__in IComponentData2 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectA,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectB);
        
        DECLSPEC_XFGVIRT(IComponentData2, QueryDispatch)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDispatch )( 
            __RPC__in IComponentData2 * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDISPATCH *ppDispatch);
        
        END_INTERFACE
    } IComponentData2Vtbl;

    interface IComponentData2
    {
        CONST_VTBL struct IComponentData2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponentData2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponentData2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponentData2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponentData2_Initialize(This,pUnknown)	\
    ( (This)->lpVtbl -> Initialize(This,pUnknown) ) 

#define IComponentData2_CreateComponent(This,ppComponent)	\
    ( (This)->lpVtbl -> CreateComponent(This,ppComponent) ) 

#define IComponentData2_Notify(This,lpDataObject,event,arg,param)	\
    ( (This)->lpVtbl -> Notify(This,lpDataObject,event,arg,param) ) 

#define IComponentData2_Destroy(This)	\
    ( (This)->lpVtbl -> Destroy(This) ) 

#define IComponentData2_QueryDataObject(This,cookie,type,ppDataObject)	\
    ( (This)->lpVtbl -> QueryDataObject(This,cookie,type,ppDataObject) ) 

#define IComponentData2_GetDisplayInfo(This,pScopeDataItem)	\
    ( (This)->lpVtbl -> GetDisplayInfo(This,pScopeDataItem) ) 

#define IComponentData2_CompareObjects(This,lpDataObjectA,lpDataObjectB)	\
    ( (This)->lpVtbl -> CompareObjects(This,lpDataObjectA,lpDataObjectB) ) 


#define IComponentData2_QueryDispatch(This,cookie,type,ppDispatch)	\
    ( (This)->lpVtbl -> QueryDispatch(This,cookie,type,ppDispatch) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponentData2_INTERFACE_DEFINED__ */


#ifndef __IComponent2_INTERFACE_DEFINED__
#define __IComponent2_INTERFACE_DEFINED__

/* interface IComponent2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IComponent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79A2D615-4A10-4ED4-8C65-8633F9335095")
    IComponent2 : public IComponent
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryDispatch( 
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDISPATCH *ppDispatch) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetResultViewType2( 
            /* [in] */ MMC_COOKIE cookie,
            /* [out][in] */ __RPC__inout PRESULT_VIEW_TYPE_INFO pResultViewType) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RestoreResultView( 
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ __RPC__in PRESULT_VIEW_TYPE_INFO pResultViewType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponent2 * This);
        
        DECLSPEC_XFGVIRT(IComponent, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IComponent2 * This,
            /* [in] */ __RPC__in_opt LPCONSOLE lpConsole);
        
        DECLSPEC_XFGVIRT(IComponent, Notify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in IComponent2 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ MMC_NOTIFY_TYPE event,
            /* [in] */ LPARAM arg,
            /* [in] */ LPARAM param);
        
        DECLSPEC_XFGVIRT(IComponent, Destroy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            __RPC__in IComponent2 * This,
            /* [in] */ MMC_COOKIE cookie);
        
        DECLSPEC_XFGVIRT(IComponent, QueryDataObject)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDataObject )( 
            __RPC__in IComponent2 * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDATAOBJECT *ppDataObject);
        
        DECLSPEC_XFGVIRT(IComponent, GetResultViewType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetResultViewType )( 
            __RPC__in IComponent2 * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *ppViewType,
            /* [out] */ __RPC__out long *pViewOptions);
        
        DECLSPEC_XFGVIRT(IComponent, GetDisplayInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDisplayInfo )( 
            __RPC__in IComponent2 * This,
            /* [out][in] */ __RPC__inout RESULTDATAITEM *pResultDataItem);
        
        DECLSPEC_XFGVIRT(IComponent, CompareObjects)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CompareObjects )( 
            __RPC__in IComponent2 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectA,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObjectB);
        
        DECLSPEC_XFGVIRT(IComponent2, QueryDispatch)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryDispatch )( 
            __RPC__in IComponent2 * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ DATA_OBJECT_TYPES type,
            /* [out] */ __RPC__deref_out_opt LPDISPATCH *ppDispatch);
        
        DECLSPEC_XFGVIRT(IComponent2, GetResultViewType2)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetResultViewType2 )( 
            __RPC__in IComponent2 * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [out][in] */ __RPC__inout PRESULT_VIEW_TYPE_INFO pResultViewType);
        
        DECLSPEC_XFGVIRT(IComponent2, RestoreResultView)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RestoreResultView )( 
            __RPC__in IComponent2 * This,
            /* [in] */ MMC_COOKIE cookie,
            /* [in] */ __RPC__in PRESULT_VIEW_TYPE_INFO pResultViewType);
        
        END_INTERFACE
    } IComponent2Vtbl;

    interface IComponent2
    {
        CONST_VTBL struct IComponent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponent2_Initialize(This,lpConsole)	\
    ( (This)->lpVtbl -> Initialize(This,lpConsole) ) 

#define IComponent2_Notify(This,lpDataObject,event,arg,param)	\
    ( (This)->lpVtbl -> Notify(This,lpDataObject,event,arg,param) ) 

#define IComponent2_Destroy(This,cookie)	\
    ( (This)->lpVtbl -> Destroy(This,cookie) ) 

#define IComponent2_QueryDataObject(This,cookie,type,ppDataObject)	\
    ( (This)->lpVtbl -> QueryDataObject(This,cookie,type,ppDataObject) ) 

#define IComponent2_GetResultViewType(This,cookie,ppViewType,pViewOptions)	\
    ( (This)->lpVtbl -> GetResultViewType(This,cookie,ppViewType,pViewOptions) ) 

#define IComponent2_GetDisplayInfo(This,pResultDataItem)	\
    ( (This)->lpVtbl -> GetDisplayInfo(This,pResultDataItem) ) 

#define IComponent2_CompareObjects(This,lpDataObjectA,lpDataObjectB)	\
    ( (This)->lpVtbl -> CompareObjects(This,lpDataObjectA,lpDataObjectB) ) 


#define IComponent2_QueryDispatch(This,cookie,type,ppDispatch)	\
    ( (This)->lpVtbl -> QueryDispatch(This,cookie,type,ppDispatch) ) 

#define IComponent2_GetResultViewType2(This,cookie,pResultViewType)	\
    ( (This)->lpVtbl -> GetResultViewType2(This,cookie,pResultViewType) ) 

#define IComponent2_RestoreResultView(This,cookie,pResultViewType)	\
    ( (This)->lpVtbl -> RestoreResultView(This,cookie,pResultViewType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponent2_INTERFACE_DEFINED__ */


#ifndef __IContextMenuCallback2_INTERFACE_DEFINED__
#define __IContextMenuCallback2_INTERFACE_DEFINED__

/* interface IContextMenuCallback2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IContextMenuCallback2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E178BC0E-2ED0-4b5e-8097-42C9087E8B33")
    IContextMenuCallback2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in CONTEXTMENUITEM2 *pItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextMenuCallback2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContextMenuCallback2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContextMenuCallback2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContextMenuCallback2 * This);
        
        DECLSPEC_XFGVIRT(IContextMenuCallback2, AddItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in IContextMenuCallback2 * This,
            /* [in] */ __RPC__in CONTEXTMENUITEM2 *pItem);
        
        END_INTERFACE
    } IContextMenuCallback2Vtbl;

    interface IContextMenuCallback2
    {
        CONST_VTBL struct IContextMenuCallback2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContextMenuCallback2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContextMenuCallback2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContextMenuCallback2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContextMenuCallback2_AddItem(This,pItem)	\
    ( (This)->lpVtbl -> AddItem(This,pItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContextMenuCallback2_INTERFACE_DEFINED__ */


#ifndef __IMMCVersionInfo_INTERFACE_DEFINED__
#define __IMMCVersionInfo_INTERFACE_DEFINED__

/* interface IMMCVersionInfo */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IMMCVersionInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A8D2C5FE-CDCB-4b9d-BDE5-A27343FF54BC")
    IMMCVersionInfo : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMMCVersion( 
            /* [out] */ __RPC__out long *pVersionMajor,
            /* [out] */ __RPC__out long *pVersionMinor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMMCVersionInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMMCVersionInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMMCVersionInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMMCVersionInfo * This);
        
        DECLSPEC_XFGVIRT(IMMCVersionInfo, GetMMCVersion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMMCVersion )( 
            __RPC__in IMMCVersionInfo * This,
            /* [out] */ __RPC__out long *pVersionMajor,
            /* [out] */ __RPC__out long *pVersionMinor);
        
        END_INTERFACE
    } IMMCVersionInfoVtbl;

    interface IMMCVersionInfo
    {
        CONST_VTBL struct IMMCVersionInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMMCVersionInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMMCVersionInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMMCVersionInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMMCVersionInfo_GetMMCVersion(This,pVersionMajor,pVersionMinor)	\
    ( (This)->lpVtbl -> GetMMCVersion(This,pVersionMajor,pVersionMinor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMMCVersionInfo_INTERFACE_DEFINED__ */



#ifndef __MMCVersionLib_LIBRARY_DEFINED__
#define __MMCVersionLib_LIBRARY_DEFINED__

/* library MMCVersionLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_MMCVersionLib;

EXTERN_C const CLSID CLSID_MMCVersionInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("D6FEDB1D-CF21-4bd9-AF3B-C5468E9C6684")
MMCVersionInfo;
#endif

EXTERN_C const CLSID CLSID_ConsolePower;

#ifdef __cplusplus

class DECLSPEC_UUID("f0285374-dff1-11d3-b433-00c04f8ecd78")
ConsolePower;
#endif
#endif /* __MMCVersionLib_LIBRARY_DEFINED__ */

#ifndef __IExtendView_INTERFACE_DEFINED__
#define __IExtendView_INTERFACE_DEFINED__

/* interface IExtendView */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IExtendView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("89995CEE-D2ED-4c0e-AE5E-DF7E76F3FA53")
    IExtendView : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetViews( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT pDataObject,
            /* [in] */ __RPC__in_opt LPVIEWEXTENSIONCALLBACK pViewExtensionCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExtendViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IExtendView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IExtendView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IExtendView * This);
        
        DECLSPEC_XFGVIRT(IExtendView, GetViews)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetViews )( 
            __RPC__in IExtendView * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT pDataObject,
            /* [in] */ __RPC__in_opt LPVIEWEXTENSIONCALLBACK pViewExtensionCallback);
        
        END_INTERFACE
    } IExtendViewVtbl;

    interface IExtendView
    {
        CONST_VTBL struct IExtendViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExtendView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExtendView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExtendView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExtendView_GetViews(This,pDataObject,pViewExtensionCallback)	\
    ( (This)->lpVtbl -> GetViews(This,pDataObject,pViewExtensionCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExtendView_INTERFACE_DEFINED__ */


#ifndef __IViewExtensionCallback_INTERFACE_DEFINED__
#define __IViewExtensionCallback_INTERFACE_DEFINED__

/* interface IViewExtensionCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IViewExtensionCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34DD928A-7599-41E5-9F5E-D6BC3062C2DA")
    IViewExtensionCallback : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddView( 
            /* [in] */ __RPC__in PMMC_EXT_VIEW_DATA pExtViewData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IViewExtensionCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IViewExtensionCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IViewExtensionCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IViewExtensionCallback * This);
        
        DECLSPEC_XFGVIRT(IViewExtensionCallback, AddView)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddView )( 
            __RPC__in IViewExtensionCallback * This,
            /* [in] */ __RPC__in PMMC_EXT_VIEW_DATA pExtViewData);
        
        END_INTERFACE
    } IViewExtensionCallbackVtbl;

    interface IViewExtensionCallback
    {
        CONST_VTBL struct IViewExtensionCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IViewExtensionCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IViewExtensionCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IViewExtensionCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IViewExtensionCallback_AddView(This,pExtViewData)	\
    ( (This)->lpVtbl -> AddView(This,pExtViewData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IViewExtensionCallback_INTERFACE_DEFINED__ */


#ifndef __IConsolePower_INTERFACE_DEFINED__
#define __IConsolePower_INTERFACE_DEFINED__

/* interface IConsolePower */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConsolePower;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1cfbdd0e-62ca-49ce-a3af-dbb2de61b068")
    IConsolePower : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetExecutionState( 
            /* [in] */ DWORD dwAdd,
            /* [in] */ DWORD dwRemove) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ResetIdleTimer( 
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsolePowerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsolePower * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsolePower * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsolePower * This);
        
        DECLSPEC_XFGVIRT(IConsolePower, SetExecutionState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetExecutionState )( 
            __RPC__in IConsolePower * This,
            /* [in] */ DWORD dwAdd,
            /* [in] */ DWORD dwRemove);
        
        DECLSPEC_XFGVIRT(IConsolePower, ResetIdleTimer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ResetIdleTimer )( 
            __RPC__in IConsolePower * This,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IConsolePowerVtbl;

    interface IConsolePower
    {
        CONST_VTBL struct IConsolePowerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsolePower_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsolePower_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsolePower_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsolePower_SetExecutionState(This,dwAdd,dwRemove)	\
    ( (This)->lpVtbl -> SetExecutionState(This,dwAdd,dwRemove) ) 

#define IConsolePower_ResetIdleTimer(This,dwFlags)	\
    ( (This)->lpVtbl -> ResetIdleTimer(This,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsolePower_INTERFACE_DEFINED__ */


#ifndef __IConsolePowerSink_INTERFACE_DEFINED__
#define __IConsolePowerSink_INTERFACE_DEFINED__

/* interface IConsolePowerSink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConsolePowerSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3333759f-fe4f-4975-b143-fec0a5dd6d65")
    IConsolePowerSink : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnPowerBroadcast( 
            /* [in] */ UINT nEvent,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plReturn) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsolePowerSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsolePowerSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsolePowerSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsolePowerSink * This);
        
        DECLSPEC_XFGVIRT(IConsolePowerSink, OnPowerBroadcast)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnPowerBroadcast )( 
            __RPC__in IConsolePowerSink * This,
            /* [in] */ UINT nEvent,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out LRESULT *plReturn);
        
        END_INTERFACE
    } IConsolePowerSinkVtbl;

    interface IConsolePowerSink
    {
        CONST_VTBL struct IConsolePowerSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsolePowerSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsolePowerSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsolePowerSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsolePowerSink_OnPowerBroadcast(This,nEvent,lParam,plReturn)	\
    ( (This)->lpVtbl -> OnPowerBroadcast(This,nEvent,lParam,plReturn) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsolePowerSink_INTERFACE_DEFINED__ */


#ifndef __INodeProperties_INTERFACE_DEFINED__
#define __INodeProperties_INTERFACE_DEFINED__

/* interface INodeProperties */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_INodeProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("15BC4D24-A522-4406-AA55-0749537A6865")
    INodeProperties : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in_opt LPDATAOBJECT pDataObject,
            /* [in] */ __RPC__in BSTR szPropertyName,
            /* [out] */ __RPC__deref_out_opt PBSTR pbstrProperty) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INodePropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INodeProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INodeProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INodeProperties * This);
        
        DECLSPEC_XFGVIRT(INodeProperties, GetProperty)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in INodeProperties * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT pDataObject,
            /* [in] */ __RPC__in BSTR szPropertyName,
            /* [out] */ __RPC__deref_out_opt PBSTR pbstrProperty);
        
        END_INTERFACE
    } INodePropertiesVtbl;

    interface INodeProperties
    {
        CONST_VTBL struct INodePropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INodeProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INodeProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INodeProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INodeProperties_GetProperty(This,pDataObject,szPropertyName,pbstrProperty)	\
    ( (This)->lpVtbl -> GetProperty(This,pDataObject,szPropertyName,pbstrProperty) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INodeProperties_INTERFACE_DEFINED__ */


#ifndef __IConsole3_INTERFACE_DEFINED__
#define __IConsole3_INTERFACE_DEFINED__

/* interface IConsole3 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConsole3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4F85EFDB-D0E1-498c-8D4A-D010DFDD404F")
    IConsole3 : public IConsole2
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RenameScopeItem( 
            /* [in] */ HSCOPEITEM hScopeItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConsole3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConsole3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConsole3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConsole3 * This);
        
        DECLSPEC_XFGVIRT(IConsole, SetHeader)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetHeader )( 
            __RPC__in IConsole3 * This,
            /* [in] */ __RPC__in_opt LPHEADERCTRL pHeader);
        
        DECLSPEC_XFGVIRT(IConsole, SetToolbar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetToolbar )( 
            __RPC__in IConsole3 * This,
            /* [in] */ __RPC__in_opt LPTOOLBAR pToolbar);
        
        DECLSPEC_XFGVIRT(IConsole, QueryResultView)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryResultView )( 
            __RPC__in IConsole3 * This,
            /* [out] */ __RPC__deref_out_opt LPUNKNOWN *pUnknown);
        
        DECLSPEC_XFGVIRT(IConsole, QueryScopeImageList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryScopeImageList )( 
            __RPC__in IConsole3 * This,
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList);
        
        DECLSPEC_XFGVIRT(IConsole, QueryResultImageList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryResultImageList )( 
            __RPC__in IConsole3 * This,
            /* [out] */ __RPC__deref_out_opt LPIMAGELIST *ppImageList);
        
        DECLSPEC_XFGVIRT(IConsole, UpdateAllViews)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateAllViews )( 
            __RPC__in IConsole3 * This,
            /* [in] */ __RPC__in_opt LPDATAOBJECT lpDataObject,
            /* [in] */ LPARAM data,
            /* [in] */ LONG_PTR hint);
        
        DECLSPEC_XFGVIRT(IConsole, MessageBox)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *MessageBox )( 
            __RPC__in IConsole3 * This,
            /* [in] */ __RPC__in LPCWSTR lpszText,
            /* [in] */ __RPC__in LPCWSTR lpszTitle,
            /* [in] */ UINT fuStyle,
            /* [out] */ __RPC__out int *piRetval);
        
        DECLSPEC_XFGVIRT(IConsole, QueryConsoleVerb)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryConsoleVerb )( 
            __RPC__in IConsole3 * This,
            /* [out] */ __RPC__deref_out_opt LPCONSOLEVERB *ppConsoleVerb);
        
        DECLSPEC_XFGVIRT(IConsole, SelectScopeItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SelectScopeItem )( 
            __RPC__in IConsole3 * This,
            /* [in] */ HSCOPEITEM hScopeItem);
        
        DECLSPEC_XFGVIRT(IConsole, GetMainWindow)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMainWindow )( 
            __RPC__in IConsole3 * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IConsole, NewWindow)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NewWindow )( 
            __RPC__in IConsole3 * This,
            /* [in] */ HSCOPEITEM hScopeItem,
            /* [in] */ unsigned long lOptions);
        
        DECLSPEC_XFGVIRT(IConsole2, Expand)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in IConsole3 * This,
            /* [in] */ HSCOPEITEM hItem,
            /* [in] */ BOOL bExpand);
        
        DECLSPEC_XFGVIRT(IConsole2, IsTaskpadViewPreferred)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsTaskpadViewPreferred )( 
            __RPC__in IConsole3 * This);
        
        DECLSPEC_XFGVIRT(IConsole2, SetStatusText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetStatusText )( 
            __RPC__in IConsole3 * This,
            /* [string][in] */ __RPC__in_string LPOLESTR pszStatusText);
        
        DECLSPEC_XFGVIRT(IConsole3, RenameScopeItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RenameScopeItem )( 
            __RPC__in IConsole3 * This,
            /* [in] */ HSCOPEITEM hScopeItem);
        
        END_INTERFACE
    } IConsole3Vtbl;

    interface IConsole3
    {
        CONST_VTBL struct IConsole3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConsole3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConsole3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConsole3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConsole3_SetHeader(This,pHeader)	\
    ( (This)->lpVtbl -> SetHeader(This,pHeader) ) 

#define IConsole3_SetToolbar(This,pToolbar)	\
    ( (This)->lpVtbl -> SetToolbar(This,pToolbar) ) 

#define IConsole3_QueryResultView(This,pUnknown)	\
    ( (This)->lpVtbl -> QueryResultView(This,pUnknown) ) 

#define IConsole3_QueryScopeImageList(This,ppImageList)	\
    ( (This)->lpVtbl -> QueryScopeImageList(This,ppImageList) ) 

#define IConsole3_QueryResultImageList(This,ppImageList)	\
    ( (This)->lpVtbl -> QueryResultImageList(This,ppImageList) ) 

#define IConsole3_UpdateAllViews(This,lpDataObject,data,hint)	\
    ( (This)->lpVtbl -> UpdateAllViews(This,lpDataObject,data,hint) ) 

#define IConsole3_MessageBox(This,lpszText,lpszTitle,fuStyle,piRetval)	\
    ( (This)->lpVtbl -> MessageBox(This,lpszText,lpszTitle,fuStyle,piRetval) ) 

#define IConsole3_QueryConsoleVerb(This,ppConsoleVerb)	\
    ( (This)->lpVtbl -> QueryConsoleVerb(This,ppConsoleVerb) ) 

#define IConsole3_SelectScopeItem(This,hScopeItem)	\
    ( (This)->lpVtbl -> SelectScopeItem(This,hScopeItem) ) 

#define IConsole3_GetMainWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetMainWindow(This,phwnd) ) 

#define IConsole3_NewWindow(This,hScopeItem,lOptions)	\
    ( (This)->lpVtbl -> NewWindow(This,hScopeItem,lOptions) ) 


#define IConsole3_Expand(This,hItem,bExpand)	\
    ( (This)->lpVtbl -> Expand(This,hItem,bExpand) ) 

#define IConsole3_IsTaskpadViewPreferred(This)	\
    ( (This)->lpVtbl -> IsTaskpadViewPreferred(This) ) 

#define IConsole3_SetStatusText(This,pszStatusText)	\
    ( (This)->lpVtbl -> SetStatusText(This,pszStatusText) ) 


#define IConsole3_RenameScopeItem(This,hScopeItem)	\
    ( (This)->lpVtbl -> RenameScopeItem(This,hScopeItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConsole3_INTERFACE_DEFINED__ */


#ifndef __IResultData2_INTERFACE_DEFINED__
#define __IResultData2_INTERFACE_DEFINED__

/* interface IResultData2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IResultData2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0F36E0EB-A7F1-4a81-BE5A-9247F7DE4B1B")
    IResultData2 : public IResultData
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RenameResultItem( 
            /* [in] */ HRESULTITEM itemID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResultData2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IResultData2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IResultData2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IResultData2 * This);
        
        DECLSPEC_XFGVIRT(IResultData, InsertItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InsertItem )( 
            __RPC__in IResultData2 * This,
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, DeleteItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IResultData2 * This,
            /* [in] */ HRESULTITEM itemID,
            /* [in] */ int nCol);
        
        DECLSPEC_XFGVIRT(IResultData, FindItemByLParam)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindItemByLParam )( 
            __RPC__in IResultData2 * This,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out HRESULTITEM *pItemID);
        
        DECLSPEC_XFGVIRT(IResultData, DeleteAllRsltItems)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DeleteAllRsltItems )( 
            __RPC__in IResultData2 * This);
        
        DECLSPEC_XFGVIRT(IResultData, SetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IResultData2 * This,
            /* [in] */ __RPC__in LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, GetItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IResultData2 * This,
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, GetNextItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNextItem )( 
            __RPC__in IResultData2 * This,
            /* [out][in] */ __RPC__inout LPRESULTDATAITEM item);
        
        DECLSPEC_XFGVIRT(IResultData, ModifyItemState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ModifyItemState )( 
            __RPC__in IResultData2 * This,
            /* [in] */ int nIndex,
            /* [in] */ HRESULTITEM itemID,
            /* [in] */ UINT uAdd,
            /* [in] */ UINT uRemove);
        
        DECLSPEC_XFGVIRT(IResultData, ModifyViewStyle)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ModifyViewStyle )( 
            __RPC__in IResultData2 * This,
            /* [in] */ MMC_RESULT_VIEW_STYLE add,
            /* [in] */ MMC_RESULT_VIEW_STYLE remove);
        
        DECLSPEC_XFGVIRT(IResultData, SetViewMode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetViewMode )( 
            __RPC__in IResultData2 * This,
            /* [in] */ long lViewMode);
        
        DECLSPEC_XFGVIRT(IResultData, GetViewMode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetViewMode )( 
            __RPC__in IResultData2 * This,
            /* [out] */ __RPC__out long *lViewMode);
        
        DECLSPEC_XFGVIRT(IResultData, UpdateItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdateItem )( 
            __RPC__in IResultData2 * This,
            /* [in] */ HRESULTITEM itemID);
        
        DECLSPEC_XFGVIRT(IResultData, Sort)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Sort )( 
            __RPC__in IResultData2 * This,
            /* [in] */ int nColumn,
            /* [in] */ DWORD dwSortOptions,
            /* [in] */ LPARAM lUserParam);
        
        DECLSPEC_XFGVIRT(IResultData, SetDescBarText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetDescBarText )( 
            __RPC__in IResultData2 * This,
            /* [in] */ __RPC__in LPOLESTR DescText);
        
        DECLSPEC_XFGVIRT(IResultData, SetItemCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetItemCount )( 
            __RPC__in IResultData2 * This,
            /* [in] */ int nItemCount,
            /* [in] */ DWORD dwOptions);
        
        DECLSPEC_XFGVIRT(IResultData2, RenameResultItem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RenameResultItem )( 
            __RPC__in IResultData2 * This,
            /* [in] */ HRESULTITEM itemID);
        
        END_INTERFACE
    } IResultData2Vtbl;

    interface IResultData2
    {
        CONST_VTBL struct IResultData2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResultData2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResultData2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResultData2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResultData2_InsertItem(This,item)	\
    ( (This)->lpVtbl -> InsertItem(This,item) ) 

#define IResultData2_DeleteItem(This,itemID,nCol)	\
    ( (This)->lpVtbl -> DeleteItem(This,itemID,nCol) ) 

#define IResultData2_FindItemByLParam(This,lParam,pItemID)	\
    ( (This)->lpVtbl -> FindItemByLParam(This,lParam,pItemID) ) 

#define IResultData2_DeleteAllRsltItems(This)	\
    ( (This)->lpVtbl -> DeleteAllRsltItems(This) ) 

#define IResultData2_SetItem(This,item)	\
    ( (This)->lpVtbl -> SetItem(This,item) ) 

#define IResultData2_GetItem(This,item)	\
    ( (This)->lpVtbl -> GetItem(This,item) ) 

#define IResultData2_GetNextItem(This,item)	\
    ( (This)->lpVtbl -> GetNextItem(This,item) ) 

#define IResultData2_ModifyItemState(This,nIndex,itemID,uAdd,uRemove)	\
    ( (This)->lpVtbl -> ModifyItemState(This,nIndex,itemID,uAdd,uRemove) ) 

#define IResultData2_ModifyViewStyle(This,add,remove)	\
    ( (This)->lpVtbl -> ModifyViewStyle(This,add,remove) ) 

#define IResultData2_SetViewMode(This,lViewMode)	\
    ( (This)->lpVtbl -> SetViewMode(This,lViewMode) ) 

#define IResultData2_GetViewMode(This,lViewMode)	\
    ( (This)->lpVtbl -> GetViewMode(This,lViewMode) ) 

#define IResultData2_UpdateItem(This,itemID)	\
    ( (This)->lpVtbl -> UpdateItem(This,itemID) ) 

#define IResultData2_Sort(This,nColumn,dwSortOptions,lUserParam)	\
    ( (This)->lpVtbl -> Sort(This,nColumn,dwSortOptions,lUserParam) ) 

#define IResultData2_SetDescBarText(This,DescText)	\
    ( (This)->lpVtbl -> SetDescBarText(This,DescText) ) 

#define IResultData2_SetItemCount(This,nItemCount,dwOptions)	\
    ( (This)->lpVtbl -> SetItemCount(This,nItemCount,dwOptions) ) 


#define IResultData2_RenameResultItem(This,itemID)	\
    ( (This)->lpVtbl -> RenameResultItem(This,itemID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResultData2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmc_0000_0047 */
/* [local] */ 

#endif // MMC_VER >= 0x0200
#if (MMC_VER >= 0x0210)
typedef 
enum _MMC_ITEM_OVERLAY
    {
        MMC_ITEM_OVERLAY_NONE	= 0,
        MMC_ITEM_OVERLAY_ERROR	= 1,
        MMC_ITEM_OVERLAY_WARNING	= 2,
        MMC_ITEM_OVERLAY_INFO	= 3,
        MMC_ITEM_OVERLAY_BUSY	= 4,
        MMC_ITEM_OVERLAY_UNAVAILABLE	= 5,
        MMC_ITEM_OVERLAY_SHORTCUT	= 6,
        MMC_ITEM_OVERLAY_RUNNING	= 7,
        MMC_ITEM_OVERLAY_PAUSED	= 8,
        MMC_ITEM_OVERLAY_STOPPED	= 9,
        MMC_ITEM_OVERLAY_COMPLETE	= 10
    } 	MMC_ITEM_OVERLAY;

#define	MMC_ITEM_OVERLAY_STATE_MASK	( 0xf00 )

#define	MMC_ITEM_OVERLAY_STATE_SHIFT	( 8 )

#define MMC_OVERLAY_ITEM_INDEX_TO_ITEM_STATE(i) (((i) << MMC_ITEM_OVERLAY_STATE_SHIFT) & MMC_ITEM_OVERLAY_STATE_MASK)
#define MMC_OVERLAY_ITEM_INDEX_FROM_ITEM_STATE(state) (((state) & MMC_ITEM_OVERLAY_STATE_MASK) >> MMC_ITEM_OVERLAY_STATE_SHIFT)
#define	MMC_ITEM_STATE_MASK	( 0xff )

#endif // MMC_VER >= 0x0210
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0047_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmc_0000_0047_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HICON_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  HPALETTE_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HPALETTE * ); 
unsigned char * __RPC_USER  HPALETTE_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HPALETTE * ); 
unsigned char * __RPC_USER  HPALETTE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HPALETTE * ); 
void                      __RPC_USER  HPALETTE_UserFree(     __RPC__in unsigned long *, __RPC__in HPALETTE * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HICON_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree64(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  HPALETTE_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HPALETTE * ); 
unsigned char * __RPC_USER  HPALETTE_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HPALETTE * ); 
unsigned char * __RPC_USER  HPALETTE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HPALETTE * ); 
void                      __RPC_USER  HPALETTE_UserFree64(     __RPC__in unsigned long *, __RPC__in HPALETTE * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


