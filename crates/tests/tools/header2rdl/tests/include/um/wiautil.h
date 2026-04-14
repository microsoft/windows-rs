/****************************************************************************
*
*  (C) COPYRIGHT 2000, MICROSOFT CORP.
*
*  FILE:        wiautil.h
*
*  VERSION:     1.0
*
*  DATE:        11/17/2000
*
*  DESCRIPTION:
*    Definitions for WIA driver helper classes and functions.
*    NOTE: This header requires wiamindr.h to be included.
*
*****************************************************************************/

#if (_WIN32_WINNT >= 0x0501) // Windows XP and later

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define STRSAFE_NO_DEPRECATE
#include <strsafe.h>

/**************************************************************************\
* CWiauFormatConverter
*
*   Helper class for converting images to BMP format.
*
\**************************************************************************/

typedef struct _BMP_IMAGE_INFO
{
    INT     Width;      // Width of the image in pixels
    INT     Height;     // Height of the image in lines
    INT     ByteWidth;  // Width of the image in bytes
    INT     Size;       // Total size of the image, including headers
} BMP_IMAGE_INFO, *PBMP_IMAGE_INFO;

typedef enum
{
    SKIP_OFF,
    SKIP_FILEHDR,
    SKIP_BOTHHDR

} SKIP_AMOUNT;

class CWiauFormatConverter
{
public:
    CWiauFormatConverter();
    ~CWiauFormatConverter();

    /**************************************************************************\
    * Init
    *
    *   Intializes this class and GDI+ for converting images. This method
    *   should be called just once.
    *
    \**************************************************************************/

    _Check_return_ HRESULT Init();


    /**************************************************************************\
    * IsFormatSupported
    *
    *   This method will verify that GDI+ supports the image format
    *   that is to be converted.
    *
    * Arguments:
    *
    *   pguidFormat - pointer to GUID format from gdiplusimaging.h
    *
    \**************************************************************************/

    _Check_return_ BOOL IsFormatSupported(_In_ const GUID *pguidFormat);


    /**************************************************************************\
    * ConvertToBmp
    *
    *   This method will convert an image to BMP format. The caller can pass
    *   a result buffer in ppDest and the size in piDestSize. Alternatively
    *   the caller can set *ppDest to NULL and *piDestSize to zero to
    *   indicate that this method should allocate the memory. The caller is
    *   responsible for freeing the memory with "delete []".
    *
    * Arguments:
    *
    *    pSource         - pointer to memory location of source image
    *    iSourceSize     - size of source image
    *    ppDest          - location to receive memory location of result image
    *    piDestSize      - location to receive size of result image
    *    pBmpImageInfo   - location to receive stats about the BMP
    *    iSkipAmt        - Indicates how much of the BMP header to skip:
    *                       SKIP_OFF     = skip neither header
    *                       SKIP_FILEHDR = skip the file header
    *                       SKIP_BOTHHDR = skip the file and info headers
    *
    \**************************************************************************/

    _Check_return_ HRESULT ConvertToBmp(_In_ BYTE *pSource, INT iSourceSize,
                                       _Outptr_result_buffer_(*piDestSize) BYTE **ppDest,
                                       _Inout_ INT *piDestSize, _Out_ BMP_IMAGE_INFO *pBmpImageInfo,
                                       SKIP_AMOUNT iSkipAmt = SKIP_OFF);

private:
    ULONG_PTR   m_Token;
    UINT        m_EncoderCount;
    BYTE       *m_pEncoderInfo;
    GUID        m_guidCodecBmp;
};


/**************************************************************************\
* CWiauPropertyList
*
*   Helper class for definining and initializing WIA properties
*
\**************************************************************************/

class CWiauPropertyList
{
private:

    int                  m_NumAlloc;    // number of slots allocated
    int                  m_NumProps;    // number of properties defined
    PROPID              *m_pId;         // property ids
    LPOLESTR            *m_pNames;      // property names
    PROPVARIANT         *m_pCurrent;    // current value
    PROPSPEC            *m_pPropSpec;   // property spec (used for WriteMultiple)
    WIA_PROPERTY_INFO   *m_pAttrib;     // property attributes

public:

    CWiauPropertyList();
    ~CWiauPropertyList();

    /**************************************************************************\
    * Init
    *
    *   Initializes the property info object.
    *
    * Arguments:
    *   NumProps - number of properties to reserve space for. This number can
    *              be larger than the actual number used, but cannot be smaller.
    *
    \**************************************************************************/

    _Check_return_ HRESULT Init(INT NumProps);


    /**************************************************************************\
    * DefineProperty
    *
    *   Adds a property definition to the object.
    *
    * Arguments:
    *   index    - pointer to an int that will be set to the property index
    *              within the object, useful for passing to other property
    *              info methods
    *   PropId   - property ID constant
    *   PropName - property name string
    *   Access   - determines access to the property, usually either
    *              WIA_PROP_READ or WIA_PROP_RW
    *   SubType  - indicates subtype of the property, usually either
    *              WIA_PROP_NONE, WIA_PROP_FLAG, WIA_PROP_RANGE, or WIA_PROP_LIST
    *
    \**************************************************************************/

    _Check_return_ HRESULT DefineProperty(_Inout_ int *pIdx, PROPID PropId,
                                         _In_ LPOLESTR PropName, ULONG Access,
                                         ULONG SubType);


    /**************************************************************************\
    * SendToWia
    *
    *   Calls the WIA service to define all of the properties currently
    *   contained in the object. Should be called once after all properties
    *   are defined and set.
    *
    * Arguments:
    *   pWiasContext - pointer to the context passed into drvInitItemProperties
    *
    \**************************************************************************/

    _Check_return_ HRESULT SendToWia(_In_ BYTE *pWiasContext);


