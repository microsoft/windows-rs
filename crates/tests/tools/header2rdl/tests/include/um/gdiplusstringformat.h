/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusStringFormat.h
*
* Abstract:
*
*   GDI+ StringFormat class
*
\**************************************************************************/

#ifndef _GDIPLUSSTRINGFORMAT_H
#define _GDIPLUSSTRINGFORMAT_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

class StringFormat : public GdiplusBase
{
public:
    friend class Graphics;
    friend class GraphicsPath;


    StringFormat(
        IN INT     formatFlags = 0,
        IN LANGID  language = LANG_NEUTRAL
    )
    {
        nativeFormat = NULL;
        lastError = DllExports::GdipCreateStringFormat(
            formatFlags,
            language,
            &nativeFormat
        );
    }

    static const StringFormat *GenericDefault();
    static const StringFormat *GenericTypographic();

    StringFormat(
        IN const StringFormat *format
    )
    {
        nativeFormat = NULL;
        lastError = DllExports::GdipCloneStringFormat(
            format ? format->nativeFormat : NULL,
            &nativeFormat
        );
    }

    StringFormat *Clone() const
    {
        GpStringFormat *clonedStringFormat = NULL;

        lastError = DllExports::GdipCloneStringFormat(
            nativeFormat,
            &clonedStringFormat
        );

        if (lastError == Ok)
            return new StringFormat(clonedStringFormat, lastError);
        else
            return NULL;
    }

    ~StringFormat()
    {
        DllExports::GdipDeleteStringFormat(nativeFormat);
    }

    Status SetFormatFlags(IN INT flags)
    {
        return SetStatus(DllExports::GdipSetStringFormatFlags(
            nativeFormat,
            flags
        ));
    }

    INT GetFormatFlags() const
    {
        INT flags;
        SetStatus(DllExports::GdipGetStringFormatFlags(nativeFormat, &flags));
        return flags;
    }

    Status SetAlignment(IN StringAlignment align)
    {
        return SetStatus(DllExports::GdipSetStringFormatAlign(
            nativeFormat,
            align
        ));
    }

    StringAlignment GetAlignment() const
    {
        StringAlignment alignment;
        SetStatus(DllExports::GdipGetStringFormatAlign(
            nativeFormat,
            &alignment
        ));
        return alignment;
    }

    Status SetLineAlignment(IN StringAlignment align)
    {
        return SetStatus(DllExports::GdipSetStringFormatLineAlign(
            nativeFormat,
            align
        ));
    }

    StringAlignment GetLineAlignment() const
    {
        StringAlignment alignment;
        SetStatus(DllExports::GdipGetStringFormatLineAlign(
            nativeFormat,
            &alignment
        ));
        return alignment;
    }

    Status SetHotkeyPrefix(IN HotkeyPrefix hotkeyPrefix)
    {
        return SetStatus(DllExports::GdipSetStringFormatHotkeyPrefix(
            nativeFormat,
            (INT)hotkeyPrefix
        ));
    }

    HotkeyPrefix GetHotkeyPrefix() const
    {
        HotkeyPrefix hotkeyPrefix;
        SetStatus(DllExports::GdipGetStringFormatHotkeyPrefix(
            nativeFormat,
            (INT*)&hotkeyPrefix
        ));
        return hotkeyPrefix;
    }

    Status SetTabStops(
        IN REAL    firstTabOffset,
        IN INT     count,
        IN const REAL    *tabStops
    )
    {
        return SetStatus(DllExports::GdipSetStringFormatTabStops(
            nativeFormat,
            firstTabOffset,
            count,
            tabStops
        ));
    }

    INT GetTabStopCount() const
    {
        INT count;
        SetStatus(DllExports::GdipGetStringFormatTabStopCount(nativeFormat, &count));
        return count;
    }

    Status GetTabStops(
        IN INT     count,
        OUT REAL   *firstTabOffset,
        OUT REAL   *tabStops
    ) const
    {
        return SetStatus(DllExports::GdipGetStringFormatTabStops(
            nativeFormat,
            count,
            firstTabOffset,
            tabStops
        ));
    }

