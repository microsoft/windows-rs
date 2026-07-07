// Fundamental typedefs as they appear in the Windows SDK system headers
// (winnt.h / windef.h / basetsd.h). They reach the scraper as *included*
// declarations, not main-file ones — exactly as they would in a real namespace
// build. The semantic ones are preserved by the Foundation seed reference; the
// universal C fundamentals (DWORD, UINT) collapse to primitives.
typedef int BOOL;
typedef unsigned char BOOLEAN;
typedef long HRESULT;
typedef long NTSTATUS;
typedef __int64 LPARAM;
typedef __int64 LRESULT;
typedef unsigned __int64 WPARAM;
typedef unsigned long DWORD;
typedef unsigned int UINT;
typedef void *HANDLE;
