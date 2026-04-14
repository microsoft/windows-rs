 
/*
 * Copyright (C) 2004 Microsoft Corporation
 */
#undef FACILITY_WINRM
// Define WSMAN specific error codes
//
//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-+-----------------------+-------------------------------+
//  |Sev|C|R|     Facility          |               Code            |
//  +---+-+-+-----------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      R - is a reserved bit
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//
#define FACILITY_WINRM                   0x33


//
// Define the severity codes
//


//
// MessageId: ERROR_WSMAN_RESOURCE_NOT_FOUND
//
// MessageText:
//
// The WS-Management service cannot process the request. The service cannot find the resource identified by the resource URI and selectors.
//
#define ERROR_WSMAN_RESOURCE_NOT_FOUND   0x80338000L

//
// MessageId: ERROR_WSMAN_INVALID_ACTIONURI
//
// MessageText:
//
// The WS-Management service cannot process the request. The WS-Addressing action URI is invalid. Check the documentation for information on how to construct an action URI.
//
#define ERROR_WSMAN_INVALID_ACTIONURI    0x80338001L

// Same text as ERROR_WSMAN_INVALID_RESOURCE_URI, but some code in R2 uses this error code
//
// MessageId: ERROR_WSMAN_INVALID_URI
//
// MessageText:
//
// The WS-Management service cannot process the request. The resource URI is missing or it has an incorrect format.
// Check the documentation or use the following command for information on how to construct a resource URI: "winrm help uris". 
//
#define ERROR_WSMAN_INVALID_URI          0x80338002L

//
// MessageId: ERROR_WSMAN_PROVIDER_FAILURE
//
// MessageText:
//
// An error was encountered inside the plugin.
//
#define ERROR_WSMAN_PROVIDER_FAILURE     0x80338003L

//
// MessageId: ERROR_WSMAN_BATCH_COMPLETE
//
// MessageText:
//
// The WS-Management service cannot complete the request. The WSManEnumerator object is full and no more items can be added.
//
#define ERROR_WSMAN_BATCH_COMPLETE       0x80338004L

//
// MessageId: ERROR_WSMAN_CONFIG_CORRUPTED
//
// MessageText:
//
// The WS-Management configuration is corrupted. Use the following command to restore defaults:
// %n%n
// winrm invoke Restore http://schemas.microsoft.com/wbem/wsman/1/config @{}
// %n%n
// Then add any custom configuration settings.
//
#define ERROR_WSMAN_CONFIG_CORRUPTED     0x80338005L

//
// MessageId: ERROR_WSMAN_PULL_IN_PROGRESS
//
// MessageText:
//
// The WS-Management service cannot process a pull request because a pull operation is already in progress.
//
#define ERROR_WSMAN_PULL_IN_PROGRESS     0x80338006L

//
// MessageId: ERROR_WSMAN_ENUMERATION_CLOSED
//
// MessageText:
//
// The WS-Management enumeration session is finished or cancelled and cannot be used. Start a new enumeration.
//
#define ERROR_WSMAN_ENUMERATION_CLOSED   0x80338007L

//
// MessageId: ERROR_WSMAN_SUBSCRIPTION_CLOSED
//
// MessageText:
//
// The event subscription is already closed and cannot be used. Start a new subscription.
//
#define ERROR_WSMAN_SUBSCRIPTION_CLOSED  0x80338008L

//
// MessageId: ERROR_WSMAN_SUBSCRIPTION_CLOSE_IN_PROGRESS
//
// MessageText:
//
// The event subscription session is closing and cannot be used. Start a new subscription.
//
#define ERROR_WSMAN_SUBSCRIPTION_CLOSE_IN_PROGRESS 0x80338009L

//
// MessageId: ERROR_WSMAN_SUBSCRIPTION_CLIENT_DID_NOT_CALL_WITHIN_HEARTBEAT
//
// MessageText:
//
// The application or script that has an event subscription did not request a pull operation within the heartbeat interval.
// The subscription session was closed. Start a new subscription.
//
#define ERROR_WSMAN_SUBSCRIPTION_CLIENT_DID_NOT_CALL_WITHIN_HEARTBEAT 0x8033800AL

//
// MessageId: ERROR_WSMAN_SUBSCRIPTION_NO_HEARTBEAT
//
// MessageText:
//
// The event source did not return events within the heartbeat interval.
// The subscription session was closed. Start a new subscription.
//
#define ERROR_WSMAN_SUBSCRIPTION_NO_HEARTBEAT 0x8033800BL

//
// MessageId: ERROR_WSMAN_UNSUPPORTED_TIMEOUT
//
// MessageText:
//
// The WS-Management service does not support the specified timeout.
// The value specified is smaller than the minimum allowed value for this setting.
// Change the timeout value and try the request again.
//
#define ERROR_WSMAN_UNSUPPORTED_TIMEOUT  0x8033800CL

// wsa, code=VersionMismatch, subcode=, details=version mismatch
//
// MessageId: ERROR_WSMAN_SOAP_VERSION_MISMATCH
//
// MessageText:
//
// The WS-Management service does not support the SOAP version specified in the request.
//
#define ERROR_WSMAN_SOAP_VERSION_MISMATCH 0x8033800DL

// wsa, code=DataEncodingUnknown, subcode=, details=version mismatch
//
// MessageId: ERROR_WSMAN_SOAP_DATA_ENCODING_UNKNOWN
//
// MessageText:
//
// The WS-Management service does not support the encoding specified in the request.
//
#define ERROR_WSMAN_SOAP_DATA_ENCODING_UNKNOWN 0x8033800EL

// wsa, code=Sender, subcode=WS-Addressing InvalidMessageInformationHeader, details=invalid_header
//
// MessageId: ERROR_WSMAN_INVALID_MESSAGE_INFORMATION_HEADER
//
// MessageText:
//
// The WS-Management service cannot process the request. The request contains one or more invalid SOAP headers.
//
#define ERROR_WSMAN_INVALID_MESSAGE_INFORMATION_HEADER 0x8033800FL

//
// MessageId: ERROR_WSMAN_SOAP_FAULT_MUST_UNDERSTAND
//
// MessageText:
//
// The WS-Management service cannot process a SOAP header in the request that is marked as mustUnderstand by the client. 
// This could be caused by the use of a version of the protocol which is not supported, or may be an incompatibility 
// between the client and server implementations.
//
#define ERROR_WSMAN_SOAP_FAULT_MUST_UNDERSTAND 0x80338010L

// wsa, code=Sender, subcode=WS-Addressing MessageInformationHeaderRequired, details=missing_header
//
// MessageId: ERROR_WSMAN_MESSAGE_INFORMATION_HEADER_REQUIRED
//
// MessageText:
//
// The WS-Management service cannot process the request. The request does not have all the expected SOAP headers.
//
#define ERROR_WSMAN_MESSAGE_INFORMATION_HEADER_REQUIRED 0x80338011L

// wsa, code=Sender, subcode=WS-Addressing DestinationUnreachable, details=
//
// MessageId: ERROR_WSMAN_DESTINATION_UNREACHABLE
//
// MessageText:
//
// The client cannot connect to the destination specified in the request.
// Verify that the service on the destination is running and is accepting requests.
// Consult the logs and documentation for the WS-Management service running on the destination, most commonly IIS or WinRM.
// If the destination is the WinRM service, run the following command on the destination to analyze and configure the WinRM service: "winrm quickconfig".
//
#define ERROR_WSMAN_DESTINATION_UNREACHABLE 0x80338012L

// wsa, code=Sender, subcode=WS-Addressing ActionNotsupported, details=action
//
// MessageId: ERROR_WSMAN_ACTION_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the action specified in the request.
//
#define ERROR_WSMAN_ACTION_NOT_SUPPORTED 0x80338013L

// This is probably what will wrap all other Windows error codes and so should not explicitly be used
// wsa, code=Receiver, subcode=WS-Addressing EndpointUnavailable, details=
//
// MessageId: ERROR_WSMAN_ENDPOINT_UNAVAILABLE
//
// MessageText:
//
// The WS-Management service cannot process the request because the resource is offline. Retry the request
// later when the resource is online.
//
#define ERROR_WSMAN_ENDPOINT_UNAVAILABLE 0x80338014L

// wsa, code=Sender, subcode=wxf:InvalidRepresentation, details=
//
// MessageId: ERROR_WSMAN_INVALID_REPRESENTATION
//
// MessageText:
//
// The WS-Management service cannot identify the format of the object passed to a Put or Create method.
// The input XML may not be appropriate for the resource or uses the wrong schema for the resource.
// Change the input XML in the request.
//
#define ERROR_WSMAN_INVALID_REPRESENTATION 0x80338015L

// wsen, code=Sender, subcode=WS-Enumeration InvalidExpirationTime, details=
//
// MessageId: ERROR_WSMAN_ENUMERATE_INVALID_EXPIRATION_TIME
//
// MessageText:
//
// The expiration time passed to the WS-Management Enumerate method is not valid. The time value may be zero
// or refer to a time in the past. Change the expiration time and try the request again.
// 
//
#define ERROR_WSMAN_ENUMERATE_INVALID_EXPIRATION_TIME 0x80338016L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature - WS-Management faultDetail/ExpirationTime, details=
//
// MessageId: ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TIME
//
// MessageText:
//
// The data source does not support expiration time. Remove the expiration time from the request and try the request again.
//
#define ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TIME 0x80338017L

// wsen, code=Sender, subcode=WS-Enumeration FilteringNotSupported, details=
//
// MessageId: ERROR_WSMAN_ENUMERATE_FILTERING_NOT_SUPPORTED
//
// MessageText:
//
// The data source does not support filtering. Remove the filter from the request and try the request again.
//
#define ERROR_WSMAN_ENUMERATE_FILTERING_NOT_SUPPORTED 0x80338018L

// wsen, code=Sender, subcode=WS-Enumeration FilterDialectRequestedUnavailable, details=supported_dialects
//
// MessageId: ERROR_WSMAN_ENUMERATE_FILTER_DIALECT_REQUESTED_UNAVAILABLE
//
// MessageText:
//
// The filter dialect (the type associated with the filter) was not supported for this resource.
// Change the filter dialect or remove it from the request and try the request again.
//
#define ERROR_WSMAN_ENUMERATE_FILTER_DIALECT_REQUESTED_UNAVAILABLE 0x80338019L

// wsen, code=Sender, subcode=WS-Enumeration CannotProcessFilter, details=
//
// MessageId: ERROR_WSMAN_ENUMERATE_CANNOT_PROCESS_FILTER
//
// MessageText:
//
// The data source could not process the filter. The filter might be missing or it might be invalid.
// Change the filter and try the request again. 
//
#define ERROR_WSMAN_ENUMERATE_CANNOT_PROCESS_FILTER 0x8033801AL

// wsen, code=Receiver, subcode=WS-Enumeration InvalidEnumerationContext, details=
//
// MessageId: ERROR_WSMAN_ENUMERATE_INVALID_ENUMERATION_CONTEXT
//
// MessageText:
//
// The WS-Enumeration context in the enumeration is not valid. Enumeration may have been completed or canceled.
// You cannot use this enumeration context anymore. Start a new enumeration.
//
#define ERROR_WSMAN_ENUMERATE_INVALID_ENUMERATION_CONTEXT 0x8033801BL

// wsen, code=Receiver, subcode=WS-Enumeration TimedOut, details=
//
// MessageId: ERROR_WSMAN_ENUMERATE_TIMED_OUT
//
// MessageText:
//
// The pull operation did not get any data in the MaxTime duration. But the enumeration is still valid.
// The client can attempt to do another pull request to retrieve data.
//
#define ERROR_WSMAN_ENUMERATE_TIMED_OUT  0x8033801CL

// wsen, code=Receiver, subcode=WS-Enumeration UnableToRenew, details=
//
// MessageId: ERROR_WSMAN_ENUMERATE_UNABLE_TO_RENEW
//
// MessageText:
//
// The WS-Management service cannot renew the enumeration. Start a new enumeration.
//
#define ERROR_WSMAN_ENUMERATE_UNABLE_TO_RENEW 0x8033801DL

// wse, code=Sender, subcode=WS-Eventing DeliveryModeRequestedUnavailable, details=List of delivery modes that are supported
//
// MessageId: ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_UNAVAILABLE
//
// MessageText:
//
// The WS-Management service does not support the delivery mode for the specified resource. The client should change the
// subscription to use one of the supported delivery modes.
//
#define ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_UNAVAILABLE 0x8033801EL

// wse, code=Sender, subcode=WS-Eventing InvalidExpirationTime, details=
//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_EXPIRATION_TIME
//
// MessageText:
//
// The expiration time of the subscription is invalid. The time is either not supported, zero or a time that happened in the past.
// Change the expiration time and try the request again.
//
#define ERROR_WSMAN_EVENTING_INVALID_EXPIRATION_TIME 0x8033801FL

// wse, code=Sender, subcode=WS-Eventing UnsupportedExpirationType, details=
//
// MessageId: ERROR_WSMAN_EVENTING_UNSUPPORTED_EXPIRATION_TYPE
//
// MessageText:
//
// The expiration time specified for subscription was invalid. Specify the expiration time as a duration.
//
#define ERROR_WSMAN_EVENTING_UNSUPPORTED_EXPIRATION_TYPE 0x80338020L

// wse, code=Sender, subcode=WS-Eventing FilteringNotSupported, details=
//
// MessageId: ERROR_WSMAN_EVENTING_FILTERING_NOT_SUPPORTED
//
// MessageText:
//
// The event source does not support filtering. Remove the filter from the request and try the request again.
//
#define ERROR_WSMAN_EVENTING_FILTERING_NOT_SUPPORTED 0x80338021L

// wse, code=Sender, subcode=WS-Eventing FilteringRequestedUnavailable, details=
//
// MessageId: ERROR_WSMAN_EVENTING_FILTERING_REQUESTED_UNAVAILABLE
//
// MessageText:
//
// The event source cannot process the specified filter.
// Change the filter or remove it from the request and try the request again.
//
#define ERROR_WSMAN_EVENTING_FILTERING_REQUESTED_UNAVAILABLE 0x80338022L

// wse, code=Receiver, subcode=WS-Eventing EventSourceUnableToProcess, details=
//
// MessageId: ERROR_WSMAN_EVENTING_SOURCE_UNABLE_TO_PROCESS
//
// MessageText:
//
// The event source cannot process the subscription.
//
#define ERROR_WSMAN_EVENTING_SOURCE_UNABLE_TO_PROCESS 0x80338023L

// wse, code=Receiver, subcode=WS-Eventing UnableToRenew, details=
//
// MessageId: ERROR_WSMAN_EVENTING_UNABLE_TO_RENEW
//
// MessageText:
//
// The WS-Management service cannot renew the event subscription. Create a new subscription.
//
#define ERROR_WSMAN_EVENTING_UNABLE_TO_RENEW 0x80338024L

// wse, code=Sender, subcode=WS-Eventing InvalidMessage, details=
//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_MESSAGE
//
// MessageText:
//
// The WS-Management service cannot complete the WS-Eventing request because the request had some unknown or invalid content and could
// not be processed.
//
#define ERROR_WSMAN_EVENTING_INVALID_MESSAGE 0x80338025L

//
// MessageId: ERROR_WSMAN_ENVELOPE_TOO_LARGE
//
// MessageText:
//
// The WS-Management service cannot process the response because it is larger than the maximum size allowed.
//
#define ERROR_WSMAN_ENVELOPE_TOO_LARGE   0x80338026L

// wsman, code=Sender, subcode=WS-Management InvalidBody, details=
//
// MessageId: ERROR_WSMAN_INVALID_SOAP_BODY
//
// MessageText:
//
// The WS-Management service cannot process the request because the request packet does not have a valid SOAP body.
//
#define ERROR_WSMAN_INVALID_SOAP_BODY    0x80338027L

// wsman, code=Sender, subcode=WS-Management InvalidResumptionContext, details=
//
// MessageId: ERROR_WSMAN_INVALID_RESUMPTION_CONTEXT
//
// MessageText:
//
// The resumption context specified in the subscription is invalid. It may have expired, or be in the wrong format.
//
#define ERROR_WSMAN_INVALID_RESUMPTION_CONTEXT 0x80338028L

// wsman, code=Receiver, subcode=WS-Management Timedout, details=
//
// MessageId: ERROR_WSMAN_OPERATION_TIMEDOUT
//
// MessageText:
//
// The WS-Management service cannot complete the operation within the time specified in OperationTimeout. 
//
#define ERROR_WSMAN_OPERATION_TIMEDOUT   0x80338029L

// wsman, code=Sender, subcode=WS-Management ResumptionNotSupported, details=
//
// MessageId: ERROR_WSMAN_RESUMPTION_NOT_SUPPORTED
//
// MessageText:
//
// The event source does not support subscriptions that can be resumed.
//
#define ERROR_WSMAN_RESUMPTION_NOT_SUPPORTED 0x8033802AL

// wsman, code=Sender, subcode=WS-Management ResumptionTypeNotSupported, details=
//
// MessageId: ERROR_WSMAN_RESUMPTION_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the type of resumption requested by the subscription.
//
#define ERROR_WSMAN_RESUMPTION_TYPE_NOT_SUPPORTED 0x8033802BL

// wsman, code=Sender, subcode=WS-Management UnsupportedEncoding, details=
//
// MessageId: ERROR_WSMAN_UNSUPPORTED_ENCODING
//
// MessageText:
//
// The request contains character encoding that is unsupported. WS-Management only supports requests that are
// encoded in UTF-8 or UTF-16. Change the character encoding in the request and try the request again.
//
#define ERROR_WSMAN_UNSUPPORTED_ENCODING 0x8033802CL

// wsman, code=Sender, subcode=WS-Management UriLimit, details=
//
// MessageId: ERROR_WSMAN_URI_LIMIT
//
// MessageText:
//
// The URI is longer than the maximum length allowed.
//
#define ERROR_WSMAN_URI_LIMIT            0x8033802DL

// wsman, code=Sender, subcode=WS-Management InvalidProposedID, details=
//
// MessageId: ERROR_WSMAN_INVALID_PROPOSED_ID
//
// MessageText:
//
// The WS-Management service cannot process the request because the subscription ID is invalid.
//
#define ERROR_WSMAN_INVALID_PROPOSED_ID  0x8033802EL

// wsman, code=Sender, subcode=WS-Management InvalidBatchParameter, details=
//
// MessageId: ERROR_WSMAN_INVALID_BATCH_PARAMETER
//
// MessageText:
//
// The WS-Management service cannot process the batch request. The request must specify either MaxItems, MaxCharacters,
// or MaxTime.
//
#define ERROR_WSMAN_INVALID_BATCH_PARAMETER 0x8033802FL

// wsman, code=Sender, subcode=WS-Management NoAck, details=
//
// MessageId: ERROR_WSMAN_NO_ACK
//
// MessageText:
//
// The receiver of the event did not acknowledge the event delivery.
// Submit the subscription again without the acknowledgement option.
//
#define ERROR_WSMAN_NO_ACK               0x80338030L

// wsman, code=Sender, subcode=WS-Management ActionMismatch, details=
//
// MessageId: ERROR_WSMAN_ACTION_MISMATCH
//
// MessageText:
//
// The WS-Management service cannot process the request because the WS-Addressing Action URI in the request is not
// compatible with the resource.
//
#define ERROR_WSMAN_ACTION_MISMATCH      0x80338031L

// wsman, code=Sender, subcode=WS-Management Concurrency, details=
//
// MessageId: ERROR_WSMAN_CONCURRENCY
//
// MessageText:
//
// The WS-Management service cannot complete the WS-Addressing Action URI in the request because the resource
// was already in use.
//
#define ERROR_WSMAN_CONCURRENCY          0x80338032L

// wsman, code=Sender, subcode=WS-Management AlreadyExists, details=
//
// MessageId: ERROR_WSMAN_ALREADY_EXISTS
//
// MessageText:
//
// The WS-Management service cannot create the resource because it already exists.
//
#define ERROR_WSMAN_ALREADY_EXISTS       0x80338033L

// wsman, code=Receiver, subcode=WS-Management DeliveryRefused, details=
//
// MessageId: ERROR_WSMAN_DELIVERY_REFUSED
//
// MessageText:
//
// The WS-Management service cannot complete the request because the receiver does not accept the delivery of events.
// The receiver requests that the subscription be cancelled. Event receivers return this message to force the
// cancellation of a subscription.
//
#define ERROR_WSMAN_DELIVERY_REFUSED     0x80338034L

// wsman, code=Sender, subcode=WS-Management EncodingLimit, details=
//
// MessageId: ERROR_WSMAN_ENCODING_LIMIT
//
// MessageText:
//
// The WS-Management service cannot process the request because the encoding of the request exceeds an internal
// encoding limit. Reconfigure the client to send messages which fit the encoding limits of the service.
//
#define ERROR_WSMAN_ENCODING_LIMIT       0x80338035L

// wsman, code=Sender, subcode=wsse:FailedAuthentication, details=
//
// MessageId: ERROR_WSMAN_FAILED_AUTHENTICATION
//
// MessageText:
//
// The WS-Management service cannot authenticate the sender.
//
#define ERROR_WSMAN_FAILED_AUTHENTICATION 0x80338036L

// wsman, code=Sender, subcode=WS-Management IncompatibleEPR, details=
//
// MessageId: ERROR_WSMAN_INCOMPATIBLE_EPR
//
// MessageText:
//
// The WS-Management service does not support the format of the WS-Addressing Endpoint Reference.
//
#define ERROR_WSMAN_INCOMPATIBLE_EPR     0x80338037L

