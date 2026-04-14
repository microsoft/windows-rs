/*******************************************************************************
// Copyright Microsoft Corporation. All Rights Reserved. 
* SPError.h *
*-----------*
*   Description:
*       This header file contains the custom error codes specific to SAPI5
*-------------------------------------------------------------------------------
*******************************************************************************/
#ifndef SPError_h
#define SPError_h

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _WINERROR_
#include <winerror.h>
#endif

#define FACILITY_SAPI      FACILITY_ITF
#define SAPI_ERROR_BASE    0x5000

#define MAKE_SAPI_HRESULT(sev, err)    MAKE_HRESULT(sev, FACILITY_SAPI, err)
#define MAKE_SAPI_ERROR(err)           MAKE_SAPI_HRESULT(SEVERITY_ERROR, err + SAPI_ERROR_BASE)
#define MAKE_SAPI_SCODE(scode)         MAKE_SAPI_HRESULT(SEVERITY_SUCCESS, scode + SAPI_ERROR_BASE)

/*** SPERR_UNINITIALIZED                                   0x80045001    -2147201023
*   The object has not been properly initialized.
*/
#define SPERR_UNINITIALIZED                                MAKE_SAPI_ERROR(0x001)

/*** SPERR_ALREADY_INITIALIZED                             0x80045002    -2147201022
*   The object has already been initialized.
*/
#define SPERR_ALREADY_INITIALIZED                          MAKE_SAPI_ERROR(0x002)

/*** SPERR_UNSUPPORTED_FORMAT                              0x80045003    -2147201021
*   The caller has specified an unsupported format.
*/
#define SPERR_UNSUPPORTED_FORMAT                           MAKE_SAPI_ERROR(0x003)

/*** SPERR_INVALID_FLAGS                                   0x80045004    -2147201020
*   The caller has specified invalid flags for this operation.
*/
#define SPERR_INVALID_FLAGS                                MAKE_SAPI_ERROR(0x004)

/*** SP_END_OF_STREAM                                      0x00045005    282629
*   The operation has reached the end of stream.
*/
#define SP_END_OF_STREAM                                   MAKE_SAPI_SCODE(0x005)

/*** SPERR_DEVICE_BUSY                                     0x80045006    -2147201018
*   The wave device is busy.
*/
#define SPERR_DEVICE_BUSY                                  MAKE_SAPI_ERROR(0x006)

/*** SPERR_DEVICE_NOT_SUPPORTED                            0x80045007    -2147201017
*   The wave device is not supported.
*/
#define SPERR_DEVICE_NOT_SUPPORTED                         MAKE_SAPI_ERROR(0x007)

/*** SPERR_DEVICE_NOT_ENABLED                              0x80045008    -2147201016
*   The wave device is not enabled.
*/
#define SPERR_DEVICE_NOT_ENABLED                           MAKE_SAPI_ERROR(0x008)

/*** SPERR_NO_DRIVER                                       0x80045009    -2147201015
*   There is no wave driver installed.
*/
#define SPERR_NO_DRIVER                                    MAKE_SAPI_ERROR(0x009)

/*** SPERR_FILEMUSTBEUNICODE                               0x8004500a    -2147201014
*   The file must be Unicode.
*/
#define SPERR_FILE_MUST_BE_UNICODE                         MAKE_SAPI_ERROR(0x00a)

/*** SP_INSUFFICIENTDATA                                   0x0004500b    282635
*
*/
#define SP_INSUFFICIENT_DATA                               MAKE_SAPI_SCODE(0x00b)

/*** SPERR_INVALID_PHRASE_ID                               0x8004500c    -2147201012
*   The phrase ID specified does not exist or is out of range.
*/
#define SPERR_INVALID_PHRASE_ID                            MAKE_SAPI_ERROR(0x00c)

/*** SPERR_BUFFER_TOO_SMALL                                0x8004500d    -2147201011
*   The caller provided a buffer too small to return a result.
*/
#define SPERR_BUFFER_TOO_SMALL                             MAKE_SAPI_ERROR(0x00d)

/*** SPERR_FORMAT_NOT_SPECIFIED                            0x8004500e    -2147201010
*   Caller did not specify a format prior to opening a stream.
*/
#define SPERR_FORMAT_NOT_SPECIFIED                         MAKE_SAPI_ERROR(0x00e)

/*** SPERR_AUDIO_STOPPED                                   0x8004500f    -2147201009
*   This method is deprecated. Use SP_AUDIO_STOPPED instead.
*/
#define SPERR_AUDIO_STOPPED                                MAKE_SAPI_ERROR(0x00f)

/*** SP_AUDIO_PAUSED                                       0x00045010    282640
*   This will be returned only on input (read) streams when the stream is paused.  Reads on
*   paused streams will not block, and this return code indicates that all of the data has been
*   removed from the stream.
*/
#define SP_AUDIO_PAUSED                                    MAKE_SAPI_SCODE(0x010)