    /**************************************************************************\
    * SetAccessSubType
    *
    *   Used to reset the access and subtype of a property.
    *
    * Arguments:
    *   index   - the property index, from DefineProperty
    *   Access  - determines access to the property, usually either
    *             WIA_PROP_READ or WIA_PROP_RW
    *   SubType - indicates subtype of the property, usually either
    *             WIA_PROP_NONE, WIA_PROP_FLAG, WIA_PROP_RANGE, or WIA_PROP_LIST
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetAccessSubType(INT index, ULONG Access, ULONG SubType);
#else
    HRESULT SetAccessSubType(INT index, ULONG Access, ULONG SubType);
#endif


    /**************************************************************************\
    * SetValidValues (flag)
    *
    *   Sets the type and current, default, and valid values of a property.
    *   Also sets property type to VT_I4 and subtype to WIA_PROP_FLAG.
    *
    * Arguments:
    *   index        - the property index, from DefineProperty
    *   defaultValue - default setting of this property on the device
    *   currentValue - current setting of this property on the device
    *   validFlags   - combination of all valid flags
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetValidValues(INT index, LONG defaultValue, LONG currentValue, LONG validFlags);
#else
    HRESULT SetValidValues(INT index, LONG defaultValue, LONG currentValue, LONG validFlags);
#endif


    /**************************************************************************\
    * SetValidValues (signed long, range)
    *
    *   Sets the type and current, default, and valid values of a property.
    *   Also sets property type to VT_I4 and subtype to WIA_PROP_RANGE.
    *
    * Arguments:
    *   index        - the property index, from DefineProperty
    *   defaultValue - default setting of this property on the device
    *   currentValue - current setting of this property on the device
    *   minValue     - minimum value for the range
    *   maxValue     - maximum value for the range
    *   stepValue    - step value for the range
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetValidValues(INT index, LONG defaultValue, LONG currentValue,
                           LONG minValue, LONG maxValue, LONG stepValue);
#else
    HRESULT SetValidValues(INT index, LONG defaultValue, LONG currentValue,
                           LONG minValue, LONG maxValue, LONG stepValue);
#endif


    /**************************************************************************\
    * SetValidValues (signed long, list)
    *
    *   Sets the type and current, default, and valid values of a property.
    *   Also sets property type to VT_I4 and subtype to WIA_PROP_LIST.
    *
    * Arguments:
    *   index        - the property index, from DefineProperty
    *   defaultValue - default setting of this property on the device
    *   currentValue - current setting of this property on the device
    *   numValues    - number of values in the list
    *   pValues      - pointer to the value list (must be valid until SendToWia
    *                  is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetValidValues(INT index, LONG defaultValue, LONG currentValue,
                           INT numValues, _In_reads_(numValues) PLONG pValues);
#else
    HRESULT SetValidValues(INT index, LONG defaultValue, LONG currentValue,
                           INT numValues, _In_reads_(numValues) PLONG pValues);
#endif


    /**************************************************************************\
    * SetValidValues (BSTR, list)
    *
    *   Sets the type and current, default, and valid values of a property.
    *   Also sets property type to VT_BSTR and subtype to WIA_PROP_LIST.
    *
    * Arguments:
    *   index        - the property index, from DefineProperty
    *   defaultValue - default setting of this property on the device
    *   currentValue - current setting of this property on the device
    *   numValues    - number of values in the list
    *   pValues      - pointer to the value list (must be valid until SendToWia
    *                  is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetValidValues(INT index, _In_ BSTR defaultValue, _In_ BSTR currentValue,
                           INT numValues, _In_reads_(numValues) BSTR *pValues);
#else
    HRESULT SetValidValues(INT index, _In_ BSTR defaultValue, _In_ BSTR currentValue,
                           INT numValues, _In_reads_(numValues) BSTR *pValues);
#endif


    /**************************************************************************\
    * SetValidValues (float, range)
    *
    *   Sets the type and current, default, and valid values of a property.
    *   Also sets property type to VT_R4 and subtype to WIA_PROP_RANGE.
    *
    * Arguments:
    *   index        - the property index, from DefineProperty
    *   defaultValue - default setting of this property on the device
    *   currentValue - current setting of this property on the device
    *   minValue     - minimum value for the range
    *   maxValue     - maximum value for the range
    *   stepValue    - step value for the range
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetValidValues(INT index, FLOAT defaultValue, FLOAT currentValue,
                           FLOAT minValue, FLOAT maxValue, FLOAT stepValue);
#else
    HRESULT SetValidValues(INT index, FLOAT defaultValue, FLOAT currentValue,
                           FLOAT minValue, FLOAT maxValue, FLOAT stepValue);
#endif


    /**************************************************************************\
    * SetValidValues (float, list)
    *
    *   Sets the type and current, default, and valid values of a property.
    *   Also sets property type to VT_R4 and subtype to WIA_PROP_LIST.
    *
    * Arguments:
    *   index        - the property index, from DefineProperty
    *   defaultValue - default setting of this property on the device
    *   currentValue - current setting of this property on the device
    *   numValues    - number of values in the list
    *   pValues      - pointer to the value list (must be valid until SendToWia
    *                  is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetValidValues(INT index, FLOAT defaultValue, FLOAT currentValue,
                           INT numValues, _In_reads_(numValues) PFLOAT pValues);
#else
    HRESULT SetValidValues(INT index, FLOAT defaultValue, FLOAT currentValue,
                           INT numValues, _In_reads_(numValues) PFLOAT pValues);
#endif


    /**************************************************************************\
    * SetValidValues (CLSID, list)
    *
    *   Sets the type and current, default, and valid values of a property.
    *   Also sets property type to VT_CLSID and subtype to WIA_PROP_LIST.
    *
    * Arguments:
    *   index        - the property index, from DefineProperty
    *   defaultValue - default setting of this property on the device
    *   currentValue - current setting of this property on the device
    *   numValues    - number of values in the list
    *   pValues      - pointer to the value list (must be valid until SendToWia
    *                  is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetValidValues(INT index, _In_ CLSID *defaultValue, _In_ CLSID *currentValue,
                           INT numValues, _In_reads_(numValues) CLSID **pValues);
#else
    HRESULT SetValidValues(INT index, _In_ CLSID *defaultValue, _In_ CLSID *currentValue,
                           INT numValues, _In_reads_(numValues) CLSID **pValues);
#endif


    /**************************************************************************\
    * SetCurrentValue (signed long)
    *
    *   Sets the current value for a property. Also sets the type to VT_I4.
    *
    * Arguments:
    *   index  - the property index, from DefineProperty
    *   value  - current setting of this property on the device
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetCurrentValue(INT index, LONG value);
#else
    HRESULT SetCurrentValue(INT index, LONG value);
#endif


    /**************************************************************************\
    * SetCurrentValue (BSTR)
    *
    *   Sets the current value for a property. Also sets the type to VT_BSTR.
    *
    * Arguments:
    *   index  - the property index, from DefineProperty
    *   value  - current setting of this property on the device
    *            (must be valid until SendToWia is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void  SetCurrentValue(INT index, _In_ BSTR value);
#else
    HRESULT SetCurrentValue(INT index, _In_ BSTR value);
#endif


    /**************************************************************************\
    * SetCurrentValue (float)
    *
    *   Sets the current value for a property. Also sets the type to VT_R4.
    *
    * Arguments:
    *   index  - the property index, from DefineProperty
    *   value  - current setting of this property on the device
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetCurrentValue(INT index, FLOAT value);
#else
    HRESULT SetCurrentValue(INT index, FLOAT value);
#endif


    /**************************************************************************\
    * SetCurrentValue (CLSID)
    *
    *   Sets the current value for a property. Also sets the type to VT_CLSID.
    *
    * Arguments:
    *   index  - the property index, from DefineProperty
    *   value  - current setting of this property on the device
    *            (must be valid until SendToWia is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetCurrentValue(INT index, _In_ CLSID *pValue);
#else
    HRESULT SetCurrentValue(INT index, _In_ CLSID *pValue);
#endif


    /**************************************************************************\
    * SetCurrentValue (SYSTEMTIME)
    *
    *   Sets the current value for a property. Also sets the type to
    *   VT_UI2 | VT_VECTOR.
    *
    * Arguments:
    *   index  - the property index, from DefineProperty
    *   value  - current setting of this property on the device
    *            (must be valid until SendToWia is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetCurrentValue(INT index, _In_ PSYSTEMTIME value);
#else
    HRESULT SetCurrentValue(INT index, _In_ PSYSTEMTIME value);
#endif


    /**************************************************************************\
    * SetCurrentValue (byte array)
    *
    *   Sets the current value for a property. Also sets the type to
    *   VT_UI1 | VT_VECTOR.
    *
    * Arguments:
    *   index  - the property index, from DefineProperty
    *   value  - pointer to current setting of this property on the device
    *            (must be valid until SendToWia is called)
    *
    \**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    void SetCurrentValue(INT index, _In_reads_opt_(size) BYTE *value, INT size);
#else
    HRESULT SetCurrentValue(INT index, _In_reads_opt_(size) BYTE *value, INT size);
#endif


    /**************************************************************************\
    * GetPropId
    *
    *   Returns the property id given a property index.
    *
    \**************************************************************************/

