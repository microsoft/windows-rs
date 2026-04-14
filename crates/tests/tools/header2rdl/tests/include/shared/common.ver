
/*---------------------------------------------------------------*/
/*                                                               */
/* The following section actually creates the version structure. */
/* They are ignored if we are not being invoked by RC.           */
/*                                                               */
/* ntverp.H must be included before including this file          */
/*                                                               */
/* If VER_LEGALCOPYRIGHT_STR is not defined, it will be          */
/* constructed using VER_LEGALCOPYRIGHT_YEARS, so at least one   */
/* these macros must be defined before including this file.      */
/*                                                               */
/* VER_FILETYPE, VER_FILESUBTYPE, VER_FILEDESCRIPTION_STR, and   */
/* VER_INTERNALNAME_STR must be defined before including this    */
/* file.                                                         */
/*                                                               */
/* If VER_FILEVERSION is not defined, VER_PRODUCTVERSION will be */
/* used instead.  If VER_FILEVERSION_STR is not defined,         */
/* VER_PRODUCTVERSION_STR will be used instead.                  */
/*                                                               */
/* If VER_ORIGINALFILENAME_STR is not defined, it is set to      */
/* the value in VER_INTERNALNAME_STR.                            */
/*                                                               */
/* If INTL is defined, then this is assumed to be an             */
/* an international build; two string blocks will be created,    */
/* (since all version resources must have English), and the      */
/* second one can be localized                                   */
/*                                                               */
/*---------------------------------------------------------------*/


/****************************************************************
    When updating the copy right year Please Make sure you also update the
    constant VER_LEGALCOPYRIGHT_STR_WITH_YEARS below
*****************************************************************/
#ifndef VER_LEGALCOPYRIGHT_YEARS
#define VER_LEGALCOPYRIGHT_YEARS    "2021"
#endif

#ifndef VER_LEGALCOPYRIGHT_STR_WITH_YEARS
#define VER_LEGALCOPYRIGHT_STR_WITH_YEARS L"\251 2021 Microsoft Corporation. All rights reserved."
#endif

#ifndef VER_LEGALCOPYRIGHT_STR
#if defined(RC_INVOKED) && !defined(WIN16)
#define VER_LEGALCOPYRIGHT_STR L"\251 Microsoft Corporation. All rights reserved."
#else
#define VER_LEGALCOPYRIGHT_STR "Copyright (c) Microsoft Corporation. All rights reserved."
#endif
#endif


#ifndef VER_PRODUCTNAME_STR
#ifdef RC_INVOKED
#define VER_PRODUCTNAME_STR L"Microsoft\256 Windows\256 Operating System"
#else
#define VER_PRODUCTNAME_STR "Microsoft (R) Windows (R) Operating System"
#endif
#endif

#ifndef VER_PRODUCTVERSION
#define VER_PRODUCTVERSION 5,00,01,001
#endif

#ifndef VER_FILEVERSION
#define VER_FILEVERSION VER_PRODUCTVERSION
#endif

#ifndef VER_PRODUCTVERSION_STR
#define VER_PRODUCTVERSION_STR "5.00"
#endif

#ifndef VER_FILEVERSION_STR
#define VER_FILEVERSION_STR VER_PRODUCTVERSION_STR
#endif

#ifndef VER_ORIGINALFILENAME_STR
#define VER_ORIGINALFILENAME_STR VER_INTERNALNAME_STR
#endif

#ifdef EXPORT_CONTROLLED

#ifdef EXPORT
#define EXPORT_TAG  " (Export Version)"
#else
#define EXPORT_TAG  " (US/Canada Only, Not for Export)"
#endif

#else           /* Not Export Controlled */

#define EXPORT_TAG

#endif

#if defined(__BUILDMACHINE__)

#if (defined(_NEED_FULLVERSION_INFO) || !defined(OFFICIAL_BUILD))
  #if defined(__BUILDDATE__)
    #define B2(x,y) " (" #x "." #y ")"
    #define B1(x,y) B2(x, y)
    #if (defined(_NEED_FULLVERSION_INFO) && defined(OFFICIAL_BUILD) && defined(_BUILDBRANCH))
      #define BUILD_MACHINE_TAG B1(_BUILDBRANCH, __BUILDDATE__)
    #else
      #define BUILD_MACHINE_TAG B1(__BUILDMACHINE__, __BUILDDATE__)
    #endif
  #else
    #define B2(x) " (" #x ".160101.0800)"
    #define B1(x) B2(x)
    #define BUILD_MACHINE_TAG B1(__BUILDMACHINE__)
  #endif
#else
  #define BUILD_MACHINE_TAG " (WinBuild.160101.0800)"
#endif

