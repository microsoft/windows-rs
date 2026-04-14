//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef __INFOTECH_H__
#define __INFOTECH_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <comdef.h>
#include <ocidl.h>


// {1F403BB1-9997-11d0-A850-00AA006C7D01}
DEFINE_GUID(IID_IITPropList, 
0x1f403bb1, 0x9997, 0x11d0, 0xa8, 0x50, 0x0, 0xaa, 0x0, 0x6c, 0x7d, 0x1);

// {4662daae-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_IITPropList, 
0x4662daae, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

typedef DWORD PROPID;

// Operations you can do on a property
#define PROP_ADD    0x00000000
#define PROP_DELETE 0x00000001
#define PROP_UPDATE 0x00000002

// Type of data
#define TYPE_VALUE   0x00000000
#define TYPE_POINTER 0x00000001
#define TYPE_STRING  0x00000002

// Class definition of CProperty
class CProperty
{
public:
    PROPID dwPropID;        // property ID
    DWORD cbData;           // Amount of data
    DWORD dwType;           // What type this is
    union
    {
        LPWSTR      lpszwData;   // String
        LPVOID      lpvData;     // Any kind of data
        DWORD       dwValue;     // Numerical data
    };
    BOOL fPersist;          // TRUE to persist this property

};

typedef CProperty* LPPROP;


// Interface def. for IITPropList
DECLARE_INTERFACE_(IITPropList, IPersistStreamInit)
{

    // dwOperation = operation (add, delete, update, etc.) to perform on property list
    STDMETHOD(Set)(PROPID PropID, DWORD dwData, DWORD dwOperation) PURE;
    STDMETHOD(Set)(PROPID PropID, LPVOID lpvData, DWORD cbData, DWORD dwOperation) PURE;
    STDMETHOD(Set)(PROPID PropID, LPCWSTR lpszwString, DWORD dwOperation) PURE;
    STDMETHOD(Add)(CProperty& Prop) PURE;

    STDMETHOD(Get)(PROPID PropID, CProperty& Property) PURE;
    STDMETHOD(Clear)() PURE;

    // set persistence state on property
    STDMETHOD(SetPersist)(PROPID PropID, BOOL fPersist) PURE;   // single property
    STDMETHOD(SetPersist)(BOOL fPersist) PURE;          // all properties in list

    // for enumerating properties
    STDMETHOD(GetFirst)(CProperty& Property) PURE;
    STDMETHOD(GetNext)(CProperty& Property) PURE;
    STDMETHOD(GetPropCount)(LONG &cProp) PURE;

    // persist header and data separately
    STDMETHOD(SaveHeader)(LPVOID lpvData, DWORD dwHdrSize) PURE;
    STDMETHOD(SaveData)(LPVOID lpvHeader, DWORD dwHdrSize, LPVOID lpvData, DWORD dwBufSize) PURE;
    STDMETHOD(GetHeaderSize)(DWORD& dwHdrSize) PURE;
    STDMETHOD(GetDataSize)(LPVOID lpvHeader, DWORD dwHdrSize, DWORD& dwDataSize) PURE;
    STDMETHOD(SaveDataToStream)(LPVOID lpvHeader, DWORD dwHdrSize, IStream* pStream) PURE;

    // persist to a memory buffer
    STDMETHOD(LoadFromMem)(LPVOID lpvData, DWORD dwBufSize) PURE;
    STDMETHOD(SaveToMem)(LPVOID lpvData, DWORD dwBufSize) PURE;

};

typedef IITPropList* LPITPROPLIST;