    _Check_return_ PROPID GetPropId(INT index) { return m_pId[index]; }


    /**************************************************************************\
    * LookupPropId
    *
    *   Finds the property index given a property ID.
    *
    \**************************************************************************/

    _Check_return_ INT LookupPropId(PROPID PropId);
};


/**************************************************************************\
* wiauGetDrvItemContext
*
*   This helper function gets the driver item context, and optionally
*   return the driver item
*
* Arguments:
*
*   pWiasContext - pointer to the item context
*   ppItemCtx    - location to store pointer to driver item context
*   ppDrvItem    - location to store pointer to driver item (can be NULL)
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauGetDrvItemContext(_In_ BYTE *pWiasContext, _Inout_ VOID **ppItemCtx,
        _Inout_opt_ IWiaDrvItem **ppDrvItem = NULL);
#else
    _Check_return_ HRESULT _stdcall wiauGetDrvItemContext(_In_ BYTE *pWiasContext, _Inout_ VOID **ppItemCtx,
        _Inout_opt_ IWiaDrvItem **ppDrvItem = NULL);
#endif


/**************************************************************************\
* wiauSetImageItemSize
*
*   Calulates the size and width in bytes for an image based on the current
*   WIA_IPA_FORMAT setting, and writes the new values to the appropriate
*   properties. If the format is not BMP, this function assumes that the
*   value passed in lSize is correct for the current format.
*
* Arguments:
*
*   pWiasContext    - pointer to item context
*   lWidth          - width of the image in pixels
*   lHeight         - height of the image in lines
*   lDepth          - depth of the image in bits
*   lSize           - size of the image as stored on the device
*   pwszExt         - optional pointer to the 3 letter extension for the
*                     item's native format (if this is NULL, the extension
*                     property won't be updated)
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauSetImageItemSize(_In_ BYTE *pWiasContext, LONG lWidth,
        LONG lHeight, LONG lDepth, LONG lSize, _In_opt_ PWSTR pwszExt = NULL);
#else
    _Check_return_ HRESULT _stdcall wiauSetImageItemSize(_In_ BYTE *pWiasContext, LONG lWidth,
        LONG lHeight, LONG lDepth, LONG lSize, _In_opt_ PWSTR pwszExt = NULL);
#endif


/**************************************************************************\
* wiauPropsInPropSpec
*
*   Returns true if one or more of the property ids in pProps are
*   contained in pPropSpecs.
*
* Arguments:
*
*   NumPropSpecs - number of property specs in the array
*   pPropSpecs   - the property specs array
*   NumProps     - number of property ids to search for
*   pProps       - pointer to the array of property ids to search for
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ BOOL wiauPropsInPropSpec(LONG NumPropSpecs,
        _In_reads_(NumPropSpecs) const PROPSPEC *pPropSpecs,
        int NumProps, _In_reads_(NumProps) PROPID *pProps);
#else
    _Check_return_ BOOL _stdcall wiauPropsInPropSpec(LONG NumPropSpecs,
        _In_reads_(NumPropSpecs) const PROPSPEC *pPropSpecs,
        int NumProps, _In_reads_(NumProps) PROPID *pProps);
#endif


/**************************************************************************\
* wiauPropInPropSpec
*
*   Returns true if the PropId property ID is in the passed pPropSpecs
*   array. Optionally will return the index of where the ID was found.
*
* Arguments:
*
*   NumPropSpecs - number of property specs in the array
*   pPropSpecs   - the property specs array
*   PropId       - property id to search for
*   pIdx         - optional pointer to the location to store the index
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ BOOL wiauPropInPropSpec(LONG NumPropSpecs,
        _In_reads_(NumPropSpecs) const PROPSPEC *pPropSpecs,
        PROPID PropId, _Out_opt_ int *pIdx = NULL);
#else
    _Check_return_ BOOL _stdcall wiauPropInPropSpec(LONG NumPropSpecs,
        _In_reads_(NumPropSpecs) const PROPSPEC *pPropSpecs,
        PROPID PropId, _Out_opt_ int *pIdx = NULL);
#endif


/**************************************************************************\
* wiauGetValidFormats
*
*   Calls drvGetWiaFormatInfo and makes a list of valid formats given
*   a tymed value. Caller is responsible for freeing the format array
*   with []delete.
*
* Arguments:
*
*   pDrv          - Pointer to WIA minidriver object (use "this")
*   pWiasContext  - WIA service context
*   TymedValue    - tymed value to search for
*   pNumFormats   - pointer to value to receive number of formats
*   ppFormatArray - pointer to a location to receive array address
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauGetValidFormats(_In_ IWiaMiniDrv *pDrv, _In_ BYTE *pWiasContext,
        LONG TymedValue, _Out_ int *pNumFormats, _Out_writes_(*pNumFormats) GUID **ppFormatArray);
#else
    _Check_return_ HRESULT _stdcall wiauGetValidFormats(_In_ IWiaMiniDrv *pDrv, _In_ BYTE *pWiasContext,
        LONG TymedValue, _Out_ int *pNumFormats, _Out_writes_(*pNumFormats)GUID **ppFormatArray);
#endif


/**************************************************************************\
* wiauGetResourceString
*
*   This helper gets a resource string and returns it as a BSTR
*
* Arguments:
*
*   hInst       - Handle to module instance
*   lResourceID - Resource ID of the target BSTR value
*   pbstrStr    - Location to store the retrieved string (caller must
*                 free this string with SysFreeString())
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauGetResourceString(HINSTANCE hInst, LONG lResourceID, _Out_ BSTR *pbstrStr);
#else
    _Check_return_ HRESULT _stdcall wiauGetResourceString(HINSTANCE hInst, LONG lResourceID, _Out_ BSTR *pbstrStr);
#endif


/**************************************************************************\
* wiauRegOpenDataW
*
*   Opens the DeviceData key. Call this function only in the STI Initialize
*   function. Call RegCloseKey when finished.
*
* Arguments:
*
*   hkeyAncestor    - HKey of parent (use hkey passed into Initialize)
*   phkeyDeviceData - Location to store opened hkey
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauRegOpenDataW(_In_ HKEY hkeyAncestor, _Inout_ HKEY *phkeyDeviceData);
#else
    _Check_return_ HRESULT _stdcall wiauRegOpenDataW(_In_ HKEY hkeyAncestor, _Inout_ HKEY *phkeyDeviceData);
#endif


/**************************************************************************\
* wiauRegOpenDataA
*
*   Opens the DeviceData key. Call this function only in the STI Initialize
*   function. Call RegCloseKey when finished.
*
* Arguments:
*
*   hkeyAncestor    - HKey of parent (use hkey passed into Initialize)
*   phkeyDeviceData - Location to store opened hkey
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauRegOpenDataA(_In_ HKEY hkeyAncestor, _Inout_ HKEY *phkeyDeviceData);
#else
    _Check_return_ HRESULT _stdcall wiauRegOpenDataA(_In_ HKEY hkeyAncestor, _Inout_ HKEY *phkeyDeviceData);
#endif


/**************************************************************************\
* wiauRegGetStrW
*
*   Use to get string value from the DeviceData section of the registry.
*
* Arguments:
*
*   hkey          - Use hkey returned from wiauRegOpenData
*   pwszValueName - Name of registry entry
*   pwszValue     - Location to store returned string
*   pdwLength     - Size of location in bytes
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauRegGetStrW(_In_ HKEY hkKey, _In_ PCWSTR pwszValueName,
        _Out_writes_bytes_(*pdwLength) PWSTR pwszValue, _Inout_ WORD *pdwLength);
#else
    _Check_return_ HRESULT _stdcall wiauRegGetStrW(_In_ HKEY hkKey, _In_ PCWSTR pwszValueName,
        _Out_writes_bytes_(*pdwLength) PWSTR pwszValue, _Inout_ DWORD *pdwLength);
#endif


/**************************************************************************\
* wiauRegGetStrA
*
*   Use to get string value from the DeviceData section of the registry.
*
* Arguments:
*
*   hkey          - Use hkey returned from wiauRegOpenData
*   pszValueName  - Name of registry entry
*   pszValue      - Location to store returned string
*   pdwLength     - Size of location in bytes
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauRegGetStrA(_In_ HKEY hkKey, _In_ PCSTR pszValueName,
        _Out_writes_bytes_(*pdwLength) PSTR pszValue, _Inout_ DWORD *pdwLength);
#else
    _Check_return_ HRESULT _stdcall wiauRegGetStrA(_In_ HKEY hkKey, _In_ PCSTR pszValueName,
        _Out_writes_bytes_(*pdwLength) PSTR pszValue, _Inout_ DWORD *pdwLength);
#endif


/**************************************************************************\
* wiauRegGetDwordW
*
*   Use to get DWORD value from the DeviceData section of the registry.
*
* Arguments:
*
*   hkey          - Use hkey returned from wiauRegOpenData
*   pwszValueName - Name of registry entry
*   pdwValue      - Location to store returned DWORD
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauRegGetDwordW(_In_ HKEY hkKey, _In_ PCWSTR pwszValueName, _Out_ DWORD *pdwValue);
#else
    _Check_return_ HRESULT _stdcall wiauRegGetDwordW(_In_ HKEY hkKey, _In_ PCWSTR pwszValueName, _Out_ DWORD *pdwValue);
#endif


/**************************************************************************\
* wiauRegGetDwordA
*
*   Use to get DWORD value from the DeviceData section of the registry.
*
* Arguments:
*
*   hkey          - Use hkey returned from wiauRegOpenData
*   pszValueName  - Name of registry entry
*   pdwValue      - Location to store returned DWORD
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauRegGetDwordA(_In_ HKEY hkKey, _In_ PCSTR pszValueName, _Out_ DWORD *pdwValue);
#else
    _Check_return_ HRESULT _stdcall wiauRegGetDwordA(_In_ HKEY hkKey, _In_ PCSTR pszValueName, _Out_ DWORD *pdwValue);
#endif


/**************************************************************************\
* WiauStrW2C
*
*   Converts a wide character string to an ANSI character string
*
* Arguments:
*   pwszSrc - wide character string to be converted
*   pszDst  - location to store the ANSI conversion
*   iSize   - size of the buffer pointed to by pszDst, in bytes
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauStrW2C(_In_reads_bytes_(iSize) _Pre_z_ WCHAR *pwszSrc,
        _Out_writes_(iSize) CHAR *pszDst, INT iSize);
#else
    _Check_return_ HRESULT _stdcall wiauStrW2C(_In_reads_bytes_(iSize) _Pre_z_ WCHAR *pwszSrc,
        _Out_writes_(iSize) CHAR *pszDst, INT iSize);
#endif

/**************************************************************************\
* WiauStrC2W
*
*   Converts an ANSI character string to a wide character string
*
* Arguments:
*   pszSrc  - ANSI string to convert
*   wpszDst - location to store the wide string
*   iSize   - size of the buffer pointed to by wpszDst, in bytes
*
\**************************************************************************/
#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauStrC2W(_In_reads_bytes_(iSize) _Pre_z_ CHAR *pszSrc,
        _Out_writes_bytes_(iSize) WCHAR *pwszDst, INT iSize);