// wsman, code=Sender, subcode=WS-Management InvalidBookmark, details=
//
// MessageId: ERROR_WSMAN_INVALID_BOOKMARK
//
// MessageText:
//
// The bookmark in the subscription is invalid. The bookmark may be expired or corrupted. Issue a new subscription
// without any bookmarks or locate the correct bookmark.
//
#define ERROR_WSMAN_INVALID_BOOKMARK     0x80338038L

// wsman, code=Sender, subcode=WS-Management InvalidOptions, details=
//
// MessageId: ERROR_WSMAN_INVALID_OPTIONS
//
// MessageText:
//
// The WS-Management service cannot process the request because one or more options are not valid. The option names or
// values may not be valid or they are used in incorrect combinations. Retrieve the catalog entry for the resource and
// determine how to correct the invalid option values.
//
#define ERROR_WSMAN_INVALID_OPTIONS      0x80338039L

// wsman, code=Sender, subcode=WS-Management InvalidParameter, details=
//
// MessageId: ERROR_WSMAN_INVALID_PARAMETER
//
// MessageText:
//
// The WS-Management service cannot process the request because a parameter for the operation is not valid.
//
#define ERROR_WSMAN_INVALID_PARAMETER    0x8033803AL

// wsman, code=Sender, subcode=WS-Management InvalidResourceURI, details=
//
// MessageId: ERROR_WSMAN_INVALID_RESOURCE_URI
//
// MessageText:
//
// The WS-Management service cannot process the request. The resource URI is missing or it has an incorrect format.
// Check the documentation or use the following command for information on how to construct a resource URI: "winrm help uris".
//
#define ERROR_WSMAN_INVALID_RESOURCE_URI 0x8033803BL

// wsman, code=Sender, subcode=WS-Management InvalidSystem, details=
//
// MessageId: ERROR_WSMAN_INVALID_SYSTEM
//
// MessageText:
//
// The WS-Management service requires a valid System URI to process the request.
//
#define ERROR_WSMAN_INVALID_SYSTEM       0x8033803CL

// wsman, code=Sender, subcode=WS-Management InvalidSelectors, details=
//
// MessageId: ERROR_WSMAN_INVALID_SELECTORS
//
// MessageText:
//
// The WS-Management service cannot process the request because the selectors for the resource are not valid.
//
#define ERROR_WSMAN_INVALID_SELECTORS    0x8033803DL

// wsman, code=Sender, subcode=WS-Management MetadaRedirect, details=
//
// MessageId: ERROR_WSMAN_METADATA_REDIRECT
//
// MessageText:
//
// The requested metadata is not available at the current address. Retry the request with a new address.
//
#define ERROR_WSMAN_METADATA_REDIRECT    0x8033803EL

// wsman, code=Sender, subcode=WS-Management QuotaLimit, details=
//
// MessageId: ERROR_WSMAN_QUOTA_LIMIT
//
// MessageText:
//
// The WS-Management service is busy servicing other requests. Retry later.
//
#define ERROR_WSMAN_QUOTA_LIMIT          0x8033803FL

// wsman, code=Sender, subcode=WS-Management RenameFailure, details=
//
// MessageId: ERROR_WSMAN_RENAME_FAILURE
//
// MessageText:
//
// The WS-Management service cannot rename the resource. The selectors for the resource are not correct. The
// resource may exist already, the address may be incorrect, or the resource URI may be invalid. Change the
// request and retry.
//
#define ERROR_WSMAN_RENAME_FAILURE       0x80338040L

// wsman, code=Sender, subcode=WS-Management SchemaValidationError, details=
//
// MessageId: ERROR_WSMAN_SCHEMA_VALIDATION_ERROR
//
// MessageText:
//
// The SOAP XML in the message does not match the corresponding XML schema definition. Change the XML and retry.
//
#define ERROR_WSMAN_SCHEMA_VALIDATION_ERROR 0x80338041L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details=
//
// MessageId: ERROR_WSMAN_UNSUPPORTED_FEATURE
//
// MessageText:
//
// The WS-Management service does not support the specified feature. Remove the unsupported feature from the request and retry.
//
#define ERROR_WSMAN_UNSUPPORTED_FEATURE  0x80338042L

//
// MessageId: ERROR_WSMAN_INVALID_XML
//
// MessageText:
//
// The WS-Management service cannot process the request because the XML is invalid.
//
#define ERROR_WSMAN_INVALID_XML          0x80338043L

//
// MessageId: ERROR_WSMAN_INVALID_KEY
//
// MessageText:
//
// The WS-Management service cannot process the request because the URI contains an unexpected selector.
//
#define ERROR_WSMAN_INVALID_KEY          0x80338044L

//
// MessageId: ERROR_WSMAN_DELIVER_IN_PROGRESS
//
// MessageText:
//
// The event source is attempting to deliver an event when a delivery is in progress already.
//
#define ERROR_WSMAN_DELIVER_IN_PROGRESS  0x80338045L

//
// MessageId: ERROR_WSMAN_SYSTEM_NOT_FOUND
//
// MessageText:
//
// The WS-Management service cannot locate the system.
//
#define ERROR_WSMAN_SYSTEM_NOT_FOUND     0x80338046L

//
// MessageId: ERROR_WSMAN_MAX_ENVELOPE_SIZE
//
// MessageText:
//
// The maximum envelope size in the request is too large.
// Change the maximum envelope size and try the request again. 
//
#define ERROR_WSMAN_MAX_ENVELOPE_SIZE    0x80338047L

//
// MessageId: ERROR_WSMAN_MAX_ENVELOPE_SIZE_EXCEEDED
//
// MessageText:
//
// The response that the WS-Management service computed exceeds the maximum envelope size in the request.
//
#define ERROR_WSMAN_MAX_ENVELOPE_SIZE_EXCEEDED 0x80338048L

//
// MessageId: ERROR_WSMAN_SERVER_ENVELOPE_LIMIT
//
// MessageText:
//
// The response that the WS-Management service computed exceed the internal limit for envelope size.
//
#define ERROR_WSMAN_SERVER_ENVELOPE_LIMIT 0x80338049L

//
// MessageId: ERROR_WSMAN_SELECTOR_LIMIT
//
// MessageText:
//
// The WS-Management service cannot process the request because the URI contains too many selectors.
//
#define ERROR_WSMAN_SELECTOR_LIMIT       0x8033804AL

//
// MessageId: ERROR_WSMAN_OPTION_LIMIT
//
// MessageText:
//
// The WS-Management service cannot process the request because it contains too many options.
//
#define ERROR_WSMAN_OPTION_LIMIT         0x8033804BL

//
// MessageId: ERROR_WSMAN_CHARACTER_SET
//
// MessageText:
//
// The WS-Management service does not support the character set used in the request. Change the request to use UTF-8 or UTF-16.
//
#define ERROR_WSMAN_CHARACTER_SET        0x8033804CL

//
// MessageId: ERROR_WSMAN_UNREPORTABLE_SUCCESS
//
// MessageText:
//
// The operation succeeded and cannot be reversed but the result is too large to send.
//
#define ERROR_WSMAN_UNREPORTABLE_SUCCESS 0x8033804DL

//
// MessageId: ERROR_WSMAN_WHITESPACE
//
// MessageText:
//
// The WS-Management service does not support white space in the request XML.
//
#define ERROR_WSMAN_WHITESPACE           0x8033804EL

//
// MessageId: ERROR_WSMAN_FILTERING_REQUIRED
//
// MessageText:
//
// The WS-Management service does not support the filter dialect in the request. The filter dialect is the type of filter, such
// as XPath or WQL.
//
#define ERROR_WSMAN_FILTERING_REQUIRED   0x8033804FL

//
// MessageId: ERROR_WSMAN_BOOKMARK_EXPIRED
//
// MessageText:
//
// The WS-Management service cannot process the request because it contains a bookmark that is expired.
//
#define ERROR_WSMAN_BOOKMARK_EXPIRED     0x80338050L

//
// MessageId: ERROR_WSMAN_OPTIONS_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management provider does not support the specified option set because mustComply for one of the options is set to true.
// Change mustComply for one of the options to false.
//
#define ERROR_WSMAN_OPTIONS_NOT_SUPPORTED 0x80338051L

//
// MessageId: ERROR_WSMAN_OPTIONS_INVALID_NAME
//
// MessageText:
//
// The WS-Management service cannot process the request because one or more of the options has an invalid name.
//
#define ERROR_WSMAN_OPTIONS_INVALID_NAME 0x80338052L

//
// MessageId: ERROR_WSMAN_OPTIONS_INVALID_VALUE
//
// MessageText:
//
// The WS-Management service cannot process the request because one or more of the options has an invalid value.
//
#define ERROR_WSMAN_OPTIONS_INVALID_VALUE 0x80338053L

//
// MessageId: ERROR_WSMAN_PARAMETER_TYPE_MISMATCH
//
// MessageText:
//
// The WS-Management service cannot process the request. A parameter that is required for the operation is not the
// correct type.
//
#define ERROR_WSMAN_PARAMETER_TYPE_MISMATCH 0x80338054L

//
// MessageId: ERROR_WSMAN_INVALID_PARAMETER_NAME
//
// MessageText:
//
// The WS-Management service cannot process the request. A parameter name is invalid.
//
#define ERROR_WSMAN_INVALID_PARAMETER_NAME 0x80338055L

//
// MessageId: ERROR_WSMAN_INVALID_XML_VALUES
//
// MessageText:
//
// The WS-Management service cannot process the request because the XML content has invalid values.
//
#define ERROR_WSMAN_INVALID_XML_VALUES   0x80338056L

//
// MessageId: ERROR_WSMAN_INVALID_XML_MISSING_VALUES
//
// MessageText:
//
// The WS-Management service cannot process the request because the XML content has missing values.
//
#define ERROR_WSMAN_INVALID_XML_MISSING_VALUES 0x80338057L

//
// MessageId: ERROR_WSMAN_INVALID_XML_NAMESPACE
//
// MessageText:
//
// The WS-Management service cannot identify the format of the object passed to a Put or Create method.
// The XML namespace for the input XML is invalid. Change the XML namespace for the input XML in the request.
//
#define ERROR_WSMAN_INVALID_XML_NAMESPACE 0x80338058L

//
// MessageId: ERROR_WSMAN_INVALID_XML_FRAGMENT
//
// MessageText:
//
// The WS-Management service cannot process the request because an XML fragment in the URI is invalid.
//
#define ERROR_WSMAN_INVALID_XML_FRAGMENT 0x80338059L

//
// MessageId: ERROR_WSMAN_INSUFFCIENT_SELECTORS
//
// MessageText:
//
// The WS-Management service cannot process the request because the request did not contain all required selectors.
//
#define ERROR_WSMAN_INSUFFCIENT_SELECTORS 0x8033805AL

//
// MessageId: ERROR_WSMAN_UNEXPECTED_SELECTORS
//
// MessageText:
//
// The WS-Management service cannot process the request because the request contained invalid selectors for the resource.
//
#define ERROR_WSMAN_UNEXPECTED_SELECTORS 0x8033805BL

//
// MessageId: ERROR_WSMAN_SELECTOR_TYPEMISMATCH
//
// MessageText:
//
// The WS-Management service cannot process the request because a value for a selector is of the wrong type.
//
#define ERROR_WSMAN_SELECTOR_TYPEMISMATCH 0x8033805CL

//
// MessageId: ERROR_WSMAN_INVALID_SELECTOR_VALUE
//
// MessageText:
//
// The WS-Management service cannot process the request because a value for the selector is invalid.
//
#define ERROR_WSMAN_INVALID_SELECTOR_VALUE 0x8033805DL

//
// MessageId: ERROR_WSMAN_AMBIGUOUS_SELECTORS
//
// MessageText:
//
// The WS-Management service cannot process the request because the selectors for the resource are ambiguous.
//
#define ERROR_WSMAN_AMBIGUOUS_SELECTORS  0x8033805EL

//
// MessageId: ERROR_WSMAN_DUPLICATE_SELECTORS
//
// MessageText:
//
// The WS-Management service cannot process the request because the request contains duplicate selectors.
//
#define ERROR_WSMAN_DUPLICATE_SELECTORS  0x8033805FL

//
// MessageId: ERROR_WSMAN_INVALID_TARGET_SELECTORS
//
// MessageText:
//
// The WS-Management service cannot process the request because the request contains invalid selectors for the target resource.
//
#define ERROR_WSMAN_INVALID_TARGET_SELECTORS 0x80338060L

//
// MessageId: ERROR_WSMAN_INVALID_TARGET_RESOURCEURI
//
// MessageText:
//
// The WS-Management service cannot process the request because the request contains an invalid URI for the target resource.
//
#define ERROR_WSMAN_INVALID_TARGET_RESOURCEURI 0x80338061L

//
// MessageId: ERROR_WSMAN_INVALID_TARGET_SYSTEM
//
// MessageText:
//
// The WS-Management service cannot process the request because the request contains an invalid target system.
//
#define ERROR_WSMAN_INVALID_TARGET_SYSTEM 0x80338062L

//
// MessageId: ERROR_WSMAN_TARGET_ALREADY_EXISTS
//
// MessageText:
//
// The WS-Management service cannot process a Create request because the target already exists.
//
#define ERROR_WSMAN_TARGET_ALREADY_EXISTS 0x80338063L

//
// MessageId: ERROR_WSMAN_AUTHORIZATION_MODE_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the mode of authorization.
//
#define ERROR_WSMAN_AUTHORIZATION_MODE_NOT_SUPPORTED 0x80338064L

//
// MessageId: ERROR_WSMAN_ACK_NOT_SUPPORTED
//
// MessageText:
//
// The client does not support acknowledgment.
//
#define ERROR_WSMAN_ACK_NOT_SUPPORTED    0x80338065L

//
// MessageId: ERROR_WSMAN_OPERATION_TIMEOUT_NOT_SUPPORTED
//
// MessageText:
//
// The data source does not support timeouts for the operation.
//
#define ERROR_WSMAN_OPERATION_TIMEOUT_NOT_SUPPORTED 0x80338066L

//
// MessageId: ERROR_WSMAN_LOCALE_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the locale.
//
#define ERROR_WSMAN_LOCALE_NOT_SUPPORTED 0x80338067L

//
// MessageId: ERROR_WSMAN_EXPIRATION_TIME_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the expiration time.
//
#define ERROR_WSMAN_EXPIRATION_TIME_NOT_SUPPORTED 0x80338068L

//
// MessageId: ERROR_WSMAN_DELIVERY_RETRIES_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not retry deliveries.
//
#define ERROR_WSMAN_DELIVERY_RETRIES_NOT_SUPPORTED 0x80338069L

//
// MessageId: ERROR_WSMAN_HEARTBEATS_NOT_SUPPORTED
//
// MessageText:
//
// The event source does not support heartbeats.
//
#define ERROR_WSMAN_HEARTBEATS_NOT_SUPPORTED 0x8033806AL

//
// MessageId: ERROR_WSMAN_BOOKMARKS_NOT_SUPPORTED
//
// MessageText:
//
// The event source does not support bookmarks.
//
#define ERROR_WSMAN_BOOKMARKS_NOT_SUPPORTED 0x8033806BL

//
// MessageId: ERROR_WSMAN_MAXITEMS_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the configuration for MaxItems.
//
#define ERROR_WSMAN_MAXITEMS_NOT_SUPPORTED 0x8033806CL

//
// MessageId: ERROR_WSMAN_MAXTIME_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the configuration for MaxTime.
//
#define ERROR_WSMAN_MAXTIME_NOT_SUPPORTED 0x8033806DL

//
// MessageId: ERROR_WSMAN_MAXENVELOPE_SIZE_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the value in the configuration for MaxEnvelopeSize.
//
#define ERROR_WSMAN_MAXENVELOPE_SIZE_NOT_SUPPORTED 0x8033806EL

//
// MessageId: ERROR_WSMAN_MAXENVELOPE_POLICY_NOT_SUPPORTED
//
// MessageText:
//
// The event source does not support the MaxEnvelopePolicy.
//
#define ERROR_WSMAN_MAXENVELOPE_POLICY_NOT_SUPPORTED 0x8033806FL

//
// MessageId: ERROR_WSMAN_FILTERING_REQUIRED_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support unfiltered enumeration.  
//
#define ERROR_WSMAN_FILTERING_REQUIRED_NOT_SUPPORTED 0x80338070L

//
// MessageId: ERROR_WSMAN_INSECURE_ADDRESS_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support insecure addresses.
//
#define ERROR_WSMAN_INSECURE_ADDRESS_NOT_SUPPORTED 0x80338071L

//
// MessageId: ERROR_WSMAN_FORMAT_MISMATCH_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support format mismatch.
//
#define ERROR_WSMAN_FORMAT_MISMATCH_NOT_SUPPORTED 0x80338072L

//
// MessageId: ERROR_WSMAN_FORMAT_SECURITY_TOKEN_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the format of the security token.
//
#define ERROR_WSMAN_FORMAT_SECURITY_TOKEN_NOT_SUPPORTED 0x80338073L

//
// MessageId: ERROR_WSMAN_BAD_METHOD
//
// MessageText:
//
// The service returned a response that indicates that the method is unsupported.
//
#define ERROR_WSMAN_BAD_METHOD           0x80338074L

//
// MessageId: ERROR_WSMAN_UNSUPPORTED_MEDIA
//
// MessageText:
//
// The WS-Management service does not support the specified media type.
//
#define ERROR_WSMAN_UNSUPPORTED_MEDIA    0x80338075L

//
// MessageId: ERROR_WSMAN_UNSUPPORTED_ADDRESSING_MODE
//
// MessageText:
//
// The WS-Management service does not support the addressing mode.
//
#define ERROR_WSMAN_UNSUPPORTED_ADDRESSING_MODE 0x80338076L

//
// MessageId: ERROR_WSMAN_FRAGMENT_TRANSFER_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service does not support fragment transfer.
//
#define ERROR_WSMAN_FRAGMENT_TRANSFER_NOT_SUPPORTED 0x80338077L

//
// MessageId: ERROR_WSMAN_ENUMERATION_INITIALIZING
//
// MessageText:
//
// The client sent a request before the enumeration was initialized.
//
#define ERROR_WSMAN_ENUMERATION_INITIALIZING 0x80338078L

//
// MessageId: ERROR_WSMAN_CONNECTOR_GET
//
// MessageText:
//
// The WS-Management service failed to locate the component that can process the request.
//
#define ERROR_WSMAN_CONNECTOR_GET        0x80338079L

//
// MessageId: ERROR_WSMAN_URI_QUERY_STRING_SYNTAX_ERROR
//
// MessageText:
//
// A syntax error occurred in the query string for the resource URI.
//
#define ERROR_WSMAN_URI_QUERY_STRING_SYNTAX_ERROR 0x8033807AL

//
// MessageId: ERROR_WSMAN_INEXISTENT_MAC_ADDRESS
//
// MessageText:
//
// The MAC that is configured is not in the list of enabled DHCP adapters on the computer.
//
#define ERROR_WSMAN_INEXISTENT_MAC_ADDRESS 0x8033807BL

//
// MessageId: ERROR_WSMAN_NO_UNICAST_ADDRESSES
//
// MessageText:
//
// The MAC address that is configured does not have any unicast addresses.
//
#define ERROR_WSMAN_NO_UNICAST_ADDRESSES 0x8033807CL

//
// MessageId: ERROR_WSMAN_NO_DHCP_ADDRESSES
//
// MessageText:
//
// The WS-Management service cannot find the dynamic IP address on the adapter with the configured MAC address.
//
#define ERROR_WSMAN_NO_DHCP_ADDRESSES    0x8033807DL

//
// MessageId: ERROR_WSMAN_MIN_ENVELOPE_SIZE
//
// MessageText:
//
// The WS-Management service cannot process the request because the envelope size in the request is too small.
//
#define ERROR_WSMAN_MIN_ENVELOPE_SIZE    0x8033807EL

//
// MessageId: ERROR_WSMAN_EPR_NESTING_EXCEEDED
//
// MessageText:
//
// The WS-Management service cannot process the request. The EndPointReference contains more nested EndPointReferences
// than WS-Management supports.
//
#define ERROR_WSMAN_EPR_NESTING_EXCEEDED 0x8033807FL

//
// MessageId: ERROR_WSMAN_REQUEST_INIT_ERROR
//
// MessageText:
//
// The WS-Management service cannot initialize the request.
//
#define ERROR_WSMAN_REQUEST_INIT_ERROR   0x80338080L

//
// MessageId: ERROR_WSMAN_INVALID_TIMEOUT_HEADER
//
// MessageText:
//
// The WS-Management service cannot process the request because the timeout header in the request is invalid.
//
#define ERROR_WSMAN_INVALID_TIMEOUT_HEADER 0x80338081L

//
// MessageId: ERROR_WSMAN_CERT_NOT_FOUND
//
// MessageText:
//
// The WS-Management service cannot find the certificate that was requested.
//
#define ERROR_WSMAN_CERT_NOT_FOUND       0x80338082L

//
// MessageId: ERROR_WSMAN_PLUGIN_FAILED
//
// MessageText:
//
// The WS-Management service cannot process the request. The data source failed to return results for the request.
//
#define ERROR_WSMAN_PLUGIN_FAILED        0x80338083L

//
// MessageId: ERROR_WSMAN_ENUMERATION_INVALID
//
// MessageText:
//
// The enumeration is invalid because previous Pull request failed.
//
#define ERROR_WSMAN_ENUMERATION_INVALID  0x80338084L