// {8fa0d5a2-dedf-11d0-9a61-00c04fb68bf7} (changed from IT 3.0)
DEFINE_GUID(IID_IITDatabase, 
0x8fa0d5a2, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

#ifdef ITPROXY

// {66673452-8C23-11d0-A84E-00AA006C7D01}
DEFINE_GUID(CLSID_IITDatabase, 
0x66673452, 0x8c23, 0x11d0, 0xa8, 0x4e, 0x0, 0xaa, 0x0, 0x6c, 0x7d, 0x1);

#else

// {4662daa9-d393-11d0-9a56-00c04fb68bf7} (changed from IT 3.0)
DEFINE_GUID(CLSID_IITDatabaseLocal, 
0x4662daa9, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

#endif	// ITPROXY


// This value is invalid for dwObjInstance params in IITDatabase methods.
#define	IITDB_OBJINST_NULL	((DWORD) 0xFFFFFFFF)


DECLARE_INTERFACE_(IITDatabase, IUnknown)
{
	STDMETHOD(Open)(LPCWSTR lpszHost, LPCWSTR lpszMoniker, DWORD dwFlags) PURE;
	STDMETHOD(Close)(void) PURE;

	// Creates an unnamed object that can be referenced in the future
	// by *pdwObjInstance.  Note that the value in *pdwObjInstance will be
	// persisted by the database when it is asked to save

	STDMETHOD(CreateObject)(REFCLSID rclsid, DWORD *pdwObjInstance) PURE;

	// Retrieves a specified IUnknown-based interface on the object identified
	// by dwObjInstance.
	STDMETHOD(GetObject)(DWORD dwObjInstance, REFIID riid, LPVOID *ppvObj) PURE;

	// To obtain a pointer to a named object's persistence the object's full
	// name (including any object-specific type prefix) should be passed in
	// lpswszObject.  If *lpwszObject is NULL, then the database's own storage
	// will be returned.  If lpwszObject is NULL, then dwObjInstance will be
	// used to identify the object and locate its persistence.  On exit,
	// *ppvPersistence will be either an IStorage* or an IStream*, depending
	// on what the caller specified with the fStream param.  The caller should
	// assume that only read operations can be performed on *ppvPersistence.
	// If the specified object's persistence doesn't exist, or if it exists
	// but is of the wrong type, then STG_E_FILENOTFOUND will be returned. 
	STDMETHOD(GetObjectPersistence)(LPCWSTR lpwszObject, DWORD dwObjInstance,
                                        LPVOID *ppvPersistence, BOOL fStream) PURE;
};

typedef IITDatabase* LPITDB;

// Document property macros
#define STDPROP_UID         1
#define STDPROP_TITLE       2
#define STDPROP_USERDATA    3
#define STDPROP_KEY         4

#define STDPROP_SORTKEY		100
#define STDPROP_DISPLAYKEY	101
#define STDPROP_SORTORDINAL 102

#define STDPROP_INDEX_TEXT    200
#define STDPROP_INDEX_VFLD    201
#define STDPROP_INDEX_DTYPE   202
#define STDPROP_INDEX_LENGTH  203
#define STDPROP_INDEX_BREAK   204

#define STDPROP_INDEX_TERM              210
#define STDPROP_INDEX_TERM_RAW_LENGTH   211

#define STDPROP_USERPROP_BASE   0x00010000
#define STDPROP_USERPROP_MAX    0x7FFFFFFF

// Property destinations for word wheels
#define SZ_WWDEST_GLOBAL    L"GLOBAL"
#define SZ_WWDEST_KEY       L"KEY"
#define SZ_WWDEST_OCC       L"OCC"


// {4662daa2-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_IITCmdInt,
0x4662daa2, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// {4662daa3-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_IITSvMgr,
0x4662daa3, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// {4CF34C30-9BF9-11d0-8764-00A0C913F764}
DEFINE_GUID(IID_IITCmdInt,
0x4cf34c30, 0x9bf9, 0x11d0, 0x87, 0x64, 0x0, 0xa0, 0xc9, 0x13, 0xf7, 0x64);

// {4E7DA031-9C11-11d0-8764-00A0C913F764}
DEFINE_GUID(IID_IITSvMgr,
0x4e7da031, 0x9c11, 0x11d0, 0x87, 0x64, 0x0, 0xa0, 0xc9, 0x13, 0xf7, 0x64);

// {4662daa5-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_IITWordWheelUpdate,
0x4662daa5, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);
    
// {8fa0d5a5-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(IID_IITBuildCollect,
0x8fa0d5a5, 0xdedf, 0x11d0, 0x9a, 0x61, 0x0, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// {4662daa4-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_IITGroupUpdate,
0x4662daa4, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// {8fa0d5aa-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(CLSID_IITIndexBuild,
0x8fa0d5aa, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// {8fa0d5ab-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(CLSID_IITWWFilterBuild,
0x8fa0d5ab, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);


// Word Breaker Defines:

// {8fa0d5a4-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(IID_IITWordWheel, 
0x8fa0d5a4, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

#ifdef ITPROXY

// {D73725C2-8C12-11d0-A84E-00AA006C7D01}
DEFINE_GUID(CLSID_IITWordWheel, 
0xd73725c2, 0x8c12, 0x11d0, 0xa8, 0x4e, 0x0, 0xaa, 0x0, 0x6c, 0x7d, 0x1);

#else

// {4662daa8-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_IITWordWheelLocal, 
0x4662daa8, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

#endif	// ITPROXY

// Word-wheel open flags
#define ITWW_OPEN_CONNECT	0x00000000    // connect to server on open (the default)
#define ITWW_OPEN_NOCONNECT	0x00000001    // don't connect to server on open

// Constants for IITWordWheel::Lookup.
#define ITWW_CBKEY_MAX		1024		// Max size of keys allowed in Word Wheels.

// Forward declarations
interface IITDatabase;
interface IITResultSet;
interface IITGroup;
interface IITPropList;
interface IITQuery;

DECLARE_INTERFACE_(IITWordWheel, IUnknown)
{

	STDMETHOD(Open)(IITDatabase* lpITDB, LPCWSTR lpszMoniker, DWORD dwFlags=0) PURE;
	STDMETHOD(Close)(void) PURE;

	// Returns the code page ID and locale ID that the word wheel was built and
	// sorted with.
	STDMETHOD(GetLocaleInfo)(DWORD *pdwCodePageID, LCID *plcid) PURE;

	// Returns in *pdwObjInstance the ID of the external sort instance being used by
	// this word wheel.  The instance ID can be passed to IITDatabase::GetObject to
	// to obtain an interface pointer on the instantiated instance.  If the word
	// wheel doesn't use external sorting, then IITDB_OBJINST_NULL.
	STDMETHOD(GetSorterInstance)(DWORD *pdwObjInstance) PURE;

	STDMETHOD(Count)(LONG *pcEntries) PURE;

	// To be safe, the length of lpvKeyBuf should always be at least ITWW_CBKEY_MAX. 
	STDMETHOD(Lookup)(LONG lEntry, LPVOID lpvKeyBuf, DWORD cbKeyBuf) PURE;
	STDMETHOD(Lookup)(LONG lEntry, IITResultSet* lpITResult, LONG cEntries) PURE;
	STDMETHOD(Lookup)(LPCVOID lpcvPrefix, BOOL fExactMatch, LONG *plEntry) PURE;

	STDMETHOD(SetGroup)(IITGroup* piitGroup) PURE;
	STDMETHOD(GetGroup)(IITGroup** ppiitGroup) PURE;

	STDMETHOD(GetDataCount)(LONG lEntry, DWORD *pdwCount) PURE;
	STDMETHOD(GetData)(LONG lEntry, IITResultSet* lpITResult) PURE;
	STDMETHOD(GetDataColumns)(IITResultSet* pRS) PURE;
};

typedef IITWordWheel* LPITWORDWHEEL;


// IWordBreaker, IWordSink, IPhraseSink, IStem

#ifndef __IStemSink_FWD_DEFINED__
#define __IStemSink_FWD_DEFINED__
typedef interface IStemSink IStemSink;
#endif 	/* __IStemSink_FWD_DEFINED__ */


#ifndef __IStemmer_FWD_DEFINED__
#define __IStemmer_FWD_DEFINED__
typedef interface IStemmer IStemmer;
#endif 	/* __IStemmer_FWD_DEFINED__ */


#ifndef __IStemmerConfig_FWD_DEFINED__
#define __IStemmerConfig_FWD_DEFINED__
typedef interface IStemmerConfig IStemmerConfig;
#endif 	/* __IStemmerConfig_FWD_DEFINED__ */


DECLARE_INTERFACE_(IStemmer, IUnknown)
{
    STDMETHOD(Init)(ULONG ulMaxTokenSize, BOOL *pfLicense) PURE;
    STDMETHOD(GetLicenseToUse)(WCHAR const **ppwcsLicense) PURE;
    STDMETHOD(StemWord)(WCHAR const *pwcInBuf, ULONG cwc,
									IStemSink *pStemSink) PURE;   
};