#else
    _Check_return_ HRESULT _stdcall wiauStrC2W(_In_reads_bytes_(iSize) _Pre_z_ CHAR *pszSrc,
        _Out_writes_bytes_(iSize) WCHAR *pwszDst, INT iSize);
#endif


/**************************************************************************\
* WiauStrW2W
*
*   Copies a wide character string to another wide character string
*
* Arguments:
*   pwszSrc - wide character string to be copied
*   pwszDst - location to copy to
*   iSize   - size of the buffer pointed to by pwszDst, in bytes
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauStrW2W(_In_reads_bytes_(iSize) _Pre_z_ WCHAR *pwszSrc,
        _Out_writes_bytes_(iSize) WCHAR *pwszDst, INT iSize);
#else
    _Check_return_ HRESULT _stdcall wiauStrW2W(_In_reads_bytes_(iSize) _Pre_z_ WCHAR *pwszSrc,
        _Out_writes_bytes_(iSize) WCHAR *pwszDst, INT iSize);
#endif


/**************************************************************************\
* WiauStrC2C
*
*   Copies an ANSI character string to another ANSI character string
*
* Arguments:
*   pszSrc - ANSI string to be copied
*   pszDst - location to copy to
*   iSize  - size of the buffer pointed to by pszDst, in bytes
*
\**************************************************************************/

