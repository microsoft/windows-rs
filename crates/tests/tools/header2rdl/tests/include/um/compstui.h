/*++

Copyright (c) 1990-1995  Microsoft Corporation


Module Name:

    compstui.h


Abstract:

    This module contains global header definition for the COMMON DRIVER UI


Author:

[Environment:]

    NT Windows - Common Property Sheet UI DLL.


[Notes:]


Revision History:


--*/

#ifndef _COMPSTUI_
#define _COMPSTUI_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

// disable warnings
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201) // named type definition in parentheses

#if (!defined(RC_INVOKED))


//
// For compilers that don't support nameless unions
//

#ifndef DUMMYUNIONNAME
#ifdef NONAMELESSUNION
#define DUMMYUNIONNAME      u
#define DUMMYUNIONNAME2     u2
#define DUMMYUNIONNAME3     u3
#define DUMMYUNIONNAME4     u4
#else
#define DUMMYUNIONNAME
#define DUMMYUNIONNAME2
#define DUMMYUNIONNAME3
#define DUMMYUNIONNAME4
#endif
#endif

//
// Predefined ID for the TreeView Option Type
//
//


#define TVOT_2STATES        0
#define TVOT_3STATES        1
#define TVOT_UDARROW        2
#define TVOT_TRACKBAR       3
#define TVOT_SCROLLBAR      4
#define TVOT_LISTBOX        5
#define TVOT_COMBOBOX       6
#define TVOT_EDITBOX        7
#define TVOT_PUSHBUTTON     8
#define TVOT_CHKBOX         9

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define TVOT_NSTATES_EX     10
#define TVOT_LAST           TVOT_NSTATES_EX
#else
#define TVOT_LAST           TVOT_CHKBOX
#endif
#define TVOT_NONE           (TVOT_LAST + 1)

//
// Predefined ID for the TreeView Option Type
//
//
// TVOT_2STATES:
//      Count       = 2
//      pOptParam[0]=pointer to the State 1 OPTPARAM
//      pOptParam[1]=pointer to the State 2 OPTPARAM
//
//      BegCtrlID  = 2 States Group Box ID
//      BegCtrlID+1= 2 States static Text
//      BegCtrlID+2= state 1 Radio button ID
//      BegCtrlID+3= state 1 icon control ID
//      BegCtrlID+4= state 2 Radio button ID
//      BegCtrlID+5= state 2 icon control ID
//      BegCtrlID+6= Extended Check Box/Push Button control ID
//      BegCtrlID+7= Extended Check Box/Push Button Icon control ID
//
//  * For TVOT_3STATES, TVOT_3STATES, each of OPTPARAM consisted
//
//      Style =Ignored
//      pData =Pointer to the string to describe the state
//      IconID=Icons resource ID, or common UI standard icon ID
//      lParam=Ignored
//
//  * OPTITEM's 'Sel' is the selection index range from 0 to 1
//  * On the non-treeview page, this must be a auto radio button
//
//
// TVOT_3STATES:
//      Count       = 3
//      pOptParam[0]=pointer to the State 1 OPTPARAM
//      pOptParam[1]=pointer to the State 2 OPTPARAM
//      pOptParam[2]=pointer to the State 3 OPTPARAM
//
//      BegCtrlID  = 3 States Group Box ID
//      BegCtrlID+1= 3 States static Text
//      BegCtrlID+2= state 1 Radio button ID
//      BegCtrlID+3= state 1 icon control ID
//      BegCtrlID+4= state 2 Radio button ID
//      BegCtrlID+5= state 2 icon control ID
//      BegCtrlID+6= state 3 Radio button ID
//      BegCtrlID+7= state 3 icon control ID
//      BegCtrlID+8= Extended Check Box/Push Button control ID
//      BegCtrlID+9= Extended Check Box/Push Button Icon control ID
//
//  * For TVOT_2STATES, TVOT_3STATES, each of OPTPARAM consisted
//
//      Style =Ignored
//      pData =Pointer to the string to describe the state
//      IconID=Icons resource ID, or common UI standard icon ID
//      lParam=Ignored
//
//  * OPTITEM's 'Sel' is the selection index range from 0 to 1
//  * On the non-treeview page, this must be a auto radio button
//
//  ** For TVOT_2STATES, TVOT_3STSATES the 'Sel' field in the OPTITEM has
//     following definitions
//
//      State 1, Sel = 0
//      State 2, Sel = 1
//      State 3, Sel = 2
//
//      for any selection which based on false/true, no/yes, off/ontrue/false,
//      none/select then state 1 (sel=0) must always be the NO, FALSE, OFF or
//      NONE type.
//
// TVOT_NSTATES_EX:
//      Count = N
//      pOptParam[0]=pointer to the first state's OPTPARAM
//      pOptParam[1]=pointer to the second state's OPTPARAM
//        .
//        .
//      pOptParam[N-1]=pointer to the Nth state's OPTPARAM
//
//      BegCtrlID  = N States Group Box ID
//      BegCtrlID+1= N States static Text
//      BegCtrlID+2= state 1 Radio button ID
//      BegCtrlID+3= state 1 icon control ID
//      BegCtrlID+4= state 1 combobox
//      BegCtrlID+5= state 2 Radio button ID
//      BegCtrlID+6= state 2 icon control ID
//      BegCtrlID+7= state 2 combobox
//        .
//        .
//      BegCtrlID+[N-1]*3+2= state N Radio button ID
//      BegCtrlID+[N-1]*3+3= state N icon control ID
//      BegCtrlID+[N-1]*3+4= state N combobox
//      BegCtrlID+[N-1]*3+5= description static text
//      BegCtrlID+[N-1]*3+6= icon ID for description static text
//      BegCtrlID+[N-1]*3+7= Extended Check Box/Push Button control ID
//      BegCtrlID+[N-1]*3+8= Extended Check Box/Push Button Icon control ID
//
//  * For TVOT_NSTATES_EX, each of OPTPARAM consisted
//
//      Style =Ignored
//      pData =Pointer to the string to describe the state
//      IconID=Icons resource ID, or common UI standard icon ID
//      lParam=Pointer to a OPTCOMBO structure
//
//  * A OPTCOMBO structure contains a list of items in a combobox. Its
//    structure is defined later in this file.
//
//  * This option type can be only displayed in a non-treeview page
//
//  * OPTITEM's 'Sel' is the selection index range from 0 to Count - 1
//     For TVOT_NSTSATES_EX the 'Sel' field in the OPTITEM has following
//     definitions
//
//      State 1, Sel = 0
//      State 2, Sel = 1
//       .
//       .
//      State N, Sel = N - 1
//
//  * In the POPTCOMBO data, each has a Sel field to indicate the current
//     selection in the state's combobox.
//
//  * For each pListItem(POPTPARAM) structure under POPTCOMBO data, the data
//    should be like following
//
//      Type    = Option parameter types, such as OPTPF_HIDE
//      pData   = Pointer to the string to describe the state
//      IconID  = Icons resource ID, or common UI standard icon ID
//      lParam  = Ignored
//
//  * It is possible that only one or two of the three states are available. In this case,
//    only the available items will be shown. If there is only one item available, all
//    three radio buttons will be hided, the description static text will be shown to
//    show the text.
//
// TVOT_UDARROW:
//      Count       = 2
//      pOptParam[0]=Pointer to the text of postfix and ICONS
//      pOptParam[1]=Pointer to the help line text above the control and
//                      IconID = (SHORT)Low range of the up-down control
//                      lParam = (SHORT)High range of the up-down control
//
//                          * Low/High must in range of a 16-bit sign integer
//
//          if pData pointed to no help text then common UI automatically
//          set the (# - #) as help line
//
//      BegCtrlID  = udarrow Group Box ID
//      BegCtrlID+1= udarrow title static title ID
//      BegCtrlID+2= udarrow's editbox ID
//      BegCtrlID+3= udarrow icon control ID
//      BegCtrlID+4= udarrow postfix static text ID
//      BegCtrlID+5= udarrow help static text ID
//      BegCtrlID+6= udarrow arrow ID
//      BegCtrlID+7= Extended Check Box/Push Button control ID
//      BegCtrlID+8= Extended Check Box/Push Button Icon control ID
//
//      * OPTITEM's 'Sel' is the selection index between Low/High range
//      * Style field in the OPTPARAM is ignored
//
//
// TVOT_TRACKBAR:
//      Count       = 3
//      pOptParam[0]=Pointer to the text for the selection postfix and ICONS
//      pOptParam[1]=Pointer to the <Low Range Text> and
//                      IconID = (SHORT)Low range of the trackbar control
//                      lParam = (SHORT)High range of the trackbar control
//
//                          * Low/High must in range of a 16-bit sign integer
//
//      pOptParam[2]=Pointer to the <High Range Text> and
//                      IconID = 'Sel' multiply factor for display
//                      lParam = Page Size (increment)
//
//          if pData pointed to NULLt then common UI automatically
//          set the Low/High range.
//
//      BegCtrlID  = trackbar Group Box ID
//      BegCtrlID+1= trackbar static title ID
//      BegCtrlID+2= trackbar(horizontal) ID (static FRAME to define size)
//      BegCtrlID+3= trackbar icon control ID
//      BegCtrlID+4= trackbar low range text control ID
//      BegCtrlID+5= trackbar high range text control ID
//      BegCtrlID+6= trackbar postfix ID
//      BegCtrlID+7= Extended Check Box/Push Button control ID
//      BegCtrlID+8= Extended Check Box/Push Button Icon control ID
//
//      * OPTITEM's 'Sel' is the selection index between Low/High range
//      * The multiply factor is used to multiply the current select with
//        this factor and display it. typically this is one
//      * the tick frequency is automatically to set to PageSize increment
//      * Style field in the OPTPARAM is ignored
//
//
// TVOT_SCROLLBAR:
//      Count       = 3
//      pOptParam[0]=Pointer to the text for the selection postfix and ICONS
//      pOptParam[1]=Pointer to the <Low Range Text> and
//                      IconID = (SHORT)Low range of the scrollbar control
//                      lParam = (SHORT)High range of the scroll control
//
//                          * Low/High must in range of a 16-bit sign integer
//
//      pOptParam[2]=Pointer to the <High Range Text> and
//                      IconID = 'Sel' multiply factor for display
//                      lParam = Page Size (increment)
//
//          if pData pointed to NULLt then common UI automatically
//          set the Low/High range.
//
//
//      BegCtrlID  = scrollbar(horizontal) group box ID
//      BegCtrlID+1= scrollbar(horizontal) static text ID
//      BegCtrlID+2= scrollbar(horizontal) ID
//      BegCtrlID+3= scrollbar icon control ID
//      BegCtrlID+4= scrollbar low range text control ID
//      BegCtrlID+5= scrollbar high range text control ID
//      BegCtrlID+6= scrollbar postfix control ID
//      BegCtrlID+7= Extended Check Box/Push Button control ID
//      BegCtrlID+8= Extended Check Box/Push Button Icon control ID
//
//      * OPTITEM's 'Sel' is the selection index between Low/High range
//      * The multiply factor is used to multiply the current select with
//        this factor and display it. typically this is one
//      * Style field in the OPTPARAM is ignored
//
//
//
// TVOT_LISTBOX:
// TVOT_COMBOBOX:
//      Count       = N
//      pOptParam[0]=pointer to the first OPTPARAM (pData=string pointer)
//      pOptParam[1]=pointer to the second OPTPARAM (pData=string pointer)
//          .
//          .
//      pOptParam[N-1]=pointer to the N item string
//
//      BegCtrlID  = Listbox/ComboBox group box ID
//      BegCtrlID+1= Listbox/ComboBox static title ID
//      BegCtrlID+2= Listbox/Combobox ID
//      BegCtrlID+3= Listbox/Combobox icon control ID
//      BegCtrlID+4= Extended Check Box/Push Button control ID
//      BegCtrlID+5= Extended Check Box/Push Button Icon control ID
//
//      * for TVOT_LISTBOX, TVOT_COMBOBOX, the field used as
//
//          Style =Ignored by the common UI
//          pData =Pointer to the name of item
//          IconID=Icon resource ID for the item
//          lParam=ignored by the common UI
//
//      * Only SINGLE selection is supported, to do a multiple selction use
//        multiple OPTITEM and create a header for it
//
//      * an OTLBCBS_SORT style can be specified in the OPTTYPE's LBCBStyle
//        field, and the listbox or combobox will be sorted according to the
//        item's string.
//
//      * OPTITEM's 'Sel' is the selection index between Low/High range
//
//      * for TVOT_LISTBOX, TVOT_COMBOBOX, when it get received the keyboard
//        focus then common UI will call callback function (only if
//        OPTIF_CALLBACK bit set) with reason of CPSUICB_REASON_LBCB_ACTIVE,
//        this give caller a chance to modify following structure flags/pdata
//        which associate with the current OPTITEM.   The caller's callback
//        function can ONLY modify the flags/data specified here.
//
//          OPTTYPE pointed by the pOptType from OPTITEM
//
//              Style: OTS_LBCB_SORT
//                     OTS_LBCB_INCL_ITEM_NONE
//
//          OPTPARAMs pointed by the pOptParam from the OPTTYPE
//
//              Flags: OPTPF_HIDE
//                     OPTPF_DISABLED
//
//              pData: change string name
//
//
//      * The TVOT_COMBOBOX typically only used in the tree-view if there is
//        only one selection available for that item, when there is only one
//        item then dropdown list will not enabled by the common UI
//
//
//
// TVOT_EDITBOX:
//      Count       = 2
//      pOptParam[0]=Pointer to the text of postfix and ICONS
//      pOptParam[1]=Pointer to the help line text above the control and
//                      IconID = Edit buffer sie in character pointed by pSel
//                                 this is including the NULL terminator.
//                      lParam = ignored.
//
//      BegCtrlID  = editBox group Box ID
//      BegCtrlID+1= editBox static title ID
//      BegCtrlID+2= editbox ID
//      BegCtrlID+3= editbox icon control ID
//      BegCtrlID+4= editbox postfix ID
//      BegCtrlID+5= editbox help ID
//      BegCtrlID+6= Extended Check Box/Push Button control ID
//      BegCtrlID+7= Extended Check Box/Push Button Icon control ID
//
//      * Style field is ignored
//
//      * pSel in the OPTITEM is the pointer to the editing string, the pSel
//        must pointed to a buffer eqaul or larger than the count of the buffer
//        (pOptParam[1]->IconID) size
//
//
// TVOT_PUSHBUTTON:
//      Count       = 1
//
//      BegCtrlID  = push button group box ID
//      BegCtrlID+1= push button static text ID (Not used by common UI)
//      BegCtrlID+2= push button ID
//      BegCtrlID+3= push button icon control ID
//      BegCtrlID+4= Extended Check Box/Push Button control ID
//      BegCtrlID+5= Extended Check Box/Push Button Icon control ID
//
//
//      PUSHBUTTON_TYPE_xxx specified the action and content of pData in the
//      pOptParam[0] as describe in the following
//
//          PUSHBUTTON_TYPE_DLGPROC
//
//              This push button is designed to bring up caller's dialog box
//
//                  pOptParam[0].pData  = Caller's DLGPROC
//                  pOptParam[0].Style  = PUSHBUTTON_TYPE_DLGPROC
//                  pOptParam[0].IconID = Icon resource ID
//                  pOptParam[0].lParam = Caller's DIALOG resource template ID
//                                        or handle to the DLGTEMPLATE depends
//                                        on the OPTPF_USE_HDLGTEMPLATE flag
//
//
//              The 'lParam' passed to the DLGPROC's WM_INITDIALOG is the
//              CPSUICBPARAM structure pointer, and the reason field is set
//              to CPSUICB_REASON_DLGPROC.
//
//
//          PUSHBUTTON_TYPE_CALLBACK
//
//              This push button is designed to have caller process the item
//              which cannot accomplished with the dialog box along.
//
//                  pOptParam[0].pData  = CPSUICALLBACK function pointer
//                  pOptParam[0].Style  = PUSHBUTTON_TYPE_CALLBACK
//                  pOptParam[0].IconID = Icon resource ID
//                  pOptParam[0].lParam = Not Used;
//
//              Durning the callback the Reason field in CPSUICBPARAM will
//              set to CPSUICB_REASON_PUSHBUTTON.
//
//              ** If pOptParam[0].pData callback function is NULL then common
//                 UI will call the pfnCallBack pointer set in the
//                 COMPROPSHEETUI structure if it is not NULL
//
//              ** The callback function should put the result of the callback
//                 in the pSel/Sel of OPTITEM associate with the push button
//
//          PUSHBUTTON_TYPE_HTCLRADJ
//
//              This push button is designed to bring up halftone color
//              adjustment dialog box.
//
//                  pOptParam[0].pData  = pointer to COLORADJUSTMENT structure
//                  pOptParam[0].Style  = PUSHBUTTON_TYPE_HTCLRADJ
//                  pOptParam[0].IconID = Icon resource ID
//                  pOptParam[0].lParam =  Not Used;
//
//
//          PUSHBUTTON_TYPE_HTSETUP
//
//              This push button is designed to bring up device halftone
//              setup dialog box.
//
//                  pOptParam[0].pData  = pointer to DEVHTADJDATA structure
//                  pOptParam[0].Style  = PUSHBUTTON_TYPE_HTSETUP
//                  pOptParam[0].IconID = Icon resource ID
//                  pOptParam[0].lParam = Not Used;
//
//
//      * 'Sel' field in the OPTITEM for the PUSHBUTTON is the last returned
//        LONG result from the called dialog box or funcitons.  The result
//        only valid if OPTIF_CHANGEONCE flag is set.  The common UI will set
//        OPTIF_CHANGEONCE if push button ever pushed.
//
//      * Since common UI donot know the meaning of the return value and
//        content of the called parameter, it is up to the caller to use
//        callback function to determine the returned result.
//
//      * When returned from the push button except push botton type
//        PUSHBUTTON_TYPE_CALLBACK common ui will call the callback function
//        if the OPTIF_CALLBACK flat is set.  The callback reason is set to
//        CPSUICB_REASON_SEL_CHANGED.
//
//      * If the passed in CPSUIF_UPDATE_PERMISSION Flags in the COMPROPSHEETUI
//        is clear then the callback function must ONLY display the dialog box
//        and not changed any OPTITEM data if OTS_PUSH_ENABLE_ALWAYS
//        flag is set in the OPTTYPE
//
//
// TVOT_CHKBOX:
//      Count               = 1
//
//      pOptparam[0].Style  = CHKBOXS_FALSE_TRUE    False/True
//                            CHKBOXS_NO_YES,       No/YES
//                            CHKBOXS_OFF_ON,       Off/ON
//                            CHKBOXS_FALSEPDATA    False/pData
//                            CHKBOXS_NO_PDATA      No/pData
//                            CHKBOXS_OFF_PDATA     Off/pData
//                            CHKBOXS_NONE_PDATA    None/pData
//      pOptParam[0].pData  = Only used if Style is CHKBOXS_NONE_PDATA
//      pOptParam[0].IconID = Icon resource ID
//      pOptParam[0].lParam = Ignored
//
//
//      BegCtrlID  = check box group ID
//      BegCtrlID+1= Check Box static text (not used by common UI)
//      BegCtrlID+2= check box button ID
//      BegCtrlID+3= check box icon control ID
//      BegCtrlID+4= Extended Check Box/Push Button control ID
//      BegCtrlID+5= Extended Check Box/Push Button Icon control ID
//
//
//  * BegCtrlID only used if the OPTITEM/OPTTYPE is belong to the the DLGPAGE
//    which has non-common UI dialog box template (DlgTemplateID in the DLGPAGE
//    is not standard DP_STD_xxx common ui dialog box template).    The common
//    UI used this ID to managed caller's dialog boxes item's selections and
//    initialization.
//
//  * for each item, it has group box ID (BegCtrlID) and static text ctronl ID
//    (BegCtrlID + 1).  The common UI will set the text in one of these two
//    control ID in followng seauence.
//
//      1) If group box control ID's window (BegCtrlID) is exist and the
//         OPTITEM's flag OPTIF_NO_GROUPBOX_NAME is not set then common UI will
//         set the pName from OPTITEM to the group box.
//
//      2) If the group box name is not set and static control ID's window
//         (BegCtrlID + 1) is exist then common UI will set the pName from
//         OPTITEM to the static text control.
//
//  * for TVOT_TRACKBAR and TVOT_SCROLLBAR, if pName in the OPTITEM is set to
//    either group box or static text control then common UI will also append
//    the current selection position of trackbar or scroll bar to the pName.
//
//  * If multiple OPTITEMs using the same POPTPARAM and need different
//    BegCtrlID for each control then then a separate OPTTYPE structure should
//    be generated but POPTPARAM pointed to the same OPTPARAM[]
//
//  * If a BegCtrlID+N is not used then skip that ID in your dialog box
//    template
//
//

