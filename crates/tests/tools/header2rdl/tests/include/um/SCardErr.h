/*
 scarderr.mc

   Error message codes from the Smart Card Resource Manager
   These messages must be reconciled with winerror.w
   They exist here to provide error messages on pre-Win2K systems.

*/
#ifndef SCARD_S_SUCCESS
//
// =============================
// Facility SCARD Error Messages
// =============================
//
#define SCARD_S_SUCCESS NO_ERROR
//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +-+-+-+-+-+---------------------+-------------------------------+
//  |S|R|C|N|r|    Facility         |               Code            |
//  +-+-+-+-+-+---------------------+-------------------------------+
//
//  where
//
//      S - Severity - indicates success/fail
//
//          0 - Success
//          1 - Fail (COERROR)
//
//      R - reserved portion of the facility code, corresponds to NT's
//              second severity bit.
//
//      C - reserved portion of the facility code, corresponds to NT's
//              C field.
//
//      N - reserved portion of the facility code. Used to indicate a
//              mapped NT status value.
//
//      r - reserved portion of the facility code. Reserved for internal
//              use. Used to indicate HRESULT values that are not status
//              values, but are instead message ids for display strings.
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//
#define FACILITY_SYSTEM                  0x0
#define FACILITY_SCARD                   0x10


//
// Define the severity codes
//
#define STATUS_SEVERITY_INFORMATIONAL    0x1
#define STATUS_SEVERITY_WARNING          0x2
#define STATUS_SEVERITY_ERROR            0x3


//
// MessageId: SCARD_F_INTERNAL_ERROR
//
// MessageText:
//
// An internal consistency check failed.
//
#define SCARD_F_INTERNAL_ERROR           ((DWORD)0x80100001L)

//
// MessageId: SCARD_E_CANCELLED
//
// MessageText:
//
// The action was cancelled by an SCardCancel request.
//
#define SCARD_E_CANCELLED                ((DWORD)0x80100002L)

//
// MessageId: SCARD_E_INVALID_HANDLE
//
// MessageText:
//
// The supplied handle was invalid.
//
#define SCARD_E_INVALID_HANDLE           ((DWORD)0x80100003L)

//
// MessageId: SCARD_E_INVALID_PARAMETER
//
// MessageText:
//
// One or more of the supplied parameters could not be properly interpreted.
//
#define SCARD_E_INVALID_PARAMETER        ((DWORD)0x80100004L)

//
// MessageId: SCARD_E_INVALID_TARGET
//
// MessageText:
//
// Registry startup information is missing or invalid.
//
#define SCARD_E_INVALID_TARGET           ((DWORD)0x80100005L)

//
// MessageId: SCARD_E_NO_MEMORY
//
// MessageText:
//
// Not enough memory available to complete this command.
//
#define SCARD_E_NO_MEMORY                ((DWORD)0x80100006L)

//
// MessageId: SCARD_F_WAITED_TOO_LONG
//
// MessageText:
//
// An internal consistency timer has expired.
//
#define SCARD_F_WAITED_TOO_LONG          ((DWORD)0x80100007L)

//
// MessageId: SCARD_E_INSUFFICIENT_BUFFER
//
// MessageText:
//
// The data buffer to receive returned data is too small for the returned data.
//
#define SCARD_E_INSUFFICIENT_BUFFER      ((DWORD)0x80100008L)

//
// MessageId: SCARD_E_UNKNOWN_READER
//
// MessageText:
//
// The specified reader name is not recognized.
//
#define SCARD_E_UNKNOWN_READER           ((DWORD)0x80100009L)

//
// MessageId: SCARD_E_TIMEOUT
//
// MessageText:
//
// The user-specified timeout value has expired.
//
#define SCARD_E_TIMEOUT                  ((DWORD)0x8010000AL)

//
// MessageId: SCARD_E_SHARING_VIOLATION
//
// MessageText:
//
// The smart card cannot be accessed because of other connections outstanding.
//
#define SCARD_E_SHARING_VIOLATION        ((DWORD)0x8010000BL)

//
// MessageId: SCARD_E_NO_SMARTCARD
//
// MessageText:
//
// The operation requires a smart card, but no smart card is currently in the device.
//
#define SCARD_E_NO_SMARTCARD             ((DWORD)0x8010000CL)