    Status SetDigitSubstitution(
        IN LANGID                language,
        IN StringDigitSubstitute substitute
    )
    {
        return SetStatus(DllExports::GdipSetStringFormatDigitSubstitution(
            nativeFormat,
            language,
            substitute
        ));
    }

    LANGID GetDigitSubstitutionLanguage(
    ) const
    {
        LANGID language;
        SetStatus(DllExports::GdipGetStringFormatDigitSubstitution(
            nativeFormat,
            &language,
            NULL
        ));
        return language;
    }

    StringDigitSubstitute GetDigitSubstitutionMethod(
    ) const
    {
        StringDigitSubstitute substitute;
        SetStatus(DllExports::GdipGetStringFormatDigitSubstitution(
            nativeFormat,
            NULL,
            &substitute
        ));
        return substitute;
    }

    Status SetTrimming(IN StringTrimming trimming)
    {
        return SetStatus(DllExports::GdipSetStringFormatTrimming(
            nativeFormat,
            trimming
        ));
    }

    StringTrimming GetTrimming() const
    {
        StringTrimming trimming;
        SetStatus(DllExports::GdipGetStringFormatTrimming(
            nativeFormat,
            &trimming
        ));
        return trimming;
    }

    Status SetMeasurableCharacterRanges(
        IN INT                  rangeCount,
        IN const CharacterRange *ranges
    )
    {
        return SetStatus(DllExports::GdipSetStringFormatMeasurableCharacterRanges(
            nativeFormat,
            rangeCount,
            ranges
        ));
    }

    INT GetMeasurableCharacterRangeCount()
    {
        INT count;
        SetStatus(DllExports::GdipGetStringFormatMeasurableCharacterRangeCount(
            nativeFormat,
            &count
        ));
        return count;
    }

    Status GetLastStatus() const
    {
        Status lastStatus = lastError;
        lastError = Ok;

        return lastStatus;
    }

protected:

    Status SetStatus(GpStatus newStatus) const
    {
        if (newStatus == Ok)
        {
            return Ok;
        }
        else
        {
            return lastError = newStatus;
        }
    }

    StringFormat(const StringFormat &source)
    {
        nativeFormat = NULL;
        lastError = DllExports::GdipCloneStringFormat(
            source.nativeFormat,
            &nativeFormat
        );
    }

    StringFormat& operator=(const StringFormat &source)
    {
        DllExports::GdipDeleteStringFormat(nativeFormat);
        lastError = DllExports::GdipCloneStringFormat(
            source.nativeFormat,
            &nativeFormat
        );
        return *this;
    }

    StringFormat(GpStringFormat * clonedStringFormat, Status status)
    {
        lastError = status;
        nativeFormat = clonedStringFormat;

    }

    GpStringFormat *nativeFormat;
    mutable Status  lastError;
};

static BYTE GenericTypographicStringFormatBuffer[sizeof(StringFormat)] = {0};
static BYTE GenericDefaultStringFormatBuffer[sizeof(StringFormat)] = {0};

inline const StringFormat *StringFormat::GenericDefault()
{
    StringFormat * genericDefaultStringFormat =
        (StringFormat*)GenericDefaultStringFormatBuffer;

    genericDefaultStringFormat->lastError =
        DllExports::GdipStringFormatGetGenericDefault(
            &(genericDefaultStringFormat->nativeFormat)
        );

    return genericDefaultStringFormat;
}

inline const StringFormat *StringFormat::GenericTypographic()
{
    StringFormat * genericTypographicStringFormat =
        (StringFormat*)GenericTypographicStringFormatBuffer;

    genericTypographicStringFormat->lastError =
        DllExports::GdipStringFormatGetGenericTypographic(
            &genericTypographicStringFormat->nativeFormat
        );

    return genericTypographicStringFormat;
}

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_GDIPLUSSTRINGFORMAT_H
