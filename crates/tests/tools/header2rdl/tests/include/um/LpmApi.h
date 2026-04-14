

/*++

Copyright (c) 1998-1999  Microsoft Corporation

Module Name:

    LPMAPI.H - Include file for Local Policy Module

Abstract:

    This module defines the LPM structures and types.

Revision History:

    There is no support for ACS in Windows XP or later versions of Windows.

--*/

/****************************************************************************

            RSVPD -- ReSerVation Protocol Daemon

                USC Information Sciences Institute
                Marina del Rey, California

        Original Version: Shai Herzog, Nov. 1993.
        Current Version:  Steven Berson & Bob Braden, may 1996.

  Copyright (c) 1996 by the University of Southern California
  All rights reserved.

  Permission to use, copy, modify, and distribute this software and its
  documentation in source and binary forms for any purpose and without
  fee is hereby granted, provided that both the above copyright notice
  and this permission notice appear in all copies, and that any
  documentation, advertising materials, and other materials related to
  such distribution and use acknowledge that the software was developed
  in part by the University of Southern California, Information
  Sciences Institute.  The name of the University may not be used to
  endorse or promote products derived from this software without
  specific prior written permission.

  THE UNIVERSITY OF SOUTHERN CALIFORNIA makes no representations about
  the suitability of this software for any purpose.  THIS SOFTWARE IS
  PROVIDED "AS IS" AND WITHOUT ANY EXPRESS OR IMPLIED WARRANTIES,
  INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF
  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.

  Other copyrights might apply to parts of this software and are so
  noted when applicable.

********************************************************************/

