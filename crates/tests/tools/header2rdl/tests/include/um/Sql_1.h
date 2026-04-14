/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    SQL_1.H

Abstract:

    Level 1 Syntax SQL Parser

History:

--*/

#ifndef _SQL_1_H_
#define _SQL_1_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


struct SQL_LEVEL_1_TOKEN
{
    enum { OP_EXPRESSION = 1, TOKEN_AND, TOKEN_OR, TOKEN_NOT };
    enum { IFUNC_NONE = 0, IFUNC_UPPER = 1, IFUNC_LOWER = 2 };

    int nTokenType; //  OP_EXPRESSION,TOKEN_AND, TOKEN_OR, TOKEN_NOT
    

    // If the field is a OP_EXPRESSION, then the following are used.
    enum { OP_EQUAL = 1, OP_NOT_EQUAL, OP_EQUALorGREATERTHAN,
		       OP_EQUALorLESSTHAN, OP_LESSTHAN, OP_GREATERTHAN, OP_LIKE };
    
    BSTR    pPropertyName;		// Name of the property on which the operator is applied
    int     nOperator;			// Operator that is applied on property
    BOOL	bConstIsStrNumeric;	// True if the vConstValue is a BSTR and is a UINT32 or any 64bit number
	VARIANT vConstValue;		// Value applied by operator
    BSTR    pPropName2;         // Property compared to.

    DWORD   dwPropertyFunction; // 0=no instrinsic function applied
    DWORD   dwConstFunction;    // "
    
    SQL_LEVEL_1_TOKEN();
    SQL_LEVEL_1_TOKEN(SQL_LEVEL_1_TOKEN&);
   ~SQL_LEVEL_1_TOKEN(); 
    SQL_LEVEL_1_TOKEN& operator=(SQL_LEVEL_1_TOKEN &Src);
    
    void Dump(FILE *);
};


// Contains RPN version of expression.
// ===================================

struct SQL_LEVEL_1_RPN_EXPRESSION
{
    int nNumTokens;
    int nCurSize;
    SQL_LEVEL_1_TOKEN *pArrayOfTokens;
    BSTR bsClassName;

	int nNumberOfProperties;          // Zero means all properties selected
    int nCurPropSize;
	_Field_size_part_(nCurPropSize, nNumberOfProperties) BSTR *pbsRequestedPropertyNames;  // Array of property names which values are to be returned if
    
    SQL_LEVEL_1_RPN_EXPRESSION();
   ~SQL_LEVEL_1_RPN_EXPRESSION();
   
   //Note: this method deletes the token it is passed as an argument
    void AddToken(SQL_LEVEL_1_TOKEN *pTok);
    void AddToken(SQL_LEVEL_1_TOKEN &pTok);
    void AddProperty(_In_ LPWSTR pProp);
    void Dump(const char *pszTextFile);
};


class SQL1_Parser
{
    CGenLexer *m_pLexer;
    int        m_nLine;
    wchar_t*   m_pTokenText;
    int        m_nCurrentToken;
    SQL_LEVEL_1_RPN_EXPRESSION* m_pExpression;

	//Cleanup used by d'tor and SetSource
	void Cleanup();

	//Init used by c'tor and SetSource
	void Init(_In_ CGenLexSource *pSrc);

    // Semantic transfer variables.
    // ============================
    VARIANT    m_vTypedConst;
    int        m_nRelOp;
    DWORD      m_dwConstFunction;
    DWORD      m_dwPropFunction;
    LPWSTR     m_pIdent;
    LPWSTR     m_pPropComp;
	BOOL       m_bConstIsStrNumeric;
        
    // Parsing functions.
    // ==================
    BOOL Next();
    
    int parse();

    int prop_list();
    int class_name();
    int opt_where();
    int expr();
    int property_name();
    int prop_list_2();
    int term();
    int expr2();
    int simple_expr();
    int term2();
    int leading_ident_expr();
    int finalize();
    int rel_operator();
    int equiv_operator();
    int comp_operator();
    int is_operator();
    int trailing_prop_expr();
    int trailing_prop_expr2();
    int trailing_or_null();
    int trailing_const_expr();
    int unknown_func_expr();
    int typed_constant();

public:
    enum { 
        SUCCESS,
        SYNTAX_ERROR,
        LEXICAL_ERROR,
        FAILED,
        BUFFER_TOO_SMALL
        };

    SQL1_Parser(_In_ CGenLexSource *pSrc);
   ~SQL1_Parser();

    int GetQueryClass(_Out_writes_opt_(nBufLen) LPWSTR pDestBuf, int nBufLen);
       
    int Parse(_Outptr_ SQL_LEVEL_1_RPN_EXPRESSION **pOutput);
        // use operator delete on pOutput
            
    int CurrentLine() { return m_nLine; }
    LPWSTR CurrentToken() { return m_pTokenText; }
	void SetSource(CGenLexSource *pSrc);
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif


