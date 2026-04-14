#include <winapifamily.h>

//*@@@+++@@@@******************************************************************
//
// Copyright (C) Microsoft Corporation. All rights reserved.
//
// Multimedia Registration
//
//*@@@---@@@@******************************************************************
//

// Define the following to skip definitions
//
// NOMMIDS      Multimedia IDs are not defined
// NONEWWAVE    No new waveform types are defined except WAVEFORMATEX
// NONEWRIFF    No new RIFF forms are defined
// NOJPEGDIB    No JPEG DIB definitions
// NONEWIC      No new Image Compressor types are defined
// NOBITMAP     No extended bitmap info header definition

#if !defined(__midl)

#ifndef _INC_MMREG
/* use version number to verify compatibility */
#define _INC_MMREG     158      // version * 100 + revision

#if _MSC_VER > 1000
#pragma once
#endif

#if !defined( RC_INVOKED ) && defined( _MSC_VER )
#if (_MSC_VER <= 800)
#pragma pack(1)
#else
#include "pshpack1.h"   /* Assume byte packing throughout */
#endif
#endif  /* RC_INVOKED */

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif  /* __cplusplus */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef NOMMIDS

/* manufacturer IDs */
#ifndef MM_MICROSOFT
#define   MM_MICROSOFT                  1           /* Microsoft Corporation */
#endif

#define   MM_CREATIVE                   2           /* Creative Labs, Inc. */
#define   MM_MEDIAVISION                3           /* Media Vision, Inc. */
#define   MM_FUJITSU                    4           /* Fujitsu Corp. */
#define   MM_PRAGMATRAX                 5           /* PRAGMATRAX Software */
#define   MM_CYRIX                      6           /* Cyrix Corporation */
#define   MM_PHILIPS_SPEECH_PROCESSING  7           /* Philips Speech Processing */
#define   MM_NETXL                      8           /* NetXL, Inc. */
#define   MM_ZYXEL                      9           /* ZyXEL Communications, Inc. */
#define   MM_BECUBED                    10          /* BeCubed Software Inc. */
#define   MM_AARDVARK                   11          /* Aardvark Computer Systems, Inc. */
#define   MM_BINTEC                     12          /* Bin Tec Communications GmbH */
#define   MM_HEWLETT_PACKARD            13          /* Hewlett-Packard Company */
#define   MM_ACULAB                     14          /* Aculab plc */
#define   MM_FAITH                      15          /* Faith,Inc. */
#define   MM_MITEL                      16          /* Mitel Corporation */
#define   MM_QUANTUM3D                  17          /* Quantum3D, Inc. */
#define   MM_SNI                        18          /* Siemens-Nixdorf */
#define   MM_EMU                        19          /* E-mu Systems, Inc. */
#define   MM_ARTISOFT                   20          /* Artisoft, Inc. */
#define   MM_TURTLE_BEACH               21          /* Turtle Beach, Inc. */
#define   MM_IBM                        22          /* IBM Corporation */
#define   MM_VOCALTEC                   23          /* Vocaltec Ltd. */
#define   MM_ROLAND                     24          /* Roland */
#define   MM_DSP_SOLUTIONS              25          /* DSP Solutions, Inc. */
#define   MM_NEC                        26          /* NEC */
#define   MM_ATI                        27          /* ATI Technologies Inc. */
#define   MM_WANGLABS                   28          /* Wang Laboratories, Inc. */
#define   MM_TANDY                      29          /* Tandy Corporation */
#define   MM_VOYETRA                    30          /* Voyetra */
#define   MM_ANTEX                      31          /* Antex Electronics Corporation */
#define   MM_ICL_PS                     32          /* ICL Personal Systems */
#define   MM_INTEL                      33          /* Intel Corporation */
#define   MM_GRAVIS                     34          /* Advanced Gravis */
#define   MM_VAL                        35          /* Video Associates Labs, Inc. */
#define   MM_INTERACTIVE                36          /* InterActive Inc. */
#define   MM_YAMAHA                     37          /* Yamaha Corporation of America */
#define   MM_EVEREX                     38          /* Everex Systems, Inc. */
#define   MM_ECHO                       39          /* Echo Speech Corporation */
#define   MM_SIERRA                     40          /* Sierra Semiconductor Corp */
#define   MM_CAT                        41          /* Computer Aided Technologies */
#define   MM_APPS                       42          /* APPS Software International */
#define   MM_DSP_GROUP                  43          /* DSP Group, Inc. */
#define   MM_MELABS                     44          /* microEngineering Labs */
#define   MM_COMPUTER_FRIENDS           45          /* Computer Friends, Inc. */
#define   MM_ESS                        46          /* ESS Technology */
#define   MM_AUDIOFILE                  47          /* Audio, Inc. */
#define   MM_MOTOROLA                   48          /* Motorola, Inc. */
#define   MM_CANOPUS                    49          /* Canopus, co., Ltd. */
#define   MM_EPSON                      50          /* Seiko Epson Corporation */
#define   MM_TRUEVISION                 51          /* Truevision */
#define   MM_AZTECH                     52          /* Aztech Labs, Inc. */
#define   MM_VIDEOLOGIC                 53          /* Videologic */
#define   MM_SCALACS                    54          /* SCALACS */
#define   MM_KORG                       55          /* Korg Inc. */
#define   MM_APT                        56          /* Audio Processing Technology */
#define   MM_ICS                        57          /* Integrated Circuit Systems, Inc. */
#define   MM_ITERATEDSYS                58          /* Iterated Systems, Inc. */
#define   MM_METHEUS                    59          /* Metheus */
#define   MM_LOGITECH                   60          /* Logitech, Inc. */
#define   MM_WINNOV                     61          /* Winnov, Inc. */
#define   MM_NCR                        62          /* NCR Corporation */
#define   MM_EXAN                       63          /* EXAN */
#define   MM_AST                        64          /* AST Research Inc. */
#define   MM_WILLOWPOND                 65          /* Willow Pond Corporation */
#define   MM_SONICFOUNDRY               66          /* Sonic Foundry */
#define   MM_VITEC                      67          /* Vitec Multimedia */
#define   MM_MOSCOM                     68          /* MOSCOM Corporation */
#define   MM_SILICONSOFT                69          /* Silicon Soft, Inc. */
#define   MM_TERRATEC                   70          /* TerraTec Electronic GmbH */
#define   MM_MEDIASONIC                 71          /* MediaSonic Ltd. */
#define   MM_SANYO                      72          /* SANYO Electric Co., Ltd. */
#define   MM_SUPERMAC                   73          /* Supermac */
#define   MM_AUDIOPT                    74          /* Audio Processing Technology */
#define   MM_NOGATECH                   75          /* NOGATECH Ltd. */
#define   MM_SPEECHCOMP                 76          /* Speech Compression */
#define   MM_AHEAD                      77          /* Ahead, Inc. */
#define   MM_DOLBY                      78          /* Dolby Laboratories */
#define   MM_OKI                        79          /* OKI */
#define   MM_AURAVISION                 80          /* AuraVision Corporation */
#define   MM_OLIVETTI                   81          /* Ing C. Olivetti & C., S.p.A. */
#define   MM_IOMAGIC                    82          /* I/O Magic Corporation */
#define   MM_MATSUSHITA                 83          /* Matsushita Electric Industrial Co., Ltd. */
#define   MM_CONTROLRES                 84          /* Control Resources Limited */
#define   MM_XEBEC                      85          /* Xebec Multimedia Solutions Limited */
#define   MM_NEWMEDIA                   86          /* New Media Corporation */
#define   MM_NMS                        87          /* Natural MicroSystems */
#define   MM_LYRRUS                     88          /* Lyrrus Inc. */
#define   MM_COMPUSIC                   89          /* Compusic */
#define   MM_OPTI                       90          /* OPTi Computers Inc. */
#define   MM_ADLACC                     91          /* Adlib Accessories Inc. */
#define   MM_COMPAQ                     92          /* Compaq Computer Corp. */
#define   MM_DIALOGIC                   93          /* Dialogic Corporation */
#define   MM_INSOFT                     94          /* InSoft, Inc. */
#define   MM_MPTUS                      95          /* M.P. Technologies, Inc. */
#define   MM_WEITEK                     96          /* Weitek */
#define   MM_LERNOUT_AND_HAUSPIE        97          /* Lernout & Hauspie */
#define   MM_QCIAR                      98          /* Quanta Computer Inc. */
#define   MM_APPLE                      99          /* Apple Computer, Inc. */
#define   MM_DIGITAL                    100         /* Digital Equipment Corporation */
#define   MM_MOTU                       101         /* Mark of the Unicorn */
#define   MM_WORKBIT                    102         /* Workbit Corporation */
#define   MM_OSITECH                    103         /* Ositech Communications Inc. */
#define   MM_MIRO                       104         /* miro Computer Products AG */
#define   MM_CIRRUSLOGIC                105         /* Cirrus Logic */
#define   MM_ISOLUTION                  106         /* ISOLUTION  B.V. */
#define   MM_HORIZONS                   107         /* Horizons Technology, Inc. */
#define   MM_CONCEPTS                   108         /* Computer Concepts Ltd. */
#define   MM_VTG                        109         /* Voice Technologies Group, Inc. */
#define   MM_RADIUS                     110         /* Radius */
#define   MM_ROCKWELL                   111         /* Rockwell International */
#define   MM_XYZ                        112         /* Co. XYZ for testing */
#define   MM_OPCODE                     113         /* Opcode Systems */
#define   MM_VOXWARE                    114         /* Voxware Inc. */
#define   MM_NORTHERN_TELECOM           115         /* Northern Telecom Limited */
#define   MM_APICOM                     116         /* APICOM */
#define   MM_GRANDE                     117         /* Grande Software */
#define   MM_ADDX                       118         /* ADDX */
#define   MM_WILDCAT                    119         /* Wildcat Canyon Software */
#define   MM_RHETOREX                   120         /* Rhetorex Inc. */
#define   MM_BROOKTREE                  121         /* Brooktree Corporation */
#define   MM_ENSONIQ                    125         /* ENSONIQ Corporation */
#define   MM_FAST                       126         /* FAST Multimedia AG */
#define   MM_NVIDIA                     127         /* NVidia Corporation */
#define   MM_OKSORI                     128         /* OKSORI Co., Ltd. */
#define   MM_DIACOUSTICS                129         /* DiAcoustics, Inc. */
#define   MM_GULBRANSEN                 130         /* Gulbransen, Inc. */
#define   MM_KAY_ELEMETRICS             131         /* Kay Elemetrics, Inc. */
#define   MM_CRYSTAL                    132         /* Crystal Semiconductor Corporation */
#define   MM_SPLASH_STUDIOS             133         /* Splash Studios */
#define   MM_QUARTERDECK                134         /* Quarterdeck Corporation */
#define   MM_TDK                        135         /* TDK Corporation */
#define   MM_DIGITAL_AUDIO_LABS         136         /* Digital Audio Labs, Inc. */
#define   MM_SEERSYS                    137         /* Seer Systems, Inc. */
#define   MM_PICTURETEL                 138         /* PictureTel Corporation */
#define   MM_ATT_MICROELECTRONICS       139         /* AT&T Microelectronics */
#define   MM_OSPREY                     140         /* Osprey Technologies, Inc. */
#define   MM_MEDIATRIX                  141         /* Mediatrix Peripherals */
#define   MM_SOUNDESIGNS                142         /* SounDesignS M.C.S. Ltd. */
#define   MM_ALDIGITAL                  143         /* A.L. Digital Ltd. */
#define   MM_SPECTRUM_SIGNAL_PROCESSING 144         /* Spectrum Signal Processing, Inc. */
#define   MM_ECS                        145         /* Electronic Courseware Systems, Inc. */
#define   MM_AMD                        146         /* AMD */
#define   MM_COREDYNAMICS               147         /* Core Dynamics */
#define   MM_CANAM                      148         /* CANAM Computers */
#define   MM_SOFTSOUND                  149         /* Softsound, Ltd. */
#define   MM_NORRIS                     150         /* Norris Communications, Inc. */
#define   MM_DDD                        151         /* Danka Data Devices */
#define   MM_EUPHONICS                  152         /* EuPhonics */
#define   MM_PRECEPT                    153         /* Precept Software, Inc. */
#define   MM_CRYSTAL_NET                154         /* Crystal Net Corporation */
#define   MM_CHROMATIC                  155         /* Chromatic Research, Inc. */
#define   MM_VOICEINFO                  156         /* Voice Information Systems, Inc. */
#define   MM_VIENNASYS                  157         /* Vienna Systems */
#define   MM_CONNECTIX                  158         /* Connectix Corporation */
#define   MM_GADGETLABS                 159         /* Gadget Labs LLC */
#define   MM_FRONTIER                   160         /* Frontier Design Group LLC */
#define   MM_VIONA                      161         /* Viona Development GmbH */
#define   MM_CASIO                      162         /* Casio Computer Co., LTD */
#define   MM_DIAMONDMM                  163         /* Diamond Multimedia */
#define   MM_S3                         164         /* S3 */
#define   MM_DVISION                    165         /* D-Vision Systems, Inc. */
#define   MM_NETSCAPE                   166         /* Netscape Communications */
#define   MM_SOUNDSPACE                 167         /* Soundspace Audio */
#define   MM_VANKOEVERING               168         /* VanKoevering Company */
#define   MM_QTEAM                      169         /* Q-Team */
#define   MM_ZEFIRO                     170         /* Zefiro Acoustics */
#define   MM_STUDER                     171         /* Studer Professional Audio AG */
#define   MM_FRAUNHOFER_IIS             172         /* Fraunhofer IIS */
#define   MM_QUICKNET                   173         /* Quicknet Technologies */
#define   MM_ALARIS                     174         /* Alaris, Inc. */
#define   MM_SICRESOURCE                175         /* SIC Resource Inc. */
#define   MM_NEOMAGIC                   176         /* NeoMagic Corporation */
#define   MM_MERGING_TECHNOLOGIES       177         /* Merging Technologies S.A. */
#define   MM_XIRLINK                    178         /* Xirlink, Inc. */
#define   MM_COLORGRAPH                 179         /* Colorgraph (UK) Ltd */
#define   MM_OTI                        180         /* Oak Technology, Inc. */
#define   MM_AUREAL                     181         /* Aureal Semiconductor */
#define   MM_VIVO                       182         /* Vivo Software */
#define   MM_SHARP                      183         /* Sharp */
#define   MM_LUCENT                     184         /* Lucent Technologies */
#define   MM_ATT                        185         /* AT&T Labs, Inc. */
#define   MM_SUNCOM                     186         /* Sun Communications, Inc. */
#define   MM_SORVIS                     187         /* Sorenson Vision */
#define   MM_INVISION                   188         /* InVision Interactive */
#define   MM_BERKOM                     189         /* Deutsche Telekom Berkom GmbH */
#define   MM_MARIAN                     190         /* Marian GbR Leipzig */
#define   MM_DPSINC                     191         /* Digital Processing Systems, Inc. */
#define   MM_BCB                        192         /* BCB Holdings Inc. */
#define   MM_MOTIONPIXELS               193         /* Motion Pixels */
#define   MM_QDESIGN                    194         /* QDesign Corporation */
#define   MM_NMP                        195         /* Nokia Mobile Phones */
#define   MM_DATAFUSION                 196         /* DataFusion Systems (Pty) (Ltd) */
#define   MM_DUCK                       197         /* The Duck Corporation */
#define   MM_FTR                        198         /* Future Technology Resources Pty Ltd */
#define   MM_BERCOS                     199         /* BERCOS GmbH */
#define   MM_ONLIVE                     200         /* OnLive! Technologies, Inc. */
#define   MM_SIEMENS_SBC                201         /* Siemens Business Communications Systems */
#define   MM_TERALOGIC                  202         /* TeraLogic, Inc. */
#define   MM_PHONET                     203         /* PhoNet Communications Ltd. */
#define   MM_WINBOND                    204         /* Winbond Electronics Corp */
#define   MM_VIRTUALMUSIC               205         /* Virtual Music, Inc. */
#define   MM_ENET                       206         /* e-Net, Inc. */
#define   MM_GUILLEMOT                  207         /* Guillemot International */
#define   MM_EMAGIC                     208         /* Emagic Soft- und Hardware GmbH */
#define   MM_MWM                        209         /* MWM Acoustics LLC */
#define   MM_PACIFICRESEARCH            210         /* Pacific Research and Engineering Corporation */
#define   MM_SIPROLAB                   211         /* Sipro Lab Telecom Inc. */
#define   MM_LYNX                       212         /* Lynx Studio Technology, Inc. */
#define   MM_SPECTRUM_PRODUCTIONS       213         /* Spectrum Productions */
#define   MM_DICTAPHONE                 214         /* Dictaphone Corporation */
#define   MM_QUALCOMM                   215         /* QUALCOMM, Inc. */
#define   MM_RZS                        216         /* Ring Zero Systems, Inc */
#define   MM_AUDIOSCIENCE               217         /* AudioScience Inc. */
#define   MM_PINNACLE                   218         /* Pinnacle Systems, Inc. */
#define   MM_EES                        219         /* EES Technik fuer Musik GmbH */
#define   MM_HAFTMANN                   220         /* haftmann#software */
#define   MM_LUCID                      221         /* Lucid Technology, Symetrix Inc. */
#define   MM_HEADSPACE                  222         /* Headspace, Inc */
#define   MM_UNISYS                     223         /* UNISYS CORPORATION */
#define   MM_LUMINOSITI                 224         /* Luminositi, Inc. */
#define   MM_ACTIVEVOICE                225         /* ACTIVE VOICE CORPORATION */
#define   MM_DTS                        226         /* Digital Theater Systems, Inc. */
#define   MM_DIGIGRAM                   227         /* DIGIGRAM */
#define   MM_SOFTLAB_NSK                228         /* Softlab-Nsk */
#define   MM_FORTEMEDIA                 229         /* ForteMedia, Inc */
#define   MM_SONORUS                    230         /* Sonorus, Inc. */
#define   MM_ARRAY                      231         /* Array Microsystems, Inc. */
#define   MM_DATARAN                    232         /* Data Translation, Inc. */
#define   MM_I_LINK                     233         /* I-link Worldwide */
#define   MM_SELSIUS_SYSTEMS            234         /* Selsius Systems Inc. */
#define   MM_ADMOS                      235         /* AdMOS Technology, Inc. */
#define   MM_LEXICON                    236         /* Lexicon Inc. */
#define   MM_SGI                        237         /* Silicon Graphics Inc. */
#define   MM_IPI                        238         /* Interactive Product Inc. */
#define   MM_ICE                        239         /* IC Ensemble, Inc. */
#define   MM_VQST                       240         /* ViewQuest Technologies Inc. */
#define   MM_ETEK                       241         /* eTEK Labs Inc. */
#define   MM_CS                         242         /* Consistent Software */
#define   MM_ALESIS                     243         /* Alesis Studio Electronics */
#define   MM_INTERNET                   244         /* INTERNET Corporation */
#define   MM_SONY                       245         /* Sony Corporation */
#define   MM_HYPERACTIVE                246         /* Hyperactive Audio Systems, Inc. */
#define   MM_UHER_INFORMATIC            247         /* UHER informatic GmbH */
#define   MM_SYDEC_NV                   248         /* Sydec NV */
#define   MM_FLEXION                    249         /* Flexion Systems Ltd. */
#define   MM_VIA                        250         /* Via Technologies, Inc. */
#define   MM_MICRONAS                   251         /* Micronas Semiconductors, Inc. */
#define   MM_ANALOGDEVICES              252         /* Analog Devices, Inc. */
#define   MM_HP                         253         /* Hewlett Packard Company */
#define   MM_MATROX_DIV                 254         /* Matrox */
#define   MM_QUICKAUDIO                 255         /* Quick Audio, GbR */
#define   MM_YOUCOM                     256         /* You/Com Audiocommunicatie BV */
#define   MM_RICHMOND                   257         /* Richmond Sound Design Ltd. */
#define   MM_IODD                       258         /* I-O Data Device, Inc. */
#define   MM_ICCC                       259         /* ICCC A/S */
#define   MM_3COM                       260         /* 3COM Corporation */
#define   MM_MALDEN                     261         /* Malden Electronics Ltd. */
#define   MM_3DFX                       262         /* 3Dfx Interactive, Inc. */
#define   MM_MINDMAKER                  263         /* Mindmaker, Inc. */
#define   MM_TELEKOL                    264         /* Telekol Corp. */
#define   MM_ST_MICROELECTRONICS        265         /* ST Microelectronics */
#define   MM_ALGOVISION                 266         /* Algo Vision Systems GmbH */

#define   MM_UNMAPPED                   0xffff      /* extensible MID mapping */

#define   MM_PID_UNMAPPED               MM_UNMAPPED /* extensible PID mapping */

#ifdef GUID_DEFINED
#if !defined(INIT_MMREG_MID)
//{d5a47fa7-6d98-11d1-a21a-00a0c9223196}
#define INIT_MMREG_MID(guid, id)\
{\
    (guid)->Data1 = 0xd5a47fa7 + (USHORT)(id);\
    (guid)->Data2 = 0x6d98;\
    (guid)->Data3 = 0x11d1;\
    (guid)->Data4[0] = 0xa2;\
    (guid)->Data4[1] = 0x1a;\
    (guid)->Data4[2] = 0x00;\
    (guid)->Data4[3] = 0xa0;\
    (guid)->Data4[4] = 0xc9;\
    (guid)->Data4[5] = 0x22;\
    (guid)->Data4[6] = 0x31;\
    (guid)->Data4[7] = 0x96;\
}
#define EXTRACT_MMREG_MID(guid)\
    (USHORT)((guid)->Data1 - 0xd5a47fa7)
#define DEFINE_MMREG_MID_GUID(id)\
    0xd5a47fa7+(USHORT)(id), 0x6d98, 0x11d1, 0xa2, 0x1a, 0x00, 0xa0, 0xc9, 0x22, 0x31, 0x96

#define IS_COMPATIBLE_MMREG_MID(guid)\
    (((guid)->Data1 >= 0xd5a47fa7) &&\
    ((guid)->Data1 < 0xd5a47fa7 + 0xffff) &&\
    ((guid)->Data2 == 0x6d98) &&\
    ((guid)->Data3 == 0x11d1) &&\
    ((guid)->Data4[0] == 0xa2) &&\
    ((guid)->Data4[1] == 0x1a) &&\
    ((guid)->Data4[2] == 0x00) &&\
    ((guid)->Data4[3] == 0xa0) &&\
    ((guid)->Data4[4] == 0xc9) &&\
    ((guid)->Data4[5] == 0x22) &&\
    ((guid)->Data4[6] == 0x31) &&\
    ((guid)->Data4[7] == 0x96))
#endif // !defined(INIT_MMREG_MID)

#if !defined(INIT_MMREG_PID)
//{e36dc2ac-6d9a-11d1-a21a-00a0c9223196}
#define INIT_MMREG_PID(guid, id)\
{\
    (guid)->Data1 = 0xe36dc2ac + (USHORT)(id);\
    (guid)->Data2 = 0x6d9a;\
    (guid)->Data3 = 0x11d1;\
    (guid)->Data4[0] = 0xa2;\
    (guid)->Data4[1] = 0x1a;\
    (guid)->Data4[2] = 0x00;\
    (guid)->Data4[3] = 0xa0;\
    (guid)->Data4[4] = 0xc9;\
    (guid)->Data4[5] = 0x22;\
    (guid)->Data4[6] = 0x31;\
    (guid)->Data4[7] = 0x96;\
}
#define EXTRACT_MMREG_PID(guid)\
    (USHORT)((guid)->Data1 - 0xe36dc2ac)
#define DEFINE_MMREG_PID_GUID(id)\
    0xe36dc2ac+(USHORT)(id), 0x6d9a, 0x11d1, 0xa2, 0x1a, 0x00, 0xa0, 0xc9, 0x22, 0x31, 0x96

#define IS_COMPATIBLE_MMREG_PID(guid)\
    (((guid)->Data1 >= 0xe36dc2ac) &&\
    ((guid)->Data1 < 0xe36dc2ac + 0xffff) &&\
    ((guid)->Data2 == 0x6d9a) &&\
    ((guid)->Data3 == 0x11d1) &&\
    ((guid)->Data4[0] == 0xa2) &&\
    ((guid)->Data4[1] == 0x1a) &&\
    ((guid)->Data4[2] == 0x00) &&\
    ((guid)->Data4[3] == 0xa0) &&\
    ((guid)->Data4[4] == 0xc9) &&\
    ((guid)->Data4[5] == 0x22) &&\
    ((guid)->Data4[6] == 0x31) &&\
    ((guid)->Data4[7] == 0x96))
#endif // !defined(INIT_MMREG_PID)
#endif // GUID_DEFINED

/* MM_MICROSOFT product IDs */

#ifndef MM_MIDI_MAPPER

#define  MM_MIDI_MAPPER                     1       /*  Midi Mapper  */
#define  MM_WAVE_MAPPER                     2       /*  Wave Mapper  */
#define  MM_SNDBLST_MIDIOUT                 3       /*  Sound Blaster MIDI output port  */
#define  MM_SNDBLST_MIDIIN                  4       /*  Sound Blaster MIDI input port  */
#define  MM_SNDBLST_SYNTH                   5       /*  Sound Blaster internal synth  */
#define  MM_SNDBLST_WAVEOUT                 6       /*  Sound Blaster waveform output  */
#define  MM_SNDBLST_WAVEIN                  7       /*  Sound Blaster waveform input  */
#define  MM_ADLIB                           9       /*  Ad Lib Compatible synth  */
#define  MM_MPU401_MIDIOUT                  10      /*  MPU 401 compatible MIDI output port  */
#define  MM_MPU401_MIDIIN                   11      /*  MPU 401 compatible MIDI input port  */
#define  MM_PC_JOYSTICK                     12      /*  Joystick adapter  */

#endif

#define  MM_PCSPEAKER_WAVEOUT               13      /*  PC speaker waveform output  */
#define  MM_MSFT_WSS_WAVEIN                 14      /*  MS Audio Board waveform input  */
#define  MM_MSFT_WSS_WAVEOUT                15      /*  MS Audio Board waveform output  */
#define  MM_MSFT_WSS_FMSYNTH_STEREO         16      /*  MS Audio Board  Stereo FM synth  */
#define  MM_MSFT_WSS_MIXER                  17      /*  MS Audio Board Mixer Driver  */
#define  MM_MSFT_WSS_OEM_WAVEIN             18      /*  MS OEM Audio Board waveform input  */
#define  MM_MSFT_WSS_OEM_WAVEOUT            19      /*  MS OEM Audio Board waveform output  */
#define  MM_MSFT_WSS_OEM_FMSYNTH_STEREO     20      /*  MS OEM Audio Board Stereo FM Synth  */
#define  MM_MSFT_WSS_AUX                    21      /*  MS Audio Board Aux. Port  */
#define  MM_MSFT_WSS_OEM_AUX                22      /*  MS OEM Audio Aux Port  */
#define  MM_MSFT_GENERIC_WAVEIN             23      /*  MS Vanilla driver waveform input  */
#define  MM_MSFT_GENERIC_WAVEOUT            24      /*  MS Vanilla driver wavefrom output  */
#define  MM_MSFT_GENERIC_MIDIIN             25      /*  MS Vanilla driver MIDI in  */
#define  MM_MSFT_GENERIC_MIDIOUT            26      /*  MS Vanilla driver MIDI  external out  */
#define  MM_MSFT_GENERIC_MIDISYNTH          27      /*  MS Vanilla driver MIDI synthesizer  */
#define  MM_MSFT_GENERIC_AUX_LINE           28      /*  MS Vanilla driver aux (line in)  */
#define  MM_MSFT_GENERIC_AUX_MIC            29      /*  MS Vanilla driver aux (mic)  */
#define  MM_MSFT_GENERIC_AUX_CD             30      /*  MS Vanilla driver aux (CD)  */
#define  MM_MSFT_WSS_OEM_MIXER              31      /*  MS OEM Audio Board Mixer Driver  */
#define  MM_MSFT_MSACM                      32      /*  MS Audio Compression Manager  */
#define  MM_MSFT_ACM_MSADPCM                33      /*  MS ADPCM Codec  */
#define  MM_MSFT_ACM_IMAADPCM               34      /*  IMA ADPCM Codec  */
#define  MM_MSFT_ACM_MSFILTER               35      /*  MS Filter  */
#define  MM_MSFT_ACM_GSM610                 36      /*  GSM 610 codec  */
#define  MM_MSFT_ACM_G711                   37      /*  G.711 codec  */
#define  MM_MSFT_ACM_PCM                    38      /*  PCM converter  */

   // Microsoft Windows Sound System drivers

