

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.00.0594 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif // __RPCNDR_H_VERSION__

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __tom_h__
#define __tom_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

/* Forward Declarations */ 

#ifndef __ITextDocument_FWD_DEFINED__
#define __ITextDocument_FWD_DEFINED__
typedef interface ITextDocument ITextDocument;

#endif 	/* __ITextDocument_FWD_DEFINED__ */


#ifndef __ITextRange_FWD_DEFINED__
#define __ITextRange_FWD_DEFINED__
typedef interface ITextRange ITextRange;

#endif 	/* __ITextRange_FWD_DEFINED__ */


#ifndef __ITextSelection_FWD_DEFINED__
#define __ITextSelection_FWD_DEFINED__
typedef interface ITextSelection ITextSelection;

#endif 	/* __ITextSelection_FWD_DEFINED__ */


#ifndef __ITextFont_FWD_DEFINED__
#define __ITextFont_FWD_DEFINED__
typedef interface ITextFont ITextFont;

#endif 	/* __ITextFont_FWD_DEFINED__ */


#ifndef __ITextPara_FWD_DEFINED__
#define __ITextPara_FWD_DEFINED__
typedef interface ITextPara ITextPara;

#endif 	/* __ITextPara_FWD_DEFINED__ */


#ifndef __ITextStoryRanges_FWD_DEFINED__
#define __ITextStoryRanges_FWD_DEFINED__
typedef interface ITextStoryRanges ITextStoryRanges;

#endif 	/* __ITextStoryRanges_FWD_DEFINED__ */


#ifndef __ITextDocument2_FWD_DEFINED__
#define __ITextDocument2_FWD_DEFINED__
typedef interface ITextDocument2 ITextDocument2;

#endif 	/* __ITextDocument2_FWD_DEFINED__ */


#ifndef __ITextRange2_FWD_DEFINED__
#define __ITextRange2_FWD_DEFINED__
typedef interface ITextRange2 ITextRange2;

#endif 	/* __ITextRange2_FWD_DEFINED__ */


#ifndef __ITextSelection2_FWD_DEFINED__
#define __ITextSelection2_FWD_DEFINED__
typedef interface ITextSelection2 ITextSelection2;

#endif 	/* __ITextSelection2_FWD_DEFINED__ */


#ifndef __ITextFont2_FWD_DEFINED__
#define __ITextFont2_FWD_DEFINED__
typedef interface ITextFont2 ITextFont2;

#endif 	/* __ITextFont2_FWD_DEFINED__ */


#ifndef __ITextPara2_FWD_DEFINED__
#define __ITextPara2_FWD_DEFINED__
typedef interface ITextPara2 ITextPara2;

#endif 	/* __ITextPara2_FWD_DEFINED__ */


#ifndef __ITextStoryRanges2_FWD_DEFINED__
#define __ITextStoryRanges2_FWD_DEFINED__
typedef interface ITextStoryRanges2 ITextStoryRanges2;

#endif 	/* __ITextStoryRanges2_FWD_DEFINED__ */


#ifndef __ITextStory_FWD_DEFINED__
#define __ITextStory_FWD_DEFINED__
typedef interface ITextStory ITextStory;

#endif 	/* __ITextStory_FWD_DEFINED__ */


#ifndef __ITextStrings_FWD_DEFINED__
#define __ITextStrings_FWD_DEFINED__
typedef interface ITextStrings ITextStrings;

#endif 	/* __ITextStrings_FWD_DEFINED__ */


#ifndef __ITextRow_FWD_DEFINED__
#define __ITextRow_FWD_DEFINED__
typedef interface ITextRow ITextRow;

#endif 	/* __ITextRow_FWD_DEFINED__ */


#ifndef __ITextDisplays_FWD_DEFINED__
#define __ITextDisplays_FWD_DEFINED__
typedef interface ITextDisplays ITextDisplays;

#endif 	/* __ITextDisplays_FWD_DEFINED__ */


#ifndef __ITextDocument_FWD_DEFINED__
#define __ITextDocument_FWD_DEFINED__
typedef interface ITextDocument ITextDocument;

#endif 	/* __ITextDocument_FWD_DEFINED__ */


#ifndef __ITextRange_FWD_DEFINED__
#define __ITextRange_FWD_DEFINED__
typedef interface ITextRange ITextRange;

#endif 	/* __ITextRange_FWD_DEFINED__ */


#ifndef __ITextSelection_FWD_DEFINED__
#define __ITextSelection_FWD_DEFINED__
typedef interface ITextSelection ITextSelection;

#endif 	/* __ITextSelection_FWD_DEFINED__ */


#ifndef __ITextFont_FWD_DEFINED__
#define __ITextFont_FWD_DEFINED__
typedef interface ITextFont ITextFont;

#endif 	/* __ITextFont_FWD_DEFINED__ */


#ifndef __ITextPara_FWD_DEFINED__
#define __ITextPara_FWD_DEFINED__
typedef interface ITextPara ITextPara;

#endif 	/* __ITextPara_FWD_DEFINED__ */


#ifndef __ITextStoryRanges_FWD_DEFINED__
#define __ITextStoryRanges_FWD_DEFINED__
typedef interface ITextStoryRanges ITextStoryRanges;

#endif 	/* __ITextStoryRanges_FWD_DEFINED__ */


#ifndef __ITextDocument2_FWD_DEFINED__
#define __ITextDocument2_FWD_DEFINED__
typedef interface ITextDocument2 ITextDocument2;

#endif 	/* __ITextDocument2_FWD_DEFINED__ */


#ifndef __ITextDocument2Old_FWD_DEFINED__
#define __ITextDocument2Old_FWD_DEFINED__
typedef interface ITextDocument2Old ITextDocument2Old;

#endif 	/* __ITextDocument2Old_FWD_DEFINED__ */


#ifndef __ITextRange2_FWD_DEFINED__
#define __ITextRange2_FWD_DEFINED__
typedef interface ITextRange2 ITextRange2;

#endif 	/* __ITextRange2_FWD_DEFINED__ */


#ifndef __ITextSelection2_FWD_DEFINED__
#define __ITextSelection2_FWD_DEFINED__
typedef interface ITextSelection2 ITextSelection2;

#endif 	/* __ITextSelection2_FWD_DEFINED__ */


#ifndef __ITextFont2_FWD_DEFINED__
#define __ITextFont2_FWD_DEFINED__
typedef interface ITextFont2 ITextFont2;

#endif 	/* __ITextFont2_FWD_DEFINED__ */


#ifndef __ITextPara2_FWD_DEFINED__
#define __ITextPara2_FWD_DEFINED__
typedef interface ITextPara2 ITextPara2;

#endif 	/* __ITextPara2_FWD_DEFINED__ */


#ifndef __ITextStoryRanges2_FWD_DEFINED__
#define __ITextStoryRanges2_FWD_DEFINED__
typedef interface ITextStoryRanges2 ITextStoryRanges2;

#endif 	/* __ITextStoryRanges2_FWD_DEFINED__ */


#ifndef __ITextStrings_FWD_DEFINED__
#define __ITextStrings_FWD_DEFINED__
typedef interface ITextStrings ITextStrings;

#endif 	/* __ITextStrings_FWD_DEFINED__ */


#ifndef __ITextStory_FWD_DEFINED__
#define __ITextStory_FWD_DEFINED__
typedef interface ITextStory ITextStory;

#endif 	/* __ITextStory_FWD_DEFINED__ */


#ifndef __ITextDisplays_FWD_DEFINED__
#define __ITextDisplays_FWD_DEFINED__
typedef interface ITextDisplays ITextDisplays;

#endif 	/* __ITextDisplays_FWD_DEFINED__ */


#ifndef __ITextRow_FWD_DEFINED__
#define __ITextRow_FWD_DEFINED__
typedef interface ITextRow ITextRow;

#endif 	/* __ITextRow_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __tom_LIBRARY_DEFINED__
#define __tom_LIBRARY_DEFINED__

/* library tom */
/* [version][uuid] */ 

typedef /* [public] */ 
enum __MIDL___MIDL_itf_tom_0000_0000_0001
    {
        tomFalse	= 0,
        tomTrue	= -1,
        tomUndefined	= -9999999,
        tomToggle	= -9999998,
        tomAutoColor	= -9999997,
        tomDefault	= -9999996,
        tomSuspend	= -9999995,
        tomResume	= -9999994,
        tomApplyNow	= 0,
        tomApplyLater	= 1,
        tomTrackParms	= 2,
        tomCacheParms	= 3,
        tomApplyTmp	= 4,
        tomDisableSmartFont	= 8,
        tomEnableSmartFont	= 9,
        tomUsePoints	= 10,
        tomUseTwips	= 11,
        tomBackward	= 0xc0000001,
        tomForward	= 0x3fffffff,
        tomMove	= 0,
        tomExtend	= 1,
        tomNoSelection	= 0,
        tomSelectionIP	= 1,
        tomSelectionNormal	= 2,
        tomSelectionFrame	= 3,
        tomSelectionColumn	= 4,
        tomSelectionRow	= 5,
        tomSelectionBlock	= 6,
        tomSelectionInlineShape	= 7,
        tomSelectionShape	= 8,
        tomSelStartActive	= 1,
        tomSelAtEOL	= 2,
        tomSelOvertype	= 4,
        tomSelActive	= 8,
        tomSelReplace	= 16,
        tomEnd	= 0,
        tomStart	= 32,
        tomCollapseEnd	= 0,
        tomCollapseStart	= 1,
        tomClientCoord	= 256,
        tomAllowOffClient	= 512,
        tomTransform	= 1024,
        tomObjectArg	= 2048,
        tomAtEnd	= 4096,
        tomNone	= 0,
        tomSingle	= 1,
        tomWords	= 2,
        tomDouble	= 3,
        tomDotted	= 4,
        tomDash	= 5,
        tomDashDot	= 6,
        tomDashDotDot	= 7,
        tomWave	= 8,
        tomThick	= 9,
        tomHair	= 10,
        tomDoubleWave	= 11,
        tomHeavyWave	= 12,
        tomLongDash	= 13,
        tomThickDash	= 14,
        tomThickDashDot	= 15,
        tomThickDashDotDot	= 16,
        tomThickDotted	= 17,
        tomThickLongDash	= 18,
        tomLineSpaceSingle	= 0,
        tomLineSpace1pt5	= 1,
        tomLineSpaceDouble	= 2,
        tomLineSpaceAtLeast	= 3,
        tomLineSpaceExactly	= 4,
        tomLineSpaceMultiple	= 5,
        tomLineSpacePercent	= 6,
        tomAlignLeft	= 0,
        tomAlignCenter	= 1,
        tomAlignRight	= 2,
        tomAlignJustify	= 3,
        tomAlignDecimal	= 3,
        tomAlignBar	= 4,
        tomDefaultTab	= 5,
        tomAlignInterWord	= 3,
        tomAlignNewspaper	= 4,
        tomAlignInterLetter	= 5,
        tomAlignScaled	= 6,
        tomSpaces	= 0,
        tomDots	= 1,
        tomDashes	= 2,
        tomLines	= 3,
        tomThickLines	= 4,
        tomEquals	= 5,
        tomTabBack	= -3,
        tomTabNext	= -2,
        tomTabHere	= -1,
        tomListNone	= 0,
        tomListBullet	= 1,
        tomListNumberAsArabic	= 2,
        tomListNumberAsLCLetter	= 3,
        tomListNumberAsUCLetter	= 4,
        tomListNumberAsLCRoman	= 5,
        tomListNumberAsUCRoman	= 6,
        tomListNumberAsSequence	= 7,
        tomListNumberedCircle	= 8,
        tomListNumberedBlackCircleWingding	= 9,
        tomListNumberedWhiteCircleWingding	= 10,
        tomListNumberedArabicWide	= 11,
        tomListNumberedChS	= 12,
        tomListNumberedChT	= 13,
        tomListNumberedJpnChS	= 14,
        tomListNumberedJpnKor	= 15,
        tomListNumberedArabic1	= 16,
        tomListNumberedArabic2	= 17,
        tomListNumberedHebrew	= 18,
        tomListNumberedThaiAlpha	= 19,
        tomListNumberedThaiNum	= 20,
        tomListNumberedHindiAlpha	= 21,
        tomListNumberedHindiAlpha1	= 22,
        tomListNumberedHindiNum	= 23,
        tomListParentheses	= 0x10000,
        tomListPeriod	= 0x20000,
        tomListPlain	= 0x30000,
        tomListNoNumber	= 0x40000,
        tomListMinus	= 0x80000,
        tomIgnoreNumberStyle	= 0x1000000,
        tomParaStyleNormal	= -1,
        tomParaStyleHeading1	= -2,
        tomParaStyleHeading2	= -3,
        tomParaStyleHeading3	= -4,
        tomParaStyleHeading4	= -5,
        tomParaStyleHeading5	= -6,
        tomParaStyleHeading6	= -7,
        tomParaStyleHeading7	= -8,
        tomParaStyleHeading8	= -9,
        tomParaStyleHeading9	= -10,
        tomCharacter	= 1,
        tomWord	= 2,
        tomSentence	= 3,
        tomParagraph	= 4,
        tomLine	= 5,
        tomStory	= 6,
        tomScreen	= 7,
        tomSection	= 8,
        tomTableColumn	= 9,
        tomColumn	= 9,
        tomRow	= 10,
        tomWindow	= 11,
        tomCell	= 12,
        tomCharFormat	= 13,
        tomParaFormat	= 14,
        tomTable	= 15,
        tomObject	= 16,
        tomPage	= 17,
        tomHardParagraph	= 18,
        tomCluster	= 19,
        tomInlineObject	= 20,
        tomInlineObjectArg	= 21,
        tomLeafLine	= 22,
        tomLayoutColumn	= 23,
        tomProcessId	= 0x40000001,
        tomMatchWord	= 2,
        tomMatchCase	= 4,
        tomMatchPattern	= 8,
        tomUnknownStory	= 0,
        tomMainTextStory	= 1,
        tomFootnotesStory	= 2,
        tomEndnotesStory	= 3,
        tomCommentsStory	= 4,
        tomTextFrameStory	= 5,
        tomEvenPagesHeaderStory	= 6,
        tomPrimaryHeaderStory	= 7,
        tomEvenPagesFooterStory	= 8,
        tomPrimaryFooterStory	= 9,
        tomFirstPageHeaderStory	= 10,
        tomFirstPageFooterStory	= 11,
        tomScratchStory	= 127,
        tomFindStory	= 128,
        tomReplaceStory	= 129,
        tomStoryInactive	= 0,
        tomStoryActiveDisplay	= 1,
        tomStoryActiveUI	= 2,
        tomStoryActiveDisplayUI	= 3,
        tomNoAnimation	= 0,
        tomLasVegasLights	= 1,
        tomBlinkingBackground	= 2,
        tomSparkleText	= 3,
        tomMarchingBlackAnts	= 4,
        tomMarchingRedAnts	= 5,
        tomShimmer	= 6,
        tomWipeDown	= 7,
        tomWipeRight	= 8,
        tomAnimationMax	= 8,
        tomLowerCase	= 0,
        tomUpperCase	= 1,
        tomTitleCase	= 2,
        tomSentenceCase	= 4,
        tomToggleCase	= 5,
        tomReadOnly	= 0x100,
        tomShareDenyRead	= 0x200,
        tomShareDenyWrite	= 0x400,
        tomPasteFile	= 0x1000,
        tomCreateNew	= 0x10,
        tomCreateAlways	= 0x20,
        tomOpenExisting	= 0x30,
        tomOpenAlways	= 0x40,
        tomTruncateExisting	= 0x50,
        tomRTF	= 0x1,
        tomText	= 0x2,
        tomHTML	= 0x3,
        tomWordDocument	= 0x4,
        tomBold	= 0x80000001,
        tomItalic	= 0x80000002,
        tomUnderline	= 0x80000004,
        tomStrikeout	= 0x80000008,
        tomProtected	= 0x80000010,
        tomLink	= 0x80000020,
        tomSmallCaps	= 0x80000040,
        tomAllCaps	= 0x80000080,
        tomHidden	= 0x80000100,
        tomOutline	= 0x80000200,
        tomShadow	= 0x80000400,
        tomEmboss	= 0x80000800,
        tomImprint	= 0x80001000,
        tomDisabled	= 0x80002000,
        tomRevised	= 0x80004000,
        tomSubscriptCF	= 0x80010000,
        tomSuperscriptCF	= 0x80020000,
        tomFontBound	= 0x80100000,
        tomLinkProtected	= 0x80800000,
        tomInlineObjectStart	= 0x81000000,
        tomExtendedChar	= 0x82000000,
        tomAutoBackColor	= 0x84000000,
        tomMathZoneNoBuildUp	= 0x88000000,
        tomMathZone	= 0x90000000,
        tomMathZoneOrdinary	= 0xa0000000,
        tomAutoTextColor	= 0xc0000000,
        tomMathZoneDisplay	= 0x40000,
        tomParaEffectRTL	= 0x1,
        tomParaEffectKeep	= 0x2,
        tomParaEffectKeepNext	= 0x4,
        tomParaEffectPageBreakBefore	= 0x8,
        tomParaEffectNoLineNumber	= 0x10,
        tomParaEffectNoWidowControl	= 0x20,
        tomParaEffectDoNotHyphen	= 0x40,
        tomParaEffectSideBySide	= 0x80,
        tomParaEffectCollapsed	= 0x100,
        tomParaEffectOutlineLevel	= 0x200,
        tomParaEffectBox	= 0x400,
        tomParaEffectTableRowDelimiter	= 0x1000,
        tomParaEffectTable	= 0x4000,
        tomModWidthPairs	= 0x1,
        tomModWidthSpace	= 0x2,
        tomAutoSpaceAlpha	= 0x4,
        tomAutoSpaceNumeric	= 0x8,
        tomAutoSpaceParens	= 0x10,
        tomEmbeddedFont	= 0x20,
        tomDoublestrike	= 0x40,
        tomOverlapping	= 0x80,
        tomNormalCaret	= 0,
        tomKoreanBlockCaret	= 0x1,
        tomNullCaret	= 0x2,
        tomIncludeInset	= 0x1,
        tomUnicodeBiDi	= 0x1,
        tomMathCFCheck	= 0x4,
        tomUnlink	= 0x8,
        tomUnhide	= 0x10,
        tomCheckTextLimit	= 0x20,
        tomIgnoreCurrentFont	= 0,
        tomMatchCharRep	= 0x1,
        tomMatchFontSignature	= 0x2,
        tomMatchAscii	= 0x4,
        tomGetHeightOnly	= 0x8,
        tomMatchMathFont	= 0x10,
        tomCharset	= 0x80000000,
        tomCharRepFromLcid	= 0x40000000,
        tomAnsi	= 0,
        tomEastEurope	= 1,
        tomCyrillic	= 2,
        tomGreek	= 3,
        tomTurkish	= 4,
        tomHebrew	= 5,
        tomArabic	= 6,
        tomBaltic	= 7,
        tomVietnamese	= 8,
        tomDefaultCharRep	= 9,
        tomSymbol	= 10,
        tomThai	= 11,
        tomShiftJIS	= 12,
        tomGB2312	= 13,
        tomHangul	= 14,
        tomBIG5	= 15,
        tomPC437	= 16,
        tomOEM	= 17,
        tomMac	= 18,
        tomArmenian	= 19,
        tomSyriac	= 20,
        tomThaana	= 21,
        tomDevanagari	= 22,
        tomBengali	= 23,
        tomGurmukhi	= 24,
        tomGujarati	= 25,
        tomOriya	= 26,
        tomTamil	= 27,
        tomTelugu	= 28,
        tomKannada	= 29,
        tomMalayalam	= 30,
        tomSinhala	= 31,
        tomLao	= 32,
        tomTibetan	= 33,
        tomMyanmar	= 34,
        tomGeorgian	= 35,
        tomJamo	= 36,
        tomEthiopic	= 37,
        tomCherokee	= 38,
        tomAboriginal	= 39,
        tomOgham	= 40,
        tomRunic	= 41,
        tomKhmer	= 42,
        tomMongolian	= 43,
        tomBraille	= 44,
        tomYi	= 45,
        tomLimbu	= 46,
        tomTaiLe	= 47,
        tomNewTaiLue	= 48,
        tomSylotiNagri	= 49,
        tomKharoshthi	= 50,
        tomKayahli	= 51,
        tomUsymbol	= 52,
        tomEmoji	= 53,
        tomGlagolitic	= 54,
        tomLisu	= 55,
        tomVai	= 56,
        tomNKo	= 57,
        tomOsmanya	= 58,
        tomPhagsPa	= 59,
        tomGothic	= 60,
        tomDeseret	= 61,
        tomTifinagh	= 62,
        tomCharRepMax	= 63,
        tomRE10Mode	= 0x1,
        tomUseAtFont	= 0x2,
        tomTextFlowMask	= 0xc,
        tomTextFlowES	= 0,
        tomTextFlowSW	= 0x4,
        tomTextFlowWN	= 0x8,
        tomTextFlowNE	= 0xc,
        tomNoIME	= 0x80000,
        tomSelfIME	= 0x40000,
        tomNoUpScroll	= 0x10000,
        tomNoVpScroll	= 0x40000,
        tomNoLink	= 0,
        tomClientLink	= 1,
        tomFriendlyLinkName	= 2,
        tomFriendlyLinkAddress	= 3,
        tomAutoLinkURL	= 4,
        tomAutoLinkEmail	= 5,
        tomAutoLinkPhone	= 6,
        tomAutoLinkPath	= 7,
        tomCompressNone	= 0,
        tomCompressPunctuation	= 1,
        tomCompressPunctuationAndKana	= 2,
        tomCompressMax	= 2,
        tomUnderlinePositionAuto	= 0,
        tomUnderlinePositionBelow	= 1,
        tomUnderlinePositionAbove	= 2,
        tomUnderlinePositionMax	= 2,
        tomFontAlignmentAuto	= 0,
        tomFontAlignmentTop	= 1,
        tomFontAlignmentBaseline	= 2,
        tomFontAlignmentBottom	= 3,
        tomFontAlignmentCenter	= 4,
        tomFontAlignmentMax	= 4,
        tomRubyBelow	= 0x80,
        tomRubyAlignCenter	= 0,
        tomRubyAlign010	= 1,
        tomRubyAlign121	= 2,
        tomRubyAlignLeft	= 3,
        tomRubyAlignRight	= 4,
        tomLimitsDefault	= 0,
        tomLimitsUnderOver	= 1,
        tomLimitsSubSup	= 2,
        tomUpperLimitAsSuperScript	= 3,
        tomLimitsOpposite	= 4,
        tomShowLLimPlaceHldr	= 8,
        tomShowULimPlaceHldr	= 16,
        tomDontGrowWithContent	= 64,
        tomGrowWithContent	= 128,
        tomSubSupAlign	= 1,
        tomLimitAlignMask	= 3,
        tomLimitAlignCenter	= 0,
        tomLimitAlignLeft	= 1,
        tomLimitAlignRight	= 2,
        tomShowDegPlaceHldr	= 8,
        tomAlignDefault	= 0,
        tomAlignMatchAscentDescent	= 2,
        tomMathVariant	= 0x20,
        tomStyleDefault	= 0,
        tomStyleScriptScriptCramped	= 1,
        tomStyleScriptScript	= 2,
        tomStyleScriptCramped	= 3,
        tomStyleScript	= 4,
        tomStyleTextCramped	= 5,
        tomStyleText	= 6,
        tomStyleDisplayCramped	= 7,
        tomStyleDisplay	= 8,
        tomMathRelSize	= 0x40,
        tomDecDecSize	= 0xfe,
        tomDecSize	= 0xff,
        tomIncSize	= ( 1 | tomMathRelSize ) ,
        tomIncIncSize	= ( 2 | tomMathRelSize ) ,
        tomGravityUI	= 0,
        tomGravityBack	= 1,
        tomGravityFore	= 2,
        tomGravityIn	= 3,
        tomGravityOut	= 4,
        tomGravityBackward	= 0x20000000,
        tomGravityForward	= 0x40000000,
        tomAdjustCRLF	= 1,
        tomUseCRLF	= 2,
        tomTextize	= 4,
        tomAllowFinalEOP	= 8,
        tomFoldMathAlpha	= 16,
        tomNoHidden	= 32,
        tomIncludeNumbering	= 64,
        tomTranslateTableCell	= 128,
        tomNoMathZoneBrackets	= 0x100,
        tomConvertMathChar	= 0x200,
        tomNoUCGreekItalic	= 0x400,
        tomAllowMathBold	= 0x800,
        tomLanguageTag	= 0x1000,
        tomConvertRTF	= 0x2000,
        tomApplyRtfDocProps	= 0x4000,
        tomPhantomShow	= 1,
        tomPhantomZeroWidth	= 2,
        tomPhantomZeroAscent	= 4,
        tomPhantomZeroDescent	= 8,
        tomPhantomTransparent	= 16,
        tomPhantomASmash	= ( tomPhantomShow | tomPhantomZeroAscent ) ,
        tomPhantomDSmash	= ( tomPhantomShow | tomPhantomZeroDescent ) ,
        tomPhantomHSmash	= ( tomPhantomShow | tomPhantomZeroWidth ) ,
        tomPhantomSmash	= ( ( tomPhantomShow | tomPhantomZeroAscent )  | tomPhantomZeroDescent ) ,
        tomPhantomHorz	= ( tomPhantomZeroAscent | tomPhantomZeroDescent ) ,
        tomPhantomVert	= tomPhantomZeroWidth,
        tomBoxHideTop	= 1,
        tomBoxHideBottom	= 2,
        tomBoxHideLeft	= 4,
        tomBoxHideRight	= 8,
        tomBoxStrikeH	= 16,
        tomBoxStrikeV	= 32,
        tomBoxStrikeTLBR	= 64,
        tomBoxStrikeBLTR	= 128,
        tomBoxAlignCenter	= 1,
        tomSpaceMask	= 0x1c,
        tomSpaceDefault	= 0,
        tomSpaceUnary	= 4,
        tomSpaceBinary	= 8,
        tomSpaceRelational	= 12,
        tomSpaceSkip	= 16,
        tomSpaceOrd	= 20,
        tomSpaceDifferential	= 24,
        tomSizeText	= 32,
        tomSizeScript	= 64,
        tomSizeScriptScript	= 96,
        tomNoBreak	= 128,
        tomTransparentForPositioning	= 256,
        tomTransparentForSpacing	= 512,
        tomStretchCharBelow	= 0,
        tomStretchCharAbove	= 1,
        tomStretchBaseBelow	= 2,
        tomStretchBaseAbove	= 3,
        tomMatrixAlignMask	= 3,
        tomMatrixAlignCenter	= 0,
        tomMatrixAlignTopRow	= 1,
        tomMatrixAlignBottomRow	= 3,
        tomShowMatPlaceHldr	= 8,
        tomEqArrayLayoutWidth	= 1,
        tomEqArrayAlignMask	= 0xc,
        tomEqArrayAlignCenter	= 0,
        tomEqArrayAlignTopRow	= 4,
        tomEqArrayAlignBottomRow	= 0xc,
        tomMathManualBreakMask	= 0x7f,
        tomMathBreakLeft	= 0x7d,
        tomMathBreakCenter	= 0x7e,
        tomMathBreakRight	= 0x7f,
        tomMathEqAlign	= 0x80,
        tomMathArgShadingStart	= 0x251,
        tomMathArgShadingEnd	= 0x252,
        tomMathObjShadingStart	= 0x253,
        tomMathObjShadingEnd	= 0x254,
        tomFunctionTypeNone	= 0,
        tomFunctionTypeTakesArg	= 1,
        tomFunctionTypeTakesLim	= 2,
        tomFunctionTypeTakesLim2	= 3,
        tomFunctionTypeIsLim	= 4,
        tomMathParaAlignDefault	= 0,
        tomMathParaAlignCenterGroup	= 1,
        tomMathParaAlignCenter	= 2,
        tomMathParaAlignLeft	= 3,
        tomMathParaAlignRight	= 4,
        tomMathDispAlignMask	= 3,
        tomMathDispAlignCenterGroup	= 0,
        tomMathDispAlignCenter	= 1,
        tomMathDispAlignLeft	= 2,
        tomMathDispAlignRight	= 3,
        tomMathDispIntUnderOver	= 4,
        tomMathDispFracTeX	= 8,
        tomMathDispNaryGrow	= 0x10,
        tomMathDocEmptyArgMask	= 0x60,
        tomMathDocEmptyArgAuto	= 0,
        tomMathDocEmptyArgAlways	= 0x20,
        tomMathDocEmptyArgNever	= 0x40,
        tomMathDocSbSpOpUnchanged	= 0x80,
        tomMathDocDiffMask	= 0x300,
        tomMathDocDiffDefault	= 0,
        tomMathDocDiffUpright	= 0x100,
        tomMathDocDiffItalic	= 0x200,
        tomMathDocDiffOpenItalic	= 0x300,
        tomMathDispNarySubSup	= 0x400,
        tomMathDispDef	= 0x800,
        tomMathEnableRtl	= 0x1000,
        tomMathBrkBinMask	= 0x30000,
        tomMathBrkBinBefore	= 0,
        tomMathBrkBinAfter	= 0x10000,
        tomMathBrkBinDup	= 0x20000,
        tomMathBrkBinSubMask	= 0xc0000,
        tomMathBrkBinSubMM	= 0,
        tomMathBrkBinSubPM	= 0x40000,
        tomMathBrkBinSubMP	= 0x80000,
        tomSelRange	= 0x255,
        tomHstring	= 0x254,
        tomFontPropTeXStyle	= 0x33c,
        tomFontPropAlign	= 0x33d,
        tomFontStretch	= 0x33e,
        tomFontStyle	= 0x33f,
        tomFontStyleUpright	= 0,
        tomFontStyleOblique	= 1,
        tomFontStyleItalic	= 2,
        tomFontStretchDefault	= 0,
        tomFontStretchUltraCondensed	= 1,
        tomFontStretchExtraCondensed	= 2,
        tomFontStretchCondensed	= 3,
        tomFontStretchSemiCondensed	= 4,
        tomFontStretchNormal	= 5,
        tomFontStretchSemiExpanded	= 6,
        tomFontStretchExpanded	= 7,
        tomFontStretchExtraExpanded	= 8,
        tomFontStretchUltraExpanded	= 9,
        tomFontWeightDefault	= 0,
        tomFontWeightThin	= 100,
        tomFontWeightExtraLight	= 200,
        tomFontWeightLight	= 300,
        tomFontWeightNormal	= 400,
        tomFontWeightRegular	= 400,
        tomFontWeightMedium	= 500,
        tomFontWeightSemiBold	= 600,
        tomFontWeightBold	= 700,
        tomFontWeightExtraBold	= 800,
        tomFontWeightBlack	= 900,
        tomFontWeightHeavy	= 900,
        tomFontWeightExtraBlack	= 950,
        tomParaPropMathAlign	= 0x437,
        tomDocMathBuild	= 0x80,
        tomMathLMargin	= 0x81,
        tomMathRMargin	= 0x82,
        tomMathWrapIndent	= 0x83,
        tomMathWrapRight	= 0x84,
        tomMathPostSpace	= 0x86,
        tomMathPreSpace	= 0x85,
        tomMathInterSpace	= 0x87,
        tomMathIntraSpace	= 0x88,
        tomCanCopy	= 0x89,
        tomCanRedo	= 0x8a,
        tomCanUndo	= 0x8b,
        tomUndoLimit	= 0x8c,
        tomDocAutoLink	= 0x8d,
        tomEllipsisMode	= 0x8e,
        tomEllipsisState	= 0x8f,
        tomEllipsisNone	= 0,
        tomEllipsisEnd	= 1,
        tomEllipsisWord	= 3,
        tomEllipsisPresent	= 1,
        tomVTopCell	= 1,
        tomVLowCell	= 2,
        tomHStartCell	= 4,
        tomHContCell	= 8,
        tomRowUpdate	= 1,
        tomRowApplyDefault	= 0,
        tomCellStructureChangeOnly	= 1,
        tomRowHeightActual	= 0x80b
    } 	tomConstants;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_tom_0000_0000_0002
    {
        tomSimpleText	= 0,
        tomRuby	= ( tomSimpleText + 1 ) ,
        tomHorzVert	= ( tomRuby + 1 ) ,
        tomWarichu	= ( tomHorzVert + 1 ) ,
        tomEq	= 9,
        tomMath	= 10,
        tomAccent	= tomMath,
        tomBox	= ( tomAccent + 1 ) ,
        tomBoxedFormula	= ( tomBox + 1 ) ,
        tomBrackets	= ( tomBoxedFormula + 1 ) ,
        tomBracketsWithSeps	= ( tomBrackets + 1 ) ,
        tomEquationArray	= ( tomBracketsWithSeps + 1 ) ,
        tomFraction	= ( tomEquationArray + 1 ) ,
        tomFunctionApply	= ( tomFraction + 1 ) ,
        tomLeftSubSup	= ( tomFunctionApply + 1 ) ,
        tomLowerLimit	= ( tomLeftSubSup + 1 ) ,
        tomMatrix	= ( tomLowerLimit + 1 ) ,
        tomNary	= ( tomMatrix + 1 ) ,
        tomOpChar	= ( tomNary + 1 ) ,
        tomOverbar	= ( tomOpChar + 1 ) ,
        tomPhantom	= ( tomOverbar + 1 ) ,
        tomRadical	= ( tomPhantom + 1 ) ,
        tomSlashedFraction	= ( tomRadical + 1 ) ,
        tomStack	= ( tomSlashedFraction + 1 ) ,
        tomStretchStack	= ( tomStack + 1 ) ,
        tomSubscript	= ( tomStretchStack + 1 ) ,
        tomSubSup	= ( tomSubscript + 1 ) ,
        tomSuperscript	= ( tomSubSup + 1 ) ,
        tomUnderbar	= ( tomSuperscript + 1 ) ,
        tomUpperLimit	= ( tomUnderbar + 1 ) ,
        tomObjectMax	= tomUpperLimit
    } 	OBJECTTYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_tom_0000_0000_0003
    {
        MBOLD	= 0x10,
        MITAL	= 0x20,
        MGREEK	= 0x40,
        MROMN	= 0,
        MSCRP	= 1,
        MFRAK	= 2,
        MOPEN	= 3,
        MSANS	= 4,
        MMONO	= 5,
        MMATH	= 6,
        MISOL	= 7,
        MINIT	= 8,
        MTAIL	= 9,
        MSTRCH	= 10,
        MLOOP	= 11,
        MOPENA	= 12
    } 	MANCODE;


