/*** SPERR_RULE_NOT_FOUND                                  0x80045011    -2147201007
*   Invalid rule name passed to ActivateGrammar.
*/
#define SPERR_RULE_NOT_FOUND                               MAKE_SAPI_ERROR(0x011)

/*** SPERR_TTS_ENGINE_EXCEPTION                            0x80045012    -2147201006
*   An exception was raised during a call to the current TTS driver.
*/
#define SPERR_TTS_ENGINE_EXCEPTION                         MAKE_SAPI_ERROR(0x012)

/*** SPERR_TTS_NLP_EXCEPTION                               0x80045013    -2147201005
*   An exception was raised during a call to an application sentence filter.
*/
#define SPERR_TTS_NLP_EXCEPTION                            MAKE_SAPI_ERROR(0x013)

/*** SPERR_ENGINE_BUSY                                     0x80045014    -2147201004
*   In speech recognition, the current method can not be performed while
*   a grammar rule is active.
*/
#define SPERR_ENGINE_BUSY                                  MAKE_SAPI_ERROR(0x014)

/*** SP_AUDIO_CONVERSION_ENABLED                           0x00045015    282645
*   The operation was successful, but only with automatic stream format conversion.
*/
#define SP_AUDIO_CONVERSION_ENABLED                        MAKE_SAPI_SCODE(0x015)

/*** SP_NO_HYPOTHESIS_AVAILABLE                            0x00045016    282646
*   There is currently no hypothesis recognition available.
*/
#define SP_NO_HYPOTHESIS_AVAILABLE                         MAKE_SAPI_SCODE(0x016)

/*** SPERR_CANT_CREATE                                     0x80045017    -2147201001
*   Can not create a new object instance for the specified object category.
*/
#define SPERR_CANT_CREATE                                  MAKE_SAPI_ERROR(0x017)

/*** SP_ALREADY_IN_LEX                                     0x00045018    282648
*   The word, pronunciation, or POS pair being added is already in lexicon.
*/
#define SP_ALREADY_IN_LEX                                  MAKE_SAPI_SCODE(0x018)

/*** SPERR_NOT_IN_LEX                                      0x80045019    -2147200999
*   The word does not exist in the lexicon.
*/
#define SPERR_NOT_IN_LEX                                   MAKE_SAPI_ERROR(0x019)

/*** SP_LEX_NOTHING_TO_SYNC                                0x0004501a    282650
*   The client is currently synced with the lexicon.
*/
#define SP_LEX_NOTHING_TO_SYNC                             MAKE_SAPI_SCODE(0x01a)

/*** SPERR_LEX_VERY_OUT_OF_SYNC                            0x8004501b    -2147200997
*   The client is excessively out of sync with the lexicon. Mismatches may not be incrementally sync'd.
*/
#define SPERR_LEX_VERY_OUT_OF_SYNC                         MAKE_SAPI_ERROR(0x01b)

/*** SPERR_UNDEFINED_FORWARD_RULE_REF                      0x8004501c    -2147200996
*   A rule reference in a grammar was made to a named rule that was never defined.
*/
#define SPERR_UNDEFINED_FORWARD_RULE_REF                   MAKE_SAPI_ERROR(0x01c)

/*** SPERR_EMPTY_RULE                                      0x8004501d    -2147200995
*   A non-dynamic grammar rule that has no body.
*/
#define SPERR_EMPTY_RULE                                   MAKE_SAPI_ERROR(0x01d)

/*** SPERR_GRAMMAR_COMPILER_INTERNAL_ERROR                 0x8004501e    -2147200994
*   The grammar compiler failed due to an internal state error.
*/
#define SPERR_GRAMMAR_COMPILER_INTERNAL_ERROR              MAKE_SAPI_ERROR(0x01e)

/*** SPERR_RULE_NOT_DYNAMIC                                0x8004501f    -2147200993
*   An attempt was made to modify a non-dynamic rule.
*/
#define SPERR_RULE_NOT_DYNAMIC                             MAKE_SAPI_ERROR(0x01f)

/*** SPERR_DUPLICATE_RULE_NAME                             0x80045020    -2147200992
*   A rule name was duplicated.
*/
#define SPERR_DUPLICATE_RULE_NAME                          MAKE_SAPI_ERROR(0x020)

/*** SPERR_DUPLICATE_RESOURCE_NAME                         0x80045021    -2147200991
*   A resource name was duplicated for a given rule.
*/
#define SPERR_DUPLICATE_RESOURCE_NAME                      MAKE_SAPI_ERROR(0x021)