#define CHKBOXS_FALSE_TRUE          0
#define CHKBOXS_NO_YES              1
#define CHKBOXS_OFF_ON              2
#define CHKBOXS_FALSE_PDATA         3
#define CHKBOXS_NO_PDATA            4
#define CHKBOXS_OFF_PDATA           5
#define CHKBOXS_NONE_PDATA          6


#define PUSHBUTTON_TYPE_DLGPROC     0
#define PUSHBUTTON_TYPE_CALLBACK    1
#define PUSHBUTTON_TYPE_HTCLRADJ    2
#define PUSHBUTTON_TYPE_HTSETUP     3


#define MAX_RES_STR_CHARS           160

//
// Common Printer UI's LPTSTR
//
// All string pointer in common printer UI structures can be either a real
// memory pointer or a string resource ID.  These are applied to LPTSTR type.
//
// The LPTSTR is defined to identify that the pointer can be a real string
// pointer or a resource ID (either common printer UI provided ID or caller's
// own resource ID).  common UI using following logic to get the final string.
//
//  LPTSTR  pData;
//
//      if ((pData & ~(ULONG_PTR)0xFFFF) != 0) then pData is a NULL terminated
//      string pointer
//
//          ELSE
//
//          (pData & (ULONG_PTR)0xFFFF) = Resource ID
//
//          if (Resource ID is within the common UI string resource ID range)
//          then it load the string from common UI DLL
//
//          ELSE
//
//              it load string from caller's resource
//
//
//  *  You can use MAKEINTRESOURCE(StrResID) to set this field
//
//  * The MAX characters loaded by the common UI from the resource is defined
//    as MAX_RES_STR_CHARS
//
//  * You cannot use LPTSTR as resource ID for the TVOT_EDITBOX style's
//    pSel in the OPTITEM, this pointer must be a real buffer pointer
//
//
//
// ICONs
//
//  Common UI using two types of Icons, One is 32x32 and the other is 16x16
//  plus if any monochrome icon with 32x32 and 16x16 sizes.
//
//  The 16x16 icon when displayed on the screen is using 16x17 pixel space,
//  this is ensure that downware adjacent icon is not crowded together.
//
//  In common UI, if you need to passed a ICON ID, it can either passed a
//  common UI's predefined ID or caller's own ICON resource ID.
//
//
//  * You can use to imagedit or any other Window icon editor to create the
//    icon, each icon file should have one unique icon resource ID which is
//    not overlay with the standard common UI IDI_CPSUI_xxx identifier.  For
//    each icon file, its should have both 32x32 and 16x16 size icon on
//    different display. (ie. monochrome).
//
//    Common UI will try to load the correct size of icon from the icon
//    resource, but it will stretch them if the size is not found.
//
//

//
// Flags for the OPTPARAM
//

#define OPTPF_HIDE                  0x01
#define OPTPF_DISABLED              0x02
#define OPTPF_ICONID_AS_HICON       0x04
#define OPTPF_OVERLAY_WARNING_ICON  0x08
#define OPTPF_OVERLAY_STOP_ICON     0x10
#define OPTPF_OVERLAY_NO_ICON       0x20
#define OPTPF_USE_HDLGTEMPLATE      0x40

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define OPTPF_MASK                  0x7f
#endif

typedef struct _OPTPARAM {
    WORD        cbSize;         // size of this structure
    BYTE        Flags;          // OPTPF_xxxx flags
    BYTE        Style;          // style use in this structure
    LPTSTR      pData;          // pointer to the data
    ULONG_PTR   IconID;         // iconID;
    LPARAM      lParam;         // parameter used
    ULONG_PTR   dwReserved[2];  // reserved dword, must be 0
    } OPTPARAM, *POPTPARAM;


//
// OPTPARAM
//
//  The OPTPARAM structure is used to describe each slectable item in the
//  common UI such as 'letter', 'legal' in the form slection list box
//
//  cbSize      - size of this structure
//
//  Flags       - defined as OPTPF_xxxx
//
//                  OPTPF_HIDE
//
//                      Specified hide this listed selection item and not
//                      availabe for user to select. This only available to
//                      following TVOT_xxx types
//
//                          TVOT_3STATES
//                          TVOT_LISTBOX
//                          TVOT_COMBOBOX
//                          TVOT_NSTATES_EX
//
//                      If all the seclection items are OPTPF_HIDE then the
//                      OPTITEM is automatically hided by the common UI, if
//                      TVOT_3STATES has 2 states hide then an error is
//                      returned
//
//
//                  OPTPF_DISABLED
//
//                      Specified this listed selection item is disabled and
//                      not availabe for user to select.   This only available
//                      to following TVOT_xxx types
//
//                          TVOT_2STATES
//                          TVOT_3STATES
//                          TVOT_LISTBOX
//                          TVOT_COMBOBOX
//                          TVOT_NSTATES_EX
//
//                  OPTPF_ICONID_AS_HICON
//
//                      If this flag is set then IconID DWORD field is treated
//                      as a handle to the icon rather then the resource ID
//
//
//                  OPTPF_OVERLAY_WARNING_ICON
//
//                      If this bit is set then this OPTPARAM item's icon will
//                      be overlaied by a common UI's IDI_CPSUI_WARNING icon.
//
//
//                  OPTPF_OVERLAY_STOP_ICON
//
//                      If this bit is set then this OPTPARAM item's icon will
//                      be overlaied by a common UI's IDI_CPSUI_STOP icon.
//
//
//                  OPTPF_OVERLAY_NO_ICON
//
//                      If this bit is set then this OPTPARAM item's icon will
//                      be overlaied by a common UI's IDI_CPSUI_NO icon.
//
//
//  Style       - Style for the OPTPARAM, it depends on the TVOT_xxx type as
//                describe below
//
//                  TVOT_PUSBUTTON
//
//                      it can be one of PUSHBUTTON_TYPE_xxxx.
//
//                  other TVOT_xxxx
//
//                      this fields is not used.
//
//
//  pData       - Is either a pointer to the item name (string) or it is
//                used to describe other data.
//
//                   * If the pData in the OPTPARAM is supposed to be a static
//                     pointer to a string and the string is a common UI
//                     standard resource ID then common UI will check if pData
//                     is equal to IDS_CPSUI_NOTINSTALLED, if true then
//                     common UI will overaly a not installed icon on top of
//                     the OPTPARAM's Icon.   This will not applied to the
//                     TVOT_EDITBOX type since the pData is not a static text
//                     pointer or a string resource ID.
//
//                  ** See LPTSTR description above
//
//  IconID      - This is the icon identifier, which can be a common strandard
//                IDI_CPSUI_xxx icon ID, caller's own icon resource ID, or a
//                handle to the caller defined icon if OPTPF_ICONID_AS_HICON
//                flag is set, in any case if the IconID is zero then it
//                indicated no icon.
//
//  lParam      - Extra data used by the OPTPARAM, it depends on the TVOT_xxx
//                type.
//
//  dwReserved[]- Reserved DWORDs, must be 0
//
//

//
// Flags for the OPTCOMBO
//

#if (NTDDI_VERSION >= NTDDI_VISTA)

#define OPTCF_HIDE      0x01

#define OPTCF_MASK      0x01

typedef struct _OPTCOMBO {
    WORD        cbSize;
    BYTE        Flags;
    WORD        cListItem;
    POPTPARAM   pListItem;
    LONG        Sel;
    DWORD       dwReserved[3];
} OPTCOMBO, *POPTCOMBO;

#endif

//
// OPTCOMBO is to describe data each combobox/listbox item in
// TVOT_NSTATES_EX option type
//
//  cbSize          - sizeof this structure
//
//  Flags           - Item flags
//
//                    OPTCF_HIDE
//
//                        Specified hide this combobox is not available
//                        for user to select.
//
//  cListItem       - Number of the list items
//
//  pListItem       - Pointer to the first list item
//
//  Sel             - Current selection for this item, range from 0 to cListItem - 1
//
//  dwReserved      - DWORD reserved, must be 0
//

//
// Flags for the OPTTYPE
//

#define OPTTF_TYPE_DISABLED             0x01
#define OPTTF_NOSPACE_BEFORE_POSTFIX    0x02

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define OPTTF_MASK                      0x03
#endif

//
// Flags for the OPTTYPE style
//

#define OTS_LBCB_SORT                   0x0001
#define OTS_LBCB_PROPPAGE_LBUSECB       0x0002
#define OTS_LBCB_PROPPAGE_CBUSELB       0x0004
#define OTS_LBCB_INCL_ITEM_NONE         0x0008
#define OTS_LBCB_NO_ICON16_IN_ITEM      0x0010
#define OTS_PUSH_INCL_SETUP_TITLE       0x0020
#define OTS_PUSH_NO_DOT_DOT_DOT         0x0040
#define OTS_PUSH_ENABLE_ALWAYS          0x0080

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define OTS_MASK                        0x00ff
#endif

typedef struct _OPTTYPE {
    WORD        cbSize;
    BYTE        Type;           // TVOT_xxxx type of OPTIONS
    BYTE        Flags;          // OPTTF_xxx flags
    WORD        Count;          // Count of pOptParam passed
    WORD        BegCtrlID;      // start of item's group window ID
    POPTPARAM   pOptParam;      // pointer to the OPTPARAM
    WORD        Style;          // option type style as OTS_xxxx
    WORD        wReserved[3];   // wReserved, must be 0
    ULONG_PTR   dwReserved[3];  // DWORD reserved field (must be 0)
} OPTTYPE, *POPTTYPE;


//
// OPTTYPE
//
//  The OPTTYPE structure is used to describe a set of selection and its
//  select method,  such as Form/Tray assignment.  It has a pointer to a set
//  of selection item (OPTPARAM)
//
//
//  cbSize      - size of this structure
//
//  Type        - Specified the option type using predefined ID as TVOT_xxxx
//
//  Flags       - currently only one flag is defined
//
//
//                  OPTTF_TYPE_DISABLED
//
//                      The whole OPTTYPE's OPTPARAMs are disabled, and non of
//                      the selection in the OPTTYPE can be selected
//
//
//                  OPTTF_NOSPACE_BEFORE_POSTFIX
//
//                      This bit only valid if the OPTTYPE's pOptParam item
//                      specified a postfix string as describe in the above
//                      section.  If this flag is set then it asked common UI
//                      do not add a space character before the postfix string
//                      when it combine the pName in the OPTITEM and postfix
//                      string.   Typeically this bit is not set for the
//                      postfix string, but sometime it may be required not to
//                      add a space character in front of it, such as '%'
//                      postfix string.
//
//
//  Count       - Count of item pointed by pOptParam.  Some predefined number
//                must be set according to the TVOT_XXX description.
//
//  BegCtrlID   - Only used if the OPTITEM/OPTTYPE is belong to the the DLGPAGE
//                which has non-common UI dialog box template (DlgTemplateID
//                in the DLGPAGE is not standard DP_STD_xxx common ui dialog
//                box template).    The common UI used this ID to managed
//                caller's dialog boxes item's selections and initialization.
//
//                Each OPTITEM has predefined number of window ID which
//                associated with that item, the BegCtrlID specified the start
//                control window ID.  Each control window ID in the OPTITEM
//                must have the control ID sequence as describe in the TVOT_xxx
//                above.
//
//  pOptParam   - Pointer to array of OPTPARAM to describe each selectable item
//
//  Style       - Specified the style of type of control box, certain style
//                only apply to centain type of TVOT_xxxx.
//
//                OTS_LBCB_xxx only applied to TVOT_LISTBOX, TVOT_COMBOBOX
//                OTS_PUSH_xxx only applied to TVOT_PUSHBUTTON
//
//
//                  OTS_LBCB_SORT
//
//                      Specified that the listbox or combobox item is sorted
//                      in ascending order based on the pData string
//
//
//                  OTS_LBCB_PROPAGE_LBUSECB
//
//                      Used when Type is TVOT_LISTBOX, if it specified and
//                      this OPTTYPE also on the non-treeview user defined
//                      property sheet page dialog then common UI assume
//                      control is comobobox instead of listbox on the non-
//                      treeview page
//
//
//                  OTS_LBCB_PROPAGE_CBUSELB
//
//                      Used when Type is TVOT_COMBOBOX, if it specified and
//                      this OPTTYPE also on the non-treeview user defined
//                      property sheet page dialog then common UI assume
//                      control is listbox instead of combobox on the non-
//                      treeview page
//
//
//                  OTS_LBCB_INCL_ITEM_NONE
//
//                      when this flag is specified, the common ui will
//                      automatically add a 'None' selection to the listbox or
//                      combobox.   The 'Sel' will set to -1 if 'none' is
//                      selection is selected by the user.  It will also
//                      validate the 'Sel' durning the initialization,  any
//                      out of range value will be set to -1 (None).
//
//
//                  OTS_LBCB_NO_ICON16_IN_ITEM
//
//                      By default, each listbox, combox will have a small
//                      icon (16x16) in front of item text. by specified this
//                      bit, the listbox/combobox will not includes icons
//                      in the listbox/combobox.
//
//                      If clear then it specified that in the listbox/combobox
//                      to have 16x16 Icon added to the front of each item
//
//
//                  OTS_PUSH_INCL_SETUP_TITLE
//
//                      If specified for the push button then it automatically
//                      add the 'Setup' to the end of push botton text.
//
//
//                  OTS_PUSH_NO_DOT_DOT_DOT
//
//                      If specified then common UI will not add '...' to the
//                      end of the pName in the OPTITEM and push button name
//
//
//                  OTS_PUSH_ENABLE_ALWAYS
//
//                      This flag specified that even update permissio is not
//                      allowed, it still let user push the push button, if
//                      this flag is set then callback function or dialog box
//                      proc must disable all the control which let user
//                      modified the content, but just let user view the
//                      current setting.
//
//
//  wReserved[] - Reserved fields, must be 0
//
//  dwReserved[]- Reserved fields, must be 0
//
//


//
// Following are flags for the EXTPUSH
//


#define EPF_PUSH_TYPE_DLGPROC       0x0001
#define EPF_INCL_SETUP_TITLE        0x0002
#define EPF_NO_DOT_DOT_DOT          0x0004
#define EPF_ICONID_AS_HICON         0x0008
#define EPF_OVERLAY_WARNING_ICON    0x0010
#define EPF_OVERLAY_STOP_ICON       0x0020
#define EPF_OVERLAY_NO_ICON         0x0040
#define EPF_USE_HDLGTEMPLATE        0x0080

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define EPF_MASK                    0x00ff
#endif

typedef struct _EXTPUSH {
    WORD                cbSize;         // size of the structure
    WORD                Flags;          // EPCBF_xxx flags
    LPTSTR              pTitle;         // extended push botton title
    union {
        DLGPROC         DlgProc;        // pointer to the dialog box proc
        FARPROC         pfnCallBack;    // callback function pointer
        } DUMMYUNIONNAME;
    ULONG_PTR           IconID;         // icon to be used
    union {
        WORD    DlgTemplateID;          // dialog box template ID
        HANDLE  hDlgTemplate;           // handle to the dialog template
        } DUMMYUNIONNAME2;
    ULONG_PTR   dwReserved[3];          // reserved field, must be 0
    } EXTPUSH, *PEXTPUSH;

//
// EXTPUSH structure is used to describe the extened push button available
// on OPTITEM/OPTITEM, each OPTTYPE can optional have either one extended check
// box or one extended push button callback.
//
//
//  cbSize          - size of this structure
//
//  Flags           - flags for the EXTPUSH as EPF_xxxx
//
//                      EPF_PUSH_TYPE_DLGPROC
//
//                          If this bit is set then it specified the extended
//                          push button is type of DLGPROC and DlgProc and
//                          DlgTemplateID is valid for common UI to call.
//
//                          If this bit is clear then it specfied the extended
//                          push button is the callback style and pfnCallBack
//                          should be called by the common UI
//
//
//                      EPF_INCL_SETUP_TITLE
//
//                          If specified for the extended push button then it
//                          automatically add the 'Setup' to the end of
//                          extended push button's title
//
//
//                      EPF_NO_DOT_DOT_DOT
//
//                          If specified then common UI will not add '...' to
//                          the end of the pTitle in the EXTPUSH.
//
//
//                      EPF_ICONID_AS_HICON
//
//                          If this flag is set then IconID DWORD field is
//                          treated as a handle to the icon rather then the
//                          resource ID.
//
//
//                      EPF_OVERLAY_WARNING_ICON
//
//                          If this bit is set then this EXTPUSH's icon will be
//                          overlaied by a common UI's IDI_CPSUI_WARNING icon.
//
//
//                      EPF_OVERLAY_STOP_ICON
//
//                          If this bit is set then this EXTPUSH's icon will be
//                          overlaied by a common UI's IDI_CPSUI_STOP icon.
//
//
//                      EPF_OVERLAY_NO_ICON
//
//                          If this bit is set then this EXTPUSH's icon will be
//                          overlaied by a common UI's IDI_CPSUI_NO icon.
//
//
//  pTitle          - Pointed to extended push botton title
//
//                      ** See LPTSTR description above
//
//  DlgProc         - Pointer to the DLGPROC function supplied by the caller.
//                    When user push the button the common UI will call
//                    DialogBoxParam() with this fucction pointer and passed
//                    CPSUICBPARAM structure pointer to the WM_INITDIALOG with
//                    the Reason set to CPSUICB_REASON_EXTPUSH.  If this
//                    filed is NULL then common UI assumed that EXTPUSH is
//                    disabled (OPTIF_EXT_DISABLED | OPTIF_EXT_HIDE), this
//                    fields only used if EPF_PUSH_TYPE_DLGPROC flag is set
//
//  pfnCallBack     - Pointer to CPSUICALLBACK function, this only used if
//                    EPF_PUSH_TYPE_DLGPROC bit is clear, duringing callback
//                    it passed the CPSUICBPARAM pointer as parameter
//
//  IconID          - This is the icon identifier, which can be a common
//                    strandard IDI_CPSUI_xxx icon ID, caller's own icon
//                    resource ID, or a handle to the caller defined icon if
//                    EPF_ICONID_AS_HICON flag is set, in any case if the
//                    IconID is zero then it indicated no icon.
//
//  DlgTemplateID   - Specified the ressource ID for the dilaog box. If the
//                    DlgTemplateID = 0 then common UI will call the DlgProc
//                    with following parameter.
//
//                      DlgProc(hDlg, WM_USER, NULL, (LPARAM)pCPSUICBPaam);
//
//  hDlgTemplate    - Handle to the DLGTEMPLATE which will be use for pop up
//                    dialog box
//
//  wReserved[]     - WORD reserved field, must be 0
//
//  dwReserved[]    - DWORD reserved field, must be 0
//
//
//


//
// Following are flags for the EXTCHKBOX
//

#define ECBF_CHECKNAME_AT_FRONT         0x0001
#define ECBF_CHECKNAME_ONLY_ENABLED     0x0002
#define ECBF_ICONID_AS_HICON            0x0004
#define ECBF_OVERLAY_WARNING_ICON       0x0008
#define ECBF_OVERLAY_ECBICON_IF_CHECKED 0x0010
#define ECBF_OVERLAY_STOP_ICON          0x0020
#define ECBF_OVERLAY_NO_ICON            0x0040
#define ECBF_CHECKNAME_ONLY             0x0080

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define ECBF_MASK                       0x00ff
#endif