EXTERN_C const IID LIBID_tom;

#ifndef __ITextDocument_INTERFACE_DEFINED__
#define __ITextDocument_INTERFACE_DEFINED__

/* interface ITextDocument */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CC497C0-A1DF-11ce-8098-00AA0047BE5D")
    ITextDocument : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection **ppSel) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStoryCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStoryRanges( 
            /* [retval][out] */ __RPC__deref_out_opt ITextStoryRanges **ppStories) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSaved( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSaved( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDefaultTabStop( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetDefaultTabStop( 
            /* [in] */ float Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE New( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Save( 
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Freeze( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Unfreeze( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BeginEditCollection( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EndEditCollection( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Undo( 
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Redo( 
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Range( 
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RangeFromPoint( 
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextDocumentVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextDocument * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextDocument * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextDocument * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextDocument * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextDocument * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextDocument * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextDocument * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection **ppSel);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryCount )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryRanges )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextStoryRanges **ppStories);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSaved )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSaved )( 
            __RPC__in ITextDocument * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultTabStop )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultTabStop )( 
            __RPC__in ITextDocument * This,
            /* [in] */ float Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *New )( 
            __RPC__in ITextDocument * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in ITextDocument * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in ITextDocument * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Freeze )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Unfreeze )( 
            __RPC__in ITextDocument * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BeginEditCollection )( 
            __RPC__in ITextDocument * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndEditCollection )( 
            __RPC__in ITextDocument * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Undo )( 
            __RPC__in ITextDocument * This,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Redo )( 
            __RPC__in ITextDocument * This,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Range )( 
            __RPC__in ITextDocument * This,
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RangeFromPoint )( 
            __RPC__in ITextDocument * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        END_INTERFACE
    } ITextDocumentVtbl;

    interface ITextDocument
    {
        CONST_VTBL struct ITextDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextDocument_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextDocument_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextDocument_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextDocument_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextDocument_GetName(This,pName)	\
    ( (This)->lpVtbl -> GetName(This,pName) ) 

#define ITextDocument_GetSelection(This,ppSel)	\
    ( (This)->lpVtbl -> GetSelection(This,ppSel) ) 

#define ITextDocument_GetStoryCount(This,pCount)	\
    ( (This)->lpVtbl -> GetStoryCount(This,pCount) ) 

#define ITextDocument_GetStoryRanges(This,ppStories)	\
    ( (This)->lpVtbl -> GetStoryRanges(This,ppStories) ) 

#define ITextDocument_GetSaved(This,pValue)	\
    ( (This)->lpVtbl -> GetSaved(This,pValue) ) 

#define ITextDocument_SetSaved(This,Value)	\
    ( (This)->lpVtbl -> SetSaved(This,Value) ) 

#define ITextDocument_GetDefaultTabStop(This,pValue)	\
    ( (This)->lpVtbl -> GetDefaultTabStop(This,pValue) ) 

#define ITextDocument_SetDefaultTabStop(This,Value)	\
    ( (This)->lpVtbl -> SetDefaultTabStop(This,Value) ) 

#define ITextDocument_New(This)	\
    ( (This)->lpVtbl -> New(This) ) 

#define ITextDocument_Open(This,pVar,Flags,CodePage)	\
    ( (This)->lpVtbl -> Open(This,pVar,Flags,CodePage) ) 

#define ITextDocument_Save(This,pVar,Flags,CodePage)	\
    ( (This)->lpVtbl -> Save(This,pVar,Flags,CodePage) ) 

#define ITextDocument_Freeze(This,pCount)	\
    ( (This)->lpVtbl -> Freeze(This,pCount) ) 

#define ITextDocument_Unfreeze(This,pCount)	\
    ( (This)->lpVtbl -> Unfreeze(This,pCount) ) 

#define ITextDocument_BeginEditCollection(This)	\
    ( (This)->lpVtbl -> BeginEditCollection(This) ) 

#define ITextDocument_EndEditCollection(This)	\
    ( (This)->lpVtbl -> EndEditCollection(This) ) 

#define ITextDocument_Undo(This,Count,pCount)	\
    ( (This)->lpVtbl -> Undo(This,Count,pCount) ) 

#define ITextDocument_Redo(This,Count,pCount)	\
    ( (This)->lpVtbl -> Redo(This,Count,pCount) ) 

#define ITextDocument_Range(This,cpActive,cpAnchor,ppRange)	\
    ( (This)->lpVtbl -> Range(This,cpActive,cpAnchor,ppRange) ) 

#define ITextDocument_RangeFromPoint(This,x,y,ppRange)	\
    ( (This)->lpVtbl -> RangeFromPoint(This,x,y,ppRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextDocument_INTERFACE_DEFINED__ */


#ifndef __ITextRange_INTERFACE_DEFINED__
#define __ITextRange_INTERFACE_DEFINED__

/* interface ITextRange */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextRange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CC497C2-A1DF-11ce-8098-00AA0047BE5D")
    ITextRange : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetChar( 
            /* [retval][out] */ __RPC__out long *pChar) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetChar( 
            /* [in] */ long Char) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDuplicate( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetFormattedText( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetFormattedText( 
            /* [in] */ __RPC__in_opt ITextRange *pRange) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStart( 
            /* [retval][out] */ __RPC__out long *pcpFirst) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetStart( 
            /* [in] */ long cpFirst) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetEnd( 
            /* [retval][out] */ __RPC__out long *pcpLim) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetEnd( 
            /* [in] */ long cpLim) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetFont( 
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetFont( 
            /* [in] */ __RPC__in_opt ITextFont *pFont) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetPara( 
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetPara( 
            /* [in] */ __RPC__in_opt ITextPara *pPara) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStoryLength( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStoryType( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Collapse( 
            /* [in] */ long bStart) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Expand( 
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetIndex( 
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pIndex) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetIndex( 
            /* [in] */ long Unit,
            /* [in] */ long Index,
            /* [in] */ long Extend) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetRange( 
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE InRange( 
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE InStory( 
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Select( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE StartOf( 
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EndOf( 
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveStart( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveEnd( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveWhile( 
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveStartWhile( 
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveEndWhile( 
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveUntil( 
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveStartUntil( 
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveEndUntil( 
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE FindText( 
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE FindTextStart( 
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE FindTextEnd( 
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Cut( 
            /* [out] */ __RPC__out VARIANT *pVar) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Copy( 
            /* [out] */ __RPC__out VARIANT *pVar) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Paste( 
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CanPaste( 
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CanEdit( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ChangeCase( 
            /* [in] */ long Type) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetPoint( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *px,
            /* [out] */ __RPC__out long *py) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetPoint( 
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ long Type,
            /* [in] */ long Extend) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ScrollIntoView( 
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEmbeddedObject( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextRangeVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextRange * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextRange * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextRange * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextRange * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextRange * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetChar )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__out long *pChar);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetChar )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Char);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStart )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__out long *pcpFirst);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStart )( 
            __RPC__in ITextRange * This,
            /* [in] */ long cpFirst);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEnd )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__out long *pcpLim);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEnd )( 
            __RPC__in ITextRange * This,
            /* [in] */ long cpLim);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFont )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFont )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPara )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPara )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryLength )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryType )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Collapse )( 
            __RPC__in ITextRange * This,
            /* [in] */ long bStart);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pIndex);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetIndex )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [in] */ long Index,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRange )( 
            __RPC__in ITextRange * This,
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InRange )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InStory )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ITextRange * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StartOf )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndOf )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStart )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEnd )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveWhile )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartWhile )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndWhile )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveUntil )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartUntil )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndUntil )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindText )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextStart )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextEnd )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cut )( 
            __RPC__in ITextRange * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in ITextRange * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Paste )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanPaste )( 
            __RPC__in ITextRange * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanEdit )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ChangeCase )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Type);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPoint )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *px,
            /* [out] */ __RPC__out long *py);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPoint )( 
            __RPC__in ITextRange * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ long Type,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ScrollIntoView )( 
            __RPC__in ITextRange * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEmbeddedObject )( 
            __RPC__in ITextRange * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        END_INTERFACE
    } ITextRangeVtbl;

    interface ITextRange
    {
        CONST_VTBL struct ITextRangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextRange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextRange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextRange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextRange_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextRange_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextRange_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextRange_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextRange_GetText(This,pbstr)	\
    ( (This)->lpVtbl -> GetText(This,pbstr) ) 

#define ITextRange_SetText(This,bstr)	\
    ( (This)->lpVtbl -> SetText(This,bstr) ) 

#define ITextRange_GetChar(This,pChar)	\
    ( (This)->lpVtbl -> GetChar(This,pChar) ) 

#define ITextRange_SetChar(This,Char)	\
    ( (This)->lpVtbl -> SetChar(This,Char) ) 

#define ITextRange_GetDuplicate(This,ppRange)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppRange) ) 

#define ITextRange_GetFormattedText(This,ppRange)	\
    ( (This)->lpVtbl -> GetFormattedText(This,ppRange) ) 

#define ITextRange_SetFormattedText(This,pRange)	\
    ( (This)->lpVtbl -> SetFormattedText(This,pRange) ) 

#define ITextRange_GetStart(This,pcpFirst)	\
    ( (This)->lpVtbl -> GetStart(This,pcpFirst) ) 

#define ITextRange_SetStart(This,cpFirst)	\
    ( (This)->lpVtbl -> SetStart(This,cpFirst) ) 

#define ITextRange_GetEnd(This,pcpLim)	\
    ( (This)->lpVtbl -> GetEnd(This,pcpLim) ) 

#define ITextRange_SetEnd(This,cpLim)	\
    ( (This)->lpVtbl -> SetEnd(This,cpLim) ) 

#define ITextRange_GetFont(This,ppFont)	\
    ( (This)->lpVtbl -> GetFont(This,ppFont) ) 

#define ITextRange_SetFont(This,pFont)	\
    ( (This)->lpVtbl -> SetFont(This,pFont) ) 

#define ITextRange_GetPara(This,ppPara)	\
    ( (This)->lpVtbl -> GetPara(This,ppPara) ) 

#define ITextRange_SetPara(This,pPara)	\
    ( (This)->lpVtbl -> SetPara(This,pPara) ) 

#define ITextRange_GetStoryLength(This,pCount)	\
    ( (This)->lpVtbl -> GetStoryLength(This,pCount) ) 

#define ITextRange_GetStoryType(This,pValue)	\
    ( (This)->lpVtbl -> GetStoryType(This,pValue) ) 

#define ITextRange_Collapse(This,bStart)	\
    ( (This)->lpVtbl -> Collapse(This,bStart) ) 

#define ITextRange_Expand(This,Unit,pDelta)	\
    ( (This)->lpVtbl -> Expand(This,Unit,pDelta) ) 

#define ITextRange_GetIndex(This,Unit,pIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,Unit,pIndex) ) 

#define ITextRange_SetIndex(This,Unit,Index,Extend)	\
    ( (This)->lpVtbl -> SetIndex(This,Unit,Index,Extend) ) 

#define ITextRange_SetRange(This,cpAnchor,cpActive)	\
    ( (This)->lpVtbl -> SetRange(This,cpAnchor,cpActive) ) 

#define ITextRange_InRange(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InRange(This,pRange,pValue) ) 

#define ITextRange_InStory(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InStory(This,pRange,pValue) ) 

#define ITextRange_IsEqual(This,pRange,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pRange,pValue) ) 

#define ITextRange_Select(This)	\
    ( (This)->lpVtbl -> Select(This) ) 

#define ITextRange_StartOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> StartOf(This,Unit,Extend,pDelta) ) 

#define ITextRange_EndOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> EndOf(This,Unit,Extend,pDelta) ) 

#define ITextRange_Move(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Move(This,Unit,Count,pDelta) ) 

#define ITextRange_MoveStart(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStart(This,Unit,Count,pDelta) ) 

#define ITextRange_MoveEnd(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEnd(This,Unit,Count,pDelta) ) 

#define ITextRange_MoveWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveWhile(This,Cset,Count,pDelta) ) 

#define ITextRange_MoveStartWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartWhile(This,Cset,Count,pDelta) ) 

#define ITextRange_MoveEndWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndWhile(This,Cset,Count,pDelta) ) 

#define ITextRange_MoveUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveUntil(This,Cset,Count,pDelta) ) 

#define ITextRange_MoveStartUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartUntil(This,Cset,Count,pDelta) ) 

#define ITextRange_MoveEndUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndUntil(This,Cset,Count,pDelta) ) 

#define ITextRange_FindText(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindText(This,bstr,Count,Flags,pLength) ) 

#define ITextRange_FindTextStart(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextStart(This,bstr,Count,Flags,pLength) ) 

#define ITextRange_FindTextEnd(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextEnd(This,bstr,Count,Flags,pLength) ) 

#define ITextRange_Delete(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Delete(This,Unit,Count,pDelta) ) 

#define ITextRange_Cut(This,pVar)	\
    ( (This)->lpVtbl -> Cut(This,pVar) ) 

#define ITextRange_Copy(This,pVar)	\
    ( (This)->lpVtbl -> Copy(This,pVar) ) 

#define ITextRange_Paste(This,pVar,Format)	\
    ( (This)->lpVtbl -> Paste(This,pVar,Format) ) 

#define ITextRange_CanPaste(This,pVar,Format,pValue)	\
    ( (This)->lpVtbl -> CanPaste(This,pVar,Format,pValue) ) 

#define ITextRange_CanEdit(This,pValue)	\
    ( (This)->lpVtbl -> CanEdit(This,pValue) ) 

#define ITextRange_ChangeCase(This,Type)	\
    ( (This)->lpVtbl -> ChangeCase(This,Type) ) 

#define ITextRange_GetPoint(This,Type,px,py)	\
    ( (This)->lpVtbl -> GetPoint(This,Type,px,py) ) 

#define ITextRange_SetPoint(This,x,y,Type,Extend)	\
    ( (This)->lpVtbl -> SetPoint(This,x,y,Type,Extend) ) 

#define ITextRange_ScrollIntoView(This,Value)	\
    ( (This)->lpVtbl -> ScrollIntoView(This,Value) ) 

#define ITextRange_GetEmbeddedObject(This,ppObject)	\
    ( (This)->lpVtbl -> GetEmbeddedObject(This,ppObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextRange_INTERFACE_DEFINED__ */


#ifndef __ITextSelection_INTERFACE_DEFINED__
#define __ITextSelection_INTERFACE_DEFINED__

/* interface ITextSelection */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextSelection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CC497C1-A1DF-11ce-8098-00AA0047BE5D")
    ITextSelection : public ITextRange
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [retval][out] */ __RPC__out long *pFlags) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [in] */ long Flags) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetType( 
            /* [retval][out] */ __RPC__out long *pType) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveLeft( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveRight( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveUp( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveDown( 
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE HomeKey( 
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EndKey( 
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE TypeText( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextSelectionVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextSelection * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextSelection * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextSelection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextSelection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextSelection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetChar )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pChar);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetChar )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Char);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStart )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pcpFirst);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStart )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long cpFirst);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEnd )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pcpLim);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEnd )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long cpLim);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFont )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFont )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPara )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPara )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryLength )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryType )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Collapse )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long bStart);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pIndex);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetIndex )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Index,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRange )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InRange )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InStory )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ITextSelection * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StartOf )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndOf )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStart )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEnd )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveWhile )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartWhile )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndWhile )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveUntil )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartUntil )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndUntil )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindText )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextStart )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextEnd )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cut )( 
            __RPC__in ITextSelection * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in ITextSelection * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Paste )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanPaste )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanEdit )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ChangeCase )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Type);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPoint )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *px,
            /* [out] */ __RPC__out long *py);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPoint )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ long Type,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ScrollIntoView )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEmbeddedObject )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pFlags);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Flags);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITextSelection * This,
            /* [retval][out] */ __RPC__out long *pType);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveLeft )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveRight )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveUp )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveDown )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HomeKey )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndKey )( 
            __RPC__in ITextSelection * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *TypeText )( 
            __RPC__in ITextSelection * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        END_INTERFACE
    } ITextSelectionVtbl;

    interface ITextSelection
    {
        CONST_VTBL struct ITextSelectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextSelection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextSelection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextSelection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextSelection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextSelection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextSelection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextSelection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextSelection_GetText(This,pbstr)	\
    ( (This)->lpVtbl -> GetText(This,pbstr) ) 

#define ITextSelection_SetText(This,bstr)	\
    ( (This)->lpVtbl -> SetText(This,bstr) ) 

#define ITextSelection_GetChar(This,pChar)	\
    ( (This)->lpVtbl -> GetChar(This,pChar) ) 

#define ITextSelection_SetChar(This,Char)	\
    ( (This)->lpVtbl -> SetChar(This,Char) ) 

#define ITextSelection_GetDuplicate(This,ppRange)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppRange) ) 

#define ITextSelection_GetFormattedText(This,ppRange)	\
    ( (This)->lpVtbl -> GetFormattedText(This,ppRange) ) 

#define ITextSelection_SetFormattedText(This,pRange)	\
    ( (This)->lpVtbl -> SetFormattedText(This,pRange) ) 

#define ITextSelection_GetStart(This,pcpFirst)	\
    ( (This)->lpVtbl -> GetStart(This,pcpFirst) ) 

#define ITextSelection_SetStart(This,cpFirst)	\
    ( (This)->lpVtbl -> SetStart(This,cpFirst) ) 

#define ITextSelection_GetEnd(This,pcpLim)	\
    ( (This)->lpVtbl -> GetEnd(This,pcpLim) ) 

#define ITextSelection_SetEnd(This,cpLim)	\
    ( (This)->lpVtbl -> SetEnd(This,cpLim) ) 

#define ITextSelection_GetFont(This,ppFont)	\
    ( (This)->lpVtbl -> GetFont(This,ppFont) ) 

#define ITextSelection_SetFont(This,pFont)	\
    ( (This)->lpVtbl -> SetFont(This,pFont) ) 

#define ITextSelection_GetPara(This,ppPara)	\
    ( (This)->lpVtbl -> GetPara(This,ppPara) ) 

#define ITextSelection_SetPara(This,pPara)	\
    ( (This)->lpVtbl -> SetPara(This,pPara) ) 

#define ITextSelection_GetStoryLength(This,pCount)	\
    ( (This)->lpVtbl -> GetStoryLength(This,pCount) ) 

#define ITextSelection_GetStoryType(This,pValue)	\
    ( (This)->lpVtbl -> GetStoryType(This,pValue) ) 

#define ITextSelection_Collapse(This,bStart)	\
    ( (This)->lpVtbl -> Collapse(This,bStart) ) 

#define ITextSelection_Expand(This,Unit,pDelta)	\
    ( (This)->lpVtbl -> Expand(This,Unit,pDelta) ) 

#define ITextSelection_GetIndex(This,Unit,pIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,Unit,pIndex) ) 

#define ITextSelection_SetIndex(This,Unit,Index,Extend)	\
    ( (This)->lpVtbl -> SetIndex(This,Unit,Index,Extend) ) 

#define ITextSelection_SetRange(This,cpAnchor,cpActive)	\
    ( (This)->lpVtbl -> SetRange(This,cpAnchor,cpActive) ) 

#define ITextSelection_InRange(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InRange(This,pRange,pValue) ) 

#define ITextSelection_InStory(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InStory(This,pRange,pValue) ) 

#define ITextSelection_IsEqual(This,pRange,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pRange,pValue) ) 

#define ITextSelection_Select(This)	\
    ( (This)->lpVtbl -> Select(This) ) 

#define ITextSelection_StartOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> StartOf(This,Unit,Extend,pDelta) ) 

#define ITextSelection_EndOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> EndOf(This,Unit,Extend,pDelta) ) 

#define ITextSelection_Move(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Move(This,Unit,Count,pDelta) ) 

#define ITextSelection_MoveStart(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStart(This,Unit,Count,pDelta) ) 

#define ITextSelection_MoveEnd(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEnd(This,Unit,Count,pDelta) ) 

#define ITextSelection_MoveWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveWhile(This,Cset,Count,pDelta) ) 

#define ITextSelection_MoveStartWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartWhile(This,Cset,Count,pDelta) ) 

#define ITextSelection_MoveEndWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndWhile(This,Cset,Count,pDelta) ) 

#define ITextSelection_MoveUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveUntil(This,Cset,Count,pDelta) ) 

#define ITextSelection_MoveStartUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartUntil(This,Cset,Count,pDelta) ) 

#define ITextSelection_MoveEndUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndUntil(This,Cset,Count,pDelta) ) 

#define ITextSelection_FindText(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindText(This,bstr,Count,Flags,pLength) ) 

#define ITextSelection_FindTextStart(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextStart(This,bstr,Count,Flags,pLength) ) 

#define ITextSelection_FindTextEnd(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextEnd(This,bstr,Count,Flags,pLength) ) 

#define ITextSelection_Delete(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Delete(This,Unit,Count,pDelta) ) 

#define ITextSelection_Cut(This,pVar)	\
    ( (This)->lpVtbl -> Cut(This,pVar) ) 

#define ITextSelection_Copy(This,pVar)	\
    ( (This)->lpVtbl -> Copy(This,pVar) ) 

#define ITextSelection_Paste(This,pVar,Format)	\
    ( (This)->lpVtbl -> Paste(This,pVar,Format) ) 

#define ITextSelection_CanPaste(This,pVar,Format,pValue)	\
    ( (This)->lpVtbl -> CanPaste(This,pVar,Format,pValue) ) 

#define ITextSelection_CanEdit(This,pValue)	\
    ( (This)->lpVtbl -> CanEdit(This,pValue) ) 

#define ITextSelection_ChangeCase(This,Type)	\
    ( (This)->lpVtbl -> ChangeCase(This,Type) ) 

#define ITextSelection_GetPoint(This,Type,px,py)	\
    ( (This)->lpVtbl -> GetPoint(This,Type,px,py) ) 

#define ITextSelection_SetPoint(This,x,y,Type,Extend)	\
    ( (This)->lpVtbl -> SetPoint(This,x,y,Type,Extend) ) 

#define ITextSelection_ScrollIntoView(This,Value)	\
    ( (This)->lpVtbl -> ScrollIntoView(This,Value) ) 

#define ITextSelection_GetEmbeddedObject(This,ppObject)	\
    ( (This)->lpVtbl -> GetEmbeddedObject(This,ppObject) ) 


#define ITextSelection_GetFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pFlags) ) 

#define ITextSelection_SetFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetFlags(This,Flags) ) 

#define ITextSelection_GetType(This,pType)	\
    ( (This)->lpVtbl -> GetType(This,pType) ) 

#define ITextSelection_MoveLeft(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveLeft(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection_MoveRight(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveRight(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection_MoveUp(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveUp(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection_MoveDown(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveDown(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection_HomeKey(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> HomeKey(This,Unit,Extend,pDelta) ) 

#define ITextSelection_EndKey(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> EndKey(This,Unit,Extend,pDelta) ) 

#define ITextSelection_TypeText(This,bstr)	\
    ( (This)->lpVtbl -> TypeText(This,bstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextSelection_INTERFACE_DEFINED__ */


#ifndef __ITextFont_INTERFACE_DEFINED__
#define __ITextFont_INTERFACE_DEFINED__

/* interface ITextFont */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextFont;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CC497C3-A1DF-11ce-8098-00AA0047BE5D")
    ITextFont : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDuplicate( 
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetDuplicate( 
            /* [in] */ __RPC__in_opt ITextFont *pFont) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CanChange( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ __RPC__in_opt ITextFont *pFont,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStyle( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetStyle( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAllCaps( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAllCaps( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAnimation( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAnimation( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetBackColor( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetBackColor( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetBold( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetBold( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetEmboss( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetEmboss( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetForeColor( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetForeColor( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetHidden( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetHidden( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetEngrave( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetEngrave( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetItalic( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetItalic( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetKerning( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetKerning( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetLanguageID( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetLanguageID( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetName( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetOutline( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetOutline( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetPosition( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetPosition( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetProtected( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetProtected( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetShadow( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetShadow( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSize( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSize( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSmallCaps( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSmallCaps( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSpacing( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSpacing( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStrikeThrough( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetStrikeThrough( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSubscript( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSubscript( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSuperscript( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSuperscript( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetUnderline( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetUnderline( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetWeight( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetWeight( 
            /* [in] */ long Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextFontVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextFont * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextFont * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextFont * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextFont * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextFont * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextFont * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextFont * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDuplicate )( 
            __RPC__in ITextFont * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanChange )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextFont * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStyle )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStyle )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAllCaps )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAllCaps )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAnimation )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAnimation )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetBackColor )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetBackColor )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetBold )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetBold )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEmboss )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEmboss )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetForeColor )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetForeColor )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetHidden )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetHidden )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEngrave )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEngrave )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetItalic )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetItalic )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKerning )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKerning )( 
            __RPC__in ITextFont * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLanguageID )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetLanguageID )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in ITextFont * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetOutline )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetOutline )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPosition )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPosition )( 
            __RPC__in ITextFont * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetProtected )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetProtected )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetShadow )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetShadow )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in ITextFont * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSmallCaps )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSmallCaps )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSpacing )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSpacing )( 
            __RPC__in ITextFont * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStrikeThrough )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStrikeThrough )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSubscript )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSubscript )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSuperscript )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSuperscript )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetUnderline )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetUnderline )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetWeight )( 
            __RPC__in ITextFont * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetWeight )( 
            __RPC__in ITextFont * This,
            /* [in] */ long Value);
        
        END_INTERFACE
    } ITextFontVtbl;

    interface ITextFont
    {
        CONST_VTBL struct ITextFontVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextFont_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextFont_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextFont_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextFont_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextFont_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextFont_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextFont_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextFont_GetDuplicate(This,ppFont)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppFont) ) 

#define ITextFont_SetDuplicate(This,pFont)	\
    ( (This)->lpVtbl -> SetDuplicate(This,pFont) ) 

#define ITextFont_CanChange(This,pValue)	\
    ( (This)->lpVtbl -> CanChange(This,pValue) ) 

#define ITextFont_IsEqual(This,pFont,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pFont,pValue) ) 

#define ITextFont_Reset(This,Value)	\
    ( (This)->lpVtbl -> Reset(This,Value) ) 

#define ITextFont_GetStyle(This,pValue)	\
    ( (This)->lpVtbl -> GetStyle(This,pValue) ) 

#define ITextFont_SetStyle(This,Value)	\
    ( (This)->lpVtbl -> SetStyle(This,Value) ) 

#define ITextFont_GetAllCaps(This,pValue)	\
    ( (This)->lpVtbl -> GetAllCaps(This,pValue) ) 

#define ITextFont_SetAllCaps(This,Value)	\
    ( (This)->lpVtbl -> SetAllCaps(This,Value) ) 

#define ITextFont_GetAnimation(This,pValue)	\
    ( (This)->lpVtbl -> GetAnimation(This,pValue) ) 

#define ITextFont_SetAnimation(This,Value)	\
    ( (This)->lpVtbl -> SetAnimation(This,Value) ) 

#define ITextFont_GetBackColor(This,pValue)	\
    ( (This)->lpVtbl -> GetBackColor(This,pValue) ) 

#define ITextFont_SetBackColor(This,Value)	\
    ( (This)->lpVtbl -> SetBackColor(This,Value) ) 

#define ITextFont_GetBold(This,pValue)	\
    ( (This)->lpVtbl -> GetBold(This,pValue) ) 

#define ITextFont_SetBold(This,Value)	\
    ( (This)->lpVtbl -> SetBold(This,Value) ) 

#define ITextFont_GetEmboss(This,pValue)	\
    ( (This)->lpVtbl -> GetEmboss(This,pValue) ) 

#define ITextFont_SetEmboss(This,Value)	\
    ( (This)->lpVtbl -> SetEmboss(This,Value) ) 

#define ITextFont_GetForeColor(This,pValue)	\
    ( (This)->lpVtbl -> GetForeColor(This,pValue) ) 

#define ITextFont_SetForeColor(This,Value)	\
    ( (This)->lpVtbl -> SetForeColor(This,Value) ) 

#define ITextFont_GetHidden(This,pValue)	\
    ( (This)->lpVtbl -> GetHidden(This,pValue) ) 

#define ITextFont_SetHidden(This,Value)	\
    ( (This)->lpVtbl -> SetHidden(This,Value) ) 

#define ITextFont_GetEngrave(This,pValue)	\
    ( (This)->lpVtbl -> GetEngrave(This,pValue) ) 

#define ITextFont_SetEngrave(This,Value)	\
    ( (This)->lpVtbl -> SetEngrave(This,Value) ) 

#define ITextFont_GetItalic(This,pValue)	\
    ( (This)->lpVtbl -> GetItalic(This,pValue) ) 

#define ITextFont_SetItalic(This,Value)	\
    ( (This)->lpVtbl -> SetItalic(This,Value) ) 

#define ITextFont_GetKerning(This,pValue)	\
    ( (This)->lpVtbl -> GetKerning(This,pValue) ) 

#define ITextFont_SetKerning(This,Value)	\
    ( (This)->lpVtbl -> SetKerning(This,Value) ) 

#define ITextFont_GetLanguageID(This,pValue)	\
    ( (This)->lpVtbl -> GetLanguageID(This,pValue) ) 

#define ITextFont_SetLanguageID(This,Value)	\
    ( (This)->lpVtbl -> SetLanguageID(This,Value) ) 

#define ITextFont_GetName(This,pbstr)	\
    ( (This)->lpVtbl -> GetName(This,pbstr) ) 

#define ITextFont_SetName(This,bstr)	\
    ( (This)->lpVtbl -> SetName(This,bstr) ) 

#define ITextFont_GetOutline(This,pValue)	\
    ( (This)->lpVtbl -> GetOutline(This,pValue) ) 

#define ITextFont_SetOutline(This,Value)	\
    ( (This)->lpVtbl -> SetOutline(This,Value) ) 

#define ITextFont_GetPosition(This,pValue)	\
    ( (This)->lpVtbl -> GetPosition(This,pValue) ) 

#define ITextFont_SetPosition(This,Value)	\
    ( (This)->lpVtbl -> SetPosition(This,Value) ) 

#define ITextFont_GetProtected(This,pValue)	\
    ( (This)->lpVtbl -> GetProtected(This,pValue) ) 

#define ITextFont_SetProtected(This,Value)	\
    ( (This)->lpVtbl -> SetProtected(This,Value) ) 

#define ITextFont_GetShadow(This,pValue)	\
    ( (This)->lpVtbl -> GetShadow(This,pValue) ) 

#define ITextFont_SetShadow(This,Value)	\
    ( (This)->lpVtbl -> SetShadow(This,Value) ) 

#define ITextFont_GetSize(This,pValue)	\
    ( (This)->lpVtbl -> GetSize(This,pValue) ) 

#define ITextFont_SetSize(This,Value)	\
    ( (This)->lpVtbl -> SetSize(This,Value) ) 

#define ITextFont_GetSmallCaps(This,pValue)	\
    ( (This)->lpVtbl -> GetSmallCaps(This,pValue) ) 

#define ITextFont_SetSmallCaps(This,Value)	\
    ( (This)->lpVtbl -> SetSmallCaps(This,Value) ) 

#define ITextFont_GetSpacing(This,pValue)	\
    ( (This)->lpVtbl -> GetSpacing(This,pValue) ) 

#define ITextFont_SetSpacing(This,Value)	\
    ( (This)->lpVtbl -> SetSpacing(This,Value) ) 

#define ITextFont_GetStrikeThrough(This,pValue)	\
    ( (This)->lpVtbl -> GetStrikeThrough(This,pValue) ) 

#define ITextFont_SetStrikeThrough(This,Value)	\
    ( (This)->lpVtbl -> SetStrikeThrough(This,Value) ) 

#define ITextFont_GetSubscript(This,pValue)	\
    ( (This)->lpVtbl -> GetSubscript(This,pValue) ) 

#define ITextFont_SetSubscript(This,Value)	\
    ( (This)->lpVtbl -> SetSubscript(This,Value) ) 

#define ITextFont_GetSuperscript(This,pValue)	\
    ( (This)->lpVtbl -> GetSuperscript(This,pValue) ) 

#define ITextFont_SetSuperscript(This,Value)	\
    ( (This)->lpVtbl -> SetSuperscript(This,Value) ) 

#define ITextFont_GetUnderline(This,pValue)	\
    ( (This)->lpVtbl -> GetUnderline(This,pValue) ) 

#define ITextFont_SetUnderline(This,Value)	\
    ( (This)->lpVtbl -> SetUnderline(This,Value) ) 

#define ITextFont_GetWeight(This,pValue)	\
    ( (This)->lpVtbl -> GetWeight(This,pValue) ) 

#define ITextFont_SetWeight(This,Value)	\
    ( (This)->lpVtbl -> SetWeight(This,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextFont_INTERFACE_DEFINED__ */


#ifndef __ITextPara_INTERFACE_DEFINED__
#define __ITextPara_INTERFACE_DEFINED__

/* interface ITextPara */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextPara;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CC497C4-A1DF-11ce-8098-00AA0047BE5D")
    ITextPara : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDuplicate( 
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetDuplicate( 
            /* [in] */ __RPC__in_opt ITextPara *pPara) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CanChange( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ __RPC__in_opt ITextPara *pPara,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStyle( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetStyle( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAlignment( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAlignment( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetHyphenation( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetHyphenation( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetFirstLineIndent( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetKeepTogether( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetKeepTogether( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetKeepWithNext( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetKeepWithNext( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetLeftIndent( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetLineSpacing( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetLineSpacingRule( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetListAlignment( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetListAlignment( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetListLevelIndex( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetListLevelIndex( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetListStart( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetListStart( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetListTab( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetListTab( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetListType( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetListType( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetNoLineNumber( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetNoLineNumber( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetPageBreakBefore( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetPageBreakBefore( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetRightIndent( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetRightIndent( 
            /* [in] */ float Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetIndents( 
            /* [in] */ float First,
            /* [in] */ float Left,
            /* [in] */ float Right) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetLineSpacing( 
            /* [in] */ long Rule,
            /* [in] */ float Spacing) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSpaceAfter( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSpaceAfter( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSpaceBefore( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSpaceBefore( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetWidowControl( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetWidowControl( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetTabCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddTab( 
            /* [in] */ float tbPos,
            /* [in] */ long tbAlign,
            /* [in] */ long tbLeader) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ClearAllTabs( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteTab( 
            /* [in] */ float tbPos) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetTab( 
            /* [in] */ long iTab,
            /* [out] */ __RPC__out float *ptbPos,
            /* [out] */ __RPC__out long *ptbAlign,
            /* [out] */ __RPC__out long *ptbLeader) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextParaVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextPara * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextPara * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextPara * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextPara * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextPara * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextPara * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextPara * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDuplicate )( 
            __RPC__in ITextPara * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanChange )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextPara * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStyle )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStyle )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAlignment )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAlignment )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetHyphenation )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetHyphenation )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFirstLineIndent )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKeepTogether )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKeepTogether )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKeepWithNext )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKeepWithNext )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLeftIndent )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLineSpacing )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLineSpacingRule )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListAlignment )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListAlignment )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListLevelIndex )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListLevelIndex )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListStart )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListStart )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListTab )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListTab )( 
            __RPC__in ITextPara * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListType )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListType )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetNoLineNumber )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetNoLineNumber )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPageBreakBefore )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPageBreakBefore )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetRightIndent )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetRightIndent )( 
            __RPC__in ITextPara * This,
            /* [in] */ float Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetIndents )( 
            __RPC__in ITextPara * This,
            /* [in] */ float First,
            /* [in] */ float Left,
            /* [in] */ float Right);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetLineSpacing )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Rule,
            /* [in] */ float Spacing);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSpaceAfter )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSpaceAfter )( 
            __RPC__in ITextPara * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSpaceBefore )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSpaceBefore )( 
            __RPC__in ITextPara * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetWidowControl )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetWidowControl )( 
            __RPC__in ITextPara * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetTabCount )( 
            __RPC__in ITextPara * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddTab )( 
            __RPC__in ITextPara * This,
            /* [in] */ float tbPos,
            /* [in] */ long tbAlign,
            /* [in] */ long tbLeader);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ClearAllTabs )( 
            __RPC__in ITextPara * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteTab )( 
            __RPC__in ITextPara * This,
            /* [in] */ float tbPos);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetTab )( 
            __RPC__in ITextPara * This,
            /* [in] */ long iTab,
            /* [out] */ __RPC__out float *ptbPos,
            /* [out] */ __RPC__out long *ptbAlign,
            /* [out] */ __RPC__out long *ptbLeader);
        
        END_INTERFACE
    } ITextParaVtbl;

    interface ITextPara
    {
        CONST_VTBL struct ITextParaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextPara_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextPara_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextPara_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextPara_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextPara_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextPara_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextPara_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextPara_GetDuplicate(This,ppPara)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppPara) ) 

#define ITextPara_SetDuplicate(This,pPara)	\
    ( (This)->lpVtbl -> SetDuplicate(This,pPara) ) 

#define ITextPara_CanChange(This,pValue)	\
    ( (This)->lpVtbl -> CanChange(This,pValue) ) 

#define ITextPara_IsEqual(This,pPara,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pPara,pValue) ) 

#define ITextPara_Reset(This,Value)	\
    ( (This)->lpVtbl -> Reset(This,Value) ) 

#define ITextPara_GetStyle(This,pValue)	\
    ( (This)->lpVtbl -> GetStyle(This,pValue) ) 

#define ITextPara_SetStyle(This,Value)	\
    ( (This)->lpVtbl -> SetStyle(This,Value) ) 

#define ITextPara_GetAlignment(This,pValue)	\
    ( (This)->lpVtbl -> GetAlignment(This,pValue) ) 

#define ITextPara_SetAlignment(This,Value)	\
    ( (This)->lpVtbl -> SetAlignment(This,Value) ) 

#define ITextPara_GetHyphenation(This,pValue)	\
    ( (This)->lpVtbl -> GetHyphenation(This,pValue) ) 

#define ITextPara_SetHyphenation(This,Value)	\
    ( (This)->lpVtbl -> SetHyphenation(This,Value) ) 

#define ITextPara_GetFirstLineIndent(This,pValue)	\
    ( (This)->lpVtbl -> GetFirstLineIndent(This,pValue) ) 

#define ITextPara_GetKeepTogether(This,pValue)	\
    ( (This)->lpVtbl -> GetKeepTogether(This,pValue) ) 

#define ITextPara_SetKeepTogether(This,Value)	\
    ( (This)->lpVtbl -> SetKeepTogether(This,Value) ) 

#define ITextPara_GetKeepWithNext(This,pValue)	\
    ( (This)->lpVtbl -> GetKeepWithNext(This,pValue) ) 

#define ITextPara_SetKeepWithNext(This,Value)	\
    ( (This)->lpVtbl -> SetKeepWithNext(This,Value) ) 

#define ITextPara_GetLeftIndent(This,pValue)	\
    ( (This)->lpVtbl -> GetLeftIndent(This,pValue) ) 

#define ITextPara_GetLineSpacing(This,pValue)	\
    ( (This)->lpVtbl -> GetLineSpacing(This,pValue) ) 

#define ITextPara_GetLineSpacingRule(This,pValue)	\
    ( (This)->lpVtbl -> GetLineSpacingRule(This,pValue) ) 

#define ITextPara_GetListAlignment(This,pValue)	\
    ( (This)->lpVtbl -> GetListAlignment(This,pValue) ) 

#define ITextPara_SetListAlignment(This,Value)	\
    ( (This)->lpVtbl -> SetListAlignment(This,Value) ) 

#define ITextPara_GetListLevelIndex(This,pValue)	\
    ( (This)->lpVtbl -> GetListLevelIndex(This,pValue) ) 

#define ITextPara_SetListLevelIndex(This,Value)	\
    ( (This)->lpVtbl -> SetListLevelIndex(This,Value) ) 

#define ITextPara_GetListStart(This,pValue)	\
    ( (This)->lpVtbl -> GetListStart(This,pValue) ) 

#define ITextPara_SetListStart(This,Value)	\
    ( (This)->lpVtbl -> SetListStart(This,Value) ) 

#define ITextPara_GetListTab(This,pValue)	\
    ( (This)->lpVtbl -> GetListTab(This,pValue) ) 

#define ITextPara_SetListTab(This,Value)	\
    ( (This)->lpVtbl -> SetListTab(This,Value) ) 

#define ITextPara_GetListType(This,pValue)	\
    ( (This)->lpVtbl -> GetListType(This,pValue) ) 

#define ITextPara_SetListType(This,Value)	\
    ( (This)->lpVtbl -> SetListType(This,Value) ) 

#define ITextPara_GetNoLineNumber(This,pValue)	\
    ( (This)->lpVtbl -> GetNoLineNumber(This,pValue) ) 

#define ITextPara_SetNoLineNumber(This,Value)	\
    ( (This)->lpVtbl -> SetNoLineNumber(This,Value) ) 

#define ITextPara_GetPageBreakBefore(This,pValue)	\
    ( (This)->lpVtbl -> GetPageBreakBefore(This,pValue) ) 

#define ITextPara_SetPageBreakBefore(This,Value)	\
    ( (This)->lpVtbl -> SetPageBreakBefore(This,Value) ) 

#define ITextPara_GetRightIndent(This,pValue)	\
    ( (This)->lpVtbl -> GetRightIndent(This,pValue) ) 

#define ITextPara_SetRightIndent(This,Value)	\
    ( (This)->lpVtbl -> SetRightIndent(This,Value) ) 

#define ITextPara_SetIndents(This,First,Left,Right)	\
    ( (This)->lpVtbl -> SetIndents(This,First,Left,Right) ) 

#define ITextPara_SetLineSpacing(This,Rule,Spacing)	\
    ( (This)->lpVtbl -> SetLineSpacing(This,Rule,Spacing) ) 

#define ITextPara_GetSpaceAfter(This,pValue)	\
    ( (This)->lpVtbl -> GetSpaceAfter(This,pValue) ) 

#define ITextPara_SetSpaceAfter(This,Value)	\
    ( (This)->lpVtbl -> SetSpaceAfter(This,Value) ) 

#define ITextPara_GetSpaceBefore(This,pValue)	\
    ( (This)->lpVtbl -> GetSpaceBefore(This,pValue) ) 

#define ITextPara_SetSpaceBefore(This,Value)	\
    ( (This)->lpVtbl -> SetSpaceBefore(This,Value) ) 

#define ITextPara_GetWidowControl(This,pValue)	\
    ( (This)->lpVtbl -> GetWidowControl(This,pValue) ) 

#define ITextPara_SetWidowControl(This,Value)	\
    ( (This)->lpVtbl -> SetWidowControl(This,Value) ) 

#define ITextPara_GetTabCount(This,pCount)	\
    ( (This)->lpVtbl -> GetTabCount(This,pCount) ) 

#define ITextPara_AddTab(This,tbPos,tbAlign,tbLeader)	\
    ( (This)->lpVtbl -> AddTab(This,tbPos,tbAlign,tbLeader) ) 

#define ITextPara_ClearAllTabs(This)	\
    ( (This)->lpVtbl -> ClearAllTabs(This) ) 

#define ITextPara_DeleteTab(This,tbPos)	\
    ( (This)->lpVtbl -> DeleteTab(This,tbPos) ) 

#define ITextPara_GetTab(This,iTab,ptbPos,ptbAlign,ptbLeader)	\
    ( (This)->lpVtbl -> GetTab(This,iTab,ptbPos,ptbAlign,ptbLeader) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextPara_INTERFACE_DEFINED__ */


#ifndef __ITextStoryRanges_INTERFACE_DEFINED__
#define __ITextStoryRanges_INTERFACE_DEFINED__

/* interface ITextStoryRanges */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextStoryRanges;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CC497C5-A1DF-11ce-8098-00AA0047BE5D")
    ITextStoryRanges : public IDispatch
    {
    public:
        virtual /* [restricted][id] */ HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunkEnum) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoryRangesVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoryRanges * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoryRanges * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoryRanges * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextStoryRanges * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextStoryRanges * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextStoryRanges * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextStoryRanges * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [restricted][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ITextStoryRanges * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunkEnum);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in ITextStoryRanges * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ITextStoryRanges * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        END_INTERFACE
    } ITextStoryRangesVtbl;

    interface ITextStoryRanges
    {
        CONST_VTBL struct ITextStoryRangesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoryRanges_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoryRanges_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoryRanges_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoryRanges_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextStoryRanges_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextStoryRanges_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextStoryRanges_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextStoryRanges__NewEnum(This,ppunkEnum)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunkEnum) ) 

#define ITextStoryRanges_Item(This,Index,ppRange)	\
    ( (This)->lpVtbl -> Item(This,Index,ppRange) ) 

#define ITextStoryRanges_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoryRanges_INTERFACE_DEFINED__ */


#ifndef __ITextDocument2_INTERFACE_DEFINED__
#define __ITextDocument2_INTERFACE_DEFINED__

/* interface ITextDocument2 */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextDocument2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5E0-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextDocument2 : public ITextDocument
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetCaretType( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE SetCaretType( 
            /* [in] */ long Value) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetDisplays( 
            /* [retval][out] */ __RPC__deref_out_opt ITextDisplays **ppDisplays) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetDocumentFont( 
            /* [retval][out] */ __RPC__deref_out_opt ITextFont2 **ppFont) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE SetDocumentFont( 
            /* [in] */ __RPC__in_opt ITextFont2 *pFont) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetDocumentPara( 
            /* [retval][out] */ __RPC__deref_out_opt ITextPara2 **ppPara) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE SetDocumentPara( 
            /* [in] */ __RPC__in_opt ITextPara2 *pPara) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetEastAsianFlags( 
            /* [retval][out] */ __RPC__out long *pFlags) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetGenerator( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE SetIMEInProgress( 
            /* [in] */ long Value) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetNotificationMode( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE SetNotificationMode( 
            /* [in] */ long Value) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetSelection2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection2 **ppSel) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetStoryRanges2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextStoryRanges2 **ppStories) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetTypographyOptions( 
            /* [retval][out] */ __RPC__out long *pOptions) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetWindow( 
            /* [retval][out] */ __RPC__out __int64 *pHwnd) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AttachMsgFilter( 
            /* [in] */ __RPC__in_opt IUnknown *pFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CheckTextLimit( 
            long cch,
            __RPC__in long *pcch) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCallManager( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVoid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetClientRect( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pRight,
            /* [out] */ __RPC__out long *pBottom) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEffectColor( 
            /* [in] */ long Index,
            /* [out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetImmContext( 
            /* [retval][out] */ __RPC__out __int64 *pContext) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPreferredFont( 
            /* [in] */ long cp,
            /* [in] */ long CharRep,
            /* [in] */ long Options,
            /* [in] */ long curCharRep,
            /* [in] */ long curFontSize,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr,
            /* [out] */ __RPC__out long *pPitchAndFamily,
            /* [out] */ __RPC__out long *pNewFontSize) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetStrings( 
            /* [out] */ __RPC__deref_out_opt ITextStrings **ppStrs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ long Notify) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Range2( 
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RangeFromPoint2( 
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ long Type,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseCallManager( 
            /* [in] */ __RPC__in_opt IUnknown *pVoid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseImmContext( 
            /* [in] */ __int64 Context) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetEffectColor( 
            /* [in] */ long Index,
            /* [in] */ long Value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ long Type,
            /* [in] */ long Value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetTypographyOptions( 
            /* [in] */ long Options,
            /* [in] */ long Mask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SysBeep( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ long Value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateWindow( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMathProperties( 
            /* [out] */ __RPC__out long *pOptions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetMathProperties( 
            /* [in] */ long Options,
            /* [in] */ long Mask) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetActiveStory( 
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetActiveStory( 
            /* [in] */ __RPC__in_opt ITextStory *pStory) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetMainStory( 
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetNewStory( 
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetStory( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextDocument2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextDocument2 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextDocument2 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextDocument2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextDocument2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection **ppSel);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryCount )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryRanges )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextStoryRanges **ppStories);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSaved )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSaved )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultTabStop )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultTabStop )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ float Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *New )( 
            __RPC__in ITextDocument2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Freeze )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Unfreeze )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BeginEditCollection )( 
            __RPC__in ITextDocument2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndEditCollection )( 
            __RPC__in ITextDocument2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Undo )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Redo )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Range )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RangeFromPoint )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCaretType )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCaretType )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Value);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDisplays )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextDisplays **ppDisplays);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentFont )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont2 **ppFont);
        
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDocumentFont )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in_opt ITextFont2 *pFont);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentPara )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara2 **ppPara);
        
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDocumentPara )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in_opt ITextPara2 *pPara);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEastAsianFlags )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pFlags);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetGenerator )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetIMEInProgress )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Value);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetNotificationMode )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetNotificationMode )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Value);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSelection2 )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection2 **ppSel);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryRanges2 )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextStoryRanges2 **ppStories);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetTypographyOptions )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pOptions);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out __int64 *pHwnd);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AttachMsgFilter )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in_opt IUnknown *pFilter);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CheckTextLimit )( 
            __RPC__in ITextDocument2 * This,
            long cch,
            __RPC__in long *pcch);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCallManager )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVoid);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetClientRect )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pRight,
            /* [out] */ __RPC__out long *pBottom);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEffectColor )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Index,
            /* [out] */ __RPC__out long *pValue);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetImmContext )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__out __int64 *pContext);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPreferredFont )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long cp,
            /* [in] */ long CharRep,
            /* [in] */ long Options,
            /* [in] */ long curCharRep,
            /* [in] */ long curFontSize,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr,
            /* [out] */ __RPC__out long *pPitchAndFamily,
            /* [out] */ __RPC__out long *pNewFontSize);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStrings )( 
            __RPC__in ITextDocument2 * This,
            /* [out] */ __RPC__deref_out_opt ITextStrings **ppStrs);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Notify);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Range2 )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RangeFromPoint2 )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ long Type,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseCallManager )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in_opt IUnknown *pVoid);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseImmContext )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __int64 Context);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetEffectColor )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Index,
            /* [in] */ long Value);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Type,
            /* [in] */ long Value);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetTypographyOptions )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Options,
            /* [in] */ long Mask);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SysBeep )( 
            __RPC__in ITextDocument2 * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Value);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateWindow )( 
            __RPC__in ITextDocument2 * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMathProperties )( 
            __RPC__in ITextDocument2 * This,
            /* [out] */ __RPC__out long *pOptions);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetMathProperties )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Options,
            /* [in] */ long Mask);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetActiveStory )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetActiveStory )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ __RPC__in_opt ITextStory *pStory);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetMainStory )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetNewStory )( 
            __RPC__in ITextDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetStory )( 
            __RPC__in ITextDocument2 * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextStory **ppStory);
        
        END_INTERFACE
    } ITextDocument2Vtbl;

    interface ITextDocument2
    {
        CONST_VTBL struct ITextDocument2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextDocument2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextDocument2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextDocument2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextDocument2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextDocument2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextDocument2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextDocument2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextDocument2_GetName(This,pName)	\
    ( (This)->lpVtbl -> GetName(This,pName) ) 

#define ITextDocument2_GetSelection(This,ppSel)	\
    ( (This)->lpVtbl -> GetSelection(This,ppSel) ) 

#define ITextDocument2_GetStoryCount(This,pCount)	\
    ( (This)->lpVtbl -> GetStoryCount(This,pCount) ) 

#define ITextDocument2_GetStoryRanges(This,ppStories)	\
    ( (This)->lpVtbl -> GetStoryRanges(This,ppStories) ) 

#define ITextDocument2_GetSaved(This,pValue)	\
    ( (This)->lpVtbl -> GetSaved(This,pValue) ) 

#define ITextDocument2_SetSaved(This,Value)	\
    ( (This)->lpVtbl -> SetSaved(This,Value) ) 

#define ITextDocument2_GetDefaultTabStop(This,pValue)	\
    ( (This)->lpVtbl -> GetDefaultTabStop(This,pValue) ) 

#define ITextDocument2_SetDefaultTabStop(This,Value)	\
    ( (This)->lpVtbl -> SetDefaultTabStop(This,Value) ) 

#define ITextDocument2_New(This)	\
    ( (This)->lpVtbl -> New(This) ) 

#define ITextDocument2_Open(This,pVar,Flags,CodePage)	\
    ( (This)->lpVtbl -> Open(This,pVar,Flags,CodePage) ) 

#define ITextDocument2_Save(This,pVar,Flags,CodePage)	\
    ( (This)->lpVtbl -> Save(This,pVar,Flags,CodePage) ) 

#define ITextDocument2_Freeze(This,pCount)	\
    ( (This)->lpVtbl -> Freeze(This,pCount) ) 

#define ITextDocument2_Unfreeze(This,pCount)	\
    ( (This)->lpVtbl -> Unfreeze(This,pCount) ) 

#define ITextDocument2_BeginEditCollection(This)	\
    ( (This)->lpVtbl -> BeginEditCollection(This) ) 

#define ITextDocument2_EndEditCollection(This)	\
    ( (This)->lpVtbl -> EndEditCollection(This) ) 

#define ITextDocument2_Undo(This,Count,pCount)	\
    ( (This)->lpVtbl -> Undo(This,Count,pCount) ) 

#define ITextDocument2_Redo(This,Count,pCount)	\
    ( (This)->lpVtbl -> Redo(This,Count,pCount) ) 

#define ITextDocument2_Range(This,cpActive,cpAnchor,ppRange)	\
    ( (This)->lpVtbl -> Range(This,cpActive,cpAnchor,ppRange) ) 

#define ITextDocument2_RangeFromPoint(This,x,y,ppRange)	\
    ( (This)->lpVtbl -> RangeFromPoint(This,x,y,ppRange) ) 


#define ITextDocument2_GetCaretType(This,pValue)	\
    ( (This)->lpVtbl -> GetCaretType(This,pValue) ) 

#define ITextDocument2_SetCaretType(This,Value)	\
    ( (This)->lpVtbl -> SetCaretType(This,Value) ) 

#define ITextDocument2_GetDisplays(This,ppDisplays)	\
    ( (This)->lpVtbl -> GetDisplays(This,ppDisplays) ) 

#define ITextDocument2_GetDocumentFont(This,ppFont)	\
    ( (This)->lpVtbl -> GetDocumentFont(This,ppFont) ) 

#define ITextDocument2_SetDocumentFont(This,pFont)	\
    ( (This)->lpVtbl -> SetDocumentFont(This,pFont) ) 

#define ITextDocument2_GetDocumentPara(This,ppPara)	\
    ( (This)->lpVtbl -> GetDocumentPara(This,ppPara) ) 

#define ITextDocument2_SetDocumentPara(This,pPara)	\
    ( (This)->lpVtbl -> SetDocumentPara(This,pPara) ) 

#define ITextDocument2_GetEastAsianFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetEastAsianFlags(This,pFlags) ) 

#define ITextDocument2_GetGenerator(This,pbstr)	\
    ( (This)->lpVtbl -> GetGenerator(This,pbstr) ) 

#define ITextDocument2_SetIMEInProgress(This,Value)	\
    ( (This)->lpVtbl -> SetIMEInProgress(This,Value) ) 

#define ITextDocument2_GetNotificationMode(This,pValue)	\
    ( (This)->lpVtbl -> GetNotificationMode(This,pValue) ) 

#define ITextDocument2_SetNotificationMode(This,Value)	\
    ( (This)->lpVtbl -> SetNotificationMode(This,Value) ) 

#define ITextDocument2_GetSelection2(This,ppSel)	\
    ( (This)->lpVtbl -> GetSelection2(This,ppSel) ) 

#define ITextDocument2_GetStoryRanges2(This,ppStories)	\
    ( (This)->lpVtbl -> GetStoryRanges2(This,ppStories) ) 

#define ITextDocument2_GetTypographyOptions(This,pOptions)	\
    ( (This)->lpVtbl -> GetTypographyOptions(This,pOptions) ) 

#define ITextDocument2_GetVersion(This,pValue)	\
    ( (This)->lpVtbl -> GetVersion(This,pValue) ) 

#define ITextDocument2_GetWindow(This,pHwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,pHwnd) ) 

#define ITextDocument2_AttachMsgFilter(This,pFilter)	\
    ( (This)->lpVtbl -> AttachMsgFilter(This,pFilter) ) 

#define ITextDocument2_CheckTextLimit(This,cch,pcch)	\
    ( (This)->lpVtbl -> CheckTextLimit(This,cch,pcch) ) 

#define ITextDocument2_GetCallManager(This,ppVoid)	\
    ( (This)->lpVtbl -> GetCallManager(This,ppVoid) ) 

#define ITextDocument2_GetClientRect(This,Type,pLeft,pTop,pRight,pBottom)	\
    ( (This)->lpVtbl -> GetClientRect(This,Type,pLeft,pTop,pRight,pBottom) ) 

#define ITextDocument2_GetEffectColor(This,Index,pValue)	\
    ( (This)->lpVtbl -> GetEffectColor(This,Index,pValue) ) 

#define ITextDocument2_GetImmContext(This,pContext)	\
    ( (This)->lpVtbl -> GetImmContext(This,pContext) ) 

#define ITextDocument2_GetPreferredFont(This,cp,CharRep,Options,curCharRep,curFontSize,pbstr,pPitchAndFamily,pNewFontSize)	\
    ( (This)->lpVtbl -> GetPreferredFont(This,cp,CharRep,Options,curCharRep,curFontSize,pbstr,pPitchAndFamily,pNewFontSize) ) 

#define ITextDocument2_GetProperty(This,Type,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Type,pValue) ) 

#define ITextDocument2_GetStrings(This,ppStrs)	\
    ( (This)->lpVtbl -> GetStrings(This,ppStrs) ) 

#define ITextDocument2_Notify(This,Notify)	\
    ( (This)->lpVtbl -> Notify(This,Notify) ) 

#define ITextDocument2_Range2(This,cpActive,cpAnchor,ppRange)	\
    ( (This)->lpVtbl -> Range2(This,cpActive,cpAnchor,ppRange) ) 

#define ITextDocument2_RangeFromPoint2(This,x,y,Type,ppRange)	\
    ( (This)->lpVtbl -> RangeFromPoint2(This,x,y,Type,ppRange) ) 

#define ITextDocument2_ReleaseCallManager(This,pVoid)	\
    ( (This)->lpVtbl -> ReleaseCallManager(This,pVoid) ) 

#define ITextDocument2_ReleaseImmContext(This,Context)	\
    ( (This)->lpVtbl -> ReleaseImmContext(This,Context) ) 

#define ITextDocument2_SetEffectColor(This,Index,Value)	\
    ( (This)->lpVtbl -> SetEffectColor(This,Index,Value) ) 

#define ITextDocument2_SetProperty(This,Type,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Type,Value) ) 

#define ITextDocument2_SetTypographyOptions(This,Options,Mask)	\
    ( (This)->lpVtbl -> SetTypographyOptions(This,Options,Mask) ) 

#define ITextDocument2_SysBeep(This)	\
    ( (This)->lpVtbl -> SysBeep(This) ) 

#define ITextDocument2_Update(This,Value)	\
    ( (This)->lpVtbl -> Update(This,Value) ) 

#define ITextDocument2_UpdateWindow(This)	\
    ( (This)->lpVtbl -> UpdateWindow(This) ) 

#define ITextDocument2_GetMathProperties(This,pOptions)	\
    ( (This)->lpVtbl -> GetMathProperties(This,pOptions) ) 

#define ITextDocument2_SetMathProperties(This,Options,Mask)	\
    ( (This)->lpVtbl -> SetMathProperties(This,Options,Mask) ) 

#define ITextDocument2_GetActiveStory(This,ppStory)	\
    ( (This)->lpVtbl -> GetActiveStory(This,ppStory) ) 

#define ITextDocument2_SetActiveStory(This,pStory)	\
    ( (This)->lpVtbl -> SetActiveStory(This,pStory) ) 

#define ITextDocument2_GetMainStory(This,ppStory)	\
    ( (This)->lpVtbl -> GetMainStory(This,ppStory) ) 

#define ITextDocument2_GetNewStory(This,ppStory)	\
    ( (This)->lpVtbl -> GetNewStory(This,ppStory) ) 

#define ITextDocument2_GetStory(This,Index,ppStory)	\
    ( (This)->lpVtbl -> GetStory(This,Index,ppStory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextDocument2_INTERFACE_DEFINED__ */


#ifndef __ITextRange2_INTERFACE_DEFINED__
#define __ITextRange2_INTERFACE_DEFINED__

/* interface ITextRange2 */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextRange2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5E2-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextRange2 : public ITextSelection
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCch( 
            /* [retval][out] */ __RPC__out long *pcch) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCells( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppCells) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetColumn( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppColumn) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDuplicate2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetFont2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextFont2 **ppFont) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetFont2( 
            /* [in] */ __RPC__in_opt ITextFont2 *pFont) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetFormattedText2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetFormattedText2( 
            /* [in] */ __RPC__in_opt ITextRange2 *pRange) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetGravity( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetGravity( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetPara2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextPara2 **ppPara) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetPara2( 
            /* [in] */ __RPC__in_opt ITextPara2 *pPara) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetRow( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRow **ppRow) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetStartPara( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetTable( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppTable) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetURL( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetURL( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddSubrange( 
            /* [in] */ long cp1,
            /* [in] */ long cp2,
            /* [in] */ long Activate) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE BuildUpMath( 
            /* [in] */ long Flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteSubrange( 
            /* [in] */ long cpFirst,
            /* [in] */ long cpLim) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Find( 
            /* [in] */ __RPC__in_opt ITextRange2 *pRange,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [out] */ __RPC__out long *pDelta) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetChar2( 
            /* [out] */ __RPC__out long *pChar,
            /* [in] */ long Offset) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetDropCap( 
            /* [out] */ __RPC__out long *pcLine,
            /* [out] */ __RPC__out long *pPosition) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetInlineObject( 
            /* [out] */ __RPC__out long *pType,
            /* [out] */ __RPC__out long *pAlign,
            /* [out] */ __RPC__out long *pChar,
            /* [out] */ __RPC__out long *pChar1,
            /* [out] */ __RPC__out long *pChar2,
            /* [out] */ __RPC__out long *pCount,
            /* [out] */ __RPC__out long *pTeXStyle,
            /* [out] */ __RPC__out long *pcCol,
            /* [out] */ __RPC__out long *pLevel) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetRect( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pRight,
            /* [out] */ __RPC__out long *pBottom,
            /* [out] */ __RPC__out long *pHit) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetSubrange( 
            /* [in] */ long iSubrange,
            /* [out] */ __RPC__out long *pcpFirst,
            /* [out] */ __RPC__out long *pcpLim) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetText2( 
            /* [in] */ long Flags,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE HexToUnicode( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE InsertTable( 
            /* [in] */ long cCol,
            /* [in] */ long cRow,
            /* [in] */ long AutoFit) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Linearize( 
            /* [in] */ long Flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetActiveSubrange( 
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetDropCap( 
            /* [in] */ long cLine,
            /* [in] */ long Position) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ long Type,
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetText2( 
            /* [in] */ long Flags,
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE UnicodeToHex( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetInlineObject( 
            /* [in] */ long Type,
            /* [in] */ long Align,
            /* [in] */ long Char,
            /* [in] */ long Char1,
            /* [in] */ long Char2,
            /* [in] */ long Count,
            /* [in] */ long TeXStyle,
            /* [in] */ long cCol) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetMathFunctionType( 
            /* [in] */ __RPC__in BSTR bstr,
            /* [out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE InsertImage( 
            /* [in] */ long width,
            /* [in] */ long height,
            /* [in] */ long ascent,
            /* [in] */ long Type,
            /* [in] */ __RPC__in BSTR bstrAltText,
            /* [in] */ __RPC__in_opt IStream *pStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextRange2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextRange2 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextRange2 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextRange2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextRange2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetChar )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pChar);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetChar )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Char);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStart )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pcpFirst);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStart )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cpFirst);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEnd )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pcpLim);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEnd )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cpLim);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFont )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFont )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPara )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPara )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryLength )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryType )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Collapse )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long bStart);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pIndex);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetIndex )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Index,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRange )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InRange )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InStory )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ITextRange2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StartOf )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndOf )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStart )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEnd )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveWhile )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartWhile )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndWhile )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveUntil )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartUntil )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndUntil )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindText )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextStart )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextEnd )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cut )( 
            __RPC__in ITextRange2 * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in ITextRange2 * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Paste )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanPaste )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanEdit )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ChangeCase )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Type);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPoint )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *px,
            /* [out] */ __RPC__out long *py);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPoint )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ long Type,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ScrollIntoView )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEmbeddedObject )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pFlags);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Flags);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pType);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveLeft )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveRight )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveUp )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveDown )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HomeKey )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndKey )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *TypeText )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCch )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pcch);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCells )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppCells);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetColumn )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppColumn);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate2 )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFont2 )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont2 **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFont2 )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextFont2 *pFont);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFormattedText2 )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText2 )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetGravity )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetGravity )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPara2 )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara2 **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPara2 )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextPara2 *pPara);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetRow )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRow **ppRow);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStartPara )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetTable )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppTable);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetURL )( 
            __RPC__in ITextRange2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetURL )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSubrange )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cp1,
            /* [in] */ long cp2,
            /* [in] */ long Activate);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildUpMath )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Flags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteSubrange )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cpFirst,
            /* [in] */ long cpLim);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Find )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetChar2 )( 
            __RPC__in ITextRange2 * This,
            /* [out] */ __RPC__out long *pChar,
            /* [in] */ long Offset);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetDropCap )( 
            __RPC__in ITextRange2 * This,
            /* [out] */ __RPC__out long *pcLine,
            /* [out] */ __RPC__out long *pPosition);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInlineObject )( 
            __RPC__in ITextRange2 * This,
            /* [out] */ __RPC__out long *pType,
            /* [out] */ __RPC__out long *pAlign,
            /* [out] */ __RPC__out long *pChar,
            /* [out] */ __RPC__out long *pChar1,
            /* [out] */ __RPC__out long *pChar2,
            /* [out] */ __RPC__out long *pCount,
            /* [out] */ __RPC__out long *pTeXStyle,
            /* [out] */ __RPC__out long *pcCol,
            /* [out] */ __RPC__out long *pLevel);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetRect )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pRight,
            /* [out] */ __RPC__out long *pBottom,
            /* [out] */ __RPC__out long *pHit);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetSubrange )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long iSubrange,
            /* [out] */ __RPC__out long *pcpFirst,
            /* [out] */ __RPC__out long *pcpLim);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetText2 )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Flags,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HexToUnicode )( 
            __RPC__in ITextRange2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InsertTable )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cCol,
            /* [in] */ long cRow,
            /* [in] */ long AutoFit);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Linearize )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Flags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActiveSubrange )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetDropCap )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long cLine,
            /* [in] */ long Position);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Type,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetText2 )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Flags,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *UnicodeToHex )( 
            __RPC__in ITextRange2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInlineObject )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long Type,
            /* [in] */ long Align,
            /* [in] */ long Char,
            /* [in] */ long Char1,
            /* [in] */ long Char2,
            /* [in] */ long Count,
            /* [in] */ long TeXStyle,
            /* [in] */ long cCol);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetMathFunctionType )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InsertImage )( 
            __RPC__in ITextRange2 * This,
            /* [in] */ long width,
            /* [in] */ long height,
            /* [in] */ long ascent,
            /* [in] */ long Type,
            /* [in] */ __RPC__in BSTR bstrAltText,
            /* [in] */ __RPC__in_opt IStream *pStream);
        
        END_INTERFACE
    } ITextRange2Vtbl;

    interface ITextRange2
    {
        CONST_VTBL struct ITextRange2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextRange2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextRange2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextRange2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextRange2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextRange2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextRange2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextRange2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextRange2_GetText(This,pbstr)	\
    ( (This)->lpVtbl -> GetText(This,pbstr) ) 

#define ITextRange2_SetText(This,bstr)	\
    ( (This)->lpVtbl -> SetText(This,bstr) ) 

#define ITextRange2_GetChar(This,pChar)	\
    ( (This)->lpVtbl -> GetChar(This,pChar) ) 

#define ITextRange2_SetChar(This,Char)	\
    ( (This)->lpVtbl -> SetChar(This,Char) ) 

#define ITextRange2_GetDuplicate(This,ppRange)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppRange) ) 

#define ITextRange2_GetFormattedText(This,ppRange)	\
    ( (This)->lpVtbl -> GetFormattedText(This,ppRange) ) 

#define ITextRange2_SetFormattedText(This,pRange)	\
    ( (This)->lpVtbl -> SetFormattedText(This,pRange) ) 

#define ITextRange2_GetStart(This,pcpFirst)	\
    ( (This)->lpVtbl -> GetStart(This,pcpFirst) ) 

#define ITextRange2_SetStart(This,cpFirst)	\
    ( (This)->lpVtbl -> SetStart(This,cpFirst) ) 

#define ITextRange2_GetEnd(This,pcpLim)	\
    ( (This)->lpVtbl -> GetEnd(This,pcpLim) ) 

#define ITextRange2_SetEnd(This,cpLim)	\
    ( (This)->lpVtbl -> SetEnd(This,cpLim) ) 

#define ITextRange2_GetFont(This,ppFont)	\
    ( (This)->lpVtbl -> GetFont(This,ppFont) ) 

#define ITextRange2_SetFont(This,pFont)	\
    ( (This)->lpVtbl -> SetFont(This,pFont) ) 

#define ITextRange2_GetPara(This,ppPara)	\
    ( (This)->lpVtbl -> GetPara(This,ppPara) ) 

#define ITextRange2_SetPara(This,pPara)	\
    ( (This)->lpVtbl -> SetPara(This,pPara) ) 

#define ITextRange2_GetStoryLength(This,pCount)	\
    ( (This)->lpVtbl -> GetStoryLength(This,pCount) ) 

#define ITextRange2_GetStoryType(This,pValue)	\
    ( (This)->lpVtbl -> GetStoryType(This,pValue) ) 

#define ITextRange2_Collapse(This,bStart)	\
    ( (This)->lpVtbl -> Collapse(This,bStart) ) 

#define ITextRange2_Expand(This,Unit,pDelta)	\
    ( (This)->lpVtbl -> Expand(This,Unit,pDelta) ) 

#define ITextRange2_GetIndex(This,Unit,pIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,Unit,pIndex) ) 

#define ITextRange2_SetIndex(This,Unit,Index,Extend)	\
    ( (This)->lpVtbl -> SetIndex(This,Unit,Index,Extend) ) 

#define ITextRange2_SetRange(This,cpAnchor,cpActive)	\
    ( (This)->lpVtbl -> SetRange(This,cpAnchor,cpActive) ) 

#define ITextRange2_InRange(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InRange(This,pRange,pValue) ) 

#define ITextRange2_InStory(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InStory(This,pRange,pValue) ) 

#define ITextRange2_IsEqual(This,pRange,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pRange,pValue) ) 

#define ITextRange2_Select(This)	\
    ( (This)->lpVtbl -> Select(This) ) 

#define ITextRange2_StartOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> StartOf(This,Unit,Extend,pDelta) ) 

#define ITextRange2_EndOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> EndOf(This,Unit,Extend,pDelta) ) 

#define ITextRange2_Move(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Move(This,Unit,Count,pDelta) ) 

#define ITextRange2_MoveStart(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStart(This,Unit,Count,pDelta) ) 

#define ITextRange2_MoveEnd(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEnd(This,Unit,Count,pDelta) ) 

#define ITextRange2_MoveWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveWhile(This,Cset,Count,pDelta) ) 

#define ITextRange2_MoveStartWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartWhile(This,Cset,Count,pDelta) ) 

#define ITextRange2_MoveEndWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndWhile(This,Cset,Count,pDelta) ) 

#define ITextRange2_MoveUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveUntil(This,Cset,Count,pDelta) ) 

#define ITextRange2_MoveStartUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartUntil(This,Cset,Count,pDelta) ) 

#define ITextRange2_MoveEndUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndUntil(This,Cset,Count,pDelta) ) 

#define ITextRange2_FindText(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindText(This,bstr,Count,Flags,pLength) ) 

#define ITextRange2_FindTextStart(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextStart(This,bstr,Count,Flags,pLength) ) 

#define ITextRange2_FindTextEnd(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextEnd(This,bstr,Count,Flags,pLength) ) 

#define ITextRange2_Delete(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Delete(This,Unit,Count,pDelta) ) 

#define ITextRange2_Cut(This,pVar)	\
    ( (This)->lpVtbl -> Cut(This,pVar) ) 

#define ITextRange2_Copy(This,pVar)	\
    ( (This)->lpVtbl -> Copy(This,pVar) ) 

#define ITextRange2_Paste(This,pVar,Format)	\
    ( (This)->lpVtbl -> Paste(This,pVar,Format) ) 

#define ITextRange2_CanPaste(This,pVar,Format,pValue)	\
    ( (This)->lpVtbl -> CanPaste(This,pVar,Format,pValue) ) 

#define ITextRange2_CanEdit(This,pValue)	\
    ( (This)->lpVtbl -> CanEdit(This,pValue) ) 

#define ITextRange2_ChangeCase(This,Type)	\
    ( (This)->lpVtbl -> ChangeCase(This,Type) ) 

#define ITextRange2_GetPoint(This,Type,px,py)	\
    ( (This)->lpVtbl -> GetPoint(This,Type,px,py) ) 

#define ITextRange2_SetPoint(This,x,y,Type,Extend)	\
    ( (This)->lpVtbl -> SetPoint(This,x,y,Type,Extend) ) 

#define ITextRange2_ScrollIntoView(This,Value)	\
    ( (This)->lpVtbl -> ScrollIntoView(This,Value) ) 

#define ITextRange2_GetEmbeddedObject(This,ppObject)	\
    ( (This)->lpVtbl -> GetEmbeddedObject(This,ppObject) ) 


#define ITextRange2_GetFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pFlags) ) 

#define ITextRange2_SetFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetFlags(This,Flags) ) 

#define ITextRange2_GetType(This,pType)	\
    ( (This)->lpVtbl -> GetType(This,pType) ) 

#define ITextRange2_MoveLeft(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveLeft(This,Unit,Count,Extend,pDelta) ) 

#define ITextRange2_MoveRight(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveRight(This,Unit,Count,Extend,pDelta) ) 

#define ITextRange2_MoveUp(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveUp(This,Unit,Count,Extend,pDelta) ) 

#define ITextRange2_MoveDown(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveDown(This,Unit,Count,Extend,pDelta) ) 

#define ITextRange2_HomeKey(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> HomeKey(This,Unit,Extend,pDelta) ) 

#define ITextRange2_EndKey(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> EndKey(This,Unit,Extend,pDelta) ) 

#define ITextRange2_TypeText(This,bstr)	\
    ( (This)->lpVtbl -> TypeText(This,bstr) ) 


#define ITextRange2_GetCch(This,pcch)	\
    ( (This)->lpVtbl -> GetCch(This,pcch) ) 

#define ITextRange2_GetCells(This,ppCells)	\
    ( (This)->lpVtbl -> GetCells(This,ppCells) ) 

#define ITextRange2_GetColumn(This,ppColumn)	\
    ( (This)->lpVtbl -> GetColumn(This,ppColumn) ) 

#define ITextRange2_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ITextRange2_GetDuplicate2(This,ppRange)	\
    ( (This)->lpVtbl -> GetDuplicate2(This,ppRange) ) 

#define ITextRange2_GetFont2(This,ppFont)	\
    ( (This)->lpVtbl -> GetFont2(This,ppFont) ) 

#define ITextRange2_SetFont2(This,pFont)	\
    ( (This)->lpVtbl -> SetFont2(This,pFont) ) 

#define ITextRange2_GetFormattedText2(This,ppRange)	\
    ( (This)->lpVtbl -> GetFormattedText2(This,ppRange) ) 

#define ITextRange2_SetFormattedText2(This,pRange)	\
    ( (This)->lpVtbl -> SetFormattedText2(This,pRange) ) 

#define ITextRange2_GetGravity(This,pValue)	\
    ( (This)->lpVtbl -> GetGravity(This,pValue) ) 

#define ITextRange2_SetGravity(This,Value)	\
    ( (This)->lpVtbl -> SetGravity(This,Value) ) 

#define ITextRange2_GetPara2(This,ppPara)	\
    ( (This)->lpVtbl -> GetPara2(This,ppPara) ) 

#define ITextRange2_SetPara2(This,pPara)	\
    ( (This)->lpVtbl -> SetPara2(This,pPara) ) 

#define ITextRange2_GetRow(This,ppRow)	\
    ( (This)->lpVtbl -> GetRow(This,ppRow) ) 

#define ITextRange2_GetStartPara(This,pValue)	\
    ( (This)->lpVtbl -> GetStartPara(This,pValue) ) 

#define ITextRange2_GetTable(This,ppTable)	\
    ( (This)->lpVtbl -> GetTable(This,ppTable) ) 

#define ITextRange2_GetURL(This,pbstr)	\
    ( (This)->lpVtbl -> GetURL(This,pbstr) ) 

#define ITextRange2_SetURL(This,bstr)	\
    ( (This)->lpVtbl -> SetURL(This,bstr) ) 

#define ITextRange2_AddSubrange(This,cp1,cp2,Activate)	\
    ( (This)->lpVtbl -> AddSubrange(This,cp1,cp2,Activate) ) 

#define ITextRange2_BuildUpMath(This,Flags)	\
    ( (This)->lpVtbl -> BuildUpMath(This,Flags) ) 

#define ITextRange2_DeleteSubrange(This,cpFirst,cpLim)	\
    ( (This)->lpVtbl -> DeleteSubrange(This,cpFirst,cpLim) ) 

#define ITextRange2_Find(This,pRange,Count,Flags,pDelta)	\
    ( (This)->lpVtbl -> Find(This,pRange,Count,Flags,pDelta) ) 

#define ITextRange2_GetChar2(This,pChar,Offset)	\
    ( (This)->lpVtbl -> GetChar2(This,pChar,Offset) ) 

#define ITextRange2_GetDropCap(This,pcLine,pPosition)	\
    ( (This)->lpVtbl -> GetDropCap(This,pcLine,pPosition) ) 

#define ITextRange2_GetInlineObject(This,pType,pAlign,pChar,pChar1,pChar2,pCount,pTeXStyle,pcCol,pLevel)	\
    ( (This)->lpVtbl -> GetInlineObject(This,pType,pAlign,pChar,pChar1,pChar2,pCount,pTeXStyle,pcCol,pLevel) ) 

#define ITextRange2_GetProperty(This,Type,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Type,pValue) ) 

#define ITextRange2_GetRect(This,Type,pLeft,pTop,pRight,pBottom,pHit)	\
    ( (This)->lpVtbl -> GetRect(This,Type,pLeft,pTop,pRight,pBottom,pHit) ) 

#define ITextRange2_GetSubrange(This,iSubrange,pcpFirst,pcpLim)	\
    ( (This)->lpVtbl -> GetSubrange(This,iSubrange,pcpFirst,pcpLim) ) 

#define ITextRange2_GetText2(This,Flags,pbstr)	\
    ( (This)->lpVtbl -> GetText2(This,Flags,pbstr) ) 

#define ITextRange2_HexToUnicode(This)	\
    ( (This)->lpVtbl -> HexToUnicode(This) ) 

#define ITextRange2_InsertTable(This,cCol,cRow,AutoFit)	\
    ( (This)->lpVtbl -> InsertTable(This,cCol,cRow,AutoFit) ) 

#define ITextRange2_Linearize(This,Flags)	\
    ( (This)->lpVtbl -> Linearize(This,Flags) ) 

#define ITextRange2_SetActiveSubrange(This,cpAnchor,cpActive)	\
    ( (This)->lpVtbl -> SetActiveSubrange(This,cpAnchor,cpActive) ) 

#define ITextRange2_SetDropCap(This,cLine,Position)	\
    ( (This)->lpVtbl -> SetDropCap(This,cLine,Position) ) 

#define ITextRange2_SetProperty(This,Type,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Type,Value) ) 

#define ITextRange2_SetText2(This,Flags,bstr)	\
    ( (This)->lpVtbl -> SetText2(This,Flags,bstr) ) 

#define ITextRange2_UnicodeToHex(This)	\
    ( (This)->lpVtbl -> UnicodeToHex(This) ) 

#define ITextRange2_SetInlineObject(This,Type,Align,Char,Char1,Char2,Count,TeXStyle,cCol)	\
    ( (This)->lpVtbl -> SetInlineObject(This,Type,Align,Char,Char1,Char2,Count,TeXStyle,cCol) ) 

#define ITextRange2_GetMathFunctionType(This,bstr,pValue)	\
    ( (This)->lpVtbl -> GetMathFunctionType(This,bstr,pValue) ) 

#define ITextRange2_InsertImage(This,width,height,ascent,Type,bstrAltText,pStream)	\
    ( (This)->lpVtbl -> InsertImage(This,width,height,ascent,Type,bstrAltText,pStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextRange2_INTERFACE_DEFINED__ */


#ifndef __ITextSelection2_INTERFACE_DEFINED__
#define __ITextSelection2_INTERFACE_DEFINED__

/* interface ITextSelection2 */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextSelection2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5E1-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextSelection2 : public ITextRange2
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITextSelection2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextSelection2 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextSelection2 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextSelection2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextSelection2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetChar )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pChar);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetChar )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Char);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStart )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pcpFirst);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStart )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cpFirst);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEnd )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pcpLim);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEnd )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cpLim);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFont )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFont )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPara )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPara )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryLength )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryType )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Collapse )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long bStart);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [retval][out] */ __RPC__out long *pIndex);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetIndex )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Index,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetRange )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InRange )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InStory )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextRange *pRange,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ITextSelection2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StartOf )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndOf )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStart )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEnd )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveWhile )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartWhile )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndWhile )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveUntil )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveStartUntil )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveEndUntil )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *Cset,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindText )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextStart )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *FindTextEnd )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__out long *pLength);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cut )( 
            __RPC__in ITextSelection2 * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in ITextSelection2 * This,
            /* [out] */ __RPC__out VARIANT *pVar);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Paste )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanPaste )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Format,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanEdit )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ChangeCase )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Type);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPoint )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *px,
            /* [out] */ __RPC__out long *py);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPoint )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [in] */ long Type,
            /* [in] */ long Extend);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ScrollIntoView )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEmbeddedObject )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pFlags);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Flags);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pType);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveLeft )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveRight )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveUp )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveDown )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Count,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HomeKey )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndKey )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Unit,
            /* [in] */ long Extend,
            /* [retval][out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *TypeText )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCch )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pcch);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCells )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppCells);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetColumn )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppColumn);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate2 )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFont2 )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont2 **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFont2 )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextFont2 *pFont);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFormattedText2 )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText2 )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetGravity )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetGravity )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPara2 )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara2 **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPara2 )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextPara2 *pPara);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetRow )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRow **ppRow);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStartPara )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetTable )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppTable);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetURL )( 
            __RPC__in ITextSelection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetURL )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddSubrange )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cp1,
            /* [in] */ long cp2,
            /* [in] */ long Activate);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BuildUpMath )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Flags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteSubrange )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cpFirst,
            /* [in] */ long cpLim);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Find )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange,
            /* [in] */ long Count,
            /* [in] */ long Flags,
            /* [out] */ __RPC__out long *pDelta);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetChar2 )( 
            __RPC__in ITextSelection2 * This,
            /* [out] */ __RPC__out long *pChar,
            /* [in] */ long Offset);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetDropCap )( 
            __RPC__in ITextSelection2 * This,
            /* [out] */ __RPC__out long *pcLine,
            /* [out] */ __RPC__out long *pPosition);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInlineObject )( 
            __RPC__in ITextSelection2 * This,
            /* [out] */ __RPC__out long *pType,
            /* [out] */ __RPC__out long *pAlign,
            /* [out] */ __RPC__out long *pChar,
            /* [out] */ __RPC__out long *pChar1,
            /* [out] */ __RPC__out long *pChar2,
            /* [out] */ __RPC__out long *pCount,
            /* [out] */ __RPC__out long *pTeXStyle,
            /* [out] */ __RPC__out long *pcCol,
            /* [out] */ __RPC__out long *pLevel);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetRect )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pRight,
            /* [out] */ __RPC__out long *pBottom,
            /* [out] */ __RPC__out long *pHit);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetSubrange )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long iSubrange,
            /* [out] */ __RPC__out long *pcpFirst,
            /* [out] */ __RPC__out long *pcpLim);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetText2 )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Flags,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *HexToUnicode )( 
            __RPC__in ITextSelection2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InsertTable )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cCol,
            /* [in] */ long cRow,
            /* [in] */ long AutoFit);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Linearize )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Flags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetActiveSubrange )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cpAnchor,
            /* [in] */ long cpActive);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetDropCap )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long cLine,
            /* [in] */ long Position);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Type,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetText2 )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Flags,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *UnicodeToHex )( 
            __RPC__in ITextSelection2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInlineObject )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long Type,
            /* [in] */ long Align,
            /* [in] */ long Char,
            /* [in] */ long Char1,
            /* [in] */ long Char2,
            /* [in] */ long Count,
            /* [in] */ long TeXStyle,
            /* [in] */ long cCol);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetMathFunctionType )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InsertImage )( 
            __RPC__in ITextSelection2 * This,
            /* [in] */ long width,
            /* [in] */ long height,
            /* [in] */ long ascent,
            /* [in] */ long Type,
            /* [in] */ __RPC__in BSTR bstrAltText,
            /* [in] */ __RPC__in_opt IStream *pStream);
        
        END_INTERFACE
    } ITextSelection2Vtbl;

    interface ITextSelection2
    {
        CONST_VTBL struct ITextSelection2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextSelection2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextSelection2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextSelection2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextSelection2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextSelection2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextSelection2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextSelection2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextSelection2_GetText(This,pbstr)	\
    ( (This)->lpVtbl -> GetText(This,pbstr) ) 

#define ITextSelection2_SetText(This,bstr)	\
    ( (This)->lpVtbl -> SetText(This,bstr) ) 

#define ITextSelection2_GetChar(This,pChar)	\
    ( (This)->lpVtbl -> GetChar(This,pChar) ) 

#define ITextSelection2_SetChar(This,Char)	\
    ( (This)->lpVtbl -> SetChar(This,Char) ) 

#define ITextSelection2_GetDuplicate(This,ppRange)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppRange) ) 

#define ITextSelection2_GetFormattedText(This,ppRange)	\
    ( (This)->lpVtbl -> GetFormattedText(This,ppRange) ) 

#define ITextSelection2_SetFormattedText(This,pRange)	\
    ( (This)->lpVtbl -> SetFormattedText(This,pRange) ) 

#define ITextSelection2_GetStart(This,pcpFirst)	\
    ( (This)->lpVtbl -> GetStart(This,pcpFirst) ) 

#define ITextSelection2_SetStart(This,cpFirst)	\
    ( (This)->lpVtbl -> SetStart(This,cpFirst) ) 

#define ITextSelection2_GetEnd(This,pcpLim)	\
    ( (This)->lpVtbl -> GetEnd(This,pcpLim) ) 

#define ITextSelection2_SetEnd(This,cpLim)	\
    ( (This)->lpVtbl -> SetEnd(This,cpLim) ) 

#define ITextSelection2_GetFont(This,ppFont)	\
    ( (This)->lpVtbl -> GetFont(This,ppFont) ) 

#define ITextSelection2_SetFont(This,pFont)	\
    ( (This)->lpVtbl -> SetFont(This,pFont) ) 

#define ITextSelection2_GetPara(This,ppPara)	\
    ( (This)->lpVtbl -> GetPara(This,ppPara) ) 

#define ITextSelection2_SetPara(This,pPara)	\
    ( (This)->lpVtbl -> SetPara(This,pPara) ) 

#define ITextSelection2_GetStoryLength(This,pCount)	\
    ( (This)->lpVtbl -> GetStoryLength(This,pCount) ) 

#define ITextSelection2_GetStoryType(This,pValue)	\
    ( (This)->lpVtbl -> GetStoryType(This,pValue) ) 

#define ITextSelection2_Collapse(This,bStart)	\
    ( (This)->lpVtbl -> Collapse(This,bStart) ) 

#define ITextSelection2_Expand(This,Unit,pDelta)	\
    ( (This)->lpVtbl -> Expand(This,Unit,pDelta) ) 

#define ITextSelection2_GetIndex(This,Unit,pIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,Unit,pIndex) ) 

#define ITextSelection2_SetIndex(This,Unit,Index,Extend)	\
    ( (This)->lpVtbl -> SetIndex(This,Unit,Index,Extend) ) 

#define ITextSelection2_SetRange(This,cpAnchor,cpActive)	\
    ( (This)->lpVtbl -> SetRange(This,cpAnchor,cpActive) ) 

#define ITextSelection2_InRange(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InRange(This,pRange,pValue) ) 

#define ITextSelection2_InStory(This,pRange,pValue)	\
    ( (This)->lpVtbl -> InStory(This,pRange,pValue) ) 

#define ITextSelection2_IsEqual(This,pRange,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pRange,pValue) ) 

#define ITextSelection2_Select(This)	\
    ( (This)->lpVtbl -> Select(This) ) 

#define ITextSelection2_StartOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> StartOf(This,Unit,Extend,pDelta) ) 

#define ITextSelection2_EndOf(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> EndOf(This,Unit,Extend,pDelta) ) 

#define ITextSelection2_Move(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Move(This,Unit,Count,pDelta) ) 

#define ITextSelection2_MoveStart(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStart(This,Unit,Count,pDelta) ) 

#define ITextSelection2_MoveEnd(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEnd(This,Unit,Count,pDelta) ) 

#define ITextSelection2_MoveWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveWhile(This,Cset,Count,pDelta) ) 

#define ITextSelection2_MoveStartWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartWhile(This,Cset,Count,pDelta) ) 

#define ITextSelection2_MoveEndWhile(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndWhile(This,Cset,Count,pDelta) ) 

#define ITextSelection2_MoveUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveUntil(This,Cset,Count,pDelta) ) 

#define ITextSelection2_MoveStartUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveStartUntil(This,Cset,Count,pDelta) ) 

#define ITextSelection2_MoveEndUntil(This,Cset,Count,pDelta)	\
    ( (This)->lpVtbl -> MoveEndUntil(This,Cset,Count,pDelta) ) 

#define ITextSelection2_FindText(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindText(This,bstr,Count,Flags,pLength) ) 

#define ITextSelection2_FindTextStart(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextStart(This,bstr,Count,Flags,pLength) ) 

#define ITextSelection2_FindTextEnd(This,bstr,Count,Flags,pLength)	\
    ( (This)->lpVtbl -> FindTextEnd(This,bstr,Count,Flags,pLength) ) 

#define ITextSelection2_Delete(This,Unit,Count,pDelta)	\
    ( (This)->lpVtbl -> Delete(This,Unit,Count,pDelta) ) 

#define ITextSelection2_Cut(This,pVar)	\
    ( (This)->lpVtbl -> Cut(This,pVar) ) 

#define ITextSelection2_Copy(This,pVar)	\
    ( (This)->lpVtbl -> Copy(This,pVar) ) 

#define ITextSelection2_Paste(This,pVar,Format)	\
    ( (This)->lpVtbl -> Paste(This,pVar,Format) ) 

#define ITextSelection2_CanPaste(This,pVar,Format,pValue)	\
    ( (This)->lpVtbl -> CanPaste(This,pVar,Format,pValue) ) 

#define ITextSelection2_CanEdit(This,pValue)	\
    ( (This)->lpVtbl -> CanEdit(This,pValue) ) 

#define ITextSelection2_ChangeCase(This,Type)	\
    ( (This)->lpVtbl -> ChangeCase(This,Type) ) 

#define ITextSelection2_GetPoint(This,Type,px,py)	\
    ( (This)->lpVtbl -> GetPoint(This,Type,px,py) ) 

#define ITextSelection2_SetPoint(This,x,y,Type,Extend)	\
    ( (This)->lpVtbl -> SetPoint(This,x,y,Type,Extend) ) 

#define ITextSelection2_ScrollIntoView(This,Value)	\
    ( (This)->lpVtbl -> ScrollIntoView(This,Value) ) 

#define ITextSelection2_GetEmbeddedObject(This,ppObject)	\
    ( (This)->lpVtbl -> GetEmbeddedObject(This,ppObject) ) 


#define ITextSelection2_GetFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pFlags) ) 

#define ITextSelection2_SetFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetFlags(This,Flags) ) 

#define ITextSelection2_GetType(This,pType)	\
    ( (This)->lpVtbl -> GetType(This,pType) ) 

#define ITextSelection2_MoveLeft(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveLeft(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection2_MoveRight(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveRight(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection2_MoveUp(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveUp(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection2_MoveDown(This,Unit,Count,Extend,pDelta)	\
    ( (This)->lpVtbl -> MoveDown(This,Unit,Count,Extend,pDelta) ) 

#define ITextSelection2_HomeKey(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> HomeKey(This,Unit,Extend,pDelta) ) 

#define ITextSelection2_EndKey(This,Unit,Extend,pDelta)	\
    ( (This)->lpVtbl -> EndKey(This,Unit,Extend,pDelta) ) 

#define ITextSelection2_TypeText(This,bstr)	\
    ( (This)->lpVtbl -> TypeText(This,bstr) ) 


#define ITextSelection2_GetCch(This,pcch)	\
    ( (This)->lpVtbl -> GetCch(This,pcch) ) 

#define ITextSelection2_GetCells(This,ppCells)	\
    ( (This)->lpVtbl -> GetCells(This,ppCells) ) 

#define ITextSelection2_GetColumn(This,ppColumn)	\
    ( (This)->lpVtbl -> GetColumn(This,ppColumn) ) 

#define ITextSelection2_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ITextSelection2_GetDuplicate2(This,ppRange)	\
    ( (This)->lpVtbl -> GetDuplicate2(This,ppRange) ) 

#define ITextSelection2_GetFont2(This,ppFont)	\
    ( (This)->lpVtbl -> GetFont2(This,ppFont) ) 

#define ITextSelection2_SetFont2(This,pFont)	\
    ( (This)->lpVtbl -> SetFont2(This,pFont) ) 

#define ITextSelection2_GetFormattedText2(This,ppRange)	\
    ( (This)->lpVtbl -> GetFormattedText2(This,ppRange) ) 

#define ITextSelection2_SetFormattedText2(This,pRange)	\
    ( (This)->lpVtbl -> SetFormattedText2(This,pRange) ) 

#define ITextSelection2_GetGravity(This,pValue)	\
    ( (This)->lpVtbl -> GetGravity(This,pValue) ) 

#define ITextSelection2_SetGravity(This,Value)	\
    ( (This)->lpVtbl -> SetGravity(This,Value) ) 

#define ITextSelection2_GetPara2(This,ppPara)	\
    ( (This)->lpVtbl -> GetPara2(This,ppPara) ) 

#define ITextSelection2_SetPara2(This,pPara)	\
    ( (This)->lpVtbl -> SetPara2(This,pPara) ) 

#define ITextSelection2_GetRow(This,ppRow)	\
    ( (This)->lpVtbl -> GetRow(This,ppRow) ) 

#define ITextSelection2_GetStartPara(This,pValue)	\
    ( (This)->lpVtbl -> GetStartPara(This,pValue) ) 

#define ITextSelection2_GetTable(This,ppTable)	\
    ( (This)->lpVtbl -> GetTable(This,ppTable) ) 

#define ITextSelection2_GetURL(This,pbstr)	\
    ( (This)->lpVtbl -> GetURL(This,pbstr) ) 

#define ITextSelection2_SetURL(This,bstr)	\
    ( (This)->lpVtbl -> SetURL(This,bstr) ) 

#define ITextSelection2_AddSubrange(This,cp1,cp2,Activate)	\
    ( (This)->lpVtbl -> AddSubrange(This,cp1,cp2,Activate) ) 

#define ITextSelection2_BuildUpMath(This,Flags)	\
    ( (This)->lpVtbl -> BuildUpMath(This,Flags) ) 

#define ITextSelection2_DeleteSubrange(This,cpFirst,cpLim)	\
    ( (This)->lpVtbl -> DeleteSubrange(This,cpFirst,cpLim) ) 

#define ITextSelection2_Find(This,pRange,Count,Flags,pDelta)	\
    ( (This)->lpVtbl -> Find(This,pRange,Count,Flags,pDelta) ) 

#define ITextSelection2_GetChar2(This,pChar,Offset)	\
    ( (This)->lpVtbl -> GetChar2(This,pChar,Offset) ) 

#define ITextSelection2_GetDropCap(This,pcLine,pPosition)	\
    ( (This)->lpVtbl -> GetDropCap(This,pcLine,pPosition) ) 

#define ITextSelection2_GetInlineObject(This,pType,pAlign,pChar,pChar1,pChar2,pCount,pTeXStyle,pcCol,pLevel)	\
    ( (This)->lpVtbl -> GetInlineObject(This,pType,pAlign,pChar,pChar1,pChar2,pCount,pTeXStyle,pcCol,pLevel) ) 

#define ITextSelection2_GetProperty(This,Type,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Type,pValue) ) 

#define ITextSelection2_GetRect(This,Type,pLeft,pTop,pRight,pBottom,pHit)	\
    ( (This)->lpVtbl -> GetRect(This,Type,pLeft,pTop,pRight,pBottom,pHit) ) 

#define ITextSelection2_GetSubrange(This,iSubrange,pcpFirst,pcpLim)	\
    ( (This)->lpVtbl -> GetSubrange(This,iSubrange,pcpFirst,pcpLim) ) 

#define ITextSelection2_GetText2(This,Flags,pbstr)	\
    ( (This)->lpVtbl -> GetText2(This,Flags,pbstr) ) 

#define ITextSelection2_HexToUnicode(This)	\
    ( (This)->lpVtbl -> HexToUnicode(This) ) 

#define ITextSelection2_InsertTable(This,cCol,cRow,AutoFit)	\
    ( (This)->lpVtbl -> InsertTable(This,cCol,cRow,AutoFit) ) 

#define ITextSelection2_Linearize(This,Flags)	\
    ( (This)->lpVtbl -> Linearize(This,Flags) ) 

#define ITextSelection2_SetActiveSubrange(This,cpAnchor,cpActive)	\
    ( (This)->lpVtbl -> SetActiveSubrange(This,cpAnchor,cpActive) ) 

#define ITextSelection2_SetDropCap(This,cLine,Position)	\
    ( (This)->lpVtbl -> SetDropCap(This,cLine,Position) ) 

#define ITextSelection2_SetProperty(This,Type,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Type,Value) ) 

#define ITextSelection2_SetText2(This,Flags,bstr)	\
    ( (This)->lpVtbl -> SetText2(This,Flags,bstr) ) 

#define ITextSelection2_UnicodeToHex(This)	\
    ( (This)->lpVtbl -> UnicodeToHex(This) ) 

#define ITextSelection2_SetInlineObject(This,Type,Align,Char,Char1,Char2,Count,TeXStyle,cCol)	\
    ( (This)->lpVtbl -> SetInlineObject(This,Type,Align,Char,Char1,Char2,Count,TeXStyle,cCol) ) 

#define ITextSelection2_GetMathFunctionType(This,bstr,pValue)	\
    ( (This)->lpVtbl -> GetMathFunctionType(This,bstr,pValue) ) 

#define ITextSelection2_InsertImage(This,width,height,ascent,Type,bstrAltText,pStream)	\
    ( (This)->lpVtbl -> InsertImage(This,width,height,ascent,Type,bstrAltText,pStream) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextSelection2_INTERFACE_DEFINED__ */


#ifndef __ITextFont2_INTERFACE_DEFINED__
#define __ITextFont2_INTERFACE_DEFINED__

/* interface ITextFont2 */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextFont2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5E3-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextFont2 : public ITextFont
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAutoLigatures( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAutoLigatures( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAutospaceAlpha( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAutospaceAlpha( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAutospaceNumeric( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAutospaceNumeric( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAutospaceParens( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAutospaceParens( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCharRep( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCharRep( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCompressionMode( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCompressionMode( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCookie( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCookie( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDoubleStrike( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetDoubleStrike( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDuplicate2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextFont2 **ppFont) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetDuplicate2( 
            /* [in] */ __RPC__in_opt ITextFont2 *pFont) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetLinkType( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetMathZone( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetMathZone( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetModWidthPairs( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetModWidthPairs( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetModWidthSpace( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetModWidthSpace( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetOldNumbers( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetOldNumbers( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetOverlapping( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetOverlapping( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetPositionSubSuper( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetPositionSubSuper( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetScaling( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetScaling( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSpaceExtension( 
            /* [retval][out] */ __RPC__out float *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSpaceExtension( 
            /* [in] */ float Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetUnderlinePositionMode( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetUnderlinePositionMode( 
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEffects( 
            /* [out] */ __RPC__out long *pValue,
            /* [out] */ __RPC__out long *pMask) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEffects2( 
            /* [out] */ __RPC__out long *pValue,
            /* [out] */ __RPC__out long *pMask) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ long Type,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetPropertyInfo( 
            /* [in] */ long Index,
            /* [out] */ __RPC__out long *pType,
            /* [out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsEqual2( 
            /* [in] */ __RPC__in_opt ITextFont2 *pFont,
            /* [retval][out] */ __RPC__out long *pB) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetEffects( 
            /* [in] */ long Value,
            /* [in] */ long Mask) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetEffects2( 
            /* [in] */ long Value,
            /* [in] */ long Mask) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ long Type,
            /* [in] */ long Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextFont2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextFont2 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextFont2 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextFont2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextFont2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDuplicate )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanChange )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ __RPC__in_opt ITextFont *pFont,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStyle )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStyle )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAllCaps )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAllCaps )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAnimation )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAnimation )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetBackColor )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetBackColor )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetBold )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetBold )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEmboss )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEmboss )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetForeColor )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetForeColor )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetHidden )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetHidden )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetEngrave )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetEngrave )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetItalic )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetItalic )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKerning )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKerning )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLanguageID )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetLanguageID )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetOutline )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetOutline )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPosition )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPosition )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetProtected )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetProtected )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetShadow )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetShadow )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSmallCaps )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSmallCaps )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSpacing )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSpacing )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStrikeThrough )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStrikeThrough )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSubscript )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSubscript )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSuperscript )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSuperscript )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetUnderline )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetUnderline )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetWeight )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetWeight )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAutoLigatures )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAutoLigatures )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAutospaceAlpha )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAutospaceAlpha )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAutospaceNumeric )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAutospaceNumeric )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAutospaceParens )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAutospaceParens )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCharRep )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCharRep )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCompressionMode )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCompressionMode )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCookie )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCookie )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDoubleStrike )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDoubleStrike )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate2 )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont2 **ppFont);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDuplicate2 )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ __RPC__in_opt ITextFont2 *pFont);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLinkType )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetMathZone )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetMathZone )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetModWidthPairs )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetModWidthPairs )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetModWidthSpace )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetModWidthSpace )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetOldNumbers )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetOldNumbers )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetOverlapping )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetOverlapping )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPositionSubSuper )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPositionSubSuper )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetScaling )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetScaling )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSpaceExtension )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSpaceExtension )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetUnderlinePositionMode )( 
            __RPC__in ITextFont2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetUnderlinePositionMode )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEffects )( 
            __RPC__in ITextFont2 * This,
            /* [out] */ __RPC__out long *pValue,
            /* [out] */ __RPC__out long *pMask);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEffects2 )( 
            __RPC__in ITextFont2 * This,
            /* [out] */ __RPC__out long *pValue,
            /* [out] */ __RPC__out long *pMask);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Type,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyInfo )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Index,
            /* [out] */ __RPC__out long *pType,
            /* [out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual2 )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ __RPC__in_opt ITextFont2 *pFont,
            /* [retval][out] */ __RPC__out long *pB);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetEffects )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value,
            /* [in] */ long Mask);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetEffects2 )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Value,
            /* [in] */ long Mask);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ITextFont2 * This,
            /* [in] */ long Type,
            /* [in] */ long Value);
        
        END_INTERFACE
    } ITextFont2Vtbl;

    interface ITextFont2
    {
        CONST_VTBL struct ITextFont2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextFont2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextFont2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextFont2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextFont2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextFont2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextFont2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextFont2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextFont2_GetDuplicate(This,ppFont)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppFont) ) 

