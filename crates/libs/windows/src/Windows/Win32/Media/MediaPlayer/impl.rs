#[cfg(feature = "Win32_System_Com")]
pub trait IFeedImpl: Sized + IDispatchImpl {
    fn Xml();
    fn Name();
    fn Rename();
    fn Url();
    fn SetUrl();
    fn LocalId();
    fn Path();
    fn Move();
    fn Parent();
    fn LastWriteTime();
    fn Delete();
    fn Download();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn SyncSetting();
    fn SetSyncSetting();
    fn Interval();
    fn SetInterval();
    fn LastDownloadTime();
    fn LocalEnclosurePath();
    fn Items();
    fn GetItem();
    fn Title();
    fn Description();
    fn Link();
    fn Image();
    fn LastBuildDate();
    fn PubDate();
    fn Ttl();
    fn Language();
    fn Copyright();
    fn MaxItemCount();
    fn SetMaxItemCount();
    fn DownloadEnclosuresAutomatically();
    fn SetDownloadEnclosuresAutomatically();
    fn DownloadStatus();
    fn LastDownloadError();
    fn Merge();
    fn DownloadUrl();
    fn IsList();
    fn MarkAllItemsRead();
    fn GetWatcher();
    fn UnreadItemCount();
    fn ItemCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeed2Impl: Sized + IFeedImpl + IDispatchImpl {
    fn GetItemByEffectiveId();
    fn LastItemDownloadTime();
    fn Username();
    fn Password();
    fn SetCredentials();
    fn ClearCredentials();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedEnclosureImpl: Sized + IDispatchImpl {
    fn Url();
    fn Type();
    fn Length();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn DownloadStatus();
    fn LastDownloadError();
    fn LocalPath();
    fn Parent();
    fn DownloadUrl();
    fn DownloadMimeType();
    fn RemoveFile();
    fn SetFile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedEventsImpl: Sized + IDispatchImpl {
    fn Error();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMoved();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedFolderImpl: Sized + IDispatchImpl {
    fn Feeds();
    fn Subfolders();
    fn CreateFeed();
    fn CreateSubfolder();
    fn ExistsFeed();
    fn GetFeed();
    fn ExistsSubfolder();
    fn GetSubfolder();
    fn Delete();
    fn Name();
    fn Rename();
    fn Path();
    fn Move();
    fn Parent();
    fn IsRoot();
    fn TotalUnreadItemCount();
    fn TotalItemCount();
    fn GetWatcher();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedFolderEventsImpl: Sized + IDispatchImpl {
    fn Error();
    fn FolderAdded();
    fn FolderDeleted();
    fn FolderRenamed();
    fn FolderMovedFrom();
    fn FolderMovedTo();
    fn FolderItemCountChanged();
    fn FeedAdded();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMovedFrom();
    fn FeedMovedTo();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedItemImpl: Sized + IDispatchImpl {
    fn Xml();
    fn Title();
    fn Link();
    fn Guid();
    fn Description();
    fn PubDate();
    fn Comments();
    fn Author();
    fn Enclosure();
    fn IsRead();
    fn SetIsRead();
    fn LocalId();
    fn Parent();
    fn Delete();
    fn DownloadUrl();
    fn LastDownloadTime();
    fn Modified();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedItem2Impl: Sized + IFeedItemImpl + IDispatchImpl {
    fn EffectiveId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedsEnumImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedsManagerImpl: Sized + IDispatchImpl {
    fn RootFolder();
    fn IsSubscribed();
    fn ExistsFeed();
    fn GetFeed();
    fn GetFeedByUrl();
    fn ExistsFolder();
    fn GetFolder();
    fn DeleteFeed();
    fn DeleteFolder();
    fn BackgroundSync();
    fn BackgroundSyncStatus();
    fn DefaultInterval();
    fn SetDefaultInterval();
    fn AsyncSyncAll();
    fn Normalize();
    fn ItemCountLimit();
}
pub trait IWMPAudioRenderConfigImpl: Sized {
    fn audioOutputDevice();
    fn SetaudioOutputDevice();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCdromImpl: Sized + IDispatchImpl {
    fn driveSpecifier();
    fn playlist();
    fn eject();
}
pub trait IWMPCdromBurnImpl: Sized {
    fn isAvailable();
    fn getItemInfo();
    fn label();
    fn Setlabel();
    fn burnFormat();
    fn SetburnFormat();
    fn burnPlaylist();
    fn SetburnPlaylist();
    fn refreshStatus();
    fn burnState();
    fn burnProgress();
    fn startBurn();
    fn stopBurn();
    fn erase();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCdromCollectionImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
    fn getByDriveSpecifier();
}
pub trait IWMPCdromRipImpl: Sized {
    fn ripState();
    fn ripProgress();
    fn startRip();
    fn stopRip();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPClosedCaptionImpl: Sized + IDispatchImpl {
    fn SAMIStyle();
    fn SetSAMIStyle();
    fn SAMILang();
    fn SetSAMILang();
    fn SAMIFileName();
    fn SetSAMIFileName();
    fn captioningId();
    fn SetcaptioningId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPClosedCaption2Impl: Sized + IWMPClosedCaptionImpl + IDispatchImpl {
    fn SAMILangCount();
    fn getSAMILangName();
    fn getSAMILangID();
    fn SAMIStyleCount();
    fn getSAMIStyleName();
}
pub trait IWMPContentContainerImpl: Sized {
    fn GetID();
    fn GetPrice();
    fn GetType();
    fn GetContentCount();
    fn GetContentPrice();
    fn GetContentID();
}
pub trait IWMPContentContainerListImpl: Sized {
    fn GetTransactionType();
    fn GetContainerCount();
    fn GetContainer();
}
pub trait IWMPContentPartnerImpl: Sized {
    fn SetCallback();
    fn Notify();
    fn GetItemInfo();
    fn GetContentPartnerInfo();
    fn GetCommands();
    fn InvokeCommand();
    fn CanBuySilent();
    fn Buy();
    fn GetStreamingURL();
    fn Download();
    fn DownloadTrackComplete();
    fn RefreshLicense();
    fn GetCatalogURL();
    fn GetTemplate();
    fn UpdateDevice();
    fn GetListContents();
    fn Login();
    fn Authenticate();
    fn Logout();
    fn SendMessage();
    fn StationEvent();
    fn CompareContainerListPrices();
    fn VerifyPermission();
}
pub trait IWMPContentPartnerCallbackImpl: Sized {
    fn Notify();
    fn BuyComplete();
    fn DownloadTrack();
    fn GetCatalogVersion();
    fn UpdateDeviceComplete();
    fn ChangeView();
    fn AddListContents();
    fn ListContentsComplete();
    fn SendMessageComplete();
    fn GetContentIDsInLibrary();
    fn RefreshLicenseComplete();
    fn ShowPopup();
    fn VerifyPermissionComplete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControlsImpl: Sized + IDispatchImpl {
    fn isAvailable();
    fn play();
    fn stop();
    fn pause();
    fn fastForward();
    fn fastReverse();
    fn currentPosition();
    fn SetcurrentPosition();
    fn currentPositionString();
    fn next();
    fn previous();
    fn currentItem();
    fn SetcurrentItem();
    fn currentMarker();
    fn SetcurrentMarker();
    fn playItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControls2Impl: Sized + IWMPControlsImpl + IDispatchImpl {
    fn step();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControls3Impl: Sized + IWMPControls2Impl + IWMPControlsImpl + IDispatchImpl {
    fn audioLanguageCount();
    fn getAudioLanguageID();
    fn getAudioLanguageDescription();
    fn currentAudioLanguage();
    fn SetcurrentAudioLanguage();
    fn currentAudioLanguageIndex();
    fn SetcurrentAudioLanguageIndex();
    fn getLanguageName();
    fn currentPositionTimecode();
    fn SetcurrentPositionTimecode();
}
pub trait IWMPConvertImpl: Sized {
    fn ConvertFile();
    fn GetErrorURL();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCoreImpl: Sized + IDispatchImpl {
    fn close();
    fn URL();
    fn SetURL();
    fn openState();
    fn playState();
    fn controls();
    fn settings();
    fn currentMedia();
    fn SetcurrentMedia();
    fn mediaCollection();
    fn playlistCollection();
    fn versionInfo();
    fn launchURL();
    fn network();
    fn currentPlaylist();
    fn SetcurrentPlaylist();
    fn cdromCollection();
    fn closedCaption();
    fn isOnline();
    fn error();
    fn status();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCore2Impl: Sized + IWMPCoreImpl + IDispatchImpl {
    fn dvd();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCore3Impl: Sized + IWMPCore2Impl + IWMPCoreImpl + IDispatchImpl {
    fn newPlaylist();
    fn newMedia();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDVDImpl: Sized + IDispatchImpl {
    fn isAvailable();
    fn domain();
    fn topMenu();
    fn titleMenu();
    fn back();
    fn resume();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadCollectionImpl: Sized + IDispatchImpl {
    fn id();
    fn count();
    fn item();
    fn startDownload();
    fn removeItem();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadItemImpl: Sized + IDispatchImpl {
    fn sourceURL();
    fn size();
    fn r#type();
    fn progress();
    fn downloadState();
    fn pause();
    fn resume();
    fn cancel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadItem2Impl: Sized + IWMPDownloadItemImpl + IDispatchImpl {
    fn getItemInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadManagerImpl: Sized + IDispatchImpl {
    fn getDownloadCollection();
    fn createDownloadCollection();
}
pub trait IWMPEffectsImpl: Sized {
    fn Render();
    fn MediaInfo();
    fn GetCapabilities();
    fn GetTitle();
    fn GetPresetTitle();
    fn GetPresetCount();
    fn SetCurrentPreset();
    fn GetCurrentPreset();
    fn DisplayPropertyPage();
    fn GoFullscreen();
    fn RenderFullScreen();
}
pub trait IWMPEffects2Impl: Sized + IWMPEffectsImpl {
    fn SetCore();
    fn Create();
    fn Destroy();
    fn NotifyNewMedia();
    fn OnWindowMessage();
    fn RenderWindowed();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorImpl: Sized + IDispatchImpl {
    fn clearErrorQueue();
    fn errorCount();
    fn item();
    fn webHelp();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorItemImpl: Sized + IDispatchImpl {
    fn errorCode();
    fn errorDescription();
    fn errorContext();
    fn remedy();
    fn customUrl();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorItem2Impl: Sized + IWMPErrorItemImpl + IDispatchImpl {
    fn condition();
}
pub trait IWMPEventsImpl: Sized {
    fn OpenStateChange();
    fn PlayStateChange();
    fn AudioLanguageChange();
    fn StatusChange();
    fn ScriptCommand();
    fn NewStream();
    fn Disconnect();
    fn Buffering();
    fn Error();
    fn Warning();
    fn EndOfStream();
    fn PositionChange();
    fn MarkerHit();
    fn DurationUnitChange();
    fn CdromMediaChange();
    fn PlaylistChange();
    fn CurrentPlaylistChange();
    fn CurrentPlaylistItemAvailable();
    fn MediaChange();
    fn CurrentMediaItemAvailable();
    fn CurrentItemChange();
    fn MediaCollectionChange();
    fn MediaCollectionAttributeStringAdded();
    fn MediaCollectionAttributeStringRemoved();
    fn MediaCollectionAttributeStringChanged();
    fn PlaylistCollectionChange();
    fn PlaylistCollectionPlaylistAdded();
    fn PlaylistCollectionPlaylistRemoved();
    fn PlaylistCollectionPlaylistSetAsDeleted();
    fn ModeChange();
    fn MediaError();
    fn OpenPlaylistSwitch();
    fn DomainChange();
    fn SwitchedToPlayerApplication();
    fn SwitchedToControl();
    fn PlayerDockedStateChange();
    fn PlayerReconnect();
    fn Click();
    fn DoubleClick();
    fn KeyDown();
    fn KeyPress();
    fn KeyUp();
    fn MouseDown();
    fn MouseMove();
    fn MouseUp();
}
pub trait IWMPEvents2Impl: Sized + IWMPEventsImpl {
    fn DeviceConnect();
    fn DeviceDisconnect();
    fn DeviceStatusChange();
    fn DeviceSyncStateChange();
    fn DeviceSyncError();
    fn CreatePartnershipComplete();
}
pub trait IWMPEvents3Impl: Sized + IWMPEvents2Impl + IWMPEventsImpl {
    fn CdromRipStateChange();
    fn CdromRipMediaError();
    fn CdromBurnStateChange();
    fn CdromBurnMediaError();
    fn CdromBurnError();
    fn LibraryConnect();
    fn LibraryDisconnect();
    fn FolderScanStateChange();
    fn StringCollectionChange();
    fn MediaCollectionMediaAdded();
    fn MediaCollectionMediaRemoved();
}
pub trait IWMPEvents4Impl: Sized + IWMPEvents3Impl + IWMPEvents2Impl + IWMPEventsImpl {
    fn DeviceEstimation();
}
pub trait IWMPFolderMonitorServicesImpl: Sized {
    fn count();
    fn item();
    fn add();
    fn remove();
    fn scanState();
    fn currentFolder();
    fn scannedFilesCount();
    fn addedFilesCount();
    fn updateProgress();
    fn startScan();
    fn stopScan();
}
pub trait IWMPGraphCreationImpl: Sized {
    fn GraphCreationPreRender();
    fn GraphCreationPostRender();
    fn GetGraphCreationFlags();
}
pub trait IWMPLibraryImpl: Sized {
    fn name();
    fn r#type();
    fn mediaCollection();
    fn isIdentical();
}
pub trait IWMPLibrary2Impl: Sized + IWMPLibraryImpl {
    fn getItemInfo();
}
pub trait IWMPLibraryServicesImpl: Sized {
    fn getCountByType();
    fn getLibraryByType();
}
pub trait IWMPLibrarySharingServicesImpl: Sized {
    fn isLibraryShared();
    fn isLibrarySharingEnabled();
    fn showLibrarySharing();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMediaImpl: Sized + IDispatchImpl {
    fn isIdentical();
    fn sourceURL();
    fn name();
    fn Setname();
    fn imageSourceWidth();
    fn imageSourceHeight();
    fn markerCount();
    fn getMarkerTime();
    fn getMarkerName();
    fn duration();
    fn durationString();
    fn attributeCount();
    fn getAttributeName();
    fn getItemInfo();
    fn setItemInfo();
    fn getItemInfoByAtom();
    fn isMemberOf();
    fn isReadOnlyItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMedia2Impl: Sized + IWMPMediaImpl + IDispatchImpl {
    fn error();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMedia3Impl: Sized + IWMPMedia2Impl + IWMPMediaImpl + IDispatchImpl {
    fn getAttributeCountByType();
    fn getItemInfoByType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMediaCollectionImpl: Sized + IDispatchImpl {
    fn add();
    fn getAll();
    fn getByName();
    fn getByGenre();
    fn getByAuthor();
    fn getByAlbum();
    fn getByAttribute();
    fn remove();
    fn getAttributeStringCollection();
    fn getMediaAtom();
    fn setDeleted();
    fn isDeleted();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMediaCollection2Impl: Sized + IWMPMediaCollectionImpl + IDispatchImpl {
    fn createQuery();
    fn getPlaylistByQuery();
    fn getStringCollectionByQuery();
    fn getByAttributeAndMediaType();
}
pub trait IWMPMediaPluginRegistrarImpl: Sized {
    fn WMPRegisterPlayerPlugin();
    fn WMPUnRegisterPlayerPlugin();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMetadataPictureImpl: Sized + IDispatchImpl {
    fn mimeType();
    fn pictureType();
    fn description();
    fn URL();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMetadataTextImpl: Sized + IDispatchImpl {
    fn description();
    fn text();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPNetworkImpl: Sized + IDispatchImpl {
    fn bandWidth();
    fn recoveredPackets();
    fn sourceProtocol();
    fn receivedPackets();
    fn lostPackets();
    fn receptionQuality();
    fn bufferingCount();
    fn bufferingProgress();
    fn bufferingTime();
    fn SetbufferingTime();
    fn frameRate();
    fn maxBitRate();
    fn bitRate();
    fn getProxySettings();
    fn setProxySettings();
    fn getProxyName();
    fn setProxyName();
    fn getProxyPort();
    fn setProxyPort();
    fn getProxyExceptionList();
    fn setProxyExceptionList();
    fn getProxyBypassForLocal();
    fn setProxyBypassForLocal();
    fn maxBandwidth();
    fn SetmaxBandwidth();
    fn downloadProgress();
    fn encodedFrameRate();
    fn framesSkipped();
}
pub trait IWMPNodeRealEstateImpl: Sized {
    fn GetDesiredSize();
    fn SetRects();
    fn GetRects();
    fn SetWindowless();
    fn GetWindowless();
    fn SetFullScreen();
    fn GetFullScreen();
}
pub trait IWMPNodeRealEstateHostImpl: Sized {
    fn OnDesiredSizeChange();
    fn OnFullScreenTransition();
}
pub trait IWMPNodeWindowedImpl: Sized {
    fn SetOwnerWindow();
    fn GetOwnerWindow();
}
pub trait IWMPNodeWindowedHostImpl: Sized {
    fn OnWindowMessageFromRenderer();
}
pub trait IWMPNodeWindowlessImpl: Sized + IWMPWindowMessageSinkImpl {
    fn OnDraw();
}
pub trait IWMPNodeWindowlessHostImpl: Sized {
    fn InvalidateRect();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayerImpl: Sized + IWMPCoreImpl + IDispatchImpl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer2Impl: Sized + IWMPCoreImpl + IDispatchImpl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
    fn stretchToFit();
    fn SetstretchToFit();
    fn windowlessVideo();
    fn SetwindowlessVideo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer3Impl: Sized + IWMPCore2Impl + IWMPCoreImpl + IDispatchImpl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
    fn stretchToFit();
    fn SetstretchToFit();
    fn windowlessVideo();
    fn SetwindowlessVideo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer4Impl: Sized + IWMPCore3Impl + IWMPCore2Impl + IWMPCoreImpl + IDispatchImpl {
    fn enabled();
    fn Setenabled();
    fn fullScreen();
    fn SetfullScreen();
    fn enableContextMenu();
    fn SetenableContextMenu();
    fn SetuiMode();
    fn uiMode();
    fn stretchToFit();
    fn SetstretchToFit();
    fn windowlessVideo();
    fn SetwindowlessVideo();
    fn isRemote();
    fn playerApplication();
    fn openPlayer();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayerApplicationImpl: Sized + IDispatchImpl {
    fn switchToPlayerApplication();
    fn switchToControl();
    fn playerDocked();
    fn hasDisplay();
}
pub trait IWMPPlayerServicesImpl: Sized {
    fn activateUIPlugin();
    fn setTaskPane();
    fn setTaskPaneURL();
}
pub trait IWMPPlayerServices2Impl: Sized + IWMPPlayerServicesImpl {
    fn setBackgroundProcessingPriority();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylistImpl: Sized + IDispatchImpl {
    fn count();
    fn name();
    fn Setname();
    fn attributeCount();
    fn attributeName();
    fn item();
    fn getItemInfo();
    fn setItemInfo();
    fn isIdentical();
    fn clear();
    fn insertItem();
    fn appendItem();
    fn removeItem();
    fn moveItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylistArrayImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylistCollectionImpl: Sized + IDispatchImpl {
    fn newPlaylist();
    fn getAll();
    fn getByName();
    fn remove();
    fn setDeleted();
    fn isDeleted();
    fn importPlaylist();
}
pub trait IWMPPluginImpl: Sized {
    fn Init();
    fn Shutdown();
    fn GetID();
    fn GetCaps();
    fn AdviseWMPServices();
    fn UnAdviseWMPServices();
}
pub trait IWMPPluginEnableImpl: Sized {
    fn SetEnable();
    fn GetEnable();
}
pub trait IWMPPluginUIImpl: Sized {
    fn SetCore();
    fn Create();
    fn Destroy();
    fn DisplayPropertyPage();
    fn GetProperty();
    fn SetProperty();
    fn TranslateAccelerator();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPQueryImpl: Sized + IDispatchImpl {
    fn addCondition();
    fn beginNextGroup();
}
pub trait IWMPRemoteMediaServicesImpl: Sized {
    fn GetServiceType();
    fn GetApplicationName();
    fn GetScriptableObject();
    fn GetCustomUIMode();
}
pub trait IWMPRenderConfigImpl: Sized {
    fn SetinProcOnly();
    fn inProcOnly();
}
pub trait IWMPServicesImpl: Sized {
    fn GetStreamTime();
    fn GetStreamState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSettingsImpl: Sized + IDispatchImpl {
    fn isAvailable();
    fn autoStart();
    fn SetautoStart();
    fn baseURL();
    fn SetbaseURL();
    fn defaultFrame();
    fn SetdefaultFrame();
    fn invokeURLs();
    fn SetinvokeURLs();
    fn mute();
    fn Setmute();
    fn playCount();
    fn SetplayCount();
    fn rate();
    fn Setrate();
    fn balance();
    fn Setbalance();
    fn volume();
    fn Setvolume();
    fn getMode();
    fn setMode();
    fn enableErrorDialogs();
    fn SetenableErrorDialogs();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSettings2Impl: Sized + IWMPSettingsImpl + IDispatchImpl {
    fn defaultAudioLanguage();
    fn mediaAccessRights();
    fn requestMediaAccessRights();
}
pub trait IWMPSkinManagerImpl: Sized {
    fn SetVisualStyle();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPStringCollectionImpl: Sized + IDispatchImpl {
    fn count();
    fn item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPStringCollection2Impl: Sized + IWMPStringCollectionImpl + IDispatchImpl {
    fn isIdentical();
    fn getItemInfo();
    fn getAttributeCountByType();
    fn getItemInfoByType();
}
pub trait IWMPSubscriptionServiceImpl: Sized {
    fn allowPlay();
    fn allowCDBurn();
    fn allowPDATransfer();
    fn startBackgroundProcessing();
}
pub trait IWMPSubscriptionService2Impl: Sized + IWMPSubscriptionServiceImpl {
    fn stopBackgroundProcessing();
    fn serviceEvent();
    fn deviceAvailable();
    fn prepareForSync();
}
pub trait IWMPSubscriptionServiceCallbackImpl: Sized {
    fn onComplete();
}
pub trait IWMPSyncDeviceImpl: Sized {
    fn friendlyName();
    fn SetfriendlyName();
    fn deviceName();
    fn deviceId();
    fn partnershipIndex();
    fn connected();
    fn status();
    fn syncState();
    fn progress();
    fn getItemInfo();
    fn createPartnership();
    fn deletePartnership();
    fn start();
    fn stop();
    fn showSettings();
    fn isIdentical();
}
pub trait IWMPSyncDevice2Impl: Sized + IWMPSyncDeviceImpl {
    fn setItemInfo();
}
pub trait IWMPSyncDevice3Impl: Sized + IWMPSyncDevice2Impl + IWMPSyncDeviceImpl {
    fn estimateSyncSize();
    fn cancelEstimation();
}
pub trait IWMPSyncServicesImpl: Sized {
    fn deviceCount();
    fn getDevice();
}
pub trait IWMPTranscodePolicyImpl: Sized {
    fn allowTranscode();
}
pub trait IWMPUserEventSinkImpl: Sized {
    fn NotifyUserEvent();
}
pub trait IWMPVideoRenderConfigImpl: Sized {
    fn SetpresenterActivate();
}
pub trait IWMPWindowMessageSinkImpl: Sized {
    fn OnWindowMessage();
}
pub trait IXFeedImpl: Sized {
    fn Xml();
    fn Name();
    fn Rename();
    fn Url();
    fn SetUrl();
    fn LocalId();
    fn Path();
    fn Move();
    fn Parent();
    fn LastWriteTime();
    fn Delete();
    fn Download();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn SyncSetting();
    fn SetSyncSetting();
    fn Interval();
    fn SetInterval();
    fn LastDownloadTime();
    fn LocalEnclosurePath();
    fn Items();
    fn GetItem();
    fn MarkAllItemsRead();
    fn MaxItemCount();
    fn SetMaxItemCount();
    fn DownloadEnclosuresAutomatically();
    fn SetDownloadEnclosuresAutomatically();
    fn DownloadStatus();
    fn LastDownloadError();
    fn Merge();
    fn DownloadUrl();
    fn Title();
    fn Description();
    fn Link();
    fn Image();
    fn LastBuildDate();
    fn PubDate();
    fn Ttl();
    fn Language();
    fn Copyright();
    fn IsList();
    fn GetWatcher();
    fn UnreadItemCount();
    fn ItemCount();
}
pub trait IXFeed2Impl: Sized + IXFeedImpl {
    fn GetItemByEffectiveId();
    fn LastItemDownloadTime();
    fn Username();
    fn Password();
    fn SetCredentials();
    fn ClearCredentials();
}
pub trait IXFeedEnclosureImpl: Sized {
    fn Url();
    fn Type();
    fn Length();
    fn AsyncDownload();
    fn CancelAsyncDownload();
    fn DownloadStatus();
    fn LastDownloadError();
    fn LocalPath();
    fn Parent();
    fn DownloadUrl();
    fn DownloadMimeType();
    fn RemoveFile();
    fn SetFile();
}
pub trait IXFeedEventsImpl: Sized {
    fn Error();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMoved();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
pub trait IXFeedFolderImpl: Sized {
    fn Feeds();
    fn Subfolders();
    fn CreateFeed();
    fn CreateSubfolder();
    fn ExistsFeed();
    fn ExistsSubfolder();
    fn GetFeed();
    fn GetSubfolder();
    fn Delete();
    fn Name();
    fn Rename();
    fn Path();
    fn Move();
    fn Parent();
    fn IsRoot();
    fn GetWatcher();
    fn TotalUnreadItemCount();
    fn TotalItemCount();
}
pub trait IXFeedFolderEventsImpl: Sized {
    fn Error();
    fn FolderAdded();
    fn FolderDeleted();
    fn FolderRenamed();
    fn FolderMovedFrom();
    fn FolderMovedTo();
    fn FolderItemCountChanged();
    fn FeedAdded();
    fn FeedDeleted();
    fn FeedRenamed();
    fn FeedUrlChanged();
    fn FeedMovedFrom();
    fn FeedMovedTo();
    fn FeedDownloading();
    fn FeedDownloadCompleted();
    fn FeedItemCountChanged();
}
pub trait IXFeedItemImpl: Sized {
    fn Xml();
    fn Title();
    fn Link();
    fn Guid();
    fn Description();
    fn PubDate();
    fn Comments();
    fn Author();
    fn Enclosure();
    fn IsRead();
    fn SetIsRead();
    fn LocalId();
    fn Parent();
    fn Delete();
    fn DownloadUrl();
    fn LastDownloadTime();
    fn Modified();
}
pub trait IXFeedItem2Impl: Sized + IXFeedItemImpl {
    fn EffectiveId();
}
pub trait IXFeedsEnumImpl: Sized {
    fn Count();
    fn Item();
}
pub trait IXFeedsManagerImpl: Sized {
    fn RootFolder();
    fn IsSubscribed();
    fn ExistsFeed();
    fn GetFeed();
    fn GetFeedByUrl();
    fn ExistsFolder();
    fn GetFolder();
    fn DeleteFeed();
    fn DeleteFolder();
    fn BackgroundSync();
    fn BackgroundSyncStatus();
    fn DefaultInterval();
    fn SetDefaultInterval();
    fn AsyncSyncAll();
    fn Normalize();
    fn ItemCountLimit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _WMPOCXEventsImpl: Sized + IDispatchImpl {}
