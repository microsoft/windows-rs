use super::*;
use crate::handler::subscription;

/// The state of a [`DownloadOperation`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownloadState {
    /// The download is in progress.
    InProgress,
    /// The download has stopped before completing, with a
    /// [reason](DownloadOperation::interrupt_reason). It may be resumable.
    Interrupted,
    /// The download completed successfully.
    Completed,
}

impl DownloadState {
    fn from_raw(value: COREWEBVIEW2_DOWNLOAD_STATE) -> Self {
        match value {
            1 => Self::Interrupted,
            2 => Self::Completed,
            _ => Self::InProgress,
        }
    }
}

/// Why a [`DownloadOperation`] was [interrupted](DownloadState::Interrupted).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DownloadInterruptReason {
    /// No interruption; the download is progressing or completed.
    None,
    FileFailed,
    FileAccessDenied,
    FileNoSpace,
    FileNameTooLong,
    FileTooLarge,
    FileMalicious,
    FileTransientError,
    FileBlockedByPolicy,
    FileSecurityCheckFailed,
    FileTooShort,
    FileHashMismatch,
    NetworkFailed,
    NetworkTimeout,
    NetworkDisconnected,
    NetworkServerDown,
    NetworkInvalidRequest,
    ServerFailed,
    ServerNoRange,
    ServerBadContent,
    ServerUnauthorized,
    ServerCertificateProblem,
    ServerForbidden,
    ServerUnexpectedResponse,
    ServerContentLengthMismatch,
    ServerCrossOriginRedirect,
    UserCanceled,
    UserShutdown,
    UserPaused,
    DownloadProcessCrashed,
    /// An interrupt reason not represented by the other variants.
    Unknown,
}

impl DownloadInterruptReason {
    fn from_raw(value: COREWEBVIEW2_DOWNLOAD_INTERRUPT_REASON) -> Self {
        match value {
            0 => Self::None,
            1 => Self::FileFailed,
            2 => Self::FileAccessDenied,
            3 => Self::FileNoSpace,
            4 => Self::FileNameTooLong,
            5 => Self::FileTooLarge,
            6 => Self::FileMalicious,
            7 => Self::FileTransientError,
            8 => Self::FileBlockedByPolicy,
            9 => Self::FileSecurityCheckFailed,
            10 => Self::FileTooShort,
            11 => Self::FileHashMismatch,
            12 => Self::NetworkFailed,
            13 => Self::NetworkTimeout,
            14 => Self::NetworkDisconnected,
            15 => Self::NetworkServerDown,
            16 => Self::NetworkInvalidRequest,
            17 => Self::ServerFailed,
            18 => Self::ServerNoRange,
            19 => Self::ServerBadContent,
            20 => Self::ServerUnauthorized,
            21 => Self::ServerCertificateProblem,
            22 => Self::ServerForbidden,
            23 => Self::ServerUnexpectedResponse,
            24 => Self::ServerContentLengthMismatch,
            25 => Self::ServerCrossOriginRedirect,
            26 => Self::UserCanceled,
            27 => Self::UserShutdown,
            28 => Self::UserPaused,
            29 => Self::DownloadProcessCrashed,
            _ => Self::Unknown,
        }
    }
}

/// An in-progress or finished download, obtained from a
/// [`DownloadStartingArgs`]. Query its [`state`](Self::state) and byte counts to
/// track progress, subscribe with [`on_bytes_received_changed`](Self::on_bytes_received_changed)
/// or [`on_state_changed`](Self::on_state_changed), and control it with
/// [`cancel`](Self::cancel), [`pause`](Self::pause), and [`resume`](Self::resume).
#[derive(Clone)]
pub struct DownloadOperation(pub(crate) ICoreWebView2DownloadOperation);

impl DownloadOperation {
    /// Returns the URI the content is being downloaded from.
    pub fn uri(&self) -> String {
        unsafe { string::take_result(self.0.Uri()) }
    }

    /// Returns the `Content-Disposition` header value from the download's HTTP
    /// response, if any.
    pub fn content_disposition(&self) -> String {
        unsafe { string::take_result(self.0.ContentDisposition()) }
    }

    /// Returns the MIME type of the downloaded content.
    pub fn mime_type(&self) -> String {
        unsafe { string::take_result(self.0.MimeType()) }
    }

    /// Returns the expected total size of the download in bytes, or `0` if it is
    /// unknown.
    pub fn total_bytes_to_receive(&self) -> i64 {
        unsafe { self.0.TotalBytesToReceive() }.unwrap_or(0)
    }

    /// Returns the number of bytes received so far.
    pub fn bytes_received(&self) -> i64 {
        unsafe { self.0.BytesReceived() }.unwrap_or(0)
    }

