/// Live-region activity collected while applying one mutation batch.
#[derive(Default)]
struct LiveRegionBatch {
    text_changed: Vec<ObjectId>,
    became_live: Vec<ObjectId>,
}

impl WinUiAdapter {
    /// Tracks live settings and text changes for one `SetProperties` mutation.
    fn record_live_regions(
        &mut self,
        batch: &mut LiveRegionBatch,
        object: ObjectId,
        set: &[Property],
        clear: &[PropertyId],
    ) {
        if clear.contains(&PropertyId::AutomationLiveSetting) {
            self.live_regions.remove(&object);
        }
        if let Some(property) = set
            .iter()
            .find(|property| property.id == PropertyId::AutomationLiveSetting)
        {
            if property.value == AutomationLiveSetting::Off.property_value() {
                self.live_regions.remove(&object);
            } else if self.live_regions.insert(object) {
                batch.became_live.push(object);
            }
        }
        // Text recorded before a region exists can only belong to a region that becomes live in
        // this batch, and those do not announce.
        if !self.live_regions.is_empty()
            && set
                .iter()
                .map(|property| property.id)
                .chain(clear.iter().copied())
                .any(|id| matches!(id, PropertyId::Text | PropertyId::AutomationName))
        {
            batch.text_changed.push(object);
        }
    }

    /// Raises `LiveRegionChanged` once for each live region whose text, or text in its content,
    /// changed in the batch. Regions that became live in the batch do not announce.
    fn raise_live_regions(&self, batch: LiveRegionBatch) -> Result<(), WinUiError> {
        if batch.text_changed.is_empty() {
            return Ok(());
        }
        let mut visited = HashSet::new();
        let mut regions = Vec::new();
        for object in batch.text_changed {
            let mut current = Some(object);
            while let Some(object) = current {
                if !visited.insert(object) {
                    break;
                }
                if self.live_regions.contains(&object) && !batch.became_live.contains(&object) {
                    regions.push(object);
                }
                current = self.owners.get(&object).map(|(parent, _)| *parent);
            }
        }
        if regions.is_empty()
            || !native::AutomationPeer::ListenerExists(native::AutomationEvents::LiveRegionChanged)?
        {
            return Ok(());
        }
        for region in regions {
            if let Some(peer) = automation_peer(&self.ui_element(region)?)? {
                peer.RaiseAutomationEvent(native::AutomationEvents::LiveRegionChanged)?;
            }
        }
        Ok(())
    }

    fn notify(
        &self,
        object: ObjectId,
        notification: &AutomationNotification,
    ) -> Result<(), WinUiError> {
        use AutomationNotificationKind as Kind;
        use AutomationNotificationProcessing as Processing;
        use native::AutomationNotificationKind as NativeKind;
        use native::AutomationNotificationProcessing as NativeProcessing;

        let Some(peer) = automation_peer(&self.ui_element(object)?)? else {
            return Err(
                windows_core::Error::new(native::E_FAIL, "element has no automation peer").into(),
            );
        };
        let kind = match notification.kind {
            Kind::ItemAdded => NativeKind::ItemAdded,
            Kind::ItemRemoved => NativeKind::ItemRemoved,
            Kind::ActionCompleted => NativeKind::ActionCompleted,
            Kind::ActionAborted => NativeKind::ActionAborted,
            Kind::Other => NativeKind::Other,
        };
        let processing = match notification.processing {
            Processing::ImportantAll => NativeProcessing::ImportantAll,
            Processing::ImportantMostRecent => NativeProcessing::ImportantMostRecent,
            Processing::All => NativeProcessing::All,
            Processing::MostRecent => NativeProcessing::MostRecent,
            Processing::CurrentThenMostRecent => NativeProcessing::CurrentThenMostRecent,
        };
        peer.RaiseNotificationEvent(
            kind,
            processing,
            &notification.text,
            &notification.activity_id,
        )?;
        Ok(())
    }
}

/// Returns the element's automation peer, or `None` for elements such as panels that have none.
fn automation_peer(
    element: &native::UIElement,
) -> windows_core::Result<Option<native::AutomationPeer>> {
    match native::FrameworkElementAutomationPeer::CreatePeerForElement(element) {
        Ok(peer) => Ok(Some(peer)),
        Err(error) if error.code().is_ok() => Ok(None),
        Err(error) => Err(error),
    }
}
