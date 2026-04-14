/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:		icrsint.h
//
// Contents:	ADO C/C++ Record Binding Definitions
//
// Comments:
//
//-----------------------------------------------------------------------------

#ifndef _ICRSINT_H_
#define _ICRSINT_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <olectl.h>
#include <stddef.h>

// forwards
class CADORecordBinding;

#define classoffset(base, derived) ((DWORD_PTR)(static_cast<base*>((derived*)8))-8)

enum ADOFieldStatusEnum
{	
	adFldOK = 0,
	adFldBadAccessor = 1,
	adFldCantConvertValue = 2,
	adFldNull = 3,
	adFldTruncated = 4,
	adFldSignMismatch = 5,
	adFldDataOverFlow = 6,
	adFldCantCreate = 7,
	adFldUnavailable = 8,
	adFldPermissionDenied = 9,
	adFldIntegrityViolation = 10,
	adFldSchemaViolation = 11,
	adFldBadStatus = 12,
	adFldDefault = 13
};

typedef struct stADO_BINDING_ENTRY
{
	ULONG_PTR		ulOrdinal;
    WORD			wDataType;
	BYTE			bPrecision;
	BYTE			bScale;
	ULONG_PTR		ulSize;
	ULONG_PTR		ulBufferOffset;
	ULONG_PTR		ulStatusOffset;
	ULONG_PTR		ulLengthOffset;
	ULONG_PTR		ulADORecordBindingOffSet;
	BOOL			fModify;
} ADO_BINDING_ENTRY;

#define BEGIN_ADO_BINDING(cls) public: \
	typedef cls ADORowClass; \
	const ADO_BINDING_ENTRY* STDMETHODCALLTYPE GetADOBindingEntries() { \
	static const ADO_BINDING_ENTRY rgADOBindingEntries[] = { 

//
// Fixed length non-numeric data
//
#define ADO_FIXED_LENGTH_ENTRY(Ordinal, DataType, Buffer, Status, Modify)\
	{Ordinal, \
	DataType, \
	0, \
	0, \
	0, \
	offsetof(ADORowClass, Buffer), \
	offsetof(ADORowClass, Status), \
	0, \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

#define ADO_FIXED_LENGTH_ENTRY2(Ordinal, DataType, Buffer, Modify)\
	{Ordinal, \
	DataType, \
	0, \
	0, \
	0, \
	offsetof(ADORowClass, Buffer), \
	0, \
	0, \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

//
// Numeric data
// 
#define ADO_NUMERIC_ENTRY(Ordinal, DataType, Buffer, Precision, Scale, Status, Modify)\
	{Ordinal, \
	DataType, \
	Precision, \
	Scale, \
	0, \
	offsetof(ADORowClass, Buffer), \
	offsetof(ADORowClass, Status), \
	0, \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

#define ADO_NUMERIC_ENTRY2(Ordinal, DataType, Buffer, Precision, Scale, Modify)\
	{Ordinal, \
	DataType, \
	Precision, \
	Scale, \
	0, \
	offsetof(ADORowClass, Buffer), \
	0, \
	0, \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

//
// Variable length data
//
#define ADO_VARIABLE_LENGTH_ENTRY(Ordinal, DataType, Buffer, Size, Status, Length, Modify)\
	{Ordinal, \
	DataType, \
	0, \
	0, \
	Size, \
	offsetof(ADORowClass, Buffer), \
	offsetof(ADORowClass, Status), \
	offsetof(ADORowClass, Length), \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

#define ADO_VARIABLE_LENGTH_ENTRY2(Ordinal, DataType, Buffer, Size, Status, Modify)\
	{Ordinal, \
	DataType, \
	0, \
	0, \
	Size, \
	offsetof(ADORowClass, Buffer), \
	offsetof(ADORowClass, Status), \
	0, \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

#define ADO_VARIABLE_LENGTH_ENTRY3(Ordinal, DataType, Buffer, Size, Length, Modify)\
	{Ordinal, \
	DataType, \
	0, \
	0, \
	Size, \
	offsetof(ADORowClass, Buffer), \
	0, \
	offsetof(ADORowClass, Length), \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

#define ADO_VARIABLE_LENGTH_ENTRY4(Ordinal, DataType, Buffer, Size, Modify)\
	{Ordinal, \
	DataType, \
	0, \
	0, \
	Size, \
	offsetof(ADORowClass, Buffer), \
	0, \
	0, \
	classoffset(CADORecordBinding, ADORowClass), \
	Modify},

#define END_ADO_BINDING()   {0, adEmpty, 0, 0, 0, 0, 0, 0, 0, FALSE}};\
	return rgADOBindingEntries;}

//
// Interface that the client 'record' class needs to support. The ADO Binding entries
// provide the implementation for this interface.
//
class CADORecordBinding
{
public:
	STDMETHOD_(const ADO_BINDING_ENTRY*, GetADOBindingEntries) (VOID) PURE;
};

//
// Interface that allows a client to fetch a record of data into class data members.
//
struct __declspec(uuid("00000544-0000-0010-8000-00aa006d2ea4")) IADORecordBinding;
DECLARE_INTERFACE_(IADORecordBinding, IUnknown)
{
public:
	STDMETHOD(BindToRecordset) (CADORecordBinding *pAdoRecordBinding) PURE;
	STDMETHOD(AddNew) (CADORecordBinding *pAdoRecordBinding) PURE;
	STDMETHOD(Update) (CADORecordBinding *pAdoRecordBinding) PURE;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_ICRSINT_H_
