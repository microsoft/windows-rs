/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:        ADODEF.H
//
// Contents:    ADO version definition.
//          
// 
// Comments:
//
//-----------------------------------------------------------------------------

#ifndef _ADODEF_H_
#define _ADODEF_H_

// Change the version numbers below when there are any updates in the version of the type libraries, 
// which should be changed after some updates to the type libraries

// TYPELIB MAJOR VERSIONS
#define ADO_MAJOR           6
#define ADOR_MAJOR          6
#define ADOX_MAJOR          6
#define ADOMD_MAJOR         6
#define JRO_MAJOR           2

// TYPELIB MINOR VERSION
#define ADO_MINOR           1
#define ADOR_MINOR          0
#define ADOX_MINOR          0
#define ADOMD_MINOR         0
#define JRO_MINOR           6

#define ADO_VERSION         ADO_MAJOR##.##ADO_MINOR
#define ADOR_VERSION        ADOR_MAJOR##.##ADOR_MINOR
#define ADOX_VERSION        ADOX_MAJOR##.##ADOX_MINOR
#define ADOMD_VERSION       ADOMD_MAJOR##.##ADOMD_MINOR
#define JRO_VERSION         JRO_MAJOR##.##JRO_MINOR

#define ADO_LIBRARYNAME     "Microsoft ActiveX Data Objects 6.1 Library"
#define ADOR_LIBRARYNAME    "Microsoft ActiveX Data Objects Recordset 6.0 Library"
#define ADOX_LIBRARYNAME    "Microsoft ADO Ext. 6.0 for DDL and Security"
#define ADOMD_LIBRARYNAME   "Microsoft ActiveX Data Objects (Multi-dimensional) 6.0 Library"
#define JRO_LIBRARYNAME     "Microsoft Jet and Replication Objects 2.6 Library"

#define ADOMD_TYPELIB_UUID  uuid(22813728-8BD3-11D0-B4EF-00A0C9138CA4)
#define JRO_TYPELIB_UUID    uuid(AC3B8B4C-B6CA-11d1-9F31-00C04FC29D52)

#endif // _ADODEF_H_