/*** SPERR_TOO_MANY_GRAMMARS                               0x80045022    -2147200990
*   Too many grammars have been loaded.
*/
#define SPERR_TOO_MANY_GRAMMARS                            MAKE_SAPI_ERROR(0x022)

/*** SPERR_CIRCULAR_REFERENCE                              0x80045023    -2147200989
*   Circular reference in import rules of grammars.
*/
#define SPERR_CIRCULAR_REFERENCE                           MAKE_SAPI_ERROR(0x023)

/*** SPERR_INVALID_IMPORT                                  0x80045024    -2147200988
*   A rule reference to an imported grammar that could not be resolved.
*/
#define SPERR_INVALID_IMPORT                               MAKE_SAPI_ERROR(0x024)

/*** SPERR_INVALID_WAV_FILE                                0x80045025    -2147200987
*   The format of the WAV file is not supported.
*/
#define SPERR_INVALID_WAV_FILE                             MAKE_SAPI_ERROR(0x025)

/*** SP_REQUEST_PENDING                                    0x00045026    282662
*   This success code indicates that an SR method called with the SPRIF_ASYNC flag is
*   being processed.  When it has finished processing, an SPFEI_ASYNC_COMPLETED event will be generated.
*/
#define SP_REQUEST_PENDING                                 MAKE_SAPI_SCODE(0x026)

/*** SPERR_ALL_WORDS_OPTIONAL                              0x80045027    -2147200985
*   A grammar rule was defined with a null path through the rule.  That is, it is possible
*   to satisfy the rule conditions with no words.
*/
#define SPERR_ALL_WORDS_OPTIONAL                           MAKE_SAPI_ERROR(0x027)

/*** SPERR_INSTANCE_CHANGE_INVALID                         0x80045028    -2147200984
*   It is not possible to change the current engine or input.  This occurs in the
*   following cases:
*
*       1) SelectEngine called while a recognition context exists, or
*       2) SetInput called in the shared instance case.
*/
#define SPERR_INSTANCE_CHANGE_INVALID                      MAKE_SAPI_ERROR(0x028)

/*** SPERR_RULE_NAME_ID_CONFLICT                          0x80045029    -2147200983
*   A rule exists with matching IDs (names) but different names (IDs).  
*/
#define SPERR_RULE_NAME_ID_CONFLICT                        MAKE_SAPI_ERROR(0x029)

/*** SPERR_NO_RULES                                       0x8004502a    -2147200982
*   A grammar contains no top-level, dynamic, or exported rules.  There is no possible
*   way to activate or otherwise use any rule in this grammar.
*/
#define SPERR_NO_RULES                                     MAKE_SAPI_ERROR(0x02a)

/*** SPERR_CIRCULAR_RULE_REF                              0x8004502b    -2147200981
*   Rule 'A' refers to a second rule 'B' which, in turn, refers to rule 'A'. 
*/
#define SPERR_CIRCULAR_RULE_REF                            MAKE_SAPI_ERROR(0x02b)

/*** SP_NO_PARSE_FOUND                                    0x0004502c    282668
*   Parse path cannot be parsed given the currently active rules.
*/
#define SP_NO_PARSE_FOUND                                  MAKE_SAPI_SCODE(0x02c)

/*** SPERR_NO_PARSE_FOUND                                 0x8004502d    -2147200979
*   Parse path cannot be parsed given the currently active rules.
*/
#define SPERR_INVALID_HANDLE                               MAKE_SAPI_ERROR(0x02d)

/*** SPERR_REMOTE_CALL_TIMED_OUT                          0x8004502e    -2147200978
*   A marshaled remote call failed to respond.
*/
#define SPERR_REMOTE_CALL_TIMED_OUT                        MAKE_SAPI_ERROR(0x02e)

/*** SPERR_AUDIO_BUFFER_OVERFLOW                           0x8004502f    -2147200977
*   This will only be returned on input (read) streams when the stream is paused because
*   the SR driver has not retrieved data recently.
*/
#define SPERR_AUDIO_BUFFER_OVERFLOW                        MAKE_SAPI_ERROR(0x02f)

/*** SPERR_NO_AUDIO_DATA                                   0x80045030    -2147200976
*   The result does not contain any audio, nor does the portion of the element chain of the result
*   contain any audio.
*/
#define SPERR_NO_AUDIO_DATA                                MAKE_SAPI_ERROR(0x030)

/*** SPERR_DEAD_ALTERNATE                                  0x80045031    -2147200975
*   This alternate is no longer a valid alternate to the result it was obtained from.
*   Returned from ISpPhraseAlt methods.
*/
#define SPERR_DEAD_ALTERNATE                               MAKE_SAPI_ERROR(0x031)

