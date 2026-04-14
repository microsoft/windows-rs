/**************************************************************************\
    Copyright Microsoft Corporation. All Rights Reserved.
\**************************************************************************/


#ifndef _CONTACT_PROPERTIES_H_
#define _CONTACT_PROPERTIES_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// ===================================================================
// Windows contact schema
//
// DESCRIPTION:
//      This defines how the contact schema is used to read and write
//  contact properties via IContactProperties.
//
//
// STRUCTURE:
//      Contact properties fall into one of two categories:
//
//  Category 1 - Single Value: These properties have a single simple value
//                      and don't require a hierarchy structure.
//
//  Category 2 - Hierarchical: These properties have multiple values for any contact
//                      and require labeling to differentiate individual values.
//
//                      PhoneNumber is one example of this type of property
//                      Any given contact can have one or more home, work, and mobile phone numbers.
//
//
// PROPERTY EXTENSIBILITY:
//      All applications that use contacts need to have additional data available
//      on contacts that the base schema does not provide for.
//
//      There are two ways supported by IContactProperties to extend the contact schema.
//
//      1) Labels (arbitrary strings) can be applied to any existing OS contact array node.
//          Ex: phoneNumbers/PhoneNumber[1] can be labeled with the two labels: Preferred and Business.
//
//              The set of built-in labels for all multi value properties are:
//                    Preferred
//                    Personal
//                    Business
//                    Other
//
//                Some collections can have more labels set (PhoneNumber can have Mobile, Fax, ...)
//
//              Additional labels can be set.  Custom labels must be in the form of URIs.
//
//              Manipulation of labels can be done with IContactProperties::GetLabel / SetLabel / DeleteLabels
//
//              Filtering of data contact can be done with IContactProperties::GetPropertyCollection
//
//      2) New contact properties and array nodes can be defined by an application.
//            The data contained in these properties can be enumerated by other users of IContactProperties
//
//          To create a new property for an application, the same interface is used as standard properties.
//            Use the same property Set functions as built in properties.  The caller must prepend
//            a string with a namespace (in braces) for the contact property.
//
//              **NB: make sure the selected namespace is unique to avoid conflicts with other applications**
//
//           For the application custom.exe to Get or Set a new string property, use:
//              ex:   [CustomNameSpace]CustomStringProperty
//
//           Creating new multivalue properties is also supported.
//            The above syntax is extended to include an array node name
//            for the the call to IContactProperties::CreateArrayNode
//
//              ex:   [CustomNameSpace:CustomArrayNodeName]customArrayName
//
//            CreateArrayNode will return the new property name (as always) in the outbound arguments.
//
//===================================================================

// ===================================================================
// Single Value properties
//
// DESCRIPTION:
//      These properties are simple and have no hierarchy.
//

//
// String properties
//

// free text content
#define CONTACTPROP_PUB_NOTES                       L"Notes"

// contact's email program
#define CONTACTPROP_PUB_MAILER                      L"Mailer"
// ProgID
#define CONTACTPROP_PUB_PROGID                      L"ProgID"

// gender of contact
// Chooose one of L"Male", L"Female", L"Unspecified" (default)
#define CONTACTPROP_PUB_GENDER                      L"Gender"

#define     CONTACTPROP_PUB_GENDER_UNSPECIFIED          L"Unspecified"
#define     CONTACTPROP_PUB_GENDER_MALE                 L"Male"
#define     CONTACTPROP_PUB_GENDER_FEMALE               L"Female"

//
// DateTime Properties
//

// The date and time the contact was created in the system
#define CONTACTPROP_PUB_CREATIONDATE                L"CreationDate"



// ===================================================================
// Hierarchical properties
//
// DESCRIPTION:
//      These properties contain many values differentiated by labels
//
//      Individual properties are accessed via IContactProperties by index
//
//      examples property names include:
//          PhoneNumberCollection/PhoneNumber[1]/Number
//              for the 1st phone number on the contact
//
//          EmailAddressCollection/EmailAddress[4]/Address
//              for the 4th email address on the contact
//
//      Labels differentiate second level entries(L2) entries.
//          Programmatic access for labels is via IContactProperties (see icontact.idl)
//