#if (_WIN32_WINNT == 0x0501)
    _Check_return_ HRESULT wiauStrC2C(_In_reads_z_(iSize) CHAR *pszSrc,
        _Out_writes_bytes_(iSize) CHAR *pszDst, INT iSize);
#else
    _Check_return_ HRESULT _stdcall wiauStrC2C(_In_reads_z_(iSize) CHAR *pszSrc,
        _Out_writes_bytes_(iSize) CHAR *pszDst, INT iSize);
#endif


#ifdef UNICODE

#define wiauRegOpenData wiauRegOpenDataW
#define wiauRegGetStr wiauRegGetStrW
#define wiauRegGetDword wiauRegGetDwordW

#define wiauStrT2C wiauStrW2C
#define wiauStrC2T wiauStrC2W
#define wiauStrT2W wiauStrW2W
#define wiauStrW2T wiauStrW2W
#define WIAU_DEBUG_TSTR "S"

#else

#define wiauRegOpenData wiauRegOpenDataA
#define wiauRegGetStr wiauRegGetStrA
#define wiauRegGetDword wiauRegGetDwordA

#define wiauStrT2C wiauStrC2C
#define wiauStrC2T wiauStrC2C
#define wiauStrT2W wiauStrC2W
#define wiauStrW2T wiauStrW2C
#define WIAU_DEBUG_TSTR "s"