typedef IStemmer *PISTEM;


DECLARE_INTERFACE_(IStemSink, IUnknown)
{
    STDMETHOD(PutAltWord)(WCHAR const *pwcInBuf, ULONG cwc) PURE;
    STDMETHOD(PutWord)(WCHAR const *pwcInBuf, ULONG cwc) PURE;
};

typedef IStemSink *PISTEMSNK;


DECLARE_INTERFACE_(IStemmerConfig, IUnknown)
{
	// Sets/gets locale info that will affect the stemming
	// behavior of IStemmer::StemWord.
	// Returns S_OK if locale described by params is supported
	// by the breaker object; E_INVALIDARG otherwise.
	STDMETHOD(SetLocaleInfo)(DWORD dwCodePageID, LCID lcid) PURE;
	STDMETHOD(GetLocaleInfo)(DWORD *pdwCodePageID, LCID *plcid) PURE;

	// Sets/gets info that controls certain aspects of stemming.
	// This method currently accepts only the following set of flags
	// in grfStemFlags:
	// In the future, additional information may be passed in through
	// dwReserved.
	STDMETHOD(SetControlInfo)(DWORD grfStemFlags, DWORD dwReserved) PURE;
	STDMETHOD(GetControlInfo)(DWORD *pgrfStemFlags, DWORD *pdwReserved) PURE;

	// Will load external stemmer data, such as word part lists, etc.
	// The format of the data in the stream is entirely
	// implementation-specific.
	STDMETHOD(LoadExternalStemmerData)(IStream *pStream,
                                           DWORD dwExtDataType) PURE;
};

typedef IStemmerConfig *PISTEMC;


#ifndef __IPhraseSink_FWD_DEFINED__
#define __IPhraseSink_FWD_DEFINED__
typedef interface IPhraseSink IPhraseSink;
#endif 	/* __IPhraseSink_FWD_DEFINED__ */


#ifndef __IWordSink_FWD_DEFINED__
#define __IWordSink_FWD_DEFINED__
typedef interface IWordSink IWordSink;
#endif 	/* __IWordSink_FWD_DEFINED__ */


#ifndef __IWordBreaker_FWD_DEFINED__
#define __IWordBreaker_FWD_DEFINED__
typedef interface IWordBreaker IWordBreaker;
#endif 	/* __IWordBreaker_FWD_DEFINED__ */


#ifndef __IWordBreakerConfig_FWD_DEFINED__
#define __IWordBreakerConfig_FWD_DEFINED__
typedef interface IWordBreakerConfig IWordBreakerConfig;
#endif 	/* __IWordBreakerConfig_FWD_DEFINED__ */


#ifndef __IITStopWordList_FWD_DEFINED__
#define __IITStopWordList_FWD_DEFINED__
typedef interface IITStopWordList IITStopWordList;
#endif 	/* __IITStopWordList_FWD_DEFINED__ */


// Supporting definitions for IWordBreaker.
typedef struct tagTEXT_SOURCE TEXT_SOURCE;
typedef SCODE (__stdcall *PFNFILLTEXTBUFFER)(TEXT_SOURCE *pTextSource);

typedef struct tagTEXT_SOURCE
{
    PFNFILLTEXTBUFFER pfnFillTextBuffer;
    WCHAR *awcBuffer;
    ULONG iEnd;
    ULONG iCur;
} TEXT_SOURCE;


DECLARE_INTERFACE_(IWordBreaker, IUnknown)
{
    STDMETHOD(Init)(BOOL fQuery, ULONG ulMaxTokenSize, BOOL *pfLicense) PURE;
    STDMETHOD(BreakText)(TEXT_SOURCE *pTextSource, IWordSink *pWordSink,
                         IPhraseSink *pPhraseSink) PURE;
    STDMETHOD(ComposePhrase)(WCHAR const *pwcNoun, ULONG cwcNoun,
                             WCHAR const *pwcModifier, ULONG cwcModifier,
                             ULONG ulAttachmentType, WCHAR *pwcPhrase,
                             ULONG *pcwcPhrase) PURE;
    STDMETHOD(GetLicenseToUse)(WCHAR const **ppwcsLicense) PURE;
};

typedef IWordBreaker *PIWBRK;


