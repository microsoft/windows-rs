#[cfg(feature = "Win32_System_Com")]
pub trait DICounterItemImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DILogFileItemImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DISystemMonitorImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DISystemMonitorEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DISystemMonitorInternalImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IAlertDataCollectorImpl: Sized + IDataCollectorImpl + IDispatchImpl {
    fn AlertThresholds();
    fn SetAlertThresholds();
    fn EventLog();
    fn SetEventLog();
    fn SampleInterval();
    fn SetSampleInterval();
    fn Task();
    fn SetTask();
    fn TaskRunAsSelf();
    fn SetTaskRunAsSelf();
    fn TaskArguments();
    fn SetTaskArguments();
    fn TaskUserTextArguments();
    fn SetTaskUserTextArguments();
    fn TriggerDataCollectorSet();
    fn SetTriggerDataCollectorSet();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IApiTracingDataCollectorImpl: Sized + IDataCollectorImpl + IDispatchImpl {
    fn LogApiNamesOnly();
    fn SetLogApiNamesOnly();
    fn LogApisRecursively();
    fn SetLogApisRecursively();
    fn ExePath();
    fn SetExePath();
    fn LogFilePath();
    fn SetLogFilePath();
    fn IncludeModules();
    fn SetIncludeModules();
    fn IncludeApis();
    fn SetIncludeApis();
    fn ExcludeApis();
    fn SetExcludeApis();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConfigurationDataCollectorImpl: Sized + IDataCollectorImpl + IDispatchImpl {
    fn FileMaxCount();
    fn SetFileMaxCount();
    fn FileMaxRecursiveDepth();
    fn SetFileMaxRecursiveDepth();
    fn FileMaxTotalSize();
    fn SetFileMaxTotalSize();
    fn Files();
    fn SetFiles();
    fn ManagementQueries();
    fn SetManagementQueries();
    fn QueryNetworkAdapters();
    fn SetQueryNetworkAdapters();
    fn RegistryKeys();
    fn SetRegistryKeys();
    fn RegistryMaxRecursiveDepth();
    fn SetRegistryMaxRecursiveDepth();
    fn SystemStateFile();
    fn SetSystemStateFile();
}
pub trait ICounterItemImpl: Sized {
    fn Value();
    fn SetColor();
    fn Color();
    fn SetWidth();
    fn Width();
    fn SetLineStyle();
    fn LineStyle();
    fn SetScaleFactor();
    fn ScaleFactor();
    fn Path();
    fn GetValue();
    fn GetStatistics();
}
pub trait ICounterItem2Impl: Sized + ICounterItemImpl {
    fn SetSelected();
    fn Selected();
    fn SetVisible();
    fn Visible();
    fn GetDataAt();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICountersImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollectorImpl: Sized + IDispatchImpl {
    fn DataCollectorSet();
    fn SetDataCollectorSet();
    fn DataCollectorType();
    fn FileName();
    fn SetFileName();
    fn FileNameFormat();
    fn SetFileNameFormat();
    fn FileNameFormatPattern();
    fn SetFileNameFormatPattern();
    fn LatestOutputLocation();
    fn SetLatestOutputLocation();
    fn LogAppend();
    fn SetLogAppend();
    fn LogCircular();
    fn SetLogCircular();
    fn LogOverwrite();
    fn SetLogOverwrite();
    fn Name();
    fn SetName();
    fn OutputLocation();
    fn Index();
    fn SetIndex();
    fn Xml();
    fn SetXml();
    fn CreateOutputLocation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollectorCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
    fn CreateDataCollectorFromXml();
    fn CreateDataCollector();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollectorSetImpl: Sized + IDispatchImpl {
    fn DataCollectors();
    fn Duration();
    fn SetDuration();
    fn Description();
    fn SetDescription();
    fn DescriptionUnresolved();
    fn DisplayName();
    fn SetDisplayName();
    fn DisplayNameUnresolved();
    fn Keywords();
    fn SetKeywords();
    fn LatestOutputLocation();
    fn SetLatestOutputLocation();
    fn Name();
    fn OutputLocation();
    fn RootPath();
    fn SetRootPath();
    fn Segment();
    fn SetSegment();
    fn SegmentMaxDuration();
    fn SetSegmentMaxDuration();
    fn SegmentMaxSize();
    fn SetSegmentMaxSize();
    fn SerialNumber();
    fn SetSerialNumber();
    fn Server();
    fn Status();
    fn Subdirectory();
    fn SetSubdirectory();
    fn SubdirectoryFormat();
    fn SetSubdirectoryFormat();
    fn SubdirectoryFormatPattern();
    fn SetSubdirectoryFormatPattern();
    fn Task();
    fn SetTask();
    fn TaskRunAsSelf();
    fn SetTaskRunAsSelf();
    fn TaskArguments();
    fn SetTaskArguments();
    fn TaskUserTextArguments();
    fn SetTaskUserTextArguments();
    fn Schedules();
    fn SchedulesEnabled();
    fn SetSchedulesEnabled();
    fn UserAccount();
    fn Xml();
    fn Security();
    fn SetSecurity();
    fn StopOnCompletion();
    fn SetStopOnCompletion();
    fn DataManager();
    fn SetCredentials();
    fn Query();
    fn Commit();
    fn Delete();
    fn Start();
    fn Stop();
    fn SetXml();
    fn SetValue();
    fn GetValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataCollectorSetCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
    fn GetDataCollectorSets();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataManagerImpl: Sized + IDispatchImpl {
    fn Enabled();
    fn SetEnabled();
    fn CheckBeforeRunning();
    fn SetCheckBeforeRunning();
    fn MinFreeDisk();
    fn SetMinFreeDisk();
    fn MaxSize();
    fn SetMaxSize();
    fn MaxFolderCount();
    fn SetMaxFolderCount();
    fn ResourcePolicy();
    fn SetResourcePolicy();
    fn FolderActions();
    fn ReportSchema();
    fn SetReportSchema();
    fn ReportFileName();
    fn SetReportFileName();
    fn RuleTargetFileName();
    fn SetRuleTargetFileName();
    fn EventsFileName();
    fn SetEventsFileName();
    fn Rules();
    fn SetRules();
    fn Run();
    fn Extract();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFolderActionImpl: Sized + IDispatchImpl {
    fn Age();
    fn SetAge();
    fn Size();
    fn SetSize();
    fn Actions();
    fn SetActions();
    fn SendCabTo();
    fn SetSendCabTo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFolderActionCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
    fn CreateFolderAction();
}
pub trait ILogFileItemImpl: Sized {
    fn Path();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILogFilesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPerformanceCounterDataCollectorImpl: Sized + IDataCollectorImpl + IDispatchImpl {
    fn DataSourceName();
    fn SetDataSourceName();
    fn PerformanceCounters();
    fn SetPerformanceCounters();
    fn LogFileFormat();
    fn SetLogFileFormat();
    fn SampleInterval();
    fn SetSampleInterval();
    fn SegmentMaxRecords();
    fn SetSegmentMaxRecords();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IScheduleImpl: Sized + IDispatchImpl {
    fn StartDate();
    fn SetStartDate();
    fn EndDate();
    fn SetEndDate();
    fn StartTime();
    fn SetStartTime();
    fn Days();
    fn SetDays();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IScheduleCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
    fn CreateSchedule();
}
pub trait ISystemMonitorImpl: Sized {
    fn Appearance();
    fn SetAppearance();
    fn BackColor();
    fn SetBackColor();
    fn BorderStyle();
    fn SetBorderStyle();
    fn ForeColor();
    fn SetForeColor();
    fn Font();
    fn putref_Font();
    fn Counters();
    fn SetShowVerticalGrid();
    fn ShowVerticalGrid();
    fn SetShowHorizontalGrid();
    fn ShowHorizontalGrid();
    fn SetShowLegend();
    fn ShowLegend();
    fn SetShowScaleLabels();
    fn ShowScaleLabels();
    fn SetShowValueBar();
    fn ShowValueBar();
    fn SetMaximumScale();
    fn MaximumScale();
    fn SetMinimumScale();
    fn MinimumScale();
    fn SetUpdateInterval();
    fn UpdateInterval();
    fn SetDisplayType();
    fn DisplayType();
    fn SetManualUpdate();
    fn ManualUpdate();
    fn SetGraphTitle();
    fn GraphTitle();
    fn SetYAxisLabel();
    fn YAxisLabel();
    fn CollectSample();
    fn UpdateGraph();
    fn BrowseCounters();
    fn DisplayProperties();
    fn Counter();
    fn AddCounter();
    fn DeleteCounter();
    fn BackColorCtl();
    fn SetBackColorCtl();
    fn SetLogFileName();
    fn LogFileName();
    fn SetLogViewStart();
    fn LogViewStart();
    fn SetLogViewStop();
    fn LogViewStop();
    fn GridColor();
    fn SetGridColor();
    fn TimeBarColor();
    fn SetTimeBarColor();
    fn Highlight();
    fn SetHighlight();
    fn ShowToolbar();
    fn SetShowToolbar();
    fn Paste();
    fn Copy();
    fn Reset();
    fn SetReadOnly();
    fn ReadOnly();
    fn SetReportValueType();
    fn ReportValueType();
    fn SetMonitorDuplicateInstances();
    fn MonitorDuplicateInstances();
    fn SetDisplayFilter();
    fn DisplayFilter();
    fn LogFiles();
    fn SetDataSourceType();
    fn DataSourceType();
    fn SetSqlDsnName();
    fn SqlDsnName();
    fn SetSqlLogSetName();
    fn SqlLogSetName();
}
pub trait ISystemMonitor2Impl: Sized + ISystemMonitorImpl {
    fn SetEnableDigitGrouping();
    fn EnableDigitGrouping();
    fn SetEnableToolTips();
    fn EnableToolTips();
    fn SetShowTimeAxisLabels();
    fn ShowTimeAxisLabels();
    fn SetChartScroll();
    fn ChartScroll();
    fn SetDataPointCount();
    fn DataPointCount();
    fn ScaleToFit();
    fn SaveAs();
    fn Relog();
    fn ClearData();
    fn LogSourceStartTime();
    fn LogSourceStopTime();
    fn SetLogViewRange();
    fn GetLogViewRange();
    fn BatchingLock();
    fn LoadSettings();
}
pub trait ISystemMonitorEventsImpl: Sized {
    fn OnCounterSelected();
    fn OnCounterAdded();
    fn OnCounterDeleted();
    fn OnSampleCollected();
    fn OnDblClick();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITraceDataCollectorImpl: Sized + IDataCollectorImpl + IDispatchImpl {
    fn BufferSize();
    fn SetBufferSize();
    fn BuffersLost();
    fn SetBuffersLost();
    fn BuffersWritten();
    fn SetBuffersWritten();
    fn ClockType();
    fn SetClockType();
    fn EventsLost();
    fn SetEventsLost();
    fn ExtendedModes();
    fn SetExtendedModes();
    fn FlushTimer();
    fn SetFlushTimer();
    fn FreeBuffers();
    fn SetFreeBuffers();
    fn Guid();
    fn SetGuid();
    fn IsKernelTrace();
    fn MaximumBuffers();
    fn SetMaximumBuffers();
    fn MinimumBuffers();
    fn SetMinimumBuffers();
    fn NumberOfBuffers();
    fn SetNumberOfBuffers();
    fn PreallocateFile();
    fn SetPreallocateFile();
    fn ProcessMode();
    fn SetProcessMode();
    fn RealTimeBuffersLost();
    fn SetRealTimeBuffersLost();
    fn SessionId();
    fn SetSessionId();
    fn SessionName();
    fn SetSessionName();
    fn SessionThreadId();
    fn SetSessionThreadId();
    fn StreamMode();
    fn SetStreamMode();
    fn TraceDataProviders();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITraceDataProviderImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn SetDisplayName();
    fn Guid();
    fn SetGuid();
    fn Level();
    fn KeywordsAny();
    fn KeywordsAll();
    fn Properties();
    fn FilterEnabled();
    fn SetFilterEnabled();
    fn FilterType();
    fn SetFilterType();
    fn FilterData();
    fn SetFilterData();
    fn Query();
    fn Resolve();
    fn SetSecurity();
    fn GetSecurity();
    fn GetRegisteredProcesses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITraceDataProviderCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
    fn CreateTraceDataProvider();
    fn GetTraceDataProviders();
    fn GetTraceDataProvidersByProcess();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IValueMapImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Description();
    fn SetDescription();
    fn Value();
    fn SetValue();
    fn ValueMapType();
    fn SetValueMapType();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
    fn CreateValueMapItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IValueMapItemImpl: Sized + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn Enabled();
    fn SetEnabled();
    fn Key();
    fn SetKey();
    fn Value();
    fn SetValue();
    fn ValueMapType();
    fn SetValueMapType();
}
pub trait _ICounterItemUnionImpl: Sized {
    fn Value();
    fn SetColor();
    fn Color();
    fn SetWidth();
    fn Width();
    fn SetLineStyle();
    fn LineStyle();
    fn SetScaleFactor();
    fn ScaleFactor();
    fn Path();
    fn GetValue();
    fn GetStatistics();
    fn SetSelected();
    fn Selected();
    fn SetVisible();
    fn Visible();
    fn GetDataAt();
}
pub trait _ISystemMonitorUnionImpl: Sized {
    fn Appearance();
    fn SetAppearance();
    fn BackColor();
    fn SetBackColor();
    fn BorderStyle();
    fn SetBorderStyle();
    fn ForeColor();
    fn SetForeColor();
    fn Font();
    fn putref_Font();
    fn Counters();
    fn SetShowVerticalGrid();
    fn ShowVerticalGrid();
    fn SetShowHorizontalGrid();
    fn ShowHorizontalGrid();
    fn SetShowLegend();
    fn ShowLegend();
    fn SetShowScaleLabels();
    fn ShowScaleLabels();
    fn SetShowValueBar();
    fn ShowValueBar();
    fn SetMaximumScale();
    fn MaximumScale();
    fn SetMinimumScale();
    fn MinimumScale();
    fn SetUpdateInterval();
    fn UpdateInterval();
    fn SetDisplayType();
    fn DisplayType();
    fn SetManualUpdate();
    fn ManualUpdate();
    fn SetGraphTitle();
    fn GraphTitle();
    fn SetYAxisLabel();
    fn YAxisLabel();
    fn CollectSample();
    fn UpdateGraph();
    fn BrowseCounters();
    fn DisplayProperties();
    fn Counter();
    fn AddCounter();
    fn DeleteCounter();
    fn BackColorCtl();
    fn SetBackColorCtl();
    fn SetLogFileName();
    fn LogFileName();
    fn SetLogViewStart();
    fn LogViewStart();
    fn SetLogViewStop();
    fn LogViewStop();
    fn GridColor();
    fn SetGridColor();
    fn TimeBarColor();
    fn SetTimeBarColor();
    fn Highlight();
    fn SetHighlight();
    fn ShowToolbar();
    fn SetShowToolbar();
    fn Paste();
    fn Copy();
    fn Reset();
    fn SetReadOnly();
    fn ReadOnly();
    fn SetReportValueType();
    fn ReportValueType();
    fn SetMonitorDuplicateInstances();
    fn MonitorDuplicateInstances();
    fn SetDisplayFilter();
    fn DisplayFilter();
    fn LogFiles();
    fn SetDataSourceType();
    fn DataSourceType();
    fn SetSqlDsnName();
    fn SqlDsnName();
    fn SetSqlLogSetName();
    fn SqlLogSetName();
    fn SetEnableDigitGrouping();
    fn EnableDigitGrouping();
    fn SetEnableToolTips();
    fn EnableToolTips();
    fn SetShowTimeAxisLabels();
    fn ShowTimeAxisLabels();
    fn SetChartScroll();
    fn ChartScroll();
    fn SetDataPointCount();
    fn DataPointCount();
    fn ScaleToFit();
    fn SaveAs();
    fn Relog();
    fn ClearData();
    fn LogSourceStartTime();
    fn LogSourceStopTime();
    fn SetLogViewRange();
    fn GetLogViewRange();
    fn BatchingLock();
    fn LoadSettings();
}
