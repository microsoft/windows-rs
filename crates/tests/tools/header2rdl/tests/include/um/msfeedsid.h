//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation.  All Rights Reserved.
//
//  File:       msfeedsid.h
//
//----------------------------------------------------------------------------


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define DISPID_FEEDS_RootFolder                                 0x00001000
#define DISPID_FEEDS_IsSubscribed                               0x00001001
#define DISPID_FEEDS_ExistsFeed                                 0x00001002
#define DISPID_FEEDS_GetFeed                                    0x00001003
#define DISPID_FEEDS_ExistsFolder                               0x00001004
#define DISPID_FEEDS_GetFolder                                  0x00001005
#define DISPID_FEEDS_DeleteFeed                                 0x00001006
#define DISPID_FEEDS_DeleteFolder                               0x00001007
#define DISPID_FEEDS_GetFeedByUrl                               0x00001008
#define DISPID_FEEDS_BackgroundSync                             0x00001009
#define DISPID_FEEDS_BackgroundSyncStatus                       0x0000100a
#define DISPID_FEEDS_DefaultInterval                            0x0000100b
#define DISPID_FEEDS_AsyncSyncAll                               0x0000100c
#define DISPID_FEEDS_Normalize                                  0x0000100d
#define DISPID_FEEDS_ItemCountLimit                             0x0000100e

#define DISPID_FEEDSENUM_Count                                  0x00002000
#define DISPID_FEEDSENUM_Item                                   0x00002001

#define DISPID_FEEDFOLDER_Feeds                                 0x00003000
#define DISPID_FEEDFOLDER_Subfolders                            0x00003001
#define DISPID_FEEDFOLDER_CreateFeed                            0x00003002
#define DISPID_FEEDFOLDER_CreateSubfolder                       0x00003003
#define DISPID_FEEDFOLDER_ExistsFeed                            0x00003004
#define DISPID_FEEDFOLDER_GetFeed                               0x00003005
#define DISPID_FEEDFOLDER_ExistsSubfolder                       0x00003006
#define DISPID_FEEDFOLDER_GetSubfolder                          0x00003007
#define DISPID_FEEDFOLDER_Delete                                0x00003008
#define DISPID_FEEDFOLDER_Name                                  0x00003009
#define DISPID_FEEDFOLDER_Rename                                0x0000300a
#define DISPID_FEEDFOLDER_Path                                  0x0000300b
#define DISPID_FEEDFOLDER_Move                                  0x0000300c
#define DISPID_FEEDFOLDER_Parent                                0x0000300d
#define DISPID_FEEDFOLDER_IsRoot                                0x0000300e
#define DISPID_FEEDFOLDER_TotalUnreadItemCount                  0x0000300f
#define DISPID_FEEDFOLDER_TotalItemCount                        0x00003010
#define DISPID_FEEDFOLDER_GetWatcher                            0x00003011

#define DISPID_FEED_Xml                                         0x00004000
#define DISPID_FEED_Name                                        0x00004001
#define DISPID_FEED_Rename                                      0x00004002
#define DISPID_FEED_Url                                         0x00004003
#define DISPID_FEED_LocalId                                     0x00004004
#define DISPID_FEED_Path                                        0x00004005
#define DISPID_FEED_Move                                        0x00004006
#define DISPID_FEED_Parent                                      0x00004007
#define DISPID_FEED_LastWriteTime                               0x00004008
#define DISPID_FEED_Delete                                      0x00004009
#define DISPID_FEED_Download                                    0x0000400a
#define DISPID_FEED_AsyncDownload                               0x0000400b
#define DISPID_FEED_CancelAsyncDownload                         0x0000400c
#define DISPID_FEED_Interval                                    0x0000400d
#define DISPID_FEED_SyncSetting                                 0x0000400e
#define DISPID_FEED_LastDownloadTime                            0x0000400f
#define DISPID_FEED_LocalEnclosurePath                          0x00004010
#define DISPID_FEED_Items                                       0x00004011
#define DISPID_FEED_GetItem                                     0x00004012
#define DISPID_FEED_Title                                       0x00004013
#define DISPID_FEED_Description                                 0x00004014
#define DISPID_FEED_Link                                        0x00004015
#define DISPID_FEED_Image                                       0x00004016
#define DISPID_FEED_LastBuildDate                               0x00004017
#define DISPID_FEED_PubDate                                     0x00004018
#define DISPID_FEED_Ttl                                         0x00004019
#define DISPID_FEED_Language                                    0x0000401a
#define DISPID_FEED_Copyright                                   0x0000401b
#define DISPID_FEED_DownloadEnclosuresAutomatically             0x0000401c
#define DISPID_FEED_DownloadStatus                              0x0000401d
#define DISPID_FEED_LastDownloadError                           0x0000401e
#define DISPID_FEED_Merge                                       0x0000401f
#define DISPID_FEED_DownloadUrl                                 0x00004020
#define DISPID_FEED_IsList                                      0x00004021
#define DISPID_FEED_MarkAllItemsRead                            0x00004022
#define DISPID_FEED_GetWatcher                                  0x00004023
#define DISPID_FEED_UnreadItemCount                             0x00004024
#define DISPID_FEED_ItemCount                                   0x00004025
#define DISPID_FEED_MaxItemCount                                0x00004026
#define DISPID_FEED_GetItemByEffectiveId                        0x00004027
#define DISPID_FEED_LastItemDownloadTime                        0x00004028
#define DISPID_FEED_Username                                    0x00004029
#define DISPID_FEED_Password                                    0x0000402a
#define DISPID_FEED_SetCredentials                              0x0000402b
#define DISPID_FEED_ClearCredentials                            0x0000402c