typedef struct _EXTCHKBOX {
    WORD        cbSize;         // size of the structure
    WORD        Flags;          // ECBF_xxx flags
    LPTSTR      pTitle;         // extended checkbox title
    LPTSTR      pSeparator;     // pointer to separator string for treeview
    LPTSTR      pCheckedName;   // string to be displayed when checked
    ULONG_PTR   IconID;         // icon to be used
    WORD        wReserved[4];   // reserved word, must be 0
    ULONG_PTR   dwReserved[2];  // reserved field, must be 0
    } EXTCHKBOX, *PEXTCHKBOX;


//
// EXTCHKBOX structure is used to describe the extened check box available on
// OPTITEM/OPTITEM, each OPTTYPE can optional have one extended check box.
// When using EXTCHKBOX the selection item can be checked or not checked
// based on user input.
//
//
//  cbSize      - size of this structure
//
//  Flags       - flags for the EXTCHKBOX as ECBF_xxxx
//
//                  ECBF_CHECKNAME_AT_FRONT
//
//                      This flag specified how to display item's name and its
//                      checked name in the treeview display.   If this flag is
//                      set then the checked name is display in front of
//                      separator name, otherwise the checked name is displayed
//                      after the separator.  For Example.
//
//                      Flag Set:   pCheckedName pSeparator SelectName
//                      Flag Clear: SelectName pSeparator pCheckedName
//
//
//                  ECBF_CHECKNAME_ONLY_ENABLED
//
//                      If set then it specified that in the treeview display,
//                      it will only show the pCheckedName if this extended
//                      check box is visible and enabled.  Some items may not
//                      desired to display the pCheckedName if the extended
//                      check box is disabled, such as Copy/Collate checkbox.
//
//
//                  ECBF_ICONID_AS_HICON
//
//                      If this flag is set then IconID DWORD field is treated
//                      as a handle to the icon rather then the resource ID
//
//
//                  ECBF_OVERLAY_WARNING_ICON
//
//                      If this bit is set then this EXTCHKBOX's icon will
//                      be overlaied by a common UI's IDI_CPSUI_WARNING icon.
//
//
//                  ECBF_OVERLAY_ECBICON_IF_CHECKED
//
//                      This bit specified to overlay the ExtChkBox's Icon to
//                      the OPTITEM's icon (or OPTPARAM) if the the extended
//                      checked box is checked
//
//
//                  ECBF_OVERLAY_STOP_ICON
//
//                      If this bit is set then this EXTCHKBOX's icon will
//                      be overlaied by a common UI's IDI_CPSUI_STOP icon.
//
//
//                  ECBF_OVERLAY_NO_ICON
//
//                      If this bit is set then this EXTCHKBOX's icon will
//                      be overlaied by a common UI's IDI_CPSUI_NO icon.
//
//
//  pTitle      - Pointed to extended check box title
//
//                  ** See LPTSTR description above
//
//  pSeparator  - Pointer to the separator to be used in the treeview
//                display or the static title control in the non-treeview
//                page,
//
//  pCheckedName- Pointed to the name to be displayed in the treeview if item
//                is checked.  pCheckedName is added according to the
//                pSeparator and the ECBF_CHECKNAME_AT_FRONT flags.
//
//                  * If the pCheckedName is equal to IDS_CPSUI_NOTINSTALLED
//                    then common UI will automatically overaly a not installed
//                    icon on top of the extended check box Icon.
//
//                      ** See LPTSTR description above
//
//  IconID      - This is the icon identifier, which can be a common strandard
//                IDI_CPSUI_xxx icon ID, caller's own icon resource ID, or a
//                handle to the caller defined icon if ECBF_ICONID_AS_HICON
//                flag is set, in any case if the IconID is zero then it
//                indicated no icon.
//
//  wReserved[] - WORD reserved field, must be 0
//
//  dwReserved[]- DWORD reserved field, must be 0
//


//
// Following the the Flags for the OPTITEM
//

#define OPTIF_COLLAPSE              0x00000001L
#define OPTIF_HIDE                  0x00000002L
#define OPTIF_CALLBACK              0x00000004L
#define OPTIF_CHANGED               0x00000008L
#define OPTIF_CHANGEONCE            0x00000010L
#define OPTIF_DISABLED              0x00000020L
#define OPTIF_ECB_CHECKED           0x00000040L
#define OPTIF_EXT_HIDE              0x00000080L
#define OPTIF_EXT_DISABLED          0x00000100L
#define OPTIF_SEL_AS_HICON          0x00000200L
#define OPTIF_EXT_IS_EXTPUSH        0x00000400L
#define OPTIF_NO_GROUPBOX_NAME      0x00000800L
#define OPTIF_OVERLAY_WARNING_ICON  0x00001000L
#define OPTIF_OVERLAY_STOP_ICON     0x00002000L
#define OPTIF_OVERLAY_NO_ICON       0x00004000L
#define OPTIF_INITIAL_TVITEM        0x00008000L
#define OPTIF_HAS_POIEXT            0x00010000L

#define OPTIF_MASK                  0x0001ffffL


#define DMPUB_NONE                  0
#define DMPUB_FIRST                 1

#define DMPUB_ORIENTATION           1
#define DMPUB_SCALE                 2
#define DMPUB_COPIES_COLLATE        3
#define DMPUB_DEFSOURCE             4
#define DMPUB_PRINTQUALITY          5
#define DMPUB_COLOR                 6
#define DMPUB_DUPLEX                7
#define DMPUB_TTOPTION              8
#define DMPUB_FORMNAME              9
#define DMPUB_ICMMETHOD             10
#define DMPUB_ICMINTENT             11
#define DMPUB_MEDIATYPE             12
#define DMPUB_DITHERTYPE            13
#define DMPUB_OUTPUTBIN             14
#define DMPUB_QUALITY               15
#define DMPUB_NUP                   16
#define DMPUB_PAGEORDER             17

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define DMPUB_NUP_DIRECTION         18
#define DMPUB_MANUAL_DUPLEX         19
#define DMPUB_STAPLE                20
#define DMPUB_BOOKLET_EDGE          21
#define DMPUB_LAST                  21
#else
#define DMPUB_LAST                  17
#endif

#define DMPUB_OEM_PAPER_ITEM        97
#define DMPUB_OEM_GRAPHIC_ITEM      98
#define DMPUB_OEM_ROOT_ITEM         99
#define DMPUB_USER                  100

#define MAKE_DMPUB_HIDEBIT(DMPub)   (DWORD)(((DWORD)0x01 << ((DMPub) - 1)))
#define IS_DMPUB_HIDDEN(dw, DMPub)  (BOOL)((DWORD)(dw) &                    \
                                           MAKE_DMPUB_HIDEBIT(DMPub))

//
// DMPUB_xxxx is used in OPTITEM to identify if the item is a DEVMODE public
// field. Following it identify which field correspond to the DMPUB_xxxx
//
//
//  DMPUB_ORIENTATION   - dmOrientation
//
//  DMPUB_SCALE         - dmScale
//
//  DMPUB_COPIES_COLLATE- dmCopies/dmCollate
//
//  DMPUB_DEFSOURCE     - dmDefSource    (Should only used form by caller)
//
//  DMPUB_PRINTQUALITY  - dmPrintQuality
//
//  DMPUB_COLOR         - dmColor
//
//  DMPUB_DUPLEX        - dmDuplex
//
//  DMPUB_TTOPTION      - dmTTOption
//
//  DMPUB_FORMNAME      - dmFormName
//
//  DMPUB_ICMMETHOD     - dmICMMethod
//
//  DMPUB_ICMINTENT     - dmICMIntent
//
//  DMPUB_MEDIATYPE     - dmMediaType
//
//  DMPUB_DITHERTYPE    - dmDitherType
//
//  DMPUB_USER          - Anything greater than or equal to DMPUB_USER can be
//                        used by the caller.
//
//  DMPUB_OUTPUTBIN     - Private
//
//  DMPUB_QUALITY       - Private
//
//  DMPUB_NUP           - Private
//
//  DMPUB_PAGEORDER     - Private
//
//  DMPUB_NUP_DIRECTION - Private
//
//  DMPUB_MANUAL_DUPLEX - Private
//
//  DMPUB_STAPLE        - Private
//
// When common UI is called for the 'Document Properties' (DEVMODE), it will
// group some of public items together simillar to the following in the
// treeview. How it group is common UI version dependent and caller should not
// concern its placement
//
//      Paper/Output                (Add in by the common UI)
//        Document Form             (DMPUB_FORMNAME)
//        Output Bin                (DMPUB_OUTPUTBIN)
//        Orientation               (DMPUB_ORIENTATION)
//        Source                    (DMPUB_DEFSOURCE)
//        Media                     (DMPUB_MEDIATYPE)
//        Number of Copies          (DMPUB_COPIES_COLLATE)
//        Page Order                (DMPUB_PAGEORDER)
//        Page Per Sheet            (DMPUB_NUP)
//        Duplex                    (DMPUB_DUPLEX)
//
//      Graphic                     (Add in by the common UI)
//        Print Quality             (DMPUB_PRINTQUALITY)
//        Quality Settings          (DMPUB_QUALITY)
//        Color                     (DMPUB_COLOR)
//          Image Color Matching    (Add in by the common UI)
//              ICM Method          (DMPUB_ICMMETHOD)
//              ICM Intent          (DMPUB_ICMINTENT)
//        Scaling                   (DMPUB_SCALE)
//        Dithering                 (DMPUB_DITHERTYPE)
//        TrueType Option           (DMPUB_TTOPTION)
//
//      Options
//        Halftone Color Adjustment...
//        ALL Other Caller's Document sticky options
//
//
//  For 'Document Properties' the standard page 1 (user friendly page) will
//  consist following items if it appear in the OPTITEM array passed by the
//  caller.   These items must have following predefined TVOT_xxx type
//  defined here and specified in the OPTTYPE's Type field.
//
//  All DMPUB_xxx (except >= DMPUB_USER) public ID must have following
//  TVOT_xxxx type specified, else a CPDU_INVALID_DMPUB_TVOT error is returned
//
//      DMPUB_ORIENTATION       - TVOT_2STATES/TVOT_3STATES
//      DMPUB_SCALE             - TVOT_UDARROW
//      DMPUB_COPIES_COLLATE    - TVOT_UDARROW + EXTCHKBOX (Collate)
//      DMPUB_DEFSOURCE         - TVOT_LISTBOX
//      DMPUB_PRINTQUALITY      - TVOT_LISTBOX
//      DMPUB_COLOR             - TVOT_2STATES
//      DMPUB_DUPLEX            - TVOT_2STATES/TVOT_3STATES
//      DMPUB_TTOPTION          - TVOT_LISTBOX
//      DMPUB_FORMNAME          - TVOT_LISTBOX
//      DMPUB_ICMMETHOD         - TVOT_2STATES/TVOT_3STATES
//      DMPUB_ICMINTENT         - TVOT_2STATES/TVOT_3STATES
//      DMPUB_MEDIATYPE         - TVOT_LISTBOX
//      DMPUB_DITHERTYPE        - TVOT_LISTBOX
//
//


#define OIEXTF_ANSI_STRING      0x0001

typedef struct _OIEXT {
    WORD        cbSize;
    WORD        Flags;
    HINSTANCE   hInstCaller;
    LPTSTR      pHelpFile;
    ULONG_PTR   dwReserved[4];
    } OIEXT, *POIEXT;

//
// OIEXT is a data structure used as extension to the OPTITEM data structure
//
//
//  cbSize      - sizeof this structure
//
//  Flags       - One or more of OIEXTF_xxxx may be specified.
//
//                  OIEXTF_ANSI_STRING
//
//                      Specified that LPTSTR in this data structure is a ansi
//                      string (not UNICODE).  This bit only checked if the
//                      LPTSTR is not the resource string ID
//
//  hInstCaller - DLL instance handle, when this hInst is not NULL then all
//                resource string and icon loading for this OPTITEM and its
//                OPTTYPE, OPTPARAM are loaded from this hInstCaller Handle.
//                If this filed is NULL then it will use the hInstCaller handle
//                specified in the COMPROPSHEETUI data structure
//
//  pHelpFile   - Resource string ID or pointer to the help file for this
//                OPTITEM.  If this pointer is NULL then help file for the
//                help index is assume specified in the pHelpFile field in the
//                COMPROPSHEETUI data structure.
//
//  dwReserved  - These fields are not used now, and must 0
//


typedef struct _OPTITEM {
    WORD        cbSize;                 // size of this structure
    BYTE        Level;                  // level in the tree view
    BYTE        DlgPageIdx;             // Index to the pDlgPage
    DWORD       Flags;                  // OPTIF_xxxx flags
    ULONG_PTR   UserData;               // caller's own data
    LPTSTR      pName;                  // name of the item
    union {
        LONG    Sel;                    // current selection (index)
        LPTSTR  pSel;                   // current selection (pStr)
        } DUMMYUNIONNAME;
    union {
        PEXTCHKBOX  pExtChkBox;         // Pointer to EXTCHKBOX structure
        PEXTPUSH    pExtPush;           // Pointer to EXTPUSH
        } DUMMYUNIONNAME2;
    POPTTYPE    pOptType;               // pointer to OPTTYPE structure
    DWORD       HelpIndex;              // Help file index
    BYTE        DMPubID;                // Devmode public filed ID
    BYTE        UserItemID;             // caller's own item ID
    WORD        wReserved;              // reserved WORD field, must be 0
    POIEXT      pOIExt;                 // Optitem extension pointer
    ULONG_PTR   dwReserved[3];          // reserved DWORD fields (must be 0)
    } OPTITEM, *POPTITEM;

//
// OPTITEM is to describe each treeview item's name, selection type and
// possible selection
//
//  cbSize          - sizeof this structure
//
//  Level           - The level in the treeview, the root should have lowest
//                    number and number should start with level 0.  the maximum
//                    number of levels are 256.
//
//  DlgPageIdx      - Zero-based index to the DLGPAGE araay pointee by
//                    pDlgPage. The Maximum index is MAX_DLGPPAGE_COUNT, if
//                    pDlgPage is a standard CPSUI_PDLGPAGE_xxxx then this
//                    field is automatically set the common UI
//
//  Flags           - OPTIF_xxxx flags as describe above
//
//                      OPTIF_COLLAPSE
//
//                          Collaspe treeview item and its children so it is
//                          not expanded initially.
//
//
//                      OPTIF_HIDE
//
//                          Hide this item from the treeview
//
//
//                      OPTIF_CALLBACK
//
//                          Callback to the caller when user making some
//                          changes a pointer (pfnCallBack) must provided and
//                          process as defined by the common UI.
//
//
//                      OPTIF_CHANGED
//
//                          This item was changed and need to re-display. this
//                          flag only used when caller returned from callback
//                          funciton.
//
//
//                      OPTIF_CHANGEONCE
//
//                          This item has been changed at least once.
//
//
//                      OPTIF_DISABLED
//
//                          Disable this item so it become not selectable.
//
//
//                      OPTIF_ECB_CHECKED
//
//                          Specified the associated extended check box is
//                          in checked state.
//
//
//                      OPTIF_EXT_HIDE
//
//                          Hide the extended check box/extended push botton.
//
//
//                      OPTIF_EXT_DISABLED
//
//                          The Extended check box/push botton is disabled and
//                          not selectable
//
//
//                      OPTIF_SEL_AS_HICON
//
//                          This flag only used if this item has no type,
//                          pOptType=NULL that is, when pOptType is NULL then
//                          'Sel' field is the IconID. if flag is set then it
//                          indicate 'pSel' is the Icon handle rather
//                          than the icon resource ID specified in Sel.
//
//
//                      OPTIF_EXT_IS_EXTPUSH
//
//                          Specified that pExtPush should be used when this
//                          pointer is not NULL, if this pointer is not NULL
//                          and this flag is clear then pExtChkBox is assumed.
//
//
//                      OPTIF_NO_GROUPBOX_NAME
//
//                          Specified that do not overwrite the group box title
//                          text, if group box ID is defined. See the TVOT_xx
//                          description above.
//
//
//                      OPTIF_OVERLAY_WARNING_ICON
//
//                          If this bit is set then this header OPTITEM's icon
//                          will be overlaied by a common UI's
//                          IDI_CPSUI_WARNING icon.  This bit only used if this
//                          item has no type, pOptType is NULL that is.
//
//
//                      OPTIF_OVERLAY_STOP_ICON
//
//                          If this bit is set then this header OPTITEM's icon
//                          will be overlaied by a common UI's IDI_CPSUI_STOP
//                          icon.  This bit only used if this item has no type,
//                          pOptType is NULL that is.
//
//
//                      OPTIF_OVERLAY_NO_ICON
//
//                          If this bit is set then this header OPTITEM's icon
//                          will be overlaied by a common UI's IDI_CPSUI_NO
//                          icon.  This bit only used if this item has no type,
//                          pOptType is NULL that is.
//
//
//                      OPTIF_INITIAL_TVITEM
//
//                          Specified that this item will be the initial item
//                          to be selected and display on the treeview page.
//                          If the selected item is currently a child or
//                          collapse then common UI will expand the selection
//                          then scroll it into view.
//
//                          If this flag is clear or the set item is in hide
//                          status common UI will pick the initial item to
//                          display.
//
//                      OPTIF_HAS_POIEXT
//
//                          Specified that pOIExt field is a valid pointer that
//                          points to OIEXT data structure.  The pOIExt only
//                          used by the common UI if this bit is set.
//
//
//  UserData        - a 32-bit number used by the caller and common UI will not
//                    modify it.
//
//  pName           - Pointer to the item's name, such as 'Upper Tray',
//                    'Memory' or it is used as data as describe in
//                    OPTPARAM/OPTTYPE structure
//
//                      ** See LPTSTR description above
//
//  pSel
//  Sel             - Current selection for this item. This is a union field
//                    which can be a pointer to a string or a LONG index
//                    selection.
//
//                      ** If pOptType field is NULL then 'Sel' is the icon ID
//                         to be used for the header.
//
//  pExtPush
//  pExtChkBox      - Pointer to either EXTPUSH or EXTCHKBOX data structure,
//                    if this pointer is NULL then this item does not have
//                    ectended check box/push botton associate with it.
//
//                    When an extended check box is associate with the
//                    OPTTYPE, the OPTIF_EXT_IS_EXTPUSH must not set, the
//                    OPTIF_ECB_CHECKED flag specified if the extended check
//                    box is checked or not checked.
//
//                    When an Extended push botton is associated with the
//                    OPTTYPE, the OPTIF_EXT_IS_EXTPUSH flag must set.
//
//                    The following flags are used in both EXTCHKBOX or
//                    EXTPUSH
//
//                      OPTIF_EXT_HIDE,
//                      OPTIF_EXT_DISABLED
//                      OPTIF_EXT_CHANGEONCE
//
//
//  pOptType        - Pointer to the OPTTYPE structure to describe the display
//                    and selections of the item.   If this pointer is NULL
//                    then this this item does not have any selection. and it
//                    is used as sub-items' header.
//
//                      * When pOptType is NULL then the 'Sel' is the Icon ID.
//
//
//  HelpIndex       - a index to the help file for context sensitive help
//                    if HelpInex=0 then there is no help for this item
//
//  DMPubID         - specified if this item is one of the public fields in the
//                    DEVMODE structure and supported by the common UI.
//
//                      DMPUB_NONE              - Not DEVMODE public fields
//                      DMPUB_ORIENTATION       - dmOrientation
//                      DMPUB_SCALE             - dmScale
//                      DMPUB_COPIES_COLLATE    - dmCopies/dmCollate
//                      DMPUB_DEFSOURCE         - dmDefSource
//                      DMPUB_PRINTQUALITY      - dmPrintQuality
//                      DMPUB_COLOR             - dmColor
//                      DMPUB_DUPLEX            - dmDuplex
//                      DMPUB_TTOPTION          - dmTTOption
//                      DMPUB_FORMNAME          - dmFormName
//                      DMPUB_ICMMETHOD         - dmICMMethod
//                      DMPUB_ICMINTENT         - dmICMIntent
//                      DMPUB_MEDIATYPE         - dmMediaType
//                      DMPUB_DITHERTYPE        - dmDitherType
//
//                      ** for most of DMPUB_FIRST to DMPUB_LAST each OPTITEM's
//                         pName is automatically set to the standard
//                         IDS_CPSUI_xxx for the consistancy reason, the
//                         standard pName is set according to following table.
//
//                          DMPUB_ORIENTATION    - IDS_CPSUI_ORIENTATION
//                          DMPUB_SCALE          - IDS_CPSUI_SCALING
//                          DMPUB_COPIES_COLLATE - IDS_CPSUI_COPIES
//                          DMPUB_DEFSOURCE      - IDS_CPSUI_SOURCE
//                          DMPUB_PRINTQUALITY   - IDS_CPSUI_PRINTQUALITY
//                                                 IDS_CPSUI_RESOLUTION
//                          DMPUB_COLOR          - IDS_CPSUI_COLOR_APPERANCE
//                          DMPUB_DUPLEX         - IDS_CPSUI_DUPLEX
//                          DMPUB_TTOPTION       - IDS_CPSUI_TTOPTION
//                          DMPUB_FORMNAME       - IDS_CPSUI_FORMNAME
//                          DMPUB_ICMMETHOD      - IDS_CPSUI_ICMMETHOD
//                          DMPUB_ICMINTENT      - IDS_CPSUI_ICMINTENT
//                          DMPUB_MEDIATYPE      - IDS_CPSUI_MEDIA
//                          DMPUB_DITHERTYPE     - IDS_CPSUI_DITHERING
//
//                          for DMPUB_PRINTQUALITY, if the pName is not one of
//                          IDS_CPSUI_PRINTQUALITY or IDS_CPSUI_RESOLUTION then
//                          common UI will automatically default the pName to
//                          IDS_CPSUI_RESOLUTION.
//
//                          Each pData (OPTPARAM) selection in OPTPARAM which
//                          OPTITEM's pOptType pointed to should use as much
//                          as IDS_CPSUI_xxx standard name as possible.
//
//
//                      ** for DMPUB_COPIES_COLLATE the common UI automatically
//                         doing the following before the callback
//
//                          1) Enable/Disable the collate extended check box if
//                             OPTIF_EXT_HIDE is not specified and pExtChkBox
//                             is not NULL in the OPTITEM.
//
//                          2) Automatically change the postfix for this item
//                             to be 'Copy' if selection is one, and 'Copies'
//                             if selection is greater than one in the treeview
//                             page, and it will also set the postfix in
//                             standard document property page if the postfix
//                             ID is provided (BegCtrlID + 4)
//
//
//                      ** for DMPUB_COLOR the common UI automatically doing
//                         the following before the callback, the gray
//                         selection must be Sel=0 and Color slection must be
//                         Sel=1
//
//                          1) Calling halftone color adjustment with current
//                             color/mono selection
//
//                          2) Disable ICM when color is not selected
//
//
//                      ** Please see above DMPUB_xx description for details.
//
//  UserItemID      - This is a byte ID intented to be used by the caller to
//                    identify the item
//
//  wReserved       - WORD reserved. Must be zero
//
//  pOIExt          - Pointer to the OIEXT data structure to specified that
//                    it has a OPTITEM extenstion structure.
//
//  dwReserved[]    - DWORD reserved and must be 0
//
//

