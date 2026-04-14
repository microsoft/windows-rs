/****************************************************************************
*                                                                           *
* WcmErrors.w - Definitions for error code and error message                *
*                                                                           *
* Component: Windows Config Management                                      *
*                                                                           *
* Copyright (c) 1999 - 2001, Microsoft Corporation                          *
*                                                                           *
****************************************************************************/

#ifndef _WcmErrors_
#define _WcmErrors_

//_____________________________________________________________________________
//
// READ THE FOLLOWING BEFORE YOU CHANGE ANY ERROR CODE DEFINITION
//
// Error code numbers and names are defined in this file. Be sure to make
// error code descriptions consistent with the update here. 
//
// Error code descriptions are listed in other files:
//   As unmanaged resource, in dll\SmiEngine.rc
//
//
// IMPORTANT NOTE for STATE_MANAGEMENT HRs. HRs in range of 0x80221000 - 0x80221fff
//   may not be returned from state APIs, but instead may only appear inside a
//   SettingsResult object.
//   
//_____________________________________________________________________________


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Facility 22 is reserved for State Management Error Codes. See WinError.h
//
// #define FACILITY_STATE_MANAGEMENT        34  // 0x22
//


//
// State engine processing errors
//


//
// MessageId: WCM_E_INTERNALERROR
//
// MessageText:
//
//  Unspecified internal error in the state engine.
//
#define WCM_E_INTERNALERROR                     _HRESULT_TYPEDEF_(0x80220000L)

//
// MessageId: WCM_E_STATENODENOTFOUND
//
// MessageText:
//
//  State node is not found.
//
#define WCM_E_STATENODENOTFOUND                 _HRESULT_TYPEDEF_(0x80220001L)          

//
// MessageId: WCM_E_STATENODENOTALLOWED
//
// MessageText:
//
//  State node is not allowed.
//
#define WCM_E_STATENODENOTALLOWED               _HRESULT_TYPEDEF_(0x80220002L)          

//
// MessageId: WCM_E_ATTRIBUTENOTFOUND
//
// MessageText:
//
//  Attribute is not found.
//
#define WCM_E_ATTRIBUTENOTFOUND                 _HRESULT_TYPEDEF_(0x80220003L)          

//
// MessageId: WCM_E_ATTRIBUTENOTALLOWED
//
// MessageText:
//
//  Attribute is not allowed.
//
#define WCM_E_ATTRIBUTENOTALLOWED               _HRESULT_TYPEDEF_(0x80220004L)          

//
// MessageId: WCM_E_INVALIDVALUE
//
// MessageText:
//
//  Valie is invalid.
//
#define WCM_E_INVALIDVALUE                      _HRESULT_TYPEDEF_(0x80220005L)          

//
// MessageId: WCM_E_INVALIDVALUEFORMAT
//
// MessageText:
//
//  Value is in invalid format.
//
#define WCM_E_INVALIDVALUEFORMAT                _HRESULT_TYPEDEF_(0x80220006L)          

//
// MessageId: WCM_E_TYPENOTSPECIFIED
//
// MessageText:
//
//  XSD type is missing in metadata.
//
#define WCM_E_TYPENOTSPECIFIED                  _HRESULT_TYPEDEF_(0x80220007L)               

//
// MessageId: WCM_E_INVALIDDATATYPE
//
// MessageText:
//
//  Data type is unexpected, or existing data does not match the type.
//
#define WCM_E_INVALIDDATATYPE                   _HRESULT_TYPEDEF_(0x80220008L)                  
          
//
// MessageId: WCM_E_NOTPOSITIONED
//
// MessageText:
//
//  Enumerator is not positioned.
//
#define WCM_E_NOTPOSITIONED                     _HRESULT_TYPEDEF_(0x80220009L)                 

//
// MessageId: WCM_E_READONLYITEM
//
// MessageText:
//
//  Cannot update a read-only setting or attribute.
//
#define WCM_E_READONLYITEM                      _HRESULT_TYPEDEF_(0x8022000AL)               

//
// MessageId: WCM_E_INVALIDPATH
//
// MessageText:
//
//  Name or path of a state node is in invalid format.
//
#define WCM_E_INVALIDPATH                       _HRESULT_TYPEDEF_(0x8022000BL)             

//
// MessageId: WCM_E_WRONGESCAPESTRING
//
// MessageText:
//
//  Wrong XML escape sequence in string.
//
#define WCM_E_WRONGESCAPESTRING                 _HRESULT_TYPEDEF_(0x8022000CL)             

//
// MessageId: WCM_E_INVALIDVERSIONFORMAT
//
// MessageText:
//
//  Invalid version format.
//
#define WCM_E_INVALIDVERSIONFORMAT              _HRESULT_TYPEDEF_(0x8022000DL)          

//
// MessageId: WCM_E_INVALIDLANGUAGEFORMAT
//
// MessageText:
//
//  Invalid language string format.
//
#define WCM_E_INVALIDLANGUAGEFORMAT             _HRESULT_TYPEDEF_(0x8022000EL)          

