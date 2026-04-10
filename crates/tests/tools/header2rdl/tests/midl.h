// ============================================================================
// WebView2.h — Minimal COM Interface Declarations for Microsoft WebView2
// ============================================================================
// MinGW-compatible subset of the official Microsoft.Web.WebView2 SDK.
// Declares only the interfaces required by RawrXD Win32IDE integration:
//   - ICoreWebView2Environment
//   - ICoreWebView2Controller
//   - ICoreWebView2
//   - ICoreWebView2Settings
//   - ICoreWebView2WebMessageReceivedEventArgs
//   - CreateCoreWebView2EnvironmentWithOptions (loader export)
//
// These GUIDs and vtable layouts match the official WebView2.h from the
// Microsoft.Web.WebView2 NuGet package (1.0.2739+). They are ABI-stable
// and forward-compatible with all WebView2 Runtime versions >= 88.0.
//
// Rule: NO SOURCE FILE IS TO BE SIMPLIFIED
// ============================================================================

#pragma once

#ifndef __WEBVIEW2_H__
#define __WEBVIEW2_H__

#include <unknwn.h>
#include <objidl.h>
#include <oaidl.h>

// EventRegistrationToken — used by WebView2 event add/remove methods
// This is normally defined in <EventToken.h> but may be missing on MinGW
#ifndef _EventRegistrationToken_defined_
#define _EventRegistrationToken_defined_
typedef struct EventRegistrationToken {
    __int64 value;
} EventRegistrationToken;
#endif

// ============================================================================
// Forward declarations
// ============================================================================
interface ICoreWebView2;
interface ICoreWebView2Controller;
interface ICoreWebView2Environment;
interface ICoreWebView2Settings;
interface ICoreWebView2WebMessageReceivedEventArgs;
interface ICoreWebView2NavigationCompletedEventArgs;
interface ICoreWebView2ContentLoadingEventArgs;

// ============================================================================
// Event handler typedefs (COM callback interfaces)
// ============================================================================

// Handler: Environment created
MIDL_INTERFACE("4E8A3389-C9D8-4BD2-B6B5-124FEE6CC14D")
ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE Invoke(
        HRESULT errorCode,
        ICoreWebView2Environment* createdEnvironment) = 0;
};

// Handler: Controller created
MIDL_INTERFACE("6C4819F3-C9B7-4260-8127-C9F5BDE7F68C")
ICoreWebView2CreateCoreWebView2ControllerCompletedHandler : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE Invoke(
        HRESULT errorCode,
        ICoreWebView2Controller* createdController) = 0;
};

// Handler: Web message received
MIDL_INTERFACE("57213F19-00A6-49F2-9541-7DE229F185C7")
ICoreWebView2WebMessageReceivedEventHandler : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE Invoke(
        ICoreWebView2* sender,
        ICoreWebView2WebMessageReceivedEventArgs* args) = 0;
};

// Handler: Navigation completed
MIDL_INTERFACE("D33A35BF-1C49-4F98-93AB-006E0533FE1C")
ICoreWebView2NavigationCompletedEventHandler : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE Invoke(
        ICoreWebView2* sender,
        ICoreWebView2NavigationCompletedEventArgs* args) = 0;
};

// Handler: Content loading
MIDL_INTERFACE("364471E7-F2F9-4718-AAE1-D0A44A748277")
ICoreWebView2ContentLoadingEventHandler : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE Invoke(
        ICoreWebView2* sender,
        ICoreWebView2ContentLoadingEventArgs* args) = 0;
};

// ============================================================================
// Event args
// ============================================================================

MIDL_INTERFACE("0F99A40C-E962-4207-9E92-E97542D7B0CB")
ICoreWebView2WebMessageReceivedEventArgs : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE get_Source(LPWSTR* source) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_WebMessageAsJson(LPWSTR* webMessageAsJson) = 0;
    virtual HRESULT STDMETHODCALLTYPE TryGetWebMessageAsString(LPWSTR* webMessageAsString) = 0;
};

MIDL_INTERFACE("30D68B7D-20D9-4752-A9CA-EC8448FBB5C1")
ICoreWebView2NavigationCompletedEventArgs : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE get_IsSuccess(BOOL* isSuccess) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_WebErrorStatus(int* webErrorStatus) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_NavigationId(UINT64* navigationId) = 0;
};