#define ITextFont2_SetDuplicate(This,pFont)	\
    ( (This)->lpVtbl -> SetDuplicate(This,pFont) ) 

#define ITextFont2_CanChange(This,pValue)	\
    ( (This)->lpVtbl -> CanChange(This,pValue) ) 

#define ITextFont2_IsEqual(This,pFont,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pFont,pValue) ) 

#define ITextFont2_Reset(This,Value)	\
    ( (This)->lpVtbl -> Reset(This,Value) ) 

#define ITextFont2_GetStyle(This,pValue)	\
    ( (This)->lpVtbl -> GetStyle(This,pValue) ) 

#define ITextFont2_SetStyle(This,Value)	\
    ( (This)->lpVtbl -> SetStyle(This,Value) ) 

#define ITextFont2_GetAllCaps(This,pValue)	\
    ( (This)->lpVtbl -> GetAllCaps(This,pValue) ) 

#define ITextFont2_SetAllCaps(This,Value)	\
    ( (This)->lpVtbl -> SetAllCaps(This,Value) ) 

#define ITextFont2_GetAnimation(This,pValue)	\
    ( (This)->lpVtbl -> GetAnimation(This,pValue) ) 

#define ITextFont2_SetAnimation(This,Value)	\
    ( (This)->lpVtbl -> SetAnimation(This,Value) ) 

