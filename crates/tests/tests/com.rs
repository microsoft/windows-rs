use tests::windows::win32::windows_update_agent::IAutomaticUpdates;
use windows::{create_instance, initialize_mta, initialize_sta, Guid, Result};

#[test]
fn test_sta() -> Result<()> {
    initialize_sta()?;
    let clsid = Guid::from_progid("Microsoft.Update.AutoUpdate")?;
    let updates: IAutomaticUpdates = create_instance(&clsid)?;
    let _ = unsafe { updates.Pause() };
    Ok(())
}

#[test]
fn test_mta() -> Result<()> {
    initialize_mta()?;
    let clsid = Guid::from_progid("Microsoft.Update.AutoUpdate")?;
    let updates: IAutomaticUpdates = create_instance(&clsid)?;
    let _ = unsafe { updates.Pause() };
    Ok(())
}
