//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  FRQuery.h
//
//  Purpose: query support classes
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef _FRAMEWORK_QUERY_H_
#define _FRAMEWORK_QUERY_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <stdio.h>
#include <sql_1.h>
#include <comdef.h>
#include <vector>

class POLARITY CFrameworkQuery
{
public:
    CFrameworkQuery();
    ~CFrameworkQuery();

    // Finds out if a particular field was requested by the query in either
    // the Select statement, or the Where statement.  Only meaningful if we
    // are in ExecQueryAsync and the query has been sucessfully parsed.
    bool IsPropertyRequired(LPCWSTR propName);

    // Gets the class name from the query.  Only meaningful if we are
    // in ExecQueryAsync and the query has been sucessfully parsed.  It
    // is the responsibility of the caller to SysFreeString the returned
    // string.
    BSTR GetQueryClassName(void) { return SysAllocString(m_bstrtClassName); }

    // Given a property name, it will return all the values
    // that the query requests in a CHStringArray.
    // Select * from win32_directory where drive = "C:" GetValuesForProp(L"Drive") -> C:
    // Where Drive = "C:" or Drive = "D:" GetValuesForProp(L"Drive") -> C:, D:
    // Where Path = "\DOS" GetValuesForProp(L"Drive") -> (empty)
    // Where Drive <> "C:" GetValuesForProp(L"Drive") -> (empty)
    // Where Drive = "C:" or (Drive = "D:" and Mounted = true) GetValuesForProp(L"Drive") -> C:, D:
    HRESULT GetValuesForProp(LPCWSTR wszPropName, CHStringArray& achNames);

    // Here's an overloaded version in case client wants to pass in a vector of _bstr_t's
    HRESULT GetValuesForProp(LPCWSTR wszPropName, std::vector<_bstr_t>& vectorNames);

    // Returns a list of all the properties specified in the Select clause, plus.
    // all the the properties from the Where clauses.  If the returned array is empty, all
    // properties are required.
    void GetRequiredProperties(CHStringArray &saProperties);

    // Boolean indicating if all properties are being requested.
    bool AllPropertiesAreRequired(void) { return (m_csaPropertiesRequired.GetSize() == 0); }

    // Boolean indicating if only the key properties are required.
    bool KeysOnly(void) { return m_bKeysOnly; }

    // Accessor function to retrieve wql query
    const CHString &GetQuery() ;

    // Moves the values into the member variables.  Should never be called by users.
    HRESULT Init(
        
        const BSTR bstrQueryFormat, 
        const BSTR bstrQuery, 
        long lFlags,
        CHString &sNamespace
    );

    // Moves the values into the member variables.  Should never be called by users.
    HRESULT Init(

        ParsedObjectPath *pParsedObjectPath, 
        IWbemContext *pCtx, 
        LPCWSTR lpwszClassName,
        CHString &sNamespace
    );

    // Initializes the KeysOnly data member.  Should never be called by users.
    void Init2(IWbemClassObject *IClass);


protected:

    /*****************************************************************************/
    /* The rest of these data members and functions are intended for Microsoft   */
    /* internal use only. Use by third parties is unsupported and unrecommended. */
    /*****************************************************************************/

    SQL_LEVEL_1_RPN_EXPRESSION *m_pLevel1RPNExpression;
    CHStringArray m_csaPropertiesRequired;
    enum QueryTypes{eUnknown, eWQLCommand, eContextObject} m_QueryType;

    DWORD IsInList(const CHStringArray &csaArray, LPCWSTR pwszValue);

    BOOL IsReference(LPCWSTR lpwszPropertyName);
    const CHString &GetNamespace();

private:

    CHString m_sNamespace;
    long m_lFlags;
    IWbemClassObject *m_IClass;
    CHString m_sQueryFormat;

    void Reset(void);
    bool m_bKeysOnly;
    bool m_AddKeys;
    CHString m_sQuery;
    bstr_t m_bstrtClassName;

};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