#define  MM_WSS_SB16_WAVEIN                 39      /*  Sound Blaster 16 waveform input  */
#define  MM_WSS_SB16_WAVEOUT                40      /*  Sound Blaster 16  waveform output  */
#define  MM_WSS_SB16_MIDIIN                 41      /*  Sound Blaster 16 midi-in  */
#define  MM_WSS_SB16_MIDIOUT                42      /*  Sound Blaster 16 midi out  */
#define  MM_WSS_SB16_SYNTH                  43      /*  Sound Blaster 16 FM Synthesis  */
#define  MM_WSS_SB16_AUX_LINE               44      /*  Sound Blaster 16 aux (line in)  */
#define  MM_WSS_SB16_AUX_CD                 45      /*  Sound Blaster 16 aux (CD)  */
#define  MM_WSS_SB16_MIXER                  46      /*  Sound Blaster 16 mixer device  */
#define  MM_WSS_SBPRO_WAVEIN                47      /*  Sound Blaster Pro waveform input  */
#define  MM_WSS_SBPRO_WAVEOUT               48      /*  Sound Blaster Pro waveform output  */
#define  MM_WSS_SBPRO_MIDIIN                49      /*  Sound Blaster Pro midi in  */
#define  MM_WSS_SBPRO_MIDIOUT               50      /*  Sound Blaster Pro midi out  */
#define  MM_WSS_SBPRO_SYNTH                 51      /*  Sound Blaster Pro FM synthesis  */
#define  MM_WSS_SBPRO_AUX_LINE              52      /*  Sound Blaster Pro aux (line in )  */
#define  MM_WSS_SBPRO_AUX_CD                53      /*  Sound Blaster Pro aux (CD)  */
#define  MM_WSS_SBPRO_MIXER                 54      /*  Sound Blaster Pro mixer  */
#define  MM_MSFT_WSS_NT_WAVEIN              55      /*  WSS NT wave in  */
#define  MM_MSFT_WSS_NT_WAVEOUT             56      /*  WSS NT wave out  */
#define  MM_MSFT_WSS_NT_FMSYNTH_STEREO      57      /*  WSS NT FM synth  */
#define  MM_MSFT_WSS_NT_MIXER               58      /*  WSS NT mixer  */
#define  MM_MSFT_WSS_NT_AUX                 59      /*  WSS NT aux  */
#define  MM_MSFT_SB16_WAVEIN                60      /*  Sound Blaster 16 waveform input  */
#define  MM_MSFT_SB16_WAVEOUT               61      /*  Sound Blaster 16  waveform output  */
#define  MM_MSFT_SB16_MIDIIN                62      /*  Sound Blaster 16 midi-in  */
#define  MM_MSFT_SB16_MIDIOUT               63      /*  Sound Blaster 16 midi out  */
#define  MM_MSFT_SB16_SYNTH                 64      /*  Sound Blaster 16 FM Synthesis  */
#define  MM_MSFT_SB16_AUX_LINE              65      /*  Sound Blaster 16 aux (line in)  */
#define  MM_MSFT_SB16_AUX_CD                66      /*  Sound Blaster 16 aux (CD)  */
#define  MM_MSFT_SB16_MIXER                 67      /*  Sound Blaster 16 mixer device  */
#define  MM_MSFT_SBPRO_WAVEIN               68      /*  Sound Blaster Pro waveform input  */
#define  MM_MSFT_SBPRO_WAVEOUT              69      /*  Sound Blaster Pro waveform output  */
#define  MM_MSFT_SBPRO_MIDIIN               70      /*  Sound Blaster Pro midi in  */
#define  MM_MSFT_SBPRO_MIDIOUT              71      /*  Sound Blaster Pro midi out  */
#define  MM_MSFT_SBPRO_SYNTH                72      /*  Sound Blaster Pro FM synthesis  */
#define  MM_MSFT_SBPRO_AUX_LINE             73      /*  Sound Blaster Pro aux (line in)  */
#define  MM_MSFT_SBPRO_AUX_CD               74      /*  Sound Blaster Pro aux (CD)  */
#define  MM_MSFT_SBPRO_MIXER                75      /*  Sound Blaster Pro mixer  */

#define  MM_MSFT_MSOPL_SYNTH                76      /*  Yamaha OPL2/OPL3 compatible FM synthesis */

#define  MM_MSFT_VMDMS_LINE_WAVEIN          80     /* Voice Modem Serial Line Wave Input */
#define  MM_MSFT_VMDMS_LINE_WAVEOUT         81     /* Voice Modem Serial Line Wave Output */
#define  MM_MSFT_VMDMS_HANDSET_WAVEIN       82     /* Voice Modem Serial Handset Wave Input */
#define  MM_MSFT_VMDMS_HANDSET_WAVEOUT      83     /* Voice Modem Serial Handset Wave Output */
#define  MM_MSFT_VMDMW_LINE_WAVEIN          84     /* Voice Modem Wrapper Line Wave Input */
#define  MM_MSFT_VMDMW_LINE_WAVEOUT         85     /* Voice Modem Wrapper Line Wave Output */
#define  MM_MSFT_VMDMW_HANDSET_WAVEIN       86     /* Voice Modem Wrapper Handset Wave Input */
#define  MM_MSFT_VMDMW_HANDSET_WAVEOUT      87     /* Voice Modem Wrapper Handset Wave Output */
#define  MM_MSFT_VMDMW_MIXER                88     /* Voice Modem Wrapper Mixer */
#define  MM_MSFT_VMDM_GAME_WAVEOUT          89     /* Voice Modem Game Compatible Wave Device */
#define  MM_MSFT_VMDM_GAME_WAVEIN           90     /* Voice Modem Game Compatible Wave Device */

#define  MM_MSFT_ACM_MSNAUDIO               91
#define  MM_MSFT_ACM_MSG723                 92
#define  MM_MSFT_ACM_MSRT24                 93

#define  MM_MSFT_WDMAUDIO_WAVEOUT           100    /* Generic id for WDM Audio drivers */
#define  MM_MSFT_WDMAUDIO_WAVEIN            101    /* Generic id for WDM Audio drivers */
#define  MM_MSFT_WDMAUDIO_MIDIOUT           102    /* Generic id for WDM Audio drivers */
#define  MM_MSFT_WDMAUDIO_MIDIIN            103    /* Generic id for WDM Audio drivers */
#define  MM_MSFT_WDMAUDIO_MIXER             104    /* Generic id for WDM Audio drivers */
#define  MM_MSFT_WDMAUDIO_AUX               105    /* Generic id for WDM Audio drivers */


/* MM_CREATIVE product IDs */
#define  MM_CREATIVE_SB15_WAVEIN            1       /*  SB (r) 1.5 waveform input  */
#define  MM_CREATIVE_SB20_WAVEIN            2
#define  MM_CREATIVE_SBPRO_WAVEIN           3
#define  MM_CREATIVE_SBP16_WAVEIN           4
#define  MM_CREATIVE_PHNBLST_WAVEIN         5
#define  MM_CREATIVE_SB15_WAVEOUT           101
#define  MM_CREATIVE_SB20_WAVEOUT           102
#define  MM_CREATIVE_SBPRO_WAVEOUT          103
#define  MM_CREATIVE_SBP16_WAVEOUT          104
#define  MM_CREATIVE_PHNBLST_WAVEOUT        105
#define  MM_CREATIVE_MIDIOUT                201     /*  SB (r)  */
#define  MM_CREATIVE_MIDIIN                 202     /*  SB (r)  */
#define  MM_CREATIVE_FMSYNTH_MONO           301     /*  SB (r)  */
#define  MM_CREATIVE_FMSYNTH_STEREO         302     /*  SB Pro (r) stereo synthesizer  */
#define  MM_CREATIVE_MIDI_AWE32             303
#define  MM_CREATIVE_AUX_CD                 401     /*  SB Pro (r) aux (CD)  */
#define  MM_CREATIVE_AUX_LINE               402     /*  SB Pro (r) aux (Line in )  */
#define  MM_CREATIVE_AUX_MIC                403     /*  SB Pro (r) aux (mic)  */
#define  MM_CREATIVE_AUX_MASTER             404
#define  MM_CREATIVE_AUX_PCSPK              405
#define  MM_CREATIVE_AUX_WAVE               406
#define  MM_CREATIVE_AUX_MIDI               407
#define  MM_CREATIVE_SBPRO_MIXER            408
#define  MM_CREATIVE_SB16_MIXER             409

/* MM_MEDIAVISION product IDs */

// Pro Audio Spectrum
#define  MM_MEDIAVISION_PROAUDIO            0x10
#define  MM_PROAUD_MIDIOUT                  (MM_MEDIAVISION_PROAUDIO+1)
#define  MM_PROAUD_MIDIIN                   (MM_MEDIAVISION_PROAUDIO+2)
#define  MM_PROAUD_SYNTH                    (MM_MEDIAVISION_PROAUDIO+3)
#define  MM_PROAUD_WAVEOUT                  (MM_MEDIAVISION_PROAUDIO+4)
#define  MM_PROAUD_WAVEIN                   (MM_MEDIAVISION_PROAUDIO+5)
#define  MM_PROAUD_MIXER                    (MM_MEDIAVISION_PROAUDIO+6)
#define  MM_PROAUD_AUX                      (MM_MEDIAVISION_PROAUDIO+7)

// Thunder Board
#define  MM_MEDIAVISION_THUNDER             0x20
#define  MM_THUNDER_SYNTH                   (MM_MEDIAVISION_THUNDER+3)
#define  MM_THUNDER_WAVEOUT                 (MM_MEDIAVISION_THUNDER+4)
#define  MM_THUNDER_WAVEIN                  (MM_MEDIAVISION_THUNDER+5)
#define  MM_THUNDER_AUX                     (MM_MEDIAVISION_THUNDER+7)

// Audio Port
#define  MM_MEDIAVISION_TPORT               0x40
#define  MM_TPORT_WAVEOUT                   (MM_MEDIAVISION_TPORT+1)
#define  MM_TPORT_WAVEIN                    (MM_MEDIAVISION_TPORT+2)
#define  MM_TPORT_SYNTH                     (MM_MEDIAVISION_TPORT+3)

// Pro Audio Spectrum Plus
#define  MM_MEDIAVISION_PROAUDIO_PLUS       0x50
#define  MM_PROAUD_PLUS_MIDIOUT             (MM_MEDIAVISION_PROAUDIO_PLUS+1)
#define  MM_PROAUD_PLUS_MIDIIN              (MM_MEDIAVISION_PROAUDIO_PLUS+2)
#define  MM_PROAUD_PLUS_SYNTH               (MM_MEDIAVISION_PROAUDIO_PLUS+3)
#define  MM_PROAUD_PLUS_WAVEOUT             (MM_MEDIAVISION_PROAUDIO_PLUS+4)
#define  MM_PROAUD_PLUS_WAVEIN              (MM_MEDIAVISION_PROAUDIO_PLUS+5)
#define  MM_PROAUD_PLUS_MIXER               (MM_MEDIAVISION_PROAUDIO_PLUS+6)
#define  MM_PROAUD_PLUS_AUX                 (MM_MEDIAVISION_PROAUDIO_PLUS+7)

// Pro Audio Spectrum 16
#define  MM_MEDIAVISION_PROAUDIO_16         0x60
#define  MM_PROAUD_16_MIDIOUT               (MM_MEDIAVISION_PROAUDIO_16+1)
#define  MM_PROAUD_16_MIDIIN                (MM_MEDIAVISION_PROAUDIO_16+2)
#define  MM_PROAUD_16_SYNTH                 (MM_MEDIAVISION_PROAUDIO_16+3)
#define  MM_PROAUD_16_WAVEOUT               (MM_MEDIAVISION_PROAUDIO_16+4)
#define  MM_PROAUD_16_WAVEIN                (MM_MEDIAVISION_PROAUDIO_16+5)
#define  MM_PROAUD_16_MIXER                 (MM_MEDIAVISION_PROAUDIO_16+6)
#define  MM_PROAUD_16_AUX                   (MM_MEDIAVISION_PROAUDIO_16+7)

// Pro Audio Studio 16
#define  MM_MEDIAVISION_PROSTUDIO_16        0x60
#define  MM_STUDIO_16_MIDIOUT               (MM_MEDIAVISION_PROSTUDIO_16+1)
#define  MM_STUDIO_16_MIDIIN                (MM_MEDIAVISION_PROSTUDIO_16+2)
#define  MM_STUDIO_16_SYNTH                 (MM_MEDIAVISION_PROSTUDIO_16+3)
#define  MM_STUDIO_16_WAVEOUT               (MM_MEDIAVISION_PROSTUDIO_16+4)
#define  MM_STUDIO_16_WAVEIN                (MM_MEDIAVISION_PROSTUDIO_16+5)
#define  MM_STUDIO_16_MIXER                 (MM_MEDIAVISION_PROSTUDIO_16+6)
#define  MM_STUDIO_16_AUX                   (MM_MEDIAVISION_PROSTUDIO_16+7)

// CDPC
#define  MM_MEDIAVISION_CDPC                0x70
#define  MM_CDPC_MIDIOUT                    (MM_MEDIAVISION_CDPC+1)
#define  MM_CDPC_MIDIIN                     (MM_MEDIAVISION_CDPC+2)
#define  MM_CDPC_SYNTH                      (MM_MEDIAVISION_CDPC+3)
#define  MM_CDPC_WAVEOUT                    (MM_MEDIAVISION_CDPC+4)
#define  MM_CDPC_WAVEIN                     (MM_MEDIAVISION_CDPC+5)
#define  MM_CDPC_MIXER                      (MM_MEDIAVISION_CDPC+6)
#define  MM_CDPC_AUX                        (MM_MEDIAVISION_CDPC+7)

// Opus MV 1208 Chipsent
#define  MM_MEDIAVISION_OPUS1208            0x80
#define  MM_OPUS401_MIDIOUT                 (MM_MEDIAVISION_OPUS1208+1)
#define  MM_OPUS401_MIDIIN                  (MM_MEDIAVISION_OPUS1208+2)
#define  MM_OPUS1208_SYNTH                  (MM_MEDIAVISION_OPUS1208+3)
#define  MM_OPUS1208_WAVEOUT                (MM_MEDIAVISION_OPUS1208+4)
#define  MM_OPUS1208_WAVEIN                 (MM_MEDIAVISION_OPUS1208+5)
#define  MM_OPUS1208_MIXER                  (MM_MEDIAVISION_OPUS1208+6)
#define  MM_OPUS1208_AUX                    (MM_MEDIAVISION_OPUS1208+7)

// Opus MV 1216 chipset
#define  MM_MEDIAVISION_OPUS1216            0x90
#define  MM_OPUS1216_MIDIOUT                (MM_MEDIAVISION_OPUS1216+1)
#define  MM_OPUS1216_MIDIIN                 (MM_MEDIAVISION_OPUS1216+2)
#define  MM_OPUS1216_SYNTH                  (MM_MEDIAVISION_OPUS1216+3)
#define  MM_OPUS1216_WAVEOUT                (MM_MEDIAVISION_OPUS1216+4)
#define  MM_OPUS1216_WAVEIN                 (MM_MEDIAVISION_OPUS1216+5)
#define  MM_OPUS1216_MIXER                  (MM_MEDIAVISION_OPUS1216+6)
#define  MM_OPUS1216_AUX                    (MM_MEDIAVISION_OPUS1216+7)

/* MM_CYRIX product IDs */
#define  MM_CYRIX_XASYNTH                   1
#define  MM_CYRIX_XAMIDIIN                  2
#define  MM_CYRIX_XAMIDIOUT                 3
#define  MM_CYRIX_XAWAVEIN                  4
#define  MM_CYRIX_XAWAVEOUT                 5
#define  MM_CYRIX_XAAUX                     6
#define  MM_CYRIX_XAMIXER                   7

/* MM_PHILIPS_SPEECH_PROCESSING products IDs */
#define  MM_PHILIPS_ACM_LPCBB               1

/* MM_NETXL product IDs */
#define  MM_NETXL_XLVIDEO                   1

/* MM_ZYXEL product IDs */
#define  MM_ZYXEL_ACM_ADPCM                 1

/* MM_AARDVARK product IDs */
#define  MM_AARDVARK_STUDIO12_WAVEOUT       1
#define  MM_AARDVARK_STUDIO12_WAVEIN        2
#define  MM_AARDVARK_STUDIO88_WAVEOUT       3
#define  MM_AARDVARK_STUDIO88_WAVEIN        4

/* MM_BINTEC product IDs */
#define  MM_BINTEC_TAPI_WAVE                1

/* MM_HEWLETT_PACKARD product IDs */
#define  MM_HEWLETT_PACKARD_CU_CODEC        1

/* MM_MITEL product IDs */
#define  MM_MITEL_TALKTO_LINE_WAVEOUT       100
#define  MM_MITEL_TALKTO_LINE_WAVEIN        101
#define  MM_MITEL_TALKTO_HANDSET_WAVEOUT    102
#define  MM_MITEL_TALKTO_HANDSET_WAVEIN     103
#define  MM_MITEL_TALKTO_BRIDGED_WAVEOUT    104
#define  MM_MITEL_TALKTO_BRIDGED_WAVEIN     105
#define  MM_MITEL_MPA_HANDSET_WAVEOUT       200
#define  MM_MITEL_MPA_HANDSET_WAVEIN        201
#define  MM_MITEL_MPA_HANDSFREE_WAVEOUT     202
#define  MM_MITEL_MPA_HANDSFREE_WAVEIN      203
#define  MM_MITEL_MPA_LINE1_WAVEOUT         204
#define  MM_MITEL_MPA_LINE1_WAVEIN          205
#define  MM_MITEL_MPA_LINE2_WAVEOUT         206
#define  MM_MITEL_MPA_LINE2_WAVEIN          207
#define  MM_MITEL_MEDIAPATH_WAVEOUT         300
#define  MM_MITEL_MEDIAPATH_WAVEIN          301

/*  MM_SNI product IDs */
#define  MM_SNI_ACM_G721                    1

/* MM_EMU product IDs */
#define  MM_EMU_APSSYNTH                    1
#define  MM_EMU_APSMIDIIN                   2
#define  MM_EMU_APSMIDIOUT                  3
#define  MM_EMU_APSWAVEIN                   4
#define  MM_EMU_APSWAVEOUT                  5

/* MM_ARTISOFT product IDs */
#define  MM_ARTISOFT_SBWAVEIN               1       /*  Artisoft sounding Board waveform input  */
#define  MM_ARTISOFT_SBWAVEOUT              2       /*  Artisoft sounding Board waveform output  */

/* MM_TURTLE_BEACH product IDs */
#define  MM_TBS_TROPEZ_WAVEIN               37
#define  MM_TBS_TROPEZ_WAVEOUT              38
#define  MM_TBS_TROPEZ_AUX1                 39
#define  MM_TBS_TROPEZ_AUX2                 40
#define  MM_TBS_TROPEZ_LINE                 41

/* MM_IBM product IDs */
#define  MM_MMOTION_WAVEAUX                 1       /*  IBM M-Motion Auxiliary Device  */
#define  MM_MMOTION_WAVEOUT                 2       /*  IBM M-Motion Waveform output  */
#define  MM_MMOTION_WAVEIN                  3       /*  IBM M-Motion  Waveform Input  */
#define  MM_IBM_PCMCIA_WAVEIN               11      /*  IBM waveform input  */
#define  MM_IBM_PCMCIA_WAVEOUT              12      /*  IBM Waveform output  */
#define  MM_IBM_PCMCIA_SYNTH                13      /*  IBM Midi Synthesis  */
#define  MM_IBM_PCMCIA_MIDIIN               14      /*  IBM external MIDI in  */
#define  MM_IBM_PCMCIA_MIDIOUT              15      /*  IBM external MIDI out  */
#define  MM_IBM_PCMCIA_AUX                  16      /*  IBM auxiliary control  */
#define  MM_IBM_THINKPAD200                 17
#define  MM_IBM_MWAVE_WAVEIN                18
#define  MM_IBM_MWAVE_WAVEOUT               19
#define  MM_IBM_MWAVE_MIXER                 20
#define  MM_IBM_MWAVE_MIDIIN                21
#define  MM_IBM_MWAVE_MIDIOUT               22
#define  MM_IBM_MWAVE_AUX                   23
#define  MM_IBM_WC_MIDIOUT                  30
#define  MM_IBM_WC_WAVEOUT                  31
#define  MM_IBM_WC_MIXEROUT                 33

/* MM_VOCALTEC product IDs */
#define  MM_VOCALTEC_WAVEOUT                1
#define  MM_VOCALTEC_WAVEIN                 2

/* MM_ROLAND product IDs */
#define  MM_ROLAND_RAP10_MIDIOUT            10      /* MM_ROLAND_RAP10 */
#define  MM_ROLAND_RAP10_MIDIIN             11      /* MM_ROLAND_RAP10 */
#define  MM_ROLAND_RAP10_SYNTH              12      /* MM_ROLAND_RAP10 */
#define  MM_ROLAND_RAP10_WAVEOUT            13      /* MM_ROLAND_RAP10 */
#define  MM_ROLAND_RAP10_WAVEIN             14      /* MM_ROLAND_RAP10 */
#define  MM_ROLAND_MPU401_MIDIOUT           15
#define  MM_ROLAND_MPU401_MIDIIN            16
#define  MM_ROLAND_SMPU_MIDIOUTA            17
#define  MM_ROLAND_SMPU_MIDIOUTB            18
#define  MM_ROLAND_SMPU_MIDIINA             19
#define  MM_ROLAND_SMPU_MIDIINB             20
#define  MM_ROLAND_SC7_MIDIOUT              21
#define  MM_ROLAND_SC7_MIDIIN               22
#define  MM_ROLAND_SERIAL_MIDIOUT           23
#define  MM_ROLAND_SERIAL_MIDIIN            24
#define  MM_ROLAND_SCP_MIDIOUT              38
#define  MM_ROLAND_SCP_MIDIIN               39
#define  MM_ROLAND_SCP_WAVEOUT              40
#define  MM_ROLAND_SCP_WAVEIN               41
#define  MM_ROLAND_SCP_MIXER                42
#define  MM_ROLAND_SCP_AUX                  48

/* MM_DSP_SOLUTIONS product IDs */
#define  MM_DSP_SOLUTIONS_WAVEOUT           1
#define  MM_DSP_SOLUTIONS_WAVEIN            2
#define  MM_DSP_SOLUTIONS_SYNTH             3
#define  MM_DSP_SOLUTIONS_AUX               4

/* MM_NEC product IDs */
#define  MM_NEC_73_86_SYNTH                 5
#define  MM_NEC_73_86_WAVEOUT               6
#define  MM_NEC_73_86_WAVEIN                7
#define  MM_NEC_26_SYNTH                    9
#define  MM_NEC_MPU401_MIDIOUT              10
#define  MM_NEC_MPU401_MIDIIN               11
#define  MM_NEC_JOYSTICK                    12

/* MM_WANGLABS product IDs */
#define  MM_WANGLABS_WAVEIN1                1       /*  Input audio wave on CPU board models: Exec 4010, 4030, 3450; PC 251/25c, pc 461/25s , pc 461/33c  */
#define  MM_WANGLABS_WAVEOUT1               2

/* MM_TANDY product IDs */
#define  MM_TANDY_VISWAVEIN                 1
#define  MM_TANDY_VISWAVEOUT                2
#define  MM_TANDY_VISBIOSSYNTH              3
#define  MM_TANDY_SENS_MMAWAVEIN            4
#define  MM_TANDY_SENS_MMAWAVEOUT           5
#define  MM_TANDY_SENS_MMAMIDIIN            6
#define  MM_TANDY_SENS_MMAMIDIOUT           7
#define  MM_TANDY_SENS_VISWAVEOUT           8
#define  MM_TANDY_PSSJWAVEIN                9
#define  MM_TANDY_PSSJWAVEOUT               10

/* MM_ANTEX product IDs */
#define  MM_ANTEX_SX12_WAVEIN               1
#define  MM_ANTEX_SX12_WAVEOUT              2
#define  MM_ANTEX_SX15_WAVEIN               3
#define  MM_ANTEX_SX15_WAVEOUT              4
#define  MM_ANTEX_VP625_WAVEIN              5
#define  MM_ANTEX_VP625_WAVEOUT             6
#define  MM_ANTEX_AUDIOPORT22_WAVEIN        7
#define  MM_ANTEX_AUDIOPORT22_WAVEOUT       8
#define  MM_ANTEX_AUDIOPORT22_FEEDTHRU      9

/* MM_INTEL product IDs */
#define  MM_INTELOPD_WAVEIN                 1       /*  HID2 WaveAudio Driver  */
#define  MM_INTELOPD_WAVEOUT                101     /*  HID2  */
#define  MM_INTELOPD_AUX                    401     /*  HID2 for mixing  */
#define  MM_INTEL_NSPMODEMLINEIN            501
#define  MM_INTEL_NSPMODEMLINEOUT           502

/* MM_VAL product IDs */
#define  MM_VAL_MICROKEY_AP_WAVEIN          1
#define  MM_VAL_MICROKEY_AP_WAVEOUT         2

/* MM_INTERACTIVE product IDs */
#define  MM_INTERACTIVE_WAVEIN              0x45
#define  MM_INTERACTIVE_WAVEOUT             0x45

/* MM_YAMAHA product IDs */
#define  MM_YAMAHA_GSS_SYNTH                0x01
#define  MM_YAMAHA_GSS_WAVEOUT              0x02
#define  MM_YAMAHA_GSS_WAVEIN               0x03
#define  MM_YAMAHA_GSS_MIDIOUT              0x04
#define  MM_YAMAHA_GSS_MIDIIN               0x05
#define  MM_YAMAHA_GSS_AUX                  0x06
#define  MM_YAMAHA_SERIAL_MIDIOUT           0x07
#define  MM_YAMAHA_SERIAL_MIDIIN            0x08
#define  MM_YAMAHA_OPL3SA_WAVEOUT           0x10
#define  MM_YAMAHA_OPL3SA_WAVEIN            0x11
#define  MM_YAMAHA_OPL3SA_FMSYNTH           0x12
#define  MM_YAMAHA_OPL3SA_YSYNTH            0x13
#define  MM_YAMAHA_OPL3SA_MIDIOUT           0x14
#define  MM_YAMAHA_OPL3SA_MIDIIN            0x15
#define  MM_YAMAHA_OPL3SA_MIXER             0x17
#define  MM_YAMAHA_OPL3SA_JOYSTICK          0x18
#define  MM_YAMAHA_YMF724LEG_MIDIOUT        0x19
#define  MM_YAMAHA_YMF724LEG_MIDIIN         0x1a
#define  MM_YAMAHA_YMF724_WAVEOUT           0x1b
#define  MM_YAMAHA_YMF724_WAVEIN            0x1c
#define  MM_YAMAHA_YMF724_MIDIOUT           0x1d
#define  MM_YAMAHA_YMF724_AUX               0x1e
#define  MM_YAMAHA_YMF724_MIXER             0x1f
#define  MM_YAMAHA_YMF724LEG_FMSYNTH        0x20
#define  MM_YAMAHA_YMF724LEG_MIXER          0x21
#define  MM_YAMAHA_SXG_MIDIOUT              0x22
#define  MM_YAMAHA_SXG_WAVEOUT              0x23
#define  MM_YAMAHA_SXG_MIXER                0x24
#define  MM_YAMAHA_ACXG_WAVEIN              0x25
#define  MM_YAMAHA_ACXG_WAVEOUT             0x26
#define  MM_YAMAHA_ACXG_MIDIOUT             0x27
#define  MM_YAMAHA_ACXG_MIXER               0x28
#define  MM_YAMAHA_ACXG_AUX                 0x29