#ifndef __LPMAPI_H_
#define __LPMAPI_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C"
{
#endif


#ifndef CALLBACK
#define CALLBACK __stdcall
#endif

#ifndef APIENTRY
#define APIENTRY FAR __stdcall
#endif

/*
 *  Standard format of an RSVP object header
 */
typedef struct {

    USHORT  obj_length; /* Length in bytes */
    UCHAR   obj_class;  /* Class (values defined below) */
    UCHAR   obj_ctype;  /* C-Type (values defined below) */

} RsvpObjHdr;

#define ObjLength(x)   ((RsvpObjHdr *)x)->obj_length
#define ObjCType(x)    ((RsvpObjHdr *)x)->obj_ctype
#define ObjClass(x)    ((RsvpObjHdr *)x)->obj_class
#define ObjData(x)     ((RsvpObjHdr *)(x)+1)

/*
 *  Define object classes: Class-Num values
 */
#define class_NULL              0
#define class_SESSION           1
#define class_SESSION_GROUP     2
#define class_RSVP_HOP          3
#define class_INTEGRITY         4
#define class_TIME_VALUES       5
#define class_ERROR_SPEC        6
#define class_SCOPE             7
#define class_STYLE             8
#define class_FLOWSPEC          9   // these two are the same
#define class_IS_FLOWSPEC       9  // since we added IS in front of the name
#define class_FILTER_SPEC       10
#define class_SENDER_TEMPLATE   11
#define class_SENDER_TSPEC      12
#define class_ADSPEC            13
#define class_POLICY_DATA       14
#define class_CONFIRM           15
#define class_MAX               15

/*
 *  RSVP SESSION object
 */
#define ctype_SESSION_ipv4      1
#define ctype_SESSION_ipv4GPI   3   /* IPSEC: Generalized Port Id */

#define SESSFLG_E_Police    0x01    /* E_Police: Entry policing flag*/

typedef struct {

    IN_ADDR sess_destaddr;  // DestAddress
    UCHAR   sess_protid;    // Protocol Id
    UCHAR   sess_flags;     // Use the flags defined above
    USHORT  sess_destport;  // DestPort

} Session_IPv4;

/*    GPI versions have virtual dest port instead of dest port; this
 *    changes the interpretation but not the format, so we do not
 *    define new structs for GPI.
 */

typedef struct {

    RsvpObjHdr          sess_header;

    union {

        Session_IPv4    sess_ipv4;

    }       sess_u;

} RSVP_SESSION;

// Useful defines to access components of SESSION obect
#define Sess4Addr       sess_u.sess_ipv4.sess_destaddr
#define Sess4Port       sess_u.sess_ipv4.sess_destport
#define Sess4Protocol   sess_u.sess_ipv4.sess_protid
#define Sess4Flags      sess_u.sess_ipv4.sess_flags

/*
 *  RSVP HOP object
 */
#define ctype_RSVP_HOP_ipv4 1

typedef struct {

    IN_ADDR     hop_ipaddr; // Next/Previous Hop Address
    ULONG       hop_LIH;        // Logical Interface Handle

} Rsvp_Hop_IPv4;

typedef struct {

    RsvpObjHdr          hop_header;

    union {

        Rsvp_Hop_IPv4   hop_ipv4;

    } hop_u;

} RSVP_HOP;

#define Hop4LIH    hop_u.hop_ipv4.hop_LIH
#define Hop4Addr   hop_u.hop_ipv4.hop_ipaddr

/*
 *  RSVP STYLE object
 */

//  Define values for option vector

#define Opt_Share_mask  0x00000018  // 2 bits: Sharing control
#define Opt_Distinct    0x00000008  // Distinct reservations
#define Opt_Shared      0x00000010  // Shared reservations

#define Opt_SndSel_mask 0x00000007  // 3 bits: Sender selection
#define Opt_Wildcard    0x00000001  // Wildcard scope
#define Opt_Explicit    0x00000002  // Explicit scope

#define Style_is_Wildcard(p)    (((p)&Opt_SndSel_mask) == Opt_Wildcard)
#define Style_is_Shared(p)      (((p)&Opt_Share_mask) == Opt_Shared)

//  Define style values
#define STYLE_WF    Opt_Shared + Opt_Wildcard
#define STYLE_FF    Opt_Distinct + Opt_Explicit
#define STYLE_SE    Opt_Shared + Opt_Explicit

#define ctype_STYLE 1

typedef struct {

    RsvpObjHdr  style_header;

    ULONG       style_word;

} RESV_STYLE;

/*
 *  RSVP FILTER SPEC object
 */
#define ctype_FILTER_SPEC_ipv4      1   // IPv4 FILTER_SPEC
#define ctype_FILTER_SPEC_ipv4GPI   4   // IPv4/GPI FILTER_SPEC

typedef struct {

    IN_ADDR filt_ipaddr;    // IPv4 SrcAddress
    USHORT  filt_unused;
    USHORT  filt_port;      // SrcPort

} Filter_Spec_IPv4;

typedef struct {

    IN_ADDR filt_ipaddr;    // IPv4 SrcAddress
    ULONG   filt_gpi;       // Generalized Port Id

} Filter_Spec_IPv4GPI;

typedef struct {

    RsvpObjHdr              filt_header;

    union {

        Filter_Spec_IPv4    filt_ipv4;
        Filter_Spec_IPv4GPI filt_ipv4gpi;

    } filt_u;

} FILTER_SPEC;

#define FilterSrcaddr   filt_u.filt_ipv4.filt_ipaddr
#define FilterSrcport   filt_u.filt_ipv4.filt_port

/*
 *  RSVP SENDER_TEMPLATE object
 */
#define ctype_SENDER_TEMPLATE_ipv4      1   // IPv4 SENDER_TEMPLATE
#define ctype_SENDER_TEMPLATE_ipv4GPI   4   // IPv4/GPI SENDER_TEMPLATE

typedef FILTER_SPEC  SENDER_TEMPLATE;       // Identical to FILTER_SPEC

/*
 *  RSVP SCOPE object class
 */
#define ctype_SCOPE_list_ipv4       1

typedef struct {

    IN_ADDR     scopl_ipaddr[1];        // var-len list of IP sender addrs

} Scope_list_ipv4;

typedef struct {

    RsvpObjHdr          scopl_header;

    union {

        Scope_list_ipv4 scopl_ipv4;

    } scope_u;

} RSVP_SCOPE;

#define Scope4Addr      scope_u.scopl_ipv4.scopl_ipaddr
#define ScopeCnt(scp)   ((ObjLength(scp)-sizeof(RsvpObjHdr))/sizeof(struct in_addr))
#define ScopeLen(cnt)   (cnt*sizeof(struct in_addr)+sizeof(RsvpObjHdr))


/*
 *  ERROR_SPEC object class
 */
#define ctype_ERROR_SPEC_ipv4   1

typedef struct {
    struct in_addr  errs_errnode;   /* Error Node Address       */
    u_char      errs_flags; /* Flags:           */
#define ERROR_SPECF_InPlace 0x01    /*   Left resv in place     */
#define ERROR_SPECF_NotGuilty   0x02    /*   This rcvr not guilty   */

    UCHAR       errs_code;  /* Error Code (def'd below) */
    USHORT      errs_value; /* Error Value      */
#define ERR_FORWARD_OK  0x8000      /* Flag: OK to forward state */
#define Error_Usage(x)  (((x)>>12)&3)
#define ERR_Usage_globl 0x00        /* Globally-defined sub-code */
#define ERR_Usage_local 0x10        /* Locally-defined sub-code */
#define ERR_Usage_serv  0x11        /* Service-defined sub-code */
#define ERR_global_mask 0x0fff      /* Sub-code bits in Error Val */

}    Error_Spec_IPv4;


typedef struct {

    RsvpObjHdr  errs_header;

    union {

        Error_Spec_IPv4 errs_ipv4;

    } errs_u;

}    ERROR_SPEC;

#define errspec4_enode  errs_u.errs_ipv4.errs_errnode
#define errspec4_code   errs_u.errs_ipv4.errs_code
#define errspec4_value  errs_u.errs_ipv4.errs_value
#define errspec4_flags  errs_u.errs_ipv4.errs_flags


/*
 *  POLICY_DATA object class
 *
 *      Contents are Opaque RSVP/SBM
 */
#define ctype_POLICY_DATA   1

typedef struct {

    RsvpObjHdr      PolicyObjHdr;

    USHORT          usPeOffset;     // Offset to the start of Policy Elements
                                    // from the begining of Policy Data

    USHORT          usReserved;

} POLICY_DATA;

#define PD_HDR_LEN  sizeof(POLICY_DATA)

typedef struct {

    USHORT      usPeLength;     // Policy Element length

    USHORT      usPeType;       // Policy Element type

    UCHAR       ucPeData[4];    // Just a place holder to the start of
                                // Policy Element data
} POLICY_ELEMENT;

#define PE_HDR_LEN  (2 * sizeof(USHORT))

/**************************************************************************
 *
 *  Int-Serv Data Structures
 *
 **************************************************************************/

/*
 *  Service numbers
 */
#define GENERAL_INFO            1
#define GUARANTEED_SERV         2
#define PREDICTIVE_SERV         3
#define CONTROLLED_DELAY_SERV   4
#define CONTROLLED_LOAD_SERV    5
#define QUALITATIVE_SERV        6

/*
 *  Well-known parameter IDs
 */
enum  int_serv_wkp {
    IS_WKP_HOP_CNT =        4,
    IS_WKP_PATH_BW =        6,
    IS_WKP_MIN_LATENCY =    8,
    IS_WKP_COMPOSED_MTU =   10,
    IS_WKP_TB_TSPEC =       127, /* Token-bucket TSPEC parm */
    IS_WKP_Q_TSPEC =        128
};


/*
 *  Int-serv Main header
 */
typedef struct {

    UCHAR   ismh_version;   // Version
    UCHAR   ismh_unused;
    USHORT  ismh_len32b;    // # 32-bit words excluding this hdr

} IntServMainHdr;

#define INTSERV_VERS_MASK   0xf0
#define INTSERV_VERSION0    0
#define Intserv_Version(x)      (((x)&INTSERV_VERS_MASK)>>4)
#define Intserv_Version_OK(x)   (((x)->ismh_version&INTSERV_VERS_MASK)== \
                                INTSERV_VERSION0)