// Break word types that can be passed to
// IWordBreakerConfig::SetBreakWordType.
#define IITWBC_BREAKTYPE_TEXT		((DWORD) 0)
#define IITWBC_BREAKTYPE_NUMBER		((DWORD) 1)
#define IITWBC_BREAKTYPE_DATE		((DWORD) 2)
#define IITWBC_BREAKTYPE_TIME		((DWORD) 3)
#define IITWBC_BREAKTYPE_EPOCH		((DWORD) 4)


// Breaker control flags that can be passed to
// IWordBreakerConfig::SetControlInfo.
#define IITWBC_BREAK_ACCEPT_WILDCARDS	0x00000001  // Interpret wildcard chars as such.
#define IITWBC_BREAK_AND_STEM           0x00000002  // Stem words after breaking them.

// External data types that can be passed to
// IWordBreakerConfig::LoadExternalBreakerData.
#define IITWBC_EXTDATA_CHARTABLE		((DWORD) 0)		
#define IITWBC_EXTDATA_STOPWORDLIST		((DWORD) 1)


DECLARE_INTERFACE_(IWordBreakerConfig, IUnknown)
{
	// Sets/gets locale info that will affect the word breaking
	// behavior of IWordBreaker::BreakText.
	// Returns S_OK if locale described by params is supported
	// by the breaker object; E_INVALIDARG otherwise.
	STDMETHOD(SetLocaleInfo)(DWORD dwCodePageID, LCID lcid) PURE;
	STDMETHOD(GetLocaleInfo)(DWORD *pdwCodePageID, LCID *plcid) PURE;

	// Sets/gets the type of words the breaker should expect
	// to see in all subsequent calls to IWordBreaker::BreakText.
	// Returns S_OK if the type is understood by the breaker
	//  object; E_INVALIDARG otherwise.
	STDMETHOD(SetBreakWordType)(DWORD dwBreakWordType) PURE;
	STDMETHOD(GetBreakWordType)(DWORD *pdwBreakWordType) PURE;

	// Sets/gets info that controls certain aspects of word breaking.
	// This method currently accepts only the following set of flags
	// in grfBreakFlags:
	//		IITWBC_BREAK_ACCEPT_WILDCARDS
	//		IITWBC_BREAK_AND_STEM
	// In the future, additional information may be passed in through
	// dwReserved.
	STDMETHOD(SetControlInfo)(DWORD grfBreakFlags, DWORD dwReserved) PURE;
	STDMETHOD(GetControlInfo)(DWORD *pgrfBreakFlags, DWORD *pdwReserved) PURE;

	// Will load external breaker data, such as a table containing
	// char-by-char break information or a list of stop words.
	// Although the format of the data in the stream is entirely
	// implementation-specific, this interface does define a couple
	// of general types for that data which can be passed in
	// dwStreamDataType:
	//		IITWBC_EXTDATA_CHARTABLE
	//		IITWBC_EXTDATA_STOPWORDLIST
	STDMETHOD(LoadExternalBreakerData)(IStream *pStream,
                                           DWORD dwExtDataType) PURE;

	// These methods allow a stemmer to be associated with the breaker.  The
	// breaker will take responsibility for calling
	// IPersistStreamInit::Load/Save when it is loaded/saved if the stemmer
	// supports that interface.
	STDMETHOD(SetWordStemmer)(REFCLSID rclsid, IStemmer *pStemmer) PURE;
	STDMETHOD(GetWordStemmer)(IStemmer **ppStemmer) PURE;
};