#define ITextFont2_GetBackColor(This,pValue)	\
    ( (This)->lpVtbl -> GetBackColor(This,pValue) ) 

#define ITextFont2_SetBackColor(This,Value)	\
    ( (This)->lpVtbl -> SetBackColor(This,Value) ) 

#define ITextFont2_GetBold(This,pValue)	\
    ( (This)->lpVtbl -> GetBold(This,pValue) ) 

#define ITextFont2_SetBold(This,Value)	\
    ( (This)->lpVtbl -> SetBold(This,Value) ) 

#define ITextFont2_GetEmboss(This,pValue)	\
    ( (This)->lpVtbl -> GetEmboss(This,pValue) ) 

#define ITextFont2_SetEmboss(This,Value)	\
    ( (This)->lpVtbl -> SetEmboss(This,Value) ) 

#define ITextFont2_GetForeColor(This,pValue)	\
    ( (This)->lpVtbl -> GetForeColor(This,pValue) ) 

#define ITextFont2_SetForeColor(This,Value)	\
    ( (This)->lpVtbl -> SetForeColor(This,Value) ) 

#define ITextFont2_GetHidden(This,pValue)	\
    ( (This)->lpVtbl -> GetHidden(This,pValue) ) 

#define ITextFont2_SetHidden(This,Value)	\
    ( (This)->lpVtbl -> SetHidden(This,Value) ) 