//
// MessageId: ERROR_WSMAN_CONFIG_CANNOT_CHANGE_MUTUAL
//
// MessageText:
//
// The WS-Management service cannot change a mutual configuration.
//
#define ERROR_WSMAN_CONFIG_CANNOT_CHANGE_MUTUAL 0x80338085L

//
// MessageId: ERROR_WSMAN_ENUMERATION_MODE_UNSUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the specified enumeration mode.
//
#define ERROR_WSMAN_ENUMERATION_MODE_UNSUPPORTED 0x80338086L

//
// MessageId: ERROR_WSMAN_MUSTUNDERSTAND_ON_LOCALE_UNSUPPORTED
//
// MessageText:
//
// The WS-Management service cannot guarantee that all data is returned in the requested locale as some
// data sources may not be able to comply. Resend the remote request with locale as a hint (the SOAP
// header should have mustUnderstand="false")
//
#define ERROR_WSMAN_MUSTUNDERSTAND_ON_LOCALE_UNSUPPORTED 0x80338087L

//
// MessageId: ERROR_WSMAN_POLICY_CORRUPTED
//
// MessageText:
//
// The WSMan group policy configuration is corrupted.
//
#define ERROR_WSMAN_POLICY_CORRUPTED     0x80338088L

//
// MessageId: ERROR_WSMAN_LISTENER_ADDRESS_INVALID
//
// MessageText:
//
// The listener address specified is invalid. The address can be specified in one of the following formats: *, IP:<ip_address>, MAC:<mac_address>.
// Change the listener address and try the request again.
//
#define ERROR_WSMAN_LISTENER_ADDRESS_INVALID 0x80338089L

//
// MessageId: ERROR_WSMAN_CONFIG_CANNOT_CHANGE_GPO_CONTROLLED_SETTING
//
// MessageText:
//
// Cannot change GPO controlled setting.
//
#define ERROR_WSMAN_CONFIG_CANNOT_CHANGE_GPO_CONTROLLED_SETTING 0x8033808AL

//
// MessageId: ERROR_WSMAN_EVENTING_CONCURRENT_CLIENT_RECEIVE
//
// MessageText:
//
// The client is attempting to concurrently receive events from a single subscription session.This is not supported.
//
#define ERROR_WSMAN_EVENTING_CONCURRENT_CLIENT_RECEIVE 0x8033808BL

//
// MessageId: ERROR_WSMAN_EVENTING_FAST_SENDER
//
// MessageText:
//
// The source is sending event batches faster than the subscriber can consume.
// This can happen if acknowledgments are not specified for the subscription and
// new events are arriving from the source before the client has consumed them.
//
#define ERROR_WSMAN_EVENTING_FAST_SENDER 0x8033808CL

//
// MessageId: ERROR_WSMAN_EVENTING_INSECURE_PUSHSUBSCRIPTION_CONNECTION
//
// MessageText:
//
// The source is sending events in a connection that did not match the security restrictions imposed by the client.
//
#define ERROR_WSMAN_EVENTING_INSECURE_PUSHSUBSCRIPTION_CONNECTION 0x8033808DL

//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_EVENTSOURCE
//
// MessageText:
//
// The WS-Management client cannot process the request. The event source identity does not match the identity of the machine that the client subscribed to.
//
#define ERROR_WSMAN_EVENTING_INVALID_EVENTSOURCE 0x8033808EL

//
// MessageId: ERROR_WSMAN_EVENTING_NOMATCHING_LISTENER
//
// MessageText:
//
// The client could not start a valid listener to receive subscription events based on the specified input settings.
//
#define ERROR_WSMAN_EVENTING_NOMATCHING_LISTENER 0x8033808FL

//
// MessageId: ERROR_WSMAN_FRAGMENT_DIALECT_REQUESTED_UNAVAILABLE
//
// MessageText:
//
// The fragment path dialect is not supported for this resource.
//
#define ERROR_WSMAN_FRAGMENT_DIALECT_REQUESTED_UNAVAILABLE 0x80338090L

//
// MessageId: ERROR_WSMAN_MISSING_FRAGMENT_PATH
//
// MessageText:
//
// Cannot execute the Fragment-Level operation. The fragment path cannot be missing if the fragment dialect is specified.
//
#define ERROR_WSMAN_MISSING_FRAGMENT_PATH 0x80338091L

//
// MessageId: ERROR_WSMAN_INVALID_FRAGMENT_DIALECT
//
// MessageText:
//
// Cannot execute the Fragment-Level operation because of invalid value for the fragment dialect.
//
#define ERROR_WSMAN_INVALID_FRAGMENT_DIALECT 0x80338092L

//
// MessageId: ERROR_WSMAN_INVALID_FRAGMENT_PATH
//
// MessageText:
//
// Cannot execute the Fragment-Level operation because the fragment path is invalid. Check the syntax of the fragment path string.
// Also check the spelling and the case of the property names in the fragment path string: they have to match the spelling and the case of the resource properties.
//
#define ERROR_WSMAN_INVALID_FRAGMENT_PATH 0x80338093L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /FormatMismatch
//
// MessageId: ERROR_WSMAN_EVENTING_INCOMPATIBLE_BATCHPARAMS_AND_DELIVERYMODE
//
// MessageText:
//
// The specified batch parameter is incompatible with the specified event delivery mode. This can happen if batchSettings for a specific mode are
// passed for a different mode. For example, batchSettings like "MaxItems" and
// "MaxLatency" are not compatible with single event push mode or pull mode.
//
#define ERROR_WSMAN_EVENTING_INCOMPATIBLE_BATCHPARAMS_AND_DELIVERYMODE 0x80338094L

// wsman, code=Sender, subcode=WS-Eventing EventSourceUnableToProcess, details= /UnusableAddress
//
// MessageId: ERROR_WSMAN_EVENTING_LOOPBACK_TESTFAILED
//
// MessageText:
//
// The connectivity test from the push subscription source to the client failed. This can happen if the client machine initiating the push subscription
// is unreachable from the server machine where the event source is located. Possible reasons include firewall or some other network boundary.
// Modify subscription to use Pull based subscription.
//
#define ERROR_WSMAN_EVENTING_LOOPBACK_TESTFAILED 0x80338095L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_ENDTO_ADDRESSS
//
// MessageText:
//
// The subscribe packet had an EndTo element address that does not match the NotifyTo element address or it was invalid. For subscription the EndTo element
// need not be present in the subscription request. If it exists then it's address should match the address specified in NotifyTo element.
//
#define ERROR_WSMAN_EVENTING_INVALID_ENDTO_ADDRESSS 0x80338096L

//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_INCOMING_EVENT_PACKET_HEADER
//
// MessageText:
//
// The event source sent an event packet whose header could not be processed by the client. This can happen if it was malformed
// or if the header had a mustUnderstand attribute that could not be understood by the client.
//
#define ERROR_WSMAN_EVENTING_INVALID_INCOMING_EVENT_PACKET_HEADER 0x80338097L

//
// MessageId: ERROR_WSMAN_SESSION_ALREADY_CLOSED
//
// MessageText:
//
// An operation is being attempted on a session that is being closed.This can happen if the session that is being used is also being closed
// by another thread.
//
#define ERROR_WSMAN_SESSION_ALREADY_CLOSED 0x80338098L

//
// MessageId: ERROR_WSMAN_SUBSCRIPTION_LISTENER_NOLONGERVALID
//
// MessageText:
//
// The listener on which the subscription session was established is no longer valid. This can happen if the WSMAN service listener configuration has been changed
// and a subscription was already active and using one of the configurations that was deleted.
//
#define ERROR_WSMAN_SUBSCRIPTION_LISTENER_NOLONGERVALID 0x80338099L

//
// MessageId: ERROR_WSMAN_PROVIDER_LOAD_FAILED
//
// MessageText:
//
// The system failed to load the plugin.
//
#define ERROR_WSMAN_PROVIDER_LOAD_FAILED 0x8033809AL

// wse, code=Receiver, subcode=WS Eventing SourceShuttingDown, details=,
//
// MessageId: ERROR_WSMAN_EVENTING_SUBSCRIPTIONCLOSED_BYREMOTESERVICE
//
// MessageText:
//
// The WS-Management service on the remote machine with which this subscription had been set up has requested that the subscription be closed.
// This can happen if the WS-Management service on the remote machine was being shutdown.
// To correct this problem restart the WS-Management service on the remote machine and re-create the subscription.
//
#define ERROR_WSMAN_EVENTING_SUBSCRIPTIONCLOSED_BYREMOTESERVICE 0x8033809BL

// wse, code=Receiver, subcode=WS Eventing DeliveryFailure, details=, 
//
// MessageId: ERROR_WSMAN_EVENTING_DELIVERYFAILED_FROMSOURCE
//
// MessageText:
//
// The event source was unable to deliver events to the client.This can happen due to network issues preventing the source from connecting
// to the client.
//
#define ERROR_WSMAN_EVENTING_DELIVERYFAILED_FROMSOURCE 0x8033809CL

//
// MessageId: ERROR_WSMAN_SECURITY_UNMAPPED
//
// MessageText:
//
// An unknown security error occurred.
//
#define ERROR_WSMAN_SECURITY_UNMAPPED    0x8033809DL

// wse, code=Receiver, subcode=WS Eventing SourceCancelling, details=, 
//
// MessageId: ERROR_WSMAN_EVENTING_SUBSCRIPTION_CANCELLED_BYSOURCE
//
// MessageText:
//
// The event source cancelled the subscription session.
//
#define ERROR_WSMAN_EVENTING_SUBSCRIPTION_CANCELLED_BYSOURCE 0x8033809EL

//
// MessageId: ERROR_WSMAN_INVALID_HOSTNAME_PATTERN
//
// MessageText:
//
// TrustedHosts list contains an invalid hostname or hostname pattern.
//
#define ERROR_WSMAN_INVALID_HOSTNAME_PATTERN 0x8033809FL

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO
//
// MessageText:
//
// The subscribe packet does not have NotifyTo element in the delivery section. 
//
#define ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO 0x803380A0L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO_ADDRESSS
//
// MessageText:
//
// The subscribe packet does not have Address element in the NotifyTo section. 
//
#define ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO_ADDRESSS 0x803380A1L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_NOTIFYTO_ADDRESSS
//
// MessageText:
//
// The subscribe packet contains invalid Address in the NotifyTo section. 
//
#define ERROR_WSMAN_EVENTING_INVALID_NOTIFYTO_ADDRESSS 0x803380A2L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_LOCALE_IN_DELIVERY
//
// MessageText:
//
// The subscribe packet contains invalid Locale value in the delivery section.
//
#define ERROR_WSMAN_EVENTING_INVALID_LOCALE_IN_DELIVERY 0x803380A3L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_HEARTBEAT
//
// MessageText:
//
// The subscribe packet contains invalid heartbeat value.
//
#define ERROR_WSMAN_EVENTING_INVALID_HEARTBEAT 0x803380A4L

//
// MessageId: ERROR_WSMAN_MACHINE_OPTION_REQUIRED
//
// MessageText:
//
// The WS-Management service cannot process the request. This request is valid only when the -remote option is specified. 
//
#define ERROR_WSMAN_MACHINE_OPTION_REQUIRED 0x803380A5L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details=/OptionSet
//
// MessageId: ERROR_WSMAN_UNSUPPORTED_FEATURE_OPTIONS
//
// MessageText:
//
// The WS-Management service does not support the options feature for the specified resource. Remove the options from the request and retry.
//
#define ERROR_WSMAN_UNSUPPORTED_FEATURE_OPTIONS 0x803380A6L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_BATCHSIZE_TOO_SMALL
//
// MessageText:
//
// The subscribe packet contains batch size value which is smaller than supported value.
//
#define ERROR_WSMAN_BATCHSIZE_TOO_SMALL  0x803380A7L

// wse, code=Sender, subcode=WS-Eventing DeliveryModeRequestedUnavailable, details=List of delivery modes that are supported
//
// MessageId: ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_INVALID
//
// MessageText:
//
// The WS-Management service cannot process the subscribe request. The delivery mode is either invalid or missing.
//
#define ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_INVALID 0x803380A8L

//
// MessageId: ERROR_WSMAN_PROVSYS_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service cannot process the request. The provider method was not found.
//
#define ERROR_WSMAN_PROVSYS_NOT_SUPPORTED 0x803380A9L

//
// MessageId: ERROR_WSMAN_PUSH_SUBSCRIPTION_CONFIG_INVALID
//
// MessageText:
//
// The WinRM client could not create a push subscription because there are no listeners configured that match the specified hostname and transport, or because there is no enabled firewall exception on the port used by the selected listener.
// Change the hostname and transport, create an appropriate firewall exception, or run winrm quickconfig.
//
#define ERROR_WSMAN_PUSH_SUBSCRIPTION_CONFIG_INVALID 0x803380AAL

//
// MessageId: ERROR_WSMAN_CREDS_PASSED_WITH_NO_AUTH_FLAG
//
// MessageText:
//
// The WinRM client could not process the request because credentials were specified along with the 'no authentication' flag.  
// No user name, password or client certificate should be specified with the 'no authentication' option.
//
#define ERROR_WSMAN_CREDS_PASSED_WITH_NO_AUTH_FLAG 0x803380ABL

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for this request.
// Remove or change the invalid flag and try the request again.
//
#define ERROR_WSMAN_CLIENT_INVALID_FLAG  0x803380ACL

//
// MessageId: ERROR_WSMAN_CLIENT_MULTIPLE_AUTH_FLAGS
//
// MessageText:
//
// The WinRM client cannot process the request. The request must specify only one authentication mechanism.
// If the No Authentication flag is set, no authentication mechanism should be specified.
// Change the request to specify only one authentication mechanism or 'no authentication' and try again.
//
#define ERROR_WSMAN_CLIENT_MULTIPLE_AUTH_FLAGS 0x803380ADL

//
// MessageId: ERROR_WSMAN_CLIENT_SPN_WRONG_AUTH
//
// MessageText:
//
// The WinRM client cannot process the request. The SPN Server Port can only be used when the authentication mechanism is Negotiate or Kerberos.
// Remove the SPN Server Port or change the authentication mechanism and try the request again.
//
#define ERROR_WSMAN_CLIENT_SPN_WRONG_AUTH 0x803380AEL

//
// MessageId: ERROR_WSMAN_CLIENT_CERT_UNNEEDED_CREDS
//
// MessageText:
//
// The WinRM client cannot process the request. The request must not include credentials when using a smart card or default certificate.
// Remove the credentials or change the authentication mechanism and try the request again.
//
#define ERROR_WSMAN_CLIENT_CERT_UNNEEDED_CREDS 0x803380AFL

//
// MessageId: ERROR_WSMAN_CLIENT_USERNAME_PASSWORD_NEEDED
//
// MessageText:
//
// The WinRM client cannot process the request. Requests must include user name and password when Basic or Digest authentication mechanism is used.
// Add the user name and password or change the authentication mechanism and try the request again.
//
#define ERROR_WSMAN_CLIENT_USERNAME_PASSWORD_NEEDED 0x803380B0L

//
// MessageId: ERROR_WSMAN_CLIENT_CERT_UNNEEDED_USERNAME
//
// MessageText:
//
// The WinRM client cannot process the request. Requests must not include user name and password when a certificate is used for authentication.
// Remove the user name and password or change the authentication mechanism and try the request again.
//
#define ERROR_WSMAN_CLIENT_CERT_UNNEEDED_USERNAME 0x803380B1L

//
// MessageId: ERROR_WSMAN_CLIENT_CREDENTIALS_NEEDED
//
// MessageText:
//
// The WinRM client cannot process the request. Requests must include credentials if they specify the following flag: WSManFlagCredUsernamePassword.
// Add the credentials or remove the WSManFlagCredUsernamePassword flag and try the request again.
//
#define ERROR_WSMAN_CLIENT_CREDENTIALS_NEEDED 0x803380B2L

//
// MessageId: ERROR_WSMAN_CLIENT_CREDENTIALS_FLAG_NEEDED
//
// MessageText:
//
// The WinRM client cannot process the request. Requests with credentials must include the following flag: WSManFlagCredUsernamePassword.
// Add the flag or remove the credentials and try the request again.
//
#define ERROR_WSMAN_CLIENT_CREDENTIALS_FLAG_NEEDED 0x803380B3L

//
// MessageId: ERROR_WSMAN_CLIENT_CERT_NEEDED
//
// MessageText:
//
// The WinRM client cannot process the request. Requests must include the certificate thumbprint when a certificate is used for authentication.
// Change the request to include the certificate thumbprint and try again.
//
#define ERROR_WSMAN_CLIENT_CERT_NEEDED   0x803380B4L

//
// MessageId: ERROR_WSMAN_CLIENT_CERT_UNKNOWN_TYPE
//
// MessageText:
//
// The WinRM client cannot process the request. Requests must include the type of certificate to use for authentication.
// Change the request to include the type of the certificate and try again.
//
#define ERROR_WSMAN_CLIENT_CERT_UNKNOWN_TYPE 0x803380B5L

//
// MessageId: ERROR_WSMAN_CLIENT_CERT_UNKNOWN_LOCATION
//
// MessageText:
//
// The WinRM client cannot process the request. Requests must include the location (machine or user certificate store) of the certificate used for authentication.
// Change the request to include the location of the certificate and try again.
//
#define ERROR_WSMAN_CLIENT_CERT_UNKNOWN_LOCATION 0x803380B6L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_CERT
//
// MessageText:
//
// The WinRM client cannot process the request. The certificate structure was incomplete.
// Change the certificate structure and try the request again.
//
#define ERROR_WSMAN_CLIENT_INVALID_CERT  0x803380B7L

//
// MessageId: ERROR_WSMAN_CLIENT_LOCAL_INVALID_CREDS
//
// MessageText:
//
// The WinRM client cannot process the request. Credentials must not be provided for local requests.
// Remove the credentials and try the request again.
//
#define ERROR_WSMAN_CLIENT_LOCAL_INVALID_CREDS 0x803380B8L

//
// MessageId: ERROR_WSMAN_CLIENT_LOCAL_INVALID_CONNECTION_OPTIONS
//
// MessageText:
//
// The WinRM client cannot process the request. Connection options must not be provided for local requests.
// Remove the connection options and try the request again.
//
#define ERROR_WSMAN_CLIENT_LOCAL_INVALID_CONNECTION_OPTIONS 0x803380B9L

//
// MessageId: ERROR_WSMAN_CLIENT_CREATESESSION_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManCreateSession function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_CREATESESSION_NULL_PARAM 0x803380BAL

//
// MessageId: ERROR_WSMAN_CLIENT_ENUMERATE_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManEnumerate function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_ENUMERATE_NULL_PARAM 0x803380BBL

//
// MessageId: ERROR_WSMAN_CLIENT_SUBSCRIBE_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManSubscribe function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_SUBSCRIBE_NULL_PARAM 0x803380BCL

//
// MessageId: ERROR_WSMAN_CLIENT_NULL_RESULT_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. The parameter that should contain the result of the request is null.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_NULL_RESULT_PARAM 0x803380BDL

//
// MessageId: ERROR_WSMAN_CLIENT_NO_HANDLE
//
// MessageText:
//
// The WinRM client cannot process the request. The request is missing the session or enumeration handle.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_NO_HANDLE     0x803380BEL

//
// MessageId: ERROR_WSMAN_CLIENT_BLANK_URI
//
// MessageText:
//
// The WinRM client cannot process the request. The resource URI must not be "" (blank or empty string) or NULL.
// Change the resource URI and try the request again.
//
#define ERROR_WSMAN_CLIENT_BLANK_URI     0x803380BFL

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_RESOURCE_LOCATOR
//
// MessageText:
//
// The WinRM client cannot process the request. The resource locator was invalid.
// Change the resource locator and try the request again.
//
#define ERROR_WSMAN_CLIENT_INVALID_RESOURCE_LOCATOR 0x803380C0L

//
// MessageId: ERROR_WSMAN_CLIENT_BLANK_INPUT_XML
//
// MessageText:
//
// The WinRM client cannot process the request. The input XML must not be "" (blank or empty string) or NULL.
// Change the input XML and try the request again.
//
#define ERROR_WSMAN_CLIENT_BLANK_INPUT_XML 0x803380C1L

//
// MessageId: ERROR_WSMAN_CLIENT_BATCH_ITEMS_TOO_SMALL
//
// MessageText:
//
// The WinRM client cannot process the request. The maximum number of elements to be retrieved in a batch is too small.
// Change the value for the maximum number of elements in a batch and try the request again.
//
#define ERROR_WSMAN_CLIENT_BATCH_ITEMS_TOO_SMALL 0x803380C2L

//
// MessageId: ERROR_WSMAN_CLIENT_MAX_CHARS_TOO_SMALL
//
// MessageText:
//
// The WinRM client cannot process the request. The maximum number of characters to be retrieved in a batch is too small.
// Change the value for the maximum number of characters in a batch and try the request again.
//
#define ERROR_WSMAN_CLIENT_MAX_CHARS_TOO_SMALL 0x803380C3L

//
// MessageId: ERROR_WSMAN_CLIENT_BLANK_ACTION_URI
//
// MessageText:
//
// The WinRM client cannot process the request. The action URI must not be "" (blank or empty string) or NULL.
// Change the action URI and try the request again.
//
#define ERROR_WSMAN_CLIENT_BLANK_ACTION_URI 0x803380C4L

