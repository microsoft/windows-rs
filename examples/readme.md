A few sample projects to help you get started with the `windows` crate.

## clock

[clock](clock) - An example of using [Direct2D](https://docs.microsoft.com/windows/win32/direct2d/direct2d-overview) and various other Windows APIs.

<img src=https://user-images.githubusercontent.com/9845234/106089513-ef5f3d80-60dc-11eb-9aee-f89d2416f341.gif width="75%">

## com_uri

[com_uri](com_uri) - A simple example of using the [CreateUri](https://docs.microsoft.com/previous-versions/windows/internet-explorer/ie-developer/platform-apis/ms775098(v=vs.85)) function.

## enum_windows

[enum_windows](enum_windows) - An example of using the [EnumWindows](https://docs.microsoft.com/windows/win32/api/winuser/nf-winuser-enumwindows) function.

## event

[event](event) - An example using the kernel [event object](https://docs.microsoft.com/windows/win32/api/synchapi/nf-synchapi-createeventa).

## message_box

[message_box](message_box) - An example using the age-old [MessageBox](https://docs.microsoft.com/windows/win32/api/winuser/nf-winuser-messagebox) function.

## ocr

[ocr](ocr) - An example using the [Windows.Graphics.Imaging](https://docs.microsoft.com/uwp/api/Windows.Graphics.Imaging), [Windows.Media.Ocr](https://docs.microsoft.com/en-us/uwp/api/Windows.Media.Ocr), and [Windows.Storage](https://docs.microsoft.com/uwp/api/Windows.Storage) APIs to convert an image into text.

## overlapped

[overlapped](overlapped) - An example using various Win32 APIs to create and write to a new file.

## rss

[rss](rss) - An example using the [Windows.Foundation.Uri](https://docs.microsoft.com/uwp/api/Windows.Graphics.Imaging) and [Windows.Web.Syndication](https://docs.microsoft.com/uwp/api/Windows.Web.Syndication) APIs.

## simple

[simple](simple) - An example of including bindings without a separate bindings crate when you only need to use a very small number of Windows APIs.

## win2d

[win2d](win2d) - An example using dependencies like [Win2D](https://www.nuget.org/packages/Win2D.uwp) where that component provides its own winmd for describing its API surface as well as a runtime DLL that must be deployed with the app. It is further complicated as Win2D requires the [VCRTForwarders](https://www.nuget.org/packages/Microsoft.VCRTForwarders.140/) in order to load. 

## xml

[xml](xml) - An example of using the [Windows.Data.Xml.Dom](https://docs.microsoft.com/uwp/api/Windows.Data.Xml.Dom) API for parsing XML.