/*** SPERR_HIGH_LOW_CONFIDENCE                             0x80045032    -2147200974
*   The result does not contain any audio, nor does the portion of the element chain of the result
*   contain any audio.  Returned from ISpResult::GetAudio and ISpResult::SpeakAudio.
*/
#define SPERR_HIGH_LOW_CONFIDENCE                          MAKE_SAPI_ERROR(0x032)

/*** SPERR_INVALID_FORMAT_STRING                           0x80045033    -2147200973
*   The XML format string for this RULEREF is invalid, e.g. not a GUID or REFCLSID.
*/
#define SPERR_INVALID_FORMAT_STRING                        MAKE_SAPI_ERROR(0x033)

/*** SP_UNSUPPORTED_ON_STREAM_INPUT                        0x00045034    282676
*   The operation is not supported for stream input.
*/
#define SP_UNSUPPORTED_ON_STREAM_INPUT                     MAKE_SAPI_SCODE(0x034)

/*** SPERR_APPLEX_READ_ONLY                                0x80045035    -2147200971
*   The operation is invalid for all but newly created application lexicons.
*/
#define SPERR_APPLEX_READ_ONLY                             MAKE_SAPI_ERROR(0x035)

/*** SPERR_NO_TERMINATING_RULE_PATH                        0x80045036    -2147200970
*
*/

#define SPERR_NO_TERMINATING_RULE_PATH                     MAKE_SAPI_ERROR(0x036)

/*** SP_WORD_EXISTS_WITHOUT_PRONUNCIATION                  0x00045037    282679
*   The word exists but without pronunciation.
*/
#define SP_WORD_EXISTS_WITHOUT_PRONUNCIATION               MAKE_SAPI_SCODE(0x037)

/*** SPERR_STREAM_CLOSED                                   0x80045038    -2147200968
*   An operation was attempted on a stream object that has been closed.
*/
#define SPERR_STREAM_CLOSED                                MAKE_SAPI_ERROR(0x038)

// --- The following error codes are taken directly from WIN32  ---

/*** SPERR_NO_MORE_ITEMS                                   0x80045039    -2147200967
*   When enumerating items, the requested index is greater than the count of items.
*/
#define SPERR_NO_MORE_ITEMS                                MAKE_SAPI_ERROR(0x039)

/*** SPERR_NOT_FOUND                                       0x8004503a    -2147200966
*   The requested data item (data key, value, etc.) was not found.
*/
#define SPERR_NOT_FOUND                                    MAKE_SAPI_ERROR(0x03a)

/*** SPERR_INVALID_AUDIO_STATE                             0x8004503b    -2147200965
*   Audio state passed to SetState() is invalid.
*/
#define SPERR_INVALID_AUDIO_STATE                          MAKE_SAPI_ERROR(0x03b)

/*** SPERR_GENERIC_MMSYS_ERROR                             0x8004503c    -2147200964
*   A generic MMSYS error not caught by _MMRESULT_TO_HRESULT.
*/
#define SPERR_GENERIC_MMSYS_ERROR                          MAKE_SAPI_ERROR(0x03c)

/*** SPERR_MARSHALER_EXCEPTION                             0x8004503d    -2147200963
*   An exception was raised during a call to the marshaling code.
*/
#define SPERR_MARSHALER_EXCEPTION                          MAKE_SAPI_ERROR(0x03d)

/*** SPERR_NOT_DYNAMIC_GRAMMAR                             0x8004503e    -2147200962
*   Attempt was made to manipulate a non-dynamic grammar.
*/
#define SPERR_NOT_DYNAMIC_GRAMMAR                          MAKE_SAPI_ERROR(0x03e)

/*** SPERR_AMBIGUOUS_PROPERTY                              0x8004503f    -2147200961
*   Cannot add ambiguous property.
*/
#define SPERR_AMBIGUOUS_PROPERTY                           MAKE_SAPI_ERROR(0x03f)

/*** SPERR_INVALID_REGISTRY_KEY                            0x80045040    -2147200960
*   The key specified is invalid.
*/
#define SPERR_INVALID_REGISTRY_KEY                         MAKE_SAPI_ERROR(0x040)

/*** SPERR_INVALID_TOKEN_ID                                0x80045041    -2147200959
*   The token specified is invalid.
*/
#define SPERR_INVALID_TOKEN_ID                             MAKE_SAPI_ERROR(0x041)

/*** SPERR_XML_BAD_SYNTAX                                  0x80045042    -2147200958
*   The xml parser failed due to bad syntax.
*/
#define SPERR_XML_BAD_SYNTAX                               MAKE_SAPI_ERROR(0x042)

/*** SPERR_XML_RESOURCE_NOT_FOUND                          0x80045043    -2147200957
*   The xml parser failed to load a required resource (e.g., voice, phoneconverter, etc.).
*/
#define SPERR_XML_RESOURCE_NOT_FOUND                       MAKE_SAPI_ERROR(0x043)