/* MM_EVEREX product IDs */
#define  MM_EVEREX_CARRIER                  1

/* MM_ECHO product IDs */
#define  MM_ECHO_SYNTH                      1
#define  MM_ECHO_WAVEOUT                    2
#define  MM_ECHO_WAVEIN                     3
#define  MM_ECHO_MIDIOUT                    4
#define  MM_ECHO_MIDIIN                     5
#define  MM_ECHO_AUX                        6

/* MM_SIERRA product IDs */
#define  MM_SIERRA_ARIA_MIDIOUT             0x14
#define  MM_SIERRA_ARIA_MIDIIN              0x15
#define  MM_SIERRA_ARIA_SYNTH               0x16
#define  MM_SIERRA_ARIA_WAVEOUT             0x17
#define  MM_SIERRA_ARIA_WAVEIN              0x18
#define  MM_SIERRA_ARIA_AUX                 0x19
#define  MM_SIERRA_ARIA_AUX2                0x20
#define  MM_SIERRA_QUARTET_WAVEIN           0x50
#define  MM_SIERRA_QUARTET_WAVEOUT          0x51
#define  MM_SIERRA_QUARTET_MIDIIN           0x52
#define  MM_SIERRA_QUARTET_MIDIOUT          0x53
#define  MM_SIERRA_QUARTET_SYNTH            0x54
#define  MM_SIERRA_QUARTET_AUX_CD           0x55
#define  MM_SIERRA_QUARTET_AUX_LINE         0x56
#define  MM_SIERRA_QUARTET_AUX_MODEM        0x57
#define  MM_SIERRA_QUARTET_MIXER            0x58

/* MM_CAT product IDs */
#define  MM_CAT_WAVEOUT                     1

/* MM_DSP_GROUP product IDs */
#define  MM_DSP_GROUP_TRUESPEECH            1

/* MM_MELABS product IDs */
#define  MM_MELABS_MIDI2GO                  1

/* MM_ESS product IDs */
#define  MM_ESS_AMWAVEOUT                   0x01
#define  MM_ESS_AMWAVEIN                    0x02
#define  MM_ESS_AMAUX                       0x03
#define  MM_ESS_AMSYNTH                     0x04
#define  MM_ESS_AMMIDIOUT                   0x05
#define  MM_ESS_AMMIDIIN                    0x06
#define  MM_ESS_MIXER                       0x07
#define  MM_ESS_AUX_CD                      0x08
#define  MM_ESS_MPU401_MIDIOUT              0x09
#define  MM_ESS_MPU401_MIDIIN               0x0A
#define  MM_ESS_ES488_WAVEOUT               0x10
#define  MM_ESS_ES488_WAVEIN                0x11
#define  MM_ESS_ES488_MIXER                 0x12
#define  MM_ESS_ES688_WAVEOUT               0x13
#define  MM_ESS_ES688_WAVEIN                0x14
#define  MM_ESS_ES688_MIXER                 0x15
#define  MM_ESS_ES1488_WAVEOUT              0x16
#define  MM_ESS_ES1488_WAVEIN               0x17
#define  MM_ESS_ES1488_MIXER                0x18
#define  MM_ESS_ES1688_WAVEOUT              0x19
#define  MM_ESS_ES1688_WAVEIN               0x1A
#define  MM_ESS_ES1688_MIXER                0x1B
#define  MM_ESS_ES1788_WAVEOUT              0x1C
#define  MM_ESS_ES1788_WAVEIN               0x1D
#define  MM_ESS_ES1788_MIXER                0x1E
#define  MM_ESS_ES1888_WAVEOUT              0x1F
#define  MM_ESS_ES1888_WAVEIN               0x20
#define  MM_ESS_ES1888_MIXER                0x21
#define  MM_ESS_ES1868_WAVEOUT              0x22
#define  MM_ESS_ES1868_WAVEIN               0x23
#define  MM_ESS_ES1868_MIXER                0x24
#define  MM_ESS_ES1878_WAVEOUT              0x25
#define  MM_ESS_ES1878_WAVEIN               0x26
#define  MM_ESS_ES1878_MIXER                0x27

/* MM_CANOPUS product IDs */
#define  MM_CANOPUS_ACM_DVREX               1

/* MM_EPSON product IDs */
#define  MM_EPS_FMSND                       1

/* MM_TRUEVISION product IDs */
#define  MM_TRUEVISION_WAVEIN1              1
#define  MM_TRUEVISION_WAVEOUT1             2

/* MM_AZTECH product IDs */
#define  MM_AZTECH_MIDIOUT                  3
#define  MM_AZTECH_MIDIIN                   4
#define  MM_AZTECH_WAVEIN                   17
#define  MM_AZTECH_WAVEOUT                  18
#define  MM_AZTECH_FMSYNTH                  20
#define  MM_AZTECH_MIXER                    21
#define  MM_AZTECH_PRO16_WAVEIN             33
#define  MM_AZTECH_PRO16_WAVEOUT            34
#define  MM_AZTECH_PRO16_FMSYNTH            38
#define  MM_AZTECH_DSP16_WAVEIN             65
#define  MM_AZTECH_DSP16_WAVEOUT            66
#define  MM_AZTECH_DSP16_FMSYNTH            68
#define  MM_AZTECH_DSP16_WAVESYNTH          70
#define  MM_AZTECH_NOVA16_WAVEIN            71
#define  MM_AZTECH_NOVA16_WAVEOUT           72
#define  MM_AZTECH_NOVA16_MIXER             73
#define  MM_AZTECH_WASH16_WAVEIN            74
#define  MM_AZTECH_WASH16_WAVEOUT           75
#define  MM_AZTECH_WASH16_MIXER             76
#define  MM_AZTECH_AUX_CD                   401
#define  MM_AZTECH_AUX_LINE                 402
#define  MM_AZTECH_AUX_MIC                  403
#define  MM_AZTECH_AUX                      404

/* MM_VIDEOLOGIC product IDs */
#define  MM_VIDEOLOGIC_MSWAVEIN             1
#define  MM_VIDEOLOGIC_MSWAVEOUT            2

/* MM_KORG product IDs */
#define  MM_KORG_PCIF_MIDIOUT               1
#define  MM_KORG_PCIF_MIDIIN                2
#define  MM_KORG_1212IO_MSWAVEIN            3
#define  MM_KORG_1212IO_MSWAVEOUT           4

/* MM_APT product IDs */
#define  MM_APT_ACE100CD                    1

/* MM_ICS product IDs */
#define  MM_ICS_WAVEDECK_WAVEOUT            1       /*  MS WSS compatible card and driver  */
#define  MM_ICS_WAVEDECK_WAVEIN             2
#define  MM_ICS_WAVEDECK_MIXER              3
#define  MM_ICS_WAVEDECK_AUX                4
#define  MM_ICS_WAVEDECK_SYNTH              5
#define  MM_ICS_WAVEDEC_SB_WAVEOUT          6
#define  MM_ICS_WAVEDEC_SB_WAVEIN           7
#define  MM_ICS_WAVEDEC_SB_FM_MIDIOUT       8
#define  MM_ICS_WAVEDEC_SB_MPU401_MIDIOUT   9
#define  MM_ICS_WAVEDEC_SB_MPU401_MIDIIN    10
#define  MM_ICS_WAVEDEC_SB_MIXER            11
#define  MM_ICS_WAVEDEC_SB_AUX              12
#define  MM_ICS_2115_LITE_MIDIOUT           13
#define  MM_ICS_2120_LITE_MIDIOUT           14

/* MM_ITERATEDSYS product IDs */
#define  MM_ITERATEDSYS_FUFCODEC            1

/* MM_METHEUS product IDs */
#define  MM_METHEUS_ZIPPER                  1

/* MM_WINNOV product IDs */
#define  MM_WINNOV_CAVIAR_WAVEIN            1
#define  MM_WINNOV_CAVIAR_WAVEOUT           2
#define  MM_WINNOV_CAVIAR_VIDC              3
#define  MM_WINNOV_CAVIAR_CHAMPAGNE         4       /*  Fourcc is CHAM  */
#define  MM_WINNOV_CAVIAR_YUV8              5       /*  Fourcc is YUV8  */

/* MM_NCR product IDs */
#define  MM_NCR_BA_WAVEIN                   1
#define  MM_NCR_BA_WAVEOUT                  2
#define  MM_NCR_BA_SYNTH                    3
#define  MM_NCR_BA_AUX                      4
#define  MM_NCR_BA_MIXER                    5

/* MM_AST product IDs */
#define  MM_AST_MODEMWAVE_WAVEIN            13
#define  MM_AST_MODEMWAVE_WAVEOUT           14

/* MM_WILLOWPOND product IDs */
#define  MM_WILLOWPOND_FMSYNTH_STEREO       20
#define  MM_WILLOWPOND_MPU401               21
#define  MM_WILLOWPOND_SNDPORT_WAVEIN       100
#define  MM_WILLOWPOND_SNDPORT_WAVEOUT      101
#define  MM_WILLOWPOND_SNDPORT_MIXER        102
#define  MM_WILLOWPOND_SNDPORT_AUX          103
#define  MM_WILLOWPOND_PH_WAVEIN            104
#define  MM_WILLOWPOND_PH_WAVEOUT           105
#define  MM_WILLOWPOND_PH_MIXER             106
#define  MM_WILLOWPOND_PH_AUX               107
#define  MM_WILLOPOND_SNDCOMM_WAVEIN        108
#define  MM_WILLOWPOND_SNDCOMM_WAVEOUT      109
#define  MM_WILLOWPOND_SNDCOMM_MIXER        110
#define  MM_WILLOWPOND_SNDCOMM_AUX          111
#define  MM_WILLOWPOND_GENERIC_WAVEIN       112
#define  MM_WILLOWPOND_GENERIC_WAVEOUT      113
#define  MM_WILLOWPOND_GENERIC_MIXER        114
#define  MM_WILLOWPOND_GENERIC_AUX          115

/* MM_VITEC product IDs */
#define  MM_VITEC_VMAKER                    1
#define  MM_VITEC_VMPRO                     2

/* MM_MOSCOM product IDs */
#define  MM_MOSCOM_VPC2400_IN               1       /*  Four Port Voice Processing / Voice Recognition Board  */
#define  MM_MOSCOM_VPC2400_OUT              2       /*  VPC2400 */

/* MM_SILICONSOFT product IDs */
#define  MM_SILICONSOFT_SC1_WAVEIN          1       /*  Waveform in , high sample rate  */
#define  MM_SILICONSOFT_SC1_WAVEOUT         2       /*  Waveform out , high sample rate  */
#define  MM_SILICONSOFT_SC2_WAVEIN          3       /*  Waveform in 2 channels, high sample rate  */
#define  MM_SILICONSOFT_SC2_WAVEOUT         4       /*  Waveform out 2 channels, high sample rate  */
#define  MM_SILICONSOFT_SOUNDJR2_WAVEOUT    5       /*  Waveform out, self powered, efficient  */
#define  MM_SILICONSOFT_SOUNDJR2PR_WAVEIN   6       /*  Waveform in, self powered, efficient  */
#define  MM_SILICONSOFT_SOUNDJR2PR_WAVEOUT  7       /*  Waveform out 2 channels, self powered, efficient  */
#define  MM_SILICONSOFT_SOUNDJR3_WAVEOUT    8       /*  Waveform in 2 channels, self powered, efficient  */

/* MM_TERRATEC product IDs */
#define  MM_TTEWS_WAVEIN                    1
#define  MM_TTEWS_WAVEOUT                   2
#define  MM_TTEWS_MIDIIN                    3
#define  MM_TTEWS_MIDIOUT                   4
#define  MM_TTEWS_MIDISYNTH                 5
#define  MM_TTEWS_MIDIMONITOR               6
#define  MM_TTEWS_VMIDIIN                   7
#define  MM_TTEWS_VMIDIOUT                  8
#define  MM_TTEWS_AUX                       9
#define  MM_TTEWS_MIXER                     10

/* MM_MEDIASONIC product IDs */
#define  MM_MEDIASONIC_ACM_G723             1
#define  MM_MEDIASONIC_ICOM                 2
#define  MM_ICOM_WAVEIN                     3
#define  MM_ICOM_WAVEOUT                    4
#define  MM_ICOM_MIXER                      5
#define  MM_ICOM_AUX                        6
#define  MM_ICOM_LINE                       7

/*  MM_SANYO product IDs */
#define  MM_SANYO_ACM_LD_ADPCM              1

/* MM_AHEAD product IDs */
#define  MM_AHEAD_MULTISOUND                1
#define  MM_AHEAD_SOUNDBLASTER              2
#define  MM_AHEAD_PROAUDIO                  3
#define  MM_AHEAD_GENERIC                   4

/* MM_OLIVETTI product IDs */
#define  MM_OLIVETTI_WAVEIN                 1
#define  MM_OLIVETTI_WAVEOUT                2
#define  MM_OLIVETTI_MIXER                  3
#define  MM_OLIVETTI_AUX                    4
#define  MM_OLIVETTI_MIDIIN                 5
#define  MM_OLIVETTI_MIDIOUT                6
#define  MM_OLIVETTI_SYNTH                  7
#define  MM_OLIVETTI_JOYSTICK               8
#define  MM_OLIVETTI_ACM_GSM                9
#define  MM_OLIVETTI_ACM_ADPCM              10
#define  MM_OLIVETTI_ACM_CELP               11
#define  MM_OLIVETTI_ACM_SBC                12
#define  MM_OLIVETTI_ACM_OPR                13

/* MM_IOMAGIC product IDs */
#define  MM_IOMAGIC_TEMPO_WAVEOUT           1
#define  MM_IOMAGIC_TEMPO_WAVEIN            2
#define  MM_IOMAGIC_TEMPO_SYNTH             3
#define  MM_IOMAGIC_TEMPO_MIDIOUT           4
#define  MM_IOMAGIC_TEMPO_MXDOUT            5
#define  MM_IOMAGIC_TEMPO_AUXOUT            6

/* MM_MATSUSHITA product IDs */
#define  MM_MATSUSHITA_WAVEIN               1
#define  MM_MATSUSHITA_WAVEOUT              2
#define  MM_MATSUSHITA_FMSYNTH_STEREO       3
#define  MM_MATSUSHITA_MIXER                4
#define  MM_MATSUSHITA_AUX                  5

/* MM_NEWMEDIA product IDs */
#define  MM_NEWMEDIA_WAVJAMMER              1       /*  WSS Compatible sound card.  */

/* MM_LYRRUS product IDs */
#define  MM_LYRRUS_BRIDGE_GUITAR            1

/* MM_OPTI product IDs */
#define  MM_OPTI_M16_FMSYNTH_STEREO         0x0001
#define  MM_OPTI_M16_MIDIIN                 0x0002
#define  MM_OPTI_M16_MIDIOUT                0x0003
#define  MM_OPTI_M16_WAVEIN                 0x0004
#define  MM_OPTI_M16_WAVEOUT                0x0005
#define  MM_OPTI_M16_MIXER                  0x0006
#define  MM_OPTI_M16_AUX                    0x0007
#define  MM_OPTI_P16_FMSYNTH_STEREO         0x0010
#define  MM_OPTI_P16_MIDIIN                 0x0011
#define  MM_OPTI_P16_MIDIOUT                0x0012
#define  MM_OPTI_P16_WAVEIN                 0x0013
#define  MM_OPTI_P16_WAVEOUT                0x0014
#define  MM_OPTI_P16_MIXER                  0x0015
#define  MM_OPTI_P16_AUX                    0x0016
#define  MM_OPTI_M32_WAVEIN                 0x0020
#define  MM_OPTI_M32_WAVEOUT                0x0021
#define  MM_OPTI_M32_MIDIIN                 0x0022
#define  MM_OPTI_M32_MIDIOUT                0x0023
#define  MM_OPTI_M32_SYNTH_STEREO           0x0024
#define  MM_OPTI_M32_MIXER                  0x0025
#define  MM_OPTI_M32_AUX                    0x0026

/* MM_COMPAQ product IDs */
#define  MM_COMPAQ_BB_WAVEIN                1
#define  MM_COMPAQ_BB_WAVEOUT               2
#define  MM_COMPAQ_BB_WAVEAUX               3

/* MM_MPTUS product IDs */
#define  MM_MPTUS_SPWAVEOUT                 1       /* Sound Pallette */

/* MM_LERNOUT_AND_HAUSPIE product IDs */
#define  MM_LERNOUT_ANDHAUSPIE_LHCODECACM   1

/* MM_DIGITAL product IDs */
#define  MM_DIGITAL_AV320_WAVEIN            1       /* Digital Audio Video Compression Board */
#define  MM_DIGITAL_AV320_WAVEOUT           2       /* Digital Audio Video Compression Board */
#define  MM_DIGITAL_ACM_G723                3
#define  MM_DIGITAL_ICM_H263                4
#define  MM_DIGITAL_ICM_H261                5

/* MM_MOTU product IDs */
#define  MM_MOTU_MTP_MIDIOUT_ALL            100
#define  MM_MOTU_MTP_MIDIIN_1               101
#define  MM_MOTU_MTP_MIDIOUT_1              101
#define  MM_MOTU_MTP_MIDIIN_2               102
#define  MM_MOTU_MTP_MIDIOUT_2              102
#define  MM_MOTU_MTP_MIDIIN_3               103
#define  MM_MOTU_MTP_MIDIOUT_3              103
#define  MM_MOTU_MTP_MIDIIN_4               104
#define  MM_MOTU_MTP_MIDIOUT_4              104
#define  MM_MOTU_MTP_MIDIIN_5               105
#define  MM_MOTU_MTP_MIDIOUT_5              105
#define  MM_MOTU_MTP_MIDIIN_6               106
#define  MM_MOTU_MTP_MIDIOUT_6              106
#define  MM_MOTU_MTP_MIDIIN_7               107
#define  MM_MOTU_MTP_MIDIOUT_7              107
#define  MM_MOTU_MTP_MIDIIN_8               108
#define  MM_MOTU_MTP_MIDIOUT_8              108

#define  MM_MOTU_MTPII_MIDIOUT_ALL          200
#define  MM_MOTU_MTPII_MIDIIN_SYNC          200
#define  MM_MOTU_MTPII_MIDIIN_1             201
#define  MM_MOTU_MTPII_MIDIOUT_1            201
#define  MM_MOTU_MTPII_MIDIIN_2             202
#define  MM_MOTU_MTPII_MIDIOUT_2            202
#define  MM_MOTU_MTPII_MIDIIN_3             203
#define  MM_MOTU_MTPII_MIDIOUT_3            203
#define  MM_MOTU_MTPII_MIDIIN_4             204
#define  MM_MOTU_MTPII_MIDIOUT_4            204
#define  MM_MOTU_MTPII_MIDIIN_5             205
#define  MM_MOTU_MTPII_MIDIOUT_5            205
#define  MM_MOTU_MTPII_MIDIIN_6             206
#define  MM_MOTU_MTPII_MIDIOUT_6            206
#define  MM_MOTU_MTPII_MIDIIN_7             207
#define  MM_MOTU_MTPII_MIDIOUT_7            207
#define  MM_MOTU_MTPII_MIDIIN_8             208
#define  MM_MOTU_MTPII_MIDIOUT_8            208
#define  MM_MOTU_MTPII_NET_MIDIIN_1         209
#define  MM_MOTU_MTPII_NET_MIDIOUT_1        209
#define  MM_MOTU_MTPII_NET_MIDIIN_2         210
#define  MM_MOTU_MTPII_NET_MIDIOUT_2        210
#define  MM_MOTU_MTPII_NET_MIDIIN_3         211
#define  MM_MOTU_MTPII_NET_MIDIOUT_3        211
#define  MM_MOTU_MTPII_NET_MIDIIN_4         212
#define  MM_MOTU_MTPII_NET_MIDIOUT_4        212
#define  MM_MOTU_MTPII_NET_MIDIIN_5         213
#define  MM_MOTU_MTPII_NET_MIDIOUT_5        213
#define  MM_MOTU_MTPII_NET_MIDIIN_6         214
#define  MM_MOTU_MTPII_NET_MIDIOUT_6        214
#define  MM_MOTU_MTPII_NET_MIDIIN_7         215
#define  MM_MOTU_MTPII_NET_MIDIOUT_7        215
#define  MM_MOTU_MTPII_NET_MIDIIN_8         216
#define  MM_MOTU_MTPII_NET_MIDIOUT_8        216

#define  MM_MOTU_MXP_MIDIIN_MIDIOUT_ALL     300
#define  MM_MOTU_MXP_MIDIIN_SYNC            300
#define  MM_MOTU_MXP_MIDIIN_MIDIIN_1        301
#define  MM_MOTU_MXP_MIDIIN_MIDIOUT_1       301
#define  MM_MOTU_MXP_MIDIIN_MIDIIN_2        302
#define  MM_MOTU_MXP_MIDIIN_MIDIOUT_2       302
#define  MM_MOTU_MXP_MIDIIN_MIDIIN_3        303
#define  MM_MOTU_MXP_MIDIIN_MIDIOUT_3       303
#define  MM_MOTU_MXP_MIDIIN_MIDIIN_4        304
#define  MM_MOTU_MXP_MIDIIN_MIDIOUT_4       304
#define  MM_MOTU_MXP_MIDIIN_MIDIIN_5        305
#define  MM_MOTU_MXP_MIDIIN_MIDIOUT_5       305
#define  MM_MOTU_MXP_MIDIIN_MIDIIN_6        306
#define  MM_MOTU_MXP_MIDIIN_MIDIOUT_6       306

#define  MM_MOTU_MXPMPU_MIDIOUT_ALL         400
#define  MM_MOTU_MXPMPU_MIDIIN_SYNC         400
#define  MM_MOTU_MXPMPU_MIDIIN_1            401
#define  MM_MOTU_MXPMPU_MIDIOUT_1           401
#define  MM_MOTU_MXPMPU_MIDIIN_2            402
#define  MM_MOTU_MXPMPU_MIDIOUT_2           402
#define  MM_MOTU_MXPMPU_MIDIIN_3            403
#define  MM_MOTU_MXPMPU_MIDIOUT_3           403
#define  MM_MOTU_MXPMPU_MIDIIN_4            404
#define  MM_MOTU_MXPMPU_MIDIOUT_4           404
#define  MM_MOTU_MXPMPU_MIDIIN_5            405
#define  MM_MOTU_MXPMPU_MIDIOUT_5           405
#define  MM_MOTU_MXPMPU_MIDIIN_6            406
#define  MM_MOTU_MXPMPU_MIDIOUT_6           406

#define  MM_MOTU_MXN_MIDIOUT_ALL            500
#define  MM_MOTU_MXN_MIDIIN_SYNC            500
#define  MM_MOTU_MXN_MIDIIN_1               501
#define  MM_MOTU_MXN_MIDIOUT_1              501
#define  MM_MOTU_MXN_MIDIIN_2               502
#define  MM_MOTU_MXN_MIDIOUT_2              502
#define  MM_MOTU_MXN_MIDIIN_3               503
#define  MM_MOTU_MXN_MIDIOUT_3              503
#define  MM_MOTU_MXN_MIDIIN_4               504
#define  MM_MOTU_MXN_MIDIOUT_4              504

#define  MM_MOTU_FLYER_MIDI_IN_SYNC         600
#define  MM_MOTU_FLYER_MIDI_IN_A            601
#define  MM_MOTU_FLYER_MIDI_OUT_A           601
#define  MM_MOTU_FLYER_MIDI_IN_B            602
#define  MM_MOTU_FLYER_MIDI_OUT_B           602

#define  MM_MOTU_PKX_MIDI_IN_SYNC           700
#define  MM_MOTU_PKX_MIDI_IN_A              701
#define  MM_MOTU_PKX_MIDI_OUT_A             701
#define  MM_MOTU_PKX_MIDI_IN_B              702
#define  MM_MOTU_PKX_MIDI_OUT_B             702

#define  MM_MOTU_DTX_MIDI_IN_SYNC           800
#define  MM_MOTU_DTX_MIDI_IN_A              801
#define  MM_MOTU_DTX_MIDI_OUT_A             801
#define  MM_MOTU_DTX_MIDI_IN_B              802
#define  MM_MOTU_DTX_MIDI_OUT_B             802

#define  MM_MOTU_MTPAV_MIDIOUT_ALL          900
#define  MM_MOTU_MTPAV_MIDIIN_SYNC          900
#define  MM_MOTU_MTPAV_MIDIIN_1             901
#define  MM_MOTU_MTPAV_MIDIOUT_1            901
#define  MM_MOTU_MTPAV_MIDIIN_2             902
#define  MM_MOTU_MTPAV_MIDIOUT_2            902
#define  MM_MOTU_MTPAV_MIDIIN_3             903
#define  MM_MOTU_MTPAV_MIDIOUT_3            903
#define  MM_MOTU_MTPAV_MIDIIN_4             904
#define  MM_MOTU_MTPAV_MIDIOUT_4            904
#define  MM_MOTU_MTPAV_MIDIIN_5             905
#define  MM_MOTU_MTPAV_MIDIOUT_5            905
#define  MM_MOTU_MTPAV_MIDIIN_6             906
#define  MM_MOTU_MTPAV_MIDIOUT_6            906
#define  MM_MOTU_MTPAV_MIDIIN_7             907
#define  MM_MOTU_MTPAV_MIDIOUT_7            907
#define  MM_MOTU_MTPAV_MIDIIN_8             908
#define  MM_MOTU_MTPAV_MIDIOUT_8            908
#define  MM_MOTU_MTPAV_NET_MIDIIN_1         909
#define  MM_MOTU_MTPAV_NET_MIDIOUT_1        909
#define  MM_MOTU_MTPAV_NET_MIDIIN_2         910
#define  MM_MOTU_MTPAV_NET_MIDIOUT_2        910
#define  MM_MOTU_MTPAV_NET_MIDIIN_3         911
#define  MM_MOTU_MTPAV_NET_MIDIOUT_3        911
#define  MM_MOTU_MTPAV_NET_MIDIIN_4         912
#define  MM_MOTU_MTPAV_NET_MIDIOUT_4        912
#define  MM_MOTU_MTPAV_NET_MIDIIN_5         913
#define  MM_MOTU_MTPAV_NET_MIDIOUT_5        913
#define  MM_MOTU_MTPAV_NET_MIDIIN_6         914
#define  MM_MOTU_MTPAV_NET_MIDIOUT_6        914
#define  MM_MOTU_MTPAV_NET_MIDIIN_7         915
#define  MM_MOTU_MTPAV_NET_MIDIOUT_7        915
#define  MM_MOTU_MTPAV_NET_MIDIIN_8         916
#define  MM_MOTU_MTPAV_NET_MIDIOUT_8        916
#define  MM_MOTU_MTPAV_MIDIIN_ADAT          917
#define  MM_MOTU_MTPAV_MIDIOUT_ADAT         917
#define  MM_MOTU_MXPXT_MIDIIN_SYNC          1000
#define  MM_MOTU_MXPXT_MIDIOUT_ALL          1000
#define  MM_MOTU_MXPXT_MIDIIN_1             1001
#define  MM_MOTU_MXPXT_MIDIOUT_1            1001
#define  MM_MOTU_MXPXT_MIDIOUT_2            1002
#define  MM_MOTU_MXPXT_MIDIIN_2             1002
#define  MM_MOTU_MXPXT_MIDIIN_3             1003
#define  MM_MOTU_MXPXT_MIDIOUT_3            1003
#define  MM_MOTU_MXPXT_MIDIIN_4             1004
#define  MM_MOTU_MXPXT_MIDIOUT_4            1004
#define  MM_MOTU_MXPXT_MIDIIN_5             1005
#define  MM_MOTU_MXPXT_MIDIOUT_5            1005
#define  MM_MOTU_MXPXT_MIDIOUT_6            1006
#define  MM_MOTU_MXPXT_MIDIIN_6             1006
#define  MM_MOTU_MXPXT_MIDIOUT_7            1007
#define  MM_MOTU_MXPXT_MIDIIN_7             1007
#define  MM_MOTU_MXPXT_MIDIOUT_8            1008
#define  MM_MOTU_MXPXT_MIDIIN_8             1008