//
// The InfoTech error codes
//
#define E_NOTEXIST          _HRESULT_TYPEDEF_(0x80001000L)
#define E_DUPLICATE         _HRESULT_TYPEDEF_(0x80001001L)
#define E_BADVERSION        _HRESULT_TYPEDEF_(0x80001002L)
#define E_BADFILE           _HRESULT_TYPEDEF_(0x80001003L)
#define E_BADFORMAT         _HRESULT_TYPEDEF_(0x80001004L)
#define E_NOPERMISSION      _HRESULT_TYPEDEF_(0x80001005L)
#define E_ASSERT            _HRESULT_TYPEDEF_(0x80001006L)
#define E_INTERRUPT         _HRESULT_TYPEDEF_(0x80001007L)
#define E_NOTSUPPORTED      _HRESULT_TYPEDEF_(0x80001008L)
#define E_OUTOFRANGE        _HRESULT_TYPEDEF_(0x80001009L)                  
#define E_GROUPIDTOOBIG     _HRESULT_TYPEDEF_(0x8000100AL)
#define E_TOOMANYTITLES     _HRESULT_TYPEDEF_(0x8000100BL)
#define E_NOMERGEDDATA      _HRESULT_TYPEDEF_(0x8000100CL)
#define E_NOTFOUND          _HRESULT_TYPEDEF_(0x8000100DL)
#define E_CANTFINDDLL       _HRESULT_TYPEDEF_(0x8000100EL)
#define E_NOHANDLE          _HRESULT_TYPEDEF_(0x8000100FL) 
#define E_GETLASTERROR      _HRESULT_TYPEDEF_(0x80001010L)
#define E_BADPARAM          _HRESULT_TYPEDEF_(0x80001011L)
#define E_INVALIDSTATE      _HRESULT_TYPEDEF_(0x80001012L)
#define E_NOTOPEN           _HRESULT_TYPEDEF_(0x80001013L)
#define E_ALREADYOPEN       _HRESULT_TYPEDEF_(0x80001013L)
#define E_UNKNOWN_TRANSPORT _HRESULT_TYPEDEF_(0x80001016L)
#define E_UNSUPPORTED_TRANSPORT _HRESULT_TYPEDEF_(0x80001017L)
#define E_BADFILTERSIZE     _HRESULT_TYPEDEF_(0x80001018L)
#define E_TOOMANYOBJECTS    _HRESULT_TYPEDEF_(0x80001019L)
#define E_NAMETOOLONG       _HRESULT_TYPEDEF_(0x80001020L)

#define E_FILECREATE        _HRESULT_TYPEDEF_(0x80001030L) 
#define E_FILECLOSE         _HRESULT_TYPEDEF_(0x80001031L)
#define E_FILEREAD          _HRESULT_TYPEDEF_(0x80001032L)
#define E_FILESEEK          _HRESULT_TYPEDEF_(0x80001033L)
#define E_FILEWRITE         _HRESULT_TYPEDEF_(0x80001034L)
#define E_FILEDELETE        _HRESULT_TYPEDEF_(0x80001035L)
#define E_FILEINVALID       _HRESULT_TYPEDEF_(0x80001036L)
#define E_FILENOTFOUND      _HRESULT_TYPEDEF_(0x80001037L)
#define E_DISKFULL          _HRESULT_TYPEDEF_(0x80001038L)

#define E_TOOMANYTOPICS     _HRESULT_TYPEDEF_(0x80001050L)
#define E_TOOMANYDUPS       _HRESULT_TYPEDEF_(0x80001051L)
#define E_TREETOOBIG        _HRESULT_TYPEDEF_(0x80001052L)
#define E_BADBREAKER        _HRESULT_TYPEDEF_(0x80001053L)
#define E_BADVALUE          _HRESULT_TYPEDEF_(0x80001054L)
#define E_ALL_WILD          _HRESULT_TYPEDEF_(0x80001055L)
#define E_TOODEEP           _HRESULT_TYPEDEF_(0x80001056L)
#define E_EXPECTEDTERM      _HRESULT_TYPEDEF_(0x80001057L)
#define E_MISSLPAREN        _HRESULT_TYPEDEF_(0x80001058L)
#define E_MISSRPAREN        _HRESULT_TYPEDEF_(0x80001059L)
#define E_MISSQUOTE         _HRESULT_TYPEDEF_(0x8000105AL)
#define E_NULLQUERY         _HRESULT_TYPEDEF_(0x8000105BL)
#define E_STOPWORD          _HRESULT_TYPEDEF_(0x8000105CL)
#define E_BADRANGEOP        _HRESULT_TYPEDEF_(0x8000105DL)
#define E_UNMATCHEDTYPE     _HRESULT_TYPEDEF_(0x8000105EL)
#define E_WORDTOOLONG       _HRESULT_TYPEDEF_(0x8000105FL)
#define E_BADINDEXFLAGS     _HRESULT_TYPEDEF_(0x80001060L)
#define E_WILD_IN_DTYPE     _HRESULT_TYPEDEF_(0x80001061L)   
#define E_NOSTEMMER         _HRESULT_TYPEDEF_(0x80001062L)