//
// MessageId: ERROR_WSMAN_CLIENT_ZERO_HEARTBEAT
//
// MessageText:
//
// The WinRM client cannot process the request. The heartbeat interval must be greater than 0.
// Change the heartbeat interval and try the request again.
//
#define ERROR_WSMAN_CLIENT_ZERO_HEARTBEAT 0x803380C5L

//
// MessageId: ERROR_WSMAN_CLIENT_MULTIPLE_DELIVERY_MODES
//
// MessageText:
//
// The WinRM client cannot process the request. The request must contain one and only one delivery mode.
// Change the request to contain only one delivery mode and try again.
//
#define ERROR_WSMAN_CLIENT_MULTIPLE_DELIVERY_MODES 0x803380C6L

//
// MessageId: ERROR_WSMAN_CLIENT_MULTIPLE_ENVELOPE_POLICIES
//
// MessageText:
//
// The WinRM client cannot process the request. The request contained multiple settings for the policy regarding the maximum envelope size.
// Change the request to contain only one setting for the policy and try again.
//
#define ERROR_WSMAN_CLIENT_MULTIPLE_ENVELOPE_POLICIES 0x803380C7L

//
// MessageId: ERROR_WSMAN_CLIENT_UNKNOWN_EXPIRATION_TYPE
//
// MessageText:
//
// The WinRM client cannot process the request. The request contained an expiration time, but did not specify if it was absolute or relative.
// Change the request to specify the type of the expiration time (absolute or relative) and try again.
//
#define ERROR_WSMAN_CLIENT_UNKNOWN_EXPIRATION_TYPE 0x803380C8L

//
// MessageId: ERROR_WSMAN_CLIENT_MISSING_EXPIRATION
//
// MessageText:
//
// The WinRM client cannot process the request. The request specified the type of the expiration time (absolute or relative) but it did not contain an expiration time.
// Change the request to include the expiration time and try again.
//
#define ERROR_WSMAN_CLIENT_MISSING_EXPIRATION 0x803380C9L

//
// MessageId: ERROR_WSMAN_CLIENT_PULL_INVALID_FLAGS
//
// MessageText:
//
// The WinRM client cannot process the request. The pull subscription request contained flags related to a push subscription.
// Change the flags and try the request again.
//
#define ERROR_WSMAN_CLIENT_PULL_INVALID_FLAGS 0x803380CAL

//
// MessageId: ERROR_WSMAN_CLIENT_PUSH_UNSUPPORTED_TRANSPORT
//
// MessageText:
//
// The WinRM client cannot process the request because the push subscription request contained an unsupported delivery transport. HTTP and HTTPS are the only currently supported transports.
// Change the delivery transport and try the request again.
//
#define ERROR_WSMAN_CLIENT_PUSH_UNSUPPORTED_TRANSPORT 0x803380CBL

//
// MessageId: ERROR_WSMAN_CLIENT_PUSH_HOST_TOO_LONG
//
// MessageText:
//
// The WinRM client cannot process the request. The delivery address for push subscriptions was too long.
// Change the delivery address and try the request again.
//
#define ERROR_WSMAN_CLIENT_PUSH_HOST_TOO_LONG 0x803380CCL

//
// MessageId: ERROR_WSMAN_CLIENT_COMPRESSION_INVALID_OPTION
//
// MessageText:
//
// The WinRM client cannot process the request. The request contained the compression option but contained an unrecognized value.
// Change the value for the compression option and try the request again.
//
#define ERROR_WSMAN_CLIENT_COMPRESSION_INVALID_OPTION 0x803380CDL

//
// MessageId: ERROR_WSMAN_CLIENT_DELIVERENDSUBSCRIPTION_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManDeliverEndSubscriptionNotification function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_DELIVERENDSUBSCRIPTION_NULL_PARAM 0x803380CEL

//
// MessageId: ERROR_WSMAN_CLIENT_DELIVEREVENTS_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManDeliverEvents function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_DELIVEREVENTS_NULL_PARAM 0x803380CFL

//
// MessageId: ERROR_WSMAN_CLIENT_GETBOOKMARK_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManGetBookmark function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_GETBOOKMARK_NULL_PARAM 0x803380D0L

//
// MessageId: ERROR_WSMAN_CLIENT_DECODEOBJECT_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManDecodeObject function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_DECODEOBJECT_NULL_PARAM 0x803380D1L

//
// MessageId: ERROR_WSMAN_CLIENT_ENCODEOBJECT_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManEncodeObject(Ex) function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_ENCODEOBJECT_NULL_PARAM 0x803380D2L

//
// MessageId: ERROR_WSMAN_CLIENT_ENUMERATORADDOBJECT_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManEnumeratorAddObject function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_ENUMERATORADDOBJECT_NULL_PARAM 0x803380D3L

//
// MessageId: ERROR_WSMAN_CLIENT_ENUMERATORNEXTOBJECT_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManEnumeratorNextObject function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_ENUMERATORNEXTOBJECT_NULL_PARAM 0x803380D4L

//
// MessageId: ERROR_WSMAN_CLIENT_CONSTRUCTERROR_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManConstructError function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_CONSTRUCTERROR_NULL_PARAM 0x803380D5L

//
// MessageId: ERROR_WSMAN_SERVER_NONPULLSUBSCRIBE_NULL_PARAM
//
// MessageText:
//
// The WinRM service cannot process the request. Push subscriptions are not supported for local session. Change subscription type to Pull and try again.
//
#define ERROR_WSMAN_SERVER_NONPULLSUBSCRIBE_NULL_PARAM 0x803380D6L

//
// MessageId: ERROR_WSMAN_CLIENT_UNENCRYPTED_HTTP_ONLY
//
// MessageText:
//
// The WinRM client cannot process the request. The unencrypted flag only applies to the HTTP transport.
// Remove the unencrypted flag or change the transport and try again the request.
//
#define ERROR_WSMAN_CLIENT_UNENCRYPTED_HTTP_ONLY 0x803380D7L

//
// MessageId: ERROR_WSMAN_CANNOT_USE_CERTIFICATES_FOR_HTTP
//
// MessageText:
//
// The WinRM client cannot process the request. Certificate parameters are not valid when the HTTP transport is also specified.
// Remove the certificate parameters or change the transport and try again the request.
//
#define ERROR_WSMAN_CANNOT_USE_CERTIFICATES_FOR_HTTP 0x803380D8L

//
// MessageId: ERROR_WSMAN_CONNECTIONSTR_INVALID
//
// MessageText:
//
// The WinRM client cannot process the request. The connection string should be of the form [<transport>://]<host>[:<port>][/<suffix>] where transport is one of "http" or "https".
// Transport, port and suffix are optional. The host may be a hostname or an IP address.
// For IPv6 addresses, enclose the address in brackets - e.g. "http://[1::2]:80/wsman".
// Change the connection string and try the request again.
//
#define ERROR_WSMAN_CONNECTIONSTR_INVALID 0x803380D9L

//
// MessageId: ERROR_WSMAN_TRANSPORT_NOT_SUPPORTED
//
// MessageText:
//
// The WinRM client cannot process the request. The connection string contains an unsupported transport.
// Valid transports are "http" or "https".
// Change the connection string and try the request again.
//
#define ERROR_WSMAN_TRANSPORT_NOT_SUPPORTED 0x803380DAL

//
// MessageId: ERROR_WSMAN_PORT_INVALID
//
// MessageText:
//
// The WinRM client cannot process the request because the port specified in the connection string is not valid.
// Verify the port and retry the request.
// Valid values are between 1 and 65535.
// Change the value for port and try the request again.
//
#define ERROR_WSMAN_PORT_INVALID         0x803380DBL

//
// MessageId: ERROR_WSMAN_CONFIG_PORT_INVALID
//
// MessageText:
//
// The WinRM client cannot process the request. The port specified in the configuration is invalid.
// Valid values are between 1 and 65535.
// Change the value for port and try the request again.
//
#define ERROR_WSMAN_CONFIG_PORT_INVALID  0x803380DCL

//
// MessageId: ERROR_WSMAN_SENDHEARBEAT_EMPTY_ENUMERATOR
//
// MessageText:
//
// The WinRM service cannot process the request. WSMAN_FLAG_SEND_HEARTBEAT flag requires the event enumerator to be empty.
// Change the flag or change the event enumerator and try the request again.
//
#define ERROR_WSMAN_SENDHEARBEAT_EMPTY_ENUMERATOR 0x803380DDL

//
// MessageId: ERROR_WSMAN_CLIENT_UNENCRYPTED_DISABLED
//
// MessageText:
//
// The WinRM client cannot process the request. Unencrypted traffic is currently disabled in the client configuration.
// Change the client configuration and try the request again.
//
#define ERROR_WSMAN_CLIENT_UNENCRYPTED_DISABLED 0x803380DEL

//
// MessageId: ERROR_WSMAN_CLIENT_BASIC_AUTHENTICATION_DISABLED
//
// MessageText:
//
// The WinRM client cannot process the request. Basic authentication is currently disabled in the client configuration.
// Change the client configuration and try the request again.
//
#define ERROR_WSMAN_CLIENT_BASIC_AUTHENTICATION_DISABLED 0x803380DFL

//
// MessageId: ERROR_WSMAN_CLIENT_DIGEST_AUTHENTICATION_DISABLED
//
// MessageText:
//
// The WinRM client cannot process the request. Digest authentication is currently disabled in the client configuration.
// Change the client configuration and try the request again.
//
#define ERROR_WSMAN_CLIENT_DIGEST_AUTHENTICATION_DISABLED 0x803380E0L

//
// MessageId: ERROR_WSMAN_CLIENT_NEGOTIATE_AUTHENTICATION_DISABLED
//
// MessageText:
//
// The WinRM client cannot process the request. Negotiate authentication is currently disabled in the client configuration.
// Change the client configuration and try the request again.
// If this is a request for the local configuration, use one of the enabled authentication mechanisms still enabled.
// To use Kerberos, specify the local computer name as the remote destination. 
// To use Basic, specify the local computer name as the remote destination, specify Basic authentication and provide user name and password.
//
#define ERROR_WSMAN_CLIENT_NEGOTIATE_AUTHENTICATION_DISABLED 0x803380E1L

//
// MessageId: ERROR_WSMAN_CLIENT_KERBEROS_AUTHENTICATION_DISABLED
//
// MessageText:
//
// The WinRM client cannot process the request. Kerberos authentication is currently disabled in the client configuration.
// Change the client configuration and try the request again.
//
#define ERROR_WSMAN_CLIENT_KERBEROS_AUTHENTICATION_DISABLED 0x803380E2L

//
// MessageId: ERROR_WSMAN_CLIENT_CERTIFICATES_AUTHENTICATION_DISABLED
//
// MessageText:
//
// The WinRM client cannot process the request. Certificate authentication is currently disabled in the client configuration.
// Change the client configuration and try the request again.
//
#define ERROR_WSMAN_CLIENT_CERTIFICATES_AUTHENTICATION_DISABLED 0x803380E3L

//
// MessageId: ERROR_WSMAN_SERVER_NOT_TRUSTED
//
// MessageText:
//
// The WinRM client cannot process the request. If the authentication scheme is different from Kerberos, or if the client computer is not joined to a domain,
// then HTTPS transport must be used or the destination machine must be added to the TrustedHosts configuration setting.
// Use winrm.cmd to configure TrustedHosts. Note that computers in the TrustedHosts list might not be authenticated.
// You can get more information about that by running the following command: winrm help config.
//
#define ERROR_WSMAN_SERVER_NOT_TRUSTED   0x803380E4L

//
// MessageId: ERROR_WSMAN_EXPLICIT_CREDENTIALS_REQUIRED
//
// MessageText:
//
// The WinRM client cannot process the request. 
// Default credentials with Negotiate over HTTP can be used only if the target machine is part of the TrustedHosts list or the Allow implicit credentials for Negotiate option is specified.
//
#define ERROR_WSMAN_EXPLICIT_CREDENTIALS_REQUIRED 0x803380E5L

//
// MessageId: ERROR_WSMAN_CERT_THUMBPRINT_NOT_BLANK
//
// MessageText:
//
// The WinRM client cannot process the request. The CertificateThumbprint property must be empty when the SSL configuration will be shared with another service.
//
#define ERROR_WSMAN_CERT_THUMBPRINT_NOT_BLANK 0x803380E6L

//
// MessageId: ERROR_WSMAN_CERT_THUMBPRINT_BLANK
//
// MessageText:
//
// The WinRM client cannot process the request. The CertificateThumbprint property must not be "" (blank or empty string) or NULL.
//
#define ERROR_WSMAN_CERT_THUMBPRINT_BLANK 0x803380E7L

//
// MessageId: ERROR_WSMAN_CONFIG_CANNOT_SHARE_SSL_CONFIG
//
// MessageText:
//
// The WinRM client cannot process the request. The WinRM client tried to create an SSL configuration for a pair of IP address and port according to the request,
// but the SSL configuration for that pair is owned by another service and cannot be shared. 
// Use a different IP address and port combination and try the request again.
//
#define ERROR_WSMAN_CONFIG_CANNOT_SHARE_SSL_CONFIG 0x803380E8L

//
// MessageId: ERROR_WSMAN_CONFIG_CERT_CN_DOES_NOT_MATCH_HOSTNAME
//
// MessageText:
//
// The WinRM client cannot process the request. The certificate CN and the hostname that were provided do not match.
//
#define ERROR_WSMAN_CONFIG_CERT_CN_DOES_NOT_MATCH_HOSTNAME 0x803380E9L

//
// MessageId: ERROR_WSMAN_CONFIG_HOSTNAME_CHANGE_WITHOUT_CERT
//
// MessageText:
//
// <not used>
//
#define ERROR_WSMAN_CONFIG_HOSTNAME_CHANGE_WITHOUT_CERT 0x803380EAL

//
// MessageId: ERROR_WSMAN_CONFIG_THUMBPRINT_SHOULD_BE_EMPTY
//
// MessageText:
//
// The WinRM client cannot process the request. When HTTP is the transport, the Certificate thumbprint must be blank.
// HTTP does not use the Certificate thumbprint.
//
#define ERROR_WSMAN_CONFIG_THUMBPRINT_SHOULD_BE_EMPTY 0x803380EBL

//
// MessageId: ERROR_WSMAN_INVALID_IPFILTER
//
// MessageText:
//
// The WinRM client cannot process the request. The IP Filter is invalid.
// Ranges are specified using the syntax IP1-IP2. Multiple ranges are separated using , as delimiter.
// * is used to indicate that the service should listen on all available IPs on the machine.
// When * is used, other ranges in the filter are ignored. If filter is blank, the service doesn't listen on any address.
// For example, if service should be restricted to listen on only IPv4 addresses, IPv6 filter should be left empty.
// %nExample IPv4 filters:
// 2.0.0.1-2.0.0.20, 24.0.0.1-24.0.0.22
// 
// %n Example IPv6 filters:
// 3FFE:FFFF:7654:FEDA:1245:BA98:0000:0000-3FFE:FFFF:7654:FEDA:1245:BA98:3210:4562
//
#define ERROR_WSMAN_INVALID_IPFILTER     0x803380ECL

//
// MessageId: ERROR_WSMAN_CANNOT_CHANGE_KEYS
//
// MessageText:
//
// The WinRM client cannot process the request. The input XML modifies selectors or keys for the instance.
// You cannot create a new instance or change the identity of an instance by changing the keys.
// Change the input XML and try the request again.
//
#define ERROR_WSMAN_CANNOT_CHANGE_KEYS   0x803380EDL

//
// MessageId: ERROR_WSMAN_CERT_INVALID_USAGE
//
// MessageText:
//
// The WinRM client cannot process the request. The Enhanced Key Usage (EKU) field of the certificate is not set to "Server Authentication".
// Retry the request with a certificate that has the correct EKU.
//
#define ERROR_WSMAN_CERT_INVALID_USAGE   0x803380EEL

//
// MessageId: ERROR_WSMAN_RESPONSE_NO_RESULTS
//
// MessageText:
//
// The WinRM client cannot process the request. The response from the destination computer does not include any results.
//
#define ERROR_WSMAN_RESPONSE_NO_RESULTS  0x803380EFL

//
// MessageId: ERROR_WSMAN_CREATE_RESPONSE_NO_EPR
//
// MessageText:
//
// The WinRM client cannot process the request. The response to a create request did not contain a valid end point reference.
// The ResourceCreated element was not found or did not contain valid content.
//
#define ERROR_WSMAN_CREATE_RESPONSE_NO_EPR 0x803380F0L

//
// MessageId: ERROR_WSMAN_RESPONSE_INVALID_ENUMERATION_CONTEXT
//
// MessageText:
//
// The WinRM client cannot process the request. The response from the destination computer does not contain a valid SOAP enumeration context.
//
#define ERROR_WSMAN_RESPONSE_INVALID_ENUMERATION_CONTEXT 0x803380F1L

//
// MessageId: ERROR_WSMAN_RESPONSE_NO_XML_FRAGMENT_WRAPPER
//
// MessageText:
//
// The WinRM client cannot process the request. The response from the destination computer contains a WS-Management FragmentTransfer header but the content of the body is not wrapped by the WS-Management XmlFragment wrapper.
//
#define ERROR_WSMAN_RESPONSE_NO_XML_FRAGMENT_WRAPPER 0x803380F2L

//
// MessageId: ERROR_WSMAN_RESPONSE_INVALID_MESSAGE_INFORMATION_HEADER
//
// MessageText:
//
// The WinRM client cannot process the request. The response from the destination computer contains one or more invalid SOAP headers.
//
#define ERROR_WSMAN_RESPONSE_INVALID_MESSAGE_INFORMATION_HEADER 0x803380F3L

//
// MessageId: ERROR_WSMAN_RESPONSE_NO_SOAP_HEADER_BODY
//
// MessageText:
//
// The WinRM client cannot process the request. It cannot find any SOAP Headers or Body elements in the response from the destination computer.
//
#define ERROR_WSMAN_RESPONSE_NO_SOAP_HEADER_BODY 0x803380F4L

//
// MessageId: ERROR_WSMAN_HTTP_NO_RESPONSE_DATA
//
// MessageText:
//
// The WinRM client cannot process the request. The destination computer returned an empty response to the request.
//
#define ERROR_WSMAN_HTTP_NO_RESPONSE_DATA 0x803380F5L

//
// MessageId: ERROR_WSMAN_RESPONSE_INVALID_SOAP_FAULT
//
// MessageText:
//
// The WinRM client cannot process the request. The destination computer returned an invalid SOAP fault.
//
#define ERROR_WSMAN_RESPONSE_INVALID_SOAP_FAULT 0x803380F6L

//
// MessageId: ERROR_WSMAN_HTTP_INVALID_CONTENT_TYPE_IN_RESPONSE_DATA
//
// MessageText:
//
// The WinRM client cannot process the request. It cannot determine the content type of the HTTP response from the destination computer. The content type is absent or invalid.
//
#define ERROR_WSMAN_HTTP_INVALID_CONTENT_TYPE_IN_RESPONSE_DATA 0x803380F7L

//
// MessageId: ERROR_WSMAN_HTTP_CONTENT_TYPE_MISSMATCH_RESPONSE_DATA
//
// MessageText:
//
// The WinRM client cannot process the request. The HTTP response from the destination computer was not in the same format as the request. A Unicode request packet may have been sent and an ANSI packet received.
//
#define ERROR_WSMAN_HTTP_CONTENT_TYPE_MISSMATCH_RESPONSE_DATA 0x803380F8L

//
// MessageId: ERROR_WSMAN_CANNOT_DECRYPT
//
// MessageText:
//
// The WinRM client cannot process the request. The encrypted message body has an invalid format and cannot be decrypted. Ensure that the service is encrypting the message body according to the specifications.
//
#define ERROR_WSMAN_CANNOT_DECRYPT       0x803380F9L

//
// MessageId: ERROR_WSMAN_INVALID_URI_WMI_SINGLETON
//
// MessageText:
//
// The WinRM client cannot process the request. The resource URI is not valid: it does not contain keys, but the class selected is not a singleton.
// To access an instance which is not a singleton, keys must be provided.
// Use the following command to get more information in how to construct a resource URI: "winrm help uris".
//
#define ERROR_WSMAN_INVALID_URI_WMI_SINGLETON 0x803380FAL

//
// MessageId: ERROR_WSMAN_INVALID_URI_WMI_ENUM_WQL
//
// MessageText:
//
// The WinRM client cannot process the request. The resource URI for an enumeration operation with WQL filter must not contain keys and the class name must be '*' (star).
// Use the following command to get more information in how to construct a resource URI: "winrm help uris".
//
#define ERROR_WSMAN_INVALID_URI_WMI_ENUM_WQL 0x803380FBL

//
// MessageId: ERROR_WSMAN_NO_IDENTIFY_FOR_LOCAL_SESSION
//
// MessageText:
//
// The WS-Management identification operation is only available on remote sessions.
//
#define ERROR_WSMAN_NO_IDENTIFY_FOR_LOCAL_SESSION 0x803380FCL

//
// MessageId: ERROR_WSMAN_NO_PUSH_SUBSCRIPTION_FOR_LOCAL_SESSION
//
// MessageText:
//
// Subscribe operation with Push delivery mode is only available on remote sessions.
//
#define ERROR_WSMAN_NO_PUSH_SUBSCRIPTION_FOR_LOCAL_SESSION 0x803380FDL