#define ITextFont2_GetEngrave(This,pValue)	\
    ( (This)->lpVtbl -> GetEngrave(This,pValue) ) 

#define ITextFont2_SetEngrave(This,Value)	\
    ( (This)->lpVtbl -> SetEngrave(This,Value) ) 

#define ITextFont2_GetItalic(This,pValue)	\
    ( (This)->lpVtbl -> GetItalic(This,pValue) ) 

#define ITextFont2_SetItalic(This,Value)	\
    ( (This)->lpVtbl -> SetItalic(This,Value) ) 

#define ITextFont2_GetKerning(This,pValue)	\
    ( (This)->lpVtbl -> GetKerning(This,pValue) ) 

#define ITextFont2_SetKerning(This,Value)	\
    ( (This)->lpVtbl -> SetKerning(This,Value) ) 

#define ITextFont2_GetLanguageID(This,pValue)	\
    ( (This)->lpVtbl -> GetLanguageID(This,pValue) ) 

#define ITextFont2_SetLanguageID(This,Value)	\
    ( (This)->lpVtbl -> SetLanguageID(This,Value) ) 

#define ITextFont2_GetName(This,pbstr)	\
    ( (This)->lpVtbl -> GetName(This,pbstr) ) 

#define ITextFont2_SetName(This,bstr)	\
    ( (This)->lpVtbl -> SetName(This,bstr) ) 