//
// predefined ID for call back reason
//

#define CPSUICB_REASON_SEL_CHANGED      0
#define CPSUICB_REASON_PUSHBUTTON       1
#define CPSUICB_REASON_ECB_CHANGED      2
#define CPSUICB_REASON_DLGPROC          3
#define CPSUICB_REASON_UNDO_CHANGES     4
#define CPSUICB_REASON_EXTPUSH          5
#define CPSUICB_REASON_APPLYNOW         6
#define CPSUICB_REASON_OPTITEM_SETFOCUS 7
#define CPSUICB_REASON_ITEMS_REVERTED   8
#define CPSUICB_REASON_ABOUT            9
#define CPSUICB_REASON_SETACTIVE        10
#define CPSUICB_REASON_KILLACTIVE       11


//
// predefined ID for call back action
//

#define CPSUICB_ACTION_NONE             0
#define CPSUICB_ACTION_OPTIF_CHANGED    1
#define CPSUICB_ACTION_REINIT_ITEMS     2
#define CPSUICB_ACTION_NO_APPLY_EXIT    3
#define CPSUICB_ACTION_ITEMS_APPLIED    4


typedef struct _CPSUICBPARAM {
    WORD        cbSize;             // size of this structure
    WORD        Reason;             // CPSUICB_REASON_XXXXX callback reason
    HWND        hDlg;               // handle of the dialog box
    POPTITEM    pOptItem;           // pOptItem field from COMPROPSHEETUI
    WORD        cOptItem;           // cOptItem field from COMPROPSHEETUI
    WORD        Flags;              // flags field from COMPROPSHEETUI
    POPTITEM    pCurItem;           // current selected item of callback
    union {
        LONG    OldSel;             // Last selection (index)
        LPTSTR  pOldSel;            // Last selection (pStr)
        } DUMMYUNIONNAME;
    ULONG_PTR   UserData;           // UserData in the COMPROPSHEETUI struct.
    ULONG_PTR   Result;             // OUT parameter for the APPLYNOW
    } CPSUICBPARAM, *PCPSUICBPARAM;


typedef LONG (APIENTRY *_CPSUICALLBACK)(PCPSUICBPARAM pCPSUICBParam);
#define CPSUICALLBACK   LONG APIENTRY


//
// CPSUICBPARAM is used when commom UI callback to the caller, this structure
// describe the nature of callback and passed all necessary parameter for the
// caller to make changes in the pOptItem and passed an action back to the
// commom UI to redisplay the tree or page 1 data
//
//
//  cbSize      - must be CPSUICBPARAM
//
//  Reason      - defined the nature of the callback
//
//                  CPSUICB_REASON_SEL_CHANGED
//
//                      User make change to the pCurItem.  if the item is
//                      DMPUB_COPIES_COLLATE then common UI automatically
//                      change the collate extended check box without callback
//                      to the caller of the extended check box changes
//
//
//                  CPSUICB_REASON_PUSHBUTTON
//
//                      User push the push button and push button item is set
//                      to PUSHBUTTON_TYPE_CALLBACK.
//
//
//                  CPSUICB_REASON_ECB_CHANGED
//
//                      User make change to the extended checked box (i.e. it
//                      eiterh checked or not checked) EXTCHKBOX in the
//                      pCurItem passed in the call back parameter.
//
//
//                  CPSUICB_REASON_DLGPROC
//
//                      The callback reason is PUSHBUTTON_TYPE_DLGPROC
//
//
//                  CPSUICB_REASON_UNDO_CHANGES
//
//                      This callback currently is not implmented.
//
//
//                  CPSUICB_REASON_EXTPUSH
//
//                      The callback is result of user push the extend push
//                      button.
//
//
//                  CPSUICB_REASON_APPLYNOW
//
//                      The user press the apply now button. Durning callback
//                      the pCurItem is set to equal to pOptItem in this
//                      structure and 'OldSel' is set to the active DlgPageIdx
//                      (compare to the OPTITEM's DlgPageIdx) which the page
//                      user hitting the apply now button if the page is
//                      non-treeview page, otherwise the 'OldSel' is set to -1
//                      (for treeview page) to indicate all valid item should
//                      be apply now, if the callback return
//                      CPSUICB_ACTION_NONE then the common UI will exit the
//                      property sheet and returned CPSUI_OK back to the
//                      caller, and if the callback function returned
//                      CPSUICB_ACTION_NO_APPLY_EXIT then common UI will not
//                      exit the property sheet and callback function must
//                      pop-up messages dialog box to tell user why it cannot
//                      exist the property sheet until certain action is take
//                      by the user.
//
//
//                  CPSUICB_REASON_OPTITEM_SETFOCUS
//
//                      This callback reason is used when an OPTITEM is getting
//                      the keyboard focus. and give the callback function a
//                      chance to examine the item.
//
//
//                  CPSUICB_REASON_ITEMS_REVERTED
//
//                      This callback reason is used when user changed items
//                      and decided to revert changes from the parent item in
//                      the treeview.  The callback funciton is called after
//                      all revertable items are reverted to its original.
//
//                      The CPSUICBPARAM's pCurItem is same as pOptItem and
//                      'OldSel' field is same as cOptItem field.  for each of
//                      reverted item, the OPTIF_CHANGED bit will be set in the
//                      OPTITEM by the common UI to indicate the item is revert
//                      by the common UI.   The callback function MUST NOT
//                      reset this bit if it is set.
//
//
//                  CPSUICB_REASON_ABOUT
//
//                      This callback reason is used when user hit 'About...'
//                      push button in the treeview page, and the flag
//                      CPSUIF_ABOUT_CALLBACK is set.  The pCurItem is set to
//                      same as pOptItem and 'pOldSel' is a pointer pointed to
//                      original copy of COMPROPSHEETUI data structure which
//                      passed to the common UI.
//
//
//                  CPSUICB_REASON_SETACTIVE
//                  CPSUICB_REASON_KILLACTIVE
//
//                      The current property sheet is gaining or loosing focus,
//                      the pCurItem is set to equal to pOptItem in this
//                      structure and 'OldSel' is set to the current active
//                      DlgPageIdx (compare to the OPTITEM's DlgPageIdx).
//                      if the page is non-treeview page, otherwise the
//                      'OldSel' is set to -1 (for treeview page).
//
//  hDlg        - The handle to the dialog box (Properties page TAB) current
//                active for the callback.
//
//                Durning the callback the caller must not change the
//                DWLP_USERDATA on hDlg, otherwise the common UI will be crash.
//                If callback function need to get/set DWLP_USERDATA it should
//                call common UI's SetCPSUIUserData() and GetCPSUIUserData()
//                functions instead.
//
//  pCurItem    - Pointed to POPTITEM which the callback is generated for.
//
//  pOldSel
//  OldSel      - The last OPTITEM's pSel/Sel field before the change was made
//                by the user.  The pOldSel/OldSel only valid if the callback
//                reason is CPSUICB_REASON_SEL_CHANGED, this give the callback
//                function a chance to check against the previous item
//                selection.  This is a union field which can be a pointer to
//                a string or a LONG index selection.
//
//  UserData    - a 32-bit user defined data in the COMPROPSHEETUI structure,
//                commom UI will not changed it.
//
//  Result      - When the reason is CPSUICB_REASON_APPLYNOW, the callback
//                function MUST set the requested result for the caller into
//                'Result' field when it returned a value other than the
//                CPSUICB_ACTION_NO_APPLY_EXIT and common UI will send the
//                'Result' field value to this page's parent.
//
//                The called function should save the current result of
//                pOptItem.  The default 'Result' is set to CPSUI_OK (1) from
//                common UI.  This function can alter this result before it
//                return back to to the common UI.
//
//
//  Return Values:
//
//      CPSUICB_ACTION_NONE             - No action need to be take by the
//                                        common UI.
//
//      CPSUICB_ACTION_OPTIF_CHANGED    - Ask the common UI to examine the
//                                        OPTIF_CHANGED flag in the OPTITEM
//                                        data structure.  if the flag is set
//                                        then that item is assume need to be
//                                        re-display because of OPTITEM's Flags
//                                        field changed or item's selection
//                                        changed.   This is different from
//                                        CPSUICB_ACTION_REINIT_ITEMS which
//                                        it assume OPTTYPE or OPTPARAM data
//                                        also changed.
//
//
//      CPSUICB_ACTION_REINIT_ITEMS     - Ask the common UI to examine the
//                                        OPTIF_CHANGED flag in the OPTITEM
//                                        data structure.  if the flag is set
//                                        then that item is assume need to be
//                                        re-initialized in the dilaog box
//                                        control.  This happened if item's
//                                        OPTTYPE or OPTPARAMs flag/pdata
//                                        changed.
//
//
//      CPSUICB_ACTION_NO_APPLY_EXIT    - This return value only valid durning
//                                        CPSUICB_REASON_APPLYNOW callback
//                                        reason, it tell common UI it has
//                                        some constraints in its OPTITEM which
//                                        must correct or confirm by the user
//                                        before exit.  The callback function
//                                        must display and/or have user taking
//                                        some actions before return this
//                                        action to the common UI
//
//      CPSUICB_ACTION_ITEMS_APPLIED    - When responsed to the
//                                        CPSUICB_REASON_APPLYNOW, if the
//                                        returned action is
//                                        CPSUICB_ACTION_ITEMS_APPLIED then
//                                        common UI will reset OPTIF_CHANGEONCE
//                                        bit and save the new default for the
//                                        future undo operations.
//
//

#define DP_STD_TREEVIEWPAGE             0xFFFF
#define DP_STD_DOCPROPPAGE2             0xFFFE
#define DP_STD_DOCPROPPAGE1             0XFFFD
#define DP_STD_RESERVED_START           0xFFF0

#define MAX_DLGPAGE_COUNT               64

#define DPF_ICONID_AS_HICON             0x0001
#define DPF_USE_HDLGTEMPLATE            0x0002

typedef struct _DLGPAGE {
    WORD        cbSize;         // size of this structure
    WORD        Flags;          // DPF_xxxx flags
    DLGPROC     DlgProc;        // caller's dialog box subclass procedue
    LPTSTR      pTabName;       // pointer to the tab name
    ULONG_PTR   IconID;         // icon to be used
    union {
        WORD    DlgTemplateID;  // dialog box template ID
        HANDLE  hDlgTemplate;   // handle to the dialog template
        } DUMMYUNIONNAME;
    } DLGPAGE, *PDLGPAGE;


//
// DLGPAGE structure describe non-treeview page characteristics
//
//
//  cbSize          - size of this structure
//
//  Flags           - DPF_xxxx flags
//
//                      DPF_ICONID_AS_HICON
//
//                          If this flag is set then IconID DWORD field is
//                          treated as a handle to the icon rather then the
//                          resource ID
//
//
//  DlgProc         - caller's supplied DLGPROC for sub-class the page
//                    dialog box processing,
//
//                    if DlgProc is not NULL then common UI do the following
//                    according the the message received except for the
//                    DP_STD_xxx pages
//
//
//                      WM_INITDIALOG
//
//                          Common UI initialize the dialog box and then call
//                          DlgProc(WM_INITDIALOG) the DlgProc should return
//                          exactly the behavior for the WM_INITDIALOG
//
//                          The lParam in the WM_INITDIALOG data structure is
//                          a pointer to the PROPSHEETPAGE data structure.
//
//                          the lParam field in the PROPSHEETPAGE (lParam
//                          passed to the WM_INITDIALOG) is the UserData
//                          defined in COMPROPSHEETUI data structure
//
//                          To access to the PSPINFO data structure which
//                          associate with this page, use the common UI macro
//                          PPSPINFO_FROM_WM_INITDIALOG_LPARAM(lParam) where
//                          lParam is the parameter passed to the
//                          WM_INITDIALOG message.
//
//                          The subclass function should save these pointers
//                          for its later use, but it MUST NOT modified the
//                          content of the PSPINFO data structure or system
//                          may crashed.
//
//
//                      OTHER DIALOG MESSAGES
//
//                          Iit call DlgProc() and if it returned the value is
//                          non-zero then common UI assume DlgProc() processed
//                          the message and will not process this message.
//
//                          If the returned vlaue from DlgProc() is zero then
//                          common UI will process this message.
//
//
//                      * Durning the DlgProc the caller must not change the
//                        DWLP_USERDATA on hDlg, otherwise the common UI will
//                        be crash.  If caller need to get/set DWLP_USERDATA it
//                        should call common UI's SetCPSUIUserData() and
//                        GetCPSUIUserData() instead.
//
//  pTabName        - Pointer to a string to describe the TAB title
//
//  IconID          - This is the icon identifier, which can be a common
//                    strandard IDI_CPSUI_xxx icon ID, caller's own icon
//                    resource ID, or a handle to the caller defined icon if
//                    DPF_ICONID_AS_HICON flag is set, in any case if the
//                    IconID is zero then it indicated no icon.
//
//  DlgTemplateID   - The template id to be use for the ProPage, it can be
//                    one of DP_STD_xxxx, the DP_STD_xxx has 240 x 240 dialog
//                    box units.
//
//
//                      DP_STD_TREEVIEWPAGE
//
//                          Specified that this page is a standard treeview
//                          page provided by the common ui.  The treeview page
//                          is a page using treeview display all valid OPTITEM
//                          passed to the common UI.  User can modify every
//                          valid selectable OPTITEM from the treeview page.
//
//                      DP_STD_DOCPROPPAGE
//
//                          Specified that this page is a standard document
//                          property page provided by the common UI
//
//  hDlgTemplate    - Handle to the DLGTEMPLATE which will be use for pop up
//                    dialog box
//
//
//  wReserved[]
//  dwReserved[]    - Reserved fields, must be 0
//
//
//  ** Tips of designing the dialog box controls
//
//      When designing the dialog box controls, each OPTITEM is correspoonds
//      to one input control plus one extended check box or extended push
//      button.  Since common UI will automatically disable and remove
//      OPTIF_HIDE items item from the property sheet and dynamically move
//      other controls, the following tips of designing the dialog box controls
//      should follow.
//
//      * Each item should have one input control plus optional of extended
//        check box/push botton, one icon control and other static controls
//
//      * Each item should occupied whole horizontal spaces of the property
//        sheet, items must not overlay in vertical direction.
//
//      * for TVOT_2STATES, TVOT_3STATES, if it araange radio buttons from
//        left to right in state order (ie. from first state's OPTPARAM to
//        last state's OPTPARAM) then the radio buttons and icons should
//        aligned in the Y coordinate.    If it arrange radio buttons from top
//        to bottom (ie. from first state's OPTPARAM to last state's OPTPARAM)
//        then the radio buttons and icons should aligned in the X coordinate.
//
//        common UI will automatically hide/move the radio buttons to compact
//        the dialog box controls.  If radio buttons/icons are arranged in
//        top/down order and there is other controls obscure in Y direction
//        then radio buttons will only be re-arranged but not remove any white
//        spaces in Y direction.
//
//      * If multiple items shared one group box, then the group box must
//        belongs to the first item (topmost in the dialog box group) in the
//        group, the group box must large enough to cover all the items in
//        side the group box.
//
//


#define CPSUIF_UPDATE_PERMISSION        0x0001
#define CPSUIF_ICONID_AS_HICON          0x0002
#define CPSUIF_ABOUT_CALLBACK           0x0004

#define CPSUI_PDLGPAGE_DOCPROP          (PDLGPAGE)1
#define CPSUI_PDLGPAGE_ADVDOCPROP       (PDLGPAGE)2
#define CPSUI_PDLGPAGE_PRINTERPROP      (PDLGPAGE)3
#define CPSUI_PDLGPAGE_TREEVIEWONLY     (PDLGPAGE)4

//
// For compatible misspelling #define
//

#define CPSUI_PDLGPAGE_TREEVIWONLY      CPSUI_PDLGPAGE_TREEVIEWONLY

typedef struct _COMPROPSHEETUI {
    WORD            cbSize;             // size of this structure
    WORD            Flags;              // CPSUIF_xxxx flags
    HINSTANCE       hInstCaller;        // caller's hInstance
    LPTSTR          pCallerName;        // pointer to the caller's name
    ULONG_PTR       UserData;           // caller's own data
    LPTSTR          pHelpFile;          // pointer to the help file
    _CPSUICALLBACK  pfnCallBack;        // callback function pointer
    POPTITEM        pOptItem;           // pointer to POPTITEM array
    PDLGPAGE        pDlgPage;           // pointer to the DLGPAGE array
    WORD            cOptItem;           // count of pOptItem array
    WORD            cDlgPage;           // count of pDlgPage array
    ULONG_PTR       IconID;             // icon to be used
    LPTSTR          pOptItemName;       // pointer to the optitem's data name
    WORD            CallerVersion;      // version for the caller apps
    WORD            OptItemVersion;     // version for the optitem name
    ULONG_PTR       dwReserved[4];      // reserved, must be 0
    } COMPROPSHEETUI, *PCOMPROPSHEETUI;