/* MM_WORKBIT product IDs */
#define  MM_WORKBIT_MIXER                   1      /* Harmony Mixer */
#define  MM_WORKBIT_WAVEOUT                 2      /* Harmony Mixer */
#define  MM_WORKBIT_WAVEIN                  3      /* Harmony Mixer */
#define  MM_WORKBIT_MIDIIN                  4      /* Harmony Mixer */
#define  MM_WORKBIT_MIDIOUT                 5      /* Harmony Mixer */
#define  MM_WORKBIT_FMSYNTH                 6      /* Harmony Mixer */
#define  MM_WORKBIT_AUX                     7      /* Harmony Mixer */
#define  MM_WORKBIT_JOYSTICK                8

/* MM_OSITECH product IDs */
#define  MM_OSITECH_TRUMPCARD               1       /* Trumpcard */

/* MM_MIRO product IDs */
#define  MM_MIRO_MOVIEPRO                   1       /* miroMOVIE pro */
#define  MM_MIRO_VIDEOD1                    2       /* miroVIDEO D1 */
#define  MM_MIRO_VIDEODC1TV                 3       /* miroVIDEO DC1 tv */
#define  MM_MIRO_VIDEOTD                    4       /* miroVIDEO 10/20 TD */
#define  MM_MIRO_DC30_WAVEOUT               5
#define  MM_MIRO_DC30_WAVEIN                6
#define  MM_MIRO_DC30_MIX                   7

/* MM_ISOLUTION product IDs */
#define  MM_ISOLUTION_PASCAL                1

/* MM_ROCKWELL product IDs */
#define  MM_VOICEMIXER                      1
#define  ROCKWELL_WA1_WAVEIN                100
#define  ROCKWELL_WA1_WAVEOUT               101
#define  ROCKWELL_WA1_SYNTH                 102
#define  ROCKWELL_WA1_MIXER                 103
#define  ROCKWELL_WA1_MPU401_IN             104
#define  ROCKWELL_WA1_MPU401_OUT            105
#define  ROCKWELL_WA2_WAVEIN                200
#define  ROCKWELL_WA2_WAVEOUT               201
#define  ROCKWELL_WA2_SYNTH                 202
#define  ROCKWELL_WA2_MIXER                 203
#define  ROCKWELL_WA2_MPU401_IN             204
#define  ROCKWELL_WA2_MPU401_OUT            205

/* MM_VOXWARE product IDs */
#define  MM_VOXWARE_CODEC                   1

/* MM_NORTHERN_TELECOM product IDs */
#define  MM_NORTEL_MPXAC_WAVEIN             1       /* MPX Audio Card Wave Input Device */
#define  MM_NORTEL_MPXAC_WAVEOUT            2       /* MPX Audio Card Wave Output Device */

/* MM_ADDX product IDs */
#define  MM_ADDX_PCTV_DIGITALMIX            1       /* MM_ADDX_PCTV_DIGITALMIX */
#define  MM_ADDX_PCTV_WAVEIN                2       /* MM_ADDX_PCTV_WAVEIN */
#define  MM_ADDX_PCTV_WAVEOUT               3       /* MM_ADDX_PCTV_WAVEOUT */
#define  MM_ADDX_PCTV_MIXER                 4       /* MM_ADDX_PCTV_MIXER */
#define  MM_ADDX_PCTV_AUX_CD                5       /* MM_ADDX_PCTV_AUX_CD */
#define  MM_ADDX_PCTV_AUX_LINE              6       /* MM_ADDX_PCTV_AUX_LINE */

/* MM_WILDCAT product IDs */
#define  MM_WILDCAT_AUTOSCOREMIDIIN         1       /* Autoscore */

/* MM_RHETOREX product IDs */
#define  MM_RHETOREX_WAVEIN                 1
#define  MM_RHETOREX_WAVEOUT                2

/* MM_BROOKTREE product IDs */
#define  MM_BTV_WAVEIN                      1       /* Brooktree PCM Wave Audio In */
#define  MM_BTV_WAVEOUT                     2       /* Brooktree PCM Wave Audio Out */
#define  MM_BTV_MIDIIN                      3       /* Brooktree MIDI In */
#define  MM_BTV_MIDIOUT                     4       /* Brooktree MIDI out */
#define  MM_BTV_MIDISYNTH                   5       /* Brooktree MIDI FM synth */
#define  MM_BTV_AUX_LINE                    6       /* Brooktree Line Input */
#define  MM_BTV_AUX_MIC                     7       /* Brooktree Microphone Input */
#define  MM_BTV_AUX_CD                      8       /* Brooktree CD Input */
#define  MM_BTV_DIGITALIN                   9       /* Brooktree PCM Wave in with subcode information */
#define  MM_BTV_DIGITALOUT                  10      /* Brooktree PCM Wave out with subcode information */
#define  MM_BTV_MIDIWAVESTREAM              11      /* Brooktree WaveStream */
#define  MM_BTV_MIXER                       12      /* Brooktree WSS Mixer driver */

/* MM_ENSONIQ product IDs */
#define  MM_ENSONIQ_SOUNDSCAPE              0x10    /* ENSONIQ Soundscape */
#define  MM_SOUNDSCAPE_WAVEOUT              MM_ENSONIQ_SOUNDSCAPE+1
#define  MM_SOUNDSCAPE_WAVEOUT_AUX          MM_ENSONIQ_SOUNDSCAPE+2
#define  MM_SOUNDSCAPE_WAVEIN               MM_ENSONIQ_SOUNDSCAPE+3
#define  MM_SOUNDSCAPE_MIDIOUT              MM_ENSONIQ_SOUNDSCAPE+4
#define  MM_SOUNDSCAPE_MIDIIN               MM_ENSONIQ_SOUNDSCAPE+5
#define  MM_SOUNDSCAPE_SYNTH                MM_ENSONIQ_SOUNDSCAPE+6
#define  MM_SOUNDSCAPE_MIXER                MM_ENSONIQ_SOUNDSCAPE+7
#define  MM_SOUNDSCAPE_AUX                  MM_ENSONIQ_SOUNDSCAPE+8

/* MM_NVIDIA product IDs */
#define  MM_NVIDIA_WAVEOUT                  1
#define  MM_NVIDIA_WAVEIN                   2
#define  MM_NVIDIA_MIDIOUT                  3
#define  MM_NVIDIA_MIDIIN                   4
#define  MM_NVIDIA_GAMEPORT                 5
#define  MM_NVIDIA_MIXER                    6
#define  MM_NVIDIA_AUX                      7

/* MM_OKSORI product IDs */
#define  MM_OKSORI_BASE                     0                      /* Oksori Base */
#define  MM_OKSORI_OSR8_WAVEOUT             MM_OKSORI_BASE+1       /* Oksori 8bit Wave out */
#define  MM_OKSORI_OSR8_WAVEIN              MM_OKSORI_BASE+2       /* Oksori 8bit Wave in */
#define  MM_OKSORI_OSR16_WAVEOUT            MM_OKSORI_BASE+3       /* Oksori 16 bit Wave out */
#define  MM_OKSORI_OSR16_WAVEIN             MM_OKSORI_BASE+4       /* Oksori 16 bit Wave in */
#define  MM_OKSORI_FM_OPL4                  MM_OKSORI_BASE+5       /* Oksori FM Synth Yamaha OPL4 */
#define  MM_OKSORI_MIX_MASTER               MM_OKSORI_BASE+6       /* Oksori DSP Mixer - Master Volume */
#define  MM_OKSORI_MIX_WAVE                 MM_OKSORI_BASE+7       /* Oksori DSP Mixer - Wave Volume */
#define  MM_OKSORI_MIX_FM                   MM_OKSORI_BASE+8       /* Oksori DSP Mixer - FM Volume */
#define  MM_OKSORI_MIX_LINE                 MM_OKSORI_BASE+9       /* Oksori DSP Mixer - Line Volume */
#define  MM_OKSORI_MIX_CD                   MM_OKSORI_BASE+10      /* Oksori DSP Mixer - CD Volume */
#define  MM_OKSORI_MIX_MIC                  MM_OKSORI_BASE+11      /* Oksori DSP Mixer - MIC Volume */
#define  MM_OKSORI_MIX_ECHO                 MM_OKSORI_BASE+12      /* Oksori DSP Mixer - Echo Volume */
#define  MM_OKSORI_MIX_AUX1                 MM_OKSORI_BASE+13      /* Oksori AD1848 - AUX1 Volume */
#define  MM_OKSORI_MIX_LINE1                MM_OKSORI_BASE+14      /* Oksori AD1848 - LINE1 Volume */
#define  MM_OKSORI_EXT_MIC1                 MM_OKSORI_BASE+15      /* Oksori External - One Mic Connect */
#define  MM_OKSORI_EXT_MIC2                 MM_OKSORI_BASE+16      /* Oksori External - Two Mic Connect */
#define  MM_OKSORI_MIDIOUT                  MM_OKSORI_BASE+17      /* Oksori MIDI Out Device */
#define  MM_OKSORI_MIDIIN                   MM_OKSORI_BASE+18      /* Oksori MIDI In Device */
#define  MM_OKSORI_MPEG_CDVISION            MM_OKSORI_BASE+19      /* Oksori CD-Vision MPEG Decoder */

/* MM_DIACOUSTICS product IDs */
#define  MM_DIACOUSTICS_DRUM_ACTION         1       /* Drum Action */

/* MM_KAY_ELEMETRICS product IDs */
#define  MM_KAY_ELEMETRICS_CSL              0x4300
#define  MM_KAY_ELEMETRICS_CSL_DAT          0x4308
#define  MM_KAY_ELEMETRICS_CSL_4CHANNEL     0x4309

/* MM_CRYSTAL product IDs */
#define  MM_CRYSTAL_CS4232_WAVEIN           1
#define  MM_CRYSTAL_CS4232_WAVEOUT          2
#define  MM_CRYSTAL_CS4232_WAVEMIXER        3
#define  MM_CRYSTAL_CS4232_WAVEAUX_AUX1     4
#define  MM_CRYSTAL_CS4232_WAVEAUX_AUX2     5
#define  MM_CRYSTAL_CS4232_WAVEAUX_LINE     6
#define  MM_CRYSTAL_CS4232_WAVEAUX_MONO     7
#define  MM_CRYSTAL_CS4232_WAVEAUX_MASTER   8
#define  MM_CRYSTAL_CS4232_MIDIIN           9
#define  MM_CRYSTAL_CS4232_MIDIOUT          10
#define  MM_CRYSTAL_CS4232_INPUTGAIN_AUX1   13
#define  MM_CRYSTAL_CS4232_INPUTGAIN_LOOP   14
#define  MM_CRYSTAL_SOUND_FUSION_WAVEIN     21
#define  MM_CRYSTAL_SOUND_FUSION_WAVEOUT    22
#define  MM_CRYSTAL_SOUND_FUSION_MIXER      23
#define  MM_CRYSTAL_SOUND_FUSION_MIDIIN     24
#define  MM_CRYSTAL_SOUND_FUSION_MIDIOUT    25
#define  MM_CRYSTAL_SOUND_FUSION_JOYSTICK   26

/* MM_QUARTERDECK product IDs */
#define  MM_QUARTERDECK_LHWAVEIN            0      /* Quarterdeck L&H Codec Wave In */
#define  MM_QUARTERDECK_LHWAVEOUT           1      /* Quarterdeck L&H Codec Wave Out */

/* MM_TDK product IDs */
#define  MM_TDK_MW_MIDI_SYNTH               1
#define  MM_TDK_MW_MIDI_IN                  2
#define  MM_TDK_MW_MIDI_OUT                 3
#define  MM_TDK_MW_WAVE_IN                  4
#define  MM_TDK_MW_WAVE_OUT                 5
#define  MM_TDK_MW_AUX                      6
#define  MM_TDK_MW_MIXER                    10
#define  MM_TDK_MW_AUX_MASTER               100
#define  MM_TDK_MW_AUX_BASS                 101
#define  MM_TDK_MW_AUX_TREBLE               102
#define  MM_TDK_MW_AUX_MIDI_VOL             103
#define  MM_TDK_MW_AUX_WAVE_VOL             104
#define  MM_TDK_MW_AUX_WAVE_RVB             105
#define  MM_TDK_MW_AUX_WAVE_CHR             106
#define  MM_TDK_MW_AUX_VOL                  107
#define  MM_TDK_MW_AUX_RVB                  108
#define  MM_TDK_MW_AUX_CHR                  109

/* MM_DIGITAL_AUDIO_LABS product IDs */
#define  MM_DIGITAL_AUDIO_LABS_TC           0x01
#define  MM_DIGITAL_AUDIO_LABS_DOC          0x02
#define  MM_DIGITAL_AUDIO_LABS_V8           0x10
#define  MM_DIGITAL_AUDIO_LABS_CPRO         0x11
#define  MM_DIGITAL_AUDIO_LABS_VP           0x12
#define  MM_DIGITAL_AUDIO_LABS_CDLX         0x13
#define  MM_DIGITAL_AUDIO_LABS_CTDIF        0x14

/* MM_SEERSYS product IDs */
#define  MM_SEERSYS_SEERSYNTH               1
#define  MM_SEERSYS_SEERWAVE                2
#define  MM_SEERSYS_SEERMIX                 3
#define  MM_SEERSYS_WAVESYNTH               4
#define  MM_SEERSYS_WAVESYNTH_WG            5
#define  MM_SEERSYS_REALITY                 6

/* MM_OSPREY product IDs */
#define  MM_OSPREY_1000WAVEIN               1
#define  MM_OSPREY_1000WAVEOUT              2

/* MM_SOUNDESIGNS product IDs */
#define  MM_SOUNDESIGNS_WAVEIN              1
#define  MM_SOUNDESIGNS_WAVEOUT             2

/* MM_SPECTRUM_SIGNAL_PROCESSING product IDs */
#define  MM_SSP_SNDFESWAVEIN                1       /* Sound Festa Wave In Device */
#define  MM_SSP_SNDFESWAVEOUT               2       /* Sound Festa Wave Out Device */
#define  MM_SSP_SNDFESMIDIIN                3       /* Sound Festa MIDI In Device */
#define  MM_SSP_SNDFESMIDIOUT               4       /* Sound Festa MIDI Out Device */
#define  MM_SSP_SNDFESSYNTH                 5       /* Sound Festa MIDI Synth Device */
#define  MM_SSP_SNDFESMIX                   6       /* Sound Festa Mixer Device */
#define  MM_SSP_SNDFESAUX                   7       /* Sound Festa Auxilliary Device */

/* MM_ECS product IDs */
#define  MM_ECS_AADF_MIDI_IN                10
#define  MM_ECS_AADF_MIDI_OUT               11
#define  MM_ECS_AADF_WAVE2MIDI_IN           12

/* MM_AMD product IDs */
#define  MM_AMD_INTERWAVE_WAVEIN            1
#define  MM_AMD_INTERWAVE_WAVEOUT           2
#define  MM_AMD_INTERWAVE_SYNTH             3
#define  MM_AMD_INTERWAVE_MIXER1            4
#define  MM_AMD_INTERWAVE_MIXER2            5
#define  MM_AMD_INTERWAVE_JOYSTICK          6
#define  MM_AMD_INTERWAVE_EX_CD             7
#define  MM_AMD_INTERWAVE_MIDIIN            8
#define  MM_AMD_INTERWAVE_MIDIOUT           9
#define  MM_AMD_INTERWAVE_AUX1              10
#define  MM_AMD_INTERWAVE_AUX2              11
#define  MM_AMD_INTERWAVE_AUX_MIC           12
#define  MM_AMD_INTERWAVE_AUX_CD            13
#define  MM_AMD_INTERWAVE_MONO_IN           14
#define  MM_AMD_INTERWAVE_MONO_OUT          15
#define  MM_AMD_INTERWAVE_EX_TELEPHONY      16
#define  MM_AMD_INTERWAVE_WAVEOUT_BASE      17
#define  MM_AMD_INTERWAVE_WAVEOUT_TREBLE    18
#define  MM_AMD_INTERWAVE_STEREO_ENHANCED   19

/* MM_COREDYNAMICS product IDs */
#define  MM_COREDYNAMICS_DYNAMIXHR          1       /* DynaMax Hi-Rez */
#define  MM_COREDYNAMICS_DYNASONIX_SYNTH    2       /* DynaSonix */
#define  MM_COREDYNAMICS_DYNASONIX_MIDI_IN  3
#define  MM_COREDYNAMICS_DYNASONIX_MIDI_OUT 4
#define  MM_COREDYNAMICS_DYNASONIX_WAVE_IN  5
#define  MM_COREDYNAMICS_DYNASONIX_WAVE_OUT 6
#define  MM_COREDYNAMICS_DYNASONIX_AUDIO_IN 7
#define  MM_COREDYNAMICS_DYNASONIX_AUDIO_OUT    8
#define  MM_COREDYNAMICS_DYNAGRAFX_VGA      9       /* DynaGrfx */
#define  MM_COREDYNAMICS_DYNAGRAFX_WAVE_IN  10
#define  MM_COREDYNAMICS_DYNAGRAFX_WAVE_OUT 11

/* MM_CANAM product IDs */
#define  MM_CANAM_CBXWAVEOUT                1
#define  MM_CANAM_CBXWAVEIN                 2

/* MM_SOFTSOUND product IDs */
#define  MM_SOFTSOUND_CODEC                 1

/* MM_NORRIS product IDs */
#define  MM_NORRIS_VOICELINK                1

/* MM_DDD product IDs */
#define  MM_DDD_MIDILINK_MIDIIN             1
#define  MM_DDD_MIDILINK_MIDIOUT            2

/* MM_EUPHONICS product IDs */
#define  MM_EUPHONICS_AUX_CD                1
#define  MM_EUPHONICS_AUX_LINE              2
#define  MM_EUPHONICS_AUX_MASTER            3
#define  MM_EUPHONICS_AUX_MIC               4
#define  MM_EUPHONICS_AUX_MIDI              5
#define  MM_EUPHONICS_AUX_WAVE              6
#define  MM_EUPHONICS_FMSYNTH_MONO          7
#define  MM_EUPHONICS_FMSYNTH_STEREO        8
#define  MM_EUPHONICS_MIDIIN                9
#define  MM_EUPHONICS_MIDIOUT               10
#define  MM_EUPHONICS_MIXER                 11
#define  MM_EUPHONICS_WAVEIN                12
#define  MM_EUPHONICS_WAVEOUT               13
#define  MM_EUPHONICS_EUSYNTH               14

/* MM_CRYSTAL_NET product IDs */
#define  CRYSTAL_NET_SFM_CODEC              1

/* MM_CHROMATIC product IDs */
#define  MM_CHROMATIC_M1                    0x0001
#define  MM_CHROMATIC_M1_WAVEIN             0x0002
#define  MM_CHROMATIC_M1_WAVEOUT            0x0003
#define  MM_CHROMATIC_M1_FMSYNTH            0x0004
#define  MM_CHROMATIC_M1_MIXER              0x0005
#define  MM_CHROMATIC_M1_AUX                0x0006
#define  MM_CHROMATIC_M1_AUX_CD             0x0007
#define  MM_CHROMATIC_M1_MIDIIN             0x0008
#define  MM_CHROMATIC_M1_MIDIOUT            0x0009
#define  MM_CHROMATIC_M1_WTSYNTH            0x0010
#define  MM_CHROMATIC_M1_MPEGWAVEIN         0x0011
#define  MM_CHROMATIC_M1_MPEGWAVEOUT        0x0012
#define  MM_CHROMATIC_M2                    0x0013
#define  MM_CHROMATIC_M2_WAVEIN             0x0014
#define  MM_CHROMATIC_M2_WAVEOUT            0x0015
#define  MM_CHROMATIC_M2_FMSYNTH            0x0016
#define  MM_CHROMATIC_M2_MIXER              0x0017
#define  MM_CHROMATIC_M2_AUX                0x0018
#define  MM_CHROMATIC_M2_AUX_CD             0x0019
#define  MM_CHROMATIC_M2_MIDIIN             0x0020
#define  MM_CHROMATIC_M2_MIDIOUT            0x0021
#define  MM_CHROMATIC_M2_WTSYNTH            0x0022
#define  MM_CHROMATIC_M2_MPEGWAVEIN         0x0023
#define  MM_CHROMATIC_M2_MPEGWAVEOUT        0x0024

/* MM_VIENNASYS product IDs */
#define  MM_VIENNASYS_TSP_WAVE_DRIVER       1

/* MM_CONNECTIX product IDs */
#define  MM_CONNECTIX_VIDEC_CODEC           1

/* MM_GADGETLABS product IDs */
#define  MM_GADGETLABS_WAVE44_WAVEIN        1
#define  MM_GADGETLABS_WAVE44_WAVEOUT       2
#define  MM_GADGETLABS_WAVE42_WAVEIN        3
#define  MM_GADGETLABS_WAVE42_WAVEOUT       4
#define  MM_GADGETLABS_WAVE4_MIDIIN         5
#define  MM_GADGETLABS_WAVE4_MIDIOUT        6

/* MM_FRONTIER product IDs */
#define  MM_FRONTIER_WAVECENTER_MIDIIN      1       /* WaveCenter */
#define  MM_FRONTIER_WAVECENTER_MIDIOUT     2
#define  MM_FRONTIER_WAVECENTER_WAVEIN      3
#define  MM_FRONTIER_WAVECENTER_WAVEOUT     4

/* MM_VIONA product IDs */
#define  MM_VIONA_QVINPCI_MIXER             1       /* Q-Motion PCI II/Bravado 2000 */
#define  MM_VIONA_QVINPCI_WAVEIN            2
#define  MM_VIONAQVINPCI_WAVEOUT            3
#define  MM_VIONA_BUSTER_MIXER              4       /* Buster */
#define  MM_VIONA_CINEMASTER_MIXER          5       /* Cinemaster */
#define  MM_VIONA_CONCERTO_MIXER            6       /* Concerto */

/* MM_CASIO product IDs */
#define  MM_CASIO_WP150_MIDIOUT             1       /* wp150 */
#define  MM_CASIO_WP150_MIDIIN              2
#define  MM_CASIO_LSG_MIDIOUT               3

/* MM_DIAMONDMM product IDs */
#define  MM_DIMD_PLATFORM                   0       /* Freedom Audio */
#define  MM_DIMD_DIRSOUND                   1
#define  MM_DIMD_VIRTMPU                    2
#define  MM_DIMD_VIRTSB                     3
#define  MM_DIMD_VIRTJOY                    4
#define  MM_DIMD_WAVEIN                     5
#define  MM_DIMD_WAVEOUT                    6
#define  MM_DIMD_MIDIIN                     7
#define  MM_DIMD_MIDIOUT                    8
#define  MM_DIMD_AUX_LINE                   9
#define  MM_DIMD_MIXER                      10
#define  MM_DIMD_WSS_WAVEIN                 14
#define  MM_DIMD_WSS_WAVEOUT                15
#define  MM_DIMD_WSS_MIXER                  17
#define  MM_DIMD_WSS_AUX                    21
#define  MM_DIMD_WSS_SYNTH                  76

/* MM_S3 product IDs */
#define  MM_S3_WAVEOUT                      1
#define  MM_S3_WAVEIN                       2
#define  MM_S3_MIDIOUT                      3
#define  MM_S3_MIDIIN                       4
#define  MM_S3_FMSYNTH                      5
#define  MM_S3_MIXER                        6
#define  MM_S3_AUX                          7

/* MM_VANKOEVERING product IDs */
#define  MM_VKC_MPU401_MIDIIN               0x0100
#define  MM_VKC_SERIAL_MIDIIN               0x0101
#define  MM_VKC_MPU401_MIDIOUT              0x0200
#define  MM_VKC_SERIAL_MIDIOUT              0x0201

/* MM_ZEFIRO product IDs */
#define  MM_ZEFIRO_ZA2                      2

/* MM_FRAUNHOFER_IIS product IDs */
#define MM_FHGIIS_MPEGLAYER3_DECODE         9
#define MM_FHGIIS_MPEGLAYER3                10
#define MM_FHGIIS_MPEGLAYER3_LITE           10
#define MM_FHGIIS_MPEGLAYER3_BASIC          11
#define MM_FHGIIS_MPEGLAYER3_ADVANCED       12
#define MM_FHGIIS_MPEGLAYER3_PROFESSIONAL   13
#define MM_FHGIIS_MPEGLAYER3_ADVANCEDPLUS   14


/* MM_QUICKNET product IDs */
#define  MM_QUICKNET_PJWAVEIN               1
#define  MM_QUICKNET_PJWAVEOUT              2

/* MM_SICRESOURCE product IDs */
#define  MM_SICRESOURCE_SSO3D               2
#define  MM_SICRESOURCE_SSOW3DI             3

/* MM_NEOMAGIC product IDs */
#define  MM_NEOMAGIC_SYNTH                  1
#define  MM_NEOMAGIC_WAVEOUT                2
#define  MM_NEOMAGIC_WAVEIN                 3
#define  MM_NEOMAGIC_MIDIOUT                4
#define  MM_NEOMAGIC_MIDIIN                 5
#define  MM_NEOMAGIC_AUX                    6
#define  MM_NEOMAGIC_MW3DX_WAVEOUT          10
#define  MM_NEOMAGIC_MW3DX_WAVEIN           11
#define  MM_NEOMAGIC_MW3DX_MIDIOUT          12
#define  MM_NEOMAGIC_MW3DX_MIDIIN           13
#define  MM_NEOMAGIC_MW3DX_FMSYNTH          14
#define  MM_NEOMAGIC_MW3DX_GMSYNTH          15
#define  MM_NEOMAGIC_MW3DX_MIXER            16
#define  MM_NEOMAGIC_MW3DX_AUX              17
#define  MM_NEOMAGIC_MWAVE_WAVEOUT          20
#define  MM_NEOMAGIC_MWAVE_WAVEIN           21
#define  MM_NEOMAGIC_MWAVE_MIDIOUT          22
#define  MM_NEOMAGIC_MWAVE_MIDIIN           23
#define  MM_NEOMAGIC_MWAVE_MIXER            24
#define  MM_NEOMAGIC_MWAVE_AUX              25

/* MM_MERGING_TECHNOLOGIES product IDs */
#define  MM_MERGING_MPEGL3                  1

/* MM_XIRLINK product IDs */
#define  MM_XIRLINK_VISIONLINK              1

