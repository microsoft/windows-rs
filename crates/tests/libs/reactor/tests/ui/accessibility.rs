use super::*;
use std::sync::{Arc, Mutex};
use windows_core::{BSTR, Interface, Ref, implement};

const LIVE_REGION_CHANGED: i32 = 20024;
const POLITE: i32 = 1;
const ASSERTIVE: i32 = 2;
const ACTION_COMPLETED: i32 = 2;
const IMPORTANT_MOST_RECENT: i32 = 1;

type Log<T> = Arc<Mutex<Vec<T>>>;

#[implement(IUIAutomationEventHandler)]
struct LiveRegions(Log<String>);

impl IUIAutomationEventHandler_Impl for LiveRegions_Impl {
    fn HandleAutomationEvent(
        &self,
        sender: Ref<IUIAutomationElement>,
        _: EVENTID,
    ) -> windows_core::Result<()> {
        let id = unsafe { sender.ok()?.CurrentAutomationId()? };
        self.0.lock().unwrap().push(String::from_utf16_lossy(&id));
        Ok(())
    }
}

#[implement(IUIAutomationNotificationEventHandler)]
struct Notifications(Log<(i32, i32, String, String)>);

impl IUIAutomationNotificationEventHandler_Impl for Notifications_Impl {
    fn HandleNotificationEvent(
        &self,
        _: Ref<IUIAutomationElement>,
        kind: NotificationKind,
        processing: NotificationProcessing,
        text: &BSTR,
        activity: &BSTR,
    ) -> windows_core::Result<()> {
        self.0.lock().unwrap().push((
            kind,
            processing,
            String::from_utf16_lossy(text),
            String::from_utf16_lossy(activity),
        ));
        Ok(())
    }
}

#[test]
fn automation_properties_live_regions_and_notifications() -> Result<()> {
    process::run("accessibility", |ui| {
        let notify = ui.element("notify")?;
        unsafe {
            assert_eq!(notify.CurrentHelpText()?, "Announces the result");
            assert_eq!(
                notify
                    .cast::<IUIAutomationElement6>()?
                    .CurrentFullDescription()?,
                "Raises a UI Automation notification"
            );
            let live = |id| -> Result<i32> {
                Ok(ui
                    .element(id)?
                    .cast::<IUIAutomationElement2>()?
                    .CurrentLiveSetting()?)
            };
            assert_eq!(live("status")?, POLITE);
            assert_eq!(live("region")?, ASSERTIVE);
            assert_eq!(live("quiet")?, 0);
        }

        // Notification events require the CUIAutomation8 client.
        let client: IUIAutomation5 = unsafe {
            let mut raw = std::ptr::null_mut();
            CoCreateInstance(
                &CUIAutomation8,
                std::ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &IUIAutomation5::IID,
                &mut raw,
            )
            .ok()?;
            IUIAutomation5::from_raw(raw)
        };
        let window = unsafe { client.ElementFromHandle(ui.window_handle()? as _)? };
        let regions = Log::default();
        let notifications = Log::default();
        unsafe {
            client
                .AddAutomationEventHandler(
                    LIVE_REGION_CHANGED,
                    &window,
                    TreeScope_Subtree,
                    None,
                    &IUIAutomationEventHandler::from(LiveRegions(regions.clone())),
                )
                .ok()?;
            client
                .AddNotificationEventHandler(
                    &window,
                    TreeScope_Subtree,
                    None,
                    &IUIAutomationNotificationEventHandler::from(Notifications(
                        notifications.clone(),
                    )),
                )
                .ok()?;
        }

        let result = (|| {
            // Each negative case is followed by a positive one; UIA delivers the events in
            // order, so a stray event would appear before the expected one.
            ui.invoke("change-quiet")?;
            ui.name("quiet", "Quiet 1")?;
            ui.invoke("change-status")?;
            ui.name("status", "Status 1")?;
            assert_eq!(live_regions(ui, &regions, 1)?, ["status"]);

            ui.invoke("change-region")?;
            ui.name("region", "Region 1")?;
            assert_eq!(live_regions(ui, &regions, 2)?, ["status", "region"]);

            ui.invoke("silence")?;
            ui.name("added", "Added")?;
            ui.invoke("change-status")?;
            ui.name("status", "Status 2")?;
            ui.invoke("change-region")?;
            ui.name("region", "Region 2")?;
            assert_eq!(
                live_regions(ui, &regions, 3)?,
                ["status", "region", "region"]
            );

            ui.invoke("notify")?;
            ui.name("result", "Notified Some(Ok(())); panel None")?;
            let raised = ui.wait(
                "notify raises a UIA notification",
                || Ok(notifications.lock().unwrap().clone()),
                |raised| !raised.is_empty(),
            )?;
            assert_eq!(
                raised,
                [(
                    ACTION_COMPLETED,
                    IMPORTANT_MOST_RECENT,
                    "Marked done".to_string(),
                    "fixture".to_string()
                )]
            );

            ui.invoke("notify-panel")?;
            ui.name(
                "result",
                "Notified Some(Ok(())); panel Some(Err(Native(-2147467259)))",
            )
        })();
        unsafe { client.RemoveAllEventHandlers().ok()? };
        result
    })
}

fn live_regions(ui: &Automation, log: &Log<String>, count: usize) -> Result<Vec<String>> {
    ui.wait(
        &format!("{count} LiveRegionChanged events"),
        || Ok(log.lock().unwrap().clone()),
        |raised| raised.len() >= count,
    )
}