//
// MessageId: WCM_E_KEYNOTCHANGEABLE
//
// MessageText:
//
//  Not allowed to change value in a key member.
//
#define WCM_E_KEYNOTCHANGEABLE                  _HRESULT_TYPEDEF_(0x8022000FL)        

//
// MessageId: WCM_E_EXPRESSIONNOTFOUND
//
// MessageText:
//
//  Expression is not defined.
//
#define WCM_E_EXPRESSIONNOTFOUND                _HRESULT_TYPEDEF_(0x80220010L)             

//
// MessageId: WCM_E_SUBSTITUTIONNOTFOUND
//
// MessageText:
//
//  Substitution is not defined.
//
#define WCM_E_SUBSTITUTIONNOTFOUND              _HRESULT_TYPEDEF_(0x80220011L)             

//
// MessageId: WCM_E_USERALREADYREGISTERED
//
// MessageText:
//
//  User is already registered.
//
#define WCM_E_USERALREADYREGISTERED             _HRESULT_TYPEDEF_(0x80220012L)         

//
// MessageId: WCM_E_USERNOTFOUND
//
// MessageText:
//
//  User is not registered.
//
#define WCM_E_USERNOTFOUND                      _HRESULT_TYPEDEF_(0x80220013L)             

//
// MessageId: WCM_E_NAMESPACENOTFOUND
//
// MessageText:
//
//  Namespace is not registered.
//
#define WCM_E_NAMESPACENOTFOUND                 _HRESULT_TYPEDEF_(0x80220014L)                 

//
// MessageId: WCM_E_NAMESPACEALREADYREGISTERED
//
// MessageText:
//
//  Namespace is already registered.
//
#define WCM_E_NAMESPACEALREADYREGISTERED        _HRESULT_TYPEDEF_(0x80220015L)                   

//
// MessageId: WCM_E_STORECORRUPTED
//
// MessageText:
//
//  State store is in corrupted state.
//
#define WCM_E_STORECORRUPTED                    _HRESULT_TYPEDEF_(0x80220016L)                 

//
// MessageId: WCM_E_INVALIDEXPRESSIONSYNTAX
//
// MessageText:
//
//  Expression format is invalid.
//
#define WCM_E_INVALIDEXPRESSIONSYNTAX           _HRESULT_TYPEDEF_(0x80220017L)                   

//
// MessageId: WCM_E_NOTIFICATIONNOTFOUND
//
// MessageText:
//
//  No matching Notification found.
//
#define WCM_E_NOTIFICATIONNOTFOUND              _HRESULT_TYPEDEF_(0x80220018L)

//
// MessageId: WCM_E_CONFLICTINGASSERTION
//
// MessageText:
//
//  New restriction is conflicting with existing restriction.
//
#define WCM_E_CONFLICTINGASSERTION              _HRESULT_TYPEDEF_(0x80220019L)        

//
// MessageId: WCM_E_ASSERTIONFAILED
//
// MessageText:
//
//  Assertion Validation failed.
//
#define WCM_E_ASSERTIONFAILED                   _HRESULT_TYPEDEF_(0x8022001AL)

//
// MessageId: WCM_E_DUPLICATENAME
//
// MessageText:
//
//  Name already exists.
//
#define WCM_E_DUPLICATENAME                     _HRESULT_TYPEDEF_(0x8022001BL)      
    
//
// MessageId: WCM_E_INVALIDKEY
//
// MessageText:
//
//  Member referenced by the key does not match the complexType definition.
//
#define WCM_E_INVALIDKEY                        _HRESULT_TYPEDEF_(0x8022001CL) 

//
// MessageId: WCM_E_INVALIDSTREAM
//
// MessageText:
//
//  Tried to load invalid data from stream.
//
#define WCM_E_INVALIDSTREAM                     _HRESULT_TYPEDEF_(0x8022001DL)

//
// MessageId: WCM_E_HANDLERNOTFOUND
//
// MessageText:
//
//  Handler is not defined.
//
#define WCM_E_HANDLERNOTFOUND                   _HRESULT_TYPEDEF_(0x8022001EL)            

//
// MessageId: WCM_E_INVALIDHANDLERSYNTAX
//
// MessageText:
//
//  Handler attribute is of invalid syntax.
//
#define WCM_E_INVALIDHANDLERSYNTAX              _HRESULT_TYPEDEF_(0x8022001FL)

//
// MessageId: WCM_E_VALIDATIONFAILED
//
// MessageText:
//
//  Validation of metadata failed.
//
#define WCM_E_VALIDATIONFAILED                  _HRESULT_TYPEDEF_(0x80220020L)

//
// MessageId: WCM_E_RESTRICTIONFAILED
//
// MessageText:
//
//  Invalid setting value on restriction.
//
#define WCM_E_RESTRICTIONFAILED                 _HRESULT_TYPEDEF_(0x80220021L)