/* MM_OTI product IDs */
#define  MM_OTI_611WAVEIN                   5
#define  MM_OTI_611WAVEOUT                  6
#define  MM_OTI_611MIXER                    7
#define  MM_OTI_611MIDIN                    0x12
#define  MM_OTI_611MIDIOUT                  0x13

/* MM_AUREAL product IDs */
#define  MM_AUREAL_AU8820                   16
#define  MM_AU8820_SYNTH                    17
#define  MM_AU8820_WAVEOUT                  18
#define  MM_AU8820_WAVEIN                   19
#define  MM_AU8820_MIXER                    20
#define  MM_AU8820_AUX                      21
#define  MM_AU8820_MIDIOUT                  22
#define  MM_AU8820_MIDIIN                   23
#define  MM_AUREAL_AU8830                   32
#define  MM_AU8830_SYNTH                    33
#define  MM_AU8830_WAVEOUT                  34
#define  MM_AU8830_WAVEIN                   35
#define  MM_AU8830_MIXER                    36
#define  MM_AU8830_AUX                      37
#define  MM_AU8830_MIDIOUT                  38
#define  MM_AU8830_MIDIIN                   39

/* MM_VIVO product IDs */
#define  MM_VIVO_AUDIO_CODEC                1

/* MM_SHARP product IDs */
#define  MM_SHARP_MDC_MIDI_SYNTH            1
#define  MM_SHARP_MDC_MIDI_IN               2
#define  MM_SHARP_MDC_MIDI_OUT              3
#define  MM_SHARP_MDC_WAVE_IN               4
#define  MM_SHARP_MDC_WAVE_OUT              5
#define  MM_SHARP_MDC_AUX                   6
#define  MM_SHARP_MDC_MIXER                 10
#define  MM_SHARP_MDC_AUX_MASTER            100
#define  MM_SHARP_MDC_AUX_BASS              101
#define  MM_SHARP_MDC_AUX_TREBLE            102
#define  MM_SHARP_MDC_AUX_MIDI_VOL          103
#define  MM_SHARP_MDC_AUX_WAVE_VOL          104
#define  MM_SHARP_MDC_AUX_WAVE_RVB          105
#define  MM_SHARP_MDC_AUX_WAVE_CHR          106
#define  MM_SHARP_MDC_AUX_VOL               107
#define  MM_SHARP_MDC_AUX_RVB               108
#define  MM_SHARP_MDC_AUX_CHR               109

/* MM_LUCENT product IDs */
#define  MM_LUCENT_ACM_G723                 0

/* MM_ATT product IDs */
#define  MM_ATT_G729A                       1

/* MM_MARIAN product IDs */
#define  MM_MARIAN_ARC44WAVEIN              1
#define  MM_MARIAN_ARC44WAVEOUT             2
#define  MM_MARIAN_PRODIF24WAVEIN           3
#define  MM_MARIAN_PRODIF24WAVEOUT          4
#define  MM_MARIAN_ARC88WAVEIN              5
#define  MM_MARIAN_ARC88WAVEOUT             6

/* MM_BCB product IDs */
#define  MM_BCB_NETBOARD_10                 1
#define  MM_BCB_TT75_10                     2

/* MM_MOTIONPIXELS product IDs */
#define  MM_MOTIONPIXELS_MVI2               1

/* MM_QDESIGN product IDs */
#define  MM_QDESIGN_ACM_MPEG                1
#define  MM_QDESIGN_ACM_QDESIGN_MUSIC       2

/* MM_NMP product IDs */
#define  MM_NMP_CCP_WAVEIN                  1
#define  MM_NMP_CCP_WAVEOUT                 2
#define  MM_NMP_ACM_AMR                     10

/* MM_DATAFUSION product IDs */
#define  MM_DF_ACM_G726                     1
#define  MM_DF_ACM_GSM610                   2

/* MM_BERCOS product IDs */
#define  MM_BERCOS_WAVEIN                   1
#define  MM_BERCOS_MIXER                    2
#define  MM_BERCOS_WAVEOUT                  3

/* MM_ONLIVE product IDs */
#define  MM_ONLIVE_MPCODEC                  1

/* MM_PHONET product IDs */
#define  MM_PHONET_PP_WAVEOUT               1
#define  MM_PHONET_PP_WAVEIN                2
#define  MM_PHONET_PP_MIXER                 3

/* MM_FTR product IDs */
#define  MM_FTR_ENCODER_WAVEIN              1
#define  MM_FTR_ACM                         2

/* MM_ENET product IDs */
#define  MM_ENET_T2000_LINEIN               1
#define  MM_ENET_T2000_LINEOUT              2
#define  MM_ENET_T2000_HANDSETIN            3
#define  MM_ENET_T2000_HANDSETOUT           4

/*  MM_EMAGIC product IDs */
#define  MM_EMAGIC_UNITOR8                  1

/*  MM_SIPROLAB product IDs */
#define  MM_SIPROLAB_ACELPNET               1

/*  MM_DICTAPHONE product IDs */
#define  MM_DICTAPHONE_G726                 1       /* G726 ACM codec (g726pcm.acm) */

/*  MM_RZS product IDs */
#define  MM_RZS_ACM_TUBGSM                  1      /* GSM 06.10 CODEC */

/*  MM_EES product IDs */
#define  MM_EES_PCMIDI14                    1
#define  MM_EES_PCMIDI14_IN                 2
#define  MM_EES_PCMIDI14_OUT1               3
#define  MM_EES_PCMIDI14_OUT2               4
#define  MM_EES_PCMIDI14_OUT3               5
#define  MM_EES_PCMIDI14_OUT4               6

/*  MM_HAFTMANN product IDs */
#define  MM_HAFTMANN_LPTDAC2                1

/*  MM_LUCID product IDs */
#define  MM_LUCID_PCI24WAVEIN               1
#define  MM_LUCID_PCI24WAVEOUT              2

/*  MM_HEADSPACE product IDs */
#define  MM_HEADSPACE_HAESYNTH              1
#define  MM_HEADSPACE_HAEWAVEOUT            2
#define  MM_HEADSPACE_HAEWAVEIN             3
#define  MM_HEADSPACE_HAEMIXER              4

/*  MM_UNISYS product IDs */
#define  MM_UNISYS_ACM_NAP                  1

/*  MM_LUMINOSITI product IDs */

#define  MM_LUMINOSITI_SCWAVEIN             1
#define  MM_LUMINOSITI_SCWAVEOUT            2
#define  MM_LUMINOSITI_SCWAVEMIX            3

/*  MM_ACTIVEVOICE product IDs */
#define  MM_ACTIVEVOICE_ACM_VOXADPCM        1

/*  MM_DTS product IDs */
#define  MM_DTS_DS                          1

/*  MM_SOFTLAB_NSK product IDs */
#define  MM_SOFTLAB_NSK_FRW_WAVEIN          1
#define  MM_SOFTLAB_NSK_FRW_WAVEOUT         2
#define  MM_SOFTLAB_NSK_FRW_MIXER           3
#define  MM_SOFTLAB_NSK_FRW_AUX             4

/*  MM_FORTEMEDIA product IDs */
#define  MM_FORTEMEDIA_WAVEIN               1
#define  MM_FORTEMEDIA_WAVEOUT              2
#define  MM_FORTEMEDIA_FMSYNC               3
#define  MM_FORTEMEDIA_MIXER                4
#define  MM_FORTEMEDIA_AUX                  5

/*  MM_SONORUS product IDs */
#define  MM_SONORUS_STUDIO                  1

/*  MM_I_LINK product IDs */
#define  MM_I_LINK_VOICE_CODER              1

/*  MM_SELSIUS_SYSTEMS product IDs */
#define  MM_SELSIUS_SYSTEMS_RTPWAVEOUT      1
#define  MM_SELSIUS_SYSTEMS_RTPWAVEIN       2

/*  MM_ADMOS product IDs */
#define  MM_ADMOS_FM_SYNTH                  1
#define  MM_ADMOS_QS3AMIDIOUT               2
#define  MM_ADMOS_QS3AMIDIIN                3
#define  MM_ADMOS_QS3AWAVEOUT               4
#define  MM_ADMOS_QS3AWAVEIN                5

/* MM_LEXICON product IDs */
#define  MM_LEXICON_STUDIO_WAVE_OUT         1
#define  MM_LEXICON_STUDIO_WAVE_IN          2

/* MM_SGI product IDs */
#define  MM_SGI_320_WAVEIN                  1
#define  MM_SGI_320_WAVEOUT                 2
#define  MM_SGI_320_MIXER                   3
#define  MM_SGI_540_WAVEIN                  4
#define  MM_SGI_540_WAVEOUT                 5
#define  MM_SGI_540_MIXER                   6
#define  MM_SGI_RAD_ADATMONO1_WAVEIN        7
#define  MM_SGI_RAD_ADATMONO2_WAVEIN        8
#define  MM_SGI_RAD_ADATMONO3_WAVEIN        9
#define  MM_SGI_RAD_ADATMONO4_WAVEIN        10
#define  MM_SGI_RAD_ADATMONO5_WAVEIN        11
#define  MM_SGI_RAD_ADATMONO6_WAVEIN        12
#define  MM_SGI_RAD_ADATMONO7_WAVEIN        13
#define  MM_SGI_RAD_ADATMONO8_WAVEIN        14
#define  MM_SGI_RAD_ADATSTEREO12_WAVEIN     15
#define  MM_SGI_RAD_ADATSTEREO34_WAVEIN     16
#define  MM_SGI_RAD_ADATSTEREO56_WAVEIN     17
#define  MM_SGI_RAD_ADATSTEREO78_WAVEIN     18
#define  MM_SGI_RAD_ADAT8CHAN_WAVEIN        19
#define  MM_SGI_RAD_ADATMONO1_WAVEOUT       20
#define  MM_SGI_RAD_ADATMONO2_WAVEOUT       21
#define  MM_SGI_RAD_ADATMONO3_WAVEOUT       22
#define  MM_SGI_RAD_ADATMONO4_WAVEOUT       23
#define  MM_SGI_RAD_ADATMONO5_WAVEOUT       24
#define  MM_SGI_RAD_ADATMONO6_WAVEOUT       25
#define  MM_SGI_RAD_ADATMONO7_WAVEOUT       26
#define  MM_SGI_RAD_ADATMONO8_WAVEOUT       27
#define  MM_SGI_RAD_ADATSTEREO12_WAVEOUT    28
#define  MM_SGI_RAD_ADATSTEREO32_WAVEOUT    29
#define  MM_SGI_RAD_ADATSTEREO56_WAVEOUT    30
#define  MM_SGI_RAD_ADATSTEREO78_WAVEOUT    31
#define  MM_SGI_RAD_ADAT8CHAN_WAVEOUT       32
#define  MM_SGI_RAD_AESMONO1_WAVEIN         33
#define  MM_SGI_RAD_AESMONO2_WAVEIN         34
#define  MM_SGI_RAD_AESSTEREO_WAVEIN        35
#define  MM_SGI_RAD_AESMONO1_WAVEOUT        36
#define  MM_SGI_RAD_AESMONO2_WAVEOUT        37
#define  MM_SGI_RAD_AESSTEREO_WAVEOUT       38

/* MM_IPI product IDs */
#define  MM_IPI_ACM_HSX                     1
#define  MM_IPI_ACM_RPELP                   2
#define  MM_IPI_WF_ASSS                     3
#define  MM_IPI_AT_WAVEOUT                  4
#define  MM_IPI_AT_WAVEIN                   5
#define  MM_IPI_AT_MIXER                    6

/* MM_ICE product IDs */
#define  MM_ICE_WAVEOUT                     1
#define  MM_ICE_WAVEIN                      2
#define  MM_ICE_MTWAVEOUT                   3
#define  MM_ICE_MTWAVEIN                    4
#define  MM_ICE_MIDIOUT1                    5
#define  MM_ICE_MIDIIN1                     6
#define  MM_ICE_MIDIOUT2                    7
#define  MM_ICE_MIDIIN2                     8
#define  MM_ICE_SYNTH                       9
#define  MM_ICE_MIXER                       10
#define  MM_ICE_AUX                         11

/* MM_VQST product IDs */
#define  MM_VQST_VQC1                       1
#define  MM_VQST_VQC2                       2

/* MM_ETEK product IDs */
#define  MM_ETEK_KWIKMIDI_MIDIIN            1
#define  MM_ETEK_KWIKMIDI_MIDIOUT           2

/* MM_INTERNET product IDs */
#define  MM_INTERNET_SSW_MIDIOUT            10
#define  MM_INTERNET_SSW_MIDIIN             11
#define  MM_INTERNET_SSW_WAVEOUT            12
#define  MM_INTERNET_SSW_WAVEIN             13

/* MM_SONY product IDs */
#define  MM_SONY_ACM_SCX                    1

/* MM_UHER_INFORMATIC product IDs */
#define  MM_UH_ACM_ADPCM                    1

/* MM_SYDEC_NV product IDs */
#define  MM_SYDEC_NV_WAVEIN                 1
#define  MM_SYDEC_NV_WAVEOUT                2

/* MM_FLEXION product IDs */
#define  MM_FLEXION_X300_WAVEIN             1
#define  MM_FLEXION_X300_WAVEOUT            2

/* MM_VIA product IDs */
#define  MM_VIA_WAVEOUT                     1
#define  MM_VIA_WAVEIN                      2
#define  MM_VIA_MIXER                       3
#define  MM_VIA_AUX                         4
#define  MM_VIA_MPU401_MIDIOUT              5
#define  MM_VIA_MPU401_MIDIIN               6
#define  MM_VIA_SWFM_SYNTH                  7
#define  MM_VIA_WDM_WAVEOUT                 8
#define  MM_VIA_WDM_WAVEIN                  9
#define  MM_VIA_WDM_MIXER                   10
#define  MM_VIA_WDM_MPU401_MIDIOUT          11
#define  MM_VIA_WDM_MPU401_MIDIIN           12

/* MM_MICRONAS product IDs */
#define  MM_MICRONAS_SC4                    1
#define  MM_MICRONAS_CLP833                 2

/* MM_HP product IDs */
#define  MM_HP_WAVEOUT                      1
#define  MM_HP_WAVEIN                       2

/* MM_QUICKAUDIO product IDs */
#define  MM_QUICKAUDIO_MINIMIDI             1
#define  MM_QUICKAUDIO_MAXIMIDI             2

/* MM_ICCC product IDs */
#define  MM_ICCC_UNA3_WAVEIN                1
#define  MM_ICCC_UNA3_WAVEOUT               2
#define  MM_ICCC_UNA3_AUX                   3
#define  MM_ICCC_UNA3_MIXER                 4

/* MM_3COM product IDs */
#define  MM_3COM_CB_MIXER                   1
#define  MM_3COM_CB_WAVEIN                  2
#define  MM_3COM_CB_WAVEOUT                 3

/* MM_MINDMAKER product IDs */
#define  MM_MINDMAKER_GC_WAVEIN             1
#define  MM_MINDMAKER_GC_WAVEOUT            2
#define  MM_MINDMAKER_GC_MIXER              3

/* MM_TELEKOL product IDs */
#define  MM_TELEKOL_WAVEOUT                 1
#define  MM_TELEKOL_WAVEIN                  2

/* MM_ALGOVISION product IDs */
#define  MM_ALGOVISION_VB80WAVEOUT          1
#define  MM_ALGOVISION_VB80WAVEIN           2
#define  MM_ALGOVISION_VB80MIXER            3
#define  MM_ALGOVISION_VB80AUX              4
#define  MM_ALGOVISION_VB80AUX2             5

#endif  // !NOMMIDS

/* ------------------------------------------------------------------------------ */

/*              INFO LIST CHUNKS (from the Multimedia Programmer's Reference
                                        plus new ones)
*/
#define RIFFINFO_IARL      mmioFOURCC ('I', 'A', 'R', 'L')     /*Archival location  */
#define RIFFINFO_IART      mmioFOURCC ('I', 'A', 'R', 'T')     /*Artist  */
#define RIFFINFO_ICMS      mmioFOURCC ('I', 'C', 'M', 'S')     /*Commissioned  */
#define RIFFINFO_ICMT      mmioFOURCC ('I', 'C', 'M', 'T')     /*Comments  */
#define RIFFINFO_ICOP      mmioFOURCC ('I', 'C', 'O', 'P')     /*Copyright  */
#define RIFFINFO_ICRD      mmioFOURCC ('I', 'C', 'R', 'D')     /*Creation date of subject  */
#define RIFFINFO_ICRP      mmioFOURCC ('I', 'C', 'R', 'P')     /*Cropped  */
#define RIFFINFO_IDIM      mmioFOURCC ('I', 'D', 'I', 'M')     /*Dimensions  */
#define RIFFINFO_IDPI      mmioFOURCC ('I', 'D', 'P', 'I')     /*Dots per inch  */
#define RIFFINFO_IENG      mmioFOURCC ('I', 'E', 'N', 'G')     /*Engineer  */
#define RIFFINFO_IGNR      mmioFOURCC ('I', 'G', 'N', 'R')     /*Genre  */
#define RIFFINFO_IKEY      mmioFOURCC ('I', 'K', 'E', 'Y')     /*Keywords  */
#define RIFFINFO_ILGT      mmioFOURCC ('I', 'L', 'G', 'T')     /*Lightness settings  */
#define RIFFINFO_IMED      mmioFOURCC ('I', 'M', 'E', 'D')     /*Medium  */
#define RIFFINFO_INAM      mmioFOURCC ('I', 'N', 'A', 'M')     /*Name of subject  */
#define RIFFINFO_IPLT      mmioFOURCC ('I', 'P', 'L', 'T')     /*Palette Settings. No. of colors requested.   */
#define RIFFINFO_IPRD      mmioFOURCC ('I', 'P', 'R', 'D')     /*Product  */
#define RIFFINFO_ISBJ      mmioFOURCC ('I', 'S', 'B', 'J')     /*Subject description  */
#define RIFFINFO_ISFT      mmioFOURCC ('I', 'S', 'F', 'T')     /*Software. Name of package used to create file.  */
#define RIFFINFO_ISHP      mmioFOURCC ('I', 'S', 'H', 'P')     /*Sharpness.  */
#define RIFFINFO_ISRC      mmioFOURCC ('I', 'S', 'R', 'C')     /*Source.   */
#define RIFFINFO_ISRF      mmioFOURCC ('I', 'S', 'R', 'F')     /*Source Form. ie slide, paper  */
#define RIFFINFO_ITCH      mmioFOURCC ('I', 'T', 'C', 'H')     /*Technician who digitized the subject.  */

/* New INFO Chunks as of August 30, 1993: */
#define RIFFINFO_ISMP      mmioFOURCC ('I', 'S', 'M', 'P')     /*SMPTE time code  */
/* ISMP: SMPTE time code of digitization start point expressed as a NULL terminated
                text string "HH:MM:SS:FF". If performing MCI capture in AVICAP, this
                chunk will be automatically set based on the MCI start time.
*/
#define RIFFINFO_IDIT      mmioFOURCC ('I', 'D', 'I', 'T')     /*Digitization Time  */
/* IDIT: "Digitization Time" Specifies the time and date that the digitization commenced.
                The digitization time is contained in an ASCII string which
                contains exactly 26 characters and is in the format
                "Wed Jan 02 02:03:55 1990\n\0".
                The ctime(), asctime(), functions can be used to create strings
                in this format. This chunk is automatically added to the capture
                file based on the current system time at the moment capture is initiated.
*/

#define RIFFINFO_ITRK      mmioFOURCC ('I', 'T', 'R', 'K')     /*ASCIIZ representation of the 1-based track number of the content.  */
#define RIFFINFO_ITOC      mmioFOURCC ('I', 'T', 'O', 'C')     /*A dump of the table of contents from the CD the content originated from.  */

/*Template line for new additions*/
/*#define RIFFINFO_I      mmioFOURCC ('I', '', '', '')        */

/* ------------------------------------------------------------------------------ */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

#ifndef NONEWWAVE