#define DISPID_FEEDITEM_Xml                                     0x00005000
#define DISPID_FEEDITEM_Title                                   0x00005001
#define DISPID_FEEDITEM_Link                                    0x00005002
#define DISPID_FEEDITEM_Guid                                    0x00005003
#define DISPID_FEEDITEM_Description                             0x00005004
#define DISPID_FEEDITEM_PubDate                                 0x00005005
#define DISPID_FEEDITEM_Comments                                0x00005006
#define DISPID_FEEDITEM_Author                                  0x00005007
#define DISPID_FEEDITEM_Enclosure                               0x00005008
#define DISPID_FEEDITEM_IsRead                                  0x00005009
#define DISPID_FEEDITEM_LocalId                                 0x0000500a
#define DISPID_FEEDITEM_Parent                                  0x0000500b
#define DISPID_FEEDITEM_Delete                                  0x0000500c
#define DISPID_FEEDITEM_DownloadUrl                             0x0000500d
#define DISPID_FEEDITEM_LastDownloadTime                        0x0000500e
#define DISPID_FEEDITEM_Modified                                0x0000500f
#define DISPID_FEEDITEM_EffectiveId                             0x00005010

#define DISPID_FEEDENCLOSURE_Url                                0x00006000
#define DISPID_FEEDENCLOSURE_Type                               0x00006001
#define DISPID_FEEDENCLOSURE_Length                             0x00006002
#define DISPID_FEEDENCLOSURE_AsyncDownload                      0x00006003
#define DISPID_FEEDENCLOSURE_CancelAsyncDownload                0x00006004
#define DISPID_FEEDENCLOSURE_DownloadStatus                     0x00006005
#define DISPID_FEEDENCLOSURE_LastDownloadError                  0x00006006
#define DISPID_FEEDENCLOSURE_LocalPath                          0x00006007
#define DISPID_FEEDENCLOSURE_Parent                             0x00006008
#define DISPID_FEEDENCLOSURE_DownloadUrl                        0x00006009
#define DISPID_FEEDENCLOSURE_DownloadMimeType                   0x0000600a
#define DISPID_FEEDENCLOSURE_RemoveFile                         0x0000600b
#define DISPID_FEEDENCLOSURE_SetFile                            0x0000600c

#define DISPID_FEEDFOLDEREVENTS_Error                           0x00007000
#define DISPID_FEEDFOLDEREVENTS_FolderAdded                     0x00007001
#define DISPID_FEEDFOLDEREVENTS_FolderDeleted                   0x00007002
#define DISPID_FEEDFOLDEREVENTS_FolderRenamed                   0x00007003
#define DISPID_FEEDFOLDEREVENTS_FolderMovedFrom                 0x00007004
#define DISPID_FEEDFOLDEREVENTS_FolderMovedTo                   0x00007005
#define DISPID_FEEDFOLDEREVENTS_FolderItemCountChanged          0x00007006
#define DISPID_FEEDFOLDEREVENTS_FeedAdded                       0x00007007
#define DISPID_FEEDFOLDEREVENTS_FeedDeleted                     0x00007008
#define DISPID_FEEDFOLDEREVENTS_FeedRenamed                     0x00007009
#define DISPID_FEEDFOLDEREVENTS_FeedUrlChanged                  0x0000700a
#define DISPID_FEEDFOLDEREVENTS_FeedMovedFrom                   0x0000700b
#define DISPID_FEEDFOLDEREVENTS_FeedMovedTo                     0x0000700c
#define DISPID_FEEDFOLDEREVENTS_FeedDownloading                 0x0000700d
#define DISPID_FEEDFOLDEREVENTS_FeedDownloadCompleted           0x0000700e
#define DISPID_FEEDFOLDEREVENTS_FeedItemCountChanged            0x0000700f

#define DISPID_FEEDEVENTS_Error                                 0x00008000
#define DISPID_FEEDEVENTS_FeedDeleted                           0x00008001
#define DISPID_FEEDEVENTS_FeedRenamed                           0x00008002
#define DISPID_FEEDEVENTS_FeedUrlChanged                        0x00008003
#define DISPID_FEEDEVENTS_FeedMoved                             0x00008004
#define DISPID_FEEDEVENTS_FeedDownloading                       0x00008005
#define DISPID_FEEDEVENTS_FeedDownloadCompleted                 0x00008006
#define DISPID_FEEDEVENTS_FeedItemCountChanged                  0x00008007

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

