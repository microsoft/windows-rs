/************************************************************************
*                                                                       *
*   dmerror.h -- Error codes returned by DirectMusic API's              *
*                                                                       *
*   Copyright (c) Microsoft Corporation.  All rights reserved.          *
*                                                                       *
************************************************************************/

#ifndef _DMERROR_
#define _DMERROR_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define DMUS_ERRBASE              0x1000      /* Make error codes human readable in hex */

#ifndef MAKE_HRESULT
#define MAKE_HRESULT(sev,fac,code) \
    ((HRESULT) (((unsigned long)(sev)<<31) | ((unsigned long)(fac)<<16) | ((unsigned long)(code))) )
#endif

#define MAKE_DMHRESULTSUCCESS(code)     MAKE_HRESULT(0, FACILITY_DIRECTMUSIC, (DMUS_ERRBASE + (code)))
#define MAKE_DMHRESULTERROR(code)       MAKE_HRESULT(1, FACILITY_DIRECTMUSIC, (DMUS_ERRBASE + (code)))

/* DMUS_S_PARTIALLOAD
 *
 * The object could only load partially. This can happen if some components are
 * not registered properly, such as embedded tracks and tools. This can also happen
 * if some content is missing. For example, if a segment uses a DLS collection that
 * is not in the loader's current search directory.
 */
#define DMUS_S_PARTIALLOAD              MAKE_DMHRESULTSUCCESS(0x091)

/* DMUS_S_PARTIALDOWNLOAD
 *
 * Return value from IDirectMusicBand::Download() which indicates that
 * some of the instruments safely downloaded, but others failed. This usually
 * occurs when some instruments are on PChannels not supported by the performance
 * or port.
 */
#define DMUS_S_PARTIALDOWNLOAD          MAKE_DMHRESULTSUCCESS(0x092)

/* DMUS_S_REQUEUE
 *
 * Return value from IDirectMusicTool::ProcessPMsg() which indicates to the
 * performance that it should cue the PMsg again automatically.
 */
#define DMUS_S_REQUEUE                  MAKE_DMHRESULTSUCCESS(0x200)

/* DMUS_S_FREE
 *
 * Return value from IDirectMusicTool::ProcessPMsg() which indicates to the
 * performance that it should free the PMsg automatically.
 */
#define DMUS_S_FREE                     MAKE_DMHRESULTSUCCESS(0x201)

/* DMUS_S_END
 *
 * Return value from IDirectMusicTrack::Play() which indicates to the
 * segment that the track has no more data after mtEnd.
 */
#define DMUS_S_END                      MAKE_DMHRESULTSUCCESS(0x202)

/* DMUS_S_STRING_TRUNCATED
 *
 * Returned string has been truncated to fit the buffer size.
 */
#define DMUS_S_STRING_TRUNCATED         MAKE_DMHRESULTSUCCESS(0x210)

/* DMUS_S_LAST_TOOL
 *
 * Returned from IDirectMusicGraph::StampPMsg(), this indicates that the PMsg
 * is already stamped with the last tool in the graph. The returned PMsg's
 * tool pointer is now NULL.
 */
#define DMUS_S_LAST_TOOL                MAKE_DMHRESULTSUCCESS(0x211)

/* DMUS_S_OVER_CHORD
 *
 * Returned from IDirectMusicPerformance::MusicToMIDI(), this indicates
 * that no note has been calculated because the music value has the note
 * at a position higher than the top note of the chord. This applies only
 * to DMUS_PLAYMODE_NORMALCHORD play mode. This success code indicates
 * that the caller should not do anything with the note. It is not meant
 * to be played against this chord.
 */
#define DMUS_S_OVER_CHORD               MAKE_DMHRESULTSUCCESS(0x212)

/* DMUS_S_UP_OCTAVE
 *
 * Returned from IDirectMusicPerformance::MIDIToMusic(),  and
 * IDirectMusicPerformance::MusicToMIDI(), this indicates
 * that the note conversion generated a note value that is below 0,
 * so it has been bumped up one or more octaves to be in the proper
 * MIDI range of 0 through 127.
 * Note that this is valid for MIDIToMusic() when using play modes
 * DMUS_PLAYMODE_FIXEDTOCHORD and DMUS_PLAYMODE_FIXEDTOKEY, both of
 * which store MIDI values in wMusicValue. With MusicToMIDI(), it is
 * valid for all play modes.
 * Ofcourse, DMUS_PLAYMODE_FIXED will never return this success code.
 */
#define DMUS_S_UP_OCTAVE                MAKE_DMHRESULTSUCCESS(0x213)

/* DMUS_S_DOWN_OCTAVE
 *
 * Returned from IDirectMusicPerformance::MIDIToMusic(),  and
 * IDirectMusicPerformance::MusicToMIDI(), this indicates
 * that the note conversion generated a note value that is above 127,
 * so it has been bumped down one or more octaves to be in the proper
 * MIDI range of 0 through 127.
 * Note that this is valid for MIDIToMusic() when using play modes
 * DMUS_PLAYMODE_FIXEDTOCHORD and DMUS_PLAYMODE_FIXEDTOKEY, both of
 * which store MIDI values in wMusicValue. With MusicToMIDI(), it is
 * valid for all play modes.
 * Ofcourse, DMUS_PLAYMODE_FIXED will never return this success code.
 */
#define DMUS_S_DOWN_OCTAVE              MAKE_DMHRESULTSUCCESS(0x214)

/* DMUS_S_NOBUFFERCONTROL
 *
 * Although the audio output from the port will be routed to the
 * same device as the given DirectSound buffer, buffer controls
 * such as pan and volume will not affect the output.
 *
 */
#define DMUS_S_NOBUFFERCONTROL          MAKE_DMHRESULTSUCCESS(0x215)

#if (NTDDI_VERSION >= NTDDI_WINXP) /* Windows XP or greater */

/* DMUS_S_GARBAGE_COLLECTED
 *
 * The requested operation was not performed because during CollectGarbage
 * the loader determined that the object had been released.
 */
#define DMUS_S_GARBAGE_COLLECTED        MAKE_DMHRESULTSUCCESS(0x216)

#endif

/* DMUS_E_DRIVER_FAILED
 *
 * An unexpected error was returned from a device driver, indicating
 * possible failure of the driver or hardware.
 */
#define DMUS_E_DRIVER_FAILED            MAKE_DMHRESULTERROR(0x0101)

/* DMUS_E_PORTS_OPEN
 *
 * The requested operation cannot be performed while there are
 * instantiated ports in any process in the system.
 */
#define DMUS_E_PORTS_OPEN               MAKE_DMHRESULTERROR(0x0102)

/* DMUS_E_DEVICE_IN_USE
 *
 * The requested device is already in use (possibly by a non-DirectMusic
 * client) and cannot be opened again.
 */
#define DMUS_E_DEVICE_IN_USE            MAKE_DMHRESULTERROR(0x0103)

/* DMUS_E_INSUFFICIENTBUFFER
 *
 * Buffer is not large enough for requested operation.
 */
#define DMUS_E_INSUFFICIENTBUFFER       MAKE_DMHRESULTERROR(0x0104)

/* DMUS_E_BUFFERNOTSET
 *
 * No buffer was prepared for the download data.
 */
#define DMUS_E_BUFFERNOTSET             MAKE_DMHRESULTERROR(0x0105)

/* DMUS_E_BUFFERNOTAVAILABLE
 *
 * Download failed due to inability to access or create download buffer.
 */
#define DMUS_E_BUFFERNOTAVAILABLE       MAKE_DMHRESULTERROR(0x0106)

/* DMUS_E_NOTADLSCOL
 *
 * Error parsing DLS collection. File is corrupt.
 */
#define DMUS_E_NOTADLSCOL               MAKE_DMHRESULTERROR(0x0108)

/* DMUS_E_INVALIDOFFSET
 *
 * Wave chunks in DLS collection file are at incorrect offsets.
 */
#define DMUS_E_INVALIDOFFSET            MAKE_DMHRESULTERROR(0x0109)

/* DMUS_E_ALREADY_LOADED
 *
 * Second attempt to load a DLS collection that is currently open.
 */
#define DMUS_E_ALREADY_LOADED           MAKE_DMHRESULTERROR(0x0111)

/* DMUS_E_INVALIDPOS
 *
 * Error reading wave data from DLS collection. Indicates bad file.
 */
#define DMUS_E_INVALIDPOS               MAKE_DMHRESULTERROR(0x0113)

/* DMUS_E_INVALIDPATCH
 *
 * There is no instrument in the collection that matches patch number.
 */
#define DMUS_E_INVALIDPATCH             MAKE_DMHRESULTERROR(0x0114)

/* DMUS_E_CANNOTSEEK
 *
 * The IStream* doesn't support Seek().
 */
#define DMUS_E_CANNOTSEEK               MAKE_DMHRESULTERROR(0x0115)

/* DMUS_E_CANNOTWRITE
 *
 * The IStream* doesn't support Write().
 */
#define DMUS_E_CANNOTWRITE              MAKE_DMHRESULTERROR(0x0116)

/* DMUS_E_CHUNKNOTFOUND
 *
 * The RIFF parser doesn't contain a required chunk while parsing file.
 */
#define DMUS_E_CHUNKNOTFOUND            MAKE_DMHRESULTERROR(0x0117)

/* DMUS_E_INVALID_DOWNLOADID
 *
 * Invalid download id was used in the process of creating a download buffer.
 */
#define DMUS_E_INVALID_DOWNLOADID       MAKE_DMHRESULTERROR(0x0119)

/* DMUS_E_NOT_DOWNLOADED_TO_PORT
 *
 * Tried to unload an object that was not downloaded or previously unloaded.
 */
#define DMUS_E_NOT_DOWNLOADED_TO_PORT   MAKE_DMHRESULTERROR(0x0120)

/* DMUS_E_ALREADY_DOWNLOADED
 *
 * Buffer was already downloaded to synth.
 */
#define DMUS_E_ALREADY_DOWNLOADED       MAKE_DMHRESULTERROR(0x0121)

/* DMUS_E_UNKNOWN_PROPERTY
 *
 * The specified property item was not recognized by the target object.
 */
#define DMUS_E_UNKNOWN_PROPERTY         MAKE_DMHRESULTERROR(0x0122)

/* DMUS_E_SET_UNSUPPORTED
 *
 * The specified property item may not be set on the target object.
 */
#define DMUS_E_SET_UNSUPPORTED          MAKE_DMHRESULTERROR(0x0123)

/* DMUS_E_GET_UNSUPPORTED
 *
 * The specified property item may not be retrieved from the target object.
 */
#define DMUS_E_GET_UNSUPPORTED          MAKE_DMHRESULTERROR(0x0124)

/* DMUS_E_NOTMONO
 *
 * Wave chunk has more than one interleaved channel. DLS format requires MONO.
 */
#define DMUS_E_NOTMONO                  MAKE_DMHRESULTERROR(0x0125)

/* DMUS_E_BADARTICULATION
 *
 * Invalid articulation chunk in DLS collection.
 */
#define DMUS_E_BADARTICULATION          MAKE_DMHRESULTERROR(0x0126)

/* DMUS_E_BADINSTRUMENT
 *
 * Invalid instrument chunk in DLS collection.
 */
#define DMUS_E_BADINSTRUMENT            MAKE_DMHRESULTERROR(0x0127)

/* DMUS_E_BADWAVELINK
 *
 * Wavelink chunk in DLS collection points to invalid wave.
 */
#define DMUS_E_BADWAVELINK              MAKE_DMHRESULTERROR(0x0128)

/* DMUS_E_NOARTICULATION
 *
 * Articulation missing from instrument in DLS collection.
 */
#define DMUS_E_NOARTICULATION           MAKE_DMHRESULTERROR(0x0129)

/* DMUS_E_NOTPCM
 *
 * Downoaded DLS wave is not in PCM format.
*/
#define DMUS_E_NOTPCM                   MAKE_DMHRESULTERROR(0x012A)

/* DMUS_E_BADWAVE
 *
 * Bad wave chunk in DLS collection
 */
#define DMUS_E_BADWAVE                  MAKE_DMHRESULTERROR(0x012B)

/* DMUS_E_BADOFFSETTABLE
 *
 * Offset Table for download buffer has errors.
 */
#define DMUS_E_BADOFFSETTABLE           MAKE_DMHRESULTERROR(0x012C)

/* DMUS_E_UNKNOWNDOWNLOAD
 *
 * Attempted to download unknown data type.
 */
#define DMUS_E_UNKNOWNDOWNLOAD          MAKE_DMHRESULTERROR(0x012D)

/* DMUS_E_NOSYNTHSINK
 *
 * The operation could not be completed because no sink was connected to
 * the synthesizer.
 */
#define DMUS_E_NOSYNTHSINK              MAKE_DMHRESULTERROR(0x012E)

/* DMUS_E_ALREADYOPEN
 *
 * An attempt was made to open the software synthesizer while it was already
 * open.
 * ASSERT?
 */
#define DMUS_E_ALREADYOPEN              MAKE_DMHRESULTERROR(0x012F)

/* DMUS_E_ALREADYCLOSE
 *
 * An attempt was made to close the software synthesizer while it was already
 * open.
 * ASSERT?
 */
#define DMUS_E_ALREADYCLOSED            MAKE_DMHRESULTERROR(0x0130)

/* DMUS_E_SYNTHNOTCONFIGURED
 *
 * The operation could not be completed because the software synth has not
 * yet been fully configured.
 * ASSERT?
 */
#define DMUS_E_SYNTHNOTCONFIGURED       MAKE_DMHRESULTERROR(0x0131)

/* DMUS_E_SYNTHACTIVE
 *
 * The operation cannot be carried out while the synthesizer is active.
 */
#define DMUS_E_SYNTHACTIVE              MAKE_DMHRESULTERROR(0x0132)

/* DMUS_E_CANNOTREAD
 *
 * An error occurred while attempting to read from the IStream* object.
 */
#define DMUS_E_CANNOTREAD               MAKE_DMHRESULTERROR(0x0133)

/* DMUS_E_DMUSIC_RELEASED
 *
 * The operation cannot be performed because the final instance of the
 * DirectMusic object was released. Ports cannot be used after final
 * release of the DirectMusic object.
 */
#define DMUS_E_DMUSIC_RELEASED          MAKE_DMHRESULTERROR(0x0134)

/* DMUS_E_BUFFER_EMPTY
 *
 * There was no data in the referenced buffer.
 */
#define DMUS_E_BUFFER_EMPTY             MAKE_DMHRESULTERROR(0x0135)

/* DMUS_E_BUFFER_FULL
 *
 * There is insufficient space to insert the given event into the buffer.
 */
#define DMUS_E_BUFFER_FULL              MAKE_DMHRESULTERROR(0x0136)

/* DMUS_E_PORT_NOT_CAPTURE
 *
 * The given operation could not be carried out because the port is a
 * capture port.
 */
#define DMUS_E_PORT_NOT_CAPTURE         MAKE_DMHRESULTERROR(0x0137)

/* DMUS_E_PORT_NOT_RENDER
 *
 * The given operation could not be carried out because the port is a
 * render port.
 */
#define DMUS_E_PORT_NOT_RENDER          MAKE_DMHRESULTERROR(0x0138)

/* DMUS_E_DSOUND_NOT_SET
 *
 * The port could not be created because no DirectSound has been specified.
 * Specify a DirectSound interface via the IDirectMusic::SetDirectSound
 * method; pass NULL to have DirectMusic manage usage of DirectSound.
 */
#define DMUS_E_DSOUND_NOT_SET           MAKE_DMHRESULTERROR(0x0139)

/* DMUS_E_ALREADY_ACTIVATED
 *
 * The operation cannot be carried out while the port is active.
 */
#define DMUS_E_ALREADY_ACTIVATED        MAKE_DMHRESULTERROR(0x013A)

/* DMUS_E_INVALIDBUFFER
 *
 * Invalid DirectSound buffer was handed to port.
 */
#define DMUS_E_INVALIDBUFFER            MAKE_DMHRESULTERROR(0x013B)

/* DMUS_E_WAVEFORMATNOTSUPPORTED
 *
 * Invalid buffer format was handed to the synth sink.
 */
#define DMUS_E_WAVEFORMATNOTSUPPORTED   MAKE_DMHRESULTERROR(0x013C)

/* DMUS_E_SYNTHINACTIVE
 *
 * The operation cannot be carried out while the synthesizer is inactive.
 */
#define DMUS_E_SYNTHINACTIVE            MAKE_DMHRESULTERROR(0x013D)

/* DMUS_E_DSOUND_ALREADY_SET
 *
 * IDirectMusic::SetDirectSound has already been called. It may not be
 * changed while in use.
 */
#define DMUS_E_DSOUND_ALREADY_SET       MAKE_DMHRESULTERROR(0x013E)

/* DMUS_E_INVALID_EVENT
 *
 * The given event is invalid (either it is not a valid MIDI message
 * or it makes use of running status). The event cannot be packed
 * into the buffer.
 */
#define DMUS_E_INVALID_EVENT            MAKE_DMHRESULTERROR(0x013F)

/* DMUS_E_UNSUPPORTED_STREAM
 *
 * The IStream* object does not contain data supported by the loading object.
 */
#define DMUS_E_UNSUPPORTED_STREAM       MAKE_DMHRESULTERROR(0x0150)

/* DMUS_E_ALREADY_INITED
 *
 * The object has already been initialized.
 */
#define DMUS_E_ALREADY_INITED           MAKE_DMHRESULTERROR(0x0151)

/* DMUS_E_INVALID_BAND
 *
 * The file does not contain a valid band.
 */
#define DMUS_E_INVALID_BAND             MAKE_DMHRESULTERROR(0x0152)

/* DMUS_E_TRACK_HDR_NOT_FIRST_CK
 *
 * The IStream* object's data does not have a track header as the first chunk,
 * and therefore can not be read by the segment object.
 */
#define DMUS_E_TRACK_HDR_NOT_FIRST_CK   MAKE_DMHRESULTERROR(0x0155)

/* DMUS_E_TOOL_HDR_NOT_FIRST_CK
 *
 * The IStream* object's data does not have a tool header as the first chunk,
 * and therefore can not be read by the graph object.
 */
#define DMUS_E_TOOL_HDR_NOT_FIRST_CK    MAKE_DMHRESULTERROR(0x0156)

/* DMUS_E_INVALID_TRACK_HDR
 *
 * The IStream* object's data contains an invalid track header (ckid is 0 and
 * fccType is NULL,) and therefore can not be read by the segment object.
 */
#define DMUS_E_INVALID_TRACK_HDR        MAKE_DMHRESULTERROR(0x0157)

/* DMUS_E_INVALID_TOOL_HDR
 *
 * The IStream* object's data contains an invalid tool header (ckid is 0 and
 * fccType is NULL,) and therefore can not be read by the graph object.
 */
#define DMUS_E_INVALID_TOOL_HDR         MAKE_DMHRESULTERROR(0x0158)

/* DMUS_E_ALL_TOOLS_FAILED
 *
 * The graph object was unable to load all tools from the IStream* object data.
 * This may be due to errors in the stream, or the tools being incorrectly
 * registered on the client.
 */
#define DMUS_E_ALL_TOOLS_FAILED         MAKE_DMHRESULTERROR(0x0159)

/* DMUS_E_ALL_TRACKS_FAILED
 *
 * The segment object was unable to load all tracks from the IStream* object data.
 * This may be due to errors in the stream, or the tracks being incorrectly
 * registered on the client.
 */
#define DMUS_E_ALL_TRACKS_FAILED        MAKE_DMHRESULTERROR(0x0160)

/* DMUS_E_NOT_FOUND
 *
 * The requested item was not contained by the object.
 */
#define DMUS_E_NOT_FOUND                MAKE_DMHRESULTERROR(0x0161)

/* DMUS_E_NOT_INIT
 *
 * A required object is not initialized or failed to initialize.
 */
#define DMUS_E_NOT_INIT                 MAKE_DMHRESULTERROR(0x0162)

/* DMUS_E_TYPE_DISABLED
 *
 * The requested parameter type is currently disabled. Parameter types may
 * be enabled and disabled by certain calls to SetParam().
 */
#define DMUS_E_TYPE_DISABLED            MAKE_DMHRESULTERROR(0x0163)

/* DMUS_E_TYPE_UNSUPPORTED
 *
 * The requested parameter type is not supported on the object.
 */
#define DMUS_E_TYPE_UNSUPPORTED         MAKE_DMHRESULTERROR(0x0164)

/* DMUS_E_TIME_PAST
 *
 * The time is in the past, and the operation can not succeed.
 */
#define DMUS_E_TIME_PAST                MAKE_DMHRESULTERROR(0x0165)

/* DMUS_E_TRACK_NOT_FOUND
 *
 * The requested track is not contained by the segment.
 */
#define DMUS_E_TRACK_NOT_FOUND			MAKE_DMHRESULTERROR(0x0166)

#if (NTDDI_VERSION >= NTDDI_WINXP) /* Windows XP or greater */

/* DMUS_E_TRACK_NO_CLOCKTIME_SUPPORT
 *
 * The track does not support clock time playback or getparam.
 */
#define DMUS_E_TRACK_NO_CLOCKTIME_SUPPORT   MAKE_DMHRESULTERROR(0x0167)

#endif

/* DMUS_E_NO_MASTER_CLOCK
 *
 * There is no master clock in the performance. Be sure to call
 * IDirectMusicPerformance::Init().
 */
#define DMUS_E_NO_MASTER_CLOCK          MAKE_DMHRESULTERROR(0x0170)

/* DMUS_E_LOADER_NOCLASSID
 *
 * The class id field is required and missing in the DMUS_OBJECTDESC.
 */
#define DMUS_E_LOADER_NOCLASSID         MAKE_DMHRESULTERROR(0x0180)

/* DMUS_E_LOADER_BADPATH
 *
 * The requested file path is invalid.
 */
#define DMUS_E_LOADER_BADPATH           MAKE_DMHRESULTERROR(0x0181)

/* DMUS_E_LOADER_FAILEDOPEN
 *
 * File open failed - either file doesn't exist or is locked.
 */
#define DMUS_E_LOADER_FAILEDOPEN        MAKE_DMHRESULTERROR(0x0182)

/* DMUS_E_LOADER_FORMATNOTSUPPORTED
 *
 * Search data type is not supported.
 */
#define DMUS_E_LOADER_FORMATNOTSUPPORTED    MAKE_DMHRESULTERROR(0x0183)

/* DMUS_E_LOADER_FAILEDCREATE
 *
 * Unable to find or create object.
 */
#define DMUS_E_LOADER_FAILEDCREATE      MAKE_DMHRESULTERROR(0x0184)

/* DMUS_E_LOADER_OBJECTNOTFOUND
 *
 * Object was not found.
 */
#define DMUS_E_LOADER_OBJECTNOTFOUND    MAKE_DMHRESULTERROR(0x0185)

/* DMUS_E_LOADER_NOFILENAME
 *
 * The file name is missing from the DMUS_OBJECTDESC.
 */
#define DMUS_E_LOADER_NOFILENAME	    MAKE_DMHRESULTERROR(0x0186)

/* DMUS_E_INVALIDFILE
 *
 * The file requested is not a valid file.
 */
#define DMUS_E_INVALIDFILE              MAKE_DMHRESULTERROR(0x0200)

/* DMUS_E_ALREADY_EXISTS
 *
 * The tool is already contained in the graph. Create a new instance.
 */
#define DMUS_E_ALREADY_EXISTS           MAKE_DMHRESULTERROR(0x0201)

/* DMUS_E_OUT_OF_RANGE
 *
 * Value is out of range, for instance the requested length is longer than
 * the segment.
 */
#define DMUS_E_OUT_OF_RANGE             MAKE_DMHRESULTERROR(0x0202)

/* DMUS_E_SEGMENT_INIT_FAILED
 *
 * Segment initialization failed, most likely due to a critical memory situation.
 */
#define DMUS_E_SEGMENT_INIT_FAILED      MAKE_DMHRESULTERROR(0x0203)

/* DMUS_E_ALREADY_SENT
 *
 * The DMUS_PMSG has already been sent to the performance object via
 * IDirectMusicPerformance::SendPMsg().
 */
#define DMUS_E_ALREADY_SENT             MAKE_DMHRESULTERROR(0x0204)

/* DMUS_E_CANNOT_FREE
 *
 * The DMUS_PMSG was either not allocated by the performance via
 * IDirectMusicPerformance::AllocPMsg(), or it was already freed via
 * IDirectMusicPerformance::FreePMsg().
 */
#define DMUS_E_CANNOT_FREE              MAKE_DMHRESULTERROR(0x0205)

/* DMUS_E_CANNOT_OPEN_PORT
 *
 * The default system port could not be opened.
 */
#define DMUS_E_CANNOT_OPEN_PORT         MAKE_DMHRESULTERROR(0x0206)

/* DMUS_E_CANNOT_CONVERT
 *
 * A call to MIDIToMusic() or MusicToMIDI() resulted in an error because
 * the requested conversion could not happen. This usually occurs when the
 * provided DMUS_CHORD_KEY structure has an invalid chord or scale pattern.
 */
#define DMUS_E_CANNOT_CONVERT           MAKE_DMHRESULTERROR(0x0207)
/* misspelling in previous versions of DirectX preserved for backward compatibility */
#define DMUS_E_CONNOT_CONVERT           DMUS_E_CANNOT_CONVERT

/* DMUS_E_DESCEND_CHUNK_FAIL
 *
 * DMUS_E_DESCEND_CHUNK_FAIL is returned when the end of the file
 * was reached before the desired chunk was found.
 */
#define DMUS_E_DESCEND_CHUNK_FAIL       MAKE_DMHRESULTERROR(0x0210)


#if (NTDDI_VERSION >= NTDDI_WINXP) /* Windows XP or greater */

/* DMUS_E_NOT_LOADED
 *
 * An attempt to use this object failed because it first needs to
 * be loaded.
 */
#define DMUS_E_NOT_LOADED               MAKE_DMHRESULTERROR(0x0211)

/* DMUS_E_SCRIPT_LANGUAGE_INCOMPATIBLE
 *
 * The activeX scripting engine for the script's language is not compatible with
 * DirectMusic.
 *
 */
#define DMUS_E_SCRIPT_LANGUAGE_INCOMPATIBLE  MAKE_DMHRESULTERROR(0x0213)

/* DMUS_E_SCRIPT_UNSUPPORTED_VARTYPE
 *
 * A varient was used that had a type that is not supported by DirectMusic.
 *
 */
#define DMUS_E_SCRIPT_UNSUPPORTED_VARTYPE    MAKE_DMHRESULTERROR(0x0214)

/* DMUS_E_SCRIPT_ERROR_IN_SCRIPT
 *
 * An error was encountered while parsing or executing the script.
 * The pErrorInfo parameter (if supplied) was filled with information about the error.
 */
#define DMUS_E_SCRIPT_ERROR_IN_SCRIPT        MAKE_DMHRESULTERROR(0x0215)

/* DMUS_E_SCRIPT_CANTLOAD_OLEAUT32
 *
 * Loading of oleaut32.dll failed.  VBScript and other activeX scripting languages
 * require use of oleaut32.dll.  On platforms where oleaut32.dll is not present, only
 * the DirectMusicScript language, which doesn't require oleaut32.dll can be used.
 */
#define DMUS_E_SCRIPT_CANTLOAD_OLEAUT32      MAKE_DMHRESULTERROR(0x0216)

/* DMUS_E_SCRIPT_LOADSCRIPT_ERROR
 *
 * An error occured while parsing a script loaded using LoadScript.  The script that
 * was loaded contains an error.
 */
#define DMUS_E_SCRIPT_LOADSCRIPT_ERROR       MAKE_DMHRESULTERROR(0x0217)

/* DMUS_E_SCRIPT_INVALID_FILE
 *
 * The script file is invalid.
 */
#define DMUS_E_SCRIPT_INVALID_FILE           MAKE_DMHRESULTERROR(0x0218)

/* DMUS_E_INVALID_SCRIPTTRACK
 *
 * The file contains an invalid script track.
 */
#define DMUS_E_INVALID_SCRIPTTRACK           MAKE_DMHRESULTERROR(0x0219)

/* DMUS_E_SCRIPT_VARIABLE_NOT_FOUND
 *
 * The script does not contain a variable with the specified name.
 */
#define DMUS_E_SCRIPT_VARIABLE_NOT_FOUND     MAKE_DMHRESULTERROR(0x021A)

/* DMUS_E_SCRIPT_ROUTINE_NOT_FOUND
 *
 * The script does not contain a routine with the specified name.
 */
#define DMUS_E_SCRIPT_ROUTINE_NOT_FOUND      MAKE_DMHRESULTERROR(0x021B)

/* DMUS_E_SCRIPT_CONTENT_READONLY
 *
 * Scripts variables for content referenced or embedded in a script cannot be set.
 */
#define DMUS_E_SCRIPT_CONTENT_READONLY       MAKE_DMHRESULTERROR(0x021C)

/* DMUS_E_SCRIPT_NOT_A_REFERENCE
 *
 * Attempt was made to set a script's variable by reference to a value that was
 * not an object type.
 */
#define DMUS_E_SCRIPT_NOT_A_REFERENCE        MAKE_DMHRESULTERROR(0x021D)

/* DMUS_E_SCRIPT_VALUE_NOT_SUPPORTED
 *
 * Attempt was made to set a script's variable by value to an object that does
 * not support a default value property.
 */
#define DMUS_E_SCRIPT_VALUE_NOT_SUPPORTED    MAKE_DMHRESULTERROR(0x021E)

/* DMUS_E_INVALID_SEGMENTTRIGGERTRACK
 *
 * The file contains an invalid segment trigger track.
 */
#define DMUS_E_INVALID_SEGMENTTRIGGERTRACK   MAKE_DMHRESULTERROR(0x0220)

/* DMUS_E_INVALID_LYRICSTRACK
 *
 * The file contains an invalid lyrics track.
 */