/* WAVE form wFormatTag IDs */
#define  WAVE_FORMAT_UNKNOWN                    0x0000 /* Microsoft Corporation */
#define  WAVE_FORMAT_ADPCM                      0x0002 /* Microsoft Corporation */
#define  WAVE_FORMAT_IEEE_FLOAT                 0x0003 /* Microsoft Corporation */
#define  WAVE_FORMAT_VSELP                      0x0004 /* Compaq Computer Corp. */
#define  WAVE_FORMAT_IBM_CVSD                   0x0005 /* IBM Corporation */
#define  WAVE_FORMAT_ALAW                       0x0006 /* Microsoft Corporation */
#define  WAVE_FORMAT_MULAW                      0x0007 /* Microsoft Corporation */
#define  WAVE_FORMAT_DTS                        0x0008 /* Microsoft Corporation */
#define  WAVE_FORMAT_DRM                        0x0009 /* Microsoft Corporation */
#define  WAVE_FORMAT_WMAVOICE9                  0x000A /* Microsoft Corporation */
#define  WAVE_FORMAT_WMAVOICE10                 0x000B /* Microsoft Corporation */
#define  WAVE_FORMAT_OKI_ADPCM                  0x0010 /* OKI */
#define  WAVE_FORMAT_DVI_ADPCM                  0x0011 /* Intel Corporation */
#define  WAVE_FORMAT_IMA_ADPCM                  (WAVE_FORMAT_DVI_ADPCM) /*  Intel Corporation */
#define  WAVE_FORMAT_MEDIASPACE_ADPCM           0x0012 /* Videologic */
#define  WAVE_FORMAT_SIERRA_ADPCM               0x0013 /* Sierra Semiconductor Corp */
#define  WAVE_FORMAT_G723_ADPCM                 0x0014 /* Antex Electronics Corporation */
#define  WAVE_FORMAT_DIGISTD                    0x0015 /* DSP Solutions, Inc. */
#define  WAVE_FORMAT_DIGIFIX                    0x0016 /* DSP Solutions, Inc. */
#define  WAVE_FORMAT_DIALOGIC_OKI_ADPCM         0x0017 /* Dialogic Corporation */
#define  WAVE_FORMAT_MEDIAVISION_ADPCM          0x0018 /* Media Vision, Inc. */
#define  WAVE_FORMAT_CU_CODEC                   0x0019 /* Hewlett-Packard Company */
#define  WAVE_FORMAT_HP_DYN_VOICE               0x001A /* Hewlett-Packard Company */
#define  WAVE_FORMAT_YAMAHA_ADPCM               0x0020 /* Yamaha Corporation of America */
#define  WAVE_FORMAT_SONARC                     0x0021 /* Speech Compression */
#define  WAVE_FORMAT_DSPGROUP_TRUESPEECH        0x0022 /* DSP Group, Inc */
#define  WAVE_FORMAT_ECHOSC1                    0x0023 /* Echo Speech Corporation */
#define  WAVE_FORMAT_AUDIOFILE_AF36             0x0024 /* Virtual Music, Inc. */
#define  WAVE_FORMAT_APTX                       0x0025 /* Audio Processing Technology */
#define  WAVE_FORMAT_AUDIOFILE_AF10             0x0026 /* Virtual Music, Inc. */
#define  WAVE_FORMAT_PROSODY_1612               0x0027 /* Aculab plc */
#define  WAVE_FORMAT_LRC                        0x0028 /* Merging Technologies S.A. */
#define  WAVE_FORMAT_DOLBY_AC2                  0x0030 /* Dolby Laboratories */
#define  WAVE_FORMAT_GSM610                     0x0031 /* Microsoft Corporation */
#define  WAVE_FORMAT_MSNAUDIO                   0x0032 /* Microsoft Corporation */
#define  WAVE_FORMAT_ANTEX_ADPCME               0x0033 /* Antex Electronics Corporation */
#define  WAVE_FORMAT_CONTROL_RES_VQLPC          0x0034 /* Control Resources Limited */
#define  WAVE_FORMAT_DIGIREAL                   0x0035 /* DSP Solutions, Inc. */
#define  WAVE_FORMAT_DIGIADPCM                  0x0036 /* DSP Solutions, Inc. */
#define  WAVE_FORMAT_CONTROL_RES_CR10           0x0037 /* Control Resources Limited */
#define  WAVE_FORMAT_NMS_VBXADPCM               0x0038 /* Natural MicroSystems */
#define  WAVE_FORMAT_CS_IMAADPCM                0x0039 /* Crystal Semiconductor IMA ADPCM */
#define  WAVE_FORMAT_ECHOSC3                    0x003A /* Echo Speech Corporation */
#define  WAVE_FORMAT_ROCKWELL_ADPCM             0x003B /* Rockwell International */
#define  WAVE_FORMAT_ROCKWELL_DIGITALK          0x003C /* Rockwell International */
#define  WAVE_FORMAT_XEBEC                      0x003D /* Xebec Multimedia Solutions Limited */
#define  WAVE_FORMAT_G721_ADPCM                 0x0040 /* Antex Electronics Corporation */
#define  WAVE_FORMAT_G728_CELP                  0x0041 /* Antex Electronics Corporation */
#define  WAVE_FORMAT_MSG723                     0x0042 /* Microsoft Corporation */
#define  WAVE_FORMAT_INTEL_G723_1               0x0043 /* Intel Corp. */
#define  WAVE_FORMAT_INTEL_G729                 0x0044 /* Intel Corp. */
#define  WAVE_FORMAT_SHARP_G726                 0x0045 /* Sharp */
#define  WAVE_FORMAT_MPEG                       0x0050 /* Microsoft Corporation */
#define  WAVE_FORMAT_RT24                       0x0052 /* InSoft, Inc. */
#define  WAVE_FORMAT_PAC                        0x0053 /* InSoft, Inc. */
#define  WAVE_FORMAT_MPEGLAYER3                 0x0055 /* ISO/MPEG Layer3 Format Tag */
#define  WAVE_FORMAT_LUCENT_G723                0x0059 /* Lucent Technologies */
#define  WAVE_FORMAT_CIRRUS                     0x0060 /* Cirrus Logic */
#define  WAVE_FORMAT_ESPCM                      0x0061 /* ESS Technology */
#define  WAVE_FORMAT_VOXWARE                    0x0062 /* Voxware Inc */
#define  WAVE_FORMAT_CANOPUS_ATRAC              0x0063 /* Canopus, co., Ltd. */
#define  WAVE_FORMAT_G726_ADPCM                 0x0064 /* APICOM */
#define  WAVE_FORMAT_G722_ADPCM                 0x0065 /* APICOM */
#define  WAVE_FORMAT_DSAT                       0x0066 /* Microsoft Corporation */
#define  WAVE_FORMAT_DSAT_DISPLAY               0x0067 /* Microsoft Corporation */
#define  WAVE_FORMAT_VOXWARE_BYTE_ALIGNED       0x0069 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_AC8                0x0070 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_AC10               0x0071 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_AC16               0x0072 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_AC20               0x0073 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_RT24               0x0074 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_RT29               0x0075 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_RT29HW             0x0076 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_VR12               0x0077 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_VR18               0x0078 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_TQ40               0x0079 /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_SC3                0x007A /* Voxware Inc */
#define  WAVE_FORMAT_VOXWARE_SC3_1              0x007B /* Voxware Inc */
#define  WAVE_FORMAT_SOFTSOUND                  0x0080 /* Softsound, Ltd. */
#define  WAVE_FORMAT_VOXWARE_TQ60               0x0081 /* Voxware Inc */
#define  WAVE_FORMAT_MSRT24                     0x0082 /* Microsoft Corporation */
#define  WAVE_FORMAT_G729A                      0x0083 /* AT&T Labs, Inc. */
#define  WAVE_FORMAT_MVI_MVI2                   0x0084 /* Motion Pixels */
#define  WAVE_FORMAT_DF_G726                    0x0085 /* DataFusion Systems (Pty) (Ltd) */
#define  WAVE_FORMAT_DF_GSM610                  0x0086 /* DataFusion Systems (Pty) (Ltd) */
#define  WAVE_FORMAT_ISIAUDIO                   0x0088 /* Iterated Systems, Inc. */
#define  WAVE_FORMAT_ONLIVE                     0x0089 /* OnLive! Technologies, Inc. */
#define  WAVE_FORMAT_MULTITUDE_FT_SX20          0x008A /* Multitude Inc. */
#define  WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM     0x008B /* Infocom */
#define  WAVE_FORMAT_CONVEDIA_G729              0x008C /* Convedia Corp. */
#define  WAVE_FORMAT_CONGRUENCY                 0x008D /* Congruency Inc. */
#define  WAVE_FORMAT_SBC24                      0x0091 /* Siemens Business Communications Sys */
#define  WAVE_FORMAT_DOLBY_AC3_SPDIF            0x0092 /* Sonic Foundry */
#define  WAVE_FORMAT_MEDIASONIC_G723            0x0093 /* MediaSonic */
#define  WAVE_FORMAT_PROSODY_8KBPS              0x0094 /* Aculab plc */
#define  WAVE_FORMAT_ZYXEL_ADPCM                0x0097 /* ZyXEL Communications, Inc. */
#define  WAVE_FORMAT_PHILIPS_LPCBB              0x0098 /* Philips Speech Processing */
#define  WAVE_FORMAT_PACKED                     0x0099 /* Studer Professional Audio AG */
#define  WAVE_FORMAT_MALDEN_PHONYTALK           0x00A0 /* Malden Electronics Ltd. */
#define  WAVE_FORMAT_RACAL_RECORDER_GSM         0x00A1 /* Racal recorders */
#define  WAVE_FORMAT_RACAL_RECORDER_G720_A      0x00A2 /* Racal recorders */
#define  WAVE_FORMAT_RACAL_RECORDER_G723_1      0x00A3 /* Racal recorders */
#define  WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP 0x00A4 /* Racal recorders */
#define  WAVE_FORMAT_NEC_AAC                    0x00B0 /* NEC Corp. */
#define  WAVE_FORMAT_RAW_AAC1                   0x00FF /* For Raw AAC, with format block AudioSpecificConfig() (as defined by MPEG-4), that follows WAVEFORMATEX */
#define  WAVE_FORMAT_RHETOREX_ADPCM             0x0100 /* Rhetorex Inc. */
#define  WAVE_FORMAT_IRAT                       0x0101 /* BeCubed Software Inc. */
#define  WAVE_FORMAT_VIVO_G723                  0x0111 /* Vivo Software */
#define  WAVE_FORMAT_VIVO_SIREN                 0x0112 /* Vivo Software */
#define  WAVE_FORMAT_PHILIPS_CELP               0x0120 /* Philips Speech Processing */
#define  WAVE_FORMAT_PHILIPS_GRUNDIG            0x0121 /* Philips Speech Processing */
#define  WAVE_FORMAT_DIGITAL_G723               0x0123 /* Digital Equipment Corporation */
#define  WAVE_FORMAT_SANYO_LD_ADPCM             0x0125 /* Sanyo Electric Co., Ltd. */
#define  WAVE_FORMAT_SIPROLAB_ACEPLNET          0x0130 /* Sipro Lab Telecom Inc. */
#define  WAVE_FORMAT_SIPROLAB_ACELP4800         0x0131 /* Sipro Lab Telecom Inc. */
#define  WAVE_FORMAT_SIPROLAB_ACELP8V3          0x0132 /* Sipro Lab Telecom Inc. */
#define  WAVE_FORMAT_SIPROLAB_G729              0x0133 /* Sipro Lab Telecom Inc. */
#define  WAVE_FORMAT_SIPROLAB_G729A             0x0134 /* Sipro Lab Telecom Inc. */
#define  WAVE_FORMAT_SIPROLAB_KELVIN            0x0135 /* Sipro Lab Telecom Inc. */
#define  WAVE_FORMAT_VOICEAGE_AMR               0x0136 /* VoiceAge Corp. */
#define  WAVE_FORMAT_G726ADPCM                  0x0140 /* Dictaphone Corporation */
#define  WAVE_FORMAT_DICTAPHONE_CELP68          0x0141 /* Dictaphone Corporation */
#define  WAVE_FORMAT_DICTAPHONE_CELP54          0x0142 /* Dictaphone Corporation */
#define  WAVE_FORMAT_QUALCOMM_PUREVOICE         0x0150 /* Qualcomm, Inc. */
#define  WAVE_FORMAT_QUALCOMM_HALFRATE          0x0151 /* Qualcomm, Inc. */
#define  WAVE_FORMAT_TUBGSM                     0x0155 /* Ring Zero Systems, Inc. */
#define  WAVE_FORMAT_MSAUDIO1                   0x0160 /* Microsoft Corporation */
#define  WAVE_FORMAT_WMAUDIO2                   0x0161 /* Microsoft Corporation */
#define  WAVE_FORMAT_WMAUDIO3                   0x0162 /* Microsoft Corporation */
#define  WAVE_FORMAT_WMAUDIO_LOSSLESS           0x0163 /* Microsoft Corporation */
#define  WAVE_FORMAT_WMASPDIF                   0x0164 /* Microsoft Corporation */
#define  WAVE_FORMAT_UNISYS_NAP_ADPCM           0x0170 /* Unisys Corp. */
#define  WAVE_FORMAT_UNISYS_NAP_ULAW            0x0171 /* Unisys Corp. */
#define  WAVE_FORMAT_UNISYS_NAP_ALAW            0x0172 /* Unisys Corp. */
#define  WAVE_FORMAT_UNISYS_NAP_16K             0x0173 /* Unisys Corp. */
#define  WAVE_FORMAT_SYCOM_ACM_SYC008           0x0174 /* SyCom Technologies */
#define  WAVE_FORMAT_SYCOM_ACM_SYC701_G726L     0x0175 /* SyCom Technologies */
#define  WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54    0x0176 /* SyCom Technologies */
#define  WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68    0x0177 /* SyCom Technologies */
#define  WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM  0x0178 /* Knowledge Adventure, Inc. */
#define  WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC   0x0180 /* Fraunhofer IIS */
#define  WAVE_FORMAT_DTS_DS                     0x0190 /* Digital Theatre Systems, Inc. */
#define  WAVE_FORMAT_CREATIVE_ADPCM             0x0200 /* Creative Labs, Inc */
#define  WAVE_FORMAT_CREATIVE_FASTSPEECH8       0x0202 /* Creative Labs, Inc */
#define  WAVE_FORMAT_CREATIVE_FASTSPEECH10      0x0203 /* Creative Labs, Inc */
#define  WAVE_FORMAT_UHER_ADPCM                 0x0210 /* UHER informatic GmbH */
#define  WAVE_FORMAT_ULEAD_DV_AUDIO             0x0215 /* Ulead Systems, Inc. */
#define  WAVE_FORMAT_ULEAD_DV_AUDIO_1           0x0216 /* Ulead Systems, Inc. */
#define  WAVE_FORMAT_QUARTERDECK                0x0220 /* Quarterdeck Corporation */
#define  WAVE_FORMAT_ILINK_VC                   0x0230 /* I-link Worldwide */
#define  WAVE_FORMAT_RAW_SPORT                  0x0240 /* Aureal Semiconductor */
#define  WAVE_FORMAT_ESST_AC3                   0x0241 /* ESS Technology, Inc. */
#define  WAVE_FORMAT_GENERIC_PASSTHRU           0x0249
#define  WAVE_FORMAT_IPI_HSX                    0x0250 /* Interactive Products, Inc. */
#define  WAVE_FORMAT_IPI_RPELP                  0x0251 /* Interactive Products, Inc. */
#define  WAVE_FORMAT_CS2                        0x0260 /* Consistent Software */
#define  WAVE_FORMAT_SONY_SCX                   0x0270 /* Sony Corp. */
#define  WAVE_FORMAT_SONY_SCY                   0x0271 /* Sony Corp. */
#define  WAVE_FORMAT_SONY_ATRAC3                0x0272 /* Sony Corp. */
#define  WAVE_FORMAT_SONY_SPC                   0x0273 /* Sony Corp. */
#define  WAVE_FORMAT_TELUM_AUDIO                0x0280 /* Telum Inc. */
#define  WAVE_FORMAT_TELUM_IA_AUDIO             0x0281 /* Telum Inc. */
#define  WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM 0x0285 /* Norcom Electronics Corp. */
#define  WAVE_FORMAT_FM_TOWNS_SND               0x0300 /* Fujitsu Corp. */
#define  WAVE_FORMAT_MICRONAS                   0x0350 /* Micronas Semiconductors, Inc. */
#define  WAVE_FORMAT_MICRONAS_CELP833           0x0351 /* Micronas Semiconductors, Inc. */
#define  WAVE_FORMAT_BTV_DIGITAL                0x0400 /* Brooktree Corporation */
#define  WAVE_FORMAT_INTEL_MUSIC_CODER          0x0401 /* Intel Corp. */
#define  WAVE_FORMAT_INDEO_AUDIO                0x0402 /* Ligos */
#define  WAVE_FORMAT_QDESIGN_MUSIC              0x0450 /* QDesign Corporation */
#define  WAVE_FORMAT_ON2_VP7_AUDIO              0x0500 /* On2 Technologies */
#define  WAVE_FORMAT_ON2_VP6_AUDIO              0x0501 /* On2 Technologies */
#define  WAVE_FORMAT_VME_VMPCM                  0x0680 /* AT&T Labs, Inc. */
#define  WAVE_FORMAT_TPC                        0x0681 /* AT&T Labs, Inc. */
#define  WAVE_FORMAT_LIGHTWAVE_LOSSLESS         0x08AE /* Clearjump */
#define  WAVE_FORMAT_OLIGSM                     0x1000 /* Ing C. Olivetti & C., S.p.A. */
#define  WAVE_FORMAT_OLIADPCM                   0x1001 /* Ing C. Olivetti & C., S.p.A. */
#define  WAVE_FORMAT_OLICELP                    0x1002 /* Ing C. Olivetti & C., S.p.A. */
#define  WAVE_FORMAT_OLISBC                     0x1003 /* Ing C. Olivetti & C., S.p.A. */
#define  WAVE_FORMAT_OLIOPR                     0x1004 /* Ing C. Olivetti & C., S.p.A. */
#define  WAVE_FORMAT_LH_CODEC                   0x1100 /* Lernout & Hauspie */
#define  WAVE_FORMAT_LH_CODEC_CELP              0x1101 /* Lernout & Hauspie */
#define  WAVE_FORMAT_LH_CODEC_SBC8              0x1102 /* Lernout & Hauspie */
#define  WAVE_FORMAT_LH_CODEC_SBC12             0x1103 /* Lernout & Hauspie */
#define  WAVE_FORMAT_LH_CODEC_SBC16             0x1104 /* Lernout & Hauspie */
#define  WAVE_FORMAT_NORRIS                     0x1400 /* Norris Communications, Inc. */
#define  WAVE_FORMAT_ISIAUDIO_2                 0x1401 /* ISIAudio */
#define  WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS    0x1500 /* AT&T Labs, Inc. */
#define  WAVE_FORMAT_MPEG_ADTS_AAC              0x1600 /* Microsoft Corporation */
#define  WAVE_FORMAT_MPEG_RAW_AAC               0x1601 /* Microsoft Corporation */
#define  WAVE_FORMAT_MPEG_LOAS                  0x1602 /* Microsoft Corporation (MPEG-4 Audio Transport Streams (LOAS/LATM) */
#define  WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC        0x1608 /* Microsoft Corporation */
#define  WAVE_FORMAT_NOKIA_MPEG_RAW_AAC         0x1609 /* Microsoft Corporation */
#define  WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC     0x160A /* Microsoft Corporation */
#define  WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC      0x160B /* Microsoft Corporation */
#define  WAVE_FORMAT_MPEG_HEAAC                 0x1610 /* Microsoft Corporation (MPEG-2 AAC or MPEG-4 HE-AAC v1/v2 streams with any payload (ADTS, ADIF, LOAS/LATM, RAW). Format block includes MP4 AudioSpecificConfig() -- see HEAACWAVEFORMAT below */
#define  WAVE_FORMAT_VOXWARE_RT24_SPEECH        0x181C /* Voxware Inc. */
#define  WAVE_FORMAT_SONICFOUNDRY_LOSSLESS      0x1971 /* Sonic Foundry */
#define  WAVE_FORMAT_INNINGS_TELECOM_ADPCM      0x1979 /* Innings Telecom Inc. */
#define  WAVE_FORMAT_LUCENT_SX8300P             0x1C07 /* Lucent Technologies */
#define  WAVE_FORMAT_LUCENT_SX5363S             0x1C0C /* Lucent Technologies */
#define  WAVE_FORMAT_CUSEEME                    0x1F03 /* CUSeeMe */
#define  WAVE_FORMAT_NTCSOFT_ALF2CM_ACM         0x1FC4 /* NTCSoft */
#define  WAVE_FORMAT_DVM                        0x2000 /* FAST Multimedia AG */
#define  WAVE_FORMAT_DTS2                       0x2001 
#define  WAVE_FORMAT_MAKEAVIS                   0x3313 
#define  WAVE_FORMAT_DIVIO_MPEG4_AAC            0x4143 /* Divio, Inc. */
#define  WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE   0x4201 /* Nokia */
#define  WAVE_FORMAT_DIVIO_G726                 0x4243 /* Divio, Inc. */
#define  WAVE_FORMAT_LEAD_SPEECH                0x434C /* LEAD Technologies */
#define  WAVE_FORMAT_LEAD_VORBIS                0x564C /* LEAD Technologies */
#define  WAVE_FORMAT_WAVPACK_AUDIO              0x5756 /* xiph.org */
#define  WAVE_FORMAT_ALAC                       0x6C61 /* Apple Lossless */
#define  WAVE_FORMAT_OGG_VORBIS_MODE_1          0x674F /* Ogg Vorbis */
#define  WAVE_FORMAT_OGG_VORBIS_MODE_2          0x6750 /* Ogg Vorbis */
#define  WAVE_FORMAT_OGG_VORBIS_MODE_3          0x6751 /* Ogg Vorbis */
#define  WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS     0x676F /* Ogg Vorbis */
#define  WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS     0x6770 /* Ogg Vorbis */
#define  WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS     0x6771 /* Ogg Vorbis */
#define  WAVE_FORMAT_3COM_NBX                   0x7000 /* 3COM Corp. */
#define  WAVE_FORMAT_OPUS                       0x704F /* Opus */
#define  WAVE_FORMAT_FAAD_AAC                   0x706D
#define  WAVE_FORMAT_AMR_NB                     0x7361 /* AMR Narrowband */
#define  WAVE_FORMAT_AMR_WB                     0x7362 /* AMR Wideband */
#define  WAVE_FORMAT_AMR_WP                     0x7363 /* AMR Wideband Plus */
#define  WAVE_FORMAT_GSM_AMR_CBR                0x7A21 /* GSMA/3GPP */
#define  WAVE_FORMAT_GSM_AMR_VBR_SID            0x7A22 /* GSMA/3GPP */
#define  WAVE_FORMAT_COMVERSE_INFOSYS_G723_1    0xA100 /* Comverse Infosys */
#define  WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC    0xA101 /* Comverse Infosys */
#define  WAVE_FORMAT_COMVERSE_INFOSYS_SBC       0xA102 /* Comverse Infosys */
#define  WAVE_FORMAT_SYMBOL_G729_A              0xA103 /* Symbol Technologies */
#define  WAVE_FORMAT_VOICEAGE_AMR_WB            0xA104 /* VoiceAge Corp. */
#define  WAVE_FORMAT_INGENIENT_G726             0xA105 /* Ingenient Technologies, Inc. */
#define  WAVE_FORMAT_MPEG4_AAC                  0xA106 /* ISO/MPEG-4 */
#define  WAVE_FORMAT_ENCORE_G726                0xA107 /* Encore Software */
#define  WAVE_FORMAT_ZOLL_ASAO                  0xA108 /* ZOLL Medical Corp. */
#define  WAVE_FORMAT_SPEEX_VOICE                0xA109 /* xiph.org */
#define  WAVE_FORMAT_VIANIX_MASC                0xA10A /* Vianix LLC */
#define  WAVE_FORMAT_WM9_SPECTRUM_ANALYZER      0xA10B /* Microsoft */
#define  WAVE_FORMAT_WMF_SPECTRUM_ANAYZER       0xA10C /* Microsoft */
#define  WAVE_FORMAT_GSM_610                    0xA10D
#define  WAVE_FORMAT_GSM_620                    0xA10E 
#define  WAVE_FORMAT_GSM_660                    0xA10F 
#define  WAVE_FORMAT_GSM_690                    0xA110 
#define  WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB  0xA111 
#define  WAVE_FORMAT_POLYCOM_G722               0xA112 /* Polycom */
#define  WAVE_FORMAT_POLYCOM_G728               0xA113 /* Polycom */
#define  WAVE_FORMAT_POLYCOM_G729_A             0xA114 /* Polycom */
#define  WAVE_FORMAT_POLYCOM_SIREN              0xA115 /* Polycom */
#define  WAVE_FORMAT_GLOBAL_IP_ILBC             0xA116 /* Global IP */
#define  WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO 0xA117 /* RadioTime */
#define  WAVE_FORMAT_NICE_ACA                   0xA118 /* Nice Systems */
#define  WAVE_FORMAT_NICE_ADPCM                 0xA119 /* Nice Systems */
#define  WAVE_FORMAT_VOCORD_G721                0xA11A /* Vocord Telecom */
#define  WAVE_FORMAT_VOCORD_G726                0xA11B /* Vocord Telecom */
#define  WAVE_FORMAT_VOCORD_G722_1              0xA11C /* Vocord Telecom */
#define  WAVE_FORMAT_VOCORD_G728                0xA11D /* Vocord Telecom */
#define  WAVE_FORMAT_VOCORD_G729                0xA11E /* Vocord Telecom */
#define  WAVE_FORMAT_VOCORD_G729_A              0xA11F /* Vocord Telecom */
#define  WAVE_FORMAT_VOCORD_G723_1              0xA120 /* Vocord Telecom */
#define  WAVE_FORMAT_VOCORD_LBC                 0xA121 /* Vocord Telecom */
#define  WAVE_FORMAT_NICE_G728                  0xA122 /* Nice Systems */
#define  WAVE_FORMAT_FRACE_TELECOM_G729         0xA123 /* France Telecom */
#define  WAVE_FORMAT_CODIAN                     0xA124 /* CODIAN */
#define  WAVE_FORMAT_DOLBY_AC4                  0xAC40 /* Dolby AC-4 */
#define  WAVE_FORMAT_FLAC                       0xF1AC /* flac.sourceforge.net */

#if !defined(WAVE_FORMAT_EXTENSIBLE)
#define  WAVE_FORMAT_EXTENSIBLE                 0xFFFE /* Microsoft */
#endif // !defined(WAVE_FORMAT_EXTENSIBLE)

//
//  New wave format development should be based on the
//  WAVEFORMATEXTENSIBLE structure. WAVEFORMATEXTENSIBLE allows you to
//  avoid having to register a new format tag with Microsoft. However, if
//  you must still define a new format tag, the WAVE_FORMAT_DEVELOPMENT
//  format tag can be used during the development phase of a new wave
//  format.  Before shipping, you MUST acquire an official format tag from
//  Microsoft.
//
#define WAVE_FORMAT_DEVELOPMENT         (0xFFFF)

#endif /* NONEWWAVE */

#ifndef WAVE_FORMAT_PCM

/* general waveform format structure (information common to all formats) */
typedef struct waveformat_tag {
    WORD    wFormatTag;        /* format type */
    WORD    nChannels;         /* number of channels (i.e. mono, stereo...) */
    DWORD   nSamplesPerSec;    /* sample rate */
    DWORD   nAvgBytesPerSec;   /* for buffer estimation */
    WORD    nBlockAlign;       /* block size of data */
} WAVEFORMAT;
typedef WAVEFORMAT       *PWAVEFORMAT;
typedef WAVEFORMAT NEAR *NPWAVEFORMAT;
typedef WAVEFORMAT FAR  *LPWAVEFORMAT;

/* specific waveform format structure for PCM data */
typedef struct pcmwaveformat_tag {
    WAVEFORMAT  wf;
    WORD        wBitsPerSample;
} PCMWAVEFORMAT;
typedef PCMWAVEFORMAT       *PPCMWAVEFORMAT;
typedef PCMWAVEFORMAT NEAR *NPPCMWAVEFORMAT;
typedef PCMWAVEFORMAT FAR  *LPPCMWAVEFORMAT;

#endif /* WAVE_FORMAT_PCM */

#ifndef WAVE_FORMAT_PCM
#define WAVE_FORMAT_PCM         1
#endif

/* general extended waveform format structure
   Use this for all NON PCM formats
   (information common to all formats)
*/
#ifndef _WAVEFORMATEX_
#define _WAVEFORMATEX_
typedef struct tWAVEFORMATEX
{
    WORD    wFormatTag;        /* format type */
    WORD    nChannels;         /* number of channels (i.e. mono, stereo...) */
    DWORD   nSamplesPerSec;    /* sample rate */
    DWORD   nAvgBytesPerSec;   /* for buffer estimation */
    WORD    nBlockAlign;       /* block size of data */
    WORD    wBitsPerSample;    /* Number of bits per sample of mono data */
    WORD    cbSize;            /* The count in bytes of the size of
                                    extra information (after cbSize) */

} WAVEFORMATEX;
typedef WAVEFORMATEX       *PWAVEFORMATEX;
typedef WAVEFORMATEX NEAR *NPWAVEFORMATEX;
typedef WAVEFORMATEX FAR  *LPWAVEFORMATEX;
#endif /* _WAVEFORMATEX_ */

#ifdef GUID_DEFINED

#if !defined(_NTRTL_)
#if !defined( DEFINE_GUIDEX )
    #ifndef DEFINE_GUIDEX
        #define DEFINE_GUIDEX(name) EXTERN_C const CDECL GUID name
    #endif // !defined(DEFINE_GUIDEX)

    #ifndef STATICGUIDOF
        #define STATICGUIDOF(guid) STATIC_##guid
    #endif // !defined(STATICGUIDOF)
#endif // !defined ( DEFINE_GUIDEX )
#endif // !defined(_NTRTL_)

#if !defined( DEFINE_GUIDSTRUCT )
#if defined(__cplusplus) && _MSC_VER >= 1100
#define DEFINE_GUIDSTRUCT(g, n) struct __declspec(uuid(g)) n
#define DEFINE_GUIDNAMED(n) __uuidof(struct n)
#else // !defined(__cplusplus)
#define DEFINE_GUIDSTRUCT(g, n) DEFINE_GUIDEX(n)
#define DEFINE_GUIDNAMED(n) n
#endif // !defined(__cplusplus)
#endif

#if !defined( DEFINE_WAVEFORMATEX_GUID )
#define DEFINE_WAVEFORMATEX_GUID(x) (USHORT)(x), 0x0000, 0x0010, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71
#endif

#if !defined( STATIC_KSDATAFORMAT_SUBTYPE_PCM )
#define STATIC_KSDATAFORMAT_SUBTYPE_PCM\
    DEFINE_WAVEFORMATEX_GUID(WAVE_FORMAT_PCM)
DEFINE_GUIDSTRUCT("00000001-0000-0010-8000-00aa00389b71", KSDATAFORMAT_SUBTYPE_PCM);
#define KSDATAFORMAT_SUBTYPE_PCM DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_PCM)
#endif

#if !defined(RC_INVOKED) /* RC complains about long symbols in #ifs */
#if !defined( STATIC_KSDATAFORMAT_SUBTYPE_IEEE_FLOAT )
#define STATIC_KSDATAFORMAT_SUBTYPE_IEEE_FLOAT\
    DEFINE_WAVEFORMATEX_GUID(WAVE_FORMAT_IEEE_FLOAT)
DEFINE_GUIDSTRUCT("00000003-0000-0010-8000-00aa00389b71", KSDATAFORMAT_SUBTYPE_IEEE_FLOAT);
#define KSDATAFORMAT_SUBTYPE_IEEE_FLOAT DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_IEEE_FLOAT)
#endif

#if !defined( STATIC_KSDATAFORMAT_SUBTYPE_WAVEFORMATEX )
#define STATIC_KSDATAFORMAT_SUBTYPE_WAVEFORMATEX\
    0x00000000L, 0x0000, 0x0010, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71
DEFINE_GUIDSTRUCT("00000000-0000-0010-8000-00aa00389b71", KSDATAFORMAT_SUBTYPE_WAVEFORMATEX);
#define KSDATAFORMAT_SUBTYPE_WAVEFORMATEX DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_WAVEFORMATEX)
#endif
#endif

#if !defined( INIT_WAVEFORMATEX_GUID )
#define INIT_WAVEFORMATEX_GUID(Guid, x)\
{\
    *(Guid) = KSDATAFORMAT_SUBTYPE_WAVEFORMATEX;\
    (Guid)->Data1 = (USHORT)(x);\
}
#endif

#if !defined( EXTRACT_WAVEFORMATEX_ID )
#define EXTRACT_WAVEFORMATEX_ID(Guid)\
    (USHORT)((Guid)->Data1)
#endif

#if !defined( IS_VALID_WAVEFORMATEX_GUID )
#define IS_VALID_WAVEFORMATEX_GUID(Guid)\
    (!memcmp(((PUSHORT)&KSDATAFORMAT_SUBTYPE_WAVEFORMATEX) + 1, ((PUSHORT)(Guid)) + 1, sizeof(GUID) - sizeof(USHORT)))
#endif

//
//  New wave format development should be based on the
//  WAVEFORMATEXTENSIBLE structure. WAVEFORMATEXTENSIBLE allows you to
//  avoid having to register a new format tag with Microsoft. Simply
//  define a new GUID value for the WAVEFORMATEXTENSIBLE.SubFormat field
//  and use WAVE_FORMAT_EXTENSIBLE in the
//  WAVEFORMATEXTENSIBLE.Format.wFormatTag field.
//
#ifndef _WAVEFORMATEXTENSIBLE_
#define _WAVEFORMATEXTENSIBLE_
typedef struct {
    WAVEFORMATEX    Format;
    union {
        WORD wValidBitsPerSample;       /* bits of precision  */
        WORD wSamplesPerBlock;          /* valid if wBitsPerSample==0 */
        WORD wReserved;                 /* If neither applies, set to zero. */
    } Samples;
    DWORD           dwChannelMask;      /* which channels are */
                                        /* present in stream  */
    GUID            SubFormat;
} WAVEFORMATEXTENSIBLE, *PWAVEFORMATEXTENSIBLE;
#endif // !_WAVEFORMATEXTENSIBLE_

//
//  Extended PCM waveform format structure based on WAVEFORMATEXTENSIBLE.
//  Use this for multiple channel and hi-resolution PCM data
//
typedef WAVEFORMATEXTENSIBLE    WAVEFORMATPCMEX; /* Format.cbSize = 22 */
typedef WAVEFORMATPCMEX       *PWAVEFORMATPCMEX;
typedef WAVEFORMATPCMEX NEAR *NPWAVEFORMATPCMEX;
typedef WAVEFORMATPCMEX FAR  *LPWAVEFORMATPCMEX;