// Convert ishm_length to equivalent RSVP object size, for checking
#define Intserv_Obj_size(x) (((IntServMainHdr *)(x))->ismh_len32b * 4 + \
                            sizeof(IntServMainHdr) + sizeof(RsvpObjHdr))

/*
 *  Int-serv Service Element Header
 */

// Flag: Break bit
#define ISSH_BREAK_BIT    0x80

typedef struct {

    UCHAR       issh_service;   // Service number
    UCHAR       issh_flags;     // Flag byte
    USHORT      issh_len32b;    // #32-bit words excluding this hdr

}  IntServServiceHdr;

#define Issh_len32b(p)  ((p)->issh_len32b)

/*
 *  Int-serv Parameter Element Header
 */
#define ISPH_FLG_INV    0x80        // Flag: Invalid

typedef struct {

    UCHAR       isph_parm_num;  // Parameter number
    UCHAR       isph_flags;     // Flags
    USHORT      isph_len32b;    // #32-bit words excluding this hdr

}  IntServParmHdr;

#define Next_Main_Hdr(p)   (IntServMainHdr *)((ULONG *)(p)+1+(p)->ismh_len32b)
#define Next_Serv_Hdr(p)   (IntServServiceHdr *)((ULONG *)(p)+1+(p)->issh_len32b)
#define Next_Parm_Hdr(p)   (IntServParmHdr *)((ULONG *)(p)+1+(p)->isph_len32b)

/*
 *  Generic Tspec Parameters
 */
typedef struct {

    FLOAT       TB_Tspec_r;     // Token bucket rate (B/sec)
    FLOAT       TB_Tspec_b;     // Token bucket depth (B)
    FLOAT       TB_Tspec_p;     // Peak data rate (B/sec)
    ULONG       TB_Tspec_m;     // Min Policed Unit (B)
    ULONG       TB_Tspec_M;     // Max pkt size (B)

} GenTspecParms;

/*
 *  Generic Tspec
 */
typedef struct {

    IntServServiceHdr   gen_Tspec_serv_hdr; // (GENERAL_INFO, length)

    IntServParmHdr      gen_Tspec_parm_hdr; // (IS_WKP_TB_TSPEC)

    GenTspecParms       gen_Tspec_parms;

} GenTspec;

#define gtspec_r        gen_Tspec_parms.TB_Tspec_r
#define gtspec_b        gen_Tspec_parms.TB_Tspec_b
#define gtspec_m        gen_Tspec_parms.TB_Tspec_m
#define gtspec_M        gen_Tspec_parms.TB_Tspec_M
#define gtspec_p        gen_Tspec_parms.TB_Tspec_p
#define gtspec_parmno   gen_Tspec_parm_hdr.isph_parm_num
#define gtspec_flags    gen_Tspec_parm_hdr.isph_flags

#define gtspec_len      (sizeof(GenTspec) - sizeof(IntServServiceHdr))


/* contents of qualitative tspec */

typedef struct {

    ULONG       TB_Tspec_M;     // Max pkt size (M)

} QualTspecParms;


typedef struct {

    IntServServiceHdr   qual_Tspec_serv_hdr; // (QUALITATIVE_SERV, length)

    IntServParmHdr      qual_Tspec_parm_hdr; // (IS_WKP_Q_TSPEC)

    QualTspecParms      qual_Tspec_parms;

} QualTspec;