/*** SPERR_TOKEN_IN_USE                                    0x80045044    -2147200956
*   Attempted to remove registry data from a token that is already in use elsewhere.
*/
#define SPERR_TOKEN_IN_USE                                 MAKE_SAPI_ERROR(0x044)

/*** SPERR_TOKEN_DELETED                                   0x80045045    -2147200955
*   Attempted to perform an action on an object token that has had associated registry key deleted.
*/
#define SPERR_TOKEN_DELETED                                MAKE_SAPI_ERROR(0x045)

/*** SPERR_MULTI_LINGUAL_NOT_SUPPORTED                     0x80045046    -2147200954
*   The selected voice was registered as multi-lingual. SAPI does not support multi-lingual registration. 
*/
#define SPERR_MULTI_LINGUAL_NOT_SUPPORTED                  MAKE_SAPI_ERROR(0x046)

/*** SPERR_EXPORT_DYNAMIC_RULE                             0x80045047    -2147200953
*   Exported rules cannot refer directly or indirectly to a dynamic rule.
*/
#define SPERR_EXPORT_DYNAMIC_RULE                          MAKE_SAPI_ERROR(0x047)

/*** SPERR_STGF_ERROR                                      0x80045048    -2147200952
*   Error parsing the SAPI Text Grammar Format (XML grammar).
*/
#define SPERR_STGF_ERROR                                   MAKE_SAPI_ERROR(0x048)

/*** SPERR_WORDFORMAT_ERROR                                0x80045049    -2147200951
*   Incorrect word format, probably due to incorrect pronunciation string.
*/
#define SPERR_WORDFORMAT_ERROR                             MAKE_SAPI_ERROR(0x049)

/*** SPERR_STREAM_NOT_ACTIVE                               0x8004504a    -2147200950
*   Methods associated with active audio stream cannot be called unless stream is active.
*/
#define SPERR_STREAM_NOT_ACTIVE                            MAKE_SAPI_ERROR(0x04a)

/*** SPERR_ENGINE_RESPONSE_INVALID                         0x8004504b    -2147200949
*   Arguments or data supplied by the engine are in an invalid format or are inconsistent.
*/
#define SPERR_ENGINE_RESPONSE_INVALID                      MAKE_SAPI_ERROR(0x04b)

/*** SPERR_SR_ENGINE_EXCEPTION                             0x8004504c    -2147200948
*   An exception was raised during a call to the current SR engine.
*/
#define SPERR_SR_ENGINE_EXCEPTION                          MAKE_SAPI_ERROR(0x04c)

/*** SPERR_STREAM_POS_INVALID                              0x8004504d    -2147200947
*   Stream position information supplied from engine is inconsistent.
*/
#define SPERR_STREAM_POS_INVALID                           MAKE_SAPI_ERROR(0x04d)

/*** SP_RECOGNIZER_INACTIVE                                0x0004504e    282702
*   Operation could not be completed because the recognizer is inactive. It is inactive either
*   because the recognition state is currently inactive or because no rules are active .
*/
#define SP_RECOGNIZER_INACTIVE                             MAKE_SAPI_SCODE(0x04e)

/*** SPERR_REMOTE_CALL_ON_WRONG_THREAD                     0x8004504f    -2147200945
*   When making a remote call to the server, the call was made on the wrong thread.
*/
#define SPERR_REMOTE_CALL_ON_WRONG_THREAD                  MAKE_SAPI_ERROR(0x04f)

/*** SPERR_REMOTE_PROCESS_TERMINATED                       0x80045050    -2147200944
*   The remote process terminated unexpectedly.
*/
#define SPERR_REMOTE_PROCESS_TERMINATED                    MAKE_SAPI_ERROR(0x050)

/*** SPERR_REMOTE_PROCESS_ALREADY_RUNNING                  0x80045051    -2147200943
*   The remote process is already running; it cannot be started a second time.
*/
#define SPERR_REMOTE_PROCESS_ALREADY_RUNNING               MAKE_SAPI_ERROR(0x051)

/*** SPERR_LANGID_MISMATCH                                 0x80045052    -2147200942
*   An attempt to load a CFG grammar with a LANGID different than other loaded grammars.
*/
#define SPERR_LANGID_MISMATCH                              MAKE_SAPI_ERROR(0x052)

/*** SP_PARTIAL_PARSE_FOUND                               0x00045053    282707
*   A grammar-ending parse has been found that does not use all available words.
*/
#define SP_PARTIAL_PARSE_FOUND                             MAKE_SAPI_SCODE(0x053)

/*** SPERR_NOT_TOPLEVEL_RULE                              0x80045054    -2147200940
*   An attempt to deactivate or activate a non-toplevel rule.
*/
#define SPERR_NOT_TOPLEVEL_RULE                            MAKE_SAPI_ERROR(0x054)