#endif // UNICODE


/**************************************************************************\
* WIA Debugging
*
*   Definitions for debug messages. To use WIA debugging:
*   1. Set registry HKLM\System\CurrentControlSet\Control\StillImage\Debug\<ModuleName>,
*      DWORD value "DebugFlags" to the combination of the WIAUDBG_* flags
*      desired. The application and possibly the WIA service will need to be
*      restarted to pick up the new settings. The key is auto created the
*      first time the module is executed. (Note: <ModuleName> above is the
*      name of the DLL or EXE, e.g. wiavusd.dll has a registry key of
*      "HKLM\System\CurrentControlSet\Control\StillImage\Debug\wiavusd.dll".)
*   2. Or in the debugger, set g_dwDebugFlags to the combination of the
*      WIAUDBG_* flags desired. This can be done anytime during the debug
*      session.
*   3. From the module, call wiauDbgSetFlags(<flags>), where <flags> is the
*      combination of the WIAUDBG_* flags desired.
*
*   Messages will be logged to the debugger and the file
*   %systemroot%\\wiadebug.log, unless the WIAUDBG_DONT_LOG_* flags are set.
*   Setting both flags will turn off all messages.
*
*   All strings should be ASCII. Use %S in the format string to print a
*   Unicode string.
*
\**************************************************************************/

#define _STIDEBUG_H_ // WIA debugging is incompatible with stidebug.h, so don't include it

//
// Predefined debug flags
//

const DWORD WIAUDBG_ERRORS                = 0x00000001;
const DWORD WIAUDBG_WARNINGS              = 0x00000002;
const DWORD WIAUDBG_TRACES                = 0x00000004;
const DWORD WIAUDBG_FNS                   = 0x00000008;  // Function entry and exit
const DWORD WIAUDBG_DUMP                  = 0x00000010;  // Dump data
const DWORD WIAUDBG_PRINT_TIME            = 0x08000000;  // Prints time for each message
const DWORD WIAUDBG_PRINT_INFO            = 0x10000000;  // Turns on thread, file, line info
const DWORD WIAUDBG_DONT_LOG_TO_DEBUGGER  = 0x20000000;
const DWORD WIAUDBG_DONT_LOG_TO_FILE      = 0x40000000;
const DWORD WIAUDBG_BREAK_ON_ERRORS       = 0x80000000;  // Do DebugBreak on errors

//
// Don't log at all
//
const DWORD WIAUDBG_DONT_LOG = WIAUDBG_DONT_LOG_TO_FILE | WIAUDBG_DONT_LOG_TO_DEBUGGER;

//
// Set default flags
//
#ifdef DEBUG
const DWORD WIAUDBG_DEFAULT_FLAGS = WIAUDBG_ERRORS;
#else
const DWORD WIAUDBG_DEFAULT_FLAGS = WIAUDBG_DONT_LOG;
#endif

//
// FormatMessage flags
//
const DWORD WIAUDBG_MFMT_FLAGS = FORMAT_MESSAGE_IGNORE_INSERTS |
                                 FORMAT_MESSAGE_FROM_SYSTEM |
                                 FORMAT_MESSAGE_MAX_WIDTH_MASK;

#ifdef __cplusplus
extern "C" {
#endif

//
// WIA Debugging has very little overhead and should be put into retail
// code for drivers. If it's really desired to remove it, define NO_WIA_DEBUG.
//

#ifdef NO_WIA_DEBUG

#define g_dwDebugFlags 0
#define wiauDbgInit(a)
#define wiauDbgHelper(a,b,c,d)
#define wiauDbgHelper2  wiauNull3
#define wiauDbgFlags    wiauNull4
#define wiauDbgError    wiauNull2
#define wiauDbgErrorHr  wiauNull3hr
#define wiauDbgWarning  wiauNull2
#define wiauDbgTrace    wiauNull2
#define wiauDbgDump     wiauNull2
#define wiauDbgSetFlags(a) 0
#define wiauDbgLegacyError      wiauNull1
#define wiauDbgLegacyWarning    wiauNull1
#define wiauDbgLegacyTrace      wiauNull1
#define wiauDbgLegacyError2     wiauNull2h
#define wiauDbgLegacyTrace2     wiauNull2h
#define wiauDbgLegacyHresult2   wiauNullHHr

inline void wiauNull1(LPCSTR a, ...) {}
inline void wiauNull2(LPCSTR a, LPCSTR b,...) {}
inline void wiauNull2h(HINSTANCE hInstance, LPCSTR b,...) {}
inline void wiauNull3(LPCSTR a, LPCSTR b, LPCSTR c, ...) {}
inline void wiauNull3hr(HRESULT a, LPCSTR b, LPCSTR c, ...) {}
inline void wiauNull4(DWORD a, LPCSTR b, LPCSTR c, LPCSTR d, ...) {}
inline void wiauNullHHr(HINSTANCE hInstance, HRESULT hr) {}


#else // NO_WIA_DEBUG

extern DWORD  g_dwDebugFlags;
extern HANDLE g_hDebugFile;
extern DWORD  g_dwDebugFileSizeLimit;
extern BOOL   g_bDebugInited;


/**************************************************************************\
* wiauDbgInit
*
*   Call to initialize WIA debugging. If it's not called, all DLLs will
*   inherit the debug flags of the process that creates them.
*
* Arguments:
*
*   hInstance - DLL instance handle
*
\**************************************************************************/

void __stdcall wiauDbgInit(_In_opt_ HINSTANCE hInstance);
void __stdcall wiauDbgHelper(_In_ LPCSTR prefix, _In_ LPCSTR fname, _In_ LPCSTR fmt, va_list marker);
void __stdcall wiauDbgHelper2(_In_ LPCSTR prefix, _In_ LPCSTR fname, _In_ LPCSTR fmt, ...);

inline void __stdcall wiauDbgFlags(DWORD flags, _In_ LPCSTR prefix,
                                   _In_ LPCSTR fname, _In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and the flag is enabled
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & flags)) {

        va_start(marker, fmt);
        wiauDbgHelper(prefix, fname, fmt, marker);
        va_end(marker);
    }
}