#define DMUS_E_INVALID_LYRICSTRACK           MAKE_DMHRESULTERROR(0x0221)

/* DMUS_E_INVALID_PARAMCONTROLTRACK
 *
 * The file contains an invalid parameter control track.
 */
#define DMUS_E_INVALID_PARAMCONTROLTRACK     MAKE_DMHRESULTERROR(0x0222)

/* DMUS_E_AUDIOVBSCRIPT_SYNTAXERROR
 *
 * A script written in AudioVBScript could not be read because it contained a statement that
 * is not allowed by the AudioVBScript language.
 */
#define DMUS_E_AUDIOVBSCRIPT_SYNTAXERROR     MAKE_DMHRESULTERROR(0x0223)

/* DMUS_E_AUDIOVBSCRIPT_RUNTIMEERROR
 *
 * A script routine written in AudioVBScript failed because an invalid operation occurred.  For example,
 * adding the number 3 to a segment object would produce this error.  So would attempting to call a routine
 * that doesn't exist.
 */
#define DMUS_E_AUDIOVBSCRIPT_RUNTIMEERROR     MAKE_DMHRESULTERROR(0x0224)

/* DMUS_E_AUDIOVBSCRIPT_OPERATIONFAILURE
 *
 * A script routine written in AudioVBScript failed because a function outside of a script failed to complete.
 * For example, a call to PlaySegment that fails to play because of low memory would return this error.
 */
#define DMUS_E_AUDIOVBSCRIPT_OPERATIONFAILURE     MAKE_DMHRESULTERROR(0x0225)

/* DMUS_E_AUDIOPATHS_NOT_VALID
 *
 * The Performance has set up some PChannels using the AssignPChannel command, which
 * makes it not capable of supporting audio paths.
 */
#define DMUS_E_AUDIOPATHS_NOT_VALID     MAKE_DMHRESULTERROR(0x0226)

/* DMUS_E_AUDIOPATHS_IN_USE
 *
 * This is the inverse of the previous error.
 * The Performance has set up some audio paths, which makes is incompatible
 * with the calls to allocate pchannels, etc.
 */
#define DMUS_E_AUDIOPATHS_IN_USE     MAKE_DMHRESULTERROR(0x0227)

/* DMUS_E_NO_AUDIOPATH_CONFIG
 *
 * A segment was asked for its embedded audio path configuration,
 * but there isn't any.
 */
#define DMUS_E_NO_AUDIOPATH_CONFIG     MAKE_DMHRESULTERROR(0x0228)

/* DMUS_E_AUDIOPATH_INACTIVE
 *
 * An audiopath is inactive, perhaps because closedown was called.
 */
#define DMUS_E_AUDIOPATH_INACTIVE     MAKE_DMHRESULTERROR(0x0229)

/* DMUS_E_AUDIOPATH_NOBUFFER
 *
 * An audiopath failed to create because a requested buffer could not be created.
 */
#define DMUS_E_AUDIOPATH_NOBUFFER     MAKE_DMHRESULTERROR(0x022A)

/* DMUS_E_AUDIOPATH_NOPORT
 *
 * An audiopath could not be used for playback because it lacked port assignments.
 */
#define DMUS_E_AUDIOPATH_NOPORT     MAKE_DMHRESULTERROR(0x022B)

/* DMUS_E_NO_AUDIOPATH
 *
 * Attempt was made to play segment in audiopath mode and there was no audiopath.
 */
#define DMUS_E_NO_AUDIOPATH     MAKE_DMHRESULTERROR(0x022C)

/* DMUS_E_INVALIDCHUNK
 *
 * Invalid data was found in a RIFF file chunk.
 */
#define DMUS_E_INVALIDCHUNK     MAKE_DMHRESULTERROR(0x022D)

/* DMUS_E_AUDIOPATH_NOGLOBALFXBUFFER
 *
 * Attempt was made to create an audiopath that sends to a global effects buffer which did not exist.
 */
#define DMUS_E_AUDIOPATH_NOGLOBALFXBUFFER     MAKE_DMHRESULTERROR(0x022E)

/* DMUS_E_INVALID_CONTAINER_OBJECT
 *
 * The file does not contain a valid container object.
 */
#define DMUS_E_INVALID_CONTAINER_OBJECT    MAKE_DMHRESULTERROR(0x022F)

#endif /* NTDDI_VERSION >= NTDDI_WINXP */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _DMERROR_ */