//
// MessageId: ERROR_WSMAN_INVALID_SUBSCRIPTION_MANAGER
//
// MessageText:
//
// The subscription manager address is invalid. The response was not received from the address to which the subscription request was sent.
//
#define ERROR_WSMAN_INVALID_SUBSCRIPTION_MANAGER 0x803380FEL

//
// MessageId: ERROR_WSMAN_NON_PULL_SUBSCRIPTION_NOT_SUPPORTED
//
// MessageText:
//
// Only subscriptions with Pull delivery mode are supported by the plugin.
//
#define ERROR_WSMAN_NON_PULL_SUBSCRIPTION_NOT_SUPPORTED 0x803380FFL

//
// MessageId: ERROR_WSMAN_WMI_MAX_NESTED
//
// MessageText:
//
// WinRM cannot process the request because the WMI object contains too many levels of nested embedded objects.
//
#define ERROR_WSMAN_WMI_MAX_NESTED       0x80338100L

//
// MessageId: ERROR_WSMAN_REMOTE_CIMPATH_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service cannot process the request. 
// It does not support retrieving a WMI object that contains a property of type CIM_REFERENCE and the value of that property contains a remote machine name.
//
#define ERROR_WSMAN_REMOTE_CIMPATH_NOT_SUPPORTED 0x80338101L

//
// MessageId: ERROR_WSMAN_WMI_PROVIDER_NOT_CAPABLE
//
// MessageText:
//
// The WS-Management service cannot process the request. The WMI service reported that the WMI provider could not perform the requested operation.
//
#define ERROR_WSMAN_WMI_PROVIDER_NOT_CAPABLE 0x80338102L

//
// MessageId: ERROR_WSMAN_WMI_INVALID_VALUE
//
// MessageText:
//
// The WS-Management service cannot process the request. A value retrieved from the WMI service or the WMI provider is invalid.
//
#define ERROR_WSMAN_WMI_INVALID_VALUE    0x80338103L

//
// MessageId: ERROR_WSMAN_WMI_SVC_ACCESS_DENIED
//
// MessageText:
//
// The WS-Management service cannot process the request. The WMI service returned an 'access denied' error.
//
#define ERROR_WSMAN_WMI_SVC_ACCESS_DENIED 0x80338104L

//
// MessageId: ERROR_WSMAN_WMI_PROVIDER_ACCESS_DENIED
//
// MessageText:
//
// The WS-Management service cannot process the request. The WMI provider returned an 'access denied' error.
//
#define ERROR_WSMAN_WMI_PROVIDER_ACCESS_DENIED 0x80338105L

//
// MessageId: ERROR_WSMAN_WMI_CANNOT_CONNECT_ACCESS_DENIED
//
// MessageText:
//
// The WS-Management service cannot process the request. An 'access denied' error was received when connecting to the WMI service on the computer specified.
//
#define ERROR_WSMAN_WMI_CANNOT_CONNECT_ACCESS_DENIED 0x80338106L

//
// MessageId: ERROR_WSMAN_INVALID_FILTER_XML
//
// MessageText:
//
// The WS-Management service cannot process the request because the filter XML is invalid.
//
#define ERROR_WSMAN_INVALID_FILTER_XML   0x80338107L

//
// MessageId: ERROR_WSMAN_ENUMERATE_WMI_INVALID_KEY
//
// MessageText:
//
// The WS-Management service cannot process the request. The resource URI for an Enumerate operation must not contain keys.
//
#define ERROR_WSMAN_ENUMERATE_WMI_INVALID_KEY 0x80338108L

//
// MessageId: ERROR_WSMAN_INVALID_FRAGMENT_PATH_BLANK
//
// MessageText:
//
// Cannot execute the Fragment-Level operation because the fragment path contains either "" (blank or empty string) or NULL.
// Change the value of the fragment path and try the request again.
//
#define ERROR_WSMAN_INVALID_FRAGMENT_PATH_BLANK 0x80338109L

//
// MessageId: ERROR_WSMAN_INVALID_CHARACTERS_IN_RESPONSE
//
// MessageText:
//
// The WinRM client cannot process the request. The response received from the destination machine contains invalid characters and cannot be processed.
//
#define ERROR_WSMAN_INVALID_CHARACTERS_IN_RESPONSE 0x8033810AL

//
// MessageId: ERROR_WSMAN_KERBEROS_IPADDRESS
//
// MessageText:
//
// The WinRM client cannot process the request.
// Kerberos authentication cannot be used when the destination is an IP address.
// Specify a DNS or NetBIOS destination or specify Basic or Negotiate authentication.
//
#define ERROR_WSMAN_KERBEROS_IPADDRESS   0x8033810BL

//
// MessageId: ERROR_WSMAN_CLIENT_WORKGROUP_NO_KERBEROS
//
// MessageText:
//
// The WinRM client cannot process the request. Kerberos authentication cannot be used with implicit credentials if the client computer is not joined to a domain.
// Use explicit credentials or specify a different authentication mechanism than Kerberos.
//
#define ERROR_WSMAN_CLIENT_WORKGROUP_NO_KERBEROS 0x8033810CL

//
// MessageId: ERROR_WSMAN_INVALID_BATCH_SETTINGS_PARAMETER
//
// MessageText:
//
// The WinRM client cannot process the request. The batch settings parameter is invalid.
//
#define ERROR_WSMAN_INVALID_BATCH_SETTINGS_PARAMETER 0x8033810DL

//
// MessageId: ERROR_WSMAN_SERVER_DESTINATION_LOCALHOST
//
// MessageText:
//
// The WinRM client cannot process the request. If you do not specify an authentication mechanism or you specify Kerberos, then you cannot use "localhost" or "127.0.0.1" or "[::1]" for the remote host name.
// You can explicitly specify a different authentication mechanism than Kerberos or specify the remote host as a DNS name or NetBIOS name.
//
#define ERROR_WSMAN_SERVER_DESTINATION_LOCALHOST 0x8033810EL

//
// MessageId: ERROR_WSMAN_UNKNOWN_HTTP_STATUS_RETURNED
//
// MessageText:
//
// The WinRM client received an unknown HTTP status code from the remote WS-Management service.
//
#define ERROR_WSMAN_UNKNOWN_HTTP_STATUS_RETURNED 0x8033810FL

// This error message is deprecated
//
// MessageId: ERROR_WSMAN_UNSUPPORTED_HTTP_STATUS_REDIRECT
//
// MessageText:
//
// The WinRM client received a HTTP redirect status code from the remote WS-Management service. WinRM does not support redirects.
//
#define ERROR_WSMAN_UNSUPPORTED_HTTP_STATUS_REDIRECT 0x80338110L

//
// MessageId: ERROR_WSMAN_HTTP_REQUEST_TOO_LARGE_STATUS
//
// MessageText:
//
// The WinRM client sent a request to the remote WS-Management service and was notified that the request size exceeded the configured MaxEnvelopeSize quota.
//
#define ERROR_WSMAN_HTTP_REQUEST_TOO_LARGE_STATUS 0x80338111L

//
// MessageId: ERROR_WSMAN_HTTP_SERVICE_UNAVAILABLE_STATUS
//
// MessageText:
//
// The connection to the specified remote host was refused.
// Verify that the WS-Management service is running on the remote host and configured to listen for requests on the correct port and HTTP URL.
//
#define ERROR_WSMAN_HTTP_SERVICE_UNAVAILABLE_STATUS 0x80338112L

//
// MessageId: ERROR_WSMAN_HTTP_NOT_FOUND_STATUS
//
// MessageText:
//
// The WinRM client sent a request to an HTTP server and got a response saying the requested HTTP URL was not available.
// This is usually returned by a HTTP server that does not support the WS-Management protocol.
//
#define ERROR_WSMAN_HTTP_NOT_FOUND_STATUS 0x80338113L

//
// MessageId: ERROR_WSMAN_EVENTING_MISSING_LOCALE_IN_DELIVERY
//
// MessageText:
//
// The subscribe packet had a Locale element with missing lang attribute. The lang attribute is required for the Locale element. 
//
#define ERROR_WSMAN_EVENTING_MISSING_LOCALE_IN_DELIVERY 0x80338114L

//
// MessageId: ERROR_WSMAN_QUICK_CONFIG_FAILED_CERT_REQUIRED
//
// MessageText:
//
// Cannot create a WinRM listener on HTTPS because this machine does not have an appropriate certificate. To be used for SSL, a certificate must have a CN matching the hostname, be appropriate for Server Authentication, and not be expired, revoked, or self-signed.
//
#define ERROR_WSMAN_QUICK_CONFIG_FAILED_CERT_REQUIRED 0x80338115L

//
// MessageId: ERROR_WSMAN_QUICK_CONFIG_FIREWALL_EXCEPTIONS_DISALLOWED
//
// MessageText:
//
// Firewall does not allow exceptions; WinRM cannot be setup for remote access.
//
#define ERROR_WSMAN_QUICK_CONFIG_FIREWALL_EXCEPTIONS_DISALLOWED 0x80338116L

//
// MessageId: ERROR_WSMAN_QUICK_CONFIG_LOCAL_POLICY_CHANGE_DISALLOWED
//
// MessageText:
//
// The Windows Remote Management (WinRM) service cannot be configured for remote access because Group Policy does not allow local firewall changes. Check the Group Policy settings to allow local firewall exceptions and add WinRM to the firewall exceptions.
//
#define ERROR_WSMAN_QUICK_CONFIG_LOCAL_POLICY_CHANGE_DISALLOWED 0x80338117L

//
// MessageId: ERROR_WSMAN_INVALID_SELECTOR_NAME
//
// MessageText:
//
// The WinRM client cannot process the request because the selector name is not valid. Change the selector name and retry the request.
//
#define ERROR_WSMAN_INVALID_SELECTOR_NAME 0x80338118L

//
// MessageId: ERROR_WSMAN_ENCODING_TYPE
//
// MessageText:
//
// The WS-Management service does not support the encoding type specified.
//
#define ERROR_WSMAN_ENCODING_TYPE        0x80338119L

//
// MessageId: ERROR_WSMAN_ENDPOINT_UNAVAILABLE_INVALID_VALUE
//
// MessageText:
//
// The WS-Management service cannot process the request because the selector values do not match a known resource, or the resource is offline. 
// Retry the request later when the resource is online, or try a different selector.
//
#define ERROR_WSMAN_ENDPOINT_UNAVAILABLE_INVALID_VALUE 0x8033811AL

//
// MessageId: ERROR_WSMAN_INVALID_HEADER
//
// MessageText:
//
// The WS-Management service cannot process the request because the a header in the request is invalid.
//
#define ERROR_WSMAN_INVALID_HEADER       0x8033811BL

//
// MessageId: ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TYPE
//
// MessageText:
//
// The expiration time specified for enumeration was invalid. Specify the expiration time as a duration.
//
#define ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TYPE 0x8033811CL

//
// MessageId: ERROR_WSMAN_MAX_ELEMENTS_NOT_SUPPORTED
//
// MessageText:
//
// The WS-Management service received a request which specified a maximum number of elements, but the service does not support this feature.
// Retry the request without this element specified.
//
#define ERROR_WSMAN_MAX_ELEMENTS_NOT_SUPPORTED 0x8033811DL

//
// MessageId: ERROR_WSMAN_WMI_PROVIDER_INVALID_PARAMETER
//
// MessageText:
//
// The WS-Management service cannot process the request. The WMI provider returned an 'invalid parameter' error.
//
#define ERROR_WSMAN_WMI_PROVIDER_INVALID_PARAMETER 0x8033811EL

//
// MessageId: ERROR_WSMAN_CLIENT_MULTIPLE_ENUM_MODE_FLAGS
//
// MessageText:
//
// The WinRM client cannot process the request. The request must contain one and only one enumeration mode.
// Change the request to contain only one enumeration mode and try again.
//
#define ERROR_WSMAN_CLIENT_MULTIPLE_ENUM_MODE_FLAGS 0x8033811FL

//
// MessageId: ERROR_WINRS_CLIENT_INVALID_FLAG
//
// MessageText:
//
// The WinRS client cannot process the request. An invalid flag was specified for this request.
// Remove or change the invalid flag and try the request again.
//
#define ERROR_WINRS_CLIENT_INVALID_FLAG  0x80338120L

//
// MessageId: ERROR_WINRS_CLIENT_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. One of the parameters required is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_NULL_PARAM    0x80338121L

// wsman, code=Sender, subcode=WS-Management CannotProcessFilter, details=
//
// MessageId: ERROR_WSMAN_CANNOT_PROCESS_FILTER
//
// MessageText:
//
// The data source could not process the filter. The filter might be missing, invalid or too complex to process.
// If a service only supports a subset of a filter dialect (such as XPath level 1), it may return this fault for valid
// filter expressions outside of the supported subset.
// Change the filter and try the request again. 
//
#define ERROR_WSMAN_CANNOT_PROCESS_FILTER 0x80338122L

//
// MessageId: ERROR_WSMAN_CLIENT_ENUMERATORADDEVENT_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManEnumeratorAddEvent function is null or zero. Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_ENUMERATORADDEVENT_NULL_PARAM 0x80338123L

//
// MessageId: ERROR_WSMAN_ADDOBJECT_MISSING_OBJECT
//
// MessageText:
//
// The WinRM client cannot process the request. The object parameter for the WSManEnumeratorAddObject function is null or zero, but the enumeration mode is Object or ObjectAndEPR.
//
#define ERROR_WSMAN_ADDOBJECT_MISSING_OBJECT 0x80338124L

//
// MessageId: ERROR_WSMAN_ADDOBJECT_MISSING_EPR
//
// MessageText:
//
// The WinRM client cannot process the request. The EPR parameter for the WSManEnumeratorAddObject function is null or zero, but the enumeration mode is EPR or ObjectAndEPR.
//
#define ERROR_WSMAN_ADDOBJECT_MISSING_EPR 0x80338125L

// Returned by client when get timeout from network layer
//
// MessageId: ERROR_WSMAN_NETWORK_TIMEDOUT
//
// MessageText:
//
// WinRM cannot complete the operation.
// Verify that the specified computer name is valid, that the computer is accessible over the network, and that a firewall exception for the WinRM service is enabled and allows access from this computer.
// By default, the WinRM firewall exception for public profiles limits access to remote computers within the same local subnet.
//
#define ERROR_WSMAN_NETWORK_TIMEDOUT     0x80338126L

//
// MessageId: ERROR_WINRS_RECEIVE_IN_PROGRESS
//
// MessageText:
//
// Not used. To be removed.
//
#define ERROR_WINRS_RECEIVE_IN_PROGRESS  0x80338127L

//
// MessageId: ERROR_WINRS_RECEIVE_NO_RESPONSE_DATA
//
// MessageText:
//
// The WinRS client cannot process the Receive request because the shell plugin returned an empty response to the request.
//
#define ERROR_WINRS_RECEIVE_NO_RESPONSE_DATA 0x80338128L

//
// MessageId: ERROR_WINRS_CLIENT_CREATESHELL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManCreateShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_CREATESHELL_NULL_PARAM 0x80338129L

//
// MessageId: ERROR_WINRS_CLIENT_CLOSESHELL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WinrsCloseShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_CLOSESHELL_NULL_PARAM 0x8033812AL

//
// MessageId: ERROR_WINRS_CLIENT_FREECREATESHELLRESULT_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. The parameter required for the WinrsFreeCreateShellResult function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_FREECREATESHELLRESULT_NULL_PARAM 0x8033812BL

//
// MessageId: ERROR_WINRS_CLIENT_RUNCOMMAND_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManRunShellCommand function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_RUNCOMMAND_NULL_PARAM 0x8033812CL

//
// MessageId: ERROR_WINRS_CLIENT_FREERUNCOMMANDRESULT_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. The parameter required for the WinrsFreeRunCommandResult function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_FREERUNCOMMANDRESULT_NULL_PARAM 0x8033812DL

//
// MessageId: ERROR_WINRS_CLIENT_SIGNAL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManSignalShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_SIGNAL_NULL_PARAM 0x8033812EL

//
// MessageId: ERROR_WINRS_CLIENT_RECEIVE_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSMansReceiveShellOutput function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_RECEIVE_NULL_PARAM 0x8033812FL

//
// MessageId: ERROR_WINRS_CLIENT_FREEPULLRESULT_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. The parameter required for the WinrsFreePullResult function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_FREEPULLRESULT_NULL_PARAM 0x80338130L

//
// MessageId: ERROR_WINRS_CLIENT_PULL_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. One of the parameters required for the WinrsPull function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_PULL_NULL_PARAM 0x80338131L

//
// MessageId: ERROR_WINRS_CLIENT_CLOSERECEIVEHANDLE_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. The parameter required for the WinrsCloseReceiveHandle function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_CLOSERECEIVEHANDLE_NULL_PARAM 0x80338132L

//
// MessageId: ERROR_WINRS_CLIENT_SEND_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManSendShellInput function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_SEND_NULL_PARAM 0x80338133L

//
// MessageId: ERROR_WINRS_CLIENT_PUSH_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. One of the parameters required for the WinrsPush function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_PUSH_NULL_PARAM 0x80338134L

//
// MessageId: ERROR_WINRS_CLIENT_CLOSESENDHANDLE_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. The parameter required for the WinrsCloseSendHandle function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_CLOSESENDHANDLE_NULL_PARAM 0x80338135L

//
// MessageId: ERROR_WINRS_CLIENT_GET_NULL_PARAM
//
// MessageText:
//
// The WinRS client cannot process the request. One of the parameters required for the WinrsGet function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WINRS_CLIENT_GET_NULL_PARAM 0x80338136L

//
// MessageId: ERROR_WSMAN_POLYMORPHISM_MODE_UNSUPPORTED
//
// MessageText:
//
// The WS-Management service does not support the specified polymorphism mode. Try changing the polymorphism mode specified, and try again.
//
#define ERROR_WSMAN_POLYMORPHISM_MODE_UNSUPPORTED 0x80338137L

//
// MessageId: ERROR_WSMAN_REQUEST_NOT_SUPPORTED_AT_SERVICE
//
// MessageText:
//
// The WS-Management service cannot process the request because the specified URI is not supported on the service side. Retry the request with local session.
//
#define ERROR_WSMAN_REQUEST_NOT_SUPPORTED_AT_SERVICE 0x80338138L

//
// MessageId: ERROR_WSMAN_URI_NON_DMTF_CLASS
//
// MessageText:
//
// The WS-Management service cannot process the request. A DMTF resource URI was used to access a non-DMTF class. Try again using a non-DMTF resource URI.
//
#define ERROR_WSMAN_URI_NON_DMTF_CLASS   0x80338139L

//
// MessageId: ERROR_WSMAN_URI_WRONG_DMTF_VERSION
//
// MessageText:
//
// The WS-Management service cannot process the request. The DMTF class in the repository uses a different major version number from the requested class. This class can be accessed using a non-DMTF resource URI.
//
#define ERROR_WSMAN_URI_WRONG_DMTF_VERSION 0x8033813AL

//
// MessageId: ERROR_WSMAN_DIFFERENT_CIM_SELECTOR
//
// MessageText:
//
// The WS-Management service cannot process the request. The resource URI and __cimnamespace selector attempted to use different namespaces. Try removing the __cimnamespace selector or using a DMTF resource URI. If a non-DMTF resource URI is used with a __cimnamespace selector, the namespaces must match.
//
#define ERROR_WSMAN_DIFFERENT_CIM_SELECTOR 0x8033813BL

//
// MessageId: ERROR_WSMAN_PUSHSUBSCRIPTION_INVALIDUSERACCOUNT
//
// MessageText:
//
// The WS-Management client cannot process the request. To use the WSManSubscribe API the user has to be running under Network Service account. No other account is supported currently for push subscriptions.
//
#define ERROR_WSMAN_PUSHSUBSCRIPTION_INVALIDUSERACCOUNT 0x8033813CL

//
// MessageId: ERROR_WSMAN_EVENTING_NONDOMAINJOINED_PUBLISHER
//
// MessageText:
//
// The WS-Management client cannot process the request. The event source machine is not joined to a domain.
// To set up a push subscription session to an event source the source has to be connected to a domain.
// To fix this problem either join the event source machine to a domain or use PULL as the delivery mode for the subscription.
//
#define ERROR_WSMAN_EVENTING_NONDOMAINJOINED_PUBLISHER 0x8033813DL

//
// MessageId: ERROR_WSMAN_EVENTING_NONDOMAINJOINED_COLLECTOR
//
// MessageText:
//
// The WS-Management client cannot process the request. The subscriber machine is not joined to a domain.
// To set up a push subscription session to an event source, the subscriber machine has to be connected to a domain.
// To fix this problem either join the subscriber machine to a domain or use PULL as the delivery mode for the subscription.
//
#define ERROR_WSMAN_EVENTING_NONDOMAINJOINED_COLLECTOR 0x8033813EL

//
// MessageId: ERROR_WSMAN_CONFIG_READONLY_PROPERTY
//
// MessageText:
//
// The WinRM client cannot process the request because it is trying to update a read-only setting.
// Remove this setting from the command and try again.
//
#define ERROR_WSMAN_CONFIG_READONLY_PROPERTY 0x8033813FL

//
// MessageId: ERROR_WINRS_CODE_PAGE_NOT_SUPPORTED
//
// MessageText:
//
// The WinRS client cannot process the request. The server cannot set Code Page.
// You may want to use the CHCP command to change the client Code Page to 437 and receive the results in English.
//
#define ERROR_WINRS_CODE_PAGE_NOT_SUPPORTED 0x80338140L