//
// COMPROPSHEETUI data structure is used when calling common UI to display dialog
// box of properties pages.
//
//
//  Size                - Must be sizeof (COMPROPSHEETUI)
//
//  Flags               - can be one or more of following
//
//                          CPSUIF_UPDATE_PERMISSION
//
//                              Specified the any valid pOptItem items are
//                              changeable by the user.
//
//
//                          CPSUIF_ICONID_AS_HICON
//
//                              If this flag is set then IconID DWORD field is
//                              treated as a handle to the icon rather then the
//                              resource ID
//
//
//                          CPSUIF_ABOUT_CALLBACK
//
//                              If this flag bit is set, then when user hit
//                              'About...' button in the treeview tab, it will
//                              call the callback function with a reason of
//                              CPSUICB_REASON_ABOUT, and callback MUST handle
//                              the about which pop-up dialog box to show user
//                              the information about the caller and OPTITEMs.
//
//                              If this bit is not set then common UI will call
//                              the ShellAbout() with formatted caller Name and
//                              pOptItemName with version numbers.
//
//
//  hInstCaller         - the caller's handle to its instance.  Commom UI use
//                        this handle to load caller's icon and other resources.
//
//  pCallerName         - Pointer to the caller's NULL terminated caller's
//                        name, most time this is driver's name,
//                        such as 'Postscript Driver'
//
//  UserData            - a 32-bit number used by the caller and common UI will
//                        not modify it.  this 32-bit number is passed back to
//                        the caller durning the callback function
//
//  pHelpFile           - specified a standard microsoft help file (path/file)
//                        for using in the common UI.  in OPTITEM specified
//                        HelpIndex for help in each item.
//
//  pfnCallBack         - Pointer to _CPSUICALLBACK callback function.  Common
//                        UI only callback to the caller if an OPTIF_CALLBACK
//                        is set OPTITEM data structure's flag fields and the
//                        item selection is changed by the user.
//
//  pOptItem            - Pointer to array of OPTITEM structure to be displayed
//                        by the common UI
//
//  pDlgPage            - Pointer to array of DLGPAGE structure to describe
//                        each property sheet page infomation, the following
//                        are the standard common ui DLGPAGEs.  When specified
//                        CPSUI_PDLGPAGE_xxxx, the common UI will automatically
//                        modify DlgPageIdx field in the OPTITEM, caller must
//                        set the iStartPage correctly.
//
//                          CPSUI_PDLGPAGE_DOCPROP
//
//                              specified this a common ui standard document
//                              property sheets. This includes two property
//                              sheets, 1) Page Setup 2) Advance (TreeView)
//
//
//                          CPSUI_PDLGPAGE_ADVDOCPROP
//
//                              Specified this is a treeview page only UI
//                              provided by the common UI, this only has one
//                              treeview page with tab of 'Advance'
//
//
//                          CPSUI_PDLGPAGE_PRINTERPROP
//
//                              Specified this is a common UI standard printer
//                              property sheet.  This only has one treeview
//                              page with tab of 'Device Options'
//
//
//                          CPSUI_PDLGPAGE_TREEVIEWONLY
//
//                              Specified this is a treeview page only UI
//                              provided by the common UI, this only has one
//                              treeview page
//
//
//
//  cOptItem            - Count of OPTITEM pointed by the pOptItem above
//
//  cDlgPage            - Count of DLGPAGE pointed by the pDlgPage.  If
//                        pDlgPage is one of the CPSUI_PDLGPAGE_xxxx then this
//                        field is ignored by the common UI.
//
//  IconID              - This is the icon identifier, which can be a common
//                        strandard IDI_CPSUI_xxx icon ID, caller's own icon
//                        resource ID, or a handle to the caller defined icon
//                        if CPSUIF_ICONID_AS_HICON flag is set, in any case
//                        if the IconID is zero then it indicated no icon.
//
//  pOptItemName        - Pointer to the pOptItem data NULL terminated name,
//                        most time this is device name, such as 'HP 4si'
//
//  CallerVersion       - Version for the caller, the HIBYTE(CallerVersion) is
//                        the major version, and LOBYTE(CallerVersion) is the
//                        minor version, such as 0x310 display as 3.16, 0x3ff
//                        display as 3.255 and 0x30a display as 3.10
//
//  OptItemVersion      - Version for the OPTITEM's data, the
//                        HIBYTE(OptItemVersion) is the major version, and
//                        LOBYTE(OptItemVersion) is the minor version, such as
//                        0x310 display as 3.16, 0x3ff display as 3.255 and
//                        0x30a display as 3.10.
//
//  dwReserved[4]       - reserved fields, must be 0
//
//
//  ** pTitlee and TitleBarIcon only used if CommonPrinterPropSheetUI()
//     is the last one the Property sheet UI chain and call the PropertySheet()
//
//


#define CPSFUNC_ADD_HPROPSHEETPAGE          0
#define CPSFUNC_ADD_PROPSHEETPAGEW          1
#define CPSFUNC_ADD_PCOMPROPSHEETUIA        2
#define CPSFUNC_ADD_PCOMPROPSHEETUIW        3
#define CPSFUNC_ADD_PFNPROPSHEETUIA         4
#define CPSFUNC_ADD_PFNPROPSHEETUIW         5
#define CPSFUNC_DELETE_HCOMPROPSHEET        6
#define CPSFUNC_SET_HSTARTPAGE              7
#define CPSFUNC_GET_PAGECOUNT               8
#define CPSFUNC_SET_RESULT                  9
#define CPSFUNC_GET_HPSUIPAGES              10
#define CPSFUNC_LOAD_CPSUI_STRINGA          11
#define CPSFUNC_LOAD_CPSUI_STRINGW          12
#define CPSFUNC_LOAD_CPSUI_ICON             13
#define CPSFUNC_GET_PFNPROPSHEETUI_ICON     14
#define CPSFUNC_ADD_PROPSHEETPAGEA          15
#define CPSFUNC_INSERT_PSUIPAGEA            16
#define CPSFUNC_INSERT_PSUIPAGEW            17
#define CPSFUNC_SET_PSUIPAGE_TITLEA         18
#define CPSFUNC_SET_PSUIPAGE_TITLEW         19
#define CPSFUNC_SET_PSUIPAGE_ICON           20
#define CPSFUNC_SET_DATABLOCK               21
#define CPSFUNC_QUERY_DATABLOCK             22
#define CPSFUNC_SET_DMPUB_HIDEBITS          23
#define CPSFUNC_IGNORE_CPSUI_PSN_APPLY      24
#define CPSFUNC_DO_APPLY_CPSUI              25

#if (NTDDI_VERSION >= NTDDI_WINXP)
#define CPSFUNC_SET_FUSION_CONTEXT          26
#define MAX_CPSFUNC_INDEX                   26
#else
#define MAX_CPSFUNC_INDEX                   25
#endif

#ifdef UNICODE
#define CPSFUNC_ADD_PCOMPROPSHEETUI         CPSFUNC_ADD_PCOMPROPSHEETUIW
#define CPSFUNC_ADD_PFNPROPSHEETUI          CPSFUNC_ADD_PFNPROPSHEETUIW
#define CPSFUNC_LOAD_CPSUI_STRING           CPSFUNC_LOAD_CPSUI_STRINGW
#define CPSFUNC_ADD_PROPSHEETPAGE           CPSFUNC_ADD_PROPSHEETPAGEW
#define CPSFUNC_INSERT_PSUIPAGE             CPSFUNC_INSERT_PSUIPAGEW
#define CPSFUNC_SET_PSUIPAGE_TITLE          CPSFUNC_SET_PSUIPAGE_TITLEW

#else
#define CPSFUNC_ADD_PCOMPROPSHEETUI         CPSFUNC_ADD_PCOMPROPSHEETUIA
#define CPSFUNC_ADD_PFNPROPSHEETUI          CPSFUNC_ADD_PFNPROPSHEETUIA
#define CPSFUNC_LOAD_CPSUI_STRING           CPSFUNC_LOAD_CPSUI_STRINGA
#define CPSFUNC_ADD_PROPSHEETPAGE           CPSFUNC_ADD_PROPSHEETPAGEA
#define CPSFUNC_INSERT_PSUIPAGE             CPSFUNC_INSERT_PSUIPAGEA
#define CPSFUNC_SET_PSUIPAGE_TITLE          CPSFUNC_SET_PSUIPAGE_TITLEA

#endif

//
// for the CPSFUNC_SET_RESULT
//

#define SR_OWNER            0
#define SR_OWNER_PARENT     1

typedef struct _SETRESULT_INFO {
    WORD        cbSize;
    WORD        wReserved;
    HANDLE      hSetResult;
    LRESULT     Result;
    } SETRESULT_INFO, *PSETRESULT_INFO;

//
// This is for CPSFUNC_INSERT_PSUIPAGE
//

#define HINSPSUIPAGE_FIRST              (HANDLE)0xFFFFFFFE
#define HINSPSUIPAGE_LAST               (HANDLE)0xFFFFFFFF
#define HINSPSUIPAGE_INDEX(i)           (HANDLE)MAKELONG(i, 0);

#define PSUIPAGEINSERT_GROUP_PARENT     0
#define PSUIPAGEINSERT_PCOMPROPSHEETUI  1
#define PSUIPAGEINSERT_PFNPROPSHEETUI   2
#define PSUIPAGEINSERT_PROPSHEETPAGE    3
#define PSUIPAGEINSERT_HPROPSHEETPAGE   4
#define PSUIPAGEINSERT_DLL              5
#define MAX_PSUIPAGEINSERT_INDEX        5


#define INSPSUIPAGE_MODE_BEFORE         0
#define INSPSUIPAGE_MODE_AFTER          1
#define INSPSUIPAGE_MODE_FIRST_CHILD    2
#define INSPSUIPAGE_MODE_LAST_CHILD     3
#define INSPSUIPAGE_MODE_INDEX          4


typedef struct _INSERTPSUIPAGE_INFO {
    WORD        cbSize;
    BYTE        Type;
    BYTE        Mode;
    ULONG_PTR   dwData1;
    ULONG_PTR   dwData2;
    ULONG_PTR   dwData3;
    } INSERTPSUIPAGE_INFO, *PINSERTPSUIPAGE_INFO;


//
// for the CPSFUNC_SET_HSTARTPAGE
//

#define SSP_TVPAGE          10000
#define SSP_STDPAGE1        10001
#define SSP_STDPAGE2        10002

typedef LONG_PTR (CALLBACK *PFNCOMPROPSHEET)(HANDLE hComPropSheet,
                                             UINT   Function,
                                             LPARAM lParam1,
                                             LPARAM lParam2);


typedef struct _PSPINFO {
    WORD            cbSize;
    WORD            wReserved;
    HANDLE          hComPropSheet;
    HANDLE          hCPSUIPage;
    PFNCOMPROPSHEET pfnComPropSheet;
    } PSPINFO, *PPSPINFO;

//
// PPSPINFO_FROM_WM_INITDIALOG_LPARAM(lParam) macro retrieve a pointer to the
// PSPINFO data structure. the lParam must be the lParam passed to the
// WM_INITDIALOG, otherwise the system can failed
//

#define PPSPINFO_FROM_WM_INITDIALOG_LPARAM(lParam)  \
                (PPSPINFO)((LPBYTE)lParam + ((LPPROPSHEETPAGE)lParam)->dwSize)

//
// PSPINFO
//
//  This structure is used durning property sheet page's WM_INITDIALOG message.
//  At WM_INITDIALOG, the lParam is a pointer to the PROPSHEETPAGE, and
//  lParam field in the PROPSHEETPAGE is a pointer to the PSPINFO.  the
//  original lParam in the PROPSHEETPAGE is saved in the lParam field in the
//  PSPINFO data structure.
//
//  When process WM_INITDIALOG message, it should save the lParam (PSPINFO
//  structure pointer) for later to call common UI callback functions.
//
//
//  cbSize          - Size of this structure in bytes
//
//  wReserved       - Reserved, must be set to zero
//
//  hComPropSheet   - Handle to the parent page which is the hComPropSheet
//                    passed to the CPSFUNC_ADD_PROPSHEETPAGE
//
//  hCPSUIPage      - Handle to the this added common UI property sheet page.
//
//  pfnComPropSheet - Pointer to the common UI callback function, using this
//                    function pointer to do CPSFUNC_xxxx
//
//

typedef struct _CPSUIDATABLOCK {
    DWORD   cbData;
    LPBYTE  pbData;
    } CPSUIDATABLOCK, *PCPSUIDATABLOCK;


#define APPLYCPSUI_NO_NEWDEF        0x00000001
#define APPLYCPSUI_OK_CANCEL_BUTTON 0x00000002

