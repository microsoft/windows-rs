#pragma once

#ifndef _SHDISPID_H_
#define _SHDISPID_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File: shdispid.h
//
//--------------------------------------------------------------------------

// define the events for the shell folder view
#define DISPID_SELECTIONCHANGED      200    // The Selected Items Changed
#define DISPID_FILELISTENUMDONE      201    // Done enumerating the shell folder
#define DISPID_VERBINVOKED           202    // A verb (either from the main or context menu) was invoked in the folder view
#define DISPID_DEFAULTVERBINVOKED    203    // default verb (either from the main or context menu) was invoked in the folder view
#define DISPID_BEGINDRAG             204    // user clicked on an item
#define DISPID_VIEWMODECHANGED       205    // The ListViewMode Changed
#define DISPID_NOITEMSTATE_CHANGED   206    // We went from 0->some or some->0 items in the view
#define DISPID_CONTENTSCHANGED       207    // contents of the view have changed somehow
#define DISPID_FOCUSCHANGED          208    // The Focused Item Changed
#define DISPID_CHECKSTATECHANGED     209    // Checkbox state changed.
#define DISPID_ORDERCHANGED          210    // The order of items changed
#define DISPID_VIEWPAINTDONE         211    // The enumerated items have been inserted into the view and painted
#define DISPID_COLUMNSCHANGED        212    // The set of visible details columns changed
#define DISPID_CTRLMOUSEWHEEL        213    // The mousewheel has been moved while the CTRL key was down
#define DISPID_SORTDONE              214    // Done sorting the shell folder
#define DISPID_ICONSIZECHANGED       215    // The icon size changed in the view

#define DISPID_FOLDERCHANGED         217    // The state of the folder has changed
#define DISPID_FILTERINVOKED         218    // Some filter changed
#define DISPID_WORDWHEELEDITED       219    // Text in WordWheel changed
#define DISPID_SELECTEDITEMCHANGED   220    // One of the selected items has changed (not the same as a selection change)
#define DISPID_EXPLORERWINDOWREADY   221    // Explorer window is open, been painted and is ready
#define DISPID_UPDATEIMAGE           222    // A SHCNE_UPDATEIMAGE notification was received
#define DISPID_INITIALENUMERATIONDONE 223   // Used internally by specialized views like the start menu. Not fired when the data source finishes enumeration. To detect when the data source is done enumerating, use DISPID_FILELISTENUMDONE.
#define DISPID_ENTERPRISEIDCHANGED   224    // Fired when enterprise id is changed in Common File Dialog during save as

// define the events for the ComboBoxEx control
#define DISPID_ENTERPRESSED         200     // The user hit Enter or Return


// Define Events for search object
#define DISPID_SEARCHCOMMAND_START      1
#define DISPID_SEARCHCOMMAND_COMPLETE   2
#define DISPID_SEARCHCOMMAND_ABORT      3
#define DISPID_SEARCHCOMMAND_UPDATE     4
#define DISPID_SEARCHCOMMAND_PROGRESSTEXT 5
#define DISPID_SEARCHCOMMAND_ERROR      6
#define DISPID_SEARCHCOMMAND_RESTORE    7


// Shell Add/Remove Programs events
#define DISPID_IADCCTL_DIRTY            0x100
#define DISPID_IADCCTL_PUBCAT           0x101
#define DISPID_IADCCTL_SORT             0x102
#define DISPID_IADCCTL_FORCEX86         0x103
#define DISPID_IADCCTL_SHOWPOSTSETUP    0x104
#define DISPID_IADCCTL_ONDOMAIN         0x105
#define DISPID_IADCCTL_DEFAULTCAT       0x106

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EXDISPID_H_