#define ITextFont2_GetOutline(This,pValue)	\
    ( (This)->lpVtbl -> GetOutline(This,pValue) ) 

#define ITextFont2_SetOutline(This,Value)	\
    ( (This)->lpVtbl -> SetOutline(This,Value) ) 

#define ITextFont2_GetPosition(This,pValue)	\
    ( (This)->lpVtbl -> GetPosition(This,pValue) ) 

#define ITextFont2_SetPosition(This,Value)	\
    ( (This)->lpVtbl -> SetPosition(This,Value) ) 

#define ITextFont2_GetProtected(This,pValue)	\
    ( (This)->lpVtbl -> GetProtected(This,pValue) ) 

#define ITextFont2_SetProtected(This,Value)	\
    ( (This)->lpVtbl -> SetProtected(This,Value) ) 

#define ITextFont2_GetShadow(This,pValue)	\
    ( (This)->lpVtbl -> GetShadow(This,pValue) ) 

#define ITextFont2_SetShadow(This,Value)	\
    ( (This)->lpVtbl -> SetShadow(This,Value) ) 

#define ITextFont2_GetSize(This,pValue)	\
    ( (This)->lpVtbl -> GetSize(This,pValue) ) 

#define ITextFont2_SetSize(This,Value)	\
    ( (This)->lpVtbl -> SetSize(This,Value) ) 

#define ITextFont2_GetSmallCaps(This,pValue)	\
    ( (This)->lpVtbl -> GetSmallCaps(This,pValue) ) 

#define ITextFont2_SetSmallCaps(This,Value)	\
    ( (This)->lpVtbl -> SetSmallCaps(This,Value) ) 

#define ITextFont2_GetSpacing(This,pValue)	\
    ( (This)->lpVtbl -> GetSpacing(This,pValue) ) 

#define ITextFont2_SetSpacing(This,Value)	\
    ( (This)->lpVtbl -> SetSpacing(This,Value) ) 

#define ITextFont2_GetStrikeThrough(This,pValue)	\
    ( (This)->lpVtbl -> GetStrikeThrough(This,pValue) ) 

#define ITextFont2_SetStrikeThrough(This,Value)	\
    ( (This)->lpVtbl -> SetStrikeThrough(This,Value) ) 

#define ITextFont2_GetSubscript(This,pValue)	\
    ( (This)->lpVtbl -> GetSubscript(This,pValue) ) 

#define ITextFont2_SetSubscript(This,Value)	\
    ( (This)->lpVtbl -> SetSubscript(This,Value) ) 

#define ITextFont2_GetSuperscript(This,pValue)	\
    ( (This)->lpVtbl -> GetSuperscript(This,pValue) ) 

#define ITextFont2_SetSuperscript(This,Value)	\
    ( (This)->lpVtbl -> SetSuperscript(This,Value) ) 

#define ITextFont2_GetUnderline(This,pValue)	\
    ( (This)->lpVtbl -> GetUnderline(This,pValue) ) 

#define ITextFont2_SetUnderline(This,Value)	\
    ( (This)->lpVtbl -> SetUnderline(This,Value) ) 

#define ITextFont2_GetWeight(This,pValue)	\
    ( (This)->lpVtbl -> GetWeight(This,pValue) ) 

#define ITextFont2_SetWeight(This,Value)	\
    ( (This)->lpVtbl -> SetWeight(This,Value) ) 


#define ITextFont2_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ITextFont2_GetAutoLigatures(This,pValue)	\
    ( (This)->lpVtbl -> GetAutoLigatures(This,pValue) ) 

#define ITextFont2_SetAutoLigatures(This,Value)	\
    ( (This)->lpVtbl -> SetAutoLigatures(This,Value) ) 

#define ITextFont2_GetAutospaceAlpha(This,pValue)	\
    ( (This)->lpVtbl -> GetAutospaceAlpha(This,pValue) ) 

#define ITextFont2_SetAutospaceAlpha(This,Value)	\
    ( (This)->lpVtbl -> SetAutospaceAlpha(This,Value) ) 

#define ITextFont2_GetAutospaceNumeric(This,pValue)	\
    ( (This)->lpVtbl -> GetAutospaceNumeric(This,pValue) ) 

#define ITextFont2_SetAutospaceNumeric(This,Value)	\
    ( (This)->lpVtbl -> SetAutospaceNumeric(This,Value) ) 

#define ITextFont2_GetAutospaceParens(This,pValue)	\
    ( (This)->lpVtbl -> GetAutospaceParens(This,pValue) ) 

#define ITextFont2_SetAutospaceParens(This,Value)	\
    ( (This)->lpVtbl -> SetAutospaceParens(This,Value) ) 

#define ITextFont2_GetCharRep(This,pValue)	\
    ( (This)->lpVtbl -> GetCharRep(This,pValue) ) 

#define ITextFont2_SetCharRep(This,Value)	\
    ( (This)->lpVtbl -> SetCharRep(This,Value) ) 

#define ITextFont2_GetCompressionMode(This,pValue)	\
    ( (This)->lpVtbl -> GetCompressionMode(This,pValue) ) 

#define ITextFont2_SetCompressionMode(This,Value)	\
    ( (This)->lpVtbl -> SetCompressionMode(This,Value) ) 

#define ITextFont2_GetCookie(This,pValue)	\
    ( (This)->lpVtbl -> GetCookie(This,pValue) ) 

#define ITextFont2_SetCookie(This,Value)	\
    ( (This)->lpVtbl -> SetCookie(This,Value) ) 

#define ITextFont2_GetDoubleStrike(This,pValue)	\
    ( (This)->lpVtbl -> GetDoubleStrike(This,pValue) ) 

#define ITextFont2_SetDoubleStrike(This,Value)	\
    ( (This)->lpVtbl -> SetDoubleStrike(This,Value) ) 

#define ITextFont2_GetDuplicate2(This,ppFont)	\
    ( (This)->lpVtbl -> GetDuplicate2(This,ppFont) ) 

#define ITextFont2_SetDuplicate2(This,pFont)	\
    ( (This)->lpVtbl -> SetDuplicate2(This,pFont) ) 

#define ITextFont2_GetLinkType(This,pValue)	\
    ( (This)->lpVtbl -> GetLinkType(This,pValue) ) 

#define ITextFont2_GetMathZone(This,pValue)	\
    ( (This)->lpVtbl -> GetMathZone(This,pValue) ) 

#define ITextFont2_SetMathZone(This,Value)	\
    ( (This)->lpVtbl -> SetMathZone(This,Value) ) 

#define ITextFont2_GetModWidthPairs(This,pValue)	\
    ( (This)->lpVtbl -> GetModWidthPairs(This,pValue) ) 

#define ITextFont2_SetModWidthPairs(This,Value)	\
    ( (This)->lpVtbl -> SetModWidthPairs(This,Value) ) 