//
//  Extended format structure using IEEE Float data and based
//  on WAVEFORMATEXTENSIBLE.  Use this for multiple channel
//  and hi-resolution PCM data in IEEE floating point format.
//
typedef WAVEFORMATEXTENSIBLE          WAVEFORMATIEEEFLOATEX; /* Format.cbSize = 22 */
typedef WAVEFORMATIEEEFLOATEX       *PWAVEFORMATIEEEFLOATEX;
typedef WAVEFORMATIEEEFLOATEX NEAR *NPWAVEFORMATIEEEFLOATEX;
typedef WAVEFORMATIEEEFLOATEX FAR  *LPWAVEFORMATIEEEFLOATEX;

#endif // GUID_DEFINED

#ifndef _SPEAKER_POSITIONS_
#define _SPEAKER_POSITIONS_
// Speaker Positions for dwChannelMask in WAVEFORMATEXTENSIBLE:
#define SPEAKER_FRONT_LEFT              0x1
#define SPEAKER_FRONT_RIGHT             0x2
#define SPEAKER_FRONT_CENTER            0x4
#define SPEAKER_LOW_FREQUENCY           0x8
#define SPEAKER_BACK_LEFT               0x10
#define SPEAKER_BACK_RIGHT              0x20
#define SPEAKER_FRONT_LEFT_OF_CENTER    0x40
#define SPEAKER_FRONT_RIGHT_OF_CENTER   0x80
#define SPEAKER_BACK_CENTER             0x100
#define SPEAKER_SIDE_LEFT               0x200
#define SPEAKER_SIDE_RIGHT              0x400
#define SPEAKER_TOP_CENTER              0x800
#define SPEAKER_TOP_FRONT_LEFT          0x1000
#define SPEAKER_TOP_FRONT_CENTER        0x2000
#define SPEAKER_TOP_FRONT_RIGHT         0x4000
#define SPEAKER_TOP_BACK_LEFT           0x8000
#define SPEAKER_TOP_BACK_CENTER         0x10000
#define SPEAKER_TOP_BACK_RIGHT          0x20000

// Bit mask locations reserved for future use
#define SPEAKER_RESERVED                0x7FFC0000

// Used to specify that any possible permutation of speaker configurations
#define SPEAKER_ALL                     0x80000000
#endif // _SPEAKER_POSITIONS_

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP| WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifndef NONEWWAVE

#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP| WINAPI_PARTITION_GAMES)

/* Define data for MS ADPCM */

typedef struct adpcmcoef_tag {
        short   iCoef1;
        short   iCoef2;
} ADPCMCOEFSET;
typedef ADPCMCOEFSET       *PADPCMCOEFSET;
typedef ADPCMCOEFSET NEAR *NPADPCMCOEFSET;
typedef ADPCMCOEFSET FAR  *LPADPCMCOEFSET;

/*
 *  this pragma disables the warning issued by the Microsoft C compiler
 *  when using a zero size array as place holder when compiling for
 *  C++ or with -W4.
 *
 */
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable:4200)
#endif

typedef struct adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wSamplesPerBlock;
        WORD            wNumCoef;
#if defined( _MSC_VER )        
        ADPCMCOEFSET    aCoef[];
#else
        ADPCMCOEFSET    aCoef[1];
#endif        
} ADPCMWAVEFORMAT;
typedef ADPCMWAVEFORMAT       *PADPCMWAVEFORMAT;
typedef ADPCMWAVEFORMAT NEAR *NPADPCMWAVEFORMAT;
typedef ADPCMWAVEFORMAT FAR  *LPADPCMWAVEFORMAT;

#ifdef _MSC_VER
#pragma warning(pop)
#endif

//
//  Microsoft's DRM structure definitions
//
typedef struct drmwaveformat_tag {
	WAVEFORMATEX    wfx;
	WORD            wReserved;
	ULONG           ulContentId;
	WAVEFORMATEX    wfxSecure;
} DRMWAVEFORMAT;
typedef DRMWAVEFORMAT       *PDRMWAVEFORMAT;
typedef DRMWAVEFORMAT NEAR *NPDRMWAVEFORMAT;
typedef DRMWAVEFORMAT FAR  *LPDRMWAVEFORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
//  Intel's DVI ADPCM structure definitions
//
//      for WAVE_FORMAT_DVI_ADPCM   (0x0011)
//
//

typedef struct dvi_adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wSamplesPerBlock;
} DVIADPCMWAVEFORMAT;
typedef DVIADPCMWAVEFORMAT       *PDVIADPCMWAVEFORMAT;
typedef DVIADPCMWAVEFORMAT NEAR *NPDVIADPCMWAVEFORMAT;
typedef DVIADPCMWAVEFORMAT FAR  *LPDVIADPCMWAVEFORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
//  IMA endorsed ADPCM structure definitions--note that this is exactly
//  the same format as Intel's DVI ADPCM.
//
//      for WAVE_FORMAT_IMA_ADPCM   (0x0011)
//
//

typedef struct ima_adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wSamplesPerBlock;
} IMAADPCMWAVEFORMAT;
typedef IMAADPCMWAVEFORMAT       *PIMAADPCMWAVEFORMAT;
typedef IMAADPCMWAVEFORMAT NEAR *NPIMAADPCMWAVEFORMAT;
typedef IMAADPCMWAVEFORMAT FAR  *LPIMAADPCMWAVEFORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
//VideoLogic's Media Space ADPCM Structure definitions
// for  WAVE_FORMAT_MEDIASPACE_ADPCM    (0x0012)
//
//
*/
typedef struct mediaspace_adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD    wRevision;
} MEDIASPACEADPCMWAVEFORMAT;
typedef MEDIASPACEADPCMWAVEFORMAT           *PMEDIASPACEADPCMWAVEFORMAT;
typedef MEDIASPACEADPCMWAVEFORMAT NEAR     *NPMEDIASPACEADPCMWAVEFORMAT;
typedef MEDIASPACEADPCMWAVEFORMAT FAR      *LPMEDIASPACEADPCMWAVEFORMAT;

//
//  Sierra Semiconductor
//
//      for WAVE_FORMAT_SIERRA_ADPCM   (0x0013)
//
//

typedef struct sierra_adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wRevision;
} SIERRAADPCMWAVEFORMAT;
typedef SIERRAADPCMWAVEFORMAT   *PSIERRAADPCMWAVEFORMAT;
typedef SIERRAADPCMWAVEFORMAT NEAR      *NPSIERRAADPCMWAVEFORMAT;
typedef SIERRAADPCMWAVEFORMAT FAR       *LPSIERRAADPCMWAVEFORMAT;

//
//  Antex Electronics  structure definitions
//
//      for WAVE_FORMAT_G723_ADPCM   (0x0014)
//
//

typedef struct g723_adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            cbExtraSize;
        WORD            nAuxBlockSize;
} G723_ADPCMWAVEFORMAT;
typedef G723_ADPCMWAVEFORMAT *PG723_ADPCMWAVEFORMAT;
typedef G723_ADPCMWAVEFORMAT NEAR *NPG723_ADPCMWAVEFORMAT;
typedef G723_ADPCMWAVEFORMAT FAR  *LPG723_ADPCMWAVEFORMAT;

//
//  DSP Solutions (formerly DIGISPEECH) structure definitions
//
//      for WAVE_FORMAT_DIGISTD   (0x0015)
//
//

typedef struct digistdwaveformat_tag {
        WAVEFORMATEX    wfx;
} DIGISTDWAVEFORMAT;
typedef DIGISTDWAVEFORMAT       *PDIGISTDWAVEFORMAT;
typedef DIGISTDWAVEFORMAT NEAR *NPDIGISTDWAVEFORMAT;
typedef DIGISTDWAVEFORMAT FAR  *LPDIGISTDWAVEFORMAT;

//
//  DSP Solutions (formerly DIGISPEECH) structure definitions
//
//      for WAVE_FORMAT_DIGIFIX   (0x0016)
//
//

typedef struct digifixwaveformat_tag {
        WAVEFORMATEX    wfx;
} DIGIFIXWAVEFORMAT;
typedef DIGIFIXWAVEFORMAT       *PDIGIFIXWAVEFORMAT;
typedef DIGIFIXWAVEFORMAT NEAR *NPDIGIFIXWAVEFORMAT;
typedef DIGIFIXWAVEFORMAT FAR  *LPDIGIFIXWAVEFORMAT;

//
//   Dialogic Corporation
// WAVEFORMAT_DIALOGIC_OKI_ADPCM   (0x0017)
//
typedef struct creative_fastspeechformat_tag{
        WAVEFORMATEX    ewf;
}DIALOGICOKIADPCMWAVEFORMAT;
typedef DIALOGICOKIADPCMWAVEFORMAT       *PDIALOGICOKIADPCMWAVEFORMAT;
typedef DIALOGICOKIADPCMWAVEFORMAT NEAR *NPDIALOGICOKIADPCMWAVEFORMAT;
typedef DIALOGICOKIADPCMWAVEFORMAT FAR  *LPDIALOGICOKIADPCMWAVEFORMAT;

//
//  Yamaha Compression's ADPCM structure definitions
//
//      for WAVE_FORMAT_YAMAHA_ADPCM   (0x0020)
//
//

typedef struct yamaha_adpmcwaveformat_tag {
        WAVEFORMATEX    wfx;

} YAMAHA_ADPCMWAVEFORMAT;
typedef YAMAHA_ADPCMWAVEFORMAT *PYAMAHA_ADPCMWAVEFORMAT;
typedef YAMAHA_ADPCMWAVEFORMAT NEAR *NPYAMAHA_ADPCMWAVEFORMAT;
typedef YAMAHA_ADPCMWAVEFORMAT FAR  *LPYAMAHA_ADPCMWAVEFORMAT;

//
//  Speech Compression's Sonarc structure definitions
//
//      for WAVE_FORMAT_SONARC   (0x0021)
//
//

typedef struct sonarcwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wCompType;
} SONARCWAVEFORMAT;
typedef SONARCWAVEFORMAT       *PSONARCWAVEFORMAT;
typedef SONARCWAVEFORMAT NEAR *NPSONARCWAVEFORMAT;
typedef SONARCWAVEFORMAT FAR  *LPSONARCWAVEFORMAT;

//
//  DSP Groups's TRUESPEECH structure definitions
//
//      for WAVE_FORMAT_DSPGROUP_TRUESPEECH   (0x0022)
//
//

typedef struct truespeechwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wRevision;
        WORD            nSamplesPerBlock;
        BYTE            abReserved[28];
} TRUESPEECHWAVEFORMAT;
typedef TRUESPEECHWAVEFORMAT       *PTRUESPEECHWAVEFORMAT;
typedef TRUESPEECHWAVEFORMAT NEAR *NPTRUESPEECHWAVEFORMAT;
typedef TRUESPEECHWAVEFORMAT FAR  *LPTRUESPEECHWAVEFORMAT;

//
//  Echo Speech Corp structure definitions
//
//      for WAVE_FORMAT_ECHOSC1   (0x0023)
//
//

typedef struct echosc1waveformat_tag {
        WAVEFORMATEX    wfx;
} ECHOSC1WAVEFORMAT;
typedef ECHOSC1WAVEFORMAT       *PECHOSC1WAVEFORMAT;
typedef ECHOSC1WAVEFORMAT NEAR *NPECHOSC1WAVEFORMAT;
typedef ECHOSC1WAVEFORMAT FAR  *LPECHOSC1WAVEFORMAT;

//
//  Audiofile Inc.structure definitions
//
//      for WAVE_FORMAT_AUDIOFILE_AF36   (0x0024)
//
//

typedef struct audiofile_af36waveformat_tag {
        WAVEFORMATEX    wfx;
} AUDIOFILE_AF36WAVEFORMAT;
typedef AUDIOFILE_AF36WAVEFORMAT       *PAUDIOFILE_AF36WAVEFORMAT;
typedef AUDIOFILE_AF36WAVEFORMAT NEAR *NPAUDIOFILE_AF36WAVEFORMAT;
typedef AUDIOFILE_AF36WAVEFORMAT FAR  *LPAUDIOFILE_AF36WAVEFORMAT;

//
//  Audio Processing Technology structure definitions
//
//      for WAVE_FORMAT_APTX   (0x0025)
//
//
typedef struct aptxwaveformat_tag {
        WAVEFORMATEX    wfx;
} APTXWAVEFORMAT;
typedef APTXWAVEFORMAT       *PAPTXWAVEFORMAT;
typedef APTXWAVEFORMAT NEAR *NPAPTXWAVEFORMAT;
typedef APTXWAVEFORMAT FAR  *LPAPTXWAVEFORMAT;

//
//  Audiofile Inc.structure definitions
//
//      for WAVE_FORMAT_AUDIOFILE_AF10   (0x0026)
//
//

typedef struct audiofile_af10waveformat_tag {
        WAVEFORMATEX    wfx;
} AUDIOFILE_AF10WAVEFORMAT;
typedef AUDIOFILE_AF10WAVEFORMAT       *PAUDIOFILE_AF10WAVEFORMAT;
typedef AUDIOFILE_AF10WAVEFORMAT NEAR *NPAUDIOFILE_AF10WAVEFORMAT;
typedef AUDIOFILE_AF10WAVEFORMAT FAR  *LPAUDIOFILE_AF10WAVEFORMAT;

//
/* Dolby's AC-2 wave format structure definition
           WAVE_FORMAT_DOLBY_AC2    (0x0030)*/
//
typedef struct dolbyac2waveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            nAuxBitsCode;
} DOLBYAC2WAVEFORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

/*Microsoft's */
// WAVE_FORMAT_GSM 610           0x0031
//
typedef struct gsm610waveformat_tag {
WAVEFORMATEX    wfx;
WORD                    wSamplesPerBlock;
} GSM610WAVEFORMAT;
typedef GSM610WAVEFORMAT *PGSM610WAVEFORMAT;
typedef GSM610WAVEFORMAT NEAR    *NPGSM610WAVEFORMAT;
typedef GSM610WAVEFORMAT FAR     *LPGSM610WAVEFORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
//      Antex Electronics Corp
//
//      for WAVE_FORMAT_ADPCME                  (0x0033)
//
//

typedef struct adpcmewaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wSamplesPerBlock;
} ADPCMEWAVEFORMAT;
typedef ADPCMEWAVEFORMAT                *PADPCMEWAVEFORMAT;
typedef ADPCMEWAVEFORMAT NEAR   *NPADPCMEWAVEFORMAT;
typedef ADPCMEWAVEFORMAT FAR    *LPADPCMEWAVEFORMAT;

/*       Control Resources Limited */
// WAVE_FORMAT_CONTROL_RES_VQLPC                 0x0034
//
typedef struct contres_vqlpcwaveformat_tag {
WAVEFORMATEX    wfx;
WORD                    wSamplesPerBlock;
} CONTRESVQLPCWAVEFORMAT;
typedef CONTRESVQLPCWAVEFORMAT *PCONTRESVQLPCWAVEFORMAT;
typedef CONTRESVQLPCWAVEFORMAT NEAR      *NPCONTRESVQLPCWAVEFORMAT;
typedef CONTRESVQLPCWAVEFORMAT FAR       *LPCONTRESVQLPCWAVEFORMAT;

//
//
//
//      for WAVE_FORMAT_DIGIREAL                   (0x0035)
//
//

typedef struct digirealwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wSamplesPerBlock;
} DIGIREALWAVEFORMAT;
typedef DIGIREALWAVEFORMAT *PDIGIREALWAVEFORMAT;
typedef DIGIREALWAVEFORMAT NEAR *NPDIGIREALWAVEFORMAT;
typedef DIGIREALWAVEFORMAT FAR *LPDIGIREALWAVEFORMAT;

//
//  DSP Solutions
//
//      for WAVE_FORMAT_DIGIADPCM   (0x0036)
//
//

typedef struct digiadpcmmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wSamplesPerBlock;
} DIGIADPCMWAVEFORMAT;
typedef DIGIADPCMWAVEFORMAT       *PDIGIADPCMWAVEFORMAT;
typedef DIGIADPCMWAVEFORMAT NEAR *NPDIGIADPCMWAVEFORMAT;
typedef DIGIADPCMWAVEFORMAT FAR  *LPDIGIADPCMWAVEFORMAT;

/*       Control Resources Limited */
// WAVE_FORMAT_CONTROL_RES_CR10          0x0037
//
typedef struct contres_cr10waveformat_tag {
WAVEFORMATEX    wfx;
WORD                    wSamplesPerBlock;
} CONTRESCR10WAVEFORMAT;
typedef CONTRESCR10WAVEFORMAT *PCONTRESCR10WAVEFORMAT;
typedef CONTRESCR10WAVEFORMAT NEAR       *NPCONTRESCR10WAVEFORMAT;
typedef CONTRESCR10WAVEFORMAT FAR        *LPCONTRESCR10WAVEFORMAT;

//
//  Natural Microsystems
//
//      for WAVE_FORMAT_NMS_VBXADPCM   (0x0038)
//
//

typedef struct nms_vbxadpcmmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wSamplesPerBlock;
} NMS_VBXADPCMWAVEFORMAT;
typedef NMS_VBXADPCMWAVEFORMAT       *PNMS_VBXADPCMWAVEFORMAT;
typedef NMS_VBXADPCMWAVEFORMAT NEAR *NPNMS_VBXADPCMWAVEFORMAT;
typedef NMS_VBXADPCMWAVEFORMAT FAR  *LPNMS_VBXADPCMWAVEFORMAT;

//
//  Antex Electronics  structure definitions
//
//      for WAVE_FORMAT_G721_ADPCM   (0x0040)
//
//

typedef struct g721_adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            nAuxBlockSize;
} G721_ADPCMWAVEFORMAT;
typedef G721_ADPCMWAVEFORMAT *PG721_ADPCMWAVEFORMAT;
typedef G721_ADPCMWAVEFORMAT NEAR *NPG721_ADPCMWAVEFORMAT;
typedef G721_ADPCMWAVEFORMAT FAR  *LPG721_ADPCMWAVEFORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
//
// Microsoft MPEG audio WAV definition
//
/*  MPEG-1 audio wave format (audio layer only).   (0x0050)   */
typedef struct mpeg1waveformat_tag {
    WAVEFORMATEX    wfx;
    WORD            fwHeadLayer;
    DWORD           dwHeadBitrate;
    WORD            fwHeadMode;
    WORD            fwHeadModeExt;
    WORD            wHeadEmphasis;
    WORD            fwHeadFlags;
    DWORD           dwPTSLow;
    DWORD           dwPTSHigh;
} MPEG1WAVEFORMAT;
typedef MPEG1WAVEFORMAT                 *PMPEG1WAVEFORMAT;
typedef MPEG1WAVEFORMAT NEAR           *NPMPEG1WAVEFORMAT;
typedef MPEG1WAVEFORMAT FAR            *LPMPEG1WAVEFORMAT;

#define ACM_MPEG_LAYER1             (0x0001)
#define ACM_MPEG_LAYER2             (0x0002)
#define ACM_MPEG_LAYER3             (0x0004)
#define ACM_MPEG_STEREO             (0x0001)
#define ACM_MPEG_JOINTSTEREO        (0x0002)
#define ACM_MPEG_DUALCHANNEL        (0x0004)
#define ACM_MPEG_SINGLECHANNEL      (0x0008)
#define ACM_MPEG_PRIVATEBIT         (0x0001)
#define ACM_MPEG_COPYRIGHT          (0x0002)
#define ACM_MPEG_ORIGINALHOME       (0x0004)
#define ACM_MPEG_PROTECTIONBIT      (0x0008)
#define ACM_MPEG_ID_MPEG1           (0x0010)

//
// MPEG Layer3 WAVEFORMATEX structure
// for WAVE_FORMAT_MPEGLAYER3 (0x0055)
//
#define MPEGLAYER3_WFX_EXTRA_BYTES   12

// WAVE_FORMAT_MPEGLAYER3 format sructure
//
typedef struct mpeglayer3waveformat_tag {
  WAVEFORMATEX  wfx;
  WORD          wID;
  DWORD         fdwFlags;
  WORD          nBlockSize;
  WORD          nFramesPerBlock;
  WORD          nCodecDelay;
} MPEGLAYER3WAVEFORMAT;

typedef MPEGLAYER3WAVEFORMAT          *PMPEGLAYER3WAVEFORMAT;
typedef MPEGLAYER3WAVEFORMAT NEAR    *NPMPEGLAYER3WAVEFORMAT;
typedef MPEGLAYER3WAVEFORMAT FAR     *LPMPEGLAYER3WAVEFORMAT;

//==========================================================================;

#define MPEGLAYER3_ID_UNKNOWN            0
#define MPEGLAYER3_ID_MPEG               1
#define MPEGLAYER3_ID_CONSTANTFRAMESIZE  2

#define MPEGLAYER3_FLAG_PADDING_ISO      0x00000000
#define MPEGLAYER3_FLAG_PADDING_ON       0x00000001
#define MPEGLAYER3_FLAG_PADDING_OFF      0x00000002

/* ==========================================================================

      For WAVE_FORMAT_MPEG_HEAAC (0x1610)


 "MPEG-2" in the comments below refers to ISO/IEC 13818-7 (MPEG-2 AAC spec).
 "MPEG-4" in the comments below refers to ISO/IEC 14496-3 (MPEG-4 Audio spec).

 The following defines the format block to be used for MPEG-2 AAC or MPEG-4 HE-AAC v1/v2 streams.
 When setting media type attributes in Media Foundation (MF) objects, this will appear in conjunction with
 major type MFMediaType_Audio and sub type MFAudioFormat_AAC (=MEDIASUBTYPE_MPEG_HEAAC).
 The format block structure HEAACWAVEFORMAT is defined below.  It starts with the structure
 HEAACWAVEINFO (which is an extension of WAVEFORMATEX), followed by AudioSpecificConfig() as
 defined by ISO/IEC 14496-3 (MPEG-4 audio). Since the length of AudioSpecificConfig() may vary
 from one stream to another, this is a variable size format block.

 The WAVEFORMATEX fields describe the properties of the core AAC stream,
 without SBR/PS extensions (if exists). This is the description of the WAVEFORMATEX fields:

 wfx.wFormatTag - Set this to WAVE_FORMAT_MPEG_HEAAC (0x1610).

 wfx.nChannels - Total number of channels in core AAC stream (including LFE if exists).
 This might be different than the decoded number of channels if the MPEG-4 Parametric Stereo (PS)
 tool exists. If unknown, set to zero.

 wfx.nSamplesPerSec - Sampling rate of core AAC stream. This will be one of the 12 supported
 sampling rate between 8000 and 96000 Hz, as defined in MPEG-2.  This might be different than
 the decoded sampling rate if the MPEG-4 Spectral Band Replication (SBR) tool exists.
 If not know in advance, the sampling rate can be extracted from:
 - the 4-bit sampling_frequency_index field in adts_fixed_header(), or
 - program_config_element() in adif_header for MPEG-2 streams, or
 - the 4-bit samplingFrequencyIndex field in AudioSpecificConfig() for MPEG-4 streams.

 wfx.nAvgBytesPerSec - The average bytes per second calculated based on the average bit rate of
 the compressed stream. This value may be used by parsers to seek into a particular time offset
 in the stream. It may also be used by applications to determine roughly how much buffer length to allocate.
 If this is not known in advance, this value can be estimated by parsing some (or all) of the
 compressed HE-AAC frames and calculating bit rate based on average compressed frame size.
 If unknown, set to zero.

 wfx.nBlockAlign - Set this to 1.

 wfx.wBitsPerSample - Desired bit depth of the decoded PCM. If unknown, set to zero.

 wfx.cbSize - Set this to 12 (=sizeof(HEAACWAVEINFO)-sizeof(WAVEFORMATEX)) plus the
 size of AudioSpecificConfig() in bytes.

 ===================================

 How do we parse this format block? assume pbBuff is the address of the first
 byte in the format block. We do the following:

 HEAACWAVEINFO* pwfInfo = (HEAACWAVEINFO *)pbBuff;

 if ( 0 == pwfInfo->wStructType)
 {
    HEAACWAVEFORMAT* pwf = (HEAACWAVEFORMAT*)pbBuff;

    // All HEAACWAVEFORMAT fields can now be accessed through pwf
    // ...

    //
    // To parse AudioSpecifiConfig(), write a function such as
    // ParseAudioSpecificConfig(BYTE *pbASC, DWORD dwASCLen),
    // and call:
    //
    DWORD dwASCLen = pwf->wfInfo.wfx.cbSize - sizeof(HEAACWAVEINFO) + sizeof(WAVEFORMATEX);

    ParseAudioSpecificConfig(pwf->pbAudioSpecificConfig, dwASCLen);
 }
 else
 {
    // Reserved
 }
*/

typedef struct heaacwaveinfo_tag {

    // Defines core AAC properties. See description above. WAVEFORMATEX is of size 18 bytes.
    WAVEFORMATEX wfx;

    // Defines the payload type
    // 0-RAW.  The stream contains raw_data_block() elements only.
    // 1-ADTS. The stream contains an adts_sequence(), as defined by MPEG-2.
    // 2-ADIF. The stream contains an adif_sequence(), as defined by MPEG-2.
    // 3-LOAS. The stream contains an MPEG-4 audio transport stream with a
    //         synchronization layer LOAS and a multiplex layer LATM.
    // All other codes are reserved.
    WORD  wPayloadType;

    // This is the 8-bit field audioProfileLevelIndication available in the
    // MPEG-4 object descriptor.  It is an indication (as defined in MPEG-4 audio)
    // of the audio profile and level required to process the content associated 
    // with this stream. For example values 0x28-0x2B correspond to AAC Profile,
    // values 0x2C-0x2F correspond to HE-AAC profile and 0x30-0x33 for HE-AAC v2 profile.
    // If unknown, set to zero or 0xFE ("no audio profile specified").
    WORD  wAudioProfileLevelIndication;

    // Defines the data that follows this structure. Currently only one data type is supported:
    // 0- AudioSpecificConfig() (as defined by MPEG-4 Audio, ISO/IEC 14496-3) will follow this structure.
    //    wfx.cbSize will indicate the total length including AudioSpecificConfig().
    //    Use HEAACWAVEFORMAT to gain easy access to the address of the first byte of
    //    AudioSpecificConfig() for parsing.
    //    Typical values for the size of AudioSpecificConfig (ASC) are:
    //    - 2 bytes for AAC or HE-AAC v1/v2 with implicit signaling of SBR,
    //    - 5 bytes for HE-AAC v1 with explicit signaling of SBR,
    //    - 7 bytes for HE-AAC v2 with explicit signaling of SBR and PS.
    //    The size may be longer than 7 bytes if the 4-bit channelConfiguration field in ASC is zero,
    //    which means program_config_element() is present in ASC.
    //
    // All other codes are reserved.
    WORD  wStructType;

    // Set these to zero
    WORD  wReserved1;
    DWORD dwReserved2;

} HEAACWAVEINFO;  // this structure has a size of 30 bytes

typedef HEAACWAVEINFO       *PHEAACWAVEINFO;
typedef HEAACWAVEINFO NEAR *NPHEAACWAVEINFO;
typedef HEAACWAVEINFO FAR  *LPHEAACWAVEINFO;

//
// This structure describes the format block for wStructType=0
//
typedef struct heaacwaveformat_tag {

    HEAACWAVEINFO wfInfo; // This structure has a size of 30 bytes
    BYTE          pbAudioSpecificConfig[1];  // First byte of AudioSpecificConfig()

} HEAACWAVEFORMAT; // This structure has a size of 31 bytes

typedef HEAACWAVEFORMAT       *PHEAACWAVEFORMAT;
typedef HEAACWAVEFORMAT NEAR *NPHEAACWAVEFORMAT;
typedef HEAACWAVEFORMAT FAR  *LPHEAACWAVEFORMAT;


//==========================================================================


//
//  Windows Media Audio (common)
//
//

#define MM_MSFT_ACM_WMAUDIO  39