typedef struct {

    IntServServiceHdr   Q_spec_serv_hdr;    // (QUALITATIVE_SERV,0,len)

    IntServParmHdr      Q_spec_parm_hdr;    // (IS_WKP_Q_TSPEC)

    QualTspecParms      Q_spec_parms;       // QUALITATIVE Tspec parameters

}  QualAppFlowSpec;

#define QAspec_M        Q_spec_parms.TB_Tspec_M

/*
 *  Contents of int-serv Tspec
 */
typedef struct {

    IntServMainHdr  st_mh;

    union {

        GenTspec    gen_stspec; // Generic Tspec
        QualTspec   qual_stspec;

    } tspec_u;

} IntServTspecBody;

/*
 *  SENDER_TSPEC class object
 */
#define ctype_SENDER_TSPEC  2

typedef struct {

    RsvpObjHdr          stspec_header;

    IntServTspecBody    stspec_body;

} SENDER_TSPEC;

/*
 *  Controlled-Load Flowspec
 */
typedef struct {

    IntServServiceHdr   CL_spec_serv_hdr;    // (CONTROLLED_LOAD_SERV,0,len)

    IntServParmHdr      CL_spec_parm_hdr;    // (IS_WKP_TB_TSPEC)

    GenTspecParms       CL_spec_parms;       // GENERIC Tspec parameters

}  CtrlLoadFlowspec;

#define CLspec_r        CL_spec_parms.TB_Tspec_r
#define CLspec_b        CL_spec_parms.TB_Tspec_b
#define CLspec_p        CL_spec_parms.TB_Tspec_p
#define CLspec_m        CL_spec_parms.TB_Tspec_m
#define CLspec_M        CL_spec_parms.TB_Tspec_M
#define CLspec_parmno   CL_spec_parm_hdr.isph_parm_num
#define CLspec_flags    CL_spec_parm_hdr.isph_flags
#define CLspec_len32b   CL_spec_parm_hdr.isph_len32b

#define CLspec_len      (sizeof(CtrlLoadFlowspec) - sizeof(IntServServiceHdr))

/*  Service-specific Parameter IDs
 */
enum    {

    IS_GUAR_RSPEC =     130,

    GUAR_ADSPARM_C  =   131,
    GUAR_ADSPARM_D  =   132,
    GUAR_ADSPARM_Ctot = 133,
    GUAR_ADSPARM_Dtot = 134,
    GUAR_ADSPARM_Csum = 135,
    GUAR_ADSPARM_Dsum = 136

};

/*
 *  Guaranteed Rspec parameters
 */
typedef struct {

    FLOAT       Guar_R;         //  Guaranteed Rate B/s
    ULONG       Guar_S;         //  Slack term secs

} GuarRspec;

/*
 *  Guaranteed Flowspec
 */
typedef struct {

    IntServServiceHdr   Guar_serv_hdr;      // (GUARANTEED, 0, length)

    IntServParmHdr      Guar_Tspec_hdr;     // (IS_WKP_TB_TSPEC,)
    GenTspecParms       Guar_Tspec_parms;   // GENERIC Tspec parms

    IntServParmHdr      Guar_Rspec_hdr;     // (IS_GUAR_RSPEC)
    GuarRspec           Guar_Rspec;         // Guaranteed rate (B/sec)

}   GuarFlowSpec;

#define Gspec_r         Guar_Tspec_parms.TB_Tspec_r
#define Gspec_b         Guar_Tspec_parms.TB_Tspec_b
#define Gspec_p         Guar_Tspec_parms.TB_Tspec_p
#define Gspec_m         Guar_Tspec_parms.TB_Tspec_m
#define Gspec_M         Guar_Tspec_parms.TB_Tspec_M
#define Gspec_R         Guar_Rspec.Guar_R
#define Gspec_S         Guar_Rspec.Guar_S
#define Gspec_T_parmno  Guar_Tspec_hdr.isph_parm_num
#define Gspec_T_flags   Guar_Tspec_hdr.isph_flags
#define Gspec_R_parmno  Guar_Rspec_hdr.isph_parm_num
#define Gspec_R_flags   Guar_Rspec_hdr.isph_flags

#define Gspec_len       (sizeof(GuarFlowSpec) - sizeof(IntServServiceHdr))

/*
 *  Contents of int-serv flowspec
 */
typedef struct {

    IntServMainHdr          spec_mh;

    union {

        CtrlLoadFlowspec    CL_spec;   // Controlled-Load service

        GuarFlowSpec        G_spec;    // Guaranteed service
        
        QualAppFlowSpec     Q_spec;
        
    } spec_u;

}   IntServFlowSpec;

#define ISmh_len32b     spec_mh.ismh_len32b
#define ISmh_version    spec_mh.ismh_version
#define ISmh_unused     spec_mh.ismh_unused

/*
 *  Int-Serv FLOWSPEC object
 */
#define ctype_FLOWSPEC_Intserv0  2  // The int-serv flowspec (v.0)

