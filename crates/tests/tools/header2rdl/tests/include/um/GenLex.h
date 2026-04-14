/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    GENLEX.H

Abstract:

    Generic lexer framework classes.

History:

--*/

#ifndef _GENLEX_H_
#define _GENLEX_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <Polarity.h>

class CGenLexSource
{
public:
    virtual wchar_t NextChar() = 0;
        // Return 0 on end-of-input
    virtual void Pushback(wchar_t) = 0;
    virtual void Reset() = 0;
};

class CTextLexSource : public CGenLexSource
{
    const wchar_t *m_pSrcBuf;
    const wchar_t *m_pStart;

public:
    CTextLexSource(const wchar_t *pSrc) { SetString(pSrc); }
        // Binds directly to <pSrc> buffer, but doesn't delete it.

    wchar_t NextChar()
    {
        if (!m_pSrcBuf)
            return 0;
        else
            return *m_pSrcBuf++ ? m_pSrcBuf[-1] : 0;
    }

    void Pushback(wchar_t)
    {
        if (m_pSrcBuf)
            --m_pSrcBuf;
    }

    void Reset() { m_pSrcBuf = m_pStart; }
    void SetString (const wchar_t *pSrc) { m_pSrcBuf = m_pStart = pSrc; }
};


#pragma pack(2)
struct LexEl
{
    wchar_t cFirst, cLast;
    WORD wGotoState;
    WORD wReturnTok;
    WORD wInstructions;
};
#pragma pack()


// Lexer driver instructions

#define GLEX_ACCEPT      0x1            // Add the char to the token
#define GLEX_CONSUME     0x2            // Consume the char without adding to token
#define GLEX_PUSHBACK    0x4            // Place the char back in the source buffer for next token
#define GLEX_NOT         0x8            // A match occurs if the char is NOT the one specified
#define GLEX_LINEFEED    0x10               // Increase the source linecount
#define GLEX_RETURN      0x20               // Return the indicated token to caller
#define GLEX_ANY         wchar_t(0xFFFF)    // Any character
#define GLEX_EMPTY       wchar_t(0xFFFE)    // When subrange is not specified

class CGenLexer
{
    _Field_size_(m_nCurBufSize) wchar_t    *m_pTokenBuf;
    int         m_nCurrentLine;
    int         m_nCurBufSize;
    CGenLexSource   *m_pSrc;
    LexEl           *m_pTable;
    
public:
    CGenLexer(LexEl *pTbl, CGenLexSource *pSrc);
    
   ~CGenLexer(); 
    _Success_(return == 1) int NextToken();
        // Returns 0 on end of input.

    wchar_t* GetTokenText() { return m_pTokenBuf; }
    int GetLineNum() { return m_nCurrentLine; }
    void Reset();
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