#define ITextFont2_GetModWidthSpace(This,pValue)	\
    ( (This)->lpVtbl -> GetModWidthSpace(This,pValue) ) 

#define ITextFont2_SetModWidthSpace(This,Value)	\
    ( (This)->lpVtbl -> SetModWidthSpace(This,Value) ) 

#define ITextFont2_GetOldNumbers(This,pValue)	\
    ( (This)->lpVtbl -> GetOldNumbers(This,pValue) ) 

#define ITextFont2_SetOldNumbers(This,Value)	\
    ( (This)->lpVtbl -> SetOldNumbers(This,Value) ) 

#define ITextFont2_GetOverlapping(This,pValue)	\
    ( (This)->lpVtbl -> GetOverlapping(This,pValue) ) 

#define ITextFont2_SetOverlapping(This,Value)	\
    ( (This)->lpVtbl -> SetOverlapping(This,Value) ) 

#define ITextFont2_GetPositionSubSuper(This,pValue)	\
    ( (This)->lpVtbl -> GetPositionSubSuper(This,pValue) ) 

#define ITextFont2_SetPositionSubSuper(This,Value)	\
    ( (This)->lpVtbl -> SetPositionSubSuper(This,Value) ) 

#define ITextFont2_GetScaling(This,pValue)	\
    ( (This)->lpVtbl -> GetScaling(This,pValue) ) 

#define ITextFont2_SetScaling(This,Value)	\
    ( (This)->lpVtbl -> SetScaling(This,Value) ) 

#define ITextFont2_GetSpaceExtension(This,pValue)	\
    ( (This)->lpVtbl -> GetSpaceExtension(This,pValue) ) 

#define ITextFont2_SetSpaceExtension(This,Value)	\
    ( (This)->lpVtbl -> SetSpaceExtension(This,Value) ) 

#define ITextFont2_GetUnderlinePositionMode(This,pValue)	\
    ( (This)->lpVtbl -> GetUnderlinePositionMode(This,pValue) ) 

#define ITextFont2_SetUnderlinePositionMode(This,Value)	\
    ( (This)->lpVtbl -> SetUnderlinePositionMode(This,Value) ) 

#define ITextFont2_GetEffects(This,pValue,pMask)	\
    ( (This)->lpVtbl -> GetEffects(This,pValue,pMask) ) 

#define ITextFont2_GetEffects2(This,pValue,pMask)	\
    ( (This)->lpVtbl -> GetEffects2(This,pValue,pMask) ) 

#define ITextFont2_GetProperty(This,Type,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Type,pValue) ) 

#define ITextFont2_GetPropertyInfo(This,Index,pType,pValue)	\
    ( (This)->lpVtbl -> GetPropertyInfo(This,Index,pType,pValue) ) 

#define ITextFont2_IsEqual2(This,pFont,pB)	\
    ( (This)->lpVtbl -> IsEqual2(This,pFont,pB) ) 

#define ITextFont2_SetEffects(This,Value,Mask)	\
    ( (This)->lpVtbl -> SetEffects(This,Value,Mask) ) 

#define ITextFont2_SetEffects2(This,Value,Mask)	\
    ( (This)->lpVtbl -> SetEffects2(This,Value,Mask) ) 

#define ITextFont2_SetProperty(This,Type,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Type,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextFont2_INTERFACE_DEFINED__ */


#ifndef __ITextPara2_INTERFACE_DEFINED__
#define __ITextPara2_INTERFACE_DEFINED__

/* interface ITextPara2 */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextPara2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5E4-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextPara2 : public ITextPara
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetBorders( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppBorders) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDuplicate2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextPara2 **ppPara) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetDuplicate2( 
            /* [in] */ __RPC__in_opt ITextPara2 *pPara) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetFontAlignment( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetFontAlignment( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetHangingPunctuation( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetHangingPunctuation( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetSnapToGrid( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetSnapToGrid( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetTrimPunctuationAtStart( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetTrimPunctuationAtStart( 
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEffects( 
            /* [out] */ __RPC__out long *pValue,
            /* [out] */ __RPC__out long *pMask) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ long Type,
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsEqual2( 
            /* [in] */ __RPC__in_opt ITextPara2 *pPara,
            /* [retval][out] */ __RPC__out long *pB) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetEffects( 
            /* [in] */ long Value,
            /* [in] */ long Mask) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ long Type,
            /* [in] */ long Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextPara2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextPara2 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextPara2 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextPara2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextPara2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDuplicate )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanChange )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ __RPC__in_opt ITextPara *pPara,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStyle )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetStyle )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAlignment )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAlignment )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetHyphenation )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetHyphenation )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFirstLineIndent )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKeepTogether )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKeepTogether )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKeepWithNext )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKeepWithNext )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLeftIndent )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLineSpacing )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetLineSpacingRule )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListAlignment )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListAlignment )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListLevelIndex )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListLevelIndex )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListStart )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListStart )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListTab )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListTab )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetListType )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetListType )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetNoLineNumber )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetNoLineNumber )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetPageBreakBefore )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetPageBreakBefore )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetRightIndent )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetRightIndent )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ float Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetIndents )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ float First,
            /* [in] */ float Left,
            /* [in] */ float Right);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetLineSpacing )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Rule,
            /* [in] */ float Spacing);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSpaceAfter )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSpaceAfter )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSpaceBefore )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSpaceBefore )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ float Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetWidowControl )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetWidowControl )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetTabCount )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddTab )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ float tbPos,
            /* [in] */ long tbAlign,
            /* [in] */ long tbLeader);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ClearAllTabs )( 
            __RPC__in ITextPara2 * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteTab )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ float tbPos);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetTab )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long iTab,
            /* [out] */ __RPC__out float *ptbPos,
            /* [out] */ __RPC__out long *ptbAlign,
            /* [out] */ __RPC__out long *ptbLeader);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetBorders )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppBorders);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDuplicate2 )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara2 **ppPara);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDuplicate2 )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ __RPC__in_opt ITextPara2 *pPara);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetFontAlignment )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetFontAlignment )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetHangingPunctuation )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetHangingPunctuation )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSnapToGrid )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSnapToGrid )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetTrimPunctuationAtStart )( 
            __RPC__in ITextPara2 * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetTrimPunctuationAtStart )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEffects )( 
            __RPC__in ITextPara2 * This,
            /* [out] */ __RPC__out long *pValue,
            /* [out] */ __RPC__out long *pMask);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Type,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual2 )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ __RPC__in_opt ITextPara2 *pPara,
            /* [retval][out] */ __RPC__out long *pB);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetEffects )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Value,
            /* [in] */ long Mask);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ITextPara2 * This,
            /* [in] */ long Type,
            /* [in] */ long Value);
        
        END_INTERFACE
    } ITextPara2Vtbl;

    interface ITextPara2
    {
        CONST_VTBL struct ITextPara2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextPara2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextPara2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextPara2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextPara2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextPara2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextPara2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextPara2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextPara2_GetDuplicate(This,ppPara)	\
    ( (This)->lpVtbl -> GetDuplicate(This,ppPara) ) 

#define ITextPara2_SetDuplicate(This,pPara)	\
    ( (This)->lpVtbl -> SetDuplicate(This,pPara) ) 

#define ITextPara2_CanChange(This,pValue)	\
    ( (This)->lpVtbl -> CanChange(This,pValue) ) 

#define ITextPara2_IsEqual(This,pPara,pValue)	\
    ( (This)->lpVtbl -> IsEqual(This,pPara,pValue) ) 

#define ITextPara2_Reset(This,Value)	\
    ( (This)->lpVtbl -> Reset(This,Value) ) 

#define ITextPara2_GetStyle(This,pValue)	\
    ( (This)->lpVtbl -> GetStyle(This,pValue) ) 

#define ITextPara2_SetStyle(This,Value)	\
    ( (This)->lpVtbl -> SetStyle(This,Value) ) 

#define ITextPara2_GetAlignment(This,pValue)	\
    ( (This)->lpVtbl -> GetAlignment(This,pValue) ) 

#define ITextPara2_SetAlignment(This,Value)	\
    ( (This)->lpVtbl -> SetAlignment(This,Value) ) 

#define ITextPara2_GetHyphenation(This,pValue)	\
    ( (This)->lpVtbl -> GetHyphenation(This,pValue) ) 

#define ITextPara2_SetHyphenation(This,Value)	\
    ( (This)->lpVtbl -> SetHyphenation(This,Value) ) 

#define ITextPara2_GetFirstLineIndent(This,pValue)	\
    ( (This)->lpVtbl -> GetFirstLineIndent(This,pValue) ) 

#define ITextPara2_GetKeepTogether(This,pValue)	\
    ( (This)->lpVtbl -> GetKeepTogether(This,pValue) ) 

#define ITextPara2_SetKeepTogether(This,Value)	\
    ( (This)->lpVtbl -> SetKeepTogether(This,Value) ) 

#define ITextPara2_GetKeepWithNext(This,pValue)	\
    ( (This)->lpVtbl -> GetKeepWithNext(This,pValue) ) 

#define ITextPara2_SetKeepWithNext(This,Value)	\
    ( (This)->lpVtbl -> SetKeepWithNext(This,Value) ) 

#define ITextPara2_GetLeftIndent(This,pValue)	\
    ( (This)->lpVtbl -> GetLeftIndent(This,pValue) ) 

#define ITextPara2_GetLineSpacing(This,pValue)	\
    ( (This)->lpVtbl -> GetLineSpacing(This,pValue) ) 

#define ITextPara2_GetLineSpacingRule(This,pValue)	\
    ( (This)->lpVtbl -> GetLineSpacingRule(This,pValue) ) 

#define ITextPara2_GetListAlignment(This,pValue)	\
    ( (This)->lpVtbl -> GetListAlignment(This,pValue) ) 

#define ITextPara2_SetListAlignment(This,Value)	\
    ( (This)->lpVtbl -> SetListAlignment(This,Value) ) 

#define ITextPara2_GetListLevelIndex(This,pValue)	\
    ( (This)->lpVtbl -> GetListLevelIndex(This,pValue) ) 

#define ITextPara2_SetListLevelIndex(This,Value)	\
    ( (This)->lpVtbl -> SetListLevelIndex(This,Value) ) 

#define ITextPara2_GetListStart(This,pValue)	\
    ( (This)->lpVtbl -> GetListStart(This,pValue) ) 

#define ITextPara2_SetListStart(This,Value)	\
    ( (This)->lpVtbl -> SetListStart(This,Value) ) 

#define ITextPara2_GetListTab(This,pValue)	\
    ( (This)->lpVtbl -> GetListTab(This,pValue) ) 

#define ITextPara2_SetListTab(This,Value)	\
    ( (This)->lpVtbl -> SetListTab(This,Value) ) 

#define ITextPara2_GetListType(This,pValue)	\
    ( (This)->lpVtbl -> GetListType(This,pValue) ) 

#define ITextPara2_SetListType(This,Value)	\
    ( (This)->lpVtbl -> SetListType(This,Value) ) 

#define ITextPara2_GetNoLineNumber(This,pValue)	\
    ( (This)->lpVtbl -> GetNoLineNumber(This,pValue) ) 

#define ITextPara2_SetNoLineNumber(This,Value)	\
    ( (This)->lpVtbl -> SetNoLineNumber(This,Value) ) 

#define ITextPara2_GetPageBreakBefore(This,pValue)	\
    ( (This)->lpVtbl -> GetPageBreakBefore(This,pValue) ) 

#define ITextPara2_SetPageBreakBefore(This,Value)	\
    ( (This)->lpVtbl -> SetPageBreakBefore(This,Value) ) 

#define ITextPara2_GetRightIndent(This,pValue)	\
    ( (This)->lpVtbl -> GetRightIndent(This,pValue) ) 

#define ITextPara2_SetRightIndent(This,Value)	\
    ( (This)->lpVtbl -> SetRightIndent(This,Value) ) 

#define ITextPara2_SetIndents(This,First,Left,Right)	\
    ( (This)->lpVtbl -> SetIndents(This,First,Left,Right) ) 

#define ITextPara2_SetLineSpacing(This,Rule,Spacing)	\
    ( (This)->lpVtbl -> SetLineSpacing(This,Rule,Spacing) ) 

#define ITextPara2_GetSpaceAfter(This,pValue)	\
    ( (This)->lpVtbl -> GetSpaceAfter(This,pValue) ) 

#define ITextPara2_SetSpaceAfter(This,Value)	\
    ( (This)->lpVtbl -> SetSpaceAfter(This,Value) ) 

#define ITextPara2_GetSpaceBefore(This,pValue)	\
    ( (This)->lpVtbl -> GetSpaceBefore(This,pValue) ) 

#define ITextPara2_SetSpaceBefore(This,Value)	\
    ( (This)->lpVtbl -> SetSpaceBefore(This,Value) ) 

#define ITextPara2_GetWidowControl(This,pValue)	\
    ( (This)->lpVtbl -> GetWidowControl(This,pValue) ) 

#define ITextPara2_SetWidowControl(This,Value)	\
    ( (This)->lpVtbl -> SetWidowControl(This,Value) ) 

#define ITextPara2_GetTabCount(This,pCount)	\
    ( (This)->lpVtbl -> GetTabCount(This,pCount) ) 

#define ITextPara2_AddTab(This,tbPos,tbAlign,tbLeader)	\
    ( (This)->lpVtbl -> AddTab(This,tbPos,tbAlign,tbLeader) ) 

#define ITextPara2_ClearAllTabs(This)	\
    ( (This)->lpVtbl -> ClearAllTabs(This) ) 

#define ITextPara2_DeleteTab(This,tbPos)	\
    ( (This)->lpVtbl -> DeleteTab(This,tbPos) ) 

#define ITextPara2_GetTab(This,iTab,ptbPos,ptbAlign,ptbLeader)	\
    ( (This)->lpVtbl -> GetTab(This,iTab,ptbPos,ptbAlign,ptbLeader) ) 


#define ITextPara2_GetBorders(This,ppBorders)	\
    ( (This)->lpVtbl -> GetBorders(This,ppBorders) ) 

#define ITextPara2_GetDuplicate2(This,ppPara)	\
    ( (This)->lpVtbl -> GetDuplicate2(This,ppPara) ) 

#define ITextPara2_SetDuplicate2(This,pPara)	\
    ( (This)->lpVtbl -> SetDuplicate2(This,pPara) ) 

#define ITextPara2_GetFontAlignment(This,pValue)	\
    ( (This)->lpVtbl -> GetFontAlignment(This,pValue) ) 

#define ITextPara2_SetFontAlignment(This,Value)	\
    ( (This)->lpVtbl -> SetFontAlignment(This,Value) ) 

#define ITextPara2_GetHangingPunctuation(This,pValue)	\
    ( (This)->lpVtbl -> GetHangingPunctuation(This,pValue) ) 

#define ITextPara2_SetHangingPunctuation(This,Value)	\
    ( (This)->lpVtbl -> SetHangingPunctuation(This,Value) ) 

#define ITextPara2_GetSnapToGrid(This,pValue)	\
    ( (This)->lpVtbl -> GetSnapToGrid(This,pValue) ) 

#define ITextPara2_SetSnapToGrid(This,Value)	\
    ( (This)->lpVtbl -> SetSnapToGrid(This,Value) ) 

#define ITextPara2_GetTrimPunctuationAtStart(This,pValue)	\
    ( (This)->lpVtbl -> GetTrimPunctuationAtStart(This,pValue) ) 

#define ITextPara2_SetTrimPunctuationAtStart(This,Value)	\
    ( (This)->lpVtbl -> SetTrimPunctuationAtStart(This,Value) ) 

#define ITextPara2_GetEffects(This,pValue,pMask)	\
    ( (This)->lpVtbl -> GetEffects(This,pValue,pMask) ) 

#define ITextPara2_GetProperty(This,Type,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Type,pValue) ) 

#define ITextPara2_IsEqual2(This,pPara,pB)	\
    ( (This)->lpVtbl -> IsEqual2(This,pPara,pB) ) 

#define ITextPara2_SetEffects(This,Value,Mask)	\
    ( (This)->lpVtbl -> SetEffects(This,Value,Mask) ) 

#define ITextPara2_SetProperty(This,Type,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Type,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextPara2_INTERFACE_DEFINED__ */


#ifndef __ITextStoryRanges2_INTERFACE_DEFINED__
#define __ITextStoryRanges2_INTERFACE_DEFINED__

/* interface ITextStoryRanges2 */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextStoryRanges2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5E5-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextStoryRanges2 : public ITextStoryRanges
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item2( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoryRanges2Vtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoryRanges2 * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoryRanges2 * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextStoryRanges2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [restricted][id] */ HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppunkEnum);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item2 )( 
            __RPC__in ITextStoryRanges2 * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        END_INTERFACE
    } ITextStoryRanges2Vtbl;

    interface ITextStoryRanges2
    {
        CONST_VTBL struct ITextStoryRanges2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoryRanges2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoryRanges2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoryRanges2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoryRanges2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextStoryRanges2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextStoryRanges2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextStoryRanges2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextStoryRanges2__NewEnum(This,ppunkEnum)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppunkEnum) ) 

#define ITextStoryRanges2_Item(This,Index,ppRange)	\
    ( (This)->lpVtbl -> Item(This,Index,ppRange) ) 

#define ITextStoryRanges2_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 