/*** SP_NO_RULE_ACTIVE                                    0x00045055    282709
*   An attempt to parse when no rule was active.
*/
#define SP_NO_RULE_ACTIVE                                  MAKE_SAPI_SCODE(0x055)

/*** SPERR_LEX_REQUIRES_COOKIE                            0x80045056    -2147200938
*   An attempt to ask a container lexicon for all words at once.
*/
#define SPERR_LEX_REQUIRES_COOKIE                          MAKE_SAPI_ERROR(0x056)

/*** SP_STREAM_UNINITIALIZED                              0x00045057    282711
*   An attempt to activate a rule/dictation/etc without calling SetInput 
*   first in the inproc case.
*/
#define SP_STREAM_UNINITIALIZED                            MAKE_SAPI_SCODE(0x057)


// Error x058 is not used in SAPI 5.0


/*** SPERR_UNSUPPORTED_LANG                               0x80045059    -2147200935
*   The requested language is not supported.
*/
#define SPERR_UNSUPPORTED_LANG                             MAKE_SAPI_ERROR(0x059)

/*** SPERR_VOICE_PAUSED                                   0x8004505a    -2147200934
*   The operation cannot be performed because the voice is currently paused.
*/
#define SPERR_VOICE_PAUSED                                 MAKE_SAPI_ERROR(0x05a)

/*** SPERR_AUDIO_BUFFER_UNDERFLOW                          0x8004505b    -2147200933
*   This will only be returned on input (read) streams when the real time audio device
*   stops returning data for a long period of time.
*/
#define SPERR_AUDIO_BUFFER_UNDERFLOW                       MAKE_SAPI_ERROR(0x05b)

/*** SPERR_AUDIO_STOPPED_UNEXPECTEDLY                     0x8004505c    -2147200932
*   An audio device stopped returning data from the Read() method even though it was in
*   the run state.  This error is only returned in the END_SR_STREAM event.
*/
#define SPERR_AUDIO_STOPPED_UNEXPECTEDLY                   MAKE_SAPI_ERROR(0x05c)

/*** SPERR_NO_WORD_PRONUNCIATION                           0x8004505d    -2147200931
*   The SR engine is unable to add this word to a grammar. The application may need to supply 
*   an explicit pronunciation for this word.
*/
#define SPERR_NO_WORD_PRONUNCIATION                        MAKE_SAPI_ERROR(0x05d)

/*** SPERR_ALTERNATES_WOULD_BE_INCONSISTENT                0x8004505e    -2147200930
*   An attempt to call ScaleAudio on a recognition result having previously
*   called GetAlternates. Allowing the call to succeed would result in
*   the previously created alternates located in incorrect audio stream positions.
*/
#define SPERR_ALTERNATES_WOULD_BE_INCONSISTENT             MAKE_SAPI_ERROR(0x05e)

/*** SPERR_NOT_SUPPORTED_FOR_SHARED_RECOGNIZER             0x8004505f    -2147200929
*   The method called is not supported for the shared recognizer.
*   For example, ISpRecognizer::GetInputStream().
*/
#define SPERR_NOT_SUPPORTED_FOR_SHARED_RECOGNIZER          MAKE_SAPI_ERROR(0x05f)

/*** SPERR_TIMEOUT                                         0x80045060    -2147200928
*   A task could not complete because the SR engine had timed out.
*/
#define SPERR_TIMEOUT                                      MAKE_SAPI_ERROR(0x060)

/*** SPERR_REENTER_SYNCHRONIZE                             0x80045061    -2147200927
*   A SR engine called synchronize while inside of a synchronize call.
*/
#define SPERR_REENTER_SYNCHRONIZE                          MAKE_SAPI_ERROR(0x061)

/*** SPERR_STATE_WITH_NO_ARCS                              0x80045062    -2147200926
*   The grammar contains a node no arcs.
*/
#define SPERR_STATE_WITH_NO_ARCS                           MAKE_SAPI_ERROR(0x062)

/*** SPERR_NOT_ACTIVE_SESSION                              0x80045063    -2147200925
*   Neither audio output and input is supported for non-active console sessions.
*/
#define SPERR_NOT_ACTIVE_SESSION                           MAKE_SAPI_ERROR(0x063)

/*** SPERR_ALREADY_DELETED                                 0x80045064    -2147200924
*   The object is a stale reference and is invalid to use.
*   For example having a ISpeechGrammarRule object reference and then calling 
*   ISpeechRecoGrammar::Reset() will cause the rule object to be invalidated.
*   Calling any methods after this will result in this error.
*/
#define SPERR_ALREADY_DELETED                              MAKE_SAPI_ERROR(0x064)