//
// collection of ContactIDs associated with this contact
//
#define CONTACTPROP_PUB_L1_CONTACTIDCOLLECTION      L"ContactIDCollection"
// an entry in the collection of IDs
#define     CONTACTPROP_PUB_L2_CONTACTID                L"/ContactID"
// one of the unique identifiers for this contact (as a string)
#define         CONTACTPROP_PUB_L3_VALUE                    L"/Value"

//
// collection of names associated with this contact
//
#define CONTACTPROP_PUB_L1_NAMECOLLECTION           L"NameCollection"
// an entry in the collection of names
#define     CONTACTPROP_PUB_L2_NAME                     L"/Name"

//
// Contact data related to names
//

// as displayed
#define CONTACTPROP_PUB_L3_FORMATTEDNAME                    L"/FormattedName"
// as pronounced
#define CONTACTPROP_PUB_L3_PHONETIC                         L"/Phonetic"

#define CONTACTPROP_PUB_L3_PREFIX                           L"/Prefix"
#define CONTACTPROP_PUB_L3_TITLE                            L"/Title"
#define CONTACTPROP_PUB_L3_GIVENNAME                        L"/GivenName"
#define CONTACTPROP_PUB_L3_FAMILYNAME                       L"/FamilyName"
#define CONTACTPROP_PUB_L3_MIDDLENAME                       L"/MiddleName"
#define CONTACTPROP_PUB_L3_GENERATION                       L"/Generation"
#define CONTACTPROP_PUB_L3_SUFFIX                           L"/Suffix"
#define CONTACTPROP_PUB_L3_NICKNAME                         L"/NickName"

//
// Contact data relating to positions a contact holds
//
#define CONTACTPROP_PUB_L1_POSITIONCOLLECTION       L"PositionCollection"
// an entry in the collection of names
#define     CONTACTPROP_PUB_L2_POSITION                 L"/Position"

// the organization ex: IEEE
#define CONTACTPROP_PUB_L3_ORGANIZATION                     L"/Organization"
// the company ex: Microsoft
#define CONTACTPROP_PUB_L3_COMPANY                          L"/Company"
// the department ex: Windows
#define CONTACTPROP_PUB_L3_DEPARTMENT                       L"/Department"
// the office ex: Building A / Office 1234
#define CONTACTPROP_PUB_L3_OFFICE                           L"/Office"
// any job title ex: Software Engineer
#define CONTACTPROP_PUB_L3_JOB_TITLE                        L"/JobTitle"
// the line of work ex: Engineering
#define CONTACTPROP_PUB_L3_PROFESSION                       L"/Profession"
// the role in the organization ex: Quality Assurance
#define CONTACTPROP_PUB_L3_ROLE                             L"/Role"



//
// people associated with the contact
//
#define CONTACTPROP_PUB_L1_PERSONCOLLECTION         L"PersonCollection"
// entry in the collection
#define     CONTACTPROP_PUB_L2_PERSON                   L"/Person"
//
// Use labels to indicate the relationship to contact
// ex:
//      wab:Spouse
//      wab:Child
//      wab:Manager
//      wab:Assistant

// person's formatted (display) name - (as string)
#define CONTACTPROP_PUB_L3_FORMATTEDNAME                    L"/FormattedName"

// a unique identifier for this person (optional),
// which may be one of the ContactIDs contained in an IContact 
#define CONTACTPROP_PUB_L3_PERSONID                        L"/PersonID"

//
// calendar dates associated with the contact
//
#define CONTACTPROP_PUB_L1_DATECOLLECTION           L"DateCollection"
// entry in the collection
#define     CONTACTPROP_PUB_L2_DATE                     L"/Date"
//
// Use labels to indicate the type of data being expressed.
// ex: 
//    wab:Birthday - the date of birth for the contact
//    wab:Anniversary - the spouse anniversary date for the contact