typedef struct {

    RsvpObjHdr          flow_header;

    IntServFlowSpec     flow_body;

}IS_FLOWSPEC;


/*
 *  FLOW DESCRIPTOR
 */

typedef struct flow_desc {

    union {
        SENDER_TSPEC   *stspec;
        IS_FLOWSPEC    *isflow;
    } u1;

    union {
       SENDER_TEMPLATE *stemp;
       FILTER_SPEC     *fspec;
    } u2;

} FLOW_DESC;

#define FdSenderTspec       u1.stspec
#define FdIsFlowSpec        u1.isflow

#define FdSenderTemplate    u2.stemp
#define FdFilterSpec        u2.fspec

/*
 *  ADSPEC class object
 *
 *      Opaque to RSVP -- Contents defined in rapi_lib.h
 */
#define ctype_ADSPEC_INTSERV    2

/*
 *  Guaranteed service Adspec parameters -- fixed part
 */
typedef struct {

    IntServServiceHdr   Gads_serv_hdr;  // GUARANTEED, x, len
    
    IntServParmHdr      Gads_Ctot_hdr;  // GUAR_ADSPARM_Ctot
    ULONG               Gads_Ctot;
    
    IntServParmHdr      Gads_Dtot_hdr;  // (GUAR_ADSPARM_Dtot
    ULONG               Gads_Dtot;
    
    IntServParmHdr      Gads_Csum_hdr;  // GUAR_ADSPARM_Csum
    ULONG               Gads_Csum;
    
    IntServParmHdr      Gads_Dsum_hdr;  // GUAR_ADSPARM_Dsum
    ULONG               Gads_Dsum;
    
    /*
     *  May be followed by override general param values
     */
} Gads_parms_t;


/*
 *  General Path Characterization Parameters
 */
typedef struct {

    IntServServiceHdr   gen_parm_hdr;           // GENERAL_INFO, len

    IntServParmHdr      gen_parm_hopcnt_hdr;    // (IS_WKP_HOP_CNT
    ULONG               gen_parm_hopcnt;

    IntServParmHdr      gen_parm_pathbw_hdr;    // IS_WKP_PATH_BW
    FLOAT               gen_parm_path_bw;

    IntServParmHdr      gen_parm_minlat_hdr;    // IS_WKP_MIN_LATENCY
    ULONG               gen_parm_min_latency;

    IntServParmHdr      gen_parm_compmtu_hdr;   // IS_WKP_COMPOSED_MTU 
    ULONG               gen_parm_composed_MTU;
    
} GenAdspecParams;

/*
 *  Contents of (minimal) int-serv Adspec
 */
typedef struct {
    IntServMainHdr      adspec_mh;      // Main header
    
    GenAdspecParams     adspec_genparms;// General char parm fragment
    
    /*
     *  Followed by variable-length fragments for some or all
     *  services.  These can be minimal length fragments.
     */
     
} IS_ADSPEC_BODY;


#define GEN_ADSPEC_LEN (sizeof(Object_header) + sizeof(IS_adsbody_t ) )

typedef struct {

    RsvpObjHdr      adspec_header;
    
    IS_ADSPEC_BODY  adspec_body;    /* Defined in rapi_lib.h */

} ADSPEC;


// RSVP message types

#define RSVP_PATH       1
#define RSVP_RESV       2
#define RSVP_PATH_ERR   3
#define RSVP_RESV_ERR   4
#define RSVP_PATH_TEAR  5
#define RSVP_RESV_TEAR  6

/*  RSVP error codes
 */
#define RSVP_Err_NONE       0   /* No error (CONFIRM)       */
#define RSVP_Erv_Nonev      0   /*    No-error Error Value  */

/* Admission Control failure    */
#define RSVP_Err_ADMISSION  1

/* Globally-defined sub-codes for : Admission Control failure */
#define RSVP_Erv_Other      0   /* Unspecified cause        */
#define RSVP_Erv_DelayBnd   1   /* Cannot meet delay bound req  */
#define RSVP_Erv_Bandwidth  2   /* Insufficient bandwidth   */
#define RSVP_Erv_MTU        3   /* MTU in flowspec too large    */

// Microsoft specific error values
#define RSVP_Erv_Flow_Rate          0x8001
#define RSVP_Erv_Bucket_szie        0x8002
#define RSVP_Erv_Peak_Rate          0x8003
#define RSVP_Erv_Min_Policied_size  0x8004

/* Policy control failure   */
#define RSVP_Err_POLICY     2

// Policy error values from Identity draft
#define POLICY_ERRV_NO_MORE_INFO                1
#define POLICY_ERRV_UNSUPPORTED_CREDENTIAL_TYPE 2
#define POLICY_ERRV_INSUFFICIENT_PRIVILEGES     3
#define POLICY_ERRV_EXPIRED_CREDENTIALS         4
#define POLICY_ERRV_IDENTITY_CHANGED            5

// Microsoft specific policy error values

#define POLICY_ERRV_UNKNOWN                         0