//
// PFNCOMPROPSHEET function descriptions
//
// For each function index, it passed a handle, a Function Index and two (2)
// long parameters, the 'hComPropSheet' handle passed must be the handle passed
// from common UI when common UI called the caller supplied function
//
// pfnPropSheetUI(pPropSheetUIData);
//
// Following are the description of each function index
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_ADD_HPROPSHEETPAGE
//
//      This function add a page to the hComPropSheet using handle to the
//      PROPSHEETPAGE
//
//
//  Parameters:
//
//      lParam1 - is a handle to the PROPSHEETPAGE that created by the caller
//                using CreatePropertySheetPage()
//
//      lParam2 - Not used, must be 0
//
//
//  Return Value:
//
//      The return value is the handle of newly added common property sheet
//      page, if return value is NULL then function failed.
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_ADD_PROPSHEETPAGE
//
//      This function add a page to the hComPropSheet using PROPSHEETPAGE
//      data structure.
//
//
//  Parameters:
//
//      lParam1 - is a pointer to PROPSHEETPAGE data structure
//
//      lParam2 - Not used.
//
//
//  Return Value:
//
//      The return value is the handle of newly added common property sheet
//      page, if return value is NULL then function failed.
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_ADD_PCOMPROPSHEETUI
//
//      This function add propety page(s) to the hComPropSheet handle using
//      COMPROPSHEETUI data structure.
//
//
//  Parameters:
//
//      lParam1 - is a pointer to COMPROPSHEETUI data structure
//
//      lParam2 - pointer to a 32-bit location that received the total pages
//                added by the COMPROPSHEETUI data structure if sucessful else
//                it contains the ERR_CPSUI_xxx error codes.
//
//
//  Return Value:
//
//      The return value is the handle of newly added common property sheet
//      page(s), if return value is NULL then function failed.
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_ADD_PFNPROPSHEETUI
//
//      This function add property page(s) to the hChild handle using
//      lParam1 as PFNPROPSHEETUI function pointer and lParam2 as the function
//      parameter.   The common UI call supplied function as following
//
//          PROPSHEETUI_INFO    PSUIInfo;
//
//          PSUIInfo.cbSize          = sizeof(PROPSHEETUI_INFO);
//          PSUIInfo.Version         = PROPSHEETUI_INFO_VERSION;
//          PSUIInfo.Flags           = (Ansi) ? 0: PSUIINFO_UNICODE;
//          PSUIInfo.Reason          = PROPSHEETUI_REASON_INIT;
//          PSUIInfo.hComPropSheet   = hComPropSheet;
//          PSUIInfo.pfnComPropSheet = ComPropSheetUICallBack;
//          PSUIInfo.lParamInit      = lParam2;
//          PSUIInfo.UserData        = 0;
//          PSUIInfo.Result          = 0;
//
//          ((PFNPROPSHEETUI)lParam1)(&PSUIInfo, lParam2);
//
//
//          If the pfnPropSheetUI() need to add/delete any common UI pages then
//          it must use hComPropSheet as its handle when calling the
//          ComPropSheetUICallBack().
//
//
//  Parameters:
//
//      lParam1 - a PFNPROPSHEETUI function pointer.
//
//      lParam2 - a 32-bit data that will be used as lParam when calling
//                PFNPROPSHEETUI function pointer.
//
//
//  Return Value:
//
//      The return value is the newly added property pages function handle, if
//      return value is NULL then function failed or no page is added.
//
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_DELETE_HCOMPROPSHEET
//
//      This function delete child property page(s) from hComPropSheet parent
//      using the child handle passed.
//
//
//  Parameters:
//
//      lParam1 - the handle of common property sheet pages that to be deleted.
//                This handle must be the handle returned from CPSFUNC_ADD_xxx
//                functions.
//
//      lParam2 - not used, must be 0
//
//  Return Value:
//
//      The return value is greater than zero if function sucessful, and less
//      or equal to zero if the function failed.
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_GET_PAGECOUNT
//
//      This function return total property sheet pages belongs to a common
//      UI property sheet page handle hComPropSheet
//
//
//  Parameters:
//
//      lParam1 - not used, must be 0
//
//      lParam2 - not used, must be 0.
//
//
//  Return Value:
//
//      The return value is total page count if function sucessful or zero if
//      function failed.
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_SET_RESULT
//
//      This function set the result of property sheet page to its owner that
//      added this page by CPSFUNC_ADD_xxx function indices
//
//  Parameters:
//
//      lParam1 - Handle to the common UI property sheet page that setting the
//                result.  If this handle is NULL then it is treated as
//                equal to the hComPropSheet.
//
//      lParam2 - a 32-bit DWORD result to be set.
//
//
//  Return Value:
//
//      > 0: Successful, return value is total count of parents set the result
//      = 0: There is no owner or parent for the lParam1 handle.
//      < 0: function failed because of invalid lParam1 handle.
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_SET_HSTARTPAGE
//
//
//  Parameters:
//
//      lParam1 - the handle of common property sheet pages that to be set
//                as initial page that appear when the property sheet dialog
//                boxes is created.  This handle must be the handle returned
//                from CPSFUNC_ADD_xxx functions.
//
//                If lParam1 is NULL then lParam2 is a string pointer to the
//                page name (tab) that to be set for the start page
//
//      lParam2 - a LONG number to specified the children index. if lParam1
//                handle is a parent then lParam2 specified zero base children
//                index for using as start page.
//
//                It also can be one of following special index
//
//                  SSP_TVPAGE
//
//                      set to the treeview page, this only valid if lParam1
//                      handle was added by the CPSFUNC_ADD_PCOMPROPSHEETUI.
//
//                  SSP_STDPAGE
//
//                      Set to the standard document property sheet page
//                      (Page 1 user friendly page).  this only valid if
//                      lParam1 handle was added by the
//                      CPSFUNC_ADD_PCOMPROPSHEETUI.
//
//
//  Return Value:
//
//      The return value is greater than zero if function sucessful, and less
//      or equal to zero if the function failed.  This function can only be
//      called when the property sheet is not display yet so the place to
//      call is during the PROPSHEETUI_REASON_GET_INFO_HEADER callback.
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_GET_HPSUIPAGES
//
//      This function return array of children HPROPSHEETPAGE belongs to
//      the parent hComPropSheet UI property sheet page handle hComPropSheet
//
//
//  Parameters:
//
//      lParam1 - Pointer to an array of HPROPSHEETPAGE that to be stored
//                the handle upon return.
//
//      lParam2 - Count of HPROPSHEETPAGE array pointed by the lParam1
//
//
//  Return Value:
//
//      The return value is total HPROPSHEETPAGE stored in the array pointed
//      by the lParam1.   To get all hPropSheetPage for any common property
//      sheet handle's (hCPSUIPage) children, it can use following sequence.
//
//          if ((cPage = pfnComPropSheet(hComPropSheet,
//                                       CPSFUNC_GET_PAGECOUNT,
//                                       (LPARAM)hCPSUIPage,
//                                       0L))   &&
//              (phPage = ALLOCMEM(cPage * sizeof(HANDLE)))) {
//
//              pfnComPropSheet(hCPSUIPage,
//                              CPSFUNC_GET_HPSUIPAGES,
//                              (LPARAM)phPage,
//                              (LPARAM)cPage);
//          }
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_LOAD_CPSUI_STRING
//
//      This function load the common property sheet UI resource string
//
//  Parameters:
//
//      lParam1 - Pointer to LPTSTR string which will stored the loaded
//                resource string from the common property sheet UI DLL.
//
//      lParam2 - LOWORD(lParam2) = Count of characters (includes null
//                                  terminator) which pointed by the lParam1
//
//                HIWORD(lParam2) = Common property sheet UI predefined string
//                                  resource ID as IDS_CPSUI_xxxx
//
//
//  Return Value:
//
//      > 0: Total characters stored in the string pointed by the lParam1, this
//           is not includes the null terminator
//      = 0: Invalid IDS_CPSUI_xxx passed from HIWORD(lParam)
//      < 0: Either lParam1 is NULL or count of character is 0 from
//           LOWORD(lParam2)
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_LOAD_CPSUI_ICON
//
//      This function load the common property sheet UI resource icon.
//
//  Parameters:
//
//      lParam1 - Common property sheet UI predefined icon resource ID as
//                IDI_CPSUI_xxxx.
//
//      lParam2 - LOWORD(lParam2) = cx icon size in pixel.  If zero then
//                                  SM_CXICON is used
//                HIWORD(lParam2) = cy icon size in pixel.  If zero then
//                                  SM_CYICON is used
//
//
//  Return Value:
//
//      Return value is the handle to the hIcon if function succeed, a NULL
//      if function failed. The caller must call DestroyIcon() when it no
//      longer need to use the hIcon returned
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_GET_PFNPROPSHEETUI_ICON
//
//      This function let the caller return hIcon of its children pages that
//      was added by CPSFUNC_ADD_PFNPROPSHEETUI
//
//  Parameters:
//
//      lParam1 - Handle of common property sheet pages that the hIcon will be
//                queried.  This handle must be the handle returned from
//                CPSFUNC_ADD_PFNPROPSHEETUI function.
//
//      lParam2 - LOWORD(lParam2) = cx icon size in pixel.  If zero then
//                                  SM_CXICON is used
//                HIWORD(lParam2) = cy icon size in pixel.  If zero then
//                                  SM_CYICON is used
//
//
//  Return Value:
//
//      Return value is the handle to the hIcon if function succeed, a NULL
//      if function failed. The caller must call DestroyIcon() when it no
//      longer need to use the hIcon returned
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_INSERT_PSUIPAGE
//
//      This function let the caller insert common property sheet pages at
//      set position.  The hComPropSheet must be the parent handle.
//
//  Parameters:
//
//      lParam1 - Handle of common property sheet pages that the page position
//                will be inserted.  This handle must be the handle returned
//                from previous CPSFUNC_ADD_xxx or CPSFUNC_INSERT_PSUIPAGE that
//                added or inserted using the hComPropSheet if the Mode field
//                in INSPSUIPAGE_INFO data structure is one of the follwing
//
//                  INSPSUIPAGE_MODE_BEFORE
//                  INSPSUIPAGE_MODE_AFTER      - Common UI page handle
//
//                  INSPSUIPAGE_MODE_INDEX      - the lParam1 is an zero based
//                                                child index.
//
//                  INSPSUIPAGE_MODE_FIRST_CHILD
//                  INSPSUIPAGE_MODE_LAST_CHILD - The lParam1 is ignonred.
//
//
//                If lParam1 is a valid common property sheet page handle then
//                it is  the child page handle of hComPropSheet that added by
//                CPSFUNC_ADD_xxx or inserted by CPSFUNC_INSERT_PSUIPAGE.
//
//      lParam2 - A pointer that points to INSERTPSUIPAGE_INFO data structure.
//                Fields must set according to the following.
//
//                  cbSize  - size of this structure
//
//                  Type    - Type of page(s) to be inserted.  It can be one
//                            of the following
//
//                      PSUIPAGEINSERT_GROUP_PARENT
//
//                          Insert a group parent that can be used to insert
//                          new pages under it.  This is typically used when
//                          a set of common UI pages must be group together and
//                          can be deleted later using a single group parent
//                          handle without individual deleting each page.
//
//                          This handle can be nested.  After this function
//                          returned the group parent handle, it can be used
//                          as hComPropSheet handle (first parameter in the
//                          common UI callback) as parent handle so insertion
//                          will be inserted at level below returned group
//                          parent handle.
//
//
//                      PSUIPAGEINSERT_PCOMPROPSHEETUI
//
//                          Insert pages using COMPROPSHEETUI data structure,
//                          dwData1 is a pointer to the COMPROPSHEETUI data
//                          structure.
//
//
//                      PSUIPAGEINSERT_PFNPROPSHEETUI
//
//                          Insert pages using PFNPROPSHEETUI function pointer.
//                          The dwData1 is a PFNPROPSHEETUI function pointer.
//                          The common UI will call this pfnPropSheetUI()
//                          function pointer with PROPSHEETUI_REASON_INIT to
//                          have it add pages.
//
//                          When common UI call pfnPropSheetUI() (dwData1) it
//                          also passed a 32-bit parameter from the dwData2
//                          field in INSERTPSUIPAGE_INFO data structure.
//
//
//                      PSUIPAGEINSERT_PROPSHEETPAGE
//
//                          Insert pages using PROPSHEETPAGE data structure.
//                          The dwData1 is a pointer to the PROPSHEETPAGE
//                          data structure.
//
//
//                      PSUIPAGEINSERT_HPROPSHEETPAGE
//
//                          Insert pages using HPROPSHEETPAGE handle.  The
//                          dwData1 is a PROPSHEETPAGE handle which was
//                          created by CreatePropertySheetPage().
//
//
//                      PSUIPAGEINSERT_DLL
//
//                          Insert pages from a dynnmaic link library.  The
//                          dwData1 is a pointer to a null terminated string
//                          that specified the dynamic link library file name.
//
//                          The dwData2 is a pointer to a null terminated ASCII
//                          string that specified the pfnPropSheetUI function
//                          name. (MUST BE ASCII STRING)
//
//                          Common UI will do a LoadLibrary((LPTSTR)dwData1),
//                          pfnPropSheetUI = GetProcAddress((LPTSTR)dwData2)
//                          then call the pfnPropSheetUI with a lParam from
//                          dwData3.  The called reason from common UI is
//                          set to PROPSHEETUI_REASON_INIT.
//
//                          Using this method insert pages will guaranteed that
//                          library will be unload correctly.
//
//
//                  Mode    - Insert Mode, it can be one of the following
//
//
//                      INSPSUIPAGE_MODE_BEFORE
//
//                          Insert pages before the common property sheet page
//                          handle specified by lParam1
//
//
//                      INSPSUIPAGE_MODE_AFTER
//
//                          Insert pages after the common property sheet page
//                          handle specified by lParam1
//
//
//                      INSPSUIPAGE_MODE_FIRST_CHILD
//
//                          Insert pages as the first child of hComPropSheet
//                          parent handle, the lParam1 is ignored
//
//
//                      INSPSUIPAGE_MODE_LAST_CHILD
//
//                          Insert pages as the last child of hComPropSheet
//                          parent handle, the lParam1 is ignored
//
//
//                      INSPSUIPAGE_MODE_INDEX
//
//                          Insert pages as a zero base child index of its
//                          parent handle specified by hComPropSheet.
//
//                          The lParam1 is the zero based index special handle
//                          that must generated by HINSPSUIPAGE_INDEX(Index)
//                          macro.   If the index is greater than or equal to
//                          the total count of children then it will treat the
//                          mode same as INSPSUIPAGE_MODE_LAST_CHILD
//
//
//                  dwData1
//                  dwData2
//                  dwData3 - 32-bit data associate with the 'Type' field
//                            as following
//
//
//                      PSUIPAGEINSERT_GROUP_PARENT
//
//                          dwData1 = Not used, must be 0
//                          dwData2 = Not used, must be 0
//                          dwData3 = Not used, must be 0
//
//
//                      PSUIPAGEINSERT_PCOMPROPSHEETUI
//
//                          dwData1 = pointer to COMPORPSHEETUI data structure.
//                          dwData2 = at return if sucessful, it contains total
//                                    page added.  If failed, it contains the
//                                    ERR_CPSUI_xxx codes
//                          dwData3 = Not used, must be 0
//
//
//                      PSUIPAGEINSERT_PFNPROPSHEETUI
//
//                          dwData1 = PFNPROPSHEETUI function pointer
//                          dwData2 = 32-bit parameter passed to pfnPropSheetUI
//                          dwData3 = Not used, must be 0
//
//
//                      PSUIPAGEINSERT_PROPSHEETPAGE
//
//                          dwData1 = Pointer to PROPSHEETPAGE data structure.
//                          dwData2 = not used, must be 0
//                          dwData3 = not used, must be 0
//
//
//                      PSUIPAGEINSERT_HPROPSHEETPAGE
//
//                          dwData1 = Is the HPROPSHEETPAGE handle that created
//                                    by a call to CreatePropertySheetPage().
//                          dwData2 = not used, must be 0
//                          dwData3 = not used, must be 0
//
//
//                      PSUIPAGEINSERT_DLL
//
//                          dwData1 = Pointer to a null terminated dynamic link
//                                    library filename.
//                          dwData2 = Pointer to a null terminated function
//                                    name (PFNPROPSHEETUI) in the dynamin link
//                                    library.
//                          dwData3 = 32-bit parameter passed to pfnPropSheetUI
//                                    (PFNPROPSHEETUI) function from dwData2
//
//
//  Return Value:
//
//      The return value is the handle of newly added common property sheet
//      page(s), if return value is NULL then function failed.
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_SET_PSUIPAGE_TITLE
//
//      This function let the caller set a new title for a particular common
//      UI page title (on the property sheet page tab)
//
//  Parameters:
//
//      lParam1 - the handle of common property sheet pages that title to be
//                set.  This handle must be the handle returned from following
//
//                  CPSFUNC_ADD_PROPSHEETPAGE
//                  CPSFUNC_ADD_HPROPSHEETPAGE
//                  CPSFUNC_INSERT_PSUIPAGE with type of
//                              PSUIPAGEINSERT_PROPSHEETPAGE or
//                              PSUIPAGEINSERT_HPROPSHEETPAGE
//
//      lParam2 - Pointer to a null terminated string for the new title
//
//
//  Return Value:
//
//      The return value is greater than zero if function sucessful, and less
//      or equal to zero if the function failed.
//
//      This function will returned 0 if the property sheet pages is not
//      currently displayed.
//
//
// -------------------------------------------------------------------------
// Function = CPSFUNC_SET_PSUIPAGE_ICON
//
//      This function let the caller set a new icon for a particular common
//      UI page icon (on the property sheet page tab)
//
//  Parameters:
//
//      lParam1 - the handle of common property sheet pages that icon to be
//                set.  This handle must be the handle returned from following
//
//                  CPSFUNC_ADD_PROPSHEETPAGE
//                  CPSFUNC_ADD_HPROPSHEETPAGE
//                  CPSFUNC_INSERT_PSUIPAGE with type of
//                              PSUIPAGEINSERT_PROPSHEETPAGE or
//                              PSUIPAGEINSERT_HPROPSHEETPAGE
//
//      lParam2 - Handle to Icon, this icon is best as 16x16 icon otherwise it
//                will be stretch to 16x16 (pixel).
//
//
//  Return Value:
//
//      The return value is greater than zero if function sucessful, and less
//      or equal to zero if the function failed.
//
//      This function will returned 0 if the property sheet pages is not
//      currently displayed.
//
//      After this function is successful set the icon, the caller can destroy
//      the hIcon using DestroyIcon() if the hIcon is created by CreateIcon().
//      If the hIcon (lParam2) passed is using LoadIcon() then it does not need
//      to destroy the icon.
//
// -------------------------------------------------------------------------
//
// Function = CPSFUNC_SET_DATABLOCK
//
//      This function let the caller register a new data block so it can be
//      later query by other pages in the property sheet, this function should
//      call durning PSN_KILLACTIVE message
//
//  Parameters:
//
//      lParam1 - pointer CPSUIDATABLOCK structure which speicifed the buffer
//                and size for queried datablock identify by lParam2 to be set.
//
//      lParam2 - DWORD Identifier of data block to be set
//
//
//  Return Value:
//
//      The return value is greater than zero if function sucessful which
//      indicate total bytes of data block registered.  If return value is less
//      or equal to zero then function failed and data block is not registered.
//
//      If lParam1 is NULL, (lParam1)->cbData or (lParam1)->pbbData is NULL
//      it return a -1 to indicate an error parameter passed.
//
// -------------------------------------------------------------------------
//
// Function = CPSFUNC_QUERY_DATABLOCK
//
//      This function let the caller query a registered data block so it can
//      used this data block to communicate between property sheet pages.
//      This function should call durning PSN_SETACTIVE message
//
//  Parameters:
//
//      lParam1 - pointer CPSUIDATABLOCK structure which speicifed the buffer
//                and size for queried datablock identify by lParam2
//
//      lParam2 - DWORD Identifier of data block to be queried
//
//
//  Return Value:
//
//      The return value is greater than zero if function sucessful which
//      indicate total bytes of data block copied into the pointer pointed by
//      the lParam1.  If return value is less or equal to zero then function
//      failed and data block is not copied to diciate the spcified datablock
//      Identifier (lParam2) is not found.
//
//      If CPSUIDATABLOCK pointer (lParam1) is NULL, (lParam1)->cbData or
//      (lParam1)->pbData is NULL then return value is the total bytes required
//      to copy the specified datablock identifier, if return value is less or
//      equal to zero then it indicate the spcified datablock identifier is
//      not found.
//
// -------------------------------------------------------------------------
//
// Function = CPSFUNC_SET_DMPUB_HIDEBITS
//
//      This function let the caller hide a set of predefined OPTITEMS in the
//      pages that was created using PSUIPAGEINSERT_PCOMPROPSHEETUI or
//      CPSFUNC_ADD_PCOMPROPSHEETUI with pDlgPage equal to
//      CPSUI_PDLGPAGE_DOCPROP or CPSUI_PDLGPAGE_ADVDOCPROP.
//
//      This function MUST called BEFORE the DOCPROP or ADVDOCPROP property
//      sheet pages are added using  PSUIPAGEINSERT_PCOMPROPSHEETUI or
//      CPSFUNC_ADD_PCOMPROPSHEETUI.
//
//  Parameters:
//
//      lParam1 - prdefined bit array masks to specified which DOCPROP item to
//                be hidden.  Each DMPUB item bit can be generate using macro
//                MAKE_DMPUB_HIDEBIT() and all items can be OR together.
//
//      lParam2 - Not used, MUST be 0
//
//
//  Return Value:
//
//      The return value is equal to lParam1 if this function is called before
//      DOCPROP or ADVDOCPROP pages are added. Return value is zero if these
//      pages already added which means failure.
//
// -------------------------------------------------------------------------
//
// Function = CPSFUNC_IGNORE_CPSUI_PSN_APPLY
//
//      This function let caller control how CPSUI process PSN_APPLY messages
//      for pages added through CPSFUNC_ADD_PCOMPROPSHEETUI or
//      CPSFUNC_INSERT_PSUIPAGE with type of PSUIPAGEINSERT_PROPSHEETPAGE.
//
//      If this function never called, the Default CPSUI behavior is to process
//      PSN_APPLY messages.
//
//  Parameters:
//
//      lParam1 - Handle to the page returned from pages added by using
//                CPSFUNC_ADD_PCOMPROPSHEETUI or CPSFUNC_INSERT_PSUIPAGE with
//                type of PSUIPAGEINSERT_PROPSHEETPAGE.
//
//      lParam2 - A non-zero value indicate to IGNORE the PSN_APPLY messages
//                for pages handle of lParam1.
//
//                A zero value to have CPSUI nomally process (not ignored) the
//                PSN_APPLY messages for pages handle of lParam1.
//
//                WARNING: if a PSN_APPLY messages are ignored, the caller must
//                         simulate a PSN_APPLY using CPSFUNC_DO_APPLY_CPSUI
//                         function or all changes WILL NOT applied, this is
//                         true even user hit 'OK' button.
//
//  Return Value:
//
//      the return value is zero if function failed, or non-zero to indicate
//      the function is successful.
//
// -------------------------------------------------------------------------
//
// Function = CPSFUNC_DO_APPLY_CPSUI
//
//      This function let caller simulate an apply (PSN_APPLY) to the pages
//      which added through CPSFUNC_ADD_PCOMPROPSHEETUI or
//      CPSFUNC_INSERT_PSUIPAGE with type of PSUIPAGEINSERT_PROPSHEETPAGE.
//
//  Parameters:
//
//      lParam1 - Handle to the page returned from pages added by using
//                CPSFUNC_ADD_PCOMPROPSHEETUI or CPSFUNC_INSERT_PSUIPAGE with
//                type of PSUIPAGEINSERT_PROPSHEETPAGE.
//
//      lParam2 - 32-bit flag to indicate how to do apply, currently only
//
//                  APPLYCPSUI_NO_NEWDEF
//
//                      The APPLYCPSUI_NO_NEWDEF bit specified that after the
//                      apply is done the undo buffer for all OPTITEMs still
//                      remain unchanged, this make next undo still possible.
//
//                      If APPLYCPSUI_NO_NEWDEF bit is clear then undo buffers
//                      are re-initialized after apply is done, the current
//                      changes for all OPTITEMs become the new undo default.
//
//
//                  APPLYCPSUI_OK_CANCEL_BUTTON
//
//                      If bit is set then it specified the apply is simulate
//                      a user hit 'Ok' or 'Cancel' button, if this bit is
//                      clear then it simulate a user hit 'Close' or 'Apply'
//                      button.  If caller getting a PSN_APPLY message, it can
//                      simulated using passed PSHNOTIFY (passed as lParam in
//                      WM_NOTIFY message) structure to determine how to set
//                      this bit as follow.
//
//                          if PSHNOTIFY.lParam == 0, then clear this bit
//                          otherwise set this bit.
//
//
//  Return Value:
//
//      the return value is less or equal to zero if it encounter an error and
//      apply failed (the active page will be switch to these pages).
//
//      If return value is greater than zero then changes were applied and
//      function sucessful.
//
// -------------------------------------------------------------------------
//
// Function = CPSFUNC_SET_FUSION_CONTEXT
//
//      This function is used to set a fusion activation context for
//      the specified page.
//
//  Parameters:
//
//      lParam1 - handle to fusion context. compstui duplicates the handle
//                prior attaching it to its internal structures, so the caller
//                is not bound to keep the handle around. we release the
//                passed in context handle when the compstui handle is deleted.
//
//      lParam2 - not used
//
//  Return Value:
//
//      the return value is less or equal to zero if it encounter an error and/or
//      something failed (look up the last error for details).
//
//      If return value is greater than zero then the call was successful.
//
// -------------------------------------------------------------------------
//

#define PROPSHEETUI_REASON_INIT             0
#define PROPSHEETUI_REASON_GET_INFO_HEADER  1
#define PROPSHEETUI_REASON_DESTROY          2
#define PROPSHEETUI_REASON_SET_RESULT       3
#define PROPSHEETUI_REASON_GET_ICON         4
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define PROPSHEETUI_REASON_BEFORE_INIT      5
#define MAX_PROPSHEETUI_REASON_INDEX        5
#else
#define MAX_PROPSHEETUI_REASON_INDEX        4
#endif

#define PROPSHEETUI_INFO_VERSION            0x0100

#define PSUIINFO_UNICODE                    0x0001

typedef struct _PROPSHEETUI_INFO {
    WORD            cbSize;
    WORD            Version;
    WORD            Flags;
    WORD            Reason;
    HANDLE          hComPropSheet;
    PFNCOMPROPSHEET pfnComPropSheet;
    LPARAM          lParamInit;
    ULONG_PTR       UserData;
    ULONG_PTR       Result;
    } PROPSHEETUI_INFO, *PPROPSHEETUI_INFO;


//
// For the PROPSHEETUI_REASON_GET_ICON call which lParam is a pointer to
// PROPSHEETUI_GETICON_INFO
//

typedef struct _PROPSHEETUI_GETICON_INFO {
    WORD    cbSize;
    WORD    Flags;
    WORD    cxIcon;
    WORD    cyIcon;
    HICON   hIcon;
    } PROPSHEETUI_GETICON_INFO, *PPROPSHEETUI_GETICON_INFO;


typedef LONG (FAR WINAPI *PFNPROPSHEETUI)(PPROPSHEETUI_INFO   pPSUIInfo,
                                   LPARAM              lParam);