// value for this date, as a DateTime
#define         CONTACTPROP_PUB_L3_VALUE                    L"/Value"


//
// email addresses
//
#define CONTACTPROP_PUB_L1_EMAILADDRESSCOLLECTION       L"EmailAddressCollection"
// entry in the collection
#define     CONTACTPROP_PUB_L2_EMAILADDRESS                 L"/EmailAddress"
// example@microsoft.com (as string)
#define             CONTACTPROP_PUB_L3_ADDRESS                  L"/Address"
// type of address (e.g. SMTP, x509)
#define             CONTACTPROP_PUB_L3_TYPE                     L"/Type"


//
// certificate data and thumbprints
//

#define CONTACTPROP_PUB_L1_CERTIFICATECOLLECTION        L"CertificateCollection"
// entry in the collection
#define     CONTACTPROP_PUB_L2_CERTIFICATE                  L"/Certificate"
// certificate value
#define         CONTACTPROP_PUB_L3_VALUE                        L"/Value"
// thumbprint value
#define         CONTACTPROP_PUB_L3_THUMBPRINT                   L"/ThumbPrint"

//
// phone numbers
//
#define CONTACTPROP_PUB_L1_PHONENUMBERCOLLECTION    L"PhoneNumberCollection"
// entry in the collection
#define     CONTACTPROP_PUB_L2_PHONENUMBER              L"/PhoneNumber"
// normal number to display (as string)
#define             CONTACTPROP_PUB_L3_NUMBER               L"/Number"
// alternate number (tty) (as string)
#define             CONTACTPROP_PUB_L3_ALTERNATE            L"/Alternate"


//
// physical addresses
//
#define CONTACTPROP_PUB_L1_PHYSICALADDRESSCOLLECTION    L"PhysicalAddressCollection"
// entry in the collection
#define     CONTACTPROP_PUB_L2_PHYSICALADDRESS              L"/PhysicalAddress"
// the exact data that a mailing label should have
#define             CONTACTPROP_PUB_L3_ADDRESSLABEL             L"/AddressLabel"
// number and street
#define             CONTACTPROP_PUB_L3_STREET                   L"/Street"
// City
#define             CONTACTPROP_PUB_L3_LOCALITY                 L"/Locality"
// State/Providence
#define             CONTACTPROP_PUB_L3_REGION                   L"/Region"
// Zip / PostalCode
#define             CONTACTPROP_PUB_L3_POSTALCODE               L"/PostalCode"
// the country
#define             CONTACTPROP_PUB_L3_COUNTRY                  L"/Country"
// any POBox number
#define             CONTACTPROP_PUB_L3_POBOX                    L"/POBox"
// any extra information
#define             CONTACTPROP_PUB_L3_EXTENDEDADDRESS          L"/ExtendedAddress"

//
// Instant Messaging Addresess and protocols
//
#define CONTACTPROP_PUB_L1_IMADDRESSCOLLECTION              L"IMAddressCollection"
#define     CONTACTPROP_PUB_L2_IMADDRESSENTRY                   L"/IMAddress"
// the identifing data for this ImAddress (ex: username@microsoft.com)
#define         CONTACTPROP_PUB_L3_VALUE                        L"/Value"
// the string protocol used for this ImAddress (ex: Messenger Protocol)
#define         CONTACTPROP_PUB_L3_PROTOCOL                     L"/Protocol"

//
// collection of URLs associated with this contact
//
#define CONTACTPROP_PUB_L1_URLCOLLECTION            L"UrlCollection"
// an entry in the collection of url
#define     CONTACTPROP_PUB_L2_URL                      L"/Url"
// the actual URL data
#define         CONTACTPROP_PUB_L3_VALUE                    L"/Value"