#define POLICY_ERRV_GLOBAL_DEF_FLOW_COUNT           1
#define POLICY_ERRV_GLOBAL_GRP_FLOW_COUNT           2
#define POLICY_ERRV_GLOBAL_USER_FLOW_COUNT          3
#define POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_COUNT   4
#define POLICY_ERRV_SUBNET_DEF_FLOW_COUNT           5
#define POLICY_ERRV_SUBNET_GRP_FLOW_COUNT           6
#define POLICY_ERRV_SUBNET_USER_FLOW_COUNT          7
#define POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_COUNT   8

#define POLICY_ERRV_GLOBAL_DEF_FLOW_DURATION        9
#define POLICY_ERRV_GLOBAL_GRP_FLOW_DURATION        10
#define POLICY_ERRV_GLOBAL_USER_FLOW_DURATION       11
#define POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_DURATION 12
#define POLICY_ERRV_SUBNET_DEF_FLOW_DURATION        13
#define POLICY_ERRV_SUBNET_GRP_FLOW_DURATION        14
#define POLICY_ERRV_SUBNET_USER_FLOW_DURATION       15
#define POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_DURATION   16

#define POLICY_ERRV_GLOBAL_DEF_FLOW_RATE            17
#define POLICY_ERRV_GLOBAL_GRP_FLOW_RATE            18
#define POLICY_ERRV_GLOBAL_USER_FLOW_RATE           19
#define POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_RATE    20
#define POLICY_ERRV_SUBNET_DEF_FLOW_RATE            21
#define POLICY_ERRV_SUBNET_GRP_FLOW_RATE            22
#define POLICY_ERRV_SUBNET_USER_FLOW_RATE           23
#define POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_RATE    24

#define POLICY_ERRV_GLOBAL_DEF_PEAK_RATE            25
#define POLICY_ERRV_GLOBAL_GRP_PEAK_RATE            26
#define POLICY_ERRV_GLOBAL_USER_PEAK_RATE           27
#define POLICY_ERRV_GLOBAL_UNAUTH_USER_PEAK_RATE    28
#define POLICY_ERRV_SUBNET_DEF_PEAK_RATE            29
#define POLICY_ERRV_SUBNET_GRP_PEAK_RATE            30
#define POLICY_ERRV_SUBNET_USER_PEAK_RATE           31
#define POLICY_ERRV_SUBNET_UNAUTH_USER_PEAK_RATE    32

#define POLICY_ERRV_GLOBAL_DEF_SUM_FLOW_RATE        33
#define POLICY_ERRV_GLOBAL_GRP_SUM_FLOW_RATE        34
#define POLICY_ERRV_GLOBAL_USER_SUM_FLOW_RATE       35
#define POLICY_ERRV_GLOBAL_UNAUTH_USER_SUM_FLOW_RATE 36
#define POLICY_ERRV_SUBNET_DEF_SUM_FLOW_RATE        37
#define POLICY_ERRV_SUBNET_GRP_SUM_FLOW_RATE        38
#define POLICY_ERRV_SUBNET_USER_SUM_FLOW_RATE       39
#define POLICY_ERRV_SUBNET_UNAUTH_USER_SUM_FLOW_RATE 40

#define POLICY_ERRV_GLOBAL_DEF_SUM_PEAK_RATE        41
#define POLICY_ERRV_GLOBAL_GRP_SUM_PEAK_RATE        42
#define POLICY_ERRV_GLOBAL_USER_SUM_PEAK_RATE       43
#define POLICY_ERRV_GLOBAL_UNAUTH_USER_SUM_PEAK_RATE 44
#define POLICY_ERRV_SUBNET_DEF_SUM_PEAK_RATE        45
#define POLICY_ERRV_SUBNET_GRP_SUM_PEAK_RATE        46
#define POLICY_ERRV_SUBNET_USER_SUM_PEAK_RATE       47
#define POLICY_ERRV_SUBNET_UNAUTH_USER_SUM_PEAK_RATE 48

#define POLICY_ERRV_UNKNOWN_USER                    49
#define POLICY_ERRV_NO_PRIVILEGES                   50
#define POLICY_ERRV_EXPIRED_USER_TOKEN              51
#define POLICY_ERRV_NO_RESOURCES                    52
#define POLICY_ERRV_PRE_EMPTED                      53
#define POLICY_ERRV_USER_CHANGED                    54
#define POLICY_ERRV_NO_ACCEPTS                      55
#define POLICY_ERRV_NO_MEMORY                       56
#define POLICY_ERRV_CRAZY_FLOWSPEC                  57


// Other RSVP defined Error codes
#define RSVP_Err_NO_PATH        3   /* No path state for Resv   */
#define RSVP_Err_NO_SENDER      4   /* No sender info for Resv  */
#define RSVP_Err_BAD_STYLE      5   /* Conflicting style        */
#define RSVP_Err_UNKNOWN_STYLE  6   /* Unknown reservation style    */
#define RSVP_Err_BAD_DSTPORT    7   /* Conflicting DstPort in Sess  */
#define RSVP_Err_BAD_SNDPORT    8   /* Conflicting Sender port  */
#define RSVP_Err_AMBIG_FILTER   9   /* Ambiguous Filter spec in Resv*/

#define RSVP_Err_PREEMPTED      12  /* Service Preempted        */