#if defined(__BUILDMACHINE_LEN__)
#if __BUILDMACHINE_LEN__ >= 25
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG
#elif __BUILDMACHINE_LEN__ == 24
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG " "
#elif __BUILDMACHINE_LEN__ == 23
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "  "
#elif __BUILDMACHINE_LEN__ == 22
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "   "
#elif __BUILDMACHINE_LEN__ == 21
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "    "
#elif __BUILDMACHINE_LEN__ == 20
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "     "
#elif __BUILDMACHINE_LEN__ == 19
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "      "
#elif __BUILDMACHINE_LEN__ == 18
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "       "
#elif __BUILDMACHINE_LEN__ == 17
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "        "
#elif __BUILDMACHINE_LEN__ == 16
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "         "
#elif __BUILDMACHINE_LEN__ == 15                       
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "          "
#elif __BUILDMACHINE_LEN__ == 14                               
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "           "
#elif __BUILDMACHINE_LEN__ == 13                                 
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "            "
#elif __BUILDMACHINE_LEN__ == 12                               
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "             "
#elif __BUILDMACHINE_LEN__ == 11                               
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "              "
#elif __BUILDMACHINE_LEN__ == 10                               
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "               "
#elif __BUILDMACHINE_LEN__ == 9                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                "
#elif __BUILDMACHINE_LEN__ == 8                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                 "
#elif __BUILDMACHINE_LEN__ == 7                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                  "
#elif __BUILDMACHINE_LEN__ == 6                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                   "
#elif __BUILDMACHINE_LEN__ == 5                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                    "
#elif __BUILDMACHINE_LEN__ == 4                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                     "
#elif __BUILDMACHINE_LEN__ == 3                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                      "
#elif __BUILDMACHINE_LEN__ == 2                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                       "
#elif __BUILDMACHINE_LEN__ == 1                                
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG "                        "
#else
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG
#endif
#else
#define BUILD_MACHINE_TAG_PADDED BUILD_MACHINE_TAG
#endif
#else
#define BUILD_MACHINE_TAG
#define BUILD_MACHINE_TAG_PADDED
#endif

#ifdef VER_LANGNEUTRAL
 #ifndef VER_VERSION_UNICODE_LANG
  #define VER_VERSION_UNICODE_LANG  "000004B0" /* LANG_NEUTRAL/SUBLANG_NEUTRAL, Unicode CP */
 #endif
 #ifndef VER_VERSION_ANSI_LANG
  #define VER_VERSION_ANSI_LANG     "000004E4" /* LANG_NEUTRAL/SUBLANG_NEUTRAL, Ansi CP */
 #endif
 #ifndef VER_VERSION_TRANSLATION
  #define VER_VERSION_TRANSLATION   0x0000, 0x04B0
 #endif
#else
 #ifndef VER_VERSION_UNICODE_LANG
  #define VER_VERSION_UNICODE_LANG  "040904B0" /* LANG_ENGLISH/SUBLANG_ENGLISH_US, Unicode CP */
 #endif
 #ifndef VER_VERSION_ANSI_LANG
  #define VER_VERSION_ANSI_LANG     "0c0904E4" /* LANG_ENGLISH/SUBLANG_ENGLISH_US, Ansi CP */
 #endif
 #ifndef VER_VERSION_TRANSLATION
  #define VER_VERSION_TRANSLATION   0x0409, 0x04B0
 #endif
#endif


#ifdef SMP_ID
#define VER_SAMPLE_IDENTIFIER_STR SMP_ID
#endif

#ifndef VER_SAMPLE_IDENTIFIER
#define VER_SAMPLE_IDENTIFIER    "SAMPLE_IDENTIFIER"
#endif

#ifdef VER_SAMPLE_IDENTIFIER_STR
resname RCDATA
{
  VER_SAMPLE_IDENTIFIER,
  VER_SAMPLE_IDENTIFIER_STR
}
#endif


#ifdef RC_INVOKED

VS_VERSION_INFO VERSIONINFO
FILEVERSION    VER_FILEVERSION
PRODUCTVERSION VER_PRODUCTVERSION
FILEFLAGSMASK  VER_FILEFLAGSMASK
FILEFLAGS      VER_FILEFLAGS
FILEOS         VER_FILEOS
FILETYPE       VER_FILETYPE
FILESUBTYPE    VER_FILESUBTYPE
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK VER_VERSION_UNICODE_LANG
        BEGIN
            VALUE "CompanyName",     VER_COMPANYNAME_STR
            VALUE "FileDescription", VER_FILEDESCRIPTION_STR EXPORT_TAG
            VALUE "FileVersion",     VER_FILEVERSION_STR BUILD_MACHINE_TAG_PADDED
            VALUE "InternalName",    VER_INTERNALNAME_STR
            VALUE "LegalCopyright",  VER_LEGALCOPYRIGHT_STR
            VALUE "OriginalFilename",VER_ORIGINALFILENAME_STR
            VALUE "ProductName",     VER_PRODUCTNAME_STR
            VALUE "ProductVersion",  VER_PRODUCTVERSION_STR
#ifdef VER_OLESELFREGISTER
            VALUE "OleSelfRegister", "\0"
#endif
        END

#ifdef VER_ANSICP	/* Some apps are hard coded to look for ANSI CP. */
	BLOCK VER_VERSION_ANSI_LANG
        BEGIN
            VALUE "CompanyName",     VER_COMPANYNAME_STR
            VALUE "FileDescription", VER_FILEDESCRIPTION_STR EXPORT_TAG
            VALUE "FileVersion",     VER_FILEVERSION_STR
            VALUE "InternalName",    VER_INTERNALNAME_STR
            VALUE "LegalCopyright",  VER_LEGALCOPYRIGHT_STR
            VALUE "OriginalFilename",VER_ORIGINALFILENAME_STR
            VALUE "ProductName",     VER_PRODUCTNAME_STR
            VALUE "ProductVersion",  VER_PRODUCTVERSION_STR
#ifdef VER_OLESELFREGISTER
            VALUE "OleSelfRegister", "\0"
#endif
        END
#endif
    END

    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", VER_VERSION_TRANSLATION
    END
END

#endif