//
// PFNPROPSHEETUI
//
//  This function is user defined function which will be called by the common
//  UI when a caller wish to include the executable property sheets.
//
//  pPSUIInfo   - Pointer to PROPSHEETUI_INFO below for description of
//                PROPSHEETUI_INFO.
//
//                If this pointer is NULL then this function is not called from
//                common UI, and lParam should be used to determined the
//                action and outcome of this function.
//
//  lParam      - A LPARAM intended for this function depends on the reason.
//                If pPSUIInfo is NULL then this function is not called from
//                common UI, the lParam is the parameter which agreed with
//                the caller.
//
//                if pPSUIInfo is not NULL then this function assume the call
//                is from the common UI.  lParam has following meaning depends
//                on the reason field.
//
//                PROPSHEETUI_REASON_BEFORE_INIT
//
//                  This value is new to Window 8 and provided only to the original
//                  PFNPROPSHEETUI parameter passed to CommonPropertySheetUI.
//
//                  This is used between common UI and the system provided config
//                  module for v4 printer drivers and should be ignored by v3 drivers.
//
//                PROPSHEETUI_REASON_INIT
//
//                  The lParam is either passed from CPSFUNC_ADD_PFNPROPSHEETUI
//                  callback function's second parameter (lParam2) or it is
//                  from CommonPropertySheetUI()'s lParam (seccond parameter)
//                  without any modification.
//
//                  The lParam MUST NOT be a variable or a pointer to memory
//                  block which resides on the caller function's stack, since
//                  after this function exit, the lParam will become invalid
//                  and can cause fatal system error.
//
//                  The lParam parameter is copied to the lParamInit field in
//                  PROPSHEETUI_INFO data structure.  The lParamInit field
//                  will be passed to all subsequent pfnPropSheetUI() calls
//                  without any modification.
//
//
//                PROPSHEETUI_REASON_GET_INFO_HEADER:
//
//                  lParam is a pointer to the PROPSHEETUI_INFO_HEADER data
//                  structure. this function must correctly fill this structure
//                  fields before it returned.
//
//
//                PROPSHEETUI_REASON_DESTROY
//
//                  lParam is zero to indicate the destroy is cause either by a
//                  caller calling CPSFUNC_DELETE_HCOMPROPSHEET or failed
//                  in caller's CPSFUNC_ADD_xxxx.
//
//                  lParam is non zero to indicate the destroy is cause by
//                  exiting from the property sheet user interface.
//
//
//                PROPSHEETUI_REASON_SET_RESULT
//
//                  lParam is a pointer to SETRESULT_INFO data structure.
//                  Fields in SETRESULT_INFO data structure is set to following
//
//                      hSetResult: Handle to the common UI property sheet
//                                  pages which added by the CPSFUNC_ADD_xxx
//                                  callback function indices.
//
//                          Result: The result from the hSetResult to be set.
//                                  When return greater than zero then Result
//                                  in this field is set to its parent.
//
//
//                PROPSHEETUI_REASON_GET_ICON
//
//                  lParam is a pointer to PROPSHEETUI_GETICON_INFO data
//                  structure.  Fields in PROPSHEETUI_GETICON_INFO data
//                  structure are set to following
//
//                      cxIcon = cx Icon size in pixel
//                      cyIcon = cy Icon size in pixel
//                      hIcon  = Initial to NULL, and this function must put
//                               the created icon handle in this field before
//                               returned.
//


//
// PROPSHEETUI_INFO
//
//  This structure is used when common UI calling the caller passed function
//  entry point PFNPROPSHEETUI, the pfnPropSheetUI() return a LONG to
//  indicate result of this function.
//
//  When the pfnPropSheetUI() returned, it must also put the required result
//  in the Result field, each PROPSHEETUI_REASON_xxx has different required
//  result as describe below.
//
//
//
//  cbSize          - sizeof this structure (PROPSHEETUI_INFO)
//
//  Version         - the PROPSHEETUI_INFO data structure version. Current
//                    version is set to PROPSHEETUI_INFO_VERSION
//
//  Flags           - One or more following is may be defined
//
//                      PSUIINFO_UNICODE
//
//                          The caller's executable was original compiled was
//                          indented using unicode.
//
//
//  Reason          - Following reasons are defined.
//
//                      PROPSHEETUI_REASON_INIT
//
//                          When first time the pfnPropSheetUI() called, this
//                          reason is used to have the function initialize
//                          itself and use the pfnComPropSheet() function
//                          pointer provided to add new pages to the
//                          hComPropSheet handle passed.   The UserData in this
//                          data structure is initially set equal to zero (0).
//
//                          * Return > 0 to indicate sucesful and <= 0 to
//                            indicate error.
//
//                      *NOTE*
//
//                          The lParam MUST NOT be a variable or a pointer to
//                          memory block which resides on the caller function's
//                          stack, since after this function exit, the lParam
//                          will become invalid and can cause fatal system
//                          error.
//
//                          If this function reason returned failed ( <= 0),
//                          this function (pfnPropSheetUI) will received a
//                          PROPSHEETUI_REASON_DESTROY function reason right
//                          after this function reason returned.
//
//
//                      PROPSHEETUI_REASON_GET_INFO_HEADER
//
//                          It is called after the PROPSHEETUI_REASON_INIT is
//                          successful returned.  This reason is used to asked
//                          the function fill in the PROPSHEETUI_INFO_HEADER
//                          for pop-up the property sheet dialog boxes.
//
//                          lParam in this reason is a pointer to the
//                          PROPSHEETUI_INFO_HEADER data structure, following
//                          fields are requrested to be filled in.
//
//                              Flags       - PSUIHDRF_xxx flags
//                              pTitle      - The property sheet title
//                              hWndParent  - handle to the parent of property
//                                            sheet pages.
//                              hInst       - Instance data handle for this
//                                            function.
//                              hIcon
//                              IconID      - Icon used on the title bar.
//
//                          * Return > 0 to indicate sucesful and pop-up the
//                            property sheet dialog boxes or returned <= 0 to
//                            indicate error (not property sheet UI appeared)
//
//
//                      PROPSHEETUI_REASON_DESTROY
//
//                          When the property sheet ready to dismissed or the
//                          caller is delete the common UI pages added by the
//                          pfnPropSheetUI(), the common UI will call this
//                          entry point to have it de-initialized itself and
//                          free up all the memory used for this function
//                          instance.  The UserData field passed is the
//                          'UserData' field which returned from previous
//                          PROPSHEETUI_REASON_xxxx.
//
//                          When this function called, all hComPropSheet's
//                          children are desotroyed and children's common UI
//                          handles are not longer valid.
//
//                          * Return > 0 to indicate sucesful and <= 0 to
//                            indicate error.
//
//
//                      PROPSHEETUI_REASON_SET_RESULT
//
//                          The reason is used when an added handle from
//                          CPSFUNC_ADD_xxxx whant to return the result to
//                          this pfnPropSheetUI() caller.
//
//                          The lParam in second parameter is a pointer to a
//                          SETRESULT_INFO data structure.
//
//                              hSetResult: specified the common UI property
//                                          sheet page handle which added by
//                                          this function using
//                                          CPSFUNC_ADD_xxx indicies.
//
//                                  Result: Specified the Result to be set to
//                                          this pfnPropSheetUI() form the
//                                          hSetResult property sheet page
//                                          handle. If return value is greater
//                                          than zero then the value in this
//                                          Result field will be set to its
//                                          parent if one exist, at this case
//                                          this function can alter the Result
//                                          field value for its parent.
//
//                          * Returned greater than zero to continue send to
//                            its parent, else it stop sending the Result filed
//                            value to its parent.
//
//
//                      PROPSHEETUI_REASON_GET_ICON
//
//                          The reason is used to retrived the Icon which
//                          represent this pfnPropSheetUI().
//
//                          lParam is a pointer to PROPSHEETUI_GETICON_INFO '
//                          data structure.  Fields in PROPSHEETUI_GETICON_INFO
//                          data structure are set to following
//
//                              cxIcon = cx Icon size in pixel
//                              cyIcon = cy Icon size in pixel
//                              hIcon  = Initial to NULL, and this function
//                                       must put the created icon handle in
//                                       this field before return.
//
//
//                          * Return > 0 to indicate sucesful (hIcon is the
//                            requested icon handle).   Return = 0 to indicate
//                            no icon available, or return < 0 to indicate an
//                            error.
//
//              *NOTE*
//
//                  * For all PROPSHEETUI_REASON_xxx, the function can set new
//                    user defined DWORD data in the PROPSHEETUI_INFO data
//                    structure's UserData field.
//
//                  * For all PROPSHEETUI_REASON_xxx, the function can set new
//                    pfnPropSheetUI() DWORD result in PROPSHEETUI_INFO data
//                    structure's Result field.
//
//
//  hComPropSheet   - Handle to the COMPROPSHEETPAGE which this function should
//                    used as hComPropSheet parameter when calling
//                    pfnComPropSheet() to add or delete common UI property
//                    sheet pages.  The hComPropSheet is the instance handle to
//                    pfnPropSheetUI() function.
//
//  pfnComPropSheet - Pointer to the common UI callback function which for the
//                    pfnPropSheetUI() to add, delete, set user data, for a
//                    completed set of callback, see CPSFUNC_xxx descriptions
//                    above.
//
//  lParamInit      - The lParam originally passed duning the the first call
//                    reason PROPSHEETUI_REASON_INIT.  The lParamInit will be
//                    passed to each PROPSHEETUI_REASON_xxx calls.
//
//  UserData        - the UserData field is an IN and OUT parameter for each
//                    of the Reason,
//
//                      PROPSHEETUI_REASON_INIT
//
//                           IN: Initial set to zero (0).
//
//                          OUT: Specified new callee's own user data which
//                               will be passed back to other reason calls.
//
//
//                      PROPSHEETUI_REASON_DESTROY
//                      PROPSHEETUI_REASON_SET_RESULT
//                      PROPSHEETUI_REASON_GET_INFO_HEADER
//
//                           IN: The UserData specified at time when returned
//                               from the previous PROPSHEETUI_REASON_xxx
//
//                          OUT: Specified new callee's own user data which
//                               will be passed back to other reason calls.
//
//
// Result           - The Result field is an IN and OUT parameter for each of
//                    the reason.
//
//                      PROPSHEETUI_REASON_INIT
//
//                           IN: Set to zero (0).
//
//                          OUT: Set to default result of this PropSheetUI()
//                               function.
//
//
//                      PROPSHEETUI_REASON_DESTROY
//                      PROPSHEETUI_REASON_GET_INFO_HEADER
//                      PROPSHEETUI_REASON_SET_RESULT
//
//                           IN: The current 'Result' returned from previous
//                               PROPSHEETUI_REASON_xxx function.
//
//                          OUT: Set the new result of this PropSheetUI()
//                               function.
//
//


#define PSUIHDRF_OBSOLETE       0x0001
#define PSUIHDRF_NOAPPLYNOW     0x0002
#define PSUIHDRF_PROPTITLE      0x0004
#define PSUIHDRF_USEHICON       0x0008
#define PSUIHDRF_DEFTITLE       0x0010
#define PSUIHDRF_EXACT_PTITLE   0x0020

typedef struct _PROPSHEETUI_INFO_HEADER {
    WORD                    cbSize;
    WORD                    Flags;
    LPTSTR                  pTitle;
    HWND                    hWndParent;
    HINSTANCE               hInst;
    union {
        HICON               hIcon;
        ULONG_PTR           IconID;
        } DUMMYUNIONNAME;
    } PROPSHEETUI_INFO_HEADER, *PPROPSHEETUI_INFO_HEADER;

//
// PROPSHEETUI_INFO_HEADER
//
//  This data structure is used when common UI ready to pop-up the property
//  sheet page dialog boxes and it asked caller to provide more information.
//
//  Common property sheet UI passed this data structure as lParam when it call
//  provided pfnPropSheetUI() with PROPSHEETUI_REASON_GET_INFO_HEADER reason.
//
//
//  cbSize      - size of this structure
//
//  Flags       - PSUIHDRF_xxxx flags
//
//                  PSUIHDRF_OBSOLETE
//
//                      Not used, must not set this bit
//
//
//                  PSUIHDRF_NOAPPLYNOW
//
//                      Remove 'Apply Now' button.
//
//
//                  PSUIHDRF_PROPTITLE
//
//                      Automatically include 'Properties' in the title bar
//
//
//                  PSUIHDRF_USEHICON
//
//                      If this bit is specified then hIcon union field is
//                      a valid handle to the icon otherwise the IconID is
//                      the either caller's resource ID or common UI standard
//                      icon ID.
//
//                  PSUIHDRF_DEFTITLE
//
//                      Automatically include 'Default' in the title bar, the
//                      'Default' always added right after pTitle and before
//                      'Properties' if PSUIHDRF_PROPTITLE flag is set.
//
//                  PSUIHDRF_EXACT_PTITLE
//
//                      This flag indicate the pTitle set in this structure
//                      will be use without any modification by the compstui
//                      ie. compstui will not modified pTitle in any way and
//                      it will ignored PSUIHDRF_PROPTITLE, PSUIHDRF_DEFTITLE
//
//
//  pTitle      - Pointer to the NULL terminated caption name for the
//                property sheets.
//
//                  ** See LPTSTR typedef description above
//
//  hWndParent  - The handle of the window which will be parent of the common
//                UI property sheets, if NULL then current active window for
//                the calling thread is used.
//
//  hInst       - the caller's handle to its instance.  Commom UI use this
//                handle to load caller's icon and other resources.
//
//  hIcon
//  IconID      - Specified the icon which put on the title bar, it either a
//                handle to the icon or a icon resource ID depends on the
//                PSUIHDRF_USEHICON flag.
//



//
// LONG
// CommonPropertySheetUI(
//     HWND            hWndOwner,
//     PFNPROPSHEETUI  pfnPropSheetUI,
//     LPARAM          lParam,
//     LPDWORD         pResult
//     );
//
//
// The CommonPropSheetUI is the main entry point for the common property sheet
// user interface.   The original caller that wish to using common UI to pop-up
// property sheet will call this function and passed its own PFNPROPSHEETUI
// function address and a long parameter.
//
// If pfnPropSheetUI function return a LONG number greater than zero (0) then
// common UI will pop-up the property sheet page dialog boxes, when Property
// sheet pages is finished. (either hit Ok or Cancel) it will return the
// result of CPSUI_xxxx back to the caller.
//
// If pfnPropSheetUI function return a LONG number equal or less than zero (0)
// then it will return the CPSUI_CANCEL back to caller without pop-up the
// property sheet page dialog boxes.
//
//
//  Parameters:
//
//      hWndOwner       - Window handle for the owner of this proerty sheet
//                        pages dialog boxes.
//
//      pfnPropSheetUI  - a PFNPROPSHEETUI function pointer which is used by
//                        the caller to add its property sheet pages.
//
//      lParam          - a long parameter will be passed to the pfnPropSheetUI
//                        funciton.  The common UI called the pfnPropSheetUI as
//
//                          PROPSHEETUI_INFO    PSUIInfo;
//
//                          pfnPropSheetUI(&PSUIInfo, lParam);
//
//                        The caller must use pfnComPropSheet() to add/delete
//                        pages.  When it is done adding pages, it retuned
//                        greater than zero to indicate successful, and return
//                        less or equal to zero to indicate failure.
//
//      pResult         - a pointer to DWORD which received the final result
//                        of pfnPropSheetUI() funciton, this result is a copy
//                        from Result field of PROPSHEETUI_INFO data structure
//                        which passed to the pfnPropSheetUI() as the first
//                        parameter.  The pResult only will be set if the
//                        returned value from CommonPropertySheetUI() is not
//                        ERR_CPSUI_xxx.
//
//                        if pResult is NULL then common UI will not return
//                        pfnPropSheetUI()'s result back.
//
//
//  Return Value:
//
//      LONG    - < 0                   - property page does not displayed and
//                                        ERR_CPSUI_xxx is the error code
//                CPSUI_OK              - property page displayed.
//                CPSUI_RESTARTWINDOWS  - property page displayed and need to
//                                        restart window to take effect
//                CPSUI_REBOOTSYSTEM    - property page dispalyed and need
//                                        to reboot system to take effect
//
//


#if (NTDDI_VERSION >= NTDDI_VISTA)

LONG
APIENTRY
CommonPropertySheetUIA(
    HWND hWndOwner,
    __callback PFNPROPSHEETUI pfnPropSheetUI,
    LPARAM lParam,
    _Out_opt_ LPDWORD pResult
    );

LONG
APIENTRY
CommonPropertySheetUIW(
    HWND hWndOwner,
    __callback PFNPROPSHEETUI pfnPropSheetUI,
    LPARAM lParam,
    _Out_opt_ LPDWORD pResult
    );

#else

LONG
APIENTRY
CommonPropertySheetUIA(
    HWND hWndOwner,
    PFNPROPSHEETUI pfnPropSheetUI,
    LPARAM lParam,
    LPDWORD pResult
    );

LONG
APIENTRY
CommonPropertySheetUIW(
    HWND hWndOwner,
    PFNPROPSHEETUI pfnPropSheetUI,
    LPARAM lParam,
    LPDWORD pResult
    );

#endif


#ifdef UNICODE
#define CommonPropertySheetUI   CommonPropertySheetUIW
#else
#define CommonPropertySheetUI   CommonPropertySheetUIA
#endif



//
// GetCPSUIUserData() and SetCPSUIUserData() is used for the pages added
// by the CPSFUNC_ADD_PCOMPROPSHEETUI.  The caller add this function and has
// sub class dialog procedure should not set DWLP_USERDATA but calling these
// function instead, otherwise the system can failed.
//

ULONG_PTR
APIENTRY
GetCPSUIUserData(
    HWND    hDlg
    );

BOOL
APIENTRY
SetCPSUIUserData(
    HWND        hDlg,
    ULONG_PTR   CPSUIUserData
    );


#define CPSUI_CANCEL                        0
#define CPSUI_OK                            1
#define CPSUI_RESTARTWINDOWS                2
#define CPSUI_REBOOTSYSTEM                  3

#define ERR_CPSUI_GETLASTERROR              -1
#define ERR_CPSUI_ALLOCMEM_FAILED           -2
#define ERR_CPSUI_INVALID_PDATA             -3
#define ERR_CPSUI_INVALID_LPARAM            -4
#define ERR_CPSUI_NULL_HINST                -5
#define ERR_CPSUI_NULL_CALLERNAME           -6
#define ERR_CPSUI_NULL_OPTITEMNAME          -7
#define ERR_CPSUI_NO_PROPSHEETPAGE          -8
#define ERR_CPSUI_TOO_MANY_PROPSHEETPAGES   -9
#define ERR_CPSUI_CREATEPROPPAGE_FAILED     -10
#define ERR_CPSUI_MORE_THAN_ONE_TVPAGE      -11
#define ERR_CPSUI_MORE_THAN_ONE_STDPAGE     -12
#define ERR_CPSUI_INVALID_PDLGPAGE          -13
#define ERR_CPSUI_INVALID_DLGPAGE_CBSIZE    -14
#define ERR_CPSUI_TOO_MANY_DLGPAGES         -15
#define ERR_CPSUI_INVALID_DLGPAGEIDX        -16
#define ERR_CPSUI_SUBITEM_DIFF_DLGPAGEIDX   -17
#define ERR_CPSUI_NULL_POPTITEM             -18
#define ERR_CPSUI_INVALID_OPTITEM_CBSIZE    -19
#define ERR_CPSUI_INVALID_OPTTYPE_CBSIZE    -20
#define ERR_CPSUI_INVALID_OPTTYPE_COUNT     -21
#define ERR_CPSUI_NULL_POPTPARAM            -22
#define ERR_CPSUI_INVALID_OPTPARAM_CBSIZE   -23
#define ERR_CPSUI_INVALID_EDITBOX_PSEL      -24
#define ERR_CPSUI_INVALID_EDITBOX_BUF_SIZE  -25
#define ERR_CPSUI_INVALID_ECB_CBSIZE        -26
#define ERR_CPSUI_NULL_ECB_PTITLE           -27
#define ERR_CPSUI_NULL_ECB_PCHECKEDNAME     -28
#define ERR_CPSUI_INVALID_DMPUBID           -29
#define ERR_CPSUI_INVALID_DMPUB_TVOT        -30
#define ERR_CPSUI_CREATE_TRACKBAR_FAILED    -31
#define ERR_CPSUI_CREATE_UDARROW_FAILED     -32
#define ERR_CPSUI_CREATE_IMAGELIST_FAILED   -33
#define ERR_CPSUI_INVALID_TVOT_TYPE         -34
#define ERR_CPSUI_INVALID_LBCB_TYPE         -35
#define ERR_CPSUI_SUBITEM_DIFF_OPTIF_HIDE   -36
#define ERR_CPSUI_INVALID_PUSHBUTTON_TYPE   -38
#define ERR_CPSUI_INVALID_EXTPUSH_CBSIZE    -39
#define ERR_CPSUI_NULL_EXTPUSH_DLGPROC      -40
#define ERR_CPSUI_NO_EXTPUSH_DLGTEMPLATEID  -41
#define ERR_CPSUI_NULL_EXTPUSH_CALLBACK     -42
#define ERR_CPSUI_DMCOPIES_USE_EXTPUSH      -43
#define ERR_CPSUI_ZERO_OPTITEM              -44