//
// MessageId: SCARD_E_UNKNOWN_CARD
//
// MessageText:
//
// The specified smart card name is not recognized.
//
#define SCARD_E_UNKNOWN_CARD             ((DWORD)0x8010000DL)

//
// MessageId: SCARD_E_CANT_DISPOSE
//
// MessageText:
//
// The system could not dispose of the media in the requested manner.
//
#define SCARD_E_CANT_DISPOSE             ((DWORD)0x8010000EL)

//
// MessageId: SCARD_E_PROTO_MISMATCH
//
// MessageText:
//
// The requested protocols are incompatible with the protocol currently in use with the smart card.
//
#define SCARD_E_PROTO_MISMATCH           ((DWORD)0x8010000FL)

//
// MessageId: SCARD_E_NOT_READY
//
// MessageText:
//
// The reader or smart card is not ready to accept commands.
//
#define SCARD_E_NOT_READY                ((DWORD)0x80100010L)

//
// MessageId: SCARD_E_INVALID_VALUE
//
// MessageText:
//
// One or more of the supplied parameters values could not be properly interpreted.
//
#define SCARD_E_INVALID_VALUE            ((DWORD)0x80100011L)

//
// MessageId: SCARD_E_SYSTEM_CANCELLED
//
// MessageText:
//
// The action was cancelled by the system, presumably to log off or shut down.
//
#define SCARD_E_SYSTEM_CANCELLED         ((DWORD)0x80100012L)

//
// MessageId: SCARD_F_COMM_ERROR
//
// MessageText:
//
// An internal communications error has been detected.
//
#define SCARD_F_COMM_ERROR               ((DWORD)0x80100013L)

//
// MessageId: SCARD_F_UNKNOWN_ERROR
//
// MessageText:
//
// An internal error has been detected, but the source is unknown.
//
#define SCARD_F_UNKNOWN_ERROR            ((DWORD)0x80100014L)

//
// MessageId: SCARD_E_INVALID_ATR
//
// MessageText:
//
// An ATR obtained from the registry is not a valid ATR string.
//
#define SCARD_E_INVALID_ATR              ((DWORD)0x80100015L)

//
// MessageId: SCARD_E_NOT_TRANSACTED
//
// MessageText:
//
// An attempt was made to end a non-existent transaction.
//
#define SCARD_E_NOT_TRANSACTED           ((DWORD)0x80100016L)

//
// MessageId: SCARD_E_READER_UNAVAILABLE
//
// MessageText:
//
// The specified reader is not currently available for use.
//
#define SCARD_E_READER_UNAVAILABLE       ((DWORD)0x80100017L)

//
// MessageId: SCARD_P_SHUTDOWN
//
// MessageText:
//
// The operation has been aborted to allow the server application to exit.
//
#define SCARD_P_SHUTDOWN                 ((DWORD)0x80100018L)

//
// MessageId: SCARD_E_PCI_TOO_SMALL
//
// MessageText:
//
// The PCI Receive buffer was too small.
//
#define SCARD_E_PCI_TOO_SMALL            ((DWORD)0x80100019L)

//
// MessageId: SCARD_E_READER_UNSUPPORTED
//
// MessageText:
//
// The reader driver does not meet minimal requirements for support.
//
#define SCARD_E_READER_UNSUPPORTED       ((DWORD)0x8010001AL)

//
// MessageId: SCARD_E_DUPLICATE_READER
//
// MessageText:
//
// The reader driver did not produce a unique reader name.
//
#define SCARD_E_DUPLICATE_READER         ((DWORD)0x8010001BL)

//
// MessageId: SCARD_E_CARD_UNSUPPORTED
//
// MessageText:
//
// The smart card does not meet minimal requirements for support.
//
#define SCARD_E_CARD_UNSUPPORTED         ((DWORD)0x8010001CL)

//
// MessageId: SCARD_E_NO_SERVICE
//
// MessageText:
//
// The Smart Card Resource Manager is not running.
//
#define SCARD_E_NO_SERVICE               ((DWORD)0x8010001DL)

//
// MessageId: SCARD_E_SERVICE_STOPPED
//
// MessageText:
//
// The Smart Card Resource Manager has shut down.
//
#define SCARD_E_SERVICE_STOPPED          ((DWORD)0x8010001EL)