// Property list and result set errors
#define E_MISSINGPROP       _HRESULT_TYPEDEF_(0x80001080L)
#define E_PROPLISTNOTEMPTY  _HRESULT_TYPEDEF_(0x80001081L)
#define E_PROPLISTEMPTY     _HRESULT_TYPEDEF_(0x80001082L)
#define E_ALREADYINIT       _HRESULT_TYPEDEF_(0x80001083L)
#define E_NOTINIT           _HRESULT_TYPEDEF_(0x80001084L)
#define E_RESULTSETEMPTY    _HRESULT_TYPEDEF_(0x80001085L)
#define E_TOOMANYCOLUMNS    _HRESULT_TYPEDEF_(0x80001086L)
#define E_NOKEYPROP         _HRESULT_TYPEDEF_(0x80001087L)



// ITResultSet interface declaration

// {3BB91D41-998B-11d0-A850-00AA006C7D01}
DEFINE_GUID(IID_IITResultSet, 
0x3bb91d41, 0x998b, 0x11d0, 0xa8, 0x50, 0x0, 0xaa, 0x0, 0x6c, 0x7d, 0x1);

// {4662daa7-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_IITResultSet, 
0x4662daa7, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// maximum number of columns in a row set
#define MAX_COLUMNS  256

// Column priorities
typedef enum
{
    PRIORITY_LOW = 0,
    PRIORITY_NORMAL = 1,
    PRIORITY_HIGH = 2,

} PRIORITY;


// Forward declarations
class CProperty;

typedef struct tagROWSTATUS
{
    LONG lRowFirst;
    LONG cRows;
    LONG cProperties;
    LONG cRowsTotal;

} ROWSTATUS, *LPROWSTATUS;


typedef struct tagCOLUMNSTATUS
{
    LONG cPropCount;
    LONG cPropsLoaded;

} COLUMNSTATUS, *LPCOLUMNSTATUS;


// Used by IITResultSet::SetColumnHeap.
typedef SCODE (__stdcall *PFNCOLHEAPFREE)(LPVOID);


DECLARE_INTERFACE_(IITResultSet, IUnknown)
{
    // Initialization
    STDMETHOD(SetColumnPriority)(LONG lColumnIndex, PRIORITY ColumnPriority) PURE;
    STDMETHOD(SetColumnHeap)(LONG lColumnIndex, LPVOID lpvHeap,
                             PFNCOLHEAPFREE pfnColHeapFree) PURE;
    STDMETHOD(SetKeyProp)(PROPID PropID) PURE;
    STDMETHOD(Add)(LPVOID lpvHdr) PURE;
    STDMETHOD(Add)(PROPID PropID, LPVOID lpvDefaultData, DWORD cbData, PRIORITY Priority) PURE;
    STDMETHOD(Add)(PROPID PropID, LPCWSTR lpszwDefault, PRIORITY Priority) PURE;
    STDMETHOD(Add)(PROPID PropID, DWORD dwDefaultData, PRIORITY Priority) PURE;

    // Build result set
    STDMETHOD(Append)(LPVOID lpvHdr, LPVOID lpvData) PURE;
    STDMETHOD(Set)(LONG lRowIndex, LPVOID lpvHdr, LPVOID lpvData) PURE;
    STDMETHOD(Set)(LONG lRowIndex, LONG lColumnIndex, DWORD_PTR dwData) PURE;
    STDMETHOD(Set)(LONG lRowIndex, LONG lColumnIndex, LPCWSTR lpwStr) PURE;
    STDMETHOD(Set)(LONG lRowIndex, LONG lColumnIndex, LPVOID lpvData, DWORD cbData) PURE;
    STDMETHOD(Copy)(IITResultSet* pRSCopy) PURE;
    STDMETHOD(AppendRows)(IITResultSet* pResSrc, LONG lRowSrcFirst, LONG cSrcRows, 
                          LONG& lRowFirstDest) PURE;

    // Obtain info about result set
    STDMETHOD(Get)(LONG lRowIndex, LONG lColumnIndex, CProperty& Prop) PURE;
    STDMETHOD(GetKeyProp)(PROPID& KeyPropID) PURE;
    STDMETHOD(GetColumnPriority)(LONG lColumnIndex, PRIORITY& ColumnPriority) PURE;
    STDMETHOD(GetRowCount)(LONG& lNumberOfRows) PURE;
    STDMETHOD(GetColumnCount)(LONG& lNumberOfColumns) PURE;
    STDMETHOD(GetColumn)(LONG lColumnIndex, PROPID& PropID) PURE;
    STDMETHOD(GetColumn)(LONG lColumnIndex, PROPID& PropID, DWORD& dwType, LPVOID& lpvDefaultValue,
                         DWORD& cbSize, PRIORITY& ColumnPriority) PURE;

    STDMETHOD(GetColumnFromPropID)(PROPID PropID, LONG& lColumnIndex) PURE;

    // Clear result set
    STDMETHOD(Clear)() PURE;
    STDMETHOD(ClearRows)() PURE;
    STDMETHOD(Free)() PURE;

    // Asynchronous support
    STDMETHOD(IsCompleted)() PURE;      // returns S_OK or S_FALSE
    STDMETHOD(Cancel)() PURE;
    STDMETHOD(Pause)(BOOL fPause) PURE;

    STDMETHOD(GetRowStatus)(LONG lRowFirst, LONG cRows, LPROWSTATUS lpRowStatus) PURE;
    STDMETHOD(GetColumnStatus)(LPCOLUMNSTATUS lpColStatus) PURE;
};

