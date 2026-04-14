//=============================================================================
//
// @module      WpdMtpExtensions.h
//
// @created     12-08-2004
//
// @abstract    Contains WPD definitions for working with MTP vendor-extended functionalities
//
// @copyright   (C) COPYRIGHT MICROSOFT CORPORATION, 2004
//
//=============================================================================

#include <winapifamily.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

/**************************************************************************** 
* This section defines WPD commands for MTP vendor-extended operations
****************************************************************************/
// {4D545058-1A2E-4106-A357-771E0819FC56}
DEFINE_GUID( WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 );

//
// MTP extended commands for WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS
//
// Cmd Key: WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES
// Usage:   queries for vendor extended operation codes
// Inputs:  None
// Outputs: WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: an IPortableDevicePropVariantCollection (of VT_UI4)
//                                which contains all vendor-extended operation codes 
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 11 ); 

//
// Cmd Key: WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE 
// Usage:   sends a MTP command block that no data phase follows
// Inputs:  WPD_PROPERTY_MTP_EXT_OPERATION_CODE (VT_UI4): identifies the vendor-extended MTP operation code
//          WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: An IPortableDevicePropVariantCollection (of VT_UI4)
//                                                 which identifies the required params for the vendor operation code.
// Outputs: WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: [VT_UI4] the response code to the vendor operation code, and 
//          WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: An IPortableDevicePropVariantCollection (of VT_UI4) identifying response params if any (could be empty)
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 12 ); 

//
// Cmd Key: WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ 
// Usage:   sends a MTP command block followed by a data phase with data from Device to Host
// Inputs:  WPD_PROPERTY_MTP_EXT_OPERATION_CODE (VT_UI4): identifies the vendor-extended MTP operation code
//          WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: An IPortableDevicePropVariantCollection (of VT_UI4)
//                                                 which identifies the required params for the vendor operation code.
// Outputs: WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: [VT_UI8] Returns the total data size in bytes (excluding any overhead) coming from device.
//                                                         if Devie reports unknown datasize (0xFFFFFFFF), call ReadData() repeatedly until a short chunk received
//          WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: [VT_UI4] Returns the optimal size of the transfer buffer
//          WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: [VT_LPWSTR] Returned as a context idetifier for subsequent data transfer 
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 13 ); 

//
// Cmd Key: WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE 
// Usage:   sends a MTP command block followed by a data phase with data from Host to Device 
// Inputs:  WPD_PROPERTY_MTP_EXT_OPERATION_CODE (VT_UI4): identifies the vendor-extended MTP operation code
//          WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: An IPortableDevicePropVariantCollection (of VT_UI4)
//                                                 which identifies the required params for the vendor operation code.
//          WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: [VT_UI8] Specifies the total data size in bytes (excluding any overhead) to be sent to device
// Outputs: WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: [VT_UI4] Returns the optimal size of the transfer buffer
//          WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: [VT_LPWSTR] Returned as a context idetifier for subsequent data transfer 
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 14 );

//
// Cmd Key: WPD_COMMAND_MTP_EXT_READ_DATA 
// Usage:   receives a chunk of data from device following WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ
// Inputs:  WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: [VT_LPWSTR] The context idetifier returned in previous calls 
//          WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: [VT_UI4] specifies the next number of bytes to read.
//          WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: [VT_VECTOR|VT_UI1] specifies the buffer to which the data from device will be copied
// Outputs: WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: [VT_UI4] returns actual number of bytes (no overhead) received from device in a read call 
//          WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: [VT_VECTOR|VT_UI1] Returns the buffer with received data
//          
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_READ_DATA , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 15 ); 

//
// Cmd Key: WPD_COMMAND_MTP_EXT_WRITE_DATA 
// Usage:   sends a chunk of data to device following WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE
// Inputs:  WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: [VT_LPWSTR] The context idetifier returned in previous calls 
//          WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: [VT_UI4] specifies the next number of bytes to write.
//          WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: [VT_VECTOR|VT_UI1] specifies the buffer which contains the data to send to device 
// Outputs: WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: [VT_UI4] returns actual number of bytes (no overhead) sent to device in a write call 
//    
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_WRITE_DATA , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 16 );

//
// Cmd Key: WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER 
// Usage:   completes a data transfer and read response from device. The transfer is initiated by either 
//              WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ, or WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE
// Inputs:  WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: [VT_LPWSTR] The context idetifier returned in previous calls 
// Outputs: WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: [VT_UI4] the response code to the vendor operation code, and 
//          WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: An IPortableDevicePropVariantCollection (of VT_UI4) identifying response params if any (could be empty)
//  
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 17 );  

//
// Cmd Key: WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION
// Usage:   retrieves the vendor extension description string (as defined by DeviceInfo dataset)
// Inputs:  None
// Outputs: WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: [VT_LPWSTR] contains the vendor extension description string 
DEFINE_PROPERTYKEY( WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 18 ); 