//
// MessageId: ERROR_WSMAN_CLIENT_DISABLE_LOOPBACK_WITH_EXPLICIT_CREDENTIALS
//
// MessageText:
//
// Not used. To be removed.
//
#define ERROR_WSMAN_CLIENT_DISABLE_LOOPBACK_WITH_EXPLICIT_CREDENTIALS 0x80338141L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_DISABLE_LOOPBACK
//
// MessageText:
//
// Not used. To be removed.
//
#define ERROR_WSMAN_CLIENT_INVALID_DISABLE_LOOPBACK 0x80338142L

//
// MessageId: ERROR_WSMAN_CLIENT_ENUM_RECEIVED_TOO_MANY_ITEMS
//
// MessageText:
//
// The WS-Management client received too many results from the server.
// The server implementation should never return more items than are specified by the client.
//
#define ERROR_WSMAN_CLIENT_ENUM_RECEIVED_TOO_MANY_ITEMS 0x80338143L

//
// MessageId: ERROR_WSMAN_MULTIPLE_CREDENTIALS
//
// MessageText:
//
// The WinRM client cannot process the request. A certificate thumbprint was specified together with a user name or password. 
// Only one credentials type can be specified. Remove the credentials type that does not correspond to the intended authentication mechanism and retry the request.
//
#define ERROR_WSMAN_MULTIPLE_CREDENTIALS 0x80338144L

//
// MessageId: ERROR_WSMAN_AUTHENTICATION_INVALID_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. The flag that specifies the authentication mechanism to use is incorrect.
// Remove or change the invalid flag and try the request again.
//
#define ERROR_WSMAN_AUTHENTICATION_INVALID_FLAG 0x80338145L

//
// MessageId: ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_DEFAULT_AUTHENTICATION
//
// MessageText:
//
// The WinRM client cannot process the request. When an authentication mechanism is not specified, only user name and password credentials are allowed.
// If you want to use a different type of credentials then you need to specify the authentication mechanism.
// Specify the authentication mechanism or the correct credentials and try the request again.
//
#define ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_DEFAULT_AUTHENTICATION 0x80338146L

//
// MessageId: ERROR_WSMAN_CLIENT_USERNAME_AND_PASSWORD_NEEDED
//
// MessageText:
//
// The WinRM client cannot process the request. For authentication mechanisms that require the credentials of an user account, both user name and password must be specified.
// Specify the missing user name or password and try the request again.
//
#define ERROR_WSMAN_CLIENT_USERNAME_AND_PASSWORD_NEEDED 0x80338147L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_CERT_DNS_OR_UPN
//
// MessageText:
//
// The WinRM client cannot process the request. If you are using a machine certificate, it must contain a DNS name in the Subject Alternative Name extension or in the Subject Name field, and no UPN name.
// If you are using a user certificate, the Subject Alternative Name extension must contain a UPN name and must not contain a DNS name.
// Change the certificate structure and try the request again.
//
#define ERROR_WSMAN_CLIENT_INVALID_CERT_DNS_OR_UPN 0x80338148L

//
// MessageId: ERROR_WSMAN_CREATESHELL_NULL_ENVIRONMENT_VARIABLE_NAME
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the environment variable name passed to the WSManCreateShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CREATESHELL_NULL_ENVIRONMENT_VARIABLE_NAME 0x80338149L

//
// MessageId: ERROR_WSMAN_SHELL_ALREADY_CLOSED
//
// MessageText:
//
// An operation is being attempted on a shell that is being closed. This can happen if the shell that is being used is also being closed by another thread.
//
#define ERROR_WSMAN_SHELL_ALREADY_CLOSED 0x8033814AL

//
// MessageId: ERROR_WSMAN_CREATESHELL_NULL_STREAMID
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the stream id name passed to the WSManCreateShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CREATESHELL_NULL_STREAMID 0x8033814BL

//
// MessageId: ERROR_WSMAN_SHELL_INVALID_SHELL_HANDLE
//
// MessageText:
//
// The WinRM Shell client cannot process the request. The shell handle passed to the WSMan Shell function is not valid.
// The shell handle is valid only when WSManCreateShell function completes successfully. Change the request including a valid shell handle and try again.
//
#define ERROR_WSMAN_SHELL_INVALID_SHELL_HANDLE 0x8033814CL

//
// MessageId: ERROR_WSMAN_SHELL_INVALID_COMMAND_HANDLE
//
// MessageText:
//
// The WinRM Shell client cannot process the request. The command handle passed to the WSMan Shell function is not valid.
// The command handle is valid only when WSManRunShellCommand function completes successfully. Change the request including a valid shell handle and try again.
//
#define ERROR_WSMAN_SHELL_INVALID_COMMAND_HANDLE 0x8033814DL

//
// MessageId: ERROR_WSMAN_RUNSHELLCOMMAND_NULL_ARGUMENT
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the argument value passed to the WSManRunShellCommand function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_RUNSHELLCOMMAND_NULL_ARGUMENT 0x8033814EL

//
// MessageId: ERROR_WSMAN_COMMAND_ALREADY_CLOSED
//
// MessageText:
//
// An operation is being attempted on a command that is being closed. This can happen if the command handle that is being used is also being freed
// by another thread.
//
#define ERROR_WSMAN_COMMAND_ALREADY_CLOSED 0x8033814FL

//
// MessageId: ERROR_WSMAN_SENDSHELLINPUT_INVALID_STREAMID_INDEX
//
// MessageText:
//
// The WinRM Shell client cannot process the request. The stream id index from within WSMAN_STREAM_ELEMENT passed to the WSManSendShellInput function is invalid.
// The stream id index should be an index from within inputStreamSet array passed to the WSManCreateShell function.
// Change the request with a valid index and try again.
//
#define ERROR_WSMAN_SENDSHELLINPUT_INVALID_STREAMID_INDEX 0x80338150L

//
// MessageId: ERROR_WSMAN_SHELL_SYNCHRONOUS_NOT_SUPPORTED
//
// MessageText:
//
// Not used. To be removed.
//
#define ERROR_WSMAN_SHELL_SYNCHRONOUS_NOT_SUPPORTED 0x80338151L

//
// MessageId: ERROR_WSMAN_NO_CERTMAPPING_OPERATION_FOR_LOCAL_SESSION
//
// MessageText:
//
// The WS-Management operations to update the certificate mapping store of the WINRM service config can only be done remotely.
//
#define ERROR_WSMAN_NO_CERTMAPPING_OPERATION_FOR_LOCAL_SESSION 0x80338152L

//
// MessageId: ERROR_WSMAN_CERTMAPPING_CONFIGLIMIT_EXCEEDED
//
// MessageText:
//
// The WINRM certificate mapping configuration store has reached an internal limit and cannot create any more entries. Remove some entries and try again.
//
#define ERROR_WSMAN_CERTMAPPING_CONFIGLIMIT_EXCEEDED 0x80338153L

//
// MessageId: ERROR_WSMAN_CERTMAPPING_INVALIDUSERCREDENTIALS
//
// MessageText:
//
// The WINRM certificate mapping configuration operation cannot be completed because the user credentials could not be verified. 
// Please check the username and password used for mapping this certificate and verify that it is a non-domain account and try again.
// 
//
#define ERROR_WSMAN_CERTMAPPING_INVALIDUSERCREDENTIALS 0x80338154L

//
// MessageId: ERROR_WSMAN_CERT_INVALID_USAGE_CLIENT
//
// MessageText:
//
// The WinRM client cannot process the request. The Enhanced Key Usage (EKU) field of the certificate is not set to "Client Authentication".
// Retry the request with a certificate that has the correct EKU.
//
#define ERROR_WSMAN_CERT_INVALID_USAGE_CLIENT 0x80338155L

//
// MessageId: ERROR_WSMAN_CERT_MISSING_AUTH_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. A certificate thumbprint was specified, but the following flag is missing: WSManFlagUseClientCertificate.
// Add the flag and try the request again.
//
#define ERROR_WSMAN_CERT_MISSING_AUTH_FLAG 0x80338156L

//
// MessageId: ERROR_WSMAN_CERT_MULTIPLE_CREDENTIALS_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. The following flags cannot be specified together: WSManFlagUseClientCertificate and WSManFlagCredUsernamePassword.
// Remove one of the flags and try the request again.
//
#define ERROR_WSMAN_CERT_MULTIPLE_CREDENTIALS_FLAG 0x80338157L

//
// MessageId: ERROR_WSMAN_CONFIG_SHELL_URI_INVALID
//
// MessageText:
//
// The WinRM client cannot process the request because the CustomRemoteShell URI specified is invalid.
// CustomRemoteShell URI should start with WinRM shell resource URI prefix: "http://schemas.microsoft.com/wbem/wsman/1/windows/shell".
// The URI should not contain invalid characters including '*', '?', white spaces and tabs.
// The CustomRemoteShell URI cannot be longer than 1023 characters.
//
#define ERROR_WSMAN_CONFIG_SHELL_URI_INVALID 0x80338158L

//
// MessageId: ERROR_WSMAN_CONFIG_SHELL_URI_CMDSHELLURI_NOTPERMITTED
//
// MessageText:
//
// The WinRM client cannot process the request because the CustomRemoteShell URI specified is invalid.
// Windows command shell URI ("http://schemas.microsoft.com/wbem/wsman/1/windows/shell/cmd") cannot be a CustomRemoteShell URI.
//
#define ERROR_WSMAN_CONFIG_SHELL_URI_CMDSHELLURI_NOTPERMITTED 0x80338159L

//
// MessageId: ERROR_WSMAN_CONFIG_SHELLURI_INVALID_PROCESSPATH
//
// MessageText:
//
// The WinRM client cannot process the request because the process path specified for the CustomRemoteShell table entry is invalid.
// The process path should be absolute and should point to an existing executable.
//
#define ERROR_WSMAN_CONFIG_SHELLURI_INVALID_PROCESSPATH 0x8033815AL

//
// MessageId: ERROR_WINRS_SHELL_URI_INVALID
//
// MessageText:
//
// Not used. To be removed.
//
#define ERROR_WINRS_SHELL_URI_INVALID    0x8033815BL

//
// MessageId: ERROR_WSMAN_INVALID_SECURITY_DESCRIPTOR
//
// MessageText:
//
// The WinRM client cannot process the request because the provided security descriptor is invalid.
//
#define ERROR_WSMAN_INVALID_SECURITY_DESCRIPTOR 0x8033815CL

//
// MessageId: ERROR_WSMAN_POLICY_TOO_COMPLEX
//
// MessageText:
//
// The WinRM service cannot process the request because the WS-Policy contained in the DeliverTo is too complex or uses a structure not understood by the service.
// The WinRM service supports a single layer of policy assertions underneath a wsp:ExactlyOne element.
//
#define ERROR_WSMAN_POLICY_TOO_COMPLEX   0x8033815DL

//
// MessageId: ERROR_WSMAN_POLICY_CANNOT_COMPLY
//
// MessageText:
//
// The WinRM service cannot process the request because the WS-Policy contained in the DeliverTo does not contain any options that the service can comply with.
// The WinRM service supports the following profiles: Negotiate or Kerberos over HTTP, Negotiate or Kerberos over HTTPS, and mutual certificate authentication over HTTPS using issuer thumbprints.
//
#define ERROR_WSMAN_POLICY_CANNOT_COMPLY 0x8033815EL

//
// MessageId: ERROR_WSMAN_INVALID_CONNECTIONRETRY
//
// MessageText:
//
// The WinRM service cannot process the request because the wsman:ConnectionRetry element in the DeliverTo is invalid.
//
#define ERROR_WSMAN_INVALID_CONNECTIONRETRY 0x8033815FL

//
// MessageId: ERROR_WSMAN_URISECURITY_INVALIDURIKEY
//
// MessageText:
//
// WinRM cannot make the configuration change. 
// The URI supplied for the certificate mapping operation is not valid. 
// It must contain at least one character.
// It must not contain internal whitespace.
// It must not contain '?' character.
// A prefix may be specified by using "*" as the last character.
// The URI cannot be longer than 1023 characters.
//
#define ERROR_WSMAN_URISECURITY_INVALIDURIKEY 0x80338160L

//
// MessageId: ERROR_WSMAN_CERTMAPPING_INVALIDSUBJECTKEY
//
// MessageText:
//
// WinRM cannot make the configuration change.
// The Subject used for the certificate mapping operation is not valid. 
// It must contain at least one character. 
// It must contain at most one "*" character which should be the first character.
// (This may be the only character in which case it matches all subjects). 
// The Subject cannot be longer than 1023 characters.
//
#define ERROR_WSMAN_CERTMAPPING_INVALIDSUBJECTKEY 0x80338161L

//
// MessageId: ERROR_WSMAN_CERTMAPPING_INVALIDISSUERKEY
//
// MessageText:
//
// WinRM cannot make the configuration change because the Issuer used for the certificate mapping operation is not valid. 
// The certificate identified by the issuer thumbprint must be present in the machine "Trusted Root Certification Authorities" or "Intermediate Certification Authorities" store. 
// The certificate must have key usage that allows it to sign other certificates.
//
#define ERROR_WSMAN_CERTMAPPING_INVALIDISSUERKEY 0x80338162L

//
// MessageId: ERROR_WSMAN_INVALID_PUBLISHERS_TYPE
//
// MessageText:
//
// The WinRM client cannot process the request because the type field in the WSMAN_ALLOWED_PUBLISHERS argument is invalid.
// Collector-initiated subscriptions must use WSMAN_SINGLE_PUBLISHER and Source-initiated subscriptions must use WSMAN_MULTIPLE_PUBLISHERS.
//
#define ERROR_WSMAN_INVALID_PUBLISHERS_TYPE 0x80338163L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_DELIVERY_RETRY
//
// MessageText:
//
// The WinRM client cannot process the request because the delivery retry parameters are invalid.
// If delivery retry is requested, the deliveryRetryInterval and deliveryRetryAttempts fields must both be nonzero.
//
#define ERROR_WSMAN_CLIENT_INVALID_DELIVERY_RETRY 0x80338164L

//
// MessageId: ERROR_WSMAN_CLIENT_NULL_PUBLISHERS
//
// MessageText:
//
// The WinRM client cannot process the request.
// The required WSMAN_ALLOWED_PUBLISHERS settings is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_NULL_PUBLISHERS 0x80338165L

//
// MessageId: ERROR_WSMAN_CLIENT_NULL_ISSUERS
//
// MessageText:
//
// The WinRM client cannot process the request because client certificate subject filters were specified without any issuers.
// When using client certificate authentication, specify at least one issuer thumbprint.
//
#define ERROR_WSMAN_CLIENT_NULL_ISSUERS  0x80338166L

//
// MessageId: ERROR_WSMAN_CLIENT_NO_SOURCES
//
// MessageText:
//
// The WinRM client cannot process the request because the subscription contains no domain or non-domain sources.
// Subscriptions using WSMAN_MULTIPLE_PUBLISHERS must specify either a security descriptor or an issuer list or both.
//
#define ERROR_WSMAN_CLIENT_NO_SOURCES    0x80338167L

//
// MessageId: ERROR_WSMAN_INVALID_SUBSCRIBE_OBJECT
//
// MessageText:
//
// The WinRM service cannot process the request because the subscription manager returned invalid enumeration results.
// The m:Subscription XML object or m:Version element is missing or invalid.
//
#define ERROR_WSMAN_INVALID_SUBSCRIBE_OBJECT 0x80338168L

//
// MessageId: ERROR_WSMAN_PUBLIC_FIREWALL_PROFILE_ACTIVE
//
// MessageText:
//
// WinRM firewall exception will not work since one of the network connection types on this machine is set to Public.
// Change the network connection type to either Domain or Private and try again.
//
#define ERROR_WSMAN_PUBLIC_FIREWALL_PROFILE_ACTIVE 0x80338169L

//
// MessageId: ERROR_WSMAN_CERTMAPPING_PASSWORDTOOLONG
//
// MessageText:
//
// WinRM cannot make the configuration change.
// The Password used for updating the certificate mapping configuration is not valid. 
// It cannot be longer than 255 characters.
//
#define ERROR_WSMAN_CERTMAPPING_PASSWORDTOOLONG 0x8033816AL

//
// MessageId: ERROR_WSMAN_CERTMAPPING_PASSWORDBLANK
//
// MessageText:
//
// WinRM cannot make the configuration change.
// The Password used for updating the certificate mapping configuration is not valid. 
// A user account used for configuring a certificate mapping cannot have a blank password.
//
#define ERROR_WSMAN_CERTMAPPING_PASSWORDBLANK 0x8033816BL

//
// MessageId: ERROR_WSMAN_CERTMAPPING_PASSWORDUSERTUPLE
//
// MessageText:
//
// WinRM cannot make the configuration change.
// The credential used for updating or creating the certificate mapping configuration is not valid. 
// The credential consists of both Password and UserName being supplied together in a pair.
//
#define ERROR_WSMAN_CERTMAPPING_PASSWORDUSERTUPLE 0x8033816CL

//
// MessageId: ERROR_WSMAN_INVALID_PROVIDER_RESPONSE
//
// MessageText:
//
// The WinRM service executed an operation and the provider returned inconclusive information regarding success or failure of the operation.
// The status was marked as failed, but no error code was given.
//
#define ERROR_WSMAN_INVALID_PROVIDER_RESPONSE 0x8033816DL

//
// MessageId: ERROR_WSMAN_SHELL_NOT_INITIALIZED
//
// MessageText:
//
// The WS-Management service on the remote machine cannot process the shell request.
// This can happen if the WS-Management service on the remote machine was being shutdown.
// To correct this problem restart the WS-Management service on the remote machine and re-send the shell request.
//
#define ERROR_WSMAN_SHELL_NOT_INITIALIZED 0x8033816EL

//
// MessageId: ERROR_WSMAN_CONFIG_SHELLURI_INVALID_OPERATION_ON_KEY
//
// MessageText:
//
// The WinRM service cannot process the request. 
// The URI parameter is the key to CustomRemoteShell table and cannot be modified.
//
#define ERROR_WSMAN_CONFIG_SHELLURI_INVALID_OPERATION_ON_KEY 0x8033816FL

//
// MessageId: ERROR_WSMAN_HTTP_STATUS_SERVER_ERROR
//
// MessageText:
//
// The WinRM client received an HTTP server error status (500), but the remote service did not include any other information about the cause of the failure.
//
#define ERROR_WSMAN_HTTP_STATUS_SERVER_ERROR 0x80338170L

//
// MessageId: ERROR_WSMAN_HTTP_STATUS_BAD_REQUEST
//
// MessageText:
//
// The WinRM client received an HTTP bad request status (400), but the remote service did not include any other information about the cause of the failure.
//
#define ERROR_WSMAN_HTTP_STATUS_BAD_REQUEST 0x80338171L

//
// MessageId: ERROR_WSMAN_CONFIG_CANNOT_CHANGE_CERTMAPPING_KEYS
//
// MessageText:
//
// The WinRM service cannot make the configuration change.
// The selector keys of Subject, URI or Issuer cannot be changed by overriding the selector key value in the body. 
//
#define ERROR_WSMAN_CONFIG_CANNOT_CHANGE_CERTMAPPING_KEYS 0x80338172L

//
// MessageId: ERROR_WSMAN_HTML_ERROR
//
// MessageText:
//
// The WinRM client cannot process the request because it received an HTML error packet.
//
#define ERROR_WSMAN_HTML_ERROR           0x80338173L

//
// MessageId: ERROR_WSMAN_CLIENT_INITIALIZE_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManInitialize function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_INITIALIZE_NULL_PARAM 0x80338174L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_INIT_APPLICATION_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManInitialize API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_INIT_APPLICATION_FLAG 0x80338175L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_DEINIT_APPLICATION_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManDeinitialize API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_DEINIT_APPLICATION_FLAG 0x80338176L

//
// MessageId: ERROR_WSMAN_CLIENT_SETSESSIONOPTION_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManSetSessionOption function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_SETSESSIONOPTION_NULL_PARAM 0x80338177L

//
// MessageId: ERROR_WSMAN_CLIENT_SETSESSIONOPTION_INVALID_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManSetSessionOption function is invalid.
// Change the invalid parameter and try again.
//
#define ERROR_WSMAN_CLIENT_SETSESSIONOPTION_INVALID_PARAM 0x80338178L

//
// MessageId: ERROR_WSMAN_CLIENT_GETSESSIONOPTION_INVALID_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required to get a session option is invalid.
// Change the invalid parameter and try again.
//
#define ERROR_WSMAN_CLIENT_GETSESSIONOPTION_INVALID_PARAM 0x80338179L

//
// MessageId: ERROR_WSMAN_CLIENT_CREATESHELL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManCreateShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_CREATESHELL_NULL_PARAM 0x8033817AL

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_CREATE_SHELL_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManCreateShell API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_CREATE_SHELL_FLAG 0x8033817BL

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_CLOSE_SHELL_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManCloseShell API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_CLOSE_SHELL_FLAG 0x8033817CL

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_CLOSE_COMMAND_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManCloseCommand API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_CLOSE_COMMAND_FLAG 0x8033817DL

//
// MessageId: ERROR_WSMAN_CLIENT_CLOSESHELL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManCloseShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_CLOSESHELL_NULL_PARAM 0x8033817EL

//
// MessageId: ERROR_WSMAN_CLIENT_CLOSECOMMAND_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManCloseCommand function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_CLOSECOMMAND_NULL_PARAM 0x8033817FL

//
// MessageId: ERROR_WSMAN_CLIENT_RUNCOMMAND_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManRunShellCommand function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_RUNCOMMAND_NULL_PARAM 0x80338180L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_RUNCOMMAND_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManRunShellCommand API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_RUNCOMMAND_FLAG 0x80338181L