#define ITextStoryRanges2_Item2(This,Index,ppRange)	\
    ( (This)->lpVtbl -> Item2(This,Index,ppRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoryRanges2_INTERFACE_DEFINED__ */


#ifndef __ITextStory_INTERFACE_DEFINED__
#define __ITextStory_INTERFACE_DEFINED__

/* interface ITextStory */
/* [object][nonextensible][version][uuid] */ 


EXTERN_C const IID IID_ITextStory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5F3-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextStory : public IUnknown
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetActive( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetActive( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetDisplay( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppDisplay) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetIndex( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetType( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetType( 
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetRange( 
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetFormattedText( 
            /* [in] */ __RPC__in_opt IUnknown *pUnk) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ long Type,
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ long Flags,
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoryVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStory * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStory * This);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetActive )( 
            __RPC__in ITextStory * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetActive )( 
            __RPC__in ITextStory * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDisplay )( 
            __RPC__in ITextStory * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppDisplay);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in ITextStory * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITextStory * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetType )( 
            __RPC__in ITextStory * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITextStory * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetRange )( 
            __RPC__in ITextStory * This,
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextStory * This,
            /* [in] */ long Flags,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText )( 
            __RPC__in ITextStory * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnk);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ITextStory * This,
            /* [in] */ long Type,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITextStory * This,
            /* [in] */ long Flags,
            /* [in] */ __RPC__in BSTR bstr);
        
        END_INTERFACE
    } ITextStoryVtbl;

    interface ITextStory
    {
        CONST_VTBL struct ITextStoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStory_GetActive(This,pValue)	\
    ( (This)->lpVtbl -> GetActive(This,pValue) ) 

#define ITextStory_SetActive(This,Value)	\
    ( (This)->lpVtbl -> SetActive(This,Value) ) 

#define ITextStory_GetDisplay(This,ppDisplay)	\
    ( (This)->lpVtbl -> GetDisplay(This,ppDisplay) ) 

#define ITextStory_GetIndex(This,pValue)	\
    ( (This)->lpVtbl -> GetIndex(This,pValue) ) 

#define ITextStory_GetType(This,pValue)	\
    ( (This)->lpVtbl -> GetType(This,pValue) ) 

#define ITextStory_SetType(This,Value)	\
    ( (This)->lpVtbl -> SetType(This,Value) ) 

#define ITextStory_GetProperty(This,Type,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Type,pValue) ) 

#define ITextStory_GetRange(This,cpActive,cpAnchor,ppRange)	\
    ( (This)->lpVtbl -> GetRange(This,cpActive,cpAnchor,ppRange) ) 

#define ITextStory_GetText(This,Flags,pbstr)	\
    ( (This)->lpVtbl -> GetText(This,Flags,pbstr) ) 

#define ITextStory_SetFormattedText(This,pUnk)	\
    ( (This)->lpVtbl -> SetFormattedText(This,pUnk) ) 

#define ITextStory_SetProperty(This,Type,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Type,Value) ) 

#define ITextStory_SetText(This,Flags,bstr)	\
    ( (This)->lpVtbl -> SetText(This,Flags,bstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStory_INTERFACE_DEFINED__ */


#ifndef __ITextStrings_INTERFACE_DEFINED__
#define __ITextStrings_INTERFACE_DEFINED__

/* interface ITextStrings */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextStrings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5E7-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextStrings : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt ITextRange2 *pRange,
            /* [in] */ long iString) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Cat2( 
            /* [in] */ long iString) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CatTop2( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteRange( 
            /* [in] */ __RPC__in_opt ITextRange2 *pRange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE EncodeFunction( 
            /* [in] */ long Type,
            /* [in] */ long Align,
            /* [in] */ long Char,
            /* [in] */ long Char1,
            /* [in] */ long Char2,
            /* [in] */ long Count,
            /* [in] */ long TeXStyle,
            /* [in] */ long cCol,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCch( 
            /* [in] */ long iString,
            /* [out] */ __RPC__out long *pcch) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE InsertNullStr( 
            /* [in] */ long iString) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveBoundary( 
            /* [in] */ long iString,
            /* [in] */ long cch) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PrefixTop( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long iString,
            /* [in] */ long cString) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetFormattedText( 
            /* [in] */ __RPC__in_opt ITextRange2 *pRangeD,
            /* [in] */ __RPC__in_opt ITextRange2 *pRangeS) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetOpCp( 
            /* [in] */ long iString,
            /* [in] */ long cp) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SuffixTop( 
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Swap( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStringsVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStrings * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStrings * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextStrings * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextStrings * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextStrings * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange2 **ppRange);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ITextStrings * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange,
            /* [in] */ long iString);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cat2 )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long iString);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CatTop2 )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteRange )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EncodeFunction )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long Type,
            /* [in] */ long Align,
            /* [in] */ long Char,
            /* [in] */ long Char1,
            /* [in] */ long Char2,
            /* [in] */ long Count,
            /* [in] */ long TeXStyle,
            /* [in] */ long cCol,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCch )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long iString,
            /* [out] */ __RPC__out long *pcch);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InsertNullStr )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long iString);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveBoundary )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long iString,
            /* [in] */ long cch);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PrefixTop )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long iString,
            /* [in] */ long cString);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetFormattedText )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in_opt ITextRange2 *pRangeD,
            /* [in] */ __RPC__in_opt ITextRange2 *pRangeS);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetOpCp )( 
            __RPC__in ITextStrings * This,
            /* [in] */ long iString,
            /* [in] */ long cp);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SuffixTop )( 
            __RPC__in ITextStrings * This,
            /* [in] */ __RPC__in BSTR bstr,
            /* [in] */ __RPC__in_opt ITextRange2 *pRange);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Swap )( 
            __RPC__in ITextStrings * This);
        
        END_INTERFACE
    } ITextStringsVtbl;

    interface ITextStrings
    {
        CONST_VTBL struct ITextStringsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStrings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStrings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStrings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStrings_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextStrings_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextStrings_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextStrings_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextStrings_Item(This,Index,ppRange)	\
    ( (This)->lpVtbl -> Item(This,Index,ppRange) ) 

#define ITextStrings_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ITextStrings_Add(This,bstr)	\
    ( (This)->lpVtbl -> Add(This,bstr) ) 

#define ITextStrings_Append(This,pRange,iString)	\
    ( (This)->lpVtbl -> Append(This,pRange,iString) ) 

#define ITextStrings_Cat2(This,iString)	\
    ( (This)->lpVtbl -> Cat2(This,iString) ) 

#define ITextStrings_CatTop2(This,bstr)	\
    ( (This)->lpVtbl -> CatTop2(This,bstr) ) 

#define ITextStrings_DeleteRange(This,pRange)	\
    ( (This)->lpVtbl -> DeleteRange(This,pRange) ) 

#define ITextStrings_EncodeFunction(This,Type,Align,Char,Char1,Char2,Count,TeXStyle,cCol,pRange)	\
    ( (This)->lpVtbl -> EncodeFunction(This,Type,Align,Char,Char1,Char2,Count,TeXStyle,cCol,pRange) ) 

#define ITextStrings_GetCch(This,iString,pcch)	\
    ( (This)->lpVtbl -> GetCch(This,iString,pcch) ) 

#define ITextStrings_InsertNullStr(This,iString)	\
    ( (This)->lpVtbl -> InsertNullStr(This,iString) ) 

#define ITextStrings_MoveBoundary(This,iString,cch)	\
    ( (This)->lpVtbl -> MoveBoundary(This,iString,cch) ) 

#define ITextStrings_PrefixTop(This,bstr)	\
    ( (This)->lpVtbl -> PrefixTop(This,bstr) ) 

#define ITextStrings_Remove(This,iString,cString)	\
    ( (This)->lpVtbl -> Remove(This,iString,cString) ) 

#define ITextStrings_SetFormattedText(This,pRangeD,pRangeS)	\
    ( (This)->lpVtbl -> SetFormattedText(This,pRangeD,pRangeS) ) 

#define ITextStrings_SetOpCp(This,iString,cp)	\
    ( (This)->lpVtbl -> SetOpCp(This,iString,cp) ) 

#define ITextStrings_SuffixTop(This,bstr,pRange)	\
    ( (This)->lpVtbl -> SuffixTop(This,bstr,pRange) ) 

#define ITextStrings_Swap(This)	\
    ( (This)->lpVtbl -> Swap(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStrings_INTERFACE_DEFINED__ */


#ifndef __ITextRow_INTERFACE_DEFINED__
#define __ITextRow_INTERFACE_DEFINED__

/* interface ITextRow */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextRow;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5EF-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextRow : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetAlignment( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetAlignment( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellCount( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellCount( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellCountCache( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellCountCache( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellIndex( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellIndex( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellMargin( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellMargin( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetHeight( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetHeight( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetIndent( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetIndent( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetKeepTogether( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetKeepTogether( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetKeepWithNext( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetKeepWithNext( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetNestLevel( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetRTL( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetRTL( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellAlignment( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellAlignment( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellColorBack( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellColorBack( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellColorFore( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellColorFore( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellMergeFlags( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellMergeFlags( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellShading( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellShading( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellVerticalText( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellVerticalText( 
            /* [in] */ long Value) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE GetCellWidth( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE SetCellWidth( 
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCellBorderColors( 
            /* [out] */ __RPC__out long *pcrLeft,
            /* [out] */ __RPC__out long *pcrTop,
            /* [out] */ __RPC__out long *pcrRight,
            /* [out] */ __RPC__out long *pcrBottom) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCellBorderWidths( 
            /* [out] */ __RPC__out long *pduLeft,
            /* [out] */ __RPC__out long *pduTop,
            /* [out] */ __RPC__out long *pduRight,
            /* [out] */ __RPC__out long *pduBottom) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetCellBorderColors( 
            /* [in] */ long crLeft,
            /* [in] */ long crTop,
            /* [in] */ long crRight,
            /* [in] */ long crBottom) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetCellBorderWidths( 
            /* [in] */ long duLeft,
            /* [in] */ long duTop,
            /* [in] */ long duRight,
            /* [in] */ long duBottom) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Apply( 
            /* [in] */ long cRow,
            /* [in] */ long Flags) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CanChange( 
            /* [retval][out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Insert( 
            /* [in] */ long cRow) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [in] */ __RPC__in_opt ITextRow *pRow,
            /* [retval][out] */ __RPC__out long *pB) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ long Value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ long Type,
            /* [in] */ long Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextRowVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextRow * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextRow * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextRow * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextRow * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextRow * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextRow * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextRow * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetAlignment )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetAlignment )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellCount )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellCount )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellCountCache )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellCountCache )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellIndex )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellIndex )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellMargin )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellMargin )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetHeight )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetHeight )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetIndent )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetIndent )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKeepTogether )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKeepTogether )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetKeepWithNext )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetKeepWithNext )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetNestLevel )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetRTL )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetRTL )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellAlignment )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellAlignment )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellColorBack )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellColorBack )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellColorFore )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellColorFore )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellMergeFlags )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellMergeFlags )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellShading )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellShading )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellVerticalText )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellVerticalText )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCellWidth )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCellWidth )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCellBorderColors )( 
            __RPC__in ITextRow * This,
            /* [out] */ __RPC__out long *pcrLeft,
            /* [out] */ __RPC__out long *pcrTop,
            /* [out] */ __RPC__out long *pcrRight,
            /* [out] */ __RPC__out long *pcrBottom);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCellBorderWidths )( 
            __RPC__in ITextRow * This,
            /* [out] */ __RPC__out long *pduLeft,
            /* [out] */ __RPC__out long *pduTop,
            /* [out] */ __RPC__out long *pduRight,
            /* [out] */ __RPC__out long *pduBottom);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetCellBorderColors )( 
            __RPC__in ITextRow * This,
            /* [in] */ long crLeft,
            /* [in] */ long crTop,
            /* [in] */ long crRight,
            /* [in] */ long crBottom);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetCellBorderWidths )( 
            __RPC__in ITextRow * This,
            /* [in] */ long duLeft,
            /* [in] */ long duTop,
            /* [in] */ long duRight,
            /* [in] */ long duBottom);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Apply )( 
            __RPC__in ITextRow * This,
            /* [in] */ long cRow,
            /* [in] */ long Flags);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CanChange )( 
            __RPC__in ITextRow * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pValue);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Insert )( 
            __RPC__in ITextRow * This,
            /* [in] */ long cRow);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in ITextRow * This,
            /* [in] */ __RPC__in_opt ITextRow *pRow,
            /* [retval][out] */ __RPC__out long *pB);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in ITextRow * This,
            /* [in] */ long Type,
            /* [in] */ long Value);
        
        END_INTERFACE
    } ITextRowVtbl;

    interface ITextRow
    {
        CONST_VTBL struct ITextRowVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextRow_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextRow_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextRow_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextRow_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextRow_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextRow_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextRow_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextRow_GetAlignment(This,pValue)	\
    ( (This)->lpVtbl -> GetAlignment(This,pValue) ) 

#define ITextRow_SetAlignment(This,Value)	\
    ( (This)->lpVtbl -> SetAlignment(This,Value) ) 

#define ITextRow_GetCellCount(This,pValue)	\
    ( (This)->lpVtbl -> GetCellCount(This,pValue) ) 

#define ITextRow_SetCellCount(This,Value)	\
    ( (This)->lpVtbl -> SetCellCount(This,Value) ) 

#define ITextRow_GetCellCountCache(This,pValue)	\
    ( (This)->lpVtbl -> GetCellCountCache(This,pValue) ) 

#define ITextRow_SetCellCountCache(This,Value)	\
    ( (This)->lpVtbl -> SetCellCountCache(This,Value) ) 

#define ITextRow_GetCellIndex(This,pValue)	\
    ( (This)->lpVtbl -> GetCellIndex(This,pValue) ) 

#define ITextRow_SetCellIndex(This,Value)	\
    ( (This)->lpVtbl -> SetCellIndex(This,Value) ) 

#define ITextRow_GetCellMargin(This,pValue)	\
    ( (This)->lpVtbl -> GetCellMargin(This,pValue) ) 

#define ITextRow_SetCellMargin(This,Value)	\
    ( (This)->lpVtbl -> SetCellMargin(This,Value) ) 

#define ITextRow_GetHeight(This,pValue)	\
    ( (This)->lpVtbl -> GetHeight(This,pValue) ) 

#define ITextRow_SetHeight(This,Value)	\
    ( (This)->lpVtbl -> SetHeight(This,Value) ) 

#define ITextRow_GetIndent(This,pValue)	\
    ( (This)->lpVtbl -> GetIndent(This,pValue) ) 

#define ITextRow_SetIndent(This,Value)	\
    ( (This)->lpVtbl -> SetIndent(This,Value) ) 

#define ITextRow_GetKeepTogether(This,pValue)	\
    ( (This)->lpVtbl -> GetKeepTogether(This,pValue) ) 

#define ITextRow_SetKeepTogether(This,Value)	\
    ( (This)->lpVtbl -> SetKeepTogether(This,Value) ) 

#define ITextRow_GetKeepWithNext(This,pValue)	\
    ( (This)->lpVtbl -> GetKeepWithNext(This,pValue) ) 

#define ITextRow_SetKeepWithNext(This,Value)	\
    ( (This)->lpVtbl -> SetKeepWithNext(This,Value) ) 

#define ITextRow_GetNestLevel(This,pValue)	\
    ( (This)->lpVtbl -> GetNestLevel(This,pValue) ) 

#define ITextRow_GetRTL(This,pValue)	\
    ( (This)->lpVtbl -> GetRTL(This,pValue) ) 

#define ITextRow_SetRTL(This,Value)	\
    ( (This)->lpVtbl -> SetRTL(This,Value) ) 

#define ITextRow_GetCellAlignment(This,pValue)	\
    ( (This)->lpVtbl -> GetCellAlignment(This,pValue) ) 

#define ITextRow_SetCellAlignment(This,Value)	\
    ( (This)->lpVtbl -> SetCellAlignment(This,Value) ) 

#define ITextRow_GetCellColorBack(This,pValue)	\
    ( (This)->lpVtbl -> GetCellColorBack(This,pValue) ) 

#define ITextRow_SetCellColorBack(This,Value)	\
    ( (This)->lpVtbl -> SetCellColorBack(This,Value) ) 

#define ITextRow_GetCellColorFore(This,pValue)	\
    ( (This)->lpVtbl -> GetCellColorFore(This,pValue) ) 

#define ITextRow_SetCellColorFore(This,Value)	\
    ( (This)->lpVtbl -> SetCellColorFore(This,Value) ) 

#define ITextRow_GetCellMergeFlags(This,pValue)	\
    ( (This)->lpVtbl -> GetCellMergeFlags(This,pValue) ) 

#define ITextRow_SetCellMergeFlags(This,Value)	\
    ( (This)->lpVtbl -> SetCellMergeFlags(This,Value) ) 

#define ITextRow_GetCellShading(This,pValue)	\
    ( (This)->lpVtbl -> GetCellShading(This,pValue) ) 

#define ITextRow_SetCellShading(This,Value)	\
    ( (This)->lpVtbl -> SetCellShading(This,Value) ) 

#define ITextRow_GetCellVerticalText(This,pValue)	\
    ( (This)->lpVtbl -> GetCellVerticalText(This,pValue) ) 

#define ITextRow_SetCellVerticalText(This,Value)	\
    ( (This)->lpVtbl -> SetCellVerticalText(This,Value) ) 

#define ITextRow_GetCellWidth(This,pValue)	\
    ( (This)->lpVtbl -> GetCellWidth(This,pValue) ) 

#define ITextRow_SetCellWidth(This,Value)	\
    ( (This)->lpVtbl -> SetCellWidth(This,Value) ) 

#define ITextRow_GetCellBorderColors(This,pcrLeft,pcrTop,pcrRight,pcrBottom)	\
    ( (This)->lpVtbl -> GetCellBorderColors(This,pcrLeft,pcrTop,pcrRight,pcrBottom) ) 

#define ITextRow_GetCellBorderWidths(This,pduLeft,pduTop,pduRight,pduBottom)	\
    ( (This)->lpVtbl -> GetCellBorderWidths(This,pduLeft,pduTop,pduRight,pduBottom) ) 

#define ITextRow_SetCellBorderColors(This,crLeft,crTop,crRight,crBottom)	\
    ( (This)->lpVtbl -> SetCellBorderColors(This,crLeft,crTop,crRight,crBottom) ) 

#define ITextRow_SetCellBorderWidths(This,duLeft,duTop,duRight,duBottom)	\
    ( (This)->lpVtbl -> SetCellBorderWidths(This,duLeft,duTop,duRight,duBottom) ) 

#define ITextRow_Apply(This,cRow,Flags)	\
    ( (This)->lpVtbl -> Apply(This,cRow,Flags) ) 

#define ITextRow_CanChange(This,pValue)	\
    ( (This)->lpVtbl -> CanChange(This,pValue) ) 

#define ITextRow_GetProperty(This,Type,pValue)	\
    ( (This)->lpVtbl -> GetProperty(This,Type,pValue) ) 

#define ITextRow_Insert(This,cRow)	\
    ( (This)->lpVtbl -> Insert(This,cRow) ) 

#define ITextRow_IsEqual(This,pRow,pB)	\
    ( (This)->lpVtbl -> IsEqual(This,pRow,pB) ) 

#define ITextRow_Reset(This,Value)	\
    ( (This)->lpVtbl -> Reset(This,Value) ) 

#define ITextRow_SetProperty(This,Type,Value)	\
    ( (This)->lpVtbl -> SetProperty(This,Type,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextRow_INTERFACE_DEFINED__ */


#ifndef __ITextDisplays_INTERFACE_DEFINED__
#define __ITextDisplays_INTERFACE_DEFINED__

/* interface ITextDisplays */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextDisplays;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C241F5F2-7206-11D8-A2C7-00A0D1D6C6B3")
    ITextDisplays : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ITextDisplaysVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextDisplays * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextDisplays * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextDisplays * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextDisplays * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextDisplays * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextDisplays * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextDisplays * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } ITextDisplaysVtbl;

    interface ITextDisplays
    {
        CONST_VTBL struct ITextDisplaysVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextDisplays_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextDisplays_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextDisplays_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextDisplays_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextDisplays_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextDisplays_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextDisplays_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextDisplays_INTERFACE_DEFINED__ */

#endif /* __tom_LIBRARY_DEFINED__ */

#ifndef __ITextDocument2Old_INTERFACE_DEFINED__
#define __ITextDocument2Old_INTERFACE_DEFINED__

/* interface ITextDocument2Old */
/* [object][nonextensible][dual][version][uuid] */ 


EXTERN_C const IID IID_ITextDocument2Old;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("01c25500-4268-11d1-883a-3c8b00c10000")
    ITextDocument2Old : public ITextDocument
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AttachMsgFilter( 
            /* [in] */ __RPC__in_opt IUnknown *pFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetEffectColor( 
            /* [in] */ long Index,
            /* [in] */ COLORREF cr) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEffectColor( 
            /* [in] */ long Index,
            /* [out] */ __RPC__out COLORREF *pcr) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetCaretType( 
            /* [retval][out] */ __RPC__out long *pCaretType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE SetCaretType( 
            /* [in] */ long CaretType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetImmContext( 
            /* [retval][out] */ __RPC__out __int64 *pContext) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseImmContext( 
            /* [in] */ __int64 Context) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPreferredFont( 
            /* [in] */ long cp,
            /* [in] */ long CharRep,
            /* [in] */ long Option,
            /* [in] */ long CharRepCur,
            /* [in] */ long curFontSize,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr,
            /* [out] */ __RPC__out long *pPitchAndFamily,
            /* [out] */ __RPC__out long *pNewFontSize) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetNotificationMode( 
            /* [retval][out] */ __RPC__out long *pMode) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE SetNotificationMode( 
            /* [in] */ long Mode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetClientRect( 
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pRight,
            /* [out] */ __RPC__out long *pBottom) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE GetSelection2( 
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection **ppSel) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWindow( 
            /* [out] */ __RPC__out long *phWnd) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFEFlags( 
            /* [out] */ __RPC__out long *pFlags) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateWindow( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CheckTextLimit( 
            long cch,
            __RPC__in long *pcch) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IMEInProgress( 
            long Value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SysBeep( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ long Mode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ long Notify) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDocumentFont( 
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppITextFont) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDocumentPara( 
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppITextPara) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCallManager( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVoid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseCallManager( 
            __RPC__in_opt IUnknown *pVoid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextDocument2OldVtbl
    {
        BEGIN_INTERFACE
        
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextDocument2Old * This);
        
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextDocument2Old * This);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITextDocument2Old * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITextDocument2Old * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection **ppSel);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryCount )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetStoryRanges )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextStoryRanges **ppStories);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSaved )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out long *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetSaved )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Value);
        
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultTabStop )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out float *pValue);
        
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultTabStop )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ float Value);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *New )( 
            __RPC__in ITextDocument2Old * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ __RPC__in VARIANT *pVar,
            /* [in] */ long Flags,
            /* [in] */ long CodePage);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Freeze )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Unfreeze )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *BeginEditCollection )( 
            __RPC__in ITextDocument2Old * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *EndEditCollection )( 
            __RPC__in ITextDocument2Old * This);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Undo )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Redo )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Count,
            /* [retval][out] */ __RPC__out long *pCount);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Range )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long cpActive,
            /* [in] */ long cpAnchor,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RangeFromPoint )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__deref_out_opt ITextRange **ppRange);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AttachMsgFilter )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ __RPC__in_opt IUnknown *pFilter);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetEffectColor )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Index,
            /* [in] */ COLORREF cr);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEffectColor )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Index,
            /* [out] */ __RPC__out COLORREF *pcr);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetCaretType )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out long *pCaretType);
        
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetCaretType )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long CaretType);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetImmContext )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out __int64 *pContext);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseImmContext )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ __int64 Context);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPreferredFont )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long cp,
            /* [in] */ long CharRep,
            /* [in] */ long Option,
            /* [in] */ long CharRepCur,
            /* [in] */ long curFontSize,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr,
            /* [out] */ __RPC__out long *pPitchAndFamily,
            /* [out] */ __RPC__out long *pNewFontSize);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetNotificationMode )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__out long *pMode);
        
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *SetNotificationMode )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Mode);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetClientRect )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Type,
            /* [out] */ __RPC__out long *pLeft,
            /* [out] */ __RPC__out long *pTop,
            /* [out] */ __RPC__out long *pRight,
            /* [out] */ __RPC__out long *pBottom);
        
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *GetSelection2 )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextSelection **ppSel);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            __RPC__in ITextDocument2Old * This,
            /* [out] */ __RPC__out long *phWnd);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFEFlags )( 
            __RPC__in ITextDocument2Old * This,
            /* [out] */ __RPC__out long *pFlags);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateWindow )( 
            __RPC__in ITextDocument2Old * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CheckTextLimit )( 
            __RPC__in ITextDocument2Old * This,
            long cch,
            __RPC__in long *pcch);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IMEInProgress )( 
            __RPC__in ITextDocument2Old * This,
            long Value);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SysBeep )( 
            __RPC__in ITextDocument2Old * This);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Mode);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Notify )( 
            __RPC__in ITextDocument2Old * This,
            /* [in] */ long Notify);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentFont )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextFont **ppITextFont);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentPara )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextPara **ppITextPara);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCallManager )( 
            __RPC__in ITextDocument2Old * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppVoid);
        
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseCallManager )( 
            __RPC__in ITextDocument2Old * This,
            __RPC__in_opt IUnknown *pVoid);
        
        END_INTERFACE
    } ITextDocument2OldVtbl;

    interface ITextDocument2Old
    {
        CONST_VTBL struct ITextDocument2OldVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextDocument2Old_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextDocument2Old_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextDocument2Old_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextDocument2Old_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITextDocument2Old_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITextDocument2Old_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITextDocument2Old_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITextDocument2Old_GetName(This,pName)	\
    ( (This)->lpVtbl -> GetName(This,pName) ) 

#define ITextDocument2Old_GetSelection(This,ppSel)	\
    ( (This)->lpVtbl -> GetSelection(This,ppSel) ) 

#define ITextDocument2Old_GetStoryCount(This,pCount)	\
    ( (This)->lpVtbl -> GetStoryCount(This,pCount) ) 

#define ITextDocument2Old_GetStoryRanges(This,ppStories)	\
    ( (This)->lpVtbl -> GetStoryRanges(This,ppStories) ) 

#define ITextDocument2Old_GetSaved(This,pValue)	\
    ( (This)->lpVtbl -> GetSaved(This,pValue) ) 

#define ITextDocument2Old_SetSaved(This,Value)	\
    ( (This)->lpVtbl -> SetSaved(This,Value) ) 

#define ITextDocument2Old_GetDefaultTabStop(This,pValue)	\
    ( (This)->lpVtbl -> GetDefaultTabStop(This,pValue) ) 

#define ITextDocument2Old_SetDefaultTabStop(This,Value)	\
    ( (This)->lpVtbl -> SetDefaultTabStop(This,Value) ) 

#define ITextDocument2Old_New(This)	\
    ( (This)->lpVtbl -> New(This) ) 

#define ITextDocument2Old_Open(This,pVar,Flags,CodePage)	\
    ( (This)->lpVtbl -> Open(This,pVar,Flags,CodePage) ) 

#define ITextDocument2Old_Save(This,pVar,Flags,CodePage)	\
    ( (This)->lpVtbl -> Save(This,pVar,Flags,CodePage) ) 

#define ITextDocument2Old_Freeze(This,pCount)	\
    ( (This)->lpVtbl -> Freeze(This,pCount) ) 

#define ITextDocument2Old_Unfreeze(This,pCount)	\
    ( (This)->lpVtbl -> Unfreeze(This,pCount) ) 

#define ITextDocument2Old_BeginEditCollection(This)	\
    ( (This)->lpVtbl -> BeginEditCollection(This) ) 

#define ITextDocument2Old_EndEditCollection(This)	\
    ( (This)->lpVtbl -> EndEditCollection(This) ) 

#define ITextDocument2Old_Undo(This,Count,pCount)	\
    ( (This)->lpVtbl -> Undo(This,Count,pCount) ) 

#define ITextDocument2Old_Redo(This,Count,pCount)	\
    ( (This)->lpVtbl -> Redo(This,Count,pCount) ) 

#define ITextDocument2Old_Range(This,cpActive,cpAnchor,ppRange)	\
    ( (This)->lpVtbl -> Range(This,cpActive,cpAnchor,ppRange) ) 

#define ITextDocument2Old_RangeFromPoint(This,x,y,ppRange)	\
    ( (This)->lpVtbl -> RangeFromPoint(This,x,y,ppRange) ) 


#define ITextDocument2Old_AttachMsgFilter(This,pFilter)	\
    ( (This)->lpVtbl -> AttachMsgFilter(This,pFilter) ) 

#define ITextDocument2Old_SetEffectColor(This,Index,cr)	\
    ( (This)->lpVtbl -> SetEffectColor(This,Index,cr) ) 

#define ITextDocument2Old_GetEffectColor(This,Index,pcr)	\
    ( (This)->lpVtbl -> GetEffectColor(This,Index,pcr) ) 

#define ITextDocument2Old_GetCaretType(This,pCaretType)	\
    ( (This)->lpVtbl -> GetCaretType(This,pCaretType) ) 

#define ITextDocument2Old_SetCaretType(This,CaretType)	\
    ( (This)->lpVtbl -> SetCaretType(This,CaretType) ) 

#define ITextDocument2Old_GetImmContext(This,pContext)	\
    ( (This)->lpVtbl -> GetImmContext(This,pContext) ) 

#define ITextDocument2Old_ReleaseImmContext(This,Context)	\
    ( (This)->lpVtbl -> ReleaseImmContext(This,Context) ) 

#define ITextDocument2Old_GetPreferredFont(This,cp,CharRep,Option,CharRepCur,curFontSize,pbstr,pPitchAndFamily,pNewFontSize)	\
    ( (This)->lpVtbl -> GetPreferredFont(This,cp,CharRep,Option,CharRepCur,curFontSize,pbstr,pPitchAndFamily,pNewFontSize) ) 

#define ITextDocument2Old_GetNotificationMode(This,pMode)	\
    ( (This)->lpVtbl -> GetNotificationMode(This,pMode) ) 

#define ITextDocument2Old_SetNotificationMode(This,Mode)	\
    ( (This)->lpVtbl -> SetNotificationMode(This,Mode) ) 

#define ITextDocument2Old_GetClientRect(This,Type,pLeft,pTop,pRight,pBottom)	\
    ( (This)->lpVtbl -> GetClientRect(This,Type,pLeft,pTop,pRight,pBottom) ) 

#define ITextDocument2Old_GetSelection2(This,ppSel)	\
    ( (This)->lpVtbl -> GetSelection2(This,ppSel) ) 

#define ITextDocument2Old_GetWindow(This,phWnd)	\
    ( (This)->lpVtbl -> GetWindow(This,phWnd) ) 

#define ITextDocument2Old_GetFEFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFEFlags(This,pFlags) ) 

#define ITextDocument2Old_UpdateWindow(This)	\
    ( (This)->lpVtbl -> UpdateWindow(This) ) 

#define ITextDocument2Old_CheckTextLimit(This,cch,pcch)	\
    ( (This)->lpVtbl -> CheckTextLimit(This,cch,pcch) ) 

#define ITextDocument2Old_IMEInProgress(This,Value)	\
    ( (This)->lpVtbl -> IMEInProgress(This,Value) ) 

#define ITextDocument2Old_SysBeep(This)	\
    ( (This)->lpVtbl -> SysBeep(This) ) 

#define ITextDocument2Old_Update(This,Mode)	\
    ( (This)->lpVtbl -> Update(This,Mode) ) 

#define ITextDocument2Old_Notify(This,Notify)	\
    ( (This)->lpVtbl -> Notify(This,Notify) ) 

#define ITextDocument2Old_GetDocumentFont(This,ppITextFont)	\
    ( (This)->lpVtbl -> GetDocumentFont(This,ppITextFont) ) 

#define ITextDocument2Old_GetDocumentPara(This,ppITextPara)	\
    ( (This)->lpVtbl -> GetDocumentPara(This,ppITextPara) ) 

#define ITextDocument2Old_GetCallManager(This,ppVoid)	\
    ( (This)->lpVtbl -> GetCallManager(This,ppVoid) ) 

#define ITextDocument2Old_ReleaseCallManager(This,pVoid)	\
    ( (This)->lpVtbl -> ReleaseCallManager(This,pVoid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextDocument2Old_INTERFACE_DEFINED__ */


/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