//
// collection of images associated with this contact
//
#define CONTACTPROP_PUB_L1_PHOTOCOLLECTION          L"PhotoCollection"
// an entry in the collection of photos
#define CONTACTPROP_PUB_L2_PHOTO                        L"/Photo"
// an image to use for representing the contact - as binary, with MIME type
#define CONTACTPROP_PUB_L3_VALUE                            L"/Value"
// a URL for retrieving the image - as a string
#define CONTACTPROP_PUB_L3_URL                              L"/Url"



// ===================================================================
//
// Common Labels that may be associated with any contact properties.
//

//
// Labels can also be URIs.  See comment at beginning of this file
//

// NOTE: many entries in a set may have this "Preferred" label set
#define CONTACTLABEL_PUB_PREFERRED                   L"Preferred"

// Home related data
#define CONTACTLABEL_PUB_PERSONAL                    L"Personal"

// Work related data
#define CONTACTLABEL_PUB_BUSINESS                    L"Business"

// other, non specified label
#define CONTACTLABEL_PUB_OTHER                       L"Other"

//
// Labels that can be associated with PhoneNumber elements
//

// number supports voice conversation
#define CONTACTLABEL_PUB_VOICE                       L"Voice"

// mobile phone number
#define CONTACTLABEL_PUB_MOBILE                      L"Mobile"

// PCS support
#define CONTACTLABEL_PUB_PCS                         L"PCS"

// cell phone support
#define CONTACTLABEL_PUB_CELLULAR                    L"Cellular"

// number travel with the Car
#define CONTACTLABEL_PUB_CAR                         L"Car"

// pager number
#define CONTACTLABEL_PUB_PAGER                       L"Pager"

// tty machine
#define CONTACTLABEL_PUB_TTY                         L"TTY"

// fax machine
#define CONTACTLABEL_PUB_FAX                         L"Fax"

// number supports video conversation
#define CONTACTLABEL_PUB_VIDEO                       L"Video"

// number for modem connection
#define CONTACTLABEL_PUB_MODEM                       L"Modem"

// number for BBS connection
#define CONTACTLABEL_PUB_BBS                         L"BBS"

// number for ISDN
#define CONTACTLABEL_PUB_ISDN                        L"ISDN"


//
// Labels that can be associated with Person elements
//

// to indicate this person is allowed to work on behalf of the contact
#define CONTACTLABEL_PUB_AGENT                       L"Agent"

//
// Labels that can be associated with PhysicalAddress elements
//

// a domestic mailing address
#define CONTACTLABEL_PUB_DOMESTIC                    L"Domestic"

// an international mailing address
#define CONTACTLABEL_PUB_INTERNATIONAL               L"International"

// a mailing address which accepts mail
#define CONTACTLABEL_PUB_POSTAL                      L"Postal"

// a mailing address that accepts packages
#define CONTACTLABEL_PUB_PARCEL                      L"Parcel"


//
// Labels that can be associated with Photo elements
//

// an image used to represent the contact
#define CONTACTLABEL_PUB_USERTILE                    L"UserTile"

// a logo associated with the contact (ex: image for organization)
#define CONTACTLABEL_PUB_LOGO                        L"Logo"

//
// windows address book specific labels
//

// for PersonCollection
#define CONTACTLABEL_WAB_SPOUSE                      L"wab:Spouse"
#define CONTACTLABEL_WAB_CHILD                       L"wab:Child"
#define CONTACTLABEL_WAB_MANAGER                     L"wab:Manager"
#define CONTACTLABEL_WAB_ASSISTANT                   L"wab:Assistant"

// for DateCollection 
#define CONTACTLABEL_WAB_BIRTHDAY                    L"wab:Birthday"
#define CONTACTLABEL_WAB_ANNIVERSARY                 L"wab:Anniversary"

// for UrlCollection
#define CONTACTLABEL_WAB_SOCIALNETWORK               L"wab:SocialNetwork"
#define CONTACTLABEL_WAB_SCHOOL                      L"wab:School"
#define CONTACTLABEL_WAB_WISHLIST                    L"wab:WishList"



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _CONTACT_PROPERTIES_H_


