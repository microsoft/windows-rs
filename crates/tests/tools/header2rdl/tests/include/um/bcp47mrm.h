// Copyright (C) Microsoft Corporation. All rights reserved.
#pragma once

#ifndef _BCP47MRM_H_
#define _BCP47MRM_H_

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
STDAPI GetDistanceOfClosestLanguageInList(
    _In_ PCWSTR pszLanguage,
    _In_ PCWSTR pszLanguagesList,
    _In_ wchar_t wchListDelimiter,
    _Out_ double * pClosestDistance);
 
STDAPI_(bool) IsWellFormedTag(_In_ PCWSTR pszTag);
#endif

#endif
