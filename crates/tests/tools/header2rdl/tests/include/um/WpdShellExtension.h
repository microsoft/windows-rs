//=============================================================================
//
// @module      WpdShellExtension.h
//
// @created     06-01-2005
//
// @abstract    Contains property keys specific to the WPD shell extension
//
// @copyright   (C) COPYRIGHT MICROSOFT CORPORATION, 2005
//
//=============================================================================


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/**************************************************************************** 
* This section contains the WPD NSE CLSID to be used in SHParseDisplayName() 
* 
* 
****************************************************************************/ 
DEFINE_GUID( CLSID_WPD_NAMESPACE_EXTENSION , 0x35786d3c, 0xb075, 0x49b9, 0x88, 0xdd, 0x02, 0x98, 0x76, 0xe1, 0x1c, 0x01 ); 



/**************************************************************************** 
* This section defines all property keys associated with: 
* WPDNSE_OBJECT_PROPERTIES_V1 
* 
* 
****************************************************************************/ 
DEFINE_GUID( WPDNSE_OBJECT_PROPERTIES_V1 , 0x34d71409, 0x4b47, 0x4d80, 0xaa, 0xac, 0x3a, 0x28, 0xa4, 0xa3, 0xb3, 0xe6 ); 

// Properties 
DEFINE_PROPERTYKEY( WPDNSE_OBJECT_HAS_CONTACT_PHOTO , 0x34d71409, 0x4b47, 0x4d80, 0xaa, 0xac, 0x3a, 0x28, 0xa4, 0xa3, 0xb3, 0xe6 , 2 ); // [ VT_BOOL ] Indicates whether the object has a contact photo resource.  
DEFINE_PROPERTYKEY( WPDNSE_OBJECT_HAS_THUMBNAIL , 0x34d71409, 0x4b47, 0x4d80, 0xaa, 0xac, 0x3a, 0x28, 0xa4, 0xa3, 0xb3, 0xe6 , 3 ); // [ VT_BOOL ] Indicates whether the object has a thumbnail resource.  
DEFINE_PROPERTYKEY( WPDNSE_OBJECT_HAS_ICON , 0x34d71409, 0x4b47, 0x4d80, 0xaa, 0xac, 0x3a, 0x28, 0xa4, 0xa3, 0xb3, 0xe6 , 4 ); // [ VT_BOOL ] Indicates whether the object has an icon resource.  
DEFINE_PROPERTYKEY( WPDNSE_OBJECT_HAS_AUDIO_CLIP , 0x34d71409, 0x4b47, 0x4d80, 0xaa, 0xac, 0x3a, 0x28, 0xa4, 0xa3, 0xb3, 0xe6 , 5 ); // [ VT_BOOL ] Indicates whether the object has a voice annotation resource.  
DEFINE_PROPERTYKEY( WPDNSE_OBJECT_HAS_ALBUM_ART , 0x34d71409, 0x4b47, 0x4d80, 0xaa, 0xac, 0x3a, 0x28, 0xa4, 0xa3, 0xb3, 0xe6 , 6 ); // [ VT_BOOL ] Indicates whether the object has an album art resource.  
DEFINE_PROPERTYKEY( WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE , 0x34d71409, 0x4b47, 0x4d80, 0xaa, 0xac, 0x3a, 0x28, 0xa4, 0xa3, 0xb3, 0xe6 , 7 ); // [ VT_UI4 ] The optimal buffer size clients can use to read data chunks of the default resource.  

/**************************************************************************** 
* This section defines all the property page values used with extensible 
* property pages. Property sheet extensions will receive a uint with one of
* of these values in the LOWORD and possibly an index in the HIWORD when
* their IShellPropSheetExt::ReplacePage method is called. Indexes in the
* HIWORD refer to the index into the PIDL array of the item whose property
* page will be replaced. This allows you to replace the correct storage or
* device property page in a multi-select scenario.
* 
* 
****************************************************************************/ 
#define WPDNSE_PROPSHEET_DEVICE_GENERAL     0x00000001
#define WPDNSE_PROPSHEET_STORAGE_GENERAL    0x00000002
#define WPDNSE_PROPSHEET_CONTENT_GENERAL    0x00000004
#define WPDNSE_PROPSHEET_CONTENT_REFERENCES 0x00000008
#define WPDNSE_PROPSHEET_CONTENT_RESOURCES  0x00000010
#define WPDNSE_PROPSHEET_CONTENT_DETAILS    0x00000020



/**************************************************************************** 
* This section defines the IBindCtx options used by IShellFolder::BindToObject.
* 
* 
****************************************************************************/ 
// BindCtx key which tells an IShellFolder to enumerate as quickly as possible.
// A limited property set will be available for objects when this bind option
// is used.  The limited property set contains name, size, date modified, the
// read-only, hidden and system file attributes, and the supported resources.
#define STR_WPDNSE_FAST_ENUM              L"WPDNSE Fast Enum"

// BindCtx key which tells an IShellFolder to enumerate as quickly as possible.
// A limited property set will be available for objects when this bind option
// is used.  The limited property set contains name, persistent unique id, and
// the content type.
#define STR_WPDNSE_SIMPLE_ITEM            L"WPDNSE SimpleItem"

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