//
// MessageId: SCARD_E_UNEXPECTED
//
// MessageText:
//
// An unexpected card error has occurred.
//
#define SCARD_E_UNEXPECTED               ((DWORD)0x8010001FL)

//
// MessageId: SCARD_E_ICC_INSTALLATION
//
// MessageText:
//
// No Primary Provider can be found for the smart card.
//
#define SCARD_E_ICC_INSTALLATION         ((DWORD)0x80100020L)

//
// MessageId: SCARD_E_ICC_CREATEORDER
//
// MessageText:
//
// The requested order of object creation is not supported.
//
#define SCARD_E_ICC_CREATEORDER          ((DWORD)0x80100021L)

//
// MessageId: SCARD_E_UNSUPPORTED_FEATURE
//
// MessageText:
//
// This smart card does not support the requested feature.
//
#define SCARD_E_UNSUPPORTED_FEATURE      ((DWORD)0x80100022L)

//
// MessageId: SCARD_E_DIR_NOT_FOUND
//
// MessageText:
//
// The identified directory does not exist in the smart card.
//
#define SCARD_E_DIR_NOT_FOUND            ((DWORD)0x80100023L)

//
// MessageId: SCARD_E_FILE_NOT_FOUND
//
// MessageText:
//
// The identified file does not exist in the smart card.
//
#define SCARD_E_FILE_NOT_FOUND           ((DWORD)0x80100024L)

//
// MessageId: SCARD_E_NO_DIR
//
// MessageText:
//
// The supplied path does not represent a smart card directory.
//
#define SCARD_E_NO_DIR                   ((DWORD)0x80100025L)

//
// MessageId: SCARD_E_NO_FILE
//
// MessageText:
//
// The supplied path does not represent a smart card file.
//
#define SCARD_E_NO_FILE                  ((DWORD)0x80100026L)

//
// MessageId: SCARD_E_NO_ACCESS
//
// MessageText:
//
// Access is denied to this file.
//
#define SCARD_E_NO_ACCESS                ((DWORD)0x80100027L)

//
// MessageId: SCARD_E_WRITE_TOO_MANY
//
// MessageText:
//
// The smart card does not have enough memory to store the information.
//
#define SCARD_E_WRITE_TOO_MANY           ((DWORD)0x80100028L)

//
// MessageId: SCARD_E_BAD_SEEK
//
// MessageText:
//
// There was an error trying to set the smart card file object pointer.
//
#define SCARD_E_BAD_SEEK                 ((DWORD)0x80100029L)

//
// MessageId: SCARD_E_INVALID_CHV
//
// MessageText:
//
// The supplied PIN is incorrect.
//
#define SCARD_E_INVALID_CHV              ((DWORD)0x8010002AL)

//
// MessageId: SCARD_E_UNKNOWN_RES_MNG
//
// MessageText:
//
// An unrecognized error code was returned from a layered component.
//
#define SCARD_E_UNKNOWN_RES_MNG          ((DWORD)0x8010002BL)

//
// MessageId: SCARD_E_NO_SUCH_CERTIFICATE
//
// MessageText:
//
// The requested certificate does not exist.
//
#define SCARD_E_NO_SUCH_CERTIFICATE      ((DWORD)0x8010002CL)

//
// MessageId: SCARD_E_CERTIFICATE_UNAVAILABLE
//
// MessageText:
//
// The requested certificate could not be obtained.
//
#define SCARD_E_CERTIFICATE_UNAVAILABLE  ((DWORD)0x8010002DL)

//
// MessageId: SCARD_E_NO_READERS_AVAILABLE
//
// MessageText:
//
// Cannot find a smart card reader.
//
#define SCARD_E_NO_READERS_AVAILABLE     ((DWORD)0x8010002EL)

//
// MessageId: SCARD_E_COMM_DATA_LOST
//
// MessageText:
//
// A communications error with the smart card has been detected.  Retry the operation.
//
#define SCARD_E_COMM_DATA_LOST           ((DWORD)0x8010002FL)

//
// MessageId: SCARD_E_NO_KEY_CONTAINER
//
// MessageText:
//
// The requested key container does not exist on the smart card.
//
#define SCARD_E_NO_KEY_CONTAINER         ((DWORD)0x80100030L)

//
// MessageId: SCARD_E_SERVER_TOO_BUSY
//
// MessageText:
//
// The Smart Card Resource Manager is too busy to complete this operation.
//
#define SCARD_E_SERVER_TOO_BUSY          ((DWORD)0x80100031L)