inline void __stdcall wiauDbgError(_In_ LPCSTR fname, _In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and error messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_ERRORS)) {

        va_start(marker, fmt);
        wiauDbgHelper("ERROR ", fname, fmt, marker);
        va_end(marker);
    }

    if (g_dwDebugFlags & WIAUDBG_BREAK_ON_ERRORS) {
        DebugBreak();
    }
}

inline void __stdcall wiauDbgErrorHr(HRESULT hr, _In_ LPCSTR fname, _In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and error messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_ERRORS)) {

        va_start(marker, fmt);
        wiauDbgHelper("ERROR ", fname, fmt, marker);
        va_end(marker);

        CHAR szError[MAX_PATH]; \
        if(!FormatMessageA(WIAUDBG_MFMT_FLAGS, NULL, hr, 0, szError, MAX_PATH, NULL))
        {
            StringCchCopyA(szError, ARRAYSIZE(szError), "Unknown HRESULT");
        }
        wiauDbgHelper2("ERROR ", fname, "HRESULT = 0x%08x, %s", hr, szError);
    }

    if (g_dwDebugFlags & WIAUDBG_BREAK_ON_ERRORS) {
        DebugBreak();
    }
}

inline void __stdcall wiauDbgWarning(_In_ LPCSTR fname, _In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and warning messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_WARNINGS)) {

        va_start(marker, fmt);
        wiauDbgHelper("WARN  ", fname, fmt, marker);
        va_end(marker);
    }
}

inline void __stdcall wiauDbgTrace(_In_ LPCSTR fname, _In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and trace messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_TRACES)) {

        va_start(marker, fmt);
        wiauDbgHelper("      ", fname, fmt, marker);
        va_end(marker);
    }
}

inline void __stdcall wiauDbgDump(_In_ LPCSTR fname, _In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and trace messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_DUMP)) {

        va_start(marker, fmt);
        wiauDbgHelper("      ", fname, fmt, marker);
        va_end(marker);
    }
}

inline DWORD __stdcall wiauDbgSetFlags(DWORD flags)
{
    DWORD dwOld = g_dwDebugFlags;
    g_dwDebugFlags = flags;
    return dwOld;
}


inline void __stdcall wiauDbgLegacyError(_In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and error messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_ERRORS)) {

        va_start(marker, fmt);
        wiauDbgHelper("ERROR ", "", fmt, marker);
        va_end(marker);
    }

    if (g_dwDebugFlags & WIAUDBG_BREAK_ON_ERRORS) {
        DebugBreak();
    }
}

inline void __stdcall wiauDbgLegacyWarning(_In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and warning messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_WARNINGS)) {

        va_start(marker, fmt);
        wiauDbgHelper("WARN  ", "", fmt, marker);
        va_end(marker);
    }
}

inline void __stdcall wiauDbgLegacyTrace(_In_ LPCSTR fmt, ...)
{
    va_list marker;

    //
    // See if log messages are enabled and trace messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_TRACES)) {

        va_start(marker, fmt);
        wiauDbgHelper("      ", "", fmt, marker);
        va_end(marker);
    }
}

inline void __stdcall wiauDbgLegacyError2(_In_ HINSTANCE hInstance, _In_ LPCSTR fmt, ...)
{
    UNREFERENCED_PARAMETER(hInstance);
    va_list marker;

    //
    // See if log messages are enabled and error messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_ERRORS)) {

        va_start(marker, fmt);
        wiauDbgHelper("ERROR ", "", fmt, marker);
        va_end(marker);
    }

    if (g_dwDebugFlags & WIAUDBG_BREAK_ON_ERRORS) {
        DebugBreak();
    }
}

inline void __stdcall wiauDbgLegacyTrace2(_In_ HINSTANCE hInstance, _In_ LPCSTR fmt, ...)
{
    UNREFERENCED_PARAMETER(hInstance);
    va_list marker;

    //
    // See if log messages are enabled and trace messages are turned on
    //
    if (((g_dwDebugFlags & WIAUDBG_DONT_LOG) ^ WIAUDBG_DONT_LOG) &&
        (g_dwDebugFlags & WIAUDBG_TRACES)) {

        va_start(marker, fmt);
        wiauDbgHelper("      ", "", fmt, marker);
        va_end(marker);
    }
}

inline void __stdcall wiauDbgLegacyHresult2(_In_ HINSTANCE hInstance, HRESULT hr)
{
    UNREFERENCED_PARAMETER(hInstance);
    wiauDbgErrorHr(hr, "", "");
}

#endif // NO_WIA_DEBUG


//
// Macros for mapping the old WIA logging to the new system
//
#ifdef WIA_MAP_OLD_DEBUG

#define CWiaLogProc
#define WIAS_LOGPROC(x, y, z, fname) CWiauDbgFn __CWiauDbgFnObject(fname)
#define WIAS_LERROR(x,y,params) wiauDbgLegacyError ## params
#define WIAS_LWARNING(x,y,params) wiauDbgLegacyWarning ## params
#define WIAS_LTRACE(x,y,z,params) wiauDbgLegacyTrace ## params
#define WIAS_LHRESULT(x,y) wiauDbgErrorHr(y, "", "")