//
// MessageId: ERROR_WSMAN_CLIENT_RUNCOMMAND_NOTCOMPLETED
//
// MessageText:
//
// The WinRM client cannot process the request. You must wait for the WSManRunShellCommand API call to complete before calling WSManCloseShellOperationEx API.
//
#define ERROR_WSMAN_CLIENT_RUNCOMMAND_NOTCOMPLETED 0x80338182L

//
// MessageId: ERROR_WSMAN_NO_COMMAND_RESPONSE
//
// MessageText:
//
// The WinRM client cannot process the request. The response to a Command request did not contain a valid CommandResponse element.
// The CommandResponse element was not found or did not contain valid content.
//
#define ERROR_WSMAN_NO_COMMAND_RESPONSE  0x80338183L

//
// MessageId: ERROR_WSMAN_INVALID_OPTIONSET
//
// MessageText:
//
// The WinRM client cannot process the request. The OptionSet element is invalid.
// Change the request to include a valid OptionSet element and try again.
//
#define ERROR_WSMAN_INVALID_OPTIONSET    0x80338184L

//
// MessageId: ERROR_WSMAN_NO_COMMANDID
//
// MessageText:
//
// The WinRM client cannot process the request. The response to a Command request did not contain a valid CommandResponse element.
// The CommandId element was not found or did not contain valid content.
//
#define ERROR_WSMAN_NO_COMMANDID         0x80338185L

//
// MessageId: ERROR_WSMAN_CLIENT_SIGNAL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManSignalShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_SIGNAL_NULL_PARAM 0x80338186L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_SIGNAL_SHELL_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManSignalShell API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_SIGNAL_SHELL_FLAG 0x80338187L

//
// MessageId: ERROR_WSMAN_CLIENT_SEND_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManSendShellInput function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_SEND_NULL_PARAM 0x80338188L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManSendShellInput API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_FLAG 0x80338189L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_PARAMETER
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid parameter was specified for the WSManSendShellInput API call. 
// streamData parameter should be specified in binary format using WSMAN_DATA_TYPE_BINARY type. 
// Change the invalid parameter and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_PARAMETER 0x8033818AL

//
// MessageId: ERROR_WSMAN_SHELL_INVALID_INPUT_STREAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. The stream name passed to the WSManSendShellInput function is not valid.
// The input stream name should be specified as part of the input streams during shell creation using WSManCreateShell function. 
// Change the request including a valid input stream name and try again.
//
#define ERROR_WSMAN_SHELL_INVALID_INPUT_STREAM 0x8033818BL

//
// MessageId: ERROR_WSMAN_CLIENT_RECEIVE_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManReceiveShellOutput function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_RECEIVE_NULL_PARAM 0x8033818CL

//
// MessageId: ERROR_WSMAN_SHELL_INVALID_DESIRED_STREAMS
//
// MessageText:
//
// The WinRM Shell client cannot process the request. The stream or list of streams passed to the WSManReceiveShellOutput function is not valid.
// The desired stream names should be specified as part of the output streams during shell creation using WSManCreateShell function. 
// Change the request including valid desired streams and try again.
//
#define ERROR_WSMAN_SHELL_INVALID_DESIRED_STREAMS 0x8033818DL

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_RECEIVE_SHELL_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManReceiveShellOutput API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_RECEIVE_SHELL_FLAG 0x8033818EL

//
// MessageId: ERROR_WSMAN_NO_RECEIVE_RESPONSE
//
// MessageText:
//
// The WinRM client cannot process the request. The response to a Receive request did not contain a valid ReceiveResponse element.
// The ReceiveResponse element was not found or did not contain valid content.
//
#define ERROR_WSMAN_NO_RECEIVE_RESPONSE  0x8033818FL

//
// MessageId: ERROR_WSMAN_PLUGIN_CONFIGURATION_CORRUPTED
//
// MessageText:
//
// The WSMan plugin configuration is corrupted.
//
#define ERROR_WSMAN_PLUGIN_CONFIGURATION_CORRUPTED 0x80338190L

//
// MessageId: ERROR_WSMAN_INVALID_FILEPATH
//
// MessageText:
//
// The file path specified is either not absolute, not in the system32 directory, or not valid.
//
#define ERROR_WSMAN_INVALID_FILEPATH     0x80338191L

//
// MessageId: ERROR_WSMAN_FILE_NOT_PRESENT
//
// MessageText:
//
// The file specified does not exist.
//
#define ERROR_WSMAN_FILE_NOT_PRESENT     0x80338192L

//
// MessageId: ERROR_WSMAN_IISCONFIGURATION_READ_FAILED
//
// MessageText:
//
// The WSMan extension failed to read IIS configuration.
//
#define ERROR_WSMAN_IISCONFIGURATION_READ_FAILED 0x80338193L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_LOCALE
//
// MessageText:
//
// The WinRM client cannot process the request. The locale option is invalid.
// Change the locale and try again.
//
#define ERROR_WSMAN_CLIENT_INVALID_LOCALE 0x80338194L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_UI_LANGUAGE
//
// MessageText:
//
// The WinRM client cannot process the request. The UI language option is invalid.
// Change the UI language and try again.
//
#define ERROR_WSMAN_CLIENT_INVALID_UI_LANGUAGE 0x80338195L

//
// MessageId: ERROR_WSMAN_CLIENT_GETERRORMESSAGE_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManGetErrorMessage function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_GETERRORMESSAGE_NULL_PARAM 0x80338196L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_LANGUAGE_CODE
//
// MessageText:
//
// The WinRM client cannot process the request. The language code parameter is invalid. The language code parameter should be either NULL or a valid RFC 3066 language code.
// Change the language code and try the request again.
//
#define ERROR_WSMAN_CLIENT_INVALID_LANGUAGE_CODE 0x80338197L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_GETERRORMESSAGE_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. An invalid flag was specified for the WSManGetErrorMessage API call.
// Remove or change the invalid flag and try the call again.
//
#define ERROR_WSMAN_CLIENT_INVALID_GETERRORMESSAGE_FLAG 0x80338198L

//
// MessageId: ERROR_WSMAN_REDIRECT_REQUESTED
//
// MessageText:
//
// The WinRM service cannot process the request because the request needs to be sent to a different machine.
// Use the redirect information to send the request to a new machine.
//
#define ERROR_WSMAN_REDIRECT_REQUESTED   0x80338199L

//
// MessageId: ERROR_WSMAN_PROXY_AUTHENTICATION_INVALID_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. The flag that specifies the proxy authentication mechanism to use is incorrect. 
// Remove or change the invalid flag and try the request again.
//
#define ERROR_WSMAN_PROXY_AUTHENTICATION_INVALID_FLAG 0x8033819AL

//
// MessageId: ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_PROXY_AUTHENTICATION
//
// MessageText:
//
// The WinRM client cannot process the request. The credentials for proxy authentication are not specified correctly. Both user name and password credentials must be valid.
// Specify the correct credentials and try the request again.
//
#define ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_PROXY_AUTHENTICATION 0x8033819BL

//
// MessageId: ERROR_WSMAN_PROXY_ACCESS_TYPE
//
// MessageText:
//
// The WinRM client cannot process the request. The proxy access type is incorrect. Use one of the proxy access type flags; the flags cannot be combined.
// Change the invalid proxy access type and try the request again.
//
#define ERROR_WSMAN_PROXY_ACCESS_TYPE    0x8033819CL

//
// MessageId: ERROR_WSMAN_INVALID_OPTION_NO_PROXY_SERVER
//
// MessageText:
//
// The WinRM client cannot process the request. The direct connection to the server option cannot be used with non empty proxy authentication data. 
// Change the invalid proxy access type or use empty proxy authentication data and try the request again.
//
#define ERROR_WSMAN_INVALID_OPTION_NO_PROXY_SERVER 0x8033819DL

//
// MessageId: ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_NULL_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManGetSessionOptionAsDword function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_NULL_PARAM 0x8033819EL

//
// MessageId: ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_INVALID_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManGetSessionOptionAsDword function is invalid.
// Change the invalid parameter and try again.
//
#define ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_INVALID_PARAM 0x8033819FL

//
// MessageId: ERROR_WSMAN_CLIENT_GETSESSIONOPTION_STRING_INVALID_PARAM
//
// MessageText:
//
// The WinRM client cannot process the request. One of the parameters required for the WSManGetSessionOptionAsString function is invalid.
// Change the invalid parameter and try again.
//
#define ERROR_WSMAN_CLIENT_GETSESSIONOPTION_STRING_INVALID_PARAM 0x803381A0L

//
// MessageId: ERROR_WSMAN_CREDSSP_USERNAME_PASSWORD_NEEDED
//
// MessageText:
//
// The WinRM client cannot process the request. Requests must include user name and password when CredSSP authentication mechanism is used.
// Add the user name and password or change the authentication mechanism and try the request again.
//
#define ERROR_WSMAN_CREDSSP_USERNAME_PASSWORD_NEEDED 0x803381A1L

//
// MessageId: ERROR_WSMAN_CLIENT_CREDSSP_AUTHENTICATION_DISABLED
//
// MessageText:
//
// The WinRM client cannot process the request.
// CredSSP authentication is currently disabled in the client configuration.
// Change the client configuration and try the request again.
// CredSSP authentication must also be enabled in the server configuration.
// Also, Group Policy must be edited to allow credential delegation to the target computer.
// Use gpedit.msc and look at the following policy: Computer Configuration -> Administrative Templates -> System -> Credentials Delegation -> Allow Delegating Fresh Credentials. 
// Verify that it is enabled and configured with an SPN appropriate for the target computer.
// For example, for a target computer name "myserver.domain.com", the SPN can be one of the following: WSMAN/myserver.domain.com or WSMAN/*.domain.com
//
#define ERROR_WSMAN_CLIENT_CREDSSP_AUTHENTICATION_DISABLED 0x803381A2L

//
// MessageId: ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS
//
// MessageText:
//
// The WinRM client cannot process the request. A computer policy does not allow the delegation of the user credentials to the target computer.
// Use gpedit.msc and look at the following policy: Computer Configuration -> Administrative Templates -> System -> Credentials Delegation -> Allow Delegating Fresh Credentials. 
// Verify that it is enabled and configured with an SPN appropriate for the target computer.
// For example, for a target computer name "myserver.domain.com", the SPN can be one of the following: WSMAN/myserver.domain.com or WSMAN/*.domain.com.
//
#define ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS 0x803381A3L

//
// MessageId: ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS_NTLMONLY
//
// MessageText:
//
// The WinRM client cannot process the request. A computer policy does not allow the delegation of the user credentials to the target computer because the computer is not trusted.
// The identity of the target computer can be verified if you configure the WSMAN service to use a valid certificate using the following command: winrm set winrm/config/service @{CertificateThumbprint="<thumbprint>"} 
// Or you can check the Event Viewer for an event that specifies that the following SPN could not be created: WSMAN/<computerFQDN>. If you find this event, you can manually create the SPN using setspn.exe . 
// If the SPN exists, but CredSSP cannot use Kerberos to validate the identity of the target computer and you still want to allow the delegation of the user credentials to the target computer, use gpedit.msc and look at the following policy: Computer Configuration -> Administrative Templates -> System -> Credentials Delegation -> Allow Fresh Credentials with NTLM-only Server Authentication. 
// Verify that it is enabled and configured with an SPN appropriate for the target computer.
// For example, for a target computer name "myserver.domain.com", the SPN can be one of the following: WSMAN/myserver.domain.com or WSMAN/*.domain.com.
// Try the request again after these changes. 
//
#define ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS_NTLMONLY 0x803381A4L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_SHELLS
//
// MessageText:
//
// The WS-Management service cannot process the request.
// The maximum number of concurrent shells for this user has been exceeded.
// Close existing shells or raise the quota for this user.
//
#define ERROR_WSMAN_QUOTA_MAX_SHELLS     0x803381A5L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_OPERATIONS
//
// MessageText:
//
// The WS-Management service cannot process the request.
// The maximum number of concurrent operations for this user has been exceeded.
// Close existing operations for this user, or raise the quota for this user.
//
#define ERROR_WSMAN_QUOTA_MAX_OPERATIONS 0x803381A6L

//
// MessageId: ERROR_WSMAN_QUOTA_USER
//
// MessageText:
//
// The WS-Management service cannot process the request.
// The load quota for this user has been exceeded.
// Send future requests at a slower rate or raise the quota for this user.
//
#define ERROR_WSMAN_QUOTA_USER           0x803381A7L

//
// MessageId: ERROR_WSMAN_QUOTA_SYSTEM
//
// MessageText:
//
// The WS-Management service cannot process the request.
// The load quota for the system has been exceeded.
// Send future requests at a slower rate or raise the system quota.
//
#define ERROR_WSMAN_QUOTA_SYSTEM         0x803381A8L

//
// MessageId: ERROR_WSMAN_DIFFERENT_AUTHZ_TOKEN
//
// MessageText:
//
// The WS-Management service cannot complete the authorization under the given token.
// A previous authorization attempt for the same user resulted in a different token.
// The user record will be revoked and the next request will reauthorize.
//
#define ERROR_WSMAN_DIFFERENT_AUTHZ_TOKEN 0x803381A9L

//
// MessageId: ERROR_WSMAN_REDIRECT_LOCATION_NOT_AVAILABLE
//
// MessageText:
//
// An application tried to retrieve the HTTP Redirect location from the session when no redirect error (ERROR_WSMAN_REDIRECT_REQUESTED) was returned.
// The application needs to be updated so as to only retrieve the location after this error is returned.
//
#define ERROR_WSMAN_REDIRECT_LOCATION_NOT_AVAILABLE 0x803381AAL

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_SHELLUSERS
//
// MessageText:
//
// The WS-Management service cannot process the request.
// The maximum number of users executing shell operations has been exceeded.
// Retry after some time or raise the quota for concurrent shell users.
//
#define ERROR_WSMAN_QUOTA_MAX_SHELLUSERS 0x803381ABL

//
// MessageId: ERROR_WSMAN_REMOTESHELLS_NOT_ALLOWED
//
// MessageText:
//
// The WS-Management service cannot process the request.
// The service is configured to not accept any remote shell requests.
//
#define ERROR_WSMAN_REMOTESHELLS_NOT_ALLOWED 0x803381ACL

//
// MessageId: ERROR_WSMAN_PULL_PARAMS_NOT_SAME_AS_ENUM
//
// MessageText:
//
// The WS-Management service cannot complete the Pull operation for the enumeration because the wsman:MaxEnvelopeSize, wsen:MaxCharacters or wsen:MaxElements parameters differ from those specified to the enumeration.
// The application needs to specify the same parameters for Pull as were specified for the enumeration.
//
#define ERROR_WSMAN_PULL_PARAMS_NOT_SAME_AS_ENUM 0x803381ADL

//
// MessageId: ERROR_WSMAN_DEPRECATED_CONFIG_SETTING
//
// MessageText:
//
// The WinRM service cannot process the request because it is trying to update a deprecated setting.
// Remove this setting from the command and try again.
//
#define ERROR_WSMAN_DEPRECATED_CONFIG_SETTING 0x803381AEL

//
// MessageId: ERROR_WSMAN_URI_SECURITY_URI
//
// MessageText:
//
// The WS-Management service cannot process the configuration settings.
// A Security element contains a URI that does not match its parent Resource element.
//
#define ERROR_WSMAN_URI_SECURITY_URI     0x803381AFL

//
// MessageId: ERROR_WSMAN_CANNOT_USE_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS_FOR_HTTP
//
// MessageText:
//
// The WinRM client cannot process the request. Allow implicit credentials for Negotiate authentication option is only valid for HTTPS transport.
// Remove the allow implicit credentials for Negotiate authentication option and try the request again.
//
#define ERROR_WSMAN_CANNOT_USE_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS_FOR_HTTP 0x803381B0L

//
// MessageId: ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_HTTP
//
// MessageText:
//
// The WinRM client cannot process the request. Setting proxy information is not valid when the HTTP transport is specified.
// Remove the proxy information or change the transport and try the request again.
//
#define ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_HTTP 0x803381B1L

//
// MessageId: ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_KERBEROS
//
// MessageText:
//
// The WinRM client cannot process the request. Setting proxy information is not valid when the authentication mechanism with the remote machine is Kerberos.
// Remove the proxy information or change the authentication mechanism and try the request again.
//
#define ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_KERBEROS 0x803381B2L

//
// MessageId: ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_CREDSSP
//
// MessageText:
//
// The WinRM client cannot process the request. Setting proxy information is not valid when the authentication mechanism with the remote machine is CredSSP.
// Remove the proxy information or change the authentication mechanism and try the request again.
//
#define ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_CREDSSP 0x803381B3L

//
// MessageId: ERROR_WSMAN_CLIENT_MULTIPLE_PROXY_AUTH_FLAGS
//
// MessageText:
//
// The WinRM client cannot process the request. The request must specify only one authentication mechanism for proxy.
// Change the request to specify only one authentication mechanism and try again.
//
#define ERROR_WSMAN_CLIENT_MULTIPLE_PROXY_AUTH_FLAGS 0x803381B4L

//
// MessageId: ERROR_WSMAN_INVALID_REDIRECT_ERROR
//
// MessageText:
//
// The WinRM client received a redirect error from the server when it is not appropriate.  
// The only time a redirect error can be reported correctly is during the authorization of a user.
// This would result in a properly formatted redirect response from the server that includes the redirect endpoint.
//
#define ERROR_WSMAN_INVALID_REDIRECT_ERROR 0x803381B5L

//
// MessageId: ERROR_REDIRECT_LOCATION_TOO_LONG
//
// MessageText:
//
// The WinRM service received a redirect error from an authorization plug-in where the redirect location was too long.
//
#define ERROR_REDIRECT_LOCATION_TOO_LONG 0x803381B6L

//
// MessageId: ERROR_REDIRECT_LOCATION_INVALID
//
// MessageText:
//
// The WinRM service received a HTTP redirect message redirecting the client but the location URL is invalid.
//
#define ERROR_REDIRECT_LOCATION_INVALID  0x803381B7L

//
// MessageId: ERROR_SERVICE_CBT_HARDENING_INVALID
//
// MessageText:
//
// The WinRM service cannot process the request. The Channel Binding Token Hardening Level (CbtHardeningLevel) value is invalid.
// The valid values are "None", "Relaxed" and "Strict". Change the CbtHardeningLevel value and try again.
//
#define ERROR_SERVICE_CBT_HARDENING_INVALID 0x803381B8L

//
// MessageId: ERROR_WSMAN_NAME_NOT_RESOLVED
//
// MessageText:
//
// The WinRM client cannot process the request because the server name cannot be resolved.
//
#define ERROR_WSMAN_NAME_NOT_RESOLVED    0x803381B9L

//
// MessageId: ERROR_WSMAN_SSL_CONNECTION_ABORTED
//
// MessageText:
//
// The SSL connection cannot be established.
// Verify that the service on the remote host is properly configured to listen for HTTPS requests.
// Consult the logs and documentation for the WS-Management service running on the destination, most commonly IIS or WinRM.
// If the destination is the WinRM service, run the following command on the destination to analyze and configure the WinRM service: "winrm quickconfig -transport:https".
//
#define ERROR_WSMAN_SSL_CONNECTION_ABORTED 0x803381BAL

//
// MessageId: ERROR_WSMAN_DEFAULTAUTH_IPADDRESS
//
// MessageText:
//
// The WinRM client cannot process the request.
// Default authentication may be used with an IP address under the following conditions: the transport is HTTPS or the destination is in the TrustedHosts list, and explicit credentials are provided.
// Use winrm.cmd to configure TrustedHosts. Note that computers in the TrustedHosts list might not be authenticated.
// For more information on how to set TrustedHosts run the following command: winrm help config.
//
#define ERROR_WSMAN_DEFAULTAUTH_IPADDRESS 0x803381BBL

//
// MessageId: ERROR_WSMAN_CUSTOMREMOTESHELL_DEPRECATED
//
// MessageText:
//
// The WinRM client cannot process the request.
// Custom Remote Shell has been deprecated and cannot be used.
//
#define ERROR_WSMAN_CUSTOMREMOTESHELL_DEPRECATED 0x803381BCL

//
// MessageId: ERROR_WSMAN_FEATURE_DEPRECATED
//
// MessageText:
//
// The WinRM client cannot process the request.
// The feature in use has been deprecated and cannot be used.
//
#define ERROR_WSMAN_FEATURE_DEPRECATED   0x803381BDL

//
// MessageId: ERROR_WSMAN_INVALID_USESSL_PARAM
//
// MessageText:
//
// The WinRM client used a parameter to specify the use of SSL while specifying http in the connection string.
//
#define ERROR_WSMAN_INVALID_USESSL_PARAM 0x803381BEL

//
// MessageId: ERROR_WSMAN_INVALID_CONFIGSDDL_URL
//
// MessageText:
//
// The WinRM service cannot process the request because the security for this resource URI cannot be changed.
//
#define ERROR_WSMAN_INVALID_CONFIGSDDL_URL 0x803381BFL

//
// MessageId: ERROR_WSMAN_ENUMERATE_SHELLCOMAMNDS_FILTER_EXPECTED
//
// MessageText:
//
// The WinRM service cannot process the request. The enumeration request expects a selector based filter to specify the shell identifier.
//
#define ERROR_WSMAN_ENUMERATE_SHELLCOMAMNDS_FILTER_EXPECTED 0x803381C0L