#define ERR_CPSUI_FUNCTION_NOT_IMPLEMENTED  -9999
#define ERR_CPSUI_INTERNAL_ERROR            -10000

#endif  // (!defined(RC_INVOKED))



//
//****************************************************************************
//*                                                                          *
//*      Common Property Sheet UI resource ID for the ICONs and STRINGs      *
//*                                                                          *
//* The Resource ID from 64000 to 65535 are reserved for common UI and must  *
//* not used as caller resource ID else the string, icon loading will not be *
//* correct.                                                                 *
//*                                                                          *
//****************************************************************************
//


//
// Common UI standard 32x32, 16x16 color and monochrome Icon IDs
//

#define IDI_CPSUI_ICONID_FIRST          64000

#define IDI_CPSUI_EMPTY                 64000
#define IDI_CPSUI_SEL_NONE              64001
#define IDI_CPSUI_WARNING               64002
#define IDI_CPSUI_NO                    64003
#define IDI_CPSUI_YES                   64004
#define IDI_CPSUI_FALSE                 64005
#define IDI_CPSUI_TRUE                  64006
#define IDI_CPSUI_OFF                   64007
#define IDI_CPSUI_ON                    64008
#define IDI_CPSUI_PAPER_OUTPUT          64009
#define IDI_CPSUI_ENVELOPE              64010
#define IDI_CPSUI_MEM                   64011
#define IDI_CPSUI_FONTCARTHDR           64012
#define IDI_CPSUI_FONTCART              64013
#define IDI_CPSUI_STAPLER_ON            64014
#define IDI_CPSUI_STAPLER_OFF           64015
#define IDI_CPSUI_HT_HOST               64016
#define IDI_CPSUI_HT_DEVICE             64017
#define IDI_CPSUI_TT_PRINTASGRAPHIC     64018
#define IDI_CPSUI_TT_DOWNLOADSOFT       64019
#define IDI_CPSUI_TT_DOWNLOADVECT       64020
#define IDI_CPSUI_TT_SUBDEV             64021
#define IDI_CPSUI_PORTRAIT              64022
#define IDI_CPSUI_LANDSCAPE             64023
#define IDI_CPSUI_ROT_LAND              64024
#define IDI_CPSUI_AUTOSEL               64025
#define IDI_CPSUI_PAPER_TRAY            64026
#define IDI_CPSUI_PAPER_TRAY2           64027
#define IDI_CPSUI_PAPER_TRAY3           64028
#define IDI_CPSUI_TRANSPARENT           64029
#define IDI_CPSUI_COLLATE               64030
#define IDI_CPSUI_DUPLEX_NONE           64031
#define IDI_CPSUI_DUPLEX_HORZ           64032
#define IDI_CPSUI_DUPLEX_VERT           64033
#define IDI_CPSUI_RES_DRAFT             64034
#define IDI_CPSUI_RES_LOW               64035
#define IDI_CPSUI_RES_MEDIUM            64036
#define IDI_CPSUI_RES_HIGH              64037
#define IDI_CPSUI_RES_PRESENTATION      64038
#define IDI_CPSUI_MONO                  64039
#define IDI_CPSUI_COLOR                 64040
#define IDI_CPSUI_DITHER_NONE           64041
#define IDI_CPSUI_DITHER_COARSE         64042
#define IDI_CPSUI_DITHER_FINE           64043
#define IDI_CPSUI_DITHER_LINEART        64044
#define IDI_CPSUI_SCALING               64045
#define IDI_CPSUI_COPY                  64046
#define IDI_CPSUI_HTCLRADJ              64047
#define IDI_CPSUI_HALFTONE_SETUP        64048
#define IDI_CPSUI_WATERMARK             64049
#define IDI_CPSUI_ERROR                 64050
#define IDI_CPSUI_ICM_OPTION            64051
#define IDI_CPSUI_ICM_METHOD            64052
#define IDI_CPSUI_ICM_INTENT            64053
#define IDI_CPSUI_STD_FORM              64054
#define IDI_CPSUI_OUTBIN                64055
#define IDI_CPSUI_OUTPUT                64056
#define IDI_CPSUI_GRAPHIC               64057
#define IDI_CPSUI_ADVANCE               64058
#define IDI_CPSUI_DOCUMENT              64059
#define IDI_CPSUI_DEVICE                64060
#define IDI_CPSUI_DEVICE2               64061
#define IDI_CPSUI_PRINTER               64062
#define IDI_CPSUI_PRINTER2              64063
#define IDI_CPSUI_PRINTER3              64064
#define IDI_CPSUI_PRINTER4              64065
#define IDI_CPSUI_OPTION                64066
#define IDI_CPSUI_OPTION2               64067
#define IDI_CPSUI_STOP                  64068
#define IDI_CPSUI_NOTINSTALLED          64069
#define IDI_CPSUI_WARNING_OVERLAY       64070
#define IDI_CPSUI_STOP_WARNING_OVERLAY  64071
#define IDI_CPSUI_GENERIC_OPTION        64072
#define IDI_CPSUI_GENERIC_ITEM          64073
#define IDI_CPSUI_RUN_DIALOG            64074
#define IDI_CPSUI_QUESTION              64075
#define IDI_CPSUI_FORMTRAYASSIGN        64076
#define IDI_CPSUI_PRINTER_FOLDER        64077
#define IDI_CPSUI_INSTALLABLE_OPTION    64078
#define IDI_CPSUI_PRINTER_FEATURE       64079
#define IDI_CPSUI_DEVICE_FEATURE        64080
#define IDI_CPSUI_FONTSUB               64081
#define IDI_CPSUI_POSTSCRIPT            64082
#define IDI_CPSUI_TELEPHONE             64083
#define IDI_CPSUI_DUPLEX_NONE_L         64084
#define IDI_CPSUI_DUPLEX_HORZ_L         64085
#define IDI_CPSUI_DUPLEX_VERT_L         64086
#define IDI_CPSUI_LF_PEN_PLOTTER        64087
#define IDI_CPSUI_SF_PEN_PLOTTER        64088
#define IDI_CPSUI_LF_RASTER_PLOTTER     64089
#define IDI_CPSUI_SF_RASTER_PLOTTER     64090
#define IDI_CPSUI_ROLL_PAPER            64091
#define IDI_CPSUI_PEN_CARROUSEL         64092
#define IDI_CPSUI_PLOTTER_PEN           64093
#define IDI_CPSUI_MANUAL_FEED           64094
#define IDI_CPSUI_FAX                   64095
#define IDI_CPSUI_PAGE_PROTECT          64096
#define IDI_CPSUI_ENVELOPE_FEED         64097
#define IDI_CPSUI_FONTCART_SLOT         64098
#define IDI_CPSUI_LAYOUT_BMP_PORTRAIT   64099
#define IDI_CPSUI_LAYOUT_BMP_ARROWL     64100
#define IDI_CPSUI_LAYOUT_BMP_ARROWS     64101
#define IDI_CPSUI_LAYOUT_BMP_BOOKLETL   64102
#define IDI_CPSUI_LAYOUT_BMP_BOOKLETP   64103

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define IDI_CPSUI_LAYOUT_BMP_ARROWLR    64104
#define IDI_CPSUI_LAYOUT_BMP_ROT_PORT   64105
#define IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB 64106
#define IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB 64107
#define IDI_CPSUI_ROT_PORT              64110
#define IDI_CPSUI_NUP_BORDER            64111
#define IDI_CPSUI_ICONID_LAST           64111
#else
#define IDI_CPSUI_ICONID_LAST           64103
#endif

//
// Common UI standard String IDs
//


#define IDS_CPSUI_STRID_FIRST           64700

#define IDS_CPSUI_SETUP                 64700
#define IDS_CPSUI_MORE                  64701
#define IDS_CPSUI_CHANGE                64702
#define IDS_CPSUI_OPTION                64703
#define IDS_CPSUI_OF                    64704
#define IDS_CPSUI_RANGE_FROM            64705
#define IDS_CPSUI_TO                    64706
#define IDS_CPSUI_COLON_SEP             64707
#define IDS_CPSUI_LEFT_ANGLE            64708
#define IDS_CPSUI_RIGHT_ANGLE           64709
#define IDS_CPSUI_SLASH_SEP             64710
#define IDS_CPSUI_PERCENT               64711
#define IDS_CPSUI_LBCB_NOSEL            64712
#define IDS_CPSUI_PROPERTIES            64713
#define IDS_CPSUI_DEFAULTDOCUMENT       64714
#define IDS_CPSUI_DOCUMENT              64715
#define IDS_CPSUI_ADVANCEDOCUMENT       64716
#define IDS_CPSUI_PRINTER               64717
#define IDS_CPSUI_AUTOSELECT            64718
#define IDS_CPSUI_PAPER_OUTPUT          64719
#define IDS_CPSUI_GRAPHIC               64720
#define IDS_CPSUI_OPTIONS               64721
#define IDS_CPSUI_ADVANCED              64722
#define IDS_CPSUI_STDDOCPROPTAB         64723
#define IDS_CPSUI_STDDOCPROPTVTAB       64724
#define IDS_CPSUI_DEVICEOPTIONS         64725
#define IDS_CPSUI_FALSE                 64726
#define IDS_CPSUI_TRUE                  64727
#define IDS_CPSUI_NO                    64728
#define IDS_CPSUI_YES                   64729
#define IDS_CPSUI_OFF                   64730
#define IDS_CPSUI_ON                    64731
#define IDS_CPSUI_DEFAULT               64732
#define IDS_CPSUI_ERROR                 64733
#define IDS_CPSUI_NONE                  64734
#define IDS_CPSUI_NOT                   64735
#define IDS_CPSUI_EXIST                 64736
#define IDS_CPSUI_NOTINSTALLED          64737
#define IDS_CPSUI_ORIENTATION           64738
#define IDS_CPSUI_SCALING               64739
#define IDS_CPSUI_NUM_OF_COPIES         64740
#define IDS_CPSUI_SOURCE                64741
#define IDS_CPSUI_PRINTQUALITY          64742
#define IDS_CPSUI_RESOLUTION            64743
#define IDS_CPSUI_COLOR_APPERANCE       64744
#define IDS_CPSUI_DUPLEX                64745
#define IDS_CPSUI_TTOPTION              64746
#define IDS_CPSUI_FORMNAME              64747
#define IDS_CPSUI_ICM                   64748
#define IDS_CPSUI_ICMMETHOD             64749
#define IDS_CPSUI_ICMINTENT             64750
#define IDS_CPSUI_MEDIA                 64751
#define IDS_CPSUI_DITHERING             64752
#define IDS_CPSUI_PORTRAIT              64753
#define IDS_CPSUI_LANDSCAPE             64754
#define IDS_CPSUI_ROT_LAND              64755
#define IDS_CPSUI_COLLATE               64756
#define IDS_CPSUI_COLLATED              64757
#define IDS_CPSUI_PRINTFLDSETTING       64758
#define IDS_CPSUI_DRAFT                 64759
#define IDS_CPSUI_LOW                   64760
#define IDS_CPSUI_MEDIUM                64761
#define IDS_CPSUI_HIGH                  64762
#define IDS_CPSUI_PRESENTATION          64763
#define IDS_CPSUI_COLOR                 64764
#define IDS_CPSUI_GRAYSCALE             64765
#define IDS_CPSUI_MONOCHROME            64766
#define IDS_CPSUI_SIMPLEX               64767
#define IDS_CPSUI_HORIZONTAL            64768
#define IDS_CPSUI_VERTICAL              64769
#define IDS_CPSUI_LONG_SIDE             64770
#define IDS_CPSUI_SHORT_SIDE            64771
#define IDS_CPSUI_TT_PRINTASGRAPHIC     64772
#define IDS_CPSUI_TT_DOWNLOADSOFT       64773
#define IDS_CPSUI_TT_DOWNLOADVECT       64774
#define IDS_CPSUI_TT_SUBDEV             64775
#define IDS_CPSUI_ICM_BLACKWHITE        64776
#define IDS_CPSUI_ICM_NO                64777
#define IDS_CPSUI_ICM_YES               64778
#define IDS_CPSUI_ICM_SATURATION        64779
#define IDS_CPSUI_ICM_CONTRAST          64780
#define IDS_CPSUI_ICM_COLORMETRIC       64781
#define IDS_CPSUI_STANDARD              64782
#define IDS_CPSUI_GLOSSY                64783
#define IDS_CPSUI_TRANSPARENCY          64784
#define IDS_CPSUI_REGULAR               64785
#define IDS_CPSUI_BOND                  64786
#define IDS_CPSUI_COARSE                64787
#define IDS_CPSUI_FINE                  64788
#define IDS_CPSUI_LINEART               64789
#define IDS_CPSUI_ERRDIFFUSE            64790
#define IDS_CPSUI_HALFTONE              64791
#define IDS_CPSUI_HTCLRADJ              64792
#define IDS_CPSUI_USE_HOST_HT           64793
#define IDS_CPSUI_USE_DEVICE_HT         64794
#define IDS_CPSUI_USE_PRINTER_HT        64795
#define IDS_CPSUI_OUTBINASSIGN          64796
#define IDS_CPSUI_WATERMARK             64797
#define IDS_CPSUI_FORMTRAYASSIGN        64798
#define IDS_CPSUI_UPPER_TRAY            64799
#define IDS_CPSUI_ONLYONE               64800
#define IDS_CPSUI_LOWER_TRAY            64801
#define IDS_CPSUI_MIDDLE_TRAY           64802
#define IDS_CPSUI_MANUAL_TRAY           64803
#define IDS_CPSUI_ENVELOPE_TRAY         64804
#define IDS_CPSUI_ENVMANUAL_TRAY        64805
#define IDS_CPSUI_TRACTOR_TRAY          64806
#define IDS_CPSUI_SMALLFMT_TRAY         64807
#define IDS_CPSUI_LARGEFMT_TRAY         64808
#define IDS_CPSUI_LARGECAP_TRAY         64809
#define IDS_CPSUI_CASSETTE_TRAY         64810
#define IDS_CPSUI_DEFAULT_TRAY          64811
#define IDS_CPSUI_FORMSOURCE            64812
#define IDS_CPSUI_MANUALFEED            64813
#define IDS_CPSUI_PRINTERMEM_KB         64814
#define IDS_CPSUI_PRINTERMEM_MB         64815
#define IDS_CPSUI_PAGEPROTECT           64816
#define IDS_CPSUI_HALFTONE_SETUP        64817
#define IDS_CPSUI_INSTFONTCART          64818
#define IDS_CPSUI_SLOT1                 64819
#define IDS_CPSUI_SLOT2                 64820
#define IDS_CPSUI_SLOT3                 64821
#define IDS_CPSUI_SLOT4                 64822
#define IDS_CPSUI_LEFT_SLOT             64823
#define IDS_CPSUI_RIGHT_SLOT            64824
#define IDS_CPSUI_STAPLER               64825
#define IDS_CPSUI_STAPLER_ON            64826
#define IDS_CPSUI_STAPLER_OFF           64827
#define IDS_CPSUI_STACKER               64828
#define IDS_CPSUI_MAILBOX               64829
#define IDS_CPSUI_COPY                  64830
#define IDS_CPSUI_COPIES                64831
#define IDS_CPSUI_TOTAL                 64832
#define IDS_CPSUI_MAKE                  64833
#define IDS_CPSUI_PRINT                 64834
#define IDS_CPSUI_FAX                   64835
#define IDS_CPSUI_PLOT                  64836
#define IDS_CPSUI_SLOW                  64837
#define IDS_CPSUI_FAST                  64838
#define IDS_CPSUI_ROTATED               64839
#define IDS_CPSUI_RESET                 64840
#define IDS_CPSUI_ALL                   64841
#define IDS_CPSUI_DEVICE                64842
#define IDS_CPSUI_SETTINGS              64843
#define IDS_CPSUI_REVERT                64844
#define IDS_CPSUI_CHANGES               64845
#define IDS_CPSUI_CHANGED               64846
#define IDS_CPSUI_WARNING               64847
#define IDS_CPSUI_ABOUT                 64848
#define IDS_CPSUI_VERSION               64849
#define IDS_CPSUI_NO_NAME               64850
#define IDS_CPSUI_SETTING               64851
#define IDS_CPSUI_DEVICE_SETTINGS       64852
#define IDS_CPSUI_STDDOCPROPTAB1        64853
#define IDS_CPSUI_STDDOCPROPTAB2        64854
#define IDS_CPSUI_PAGEORDER             64855
#define IDS_CPSUI_FRONTTOBACK           64856
#define IDS_CPSUI_BACKTOFRONT           64857
#define IDS_CPSUI_QUALITY_SETTINGS      64858
#define IDS_CPSUI_QUALITY_DRAFT         64859
#define IDS_CPSUI_QUALITY_BETTER        64860
#define IDS_CPSUI_QUALITY_BEST          64861
#define IDS_CPSUI_QUALITY_CUSTOM        64862
#define IDS_CPSUI_OUTPUTBIN             64863
#define IDS_CPSUI_NUP                   64864
#define IDS_CPSUI_NUP_NORMAL            64865
#define IDS_CPSUI_NUP_TWOUP             64866
#define IDS_CPSUI_NUP_FOURUP            64867
#define IDS_CPSUI_NUP_SIXUP             64868
#define IDS_CPSUI_NUP_NINEUP            64869
#define IDS_CPSUI_NUP_SIXTEENUP         64870
#define IDS_CPSUI_SIDE1                 64871
#define IDS_CPSUI_SIDE2                 64872
#define IDS_CPSUI_BOOKLET               64873

#if (NTDDI_VERSION >= NTDDI_VISTA)
//NOTE: remove these poster defines when removing poster feature.
#define IDS_CPSUI_POSTER                64874
#define IDS_CPSUI_POSTER_2x2            64875
#define IDS_CPSUI_POSTER_3x3            64876
#define IDS_CPSUI_POSTER_4x4            64877

#define IDS_CPSUI_NUP_DIRECTION         64878
#define IDS_CPSUI_RIGHT_THEN_DOWN       64879
#define IDS_CPSUI_DOWN_THEN_RIGHT       64880
#define IDS_CPSUI_LEFT_THEN_DOWN        64881
#define IDS_CPSUI_DOWN_THEN_LEFT        64882

//NOTE: remove these manual duplex defines when removing the feature.
#define IDS_CPSUI_MANUAL_DUPLEX         64883
#define IDS_CPSUI_MANUAL_DUPLEX_ON      64884
#define IDS_CPSUI_MANUAL_DUPLEX_OFF     64885

#define IDS_CPSUI_ROT_PORT              64886

//NOTE: remove this staple defines when removing the feature.
#define IDS_CPSUI_STAPLE                64887

#define IDS_CPSUI_BOOKLET_EDGE          64888
#define IDS_CPSUI_BOOKLET_EDGE_LEFT     64889
#define IDS_CPSUI_BOOKLET_EDGE_RIGHT    64890
#define IDS_CPSUI_NUP_BORDER            64891
#define IDS_CPSUI_NUP_BORDERED          64892
#define IDS_CPSUI_STRID_LAST            64892
#else
#define IDS_CPSUI_STRID_LAST            64873
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif      // _COMPSTUI