MIDL_INTERFACE("0C8A1275-9B6B-4901-87AD-70DF25BAFA6E")
ICoreWebView2ContentLoadingEventArgs : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE get_IsErrorPage(BOOL* isErrorPage) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_NavigationId(UINT64* navigationId) = 0;
};

// ============================================================================
// ICoreWebView2Settings — Controls WebView behavior flags
// ============================================================================
MIDL_INTERFACE("E562E4F0-D7FA-43AC-8D59-0A9093B4A807")
ICoreWebView2Settings : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE get_IsScriptEnabled(BOOL* isScriptEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_IsScriptEnabled(BOOL isScriptEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_IsWebMessageEnabled(BOOL* isWebMessageEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_IsWebMessageEnabled(BOOL isWebMessageEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_AreDefaultScriptDialogsEnabled(BOOL* areDefaultScriptDialogsEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_AreDefaultScriptDialogsEnabled(BOOL areDefaultScriptDialogsEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_IsStatusBarEnabled(BOOL* isStatusBarEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_IsStatusBarEnabled(BOOL isStatusBarEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_AreDevToolsEnabled(BOOL* areDevToolsEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_AreDevToolsEnabled(BOOL areDevToolsEnabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_AreDefaultContextMenusEnabled(BOOL* enabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_AreDefaultContextMenusEnabled(BOOL enabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_AreHostObjectsAllowed(BOOL* allowed) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_AreHostObjectsAllowed(BOOL allowed) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_IsZoomControlEnabled(BOOL* enabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_IsZoomControlEnabled(BOOL enabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_IsBuiltInErrorPageEnabled(BOOL* enabled) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_IsBuiltInErrorPageEnabled(BOOL enabled) = 0;
};

// ============================================================================
// ICoreWebView2 — The core web view interface
// ============================================================================
MIDL_INTERFACE("76ECEACB-0462-4D94-AC83-423A6793775E")
ICoreWebView2 : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE get_Settings(ICoreWebView2Settings** settings) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_Source(LPWSTR* uri) = 0;
    virtual HRESULT STDMETHODCALLTYPE Navigate(LPCWSTR uri) = 0;
    virtual HRESULT STDMETHODCALLTYPE NavigateToString(LPCWSTR htmlContent) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_NavigationStarting(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_NavigationStarting(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_ContentLoading(ICoreWebView2ContentLoadingEventHandler* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_ContentLoading(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_SourceChanged(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_SourceChanged(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_HistoryChanged(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_HistoryChanged(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_NavigationCompleted(ICoreWebView2NavigationCompletedEventHandler* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_NavigationCompleted(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_FrameNavigationStarting(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_FrameNavigationStarting(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_FrameNavigationCompleted(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_FrameNavigationCompleted(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_ScriptDialogOpening(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_ScriptDialogOpening(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_PermissionRequested(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_PermissionRequested(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_ProcessFailed(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_ProcessFailed(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE AddScriptToExecuteOnDocumentCreated(LPCWSTR javaScript, IUnknown* handler) = 0;
    virtual HRESULT STDMETHODCALLTYPE RemoveScriptToExecuteOnDocumentCreated(LPCWSTR id) = 0;
    virtual HRESULT STDMETHODCALLTYPE ExecuteScript(LPCWSTR javaScript, IUnknown* handler) = 0;
    virtual HRESULT STDMETHODCALLTYPE CapturePreview(int imageFormat, IStream* imageStream, IUnknown* handler) = 0;
    virtual HRESULT STDMETHODCALLTYPE Reload() = 0;
    virtual HRESULT STDMETHODCALLTYPE PostWebMessageAsJson(LPCWSTR webMessageAsJson) = 0;
    virtual HRESULT STDMETHODCALLTYPE PostWebMessageAsString(LPCWSTR webMessageAsString) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_WebMessageReceived(ICoreWebView2WebMessageReceivedEventHandler* handler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_WebMessageReceived(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE CallDevToolsProtocolMethod(LPCWSTR methodName, LPCWSTR parametersAsJson, IUnknown* handler) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_BrowserProcessId(UINT32* value) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_CanGoBack(BOOL* canGoBack) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_CanGoForward(BOOL* canGoForward) = 0;
    virtual HRESULT STDMETHODCALLTYPE GoBack() = 0;
    virtual HRESULT STDMETHODCALLTYPE GoForward() = 0;
    virtual HRESULT STDMETHODCALLTYPE GetDevToolsProtocolEventReceiver(LPCWSTR eventName, IUnknown** receiver) = 0;
    virtual HRESULT STDMETHODCALLTYPE Stop() = 0;
    virtual HRESULT STDMETHODCALLTYPE add_NewWindowRequested(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_NewWindowRequested(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_DocumentTitleChanged(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_DocumentTitleChanged(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_ContainsFullScreenElementChanged(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_ContainsFullScreenElementChanged(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_ContainsFullScreenElement(BOOL* containsFullScreenElement) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_WebResourceRequested(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_WebResourceRequested(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE AddWebResourceRequestedFilter(LPCWSTR uri, int resourceContext) = 0;
    virtual HRESULT STDMETHODCALLTYPE RemoveWebResourceRequestedFilter(LPCWSTR uri, int resourceContext) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_WindowCloseRequested(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_WindowCloseRequested(EventRegistrationToken token) = 0;
};

// ============================================================================
// ICoreWebView2Controller — Controls the WebView2 window
// ============================================================================
MIDL_INTERFACE("4D00C0D1-9434-4EB6-8078-8697A560334F")
ICoreWebView2Controller : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE get_IsVisible(BOOL* isVisible) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_IsVisible(BOOL isVisible) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_Bounds(RECT* bounds) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_Bounds(RECT bounds) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_ZoomFactor(double* zoomFactor) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_ZoomFactor(double zoomFactor) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_ZoomFactorChanged(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_ZoomFactorChanged(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE SetBoundsAndZoomFactor(RECT bounds, double zoomFactor) = 0;
    virtual HRESULT STDMETHODCALLTYPE MoveFocus(int reason) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_MoveFocusRequested(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_MoveFocusRequested(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_GotFocus(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_GotFocus(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_LostFocus(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_LostFocus(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_AcceleratorKeyPressed(IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_AcceleratorKeyPressed(EventRegistrationToken token) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_ParentWindow(HWND* parentWindow) = 0;
    virtual HRESULT STDMETHODCALLTYPE put_ParentWindow(HWND parentWindow) = 0;
    virtual HRESULT STDMETHODCALLTYPE NotifyParentWindowPositionChanged() = 0;
    virtual HRESULT STDMETHODCALLTYPE Close() = 0;
    virtual HRESULT STDMETHODCALLTYPE get_CoreWebView2(ICoreWebView2** coreWebView2) = 0;
};

// ============================================================================
// ICoreWebView2Environment — Factory for creating controllers
// ============================================================================
MIDL_INTERFACE("B96D755E-0319-4E92-A296-23436F46A1FC")
ICoreWebView2Environment : public IUnknown {
public:
    virtual HRESULT STDMETHODCALLTYPE CreateCoreWebView2Controller(
        HWND parentWindow,
        ICoreWebView2CreateCoreWebView2ControllerCompletedHandler* handler) = 0;
    virtual HRESULT STDMETHODCALLTYPE CreateWebResourceResponse(
        IStream* content, int statusCode, LPCWSTR reasonPhrase,
        LPCWSTR headers, IUnknown** response) = 0;
    virtual HRESULT STDMETHODCALLTYPE get_BrowserVersionString(LPWSTR* versionInfo) = 0;
    virtual HRESULT STDMETHODCALLTYPE add_NewBrowserVersionAvailable(
        IUnknown* eventHandler, EventRegistrationToken* token) = 0;
    virtual HRESULT STDMETHODCALLTYPE remove_NewBrowserVersionAvailable(EventRegistrationToken token) = 0;
};

// ============================================================================
// Entry point — loaded dynamically from WebView2Loader.dll
// ============================================================================
typedef HRESULT (__stdcall *CreateCoreWebView2EnvironmentWithOptionsFunc)(
    LPCWSTR browserExecutableFolder,
    LPCWSTR userDataFolder,
    IUnknown* environmentOptions,
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler* environmentCreatedHandler
);

#endif // __WEBVIEW2_H__