//
// MessageId: ERROR_WSMAN_ENUMERATE_SHELLCOMMANDS_EPRS_NOTSUPPORTED
//
// MessageText:
//
// The WinRM service cannot process the request. The enumeration of end point resources for shell commands is not supported.
//
#define ERROR_WSMAN_ENUMERATE_SHELLCOMMANDS_EPRS_NOTSUPPORTED 0x803381C1L

//
// MessageId: ERROR_WSMAN_CLIENT_CREATESHELL_NAME_INVALID
//
// MessageText:
//
// The WinRM Shell client cannot process the request because the shell name has exceeded 255 characters in length.
//
#define ERROR_WSMAN_CLIENT_CREATESHELL_NAME_INVALID 0x803381C2L

//
// MessageId: ERROR_WSMAN_RUNAS_INVALIDUSERCREDENTIALS
//
// MessageText:
//
// The WinRM runAs configuration operation cannot be completed because the user credentials could not be verified.
// Verify that the username and password used for configuration are valid and retry the operation.
//
#define ERROR_WSMAN_RUNAS_INVALIDUSERCREDENTIALS 0x803381C3L

//
// MessageId: ERROR_WINRS_SHELL_DISCONNECTED
//
// MessageText:
//
// The WinRM service cannot process the request because the WinRS shell instance is currently disconnected.
//
#define ERROR_WINRS_SHELL_DISCONNECTED   0x803381C4L

//
// MessageId: ERROR_WINRS_SHELL_DISCONNECT_NOT_SUPPORTED
//
// MessageText:
//
// The WinRM service cannot process the request. This WinRS shell instance does not support disconnect and reconnect operations
// because it was created by an older WinRS client or its provider does not support the disconnect operation.
//
#define ERROR_WINRS_SHELL_DISCONNECT_NOT_SUPPORTED 0x803381C5L

//
// MessageId: ERROR_WINRS_SHELL_CLIENTSESSIONID_MISMATCH
//
// MessageText:
//
// The WinRM service cannot process the request because the WinRS shell instance is connected to a different client.
//
#define ERROR_WINRS_SHELL_CLIENTSESSIONID_MISMATCH 0x803381C6L

//
// MessageId: ERROR_WSMAN_CLIENT_DISCONNECTSHELL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManDisconnectShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_DISCONNECTSHELL_NULL_PARAM 0x803381C7L

//
// MessageId: ERROR_WSMAN_CLIENT_RECONNECTSHELL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManReconnectShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_RECONNECTSHELL_NULL_PARAM 0x803381C8L

//
// MessageId: ERROR_WSMAN_CLIENT_CONNECTSHELL_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManConnectShell function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_CONNECTSHELL_NULL_PARAM 0x803381C9L

//
// MessageId: ERROR_WSMAN_CLIENT_CONNECTCOMMAND_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManConnectShellCommand function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_CONNECTCOMMAND_NULL_PARAM 0x803381CAL

//
// MessageId: ERROR_WINRS_CONNECT_RESPONSE_BAD_BODY
//
// MessageText:
//
// The WinRM client cannot process the request. The body response is not a valid connect request response.
//
#define ERROR_WINRS_CONNECT_RESPONSE_BAD_BODY 0x803381CBL

//
// MessageId: ERROR_WSMAN_COMMAND_TERMINATED
//
// MessageText:
//
// The WinRM Shell client cannot process the request. The command is currently terminating or was terminated.
//
#define ERROR_WSMAN_COMMAND_TERMINATED   0x803381CCL

//
// MessageId: ERROR_WINRS_SHELL_CONNECTED_TO_DIFFERENT_CLIENT
//
// MessageText:
//
// The WinRM service cannot process the request. The WinRS shell instance is currently connected to a different client.
//
#define ERROR_WINRS_SHELL_CONNECTED_TO_DIFFERENT_CLIENT 0x803381CDL

//
// MessageId: ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_GRACEFUL
//
// MessageText:
//
// The WinRM client encountered an error while communicating with the WinRM service during the disconnect operation.
// The shell has been disconnected and the streams were possibly suspended abruptly.
//
#define ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_GRACEFUL 0x803381CEL

//
// MessageId: ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_VALID
//
// MessageText:
//
// The WinRM client cannot process the request. A disconnect operation cannot be performed on a WinRS shell instance that is already disconnected.
//
#define ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_VALID 0x803381CFL

//
// MessageId: ERROR_WINRS_SHELL_RECONNECT_OPERATION_NOT_VALID
//
// MessageText:
//
// The WinRM client cannot process the request. A reconnect operation cannot be performed on a WinRS shell instance that is currently connected.
//
#define ERROR_WINRS_SHELL_RECONNECT_OPERATION_NOT_VALID 0x803381D0L

//
// MessageId: ERROR_WSMAN_CONFIG_GROUP_POLICY_CHANGE_NOTIFICATION_SUBSCRIPTION_FAILED
//
// MessageText:
//
// An error was encountered while subscribing to the Group Policy change notification.
//
#define ERROR_WSMAN_CONFIG_GROUP_POLICY_CHANGE_NOTIFICATION_SUBSCRIPTION_FAILED 0x803381D1L

//
// MessageId: ERROR_WSMAN_CLIENT_RECONNECTSHELLCOMMAND_NULL_PARAM
//
// MessageText:
//
// The WinRM Shell client cannot process the request. One of the parameters required for the WSManReconnectShellCommand function is null or zero.
// Change the request to include the missing parameter and try again.
//
#define ERROR_WSMAN_CLIENT_RECONNECTSHELLCOMMAND_NULL_PARAM 0x803381D2L

//
// MessageId: ERROR_WINRS_SHELLCOMMAND_RECONNECT_OPERATION_NOT_VALID
//
// MessageText:
//
// The WinRM client cannot process the request. A reconnect operation cannot be performed on a WinRS shell command instance that is currently connected.
//
#define ERROR_WINRS_SHELLCOMMAND_RECONNECT_OPERATION_NOT_VALID 0x803381D3L

//
// MessageId: ERROR_WINRS_SHELLCOMMAND_CLIENTID_NOT_VALID
//
// MessageText:
//
// The WinRM service cannot process the request because the command ID specified by the client is not a valid GUID. Modify the request and retry the request.
//
#define ERROR_WINRS_SHELLCOMMAND_CLIENTID_NOT_VALID 0x803381D4L

//
// MessageId: ERROR_WINRS_SHELL_CLIENTID_NOT_VALID
//
// MessageText:
//
// The WinRM service cannot process the request because the shell ID specified by the client is not a valid GUID. Provide a valid ID and try again.
//
#define ERROR_WINRS_SHELL_CLIENTID_NOT_VALID 0x803381D5L

//
// MessageId: ERROR_WINRS_SHELLCOMMAND_CLIENTID_RESOURCE_CONFLICT
//
// MessageText:
//
// The WinRM service cannot process the request. A command already exists with the command ID specified by the client.
//
#define ERROR_WINRS_SHELLCOMMAND_CLIENTID_RESOURCE_CONFLICT 0x803381D6L

//
// MessageId: ERROR_WINRS_SHELL_CLIENTID_RESOURCE_CONFLICT
//
// MessageText:
//
// The WinRM service cannot process the request. A resource already exists with the shell ID specified by the client.
//
#define ERROR_WINRS_SHELL_CLIENTID_RESOURCE_CONFLICT 0x803381D7L

//
// MessageId: ERROR_WINRS_SHELLCOMMAND_DISCONNECT_OPERATION_NOT_VALID
//
// MessageText:
//
// The WinRM client cannot process the request. A disconnect operation cannot be performed on a WinRS shell command instance that is disconnected.
//
#define ERROR_WINRS_SHELLCOMMAND_DISCONNECT_OPERATION_NOT_VALID 0x803381D8L

//
// MessageId: ERROR_WSMAN_SUBSCRIBE_WMI_INVALID_KEY
//
// MessageText:
//
// The WS-Management service cannot process the request. The resource URI for the Subscribe operation must not contain keys.
//
#define ERROR_WSMAN_SUBSCRIBE_WMI_INVALID_KEY 0x803381D9L

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_DISCONNECT_SHELL_FLAG
//
// MessageText:
//
// The WinRM client cannot process the request. 
// A flag that is not valid was specified for the WSManDisconnectShell method.
// Remove or change the flag and retry the operation.
//
#define ERROR_WSMAN_CLIENT_INVALID_DISCONNECT_SHELL_FLAG 0x803381DAL

//
// MessageId: ERROR_WSMAN_CLIENT_INVALID_SHELL_COMMAND_PAIR
//
// MessageText:
//
// The WinRM client cannot process the request
// because the command handle is not associated with the provided shell handle.
//
#define ERROR_WSMAN_CLIENT_INVALID_SHELL_COMMAND_PAIR 0x803381DBL

//
// MessageId: ERROR_WSMAN_SEMANTICCALLBACK_TIMEDOUT
//
// MessageText:
//
// The WS-Management service did not receive a response for an extended semantics operation within the timeframe specified in the OperationTimeout setting.
//
#define ERROR_WSMAN_SEMANTICCALLBACK_TIMEDOUT 0x803381DCL

//
// MessageId: ERROR_WSMAN_SERVICE_REMOTE_ACCESS_DISABLED
//
// MessageText:
//
// The WS-Management service is configured to not allow remote requests. 
//
#define ERROR_WSMAN_SERVICE_REMOTE_ACCESS_DISABLED 0x803381DDL

//
// MessageId: ERROR_WSMAN_SERVICE_STREAM_DISCONNECTED
//
// MessageText:
//
// The WS-Management service cannot process the request because the stream is currently disconnected.
//
#define ERROR_WSMAN_SERVICE_STREAM_DISCONNECTED 0x803381DEL

//
// MessageId: ERROR_WSMAN_CREATESHELL_RUNAS_FAILED
//
// MessageText:
//
// The creation of a new Shell failed. Verify that the RunAsPassword value is correctly configured and that the Group Policy setting "Disallow WinRM from storing RunAs credentials" is Disabled or Not Configured.
// To enable WinRM to store RunAs credentials, change this Group Policy setting to Disabled.
//
#define ERROR_WSMAN_CREATESHELL_RUNAS_FAILED 0x803381DFL

//
// MessageId: ERROR_WSMAN_INVALID_XML_RUNAS_DISABLED
//
// MessageText:
//
// The supplied plugin configuration XML is not valid.
// To enable WinRM to store RunAs credentials, change the "Disallow WinRM from storing RunAs credentials" Group Policy setting to Disabled.
//
#define ERROR_WSMAN_INVALID_XML_RUNAS_DISABLED 0x803381E0L

//
// MessageId: ERROR_WSMAN_WRONG_METADATA
//
// MessageText:
//
// The WinRM client cannot process the request because the XML instance does not match the class schema provided by the server.
//
#define ERROR_WSMAN_WRONG_METADATA       0x803381E1L

//
// MessageId: ERROR_WSMAN_UNSUPPORTED_TYPE
//
// MessageText:
//
// The WinRM client cannot process the request because the XML contains an unsupported type. Verify the XML and retry the operation.
//
#define ERROR_WSMAN_UNSUPPORTED_TYPE     0x803381E2L

//
// MessageId: ERROR_WSMAN_REMOTE_CONNECTION_NOT_ALLOWED
//
// MessageText:
//
// The WS-Management service cannot process the request.
// The service is configured to reject remote connection requests for this plugin.
//
#define ERROR_WSMAN_REMOTE_CONNECTION_NOT_ALLOWED 0x803381E3L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_SHELLS_PPQ
//
// MessageText:
//
// The WS-Management service cannot process the request. This user has exceeded the maximum number of concurrent shells allowed for this plugin.
// Close at least one open shell or raise the plugin quota for this user.
//
#define ERROR_WSMAN_QUOTA_MAX_SHELLS_PPQ 0x803381E4L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_USERS_PPQ
//
// MessageText:
//
// The WS-Management service cannot process the request. The maximum number of users executing remote operations has been exceeded for this plugin.
// Retry the request later or raise the quota for concurrent users.
//
#define ERROR_WSMAN_QUOTA_MAX_USERS_PPQ  0x803381E5L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_PLUGINSHELLS_PPQ
//
// MessageText:
//
// The WS-Management service cannot process the request. The maximum number of concurrent shells allowed for this plugin has been exceeded.
// Retry the request later or raise the Maximum Shells per Plugin quota.
//
#define ERROR_WSMAN_QUOTA_MAX_PLUGINSHELLS_PPQ 0x803381E6L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_PLUGINOPERATIONS_PPQ
//
// MessageText:
//
// The WS-Management service cannot process the request. The maximum number of concurrent operations allowed for this plugin has been exceeded.
// Retry the request later or raise the Maximum Operations per Plugin quota.
//
#define ERROR_WSMAN_QUOTA_MAX_PLUGINOPERATIONS_PPQ 0x803381E7L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_OPERATIONS_USER_PPQ
//
// MessageText:
//
// The WS-Management service cannot process the request. This user has exceeded the maximum number of allowed concurrent operations.
// Retry the request later or raise the Maximum Operations per User quota.
//
#define ERROR_WSMAN_QUOTA_MAX_OPERATIONS_USER_PPQ 0x803381E8L

//
// MessageId: ERROR_WSMAN_QUOTA_MAX_COMMANDS_PER_SHELL_PPQ
//
// MessageText:
//
// The WS-Management service cannot process the request. The maximum number of concurrent commands per shell has been exceeded.
// Retry the request later or raise the Maximum Commands per Shell quota.
//
#define ERROR_WSMAN_QUOTA_MAX_COMMANDS_PER_SHELL_PPQ 0x803381E9L

//
// MessageId: ERROR_WSMAN_QUOTA_MIN_REQUIREMENT_NOT_AVAILABLE_PPQ
//
// MessageText:
//
// The WS-Management service cannot process the request.
// There are not enough resources available to process this operation.
// Retry the operation later or close one or more of the currently running operations.
//
#define ERROR_WSMAN_QUOTA_MIN_REQUIREMENT_NOT_AVAILABLE_PPQ 0x803381EAL

//
// MessageId: ERROR_WSMAN_NEW_DESERIALIZER
//
// MessageText:
//
// The WinRM client cannot process the request because the MI Deserializer cannot be created.
//
#define ERROR_WSMAN_NEW_DESERIALIZER     0x803381EBL

//
// MessageId: ERROR_WSMAN_DESERIALIZE_CLASS
//
// MessageText:
//
// The WinRM client cannot process the request because the metadata could not be deserialized.
//
#define ERROR_WSMAN_DESERIALIZE_CLASS    0x803381ECL

//
// MessageId: ERROR_WSMAN_GETCLASS
//
// MessageText:
//
// The WinRM client cannot process the request because the metadata failed to be retrieved from the server.
//
#define ERROR_WSMAN_GETCLASS             0x803381EDL

//
// MessageId: ERROR_WSMAN_NEW_SESSION
//
// MessageText:
//
// The WinRM client cannot process the request because a WinRM session could not be created.
//
#define ERROR_WSMAN_NEW_SESSION          0x803381EEL

//
// MessageId: ERROR_WSMAN_NULL_KEY
//
// MessageText:
//
// The WinRM client cannot process the request because the target object has a key property set to NULL.
// Incomplete objects cannot be used as the target of an operation.
//
#define ERROR_WSMAN_NULL_KEY             0x803381EFL

//
// MessageId: ERROR_WSMAN_MUTUAL_AUTH_FAILED
//
// MessageText:
//
// The WinRM client cannot process the request as the server identity could not be verified. If the identity of the server is trusted, add it to the TrustedHosts list and retry the request.
// Use winrm.cmd to configure TrustedHosts. Note that computers in the TrustedHosts list might not be authenticated.
// For more information on how to set TrustedHosts, run the following command: winrm help config
//
#define ERROR_WSMAN_MUTUAL_AUTH_FAILED   0x803381F0L

//
// MessageId: ERROR_WSMAN_UNSUPPORTED_OCTETTYPE
//
// MessageText:
//
// The WinRM client cannot process the request because the octet string array type is not supported.
//
#define ERROR_WSMAN_UNSUPPORTED_OCTETTYPE 0x803381F1L

//
// MessageId: ERROR_WINRS_IDLETIMEOUT_OUTOFBOUNDS
//
// MessageText:
//
// The WS-Management service cannot process the request. The requested IdleTimeout is outside the allowed range.
//
#define ERROR_WINRS_IDLETIMEOUT_OUTOFBOUNDS 0x803381F2L

//
// MessageId: ERROR_WSMAN_INSUFFICIENT_METADATA_FOR_BASIC
//
// MessageText:
//
// The WinRM client cannot process the request because insufficient metadata is available.
// The application does not allow all properties to be returned as strings, but the server does not support correctly typing the properties.
// Change the request to allow all properties to be returned as strings and retry the request.
//
#define ERROR_WSMAN_INSUFFICIENT_METADATA_FOR_BASIC 0x803381F3L

//
// MessageId: ERROR_WSMAN_INVALID_LITERAL_URI
//
// MessageText:
//
// The WinRM client cannot process the request because the MI_OperationOptions contained both a Resource URI and a Resource URI Prefix. Specify only one of these two options and try again.
//
#define ERROR_WSMAN_INVALID_LITERAL_URI  0x803381F4L

//
// MessageId: ERROR_WSMAN_OBJECTONLY_INVALID
//
// MessageText:
//
// The WinRM client cannot process the request because keysOnly and WSMAN_ENUMERATIONMODE_OBJECTONLY were specified at the same time.
// These two settings are incompatible.
// Remove the WSMAN_ENUMERATIONMODE_OBJECTONLY option, or set keysOnly to MI_FALSE, and retry the request.
//
#define ERROR_WSMAN_OBJECTONLY_INVALID   0x803381F5L

//
// MessageId: ERROR_WSMAN_MISSING_CLASSNAME
//
// MessageText:
//
// The WinRM client cannot process the request because the class name is not valid.
// Supply a valid class name or set the Resource URI option and retry the request.
//
#define ERROR_WSMAN_MISSING_CLASSNAME    0x803381F6L

// wsman, code=Sender, subcode=WS-Management UnsupportedFeature, details= /AddressingMode
//
// MessageId: ERROR_WSMAN_EVENTING_INVALID_ENCODING_IN_DELIVERY
//
// MessageText:
//
// The subscribe packet contains an Encoding value that is not valid in the delivery section.
//
#define ERROR_WSMAN_EVENTING_INVALID_ENCODING_IN_DELIVERY 0x803381F7L

//
// MessageId: ERROR_WSMAN_DESTINATION_INVALID
//
// MessageText:
//
// The WinRM client cannot process the request.
// The destination computer name must be a hostname or an IP address, and must not be a URL.
// To use an IPv6 address, enclose the address in brackets, like the following: "[::1]".
// The transport, port number, and URL prefix may be controlled by setting the appropriate destination options.
// Change the destination computer name string and retry the operation.
//
#define ERROR_WSMAN_DESTINATION_INVALID  0x803381F8L

//
// MessageId: ERROR_WSMAN_UNSUPPORTED_FEATURE_IDENTIFY
//
// MessageText:
//
// The server does not support WS-Management Identify operations. Skip the TestConnection part of the request and try again.
//
#define ERROR_WSMAN_UNSUPPORTED_FEATURE_IDENTIFY 0x803381F9L

//
// MessageId: ERROR_WSMAN_CLIENT_SESSION_UNUSABLE
//
// MessageText:
//
// The WS-Management service cannot process the operation. The operation is being attempted on a client session that is unusable. 
// This may be related to a recent restart of the WS-Management service. Please create a new client session and retry the operation if re-executing the operation does not have undesired behavior.
//
#define ERROR_WSMAN_CLIENT_SESSION_UNUSABLE 0x803381FAL

//
// MessageId: ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED
//
// MessageText:
//
// The WS-Management service cannot process the operation. An attempt to create a virtual account failed. Ensure that WinRM service is running as Local System and that it has TCB privilege enabled.
//
#define ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED 0x803381FBL

//
// MessageId: ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED_DOWNLEVEL
//
// MessageText:
//
// The WS-Management service cannot process the operation. Virtual account feature is only available in Windows 7, Server 2008 R2 and above.
//
#define ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED_DOWNLEVEL 0x803381FCL

//
// MessageId: ERROR_WSMAN_RUNASUSER_MANAGEDACCOUNT_LOGON_FAILED
//
// MessageText:
//
// The WS-Management service cannot process the operation. An attempt to logon using the configured RunAs Managed Service Account failed.
//
#define ERROR_WSMAN_RUNASUSER_MANAGEDACCOUNT_LOGON_FAILED 0x803381FDL

//
// MessageId: ERROR_WSMAN_CERTMAPPING_CREDENTIAL_MANAGEMENT_FAILIED
//
// MessageText:
//
// The WS-Management service cannot process the operation. An attempt to query mapped credential failed. This will happen if the security context associated with WinRM service has changed since the credential was originally mapped.
//
#define ERROR_WSMAN_CERTMAPPING_CREDENTIAL_MANAGEMENT_FAILIED 0x803381FEL

//
// MessageId: ERROR_WSMAN_EVENTING_PUSH_SUBSCRIPTION_NOACTIVATE_EVENTSOURCE
//
// MessageText:
//
// The event source of the push subscription is in disable or inactive on the Event controller server.
//
#define ERROR_WSMAN_EVENTING_PUSH_SUBSCRIPTION_NOACTIVATE_EVENTSOURCE 0x803381FFL