/*** SP_AUDIO_STOPPED                                      0x00045065    282725
*   This can be returned from Read or Write calls audio streams when the stream is stopped.
*/
#define SP_AUDIO_STOPPED                                   MAKE_SAPI_SCODE(0x065)

/*** SPERR_RECOXML_GENERATION_FAIL                             0x80045066    -2147200922
*   The Recognition Parse Tree couldn't be genrated.
*   For example, that the rule name begins with a digit.
*   XML parser doesn't allow element name beginning with a digit.
*/
#define SPERR_RECOXML_GENERATION_FAIL                      MAKE_SAPI_ERROR(0x066)

/*** SPERR_SML_GENERATION_FAIL                             0x80045067    -2147200921
*   The SML couldn't be genrated.
*   For example, the transformation xslt template is not well formed.
*/
#define SPERR_SML_GENERATION_FAIL                          MAKE_SAPI_ERROR(0x067)

/*** SPERR_NOT_PROMPT_VOICE                                0x80045068   -2147200920
*   The current voice is not a prompt voice, so the ISpPromptVoice
*   functions don't work.
*/
#define SPERR_NOT_PROMPT_VOICE                             MAKE_SAPI_ERROR(0x068)

/*** SPERR_ROOTRULE_ALREADY_DEFINED                        0x80045069   -2147200919
*   There is already a root rule for this grammar
*   Defining another root rule will fail.
*/
#define SPERR_ROOTRULE_ALREADY_DEFINED                     MAKE_SAPI_ERROR(0x069)

/*** SPERR_SCRIPT_DISALLOWED                               0x80045070   -2147200912
*   Support for embedded script not supported because browser security settings have disabled it
*/
#define SPERR_SCRIPT_DISALLOWED                            MAKE_SAPI_ERROR(0x070)

/*** SPERR_REMOTE_CALL_TIMED_OUT_START                     0x80045071    -2147200911
*   A time out occurred starting the sapi server
*/
#define SPERR_REMOTE_CALL_TIMED_OUT_START                  MAKE_SAPI_ERROR(0x071)

/*** SPERR_REMOTE_CALL_TIMED_OUT_CONNECT                   0x80045072    -2147200910
*   A timeout occurred obtaining the lock for starting or connecting to sapi server 
*/
#define SPERR_REMOTE_CALL_TIMED_OUT_CONNECT                MAKE_SAPI_ERROR(0x072)

/*** SPERR_SECMGR_CHANGE_NOT_ALLOWED                       0x80045073    -2147200909
*   When there is a cfg grammar loaded, we don't allow changing the security manager
*/
#define SPERR_SECMGR_CHANGE_NOT_ALLOWED                    MAKE_SAPI_ERROR(0x073)

/*** SP_COMPLETE_BUT_EXTENDABLE                            0x00045074    282740
*   Parse is valid but could be extendable (internal use only)
*/
#define SP_COMPLETE_BUT_EXTENDABLE                         MAKE_SAPI_SCODE(0x074)

/*** SPERR_FAILED_TO_DELETE_FILE                           0x80045075    -2147200907
*   Tried and failed to delete an existing file.
*/
#define SPERR_FAILED_TO_DELETE_FILE                        MAKE_SAPI_ERROR(0x075)

/*** SPERR_SHARED_ENGINE_DISABLED                          0x80045076    -2147200906
*   The user has chosen to disable speech from running on the machine, or the 
*   system is not set up to run speech {e.g. initial setup and tutorial has not been run}.    
*/
#define SPERR_SHARED_ENGINE_DISABLED                       MAKE_SAPI_ERROR(0x076)

/*** SPERR_RECOGNIZER_NOT_FOUND                            0x80045077    -2147200905
*   No recognizer is installed.    
*/
#define SPERR_RECOGNIZER_NOT_FOUND                         MAKE_SAPI_ERROR(0x077)

/*** SPERR_AUDIO_NOT_FOUND                                 0x80045078    -2147200904
*   No audio device is installed.    
*/
#define SPERR_AUDIO_NOT_FOUND                              MAKE_SAPI_ERROR(0x078)

/*** SPERR_NO_VOWEL                                        0x80045079    -2147200903
*   No Vowel in a word 
*/
#define SPERR_NO_VOWEL                                     MAKE_SAPI_ERROR(0x079)

/*** SPERR_UNSUPPORTED_PHONEME                             0x8004507A    -2147200902
*   Unknown phoneme
*/
#define SPERR_UNSUPPORTED_PHONEME                          MAKE_SAPI_ERROR(0x07A)

/*** SP_NO_RULES_TO_ACTIVATE                               0x0004507B    282747
*   The grammar does not have any root or top-level active rules to activate.
*/
#define SP_NO_RULES_TO_ACTIVATE                            MAKE_SAPI_SCODE(0x07B)