typedef IITResultSet* LPITRS;


//----------------------------------------------------------------------
//------			Word Breaking Definitions				------------
//----------------------------------------------------------------------

// {D53552C8-77E3-101A-B552-08002B33B0E6}
DEFINE_GUID(IID_IWordBreaker, 
0xD53552C8, 0x77E3, 0x101A, 0xB5, 0x52, 0x08, 0x00, 0x2B, 0x33, 0xB0, 0xE6);

// {CC907054-C058-101A-B554-08002B33B0E6}
DEFINE_GUID(IID_IWordSink, 
0xCC907054, 0xC058, 0x101A, 0xB5, 0x54, 0x08, 0x00, 0x2B, 0x33, 0xB0, 0xE6);

// {CC906FF0-C058-101A-B554-08002B33B0E6}
DEFINE_GUID(IID_IPhraseSink, 
0xCC906FF0, 0xC058, 0x101A, 0xB5, 0x54, 0x08, 0x00, 0x2B, 0x33, 0xB0, 0xE6);

// {8fa0d5a6-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(IID_IWordBreakerConfig, 
0x8fa0d5a6, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// {4662daaf-d393-11d0-9a56-00c04fb68bf7}
DEFINE_GUID(CLSID_ITStdBreaker, 
0x4662daaf, 0xd393, 0x11d0, 0x9a, 0x56, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);


//----------------------------------------------------------------------
//------			Stop Word List Definitions				------------
//----------------------------------------------------------------------

// {8fa0d5ad-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(IID_IITStopWordList, 
0x8fa0d5ad, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);


//----------------------------------------------------------------------
//------				Stemming Definitions				------------
//----------------------------------------------------------------------

// {efbaf140-7f42-11ce-be57-00aa0051fe20}
DEFINE_GUID(IID_IStemmer, 
0xefbaf140, 0x7f42, 0x11ce, 0xbe, 0x57, 0x00, 0xaa, 0x00, 0x51, 0xfe, 0x20);

// {fe77c330-7f42-11ce-be57-00aa0051fe20}
DEFINE_GUID(IID_IStemSink, 
0xfe77c330, 0x7f42, 0x11ce, 0xbe, 0x57, 0x00, 0xaa, 0x00, 0x51, 0xfe, 0x20);

// {8fa0d5a7-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(IID_IStemmerConfig, 
0x8fa0d5a7, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);

// {8fa0d5a8-dedf-11d0-9a61-00c04fb68bf7}
DEFINE_GUID(CLSID_ITEngStemmer, 
0x8fa0d5a8, 0xdedf, 0x11d0, 0x9a, 0x61, 0x00, 0xc0, 0x4f, 0xb6, 0x8b, 0xf7);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // __INFOTECH_H__
