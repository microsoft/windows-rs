//
//    Copyright (C) Microsoft.  All rights reserved.
//
//**************************************************************************************************
//
//  Purpose: Defines classes to support parsing tokens from a xml file

#ifndef _PARSER_H
#define _PARSER_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <stdio.h>

#undef CLASS_IMPORT_EXPORT
#if defined(HHCTRL) || defined(_M_CEE_PURE) // define this only when building the HHCtrl DLL
  #define CLASS_IMPORT_EXPORT /**/
#else
 #ifdef HHSETUP // define this only when building the HHSetup DLL
  #define CLASS_IMPORT_EXPORT __declspec( dllexport )
 #else
  #define CLASS_IMPORT_EXPORT __declspec( dllimport )
 #endif
#endif

#ifdef _M_CEE_PURE
#define PARSER_API_INLINE __clrcall
#else
#define PARSER_API_INLINE
#endif

#define MAX_LINE_LEN 1024

#define F_OK 0
#define F_NOFILE 1
#define F_READ 2
#define F_WRITE 3
#define F_MEMORY 4
#define F_EOF 5
#define F_END 6
#define F_TAGMISSMATCH 7
#define F_MISSINGENDTAG 8
#define F_NOTFOUND 9
#define F_NOPARENT 10
#define F_NULL 11
#define F_NOTITLE 12
#define F_LOCATION 13
#define F_REFERENCED 14
#define F_DUPLICATE 15
#define F_DELETE 16
#define F_CLOSE 17
#define F_EXISTCHECK 19

class CParseXML {
private: // data

	CHAR m_cCurToken[MAX_LINE_LEN];
	CHAR m_cCurWord[MAX_LINE_LEN];
	CHAR m_cCurBuffer[MAX_LINE_LEN];
	FILE *m_fh;
	CHAR * m_pCurrentIndex;
	DWORD m_dwError;

private: // functions
	DWORD Read();
	DWORD SetError(DWORD dw) { m_dwError = dw; return m_dwError; }
public:

	CParseXML() {
		m_fh = NULL;
		m_cCurBuffer[0] = '\0';
		m_pCurrentIndex = NULL;
		m_dwError = F_OK;
	}

	~CParseXML() {
		End();
	}

	CHAR * GetFirstWord(CHAR *);
	CHAR * GetValue(CHAR *);

	DWORD Start(const CHAR *szFile);
	void End();
	CHAR *GetToken();
	DWORD GetError() { return m_dwError; }
};

// class to support a FIFO queue of strings
typedef struct  fifo {
	CHAR *string;
	fifo *prev;
} FIFO;

class CLASS_IMPORT_EXPORT  CFIFOString {
private:

	FIFO *m_fifoTail;

public:

	CFIFOString() { m_fifoTail = NULL; }
	~CFIFOString();
	void RemoveAll();

	DWORD AddTail(CHAR *sz);
	DWORD GetTail(_Outptr_ PZPSTR sz);
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