#define WIAS_TRACE(x) wiauDbgLegacyTrace2 ## x
#define WIAS_ERROR(x) wiauDbgLegacyError2 ## x
#define WIAS_HRESULT(x) wiauDbgLegacyHresult2 ## x
#define WIAS_ASSERT(x, y) \
        if (!(y)) { \
            WIAS_ERROR((x, (char*) TEXT("ASSERTION FAILED: %hs(%d): %hs"), __FILE__,__LINE__,#x)); \
            DebugBreak(); \
        }

#endif // WIA_MAP_OLD_DEBUG


//
// Macros for checking return values and common error conditions
//

#define REQUIRE_SUCCESS(hr, fname, msg) \
    if (FAILED(hr)) { \
        if (g_dwDebugFlags & WIAUDBG_PRINT_INFO) { \
            DWORD threadId = GetCurrentThreadId(); \
            wiauDbgError(fname, "[%s(%d): Thread 0x%X (%d)]", __FILE__, __LINE__, threadId, threadId); \
        } \
        wiauDbgErrorHr(hr, fname, msg); \
        goto Cleanup; \
    }

#define REQUIRE_OK(hr, fname, msg) \
    if ((hr) != S_OK) { \
        if (g_dwDebugFlags & WIAUDBG_PRINT_INFO) { \
            DWORD threadId = GetCurrentThreadId(); \
            wiauDbgError(fname, "[%s(%d): Thread 0x%X (%d)]", __FILE__, __LINE__, threadId, threadId); \
        } \
        wiauDbgErrorHr(hr, fname, msg); \
        goto Cleanup; \
    }

#define REQUIRE_ARGS(args, hr, fname) \
    if (args) { \
        if (g_dwDebugFlags & WIAUDBG_PRINT_INFO) { \
            DWORD threadId = GetCurrentThreadId(); \
            wiauDbgError(fname, "[%s(%d): Thread 0x%X (%d)]", __FILE__, __LINE__, threadId, threadId); \
        } \
        wiauDbgError(fname, "Invalid arg"); \
        hr = E_INVALIDARG; \
        goto Cleanup; \
    }

#define REQUIRE_ALLOC(var, hr, fname) \
    if (!(var)) { \
        if (g_dwDebugFlags & WIAUDBG_PRINT_INFO) { \
            DWORD threadId = GetCurrentThreadId(); \
            wiauDbgError(fname, "[%s(%d): Thread 0x%X (%d)]", __FILE__, __LINE__, threadId, threadId); \
        } \
        wiauDbgError(fname, "Memory allocation failed on " #var); \
        hr = E_OUTOFMEMORY; \
        goto Cleanup; \
    }

#define REQUIRE_FILEHANDLE(handle, hr, fname, msg) \
    if ((handle) == NULL || (handle) == INVALID_HANDLE_VALUE) { \
        hr = HRESULT_FROM_WIN32(::GetLastError()); \
        if (g_dwDebugFlags & WIAUDBG_PRINT_INFO) { \
            DWORD threadId = GetCurrentThreadId(); \
            wiauDbgError(fname, "[%s(%d): Thread 0x%X (%d)]", __FILE__, __LINE__, threadId, threadId); \
        } \
        wiauDbgErrorHr(hr, fname, msg); \
        goto Cleanup; \
    }

#define REQUIRE_FILEIO(ret, hr, fname, msg) \
    if (!(ret)) { \
        hr = HRESULT_FROM_WIN32(::GetLastError()); \
        if (g_dwDebugFlags & WIAUDBG_PRINT_INFO) { \
            DWORD threadId = GetCurrentThreadId(); \
            wiauDbgError(fname, "[%s(%d): Thread 0x%X (%d)]", __FILE__, __LINE__, threadId, threadId); \
        } \
        wiauDbgErrorHr(hr, fname, msg); \
        goto Cleanup; \
    }

#define REQUIRE_WIN32(err, hr, fname, msg) \
    if ((err) != ERROR_SUCCESS) { \
        hr = HRESULT_FROM_WIN32(err); \
        if (g_dwDebugFlags & WIAUDBG_PRINT_INFO) { \
            DWORD threadId = GetCurrentThreadId(); \
            wiauDbgError(fname, "[%s(%d): Thread 0x%X (%d)]", __FILE__, __LINE__, threadId, threadId); \
        } \
        wiauDbgErrorHr(hr, fname, msg); \
        goto Cleanup; \
    }


//
// Macro and class for entry/exit point tracing
//

#ifdef __cplusplus

#ifdef NO_WIA_DEBUG

#define DBG_FN(fname)

#else // NO_WIA_DEBUG

#define DBG_FN(fname) CWiauDbgFn __CWiauDbgFnObject(fname)

class CWiauDbgFn {
public:

    CWiauDbgFn(_In_ LPCSTR fname)
    {
        m_fname = fname;
        m_threadId = GetCurrentThreadId();
        wiauDbgFlags(WIAUDBG_FNS, "      ", m_fname, "Entering, thread 0x%x (%d)",
                     m_threadId, m_threadId);

    }

    ~CWiauDbgFn()
    {
        wiauDbgFlags(WIAUDBG_FNS, "      ", m_fname, "Exiting, thread 0x%x (%d)",
                     m_threadId, m_threadId);
    }

private:
    LPCSTR m_fname;
    DWORD  m_threadId;
};
#endif // NO_WIA_DEBUG

}

#else // __cplusplus

#define DBG_FN(fname) wiauDbgFlags(WIAUDBG_FNS, "      ", fname, "Entering");
#endif // __cplusplus


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //#if (_WIN32_WINNT >= 0x0501)