    /// Returns the absolute path the download is being written to.
    pub fn result_file_path(&self) -> String {
        unsafe { string::take_result(self.0.ResultFilePath()) }
    }

    /// Returns the current [`DownloadState`].
    pub fn state(&self) -> DownloadState {
        unsafe { self.0.State() }.map_or(DownloadState::InProgress, DownloadState::from_raw)
    }

    /// Returns why the download was interrupted, or
    /// [`DownloadInterruptReason::None`] if it has not been interrupted.
    pub fn interrupt_reason(&self) -> DownloadInterruptReason {
        unsafe { self.0.InterruptReason() }.map_or(
            DownloadInterruptReason::None,
            DownloadInterruptReason::from_raw,
        )
    }

    /// Returns `true` if a [interrupted](DownloadState::Interrupted) download can
    /// be [resumed](Self::resume).
    pub fn can_resume(&self) -> bool {
        unsafe { self.0.CanResume() }.is_ok_and(|value| value.as_bool())
    }

    /// Cancels the download. The file is deleted if it was not yet complete.
    pub fn cancel(&self) -> Result<()> {
        unsafe { self.0.Cancel() }.ok()
    }

    /// Pauses the download. It stays [in progress](DownloadState::InProgress)
    /// until [resumed](Self::resume) or [cancelled](Self::cancel).
    pub fn pause(&self) -> Result<()> {
        unsafe { self.0.Pause() }.ok()
    }

    /// Resumes a paused or [interrupted](DownloadState::Interrupted) download.
    pub fn resume(&self) -> Result<()> {
        unsafe { self.0.Resume() }.ok()
    }

    subscription! {
        /// Subscribes to the bytes-received-changed event, raised as the
        /// download makes progress. The handler receives this operation so it
        /// can read the updated [`bytes_received`](Self::bytes_received).
        on_bytes_received_changed(DownloadOperation) =>
            BytesReceivedChanged, add_BytesReceivedChanged / remove_BytesReceivedChanged
    }

    subscription! {
        /// Subscribes to the state-changed event, raised when the download's
        /// [`state`](Self::state) changes (for example to completed or
        /// interrupted). The handler receives this operation.
        on_state_changed(DownloadOperation) =>
            DownloadStateChanged, add_StateChanged / remove_StateChanged
    }
}

/// Details about a download that is about to start, delivered to a
/// [`WebView::on_download_starting`] handler. Inspect or control it via the
/// [`DownloadOperation`], change where it is saved with
/// [`set_result_file_path`](Self::set_result_file_path), or
/// [cancel](Self::set_cancel) it.
pub struct DownloadStartingArgs(pub(crate) ICoreWebView2DownloadStartingEventArgs);

impl DownloadStartingArgs {
    /// Returns the [`DownloadOperation`] for the download.
    pub fn download_operation(&self) -> Result<DownloadOperation> {
        unsafe { Ok(DownloadOperation(self.0.DownloadOperation()?)) }
    }

    /// Returns `true` if the download is currently marked to be cancelled.
    pub fn is_cancelled(&self) -> bool {
        unsafe { self.0.Cancel() }.is_ok_and(|value| value.as_bool())
    }

    /// Cancels (or un-cancels) the download.
    pub fn set_cancel(&self, cancel: bool) -> Result<()> {
        unsafe { self.0.SetCancel(cancel) }.ok()
    }

    /// Returns the absolute path the download will be written to.
    pub fn result_file_path(&self) -> String {
        unsafe { string::take_result(self.0.ResultFilePath()) }
    }

    /// Overrides the absolute path the download is written to, choosing a custom
    /// destination instead of the default.
    pub fn set_result_file_path(&self, path: &str) -> Result<()> {
        let path = HSTRING::from(path);
        unsafe { self.0.SetResultFilePath(&path) }.ok()
    }

    /// Returns `true` if the host has marked the download as handled,
    /// suppressing the default download dialog.
    pub fn is_handled(&self) -> bool {
        unsafe { self.0.Handled() }.is_ok_and(|value| value.as_bool())
    }

    /// Marks the download as handled, suppressing the default download dialog so
    /// the host can present its own UI.
    pub fn set_handled(&self, handled: bool) -> Result<()> {
        unsafe { self.0.SetHandled(handled) }.ok()
    }

    /// Takes a [`Deferral`] so the download can be resolved after the handler
    /// returns, for example once the user has chosen a destination.
    pub fn defer(&self) -> Result<Deferral> {
        Ok(Deferral::new(unsafe { self.0.GetDeferral()? }))
    }
}