/* Unknown object Class-Num */
#define RSVP_Err_UNKN_OBJ_CLASS 13
/*   ErrVal = Class_num, CType  */

 /* Unknown object C-Type    */
#define RSVP_Err_UNKNOWN_CTYPE  14
/*   ErrVal = Class_num, CType  */

#define RSVP_Err_API_ERROR      20  /* API client error     */
/*   ErrVal = API error code    */

/* Traffic Control error    */
#define RSVP_Err_TC_ERROR       21

/* Globally-defined sub-codes for : Traffic Control errors */

#define RSVP_Erv_Conflict_Serv  01  /* Service Conflict     */
#define RSVP_Erv_No_Serv        02  /* Unknown Service      */
#define RSVP_Erv_Crazy_Flowspec 03  /* Unreasonable Flowspec    */
#define RSVP_Erv_Crazy_Tspec    04  /* Unreasonable Tspec       */

#define RSVP_Err_TC_SYS_ERROR   22  /* Traffic control system error */
      /* ErrVal = kernel error code   */


/* RSVP System error      */
#define RSVP_Err_RSVP_SYS_ERROR 23

/* Globally-defined sub-codes for : RSVP system errors */
#define RSVP_Erv_MEMORY         1   /* Out of memory */
#define RSVP_Erv_API            2   /* API logic error */

// Identity Policy elements related defines

// Reseved Identity PE types
#define LPM_PE_USER_IDENTITY    2
#define LPM_PE_APP_IDENTITY     3

// Defines for Identity error values
#define ERROR_NO_MORE_INFO          1
#define UNSUPPORTED_CREDENTIAL_TYPE 2
#define INSUFFICIENT_PRIVILEGES     3
#define EXPIRED_CREDENTIAL          4
#define IDENTITY_CHANGED            5


typedef struct {

    USHORT      usIdErrLength;

    UCHAR       ucAType;

    UCHAR       ucSubType;

    USHORT      usReserved;

    USHORT      usIdErrorValue;

    UCHAR       ucIdErrData[4];

} ID_ERROR_OBJECT;

    
#define ID_ERR_OBJ_HDR_LEN     (sizeof(ID_ERROR_OBJECT) - 4 * sizeof(UCHAR) )


/*

    LPM API specific definitions

*/

/**************************************

    LPM_Initialize

***************************************/

DECLARE_HANDLE(LPM_HANDLE);

DECLARE_HANDLE(RHANDLE);

typedef ULONG   LPV;

typedef USHORT  PETYPE;

#define LPM_OK  0

typedef int     MSG_TYPE;

typedef struct rsvpmsgobjs {

    MSG_TYPE        RsvpMsgType;
    
    RSVP_SESSION    *pRsvpSession;
    
    RSVP_HOP        *pRsvpFromHop;
    
    RSVP_HOP        *pRsvpToHop;
    
    RESV_STYLE      *pResvStyle;
    
    RSVP_SCOPE      *pRsvpScope;
    
    int             FlowDescCount;
    
    FLOW_DESC       *pFlowDescs;
    
    int             PdObjectCount;
    
    POLICY_DATA     **ppPdObjects;
    
    ERROR_SPEC      *pErrorSpec;

    ADSPEC          *pAdspec;
    
} RSVP_MSG_OBJS;

#if DBG
typedef void *
(APIENTRY * PALLOCMEM) ( DWORD Size, char *szFileName, DWORD nLine );
#else
typedef void *
(APIENTRY * PALLOCMEM) ( DWORD Size );
#endif

#if DBG
typedef void
(APIENTRY * PFREEMEM) ( void *pv, char *szFileName, DWORD nLine );
#else
typedef void
(APIENTRY * PFREEMEM) ( void *pv );
#endif

typedef struct policy_decision
{
    LPV             lpvResult;        // Use the LPV values from above
    
    WORD            wPolicyErrCode;   // RSVP defined error codes
    
    WORD            wPolicyErrValue;  // RSVP defined error values
    
} POLICY_DECISION;

typedef
ULONG *
(CALLBACK * CBADMITRESULT) (    
        LPM_HANDLE      LpmHandle,
        
        RHANDLE         RequestHandle,

        ULONG           ulPcmActionFlags,
    
        int             LpmError,
        
        int             PolicyDecisionsCount,
        
        POLICY_DECISION *pPolicyDecisions );

typedef
ULONG *
(CALLBACK * CBGETRSVPOBJECTS) (  

        LPM_HANDLE  LpmHandle,
        
        RHANDLE     RequestHandle,
        
        int         LpmError,
        
        int         RsvpObjectsCount,

        RsvpObjHdr  **ppRsvpObjects );

// The above 2 call backs can return the following errors

#define INV_LPM_HANDLE      1       // Supplied LpmHandle is invalid
#define LPM_TIME_OUT        2       // LPM has returned results after the time limit
#define INV_REQ_HANDLE      3       // Supplied Request handle is invalid
#define DUP_RESULTS         4       // LPM has already returned results for this request
#define INV_RESULTS         5       // Results supplied are invalid