#define WMAUDIO_BITS_PER_SAMPLE    16 // just an uncompressed size...
#define WMAUDIO_MAX_CHANNELS       2

//
//  Windows Media Audio V1 (a.k.a. "MSAudio")
//
//      for WAVE_FORMAT_MSAUDIO1        (0x0160)
//
//
#define MM_MSFT_ACM_MSAUDIO1  39

typedef struct msaudio1waveformat_tag {
    WAVEFORMATEX wfx;
    WORD         wSamplesPerBlock; // only counting "new" samples "= half of what will be used due to overlapping
    WORD         wEncodeOptions;
} MSAUDIO1WAVEFORMAT;

typedef MSAUDIO1WAVEFORMAT FAR  *LPMSAUDIO1WAVEFORMAT;

#define MSAUDIO1_BITS_PER_SAMPLE    WMAUDIO_BITS_PER_SAMPLE
#define MSAUDIO1_MAX_CHANNELS       WMAUDIO_MAX_CHANNELS
#define MSAUDIO1_WFX_EXTRA_BYTES    (sizeof(MSAUDIO1WAVEFORMAT) - sizeof(WAVEFORMATEX))

//
//  Windows Media Audio V2
//
//      for WAVE_FORMAT_WMAUDIO2        (0x0161)
//
//

#define MM_MSFT_ACM_WMAUDIO2  101

typedef struct wmaudio2waveformat_tag {
    WAVEFORMATEX wfx;
    DWORD        dwSamplesPerBlock; // only counting "new" samples "= half of what will be used due to overlapping
    WORD         wEncodeOptions;
    DWORD        dwSuperBlockAlign; // the big size...  should be multiples of wfx.nBlockAlign.
} WMAUDIO2WAVEFORMAT;

typedef WMAUDIO2WAVEFORMAT FAR  *LPWMAUDIO2WAVEFORMAT;

#define WMAUDIO2_BITS_PER_SAMPLE    WMAUDIO_BITS_PER_SAMPLE
#define WMAUDIO2_MAX_CHANNELS       WMAUDIO_MAX_CHANNELS
#define WMAUDIO2_WFX_EXTRA_BYTES    (sizeof(WMAUDIO2WAVEFORMAT) - sizeof(WAVEFORMATEX))

//
//  Windows Media Audio V3
//
//      for WAVE_FORMAT_WMAUDIO3        (0x0162)
//
//

typedef struct wmaudio3waveformat_tag {
  WAVEFORMATEX wfx;
  WORD         wValidBitsPerSample; // bits of precision 
  DWORD        dwChannelMask;       // which channels are present in stream
  DWORD        dwReserved1;
  DWORD        dwReserved2;
  WORD         wEncodeOptions;
  WORD         wReserved3;
} WMAUDIO3WAVEFORMAT;

typedef WMAUDIO3WAVEFORMAT FAR *LPWMAUDIO3WAVEFORMAT;
#define WMAUDIO3_WFX_EXTRA_BYTES    (sizeof(WMAUDIO3WAVEFORMAT) - sizeof(WAVEFORMATEX))

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
//  Creative's ADPCM structure definitions
//
//      for WAVE_FORMAT_CREATIVE_ADPCM   (0x0200)
//
//

typedef struct creative_adpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wRevision;
} CREATIVEADPCMWAVEFORMAT;
typedef CREATIVEADPCMWAVEFORMAT       *PCREATIVEADPCMWAVEFORMAT;
typedef CREATIVEADPCMWAVEFORMAT NEAR *NPCREATIVEADPCMWAVEFORMAT;
typedef CREATIVEADPCMWAVEFORMAT FAR  *LPCREATIVEADPCMWAVEFORMAT;

//
//    Creative FASTSPEECH
// WAVEFORMAT_CREATIVE_FASTSPEECH8   (0x0202)
//
typedef struct creative_fastspeech8format_tag {
        WAVEFORMATEX    wfx;
        WORD wRevision;
} CREATIVEFASTSPEECH8WAVEFORMAT;
typedef CREATIVEFASTSPEECH8WAVEFORMAT       *PCREATIVEFASTSPEECH8WAVEFORMAT;
typedef CREATIVEFASTSPEECH8WAVEFORMAT NEAR *NPCREATIVEFASTSPEECH8WAVEFORMAT;
typedef CREATIVEFASTSPEECH8WAVEFORMAT FAR  *LPCREATIVEFASTSPEECH8WAVEFORMAT;
//
//    Creative FASTSPEECH
// WAVEFORMAT_CREATIVE_FASTSPEECH10   (0x0203)
//
typedef struct creative_fastspeech10format_tag {
        WAVEFORMATEX    wfx;
        WORD wRevision;
} CREATIVEFASTSPEECH10WAVEFORMAT;
typedef CREATIVEFASTSPEECH10WAVEFORMAT       *PCREATIVEFASTSPEECH10WAVEFORMAT;
typedef CREATIVEFASTSPEECH10WAVEFORMAT NEAR *NPCREATIVEFASTSPEECH10WAVEFORMAT;
typedef CREATIVEFASTSPEECH10WAVEFORMAT FAR  *LPCREATIVEFASTSPEECH10WAVEFORMAT;

//
//  Fujitsu FM Towns 'SND' structure
//
//      for WAVE_FORMAT_FMMTOWNS_SND   (0x0300)
//
//

typedef struct fmtowns_snd_waveformat_tag {
        WAVEFORMATEX    wfx;
        WORD            wRevision;
} FMTOWNS_SND_WAVEFORMAT;
typedef FMTOWNS_SND_WAVEFORMAT       *PFMTOWNS_SND_WAVEFORMAT;
typedef FMTOWNS_SND_WAVEFORMAT NEAR *NPFMTOWNS_SND_WAVEFORMAT;
typedef FMTOWNS_SND_WAVEFORMAT FAR  *LPFMTOWNS_SND_WAVEFORMAT;

//
//  Olivetti structure
//
//      for WAVE_FORMAT_OLIGSM   (0x1000)
//
//

typedef struct oligsmwaveformat_tag {
        WAVEFORMATEX    wfx;
} OLIGSMWAVEFORMAT;
typedef OLIGSMWAVEFORMAT     *POLIGSMWAVEFORMAT;
typedef OLIGSMWAVEFORMAT NEAR *NPOLIGSMWAVEFORMAT;
typedef OLIGSMWAVEFORMAT  FAR  *LPOLIGSMWAVEFORMAT;

//
//  Olivetti structure
//
//      for WAVE_FORMAT_OLIADPCM   (0x1001)
//
//

typedef struct oliadpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
} OLIADPCMWAVEFORMAT;
typedef OLIADPCMWAVEFORMAT     *POLIADPCMWAVEFORMAT;
typedef OLIADPCMWAVEFORMAT NEAR *NPOLIADPCMWAVEFORMAT ;
typedef OLIADPCMWAVEFORMAT  FAR  *LPOLIADPCMWAVEFORMAT;

//
//  Olivetti structure
//
//      for WAVE_FORMAT_OLICELP   (0x1002)
//
//

typedef struct olicelpwaveformat_tag {
        WAVEFORMATEX    wfx;
} OLICELPWAVEFORMAT;
typedef OLICELPWAVEFORMAT     *POLICELPWAVEFORMAT;
typedef OLICELPWAVEFORMAT NEAR *NPOLICELPWAVEFORMAT ;
typedef OLICELPWAVEFORMAT  FAR  *LPOLICELPWAVEFORMAT;

//
//  Olivetti structure
//
//      for WAVE_FORMAT_OLISBC   (0x1003)
//
//

typedef struct olisbcwaveformat_tag {
        WAVEFORMATEX    wfx;
} OLISBCWAVEFORMAT;
typedef OLISBCWAVEFORMAT     *POLISBCWAVEFORMAT;
typedef OLISBCWAVEFORMAT NEAR *NPOLISBCWAVEFORMAT ;
typedef OLISBCWAVEFORMAT  FAR  *LPOLISBCWAVEFORMAT;

//
//  Olivetti structure
//
//      for WAVE_FORMAT_OLIOPR   (0x1004)
//
//

typedef struct olioprwaveformat_tag {
        WAVEFORMATEX    wfx;
} OLIOPRWAVEFORMAT;
typedef OLIOPRWAVEFORMAT     *POLIOPRWAVEFORMAT;
typedef OLIOPRWAVEFORMAT NEAR *NPOLIOPRWAVEFORMAT ;
typedef OLIOPRWAVEFORMAT  FAR  *LPOLIOPRWAVEFORMAT;

//
//  Crystal Semiconductor IMA ADPCM format
//
//      for WAVE_FORMAT_CS_IMAADPCM   (0x0039)
//
//

typedef struct csimaadpcmwaveformat_tag {
        WAVEFORMATEX    wfx;
} CSIMAADPCMWAVEFORMAT;
typedef CSIMAADPCMWAVEFORMAT     *PCSIMAADPCMWAVEFORMAT;
typedef CSIMAADPCMWAVEFORMAT NEAR *NPCSIMAADPCMWAVEFORMAT ;
typedef CSIMAADPCMWAVEFORMAT  FAR  *LPCSIMAADPCMWAVEFORMAT;

//==========================================================================;
//
//  ACM Wave Filters
//
//
//==========================================================================;

#ifndef _ACM_WAVEFILTER
#define _ACM_WAVEFILTER

#define WAVE_FILTER_UNKNOWN         0x0000
#define WAVE_FILTER_DEVELOPMENT    (0xFFFF)

typedef struct wavefilter_tag {
    DWORD   cbStruct;           /* Size of the filter in bytes */
    DWORD   dwFilterTag;        /* filter type */
    DWORD   fdwFilter;          /* Flags for the filter (Universal Dfns) */
    DWORD   dwReserved[5];      /* Reserved for system use */
} WAVEFILTER;
typedef WAVEFILTER       *PWAVEFILTER;
typedef WAVEFILTER NEAR *NPWAVEFILTER;
typedef WAVEFILTER FAR  *LPWAVEFILTER;

#endif  /* _ACM_WAVEFILTER */

#ifndef WAVE_FILTER_VOLUME
#define WAVE_FILTER_VOLUME      0x0001

typedef struct wavefilter_volume_tag {
        WAVEFILTER      wfltr;
        DWORD           dwVolume;
} VOLUMEWAVEFILTER;
typedef VOLUMEWAVEFILTER       *PVOLUMEWAVEFILTER;
typedef VOLUMEWAVEFILTER NEAR *NPVOLUMEWAVEFILTER;
typedef VOLUMEWAVEFILTER FAR  *LPVOLUMEWAVEFILTER;

#endif  /* WAVE_FILTER_VOLUME */

#ifndef WAVE_FILTER_ECHO
#define WAVE_FILTER_ECHO        0x0002

typedef struct wavefilter_echo_tag {
        WAVEFILTER      wfltr;
        DWORD           dwVolume;
        DWORD           dwDelay;
} ECHOWAVEFILTER;
typedef ECHOWAVEFILTER       *PECHOWAVEFILTER;
typedef ECHOWAVEFILTER NEAR *NPECHOWAVEFILTER;
typedef ECHOWAVEFILTER FAR  *LPECHOWAVEFILTER;

#endif  /* WAVEFILTER_ECHO */

/* ------------------------------------------------------------------------------ */
//
// New RIFF WAVE Chunks
//

#define RIFFWAVE_inst   mmioFOURCC('i','n','s','t')

struct tag_s_RIFFWAVE_inst {
    BYTE    bUnshiftedNote;
    char    chFineTune;
    char    chGain;
    BYTE    bLowNote;
    BYTE    bHighNote;
    BYTE    bLowVelocity;
    BYTE    bHighVelocity;
};

typedef struct tag_s_RIFFWAVE_INST s_RIFFWAVE_inst;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/* ------------------------------------------------------------------------------ */
//
// New RIFF Forms
//

#ifndef NONEWRIFF

/* RIFF AVI */

//
// AVI file format is specified in a seperate file (AVIFMT.H),
// which is available in the VfW and Win 32 SDK
//

/* RIFF CPPO */

#define RIFFCPPO        mmioFOURCC('C','P','P','O')

#define RIFFCPPO_objr   mmioFOURCC('o','b','j','r')
#define RIFFCPPO_obji   mmioFOURCC('o','b','j','i')

#define RIFFCPPO_clsr   mmioFOURCC('c','l','s','r')
#define RIFFCPPO_clsi   mmioFOURCC('c','l','s','i')

#define RIFFCPPO_mbr    mmioFOURCC('m','b','r',' ')

#define RIFFCPPO_char   mmioFOURCC('c','h','a','r')

#define RIFFCPPO_byte   mmioFOURCC('b','y','t','e')
#define RIFFCPPO_int    mmioFOURCC('i','n','t',' ')
#define RIFFCPPO_word   mmioFOURCC('w','o','r','d')
#define RIFFCPPO_long   mmioFOURCC('l','o','n','g')
#define RIFFCPPO_dwrd   mmioFOURCC('d','w','r','d')
#define RIFFCPPO_flt    mmioFOURCC('f','l','t',' ')
#define RIFFCPPO_dbl    mmioFOURCC('d','b','l',' ')
#define RIFFCPPO_str    mmioFOURCC('s','t','r',' ')

#endif

/*
//////////////////////////////////////////////////////////////////////////
//
// DIB Compression Defines
//
*/

#ifndef BI_BITFIELDS
#define BI_BITFIELDS    3
#endif

#ifndef QUERYDIBSUPPORT

#define QUERYDIBSUPPORT 3073
#define QDI_SETDIBITS   0x0001
#define QDI_GETDIBITS   0x0002
#define QDI_DIBTOSCREEN 0x0004
#define QDI_STRETCHDIB  0x0008

#endif

#ifndef NOBITMAP
/* Structure definitions */

typedef struct tagEXBMINFOHEADER {
        BITMAPINFOHEADER    bmi;
        /* extended BITMAPINFOHEADER fields */
        DWORD   biExtDataOffset;

        /* Other stuff will go here */

        /* ... */

        /* Format-specific information */
        /* biExtDataOffset points here */

} EXBMINFOHEADER;

#endif          //NOBITMAP

/* New DIB Compression Defines */

#define BICOMP_IBMULTIMOTION    mmioFOURCC('U', 'L', 'T', 'I')
#define BICOMP_IBMPHOTOMOTION   mmioFOURCC('P', 'H', 'M', 'O')
#define BICOMP_CREATIVEYUV      mmioFOURCC('c', 'y', 'u', 'v')

#ifndef NOJPEGDIB

/* New DIB Compression Defines */
#define JPEG_DIB        mmioFOURCC('J','P','E','G')    /* Still image JPEG DIB biCompression */
#define MJPG_DIB        mmioFOURCC('M','J','P','G')    /* Motion JPEG DIB biCompression     */

/* JPEGProcess Definitions */
#define JPEG_PROCESS_BASELINE           0       /* Baseline DCT */

/* AVI File format extensions */
#define AVIIF_CONTROLFRAME              0x00000200L     /* This is a control frame */

    /* JIF Marker byte pairs in JPEG Interchange Format sequence */
#define JIFMK_SOF0    0xFFC0   /* SOF Huff  - Baseline DCT*/
#define JIFMK_SOF1    0xFFC1   /* SOF Huff  - Extended sequential DCT*/
#define JIFMK_SOF2    0xFFC2   /* SOF Huff  - Progressive DCT*/
#define JIFMK_SOF3    0xFFC3   /* SOF Huff  - Spatial (sequential) lossless*/
#define JIFMK_SOF5    0xFFC5   /* SOF Huff  - Differential sequential DCT*/
#define JIFMK_SOF6    0xFFC6   /* SOF Huff  - Differential progressive DCT*/
#define JIFMK_SOF7    0xFFC7   /* SOF Huff  - Differential spatial*/
#define JIFMK_JPG     0xFFC8   /* SOF Arith - Reserved for JPEG extensions*/
#define JIFMK_SOF9    0xFFC9   /* SOF Arith - Extended sequential DCT*/
#define JIFMK_SOF10   0xFFCA   /* SOF Arith - Progressive DCT*/
#define JIFMK_SOF11   0xFFCB   /* SOF Arith - Spatial (sequential) lossless*/
#define JIFMK_SOF13   0xFFCD   /* SOF Arith - Differential sequential DCT*/
#define JIFMK_SOF14   0xFFCE   /* SOF Arith - Differential progressive DCT*/
#define JIFMK_SOF15   0xFFCF   /* SOF Arith - Differential spatial*/
#define JIFMK_DHT     0xFFC4   /* Define Huffman Table(s) */
#define JIFMK_DAC     0xFFCC   /* Define Arithmetic coding conditioning(s) */
#define JIFMK_RST0    0xFFD0   /* Restart with modulo 8 count 0 */
#define JIFMK_RST1    0xFFD1   /* Restart with modulo 8 count 1 */
#define JIFMK_RST2    0xFFD2   /* Restart with modulo 8 count 2 */
#define JIFMK_RST3    0xFFD3   /* Restart with modulo 8 count 3 */
#define JIFMK_RST4    0xFFD4   /* Restart with modulo 8 count 4 */
#define JIFMK_RST5    0xFFD5   /* Restart with modulo 8 count 5 */
#define JIFMK_RST6    0xFFD6   /* Restart with modulo 8 count 6 */
#define JIFMK_RST7    0xFFD7   /* Restart with modulo 8 count 7 */
#define JIFMK_SOI     0xFFD8   /* Start of Image */
#define JIFMK_EOI     0xFFD9   /* End of Image */
#define JIFMK_SOS     0xFFDA   /* Start of Scan */
#define JIFMK_DQT     0xFFDB   /* Define quantization Table(s) */
#define JIFMK_DNL     0xFFDC   /* Define Number of Lines */
#define JIFMK_DRI     0xFFDD   /* Define Restart Interval */
#define JIFMK_DHP     0xFFDE   /* Define Hierarchical progression */
#define JIFMK_EXP     0xFFDF   /* Expand Reference Component(s) */
#define JIFMK_APP0    0xFFE0   /* Application Field 0*/
#define JIFMK_APP1    0xFFE1   /* Application Field 1*/
#define JIFMK_APP2    0xFFE2   /* Application Field 2*/
#define JIFMK_APP3    0xFFE3   /* Application Field 3*/
#define JIFMK_APP4    0xFFE4   /* Application Field 4*/
#define JIFMK_APP5    0xFFE5   /* Application Field 5*/
#define JIFMK_APP6    0xFFE6   /* Application Field 6*/
#define JIFMK_APP7    0xFFE7   /* Application Field 7*/
#define JIFMK_JPG0    0xFFF0   /* Reserved for JPEG extensions */
#define JIFMK_JPG1    0xFFF1   /* Reserved for JPEG extensions */
#define JIFMK_JPG2    0xFFF2   /* Reserved for JPEG extensions */
#define JIFMK_JPG3    0xFFF3   /* Reserved for JPEG extensions */
#define JIFMK_JPG4    0xFFF4   /* Reserved for JPEG extensions */
#define JIFMK_JPG5    0xFFF5   /* Reserved for JPEG extensions */
#define JIFMK_JPG6    0xFFF6   /* Reserved for JPEG extensions */
#define JIFMK_JPG7    0xFFF7   /* Reserved for JPEG extensions */
#define JIFMK_JPG8    0xFFF8   /* Reserved for JPEG extensions */
#define JIFMK_JPG9    0xFFF9   /* Reserved for JPEG extensions */
#define JIFMK_JPG10   0xFFFA   /* Reserved for JPEG extensions */
#define JIFMK_JPG11   0xFFFB   /* Reserved for JPEG extensions */
#define JIFMK_JPG12   0xFFFC   /* Reserved for JPEG extensions */
#define JIFMK_JPG13   0xFFFD   /* Reserved for JPEG extensions */
#define JIFMK_COM     0xFFFE   /* Comment */
#define JIFMK_TEM     0xFF01   /* for temp private use arith code */
#define JIFMK_RES     0xFF02   /* Reserved */
#define JIFMK_00      0xFF00   /* Zero stuffed byte - entropy data */
#define JIFMK_FF      0xFFFF   /* Fill byte */

/* JPEGColorSpaceID Definitions */
#define JPEG_Y          1       /* Y only component of YCbCr */
#define JPEG_YCbCr      2       /* YCbCr as define by CCIR 601 */
#define JPEG_RGB        3       /* 3 component RGB */

/* Structure definitions */

typedef struct tagJPEGINFOHEADER {
    /* compression-specific fields */
    /* these fields are defined for 'JPEG' and 'MJPG' */
    DWORD       JPEGSize;
    DWORD       JPEGProcess;

    /* Process specific fields */
    DWORD       JPEGColorSpaceID;
    DWORD       JPEGBitsPerSample;
    DWORD       JPEGHSubSampling;
    DWORD       JPEGVSubSampling;
} JPEGINFOHEADER;

#ifdef MJPGDHTSEG_STORAGE

/* Default DHT Segment */

MJPGDHTSEG_STORAGE BYTE MJPGDHTSeg[0x1A4] = {
 /* JPEG DHT Segment for YCrCb omitted from MJPG data */
0xFF,0xC4,0x01,0xA2,
0x00,0x00,0x01,0x05,0x01,0x01,0x01,0x01,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0A,0x0B,0x01,0x00,0x03,0x01,0x01,0x01,0x01,
0x01,0x01,0x01,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,
0x08,0x09,0x0A,0x0B,0x10,0x00,0x02,0x01,0x03,0x03,0x02,0x04,0x03,0x05,0x05,0x04,0x04,0x00,
0x00,0x01,0x7D,0x01,0x02,0x03,0x00,0x04,0x11,0x05,0x12,0x21,0x31,0x41,0x06,0x13,0x51,0x61,
0x07,0x22,0x71,0x14,0x32,0x81,0x91,0xA1,0x08,0x23,0x42,0xB1,0xC1,0x15,0x52,0xD1,0xF0,0x24,
0x33,0x62,0x72,0x82,0x09,0x0A,0x16,0x17,0x18,0x19,0x1A,0x25,0x26,0x27,0x28,0x29,0x2A,0x34,
0x35,0x36,0x37,0x38,0x39,0x3A,0x43,0x44,0x45,0x46,0x47,0x48,0x49,0x4A,0x53,0x54,0x55,0x56,
0x57,0x58,0x59,0x5A,0x63,0x64,0x65,0x66,0x67,0x68,0x69,0x6A,0x73,0x74,0x75,0x76,0x77,0x78,
0x79,0x7A,0x83,0x84,0x85,0x86,0x87,0x88,0x89,0x8A,0x92,0x93,0x94,0x95,0x96,0x97,0x98,0x99,
0x9A,0xA2,0xA3,0xA4,0xA5,0xA6,0xA7,0xA8,0xA9,0xAA,0xB2,0xB3,0xB4,0xB5,0xB6,0xB7,0xB8,0xB9,
0xBA,0xC2,0xC3,0xC4,0xC5,0xC6,0xC7,0xC8,0xC9,0xCA,0xD2,0xD3,0xD4,0xD5,0xD6,0xD7,0xD8,0xD9,
0xDA,0xE1,0xE2,0xE3,0xE4,0xE5,0xE6,0xE7,0xE8,0xE9,0xEA,0xF1,0xF2,0xF3,0xF4,0xF5,0xF6,0xF7,
0xF8,0xF9,0xFA,0x11,0x00,0x02,0x01,0x02,0x04,0x04,0x03,0x04,0x07,0x05,0x04,0x04,0x00,0x01,
0x02,0x77,0x00,0x01,0x02,0x03,0x11,0x04,0x05,0x21,0x31,0x06,0x12,0x41,0x51,0x07,0x61,0x71,
0x13,0x22,0x32,0x81,0x08,0x14,0x42,0x91,0xA1,0xB1,0xC1,0x09,0x23,0x33,0x52,0xF0,0x15,0x62,
0x72,0xD1,0x0A,0x16,0x24,0x34,0xE1,0x25,0xF1,0x17,0x18,0x19,0x1A,0x26,0x27,0x28,0x29,0x2A,
0x35,0x36,0x37,0x38,0x39,0x3A,0x43,0x44,0x45,0x46,0x47,0x48,0x49,0x4A,0x53,0x54,0x55,0x56,
0x57,0x58,0x59,0x5A,0x63,0x64,0x65,0x66,0x67,0x68,0x69,0x6A,0x73,0x74,0x75,0x76,0x77,0x78,
0x79,0x7A,0x82,0x83,0x84,0x85,0x86,0x87,0x88,0x89,0x8A,0x92,0x93,0x94,0x95,0x96,0x97,0x98,
0x99,0x9A,0xA2,0xA3,0xA4,0xA5,0xA6,0xA7,0xA8,0xA9,0xAA,0xB2,0xB3,0xB4,0xB5,0xB6,0xB7,0xB8,
0xB9,0xBA,0xC2,0xC3,0xC4,0xC5,0xC6,0xC7,0xC8,0xC9,0xCA,0xD2,0xD3,0xD4,0xD5,0xD6,0xD7,0xD8,
0xD9,0xDA,0xE2,0xE3,0xE4,0xE5,0xE6,0xE7,0xE8,0xE9,0xEA,0xF2,0xF3,0xF4,0xF5,0xF6,0xF7,0xF8,
0xF9,0xFA
};

/* End DHT default */
#endif

/* End JPEG */
#endif

/* ------------------------------------------------------------------------------ */
//
// Defined IC types
//

#ifndef NONEWIC

#ifndef ICTYPE_VIDEO
#define ICTYPE_VIDEO    mmioFOURCC('v', 'i', 'd', 'c')
#define ICTYPE_AUDIO    mmioFOURCC('a', 'u', 'd', 'c')
#endif

#endif
/*
//   Misc. FOURCC registration
*/

/* Sierra Semiconductor: RDSP- Proprietary RIFF file format
//       for the storage and downloading of DSP
//       code for Audio and communications devices.
*/
#define FOURCC_RDSP mmioFOURCC('R', 'D', 'S', 'P')

#ifndef MMNOMIXER
#define MIXERCONTROL_CONTROLTYPE_SRS_MTS                (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 6)
#define MIXERCONTROL_CONTROLTYPE_SRS_ONOFF              (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 7)
#define MIXERCONTROL_CONTROLTYPE_SRS_SYNTHSELECT        (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 8)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#if !defined( RC_INVOKED ) && defined( _MSC_VER )
#if _MSC_VER <= 800
#pragma pack(1)
#else
#include "poppack.h"    /* Revert to default packing */
#endif
#endif  /* RC_INVOKED */

#ifdef __cplusplus
}                       /* End of extern "C" { */
#endif  /* __cplusplus */

#endif  /* _INC_MMREG */

#else /* __midl */

cpp_quote("#if 0")
#pragma pack(push, 1)
typedef struct tWAVEFORMATEX
{

    WORD  wFormatTag;
    WORD  nChannels;
    DWORD nSamplesPerSec;
    DWORD nAvgBytesPerSec;
    WORD  nBlockAlign;
    WORD  wBitsPerSample;
    WORD  cbSize;
    [size_is(cbSize)] BYTE pExtraBytes[];
} WAVEFORMATEX, *PWAVEFORMATEX, *NPWAVEFORMATEX, *LPWAVEFORMATEX;

typedef struct {
    WORD  wFormatTag;
    WORD  nChannels;
    DWORD nSamplesPerSec;
    DWORD nAvgBytesPerSec;
    WORD  nBlockAlign;
    WORD  wBitsPerSample;
    WORD  cbSize;
    WORD  wValidBitsPerSample;
    DWORD dwChannelMask;
    GUID  SubFormat;
} WAVEFORMATEXTENSIBLE, *PWAVEFORMATEXTENSIBLE;

#pragma pack(pop)  /* Assume byte packing throughout */
cpp_quote("#endif /* 0 */")

#endif /* __midl */