/*** SP_NO_WORDENTRY_NOTIFICATION                          0x0004507C    282748
*   The engine does not need SAPI word entry handles for this grammar
*/
#define SP_NO_WORDENTRY_NOTIFICATION                       MAKE_SAPI_SCODE(0x07C)


/*** SPERR_WORD_NEEDS_NORMALIZATION                        0x8004507D    -2147200899
*   The word passed to the GetPronunciations interface needs normalizing first
*/
#define SPERR_WORD_NEEDS_NORMALIZATION                     MAKE_SAPI_ERROR(0x07D)

/*** SPERR_CANNOT_NORMALIZE                                0x8004507E    -2147200898
*   The word passed to the normalize interface cannot be normalized
*/
#define SPERR_CANNOT_NORMALIZE                             MAKE_SAPI_ERROR(0x07E)

/*** S_LIMIT_REACHED                                       0x0004507F    282751
*   The word being normalized has generated more than the maximum number of allowed normalized results
*   Indicates that returned list is not exhaustive, but contains as many alternatives as the engine is willing to provide.
*/
#define S_LIMIT_REACHED                                    MAKE_SAPI_SCODE(0x07F)

/*** S_NOTSUPPORTED                                        0x00045080    282752
*   We currently don't support this combination of function call + input
*/
#define S_NOTSUPPORTED                                    MAKE_SAPI_SCODE(0x080)

/*** SPERR_TOPIC_NOT_ADAPTABLE                            0x80045081    -2147200895
*   This topic is not adaptable
*/
#define SPERR_TOPIC_NOT_ADAPTABLE                         MAKE_SAPI_ERROR(0x081)

/*** SPERR_PHONEME_CONVERSION                            0x80045082    -2147200894
*   Cannot convert the phonemes to the specified phonetic alphabet.
*/
#define SPERR_PHONEME_CONVERSION                          MAKE_SAPI_ERROR(0x082)

/*** SPERR_NOT_SUPPORTED_FOR_INPROC_RECOGNIZER           0x80045083    -2147200893
*   The method called is not supported for the in-process recognizer.
*   For example: SetTextFeedback
*/
#define SPERR_NOT_SUPPORTED_FOR_INPROC_RECOGNIZER         MAKE_SAPI_ERROR(0x083)

/*** SPERR_OVERLOAD                 0x80045084         -2147200892
*   The operation cannot be carried out due to overload and should be attempted again.
*/
#define SPERR_OVERLOAD                                  MAKE_SAPI_ERROR(0x084)

/*** SPERR_LEX_INVALID_DATA         0x80045085         -2147200891
*   The lexicon data is invalid or corrupted.
*/
#define SPERR_LEX_INVALID_DATA                          MAKE_SAPI_ERROR(0x085)

/*** SPERR_CFG_INVALID_DATA         0x80045086         -2147200890
*   The data in the CFG grammar is invalid or corrupted
*/
#define SPERR_CFG_INVALID_DATA                          MAKE_SAPI_ERROR(0x086)

/*** SPERR_LEX_UNEXPECTED_FORMAT      0x80045087       -2147200889
*   The lexicon is not in the expected format.
*/
#define SPERR_LEX_UNEXPECTED_FORMAT                     MAKE_SAPI_ERROR(0x087)

/*** SPERR_STRING_TOO_LONG          0x80045088         -2147200888
*   The string is too long.
*/
#define SPERR_STRING_TOO_LONG                           MAKE_SAPI_ERROR(0x088)

/*** SPERR_STRING_EMPTY             0x80045089         -2147200887
*   The string cannot be empty.
*/
#define SPERR_STRING_EMPTY                              MAKE_SAPI_ERROR(0x089)
 
/*** SPERR_NON_WORD_TRANSITION      0x80045090         -2147200880
*   One of the parse transitions is not a word transition.
*/
#define SPERR_NON_WORD_TRANSITION                       MAKE_SAPI_ERROR(0x090)

/*** SPERR_SISR_ATTRIBUTES_NOT_ALLOWED     0x80045091   -2147200879
*   Attributes are not allowed at the top level.
*/
#define SPERR_SISR_ATTRIBUTES_NOT_ALLOWED               MAKE_SAPI_ERROR(0x091)

/*** SPERR_SISR_MIXED_NOT_ALLOWED          0x80045092   -2147200878
*   Mixed content is not allowed at the top level.
*/
#define SPERR_SISR_MIXED_NOT_ALLOWED                    MAKE_SAPI_ERROR(0x092)


/*** SPERR_VOICE_NOT_FOUND                 0x80045093   -2147200877
*   NO given voice is found
*/
#define SPERR_VOICE_NOT_FOUND                           MAKE_SAPI_ERROR(0x093)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  //--- This must be the last line in the file