typedef struct lpminitinfo {

    DWORD           PcmVersionNumber;
    
    DWORD           ResultTimeLimit;
    
    int             ConfiguredLpmCount;
    
    PALLOCMEM       AllocMemory;
    
    PFREEMEM        FreeMemory;
    
    CBADMITRESULT   PcmAdmitResultCallback;
    
    CBGETRSVPOBJECTS GetRsvpObjectsCallback;
    
} LPM_INIT_INFO;

// Valid PE types
// XXX ISSUE - Is 0xFFFF a better choice?
#define LPM_PE_ALL_TYPES        0

// Current LPM API version number
#define LPM_API_VERSION_1        1

// Current PCM version number
#define PCM_VERSION_1    1

ULONG
APIENTRY
LPM_Initialize (

    IN  LPM_HANDLE      LpmHandle,
    
    IN  LPM_INIT_INFO   *pLpmInitInfo,
    
    OUT DWORD           *pLpmVersionNumber,
    
    OUT PETYPE          *pSupportedPeType,
    
    OUT VOID            *Reserved );

/**************************************

    LPM_Deiitialize
    
***************************************/

ULONG
APIENTRY
LPM_Deinitialize (    
    IN  LPM_HANDLE      LpmHandle );
            

/**************************************

    LPM_AdmitRsvpMsg

***************************************/

// Valid LPV - LPM Priority Values
#define LPV_RESERVED        0
#define LPV_MIN_PRIORITY    1
#define LPV_MAX_PRIORITY    0xFF00
#define LPV_DROP_MSG        0xFFFD
#define LPV_DONT_CARE       0xFFFE
#define LPV_REJECT          0xFFFF

// Valid values for PcmActionFlags
#define FORCE_IMMEDIATE_REFRESH         1

// Function return values for LPM_AdmitResvMsg
#define LPM_RESULT_READY    0
#define LPM_RESULT_DEFER    1

ULONG
APIENTRY
LPM_AdmitRsvpMsg (

    IN  RHANDLE         PcmReqHandle,
    
    IN  RSVP_HOP        *pRecvdIntf,
    
    IN  RSVP_MSG_OBJS   *pRsvpMsgObjs,
    
    IN  int             RcvdRsvpMsgLength,
    
    IN  UCHAR           *RcvdRsvpMsg,

    OUT ULONG           *pulPcmActionFlags,
    
    OUT POLICY_DECISION *pPolicyDecisions,
    
    OUT void            *Reserved );


/**************************************

    LPM_GetRsvpObjects

***************************************/

// Function return values are defined in LPM_AdmitResvMsg section

ULONG
APIENTRY
LPM_GetRsvpObjects (

    IN  RHANDLE         PcmReqHandle,
    
    IN  ULONG           MaxPdSize,
    
    IN  RSVP_HOP        *SendingIntfAddr,
    
    IN  RSVP_MSG_OBJS   *pRsvpMsgObjs,
    
    OUT int             *pRsvpObjectsCount,

    OUT RsvpObjHdr      ***pppRsvpObjects,
    
    OUT void            *Reserved );


/**************************************

    LPM_DeleteState

***************************************/

// TearDown reasons

#define RCVD_PATH_TEAR      1
#define RCVD_RESV_TEAR      2
#define ADM_CTRL_FAILED     3
#define STATE_TIMEOUT       4
#define FLOW_DURATION       5


VOID
APIENTRY
LPM_DeleteState(

    IN  RSVP_HOP        *pRcvdIfAddr,
    
    IN  MSG_TYPE        RsvpMsgType,
    
    IN  RSVP_SESSION    *pRsvpSession,
    
    IN  RSVP_HOP        *pRsvpFromHop,
    
    IN  RESV_STYLE      *pResvStyle,
    
    IN  int             FilterSpecCount,
    
    IN  FILTER_SPEC     **ppFilterSpecList,
    
    IN  int             TearDownReason );

/**************************************

    LPM_IpAddrTable

***************************************/

typedef struct lpmiptable {

    ULONG       ulIfIndex;  // SNMP index for this interface

    ULONG       MediaType;  // As defined in IPIFCONS.H

    IN_ADDR     IfIpAddr;   // Interface IP address

    IN_ADDR     IfNetMask;  // Interface subnet mask

} LPMIPTABLE;

BOOL
APIENTRY
LPM_IpAddressTable (

    IN  ULONG       cIpAddrTable,
    
    IN  LPMIPTABLE  *pIpAddrTable );


/**************************************

    LPM_CommitResv

***************************************/

// CommitDecision values

#define RESOURCES_ALLOCATED             1
#define RESOURCES_MODIFIED              2

VOID
APIENTRY
LPM_CommitResv (

    IN  RSVP_SESSION    *RsvpSession,
    
    IN  RSVP_HOP        *FlowInstalledIntf,
    
    IN  RESV_STYLE      *RsvpStyle,
    
    IN  int             FilterSpecCount,
    
    IN  FILTER_SPEC     **ppFilterSpecList,
    
    IN  IS_FLOWSPEC     *pMergedFlowSpec,
    
    IN  ULONG           CommitDecision );


#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __LPMAPI_H_