//
// MessageId: WCM_E_MANIFESTCOMPILATIONFAILED
//
// MessageText:
//
//  XSD/XML Manifest compilation failed.
//
#define WCM_E_MANIFESTCOMPILATIONFAILED         _HRESULT_TYPEDEF_(0x80220022L)

//
// MessageId: WCM_E_CYCLICREFERENCE
//
// MessageText:
//
//  Cyclic reference detected.
//
#define WCM_E_CYCLICREFERENCE                   _HRESULT_TYPEDEF_(0x80220023L)

//
// MessageId: WCM_E_MIXTYPEASSERTION
//
// MessageText:
//
//  Assertions using shared and per-user settings are not supported
//
#define WCM_E_MIXTYPEASSERTION                 _HRESULT_TYPEDEF_(0x80220024L)

//
// MessageId: WCM_E_NOTSUPPORTEDFUNCTION
//
// MessageText:
//
//  Not supported function is found
//
#define WCM_E_NOTSUPPORTEDFUNCTION             _HRESULT_TYPEDEF_(0x80220025L)

//
// MessageId: WCM_E_VALUETOOBIG
//
// MessageText:
//
//  A value is too big to process
//
#define WCM_E_VALUETOOBIG                      _HRESULT_TYPEDEF_(0x80220026L)

//
// MessageId: WCM_E_INVALIDATTRIBUTECOMBINATION
//
// MessageText:
//
//  Invalid attribute combination
//
#define WCM_E_INVALIDATTRIBUTECOMBINATION      _HRESULT_TYPEDEF_(0x80220027L)  

//
// MessageId: WCM_E_ABORTOPERATION
//
// MessageText:
//
//  Current operation should be aborted
//
#define WCM_E_ABORTOPERATION                   _HRESULT_TYPEDEF_(0x80220028L)

//
// MessageId: WCM_E_MISSINGCONFIGURATION
//
// MessageText:
//
//  CONFIGURATION and or related tags are missing 
//
#define WCM_E_MISSINGCONFIGURATION              _HRESULT_TYPEDEF_(0x80220029L)

//
// MessageId: WCM_E_INVALIDPROCESSORFORMAT
//
// MessageText:
//
//  The processor architecture attribute has an invalid value
//
#define WCM_E_INVALIDPROCESSORFORMAT            _HRESULT_TYPEDEF_(0x8022002AL)

//
// MessageId: WCM_E_SOURCEMANEMPTYVALUE
//
// MessageText:
//
//  The source manifest has empty value
//
#define WCM_E_SOURCEMANEMPTYVALUE               _HRESULT_TYPEDEF_(0x8022002BL)

//
// HRs below this point should not be returned from APIs. Rather, they
// should only be embedded in SettingsResult objects.
//

//
// MessageId: WCM_S_INTERNALERROR
//
// NOTE: This HR should never be returned from APIs or as a result
//
// MessageText:
//
//  Unspecified internal warning in the state engine.
//
#define WCM_S_INTERNALERROR                     _HRESULT_TYPEDEF_(0x00221000L)

//
// MessageId: WCM_S_ATTRIBUTENOTFOUND
//
// NOTE: This HR may not be returned from APIs, but must always be contained
//      within a SettingsResult object.
//
// MessageText:
//
//  Attribute not found
//
#define WCM_S_ATTRIBUTENOTFOUND                 _HRESULT_TYPEDEF_(0x00221001L)

//
// MessageId: WCM_S_LEGACYSETTINGWARNING
//
// NOTE: This HR may not be returned from APIs, but must always be contained
//      within a SettingsResult object.
//
// MessageText:
//
//  Legacy setting usage for this case may have unpredictable results
//
#define WCM_S_LEGACYSETTINGWARNING              _HRESULT_TYPEDEF_(0x00221002L)

//
// MessageId: WCM_S_INVALIDATTRIBUTECOMBINATION      
//
// NOTE: This HR may not be returned from APIs, but must always be contained
//      within a SettingsResult object.
//
// MessageText:
//
//  Invalid attribute combination
//
#define WCM_S_INVALIDATTRIBUTECOMBINATION       _HRESULT_TYPEDEF_(0x00221004L)

//
// MessageId: WCM_S_ATTRIBUTENOTALLOWED
//
// MessageText:
//
//  Attribute is not allowed.
//
#define WCM_S_ATTRIBUTENOTALLOWED               _HRESULT_TYPEDEF_(0x00221005L)

//
// MessageId: WCM_S_NAMESPACENOTFOUND
//
// MessageText:
//
//  Attribute is not allowed.
//
#define WCM_S_NAMESPACENOTFOUND                 _HRESULT_TYPEDEF_(0x00221006L)

//
// MessageId: WCM_E_UNKNOWNRESULT
//
// NOTE: This HR may not be returned from APIs, but must always be contained
//      within a SettingsResult object.
//
// MessageText:
//
//  Unknown value
//
#define WCM_E_UNKNOWNRESULT                     _HRESULT_TYPEDEF_(0x80221003L)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif // _WcmErrors_