//
// MessageId: SCARD_E_PIN_CACHE_EXPIRED
//
// MessageText:
//
// The smart card PIN cache has expired.
//
#define SCARD_E_PIN_CACHE_EXPIRED        ((DWORD)0x80100032L)

//
// MessageId: SCARD_E_NO_PIN_CACHE
//
// MessageText:
//
// The smart card PIN cannot be cached.
//
#define SCARD_E_NO_PIN_CACHE             ((DWORD)0x80100033L)

//
// MessageId: SCARD_E_READ_ONLY_CARD
//
// MessageText:
//
// The smart card is read only and cannot be written to.
//
#define SCARD_E_READ_ONLY_CARD           ((DWORD)0x80100034L)

//
// These are warning codes.
//
//
// MessageId: SCARD_W_UNSUPPORTED_CARD
//
// MessageText:
//
// The reader cannot communicate with the smart card, due to ATR configuration conflicts.
//
#define SCARD_W_UNSUPPORTED_CARD         ((DWORD)0x80100065L)

//
// MessageId: SCARD_W_UNRESPONSIVE_CARD
//
// MessageText:
//
// The smart card is not responding to a reset.
//
#define SCARD_W_UNRESPONSIVE_CARD        ((DWORD)0x80100066L)

//
// MessageId: SCARD_W_UNPOWERED_CARD
//
// MessageText:
//
// Power has been removed from the smart card, so that further communication is not possible.
//
#define SCARD_W_UNPOWERED_CARD           ((DWORD)0x80100067L)

//
// MessageId: SCARD_W_RESET_CARD
//
// MessageText:
//
// The smart card has been reset, so any shared state information is invalid.
//
#define SCARD_W_RESET_CARD               ((DWORD)0x80100068L)

//
// MessageId: SCARD_W_REMOVED_CARD
//
// MessageText:
//
// The smart card has been removed, so that further communication is not possible.
//
#define SCARD_W_REMOVED_CARD             ((DWORD)0x80100069L)

//
// MessageId: SCARD_W_SECURITY_VIOLATION
//
// MessageText:
//
// Access was denied because of a security violation.
//
#define SCARD_W_SECURITY_VIOLATION       ((DWORD)0x8010006AL)

//
// MessageId: SCARD_W_WRONG_CHV
//
// MessageText:
//
// The card cannot be accessed because the wrong PIN was presented.
//
#define SCARD_W_WRONG_CHV                ((DWORD)0x8010006BL)

//
// MessageId: SCARD_W_CHV_BLOCKED
//
// MessageText:
//
// The card cannot be accessed because the maximum number of PIN entry attempts has been reached.
//
#define SCARD_W_CHV_BLOCKED              ((DWORD)0x8010006CL)

//
// MessageId: SCARD_W_EOF
//
// MessageText:
//
// The end of the smart card file has been reached.
//
#define SCARD_W_EOF                      ((DWORD)0x8010006DL)

//
// MessageId: SCARD_W_CANCELLED_BY_USER
//
// MessageText:
//
// The action was cancelled by the user.
//
#define SCARD_W_CANCELLED_BY_USER        ((DWORD)0x8010006EL)

//
// MessageId: SCARD_W_CARD_NOT_AUTHENTICATED
//
// MessageText:
//
// No PIN was presented to the smart card.
//
#define SCARD_W_CARD_NOT_AUTHENTICATED   ((DWORD)0x8010006FL)

//
// MessageId: SCARD_W_CACHE_ITEM_NOT_FOUND
//
// MessageText:
//
// The requested item could not be found in the cache.
//
#define SCARD_W_CACHE_ITEM_NOT_FOUND     ((DWORD)0x80100070L)

//
// MessageId: SCARD_W_CACHE_ITEM_STALE
//
// MessageText:
//
// The requested cache item is too old and was deleted from the cache.
//
#define SCARD_W_CACHE_ITEM_STALE         ((DWORD)0x80100071L)

//
// MessageId: SCARD_W_CACHE_ITEM_TOO_BIG
//
// MessageText:
//
// The new cache item exceeds the maximum per-item size defined for the cache.
//
#define SCARD_W_CACHE_ITEM_TOO_BIG       ((DWORD)0x80100072L)

#endif // SCARD_S_SUCCESS