// Command Parameters 
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_OPERATION_CODE , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1001 );    // [ VT_UI4 ] : Input param which identifies the vendor-extended MTP operation code
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1002 );    // [ VT_UNKNOWN ] : Input IPortableDevicePropVariantCollection (of VT_UI4) specifying the params for the vendor operation
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_RESPONSE_CODE , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1003 );    // [ VT_UI4 ] : Output param which identifies the response code for the vendor operation
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1004 );    // [ VT_UNKNOWN ] : Returns an IPortableDevicePropVariantCollection (of VT_UI4) of response params for the vendor operation
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1005 );    // [ VT_UNKNOWN ] : Returns an IPortableDevicePropVariantCollection (of VT_UI4) of Vendor-extended MTP codes 
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1006 );    // [ VT_LPWSTR ] : Returned as a context idetifier (a string value) for subsequent data transfer 
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1007 );    // [ VT_UI8 ] : Input (when writing data) or output (when reading data) param which specifies total data size in bytes (excluding any overhead)
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1008 ); // [ VT_UI4 ] : Input param specifying the number of bytes to read from device in a series of read calls 
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1009 ); // [ VT_UI4 ] : Output param specifying the actual number of bytes (no overhead) received from device in a read call 
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1010 ); // [ VT_UI4 ] : Input specifying the number of bytes to send to device in a series of write calls
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1011 ); // [ VT_UI4 ] : Returns the actual number of bytes (no overhead) sent to device in a write call 
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_TRANSFER_DATA , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1012 ); // [ VT_VECTOR|VT_UI1 ] : Stores the binary data to transfer from/to device
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1013 ); // [ VT_UI4 ] : Returns the optimal size of the transfer buffer
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION , 0x4d545058, 0x1a2e, 0x4106, 0xa3, 0x57, 0x77, 0x1e, 0x8, 0x19, 0xfc, 0x56 , 1014 ); // [ VT_LPWSTR ] : Returns vendor extension description string


/**************************************************************************** 
* This section defines the GUID for MTP Vendor-extended object properties 
****************************************************************************/
//
// Microsoft MTP driver combines this GUID and any vendor-extended MTP object property code (as pid) 
// to construct a WPD PROPERTYKEY, which is reported to WPD applications as a WPD property.
// For example, vendor extended object prop code, 0xD801, will be reported as WPD PROPERTYKEY:
// 	{4D545058-4FCE-4578-95C8-8698A9BC0F49}\D801
//
DEFINE_GUID( WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS , 0x4d545058, 0x4fce, 0x4578, 0x95, 0xc8, 0x86, 0x98, 0xa9, 0xbc, 0xf, 0x49 );


/**************************************************************************** 
* This section defines the GUID for MTP Vendor-extended device properties 
****************************************************************************/
//
// Microsoft MTP driver combines this GUID and any vendor-extended MTP device property code (as pid) 
// to construct a WPD PROPERTYKEY, which is reported to WPD applications as a WPD property.
// For example, vendor extended device prop code, 0xD001, will be reported as WPD PROPERTYKEY:
// 	{4D545058-8900-40b3-8F1D-DC246E1E8370}\D001
//
DEFINE_GUID( WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS , 0x4d545058, 0x8900, 0x40b3, 0x8f, 0x1d, 0xdc, 0x24, 0x6e, 0x1e, 0x83, 0x70 );


/**************************************************************************** 
* This section defines the mapping between WPD formats and MTP Vendor-extended formats
****************************************************************************/
//
// For a MTP vendor-extended format to work with WPD, Microsoft MTP driver creates a new WPD format GUID by combining the vendor 
// format code (UINT16) and WPD_OBJECT_FORMAT_UNSPECIFIED except its highest 16 bits (replaced by the vendor format code).
// For example, vendor-extended format code, 0xB001, will be reported as WPD format GUID:
//	{B0010000-AE6C-4804-98BA-C57B46965FE7}
//


/**************************************************************************** 
* This section defines the mapping between WPD event GUIDs and MTP Vendor-extended event codes
****************************************************************************/
//
// For a MTP vendor-extended event to work with WPD, Microsoft MTP driver creates a new WPD event GUID by 
// replacing the highest 16 bits of WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS with the vendor event code (UINT16).
// For example, vendor-extended event code, 0xC001, will be reported as WPD event GUID:
//	{C0010000-5738-4ff2-8445-BE3126691059}
//
DEFINE_GUID( WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS , 0x00000000, 0x5738, 0x4ff2, 0x84, 0x45, 0xbe, 0x31, 0x26, 0x69, 0x10, 0x59);


/**************************************************************************** 
* This section defines the WPD property key for event parameters of a vendor-extended event
****************************************************************************/
//
// Microsoft MTP driver reports a vendor-extended event to WPD with:
//      WPD_EVENT_PARAMETER_EVENT_ID: the WPD event GUID defined above; and 
//      WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: a collection of PROPVARIANTs which map to parameters of the event.
// If there is no parameters for the event, the collection will be empty.
//
//	{4D545058-EF88-4e4d-95C3-4F327F728A96}
DEFINE_PROPERTYKEY( WPD_PROPERTY_MTP_EXT_EVENT_PARAMS , 0x4d545058, 0xef88, 0x4e4d, 0x95, 0xc3, 0x4f, 0x32, 0x7f, 0x72, 0x8a, 0x96 , 1011 );    // [ VT_UNKNOWN ] : Returns an IPortableDevicePropVariantCollection (of VT_UI4) of event params for a vendor-extended event

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

