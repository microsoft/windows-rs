/// A symbol icon matching the WinUI `Microsoft.UI.Xaml.Controls.Symbol` enum.
///
/// Used across command bar buttons, navigation view items, selector bar items,
/// and any other widget that supports an icon slot.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SymbolGlyph {
    Add,
    Delete,
    Edit,
    Save,
    Cancel,
    Accept,
    More,
    Redo,
    Undo,
    Home,
    Back,
    Forward,
    Favorite,
    Camera,
    Setting,
    Find,
    Help,
    Mail,
    Send,
    Copy,
    Cut,
    Paste,
    Play,
    Pause,
    Download,
    Upload,
    Sync,
    People,
    Flag,
    World,
    /// Brightness / sun glyph (MDL2 \uE706).
    Brightness,
    /// Clear night / moon glyph (MDL2 \uE708).
    ClearNight,
}

impl SymbolGlyph {
    /// Returns the WinUI `Symbol` integer value.
    pub fn to_raw(self) -> i32 {
        match self {
            Self::Add => 57609,
            Self::Delete => 57607,
            Self::Edit => 57604,
            Self::Save => 57605,
            Self::Cancel => 57610,
            Self::Accept => 57611,
            Self::More => 57612,
            Self::Redo => 57613,
            Self::Undo => 57614,
            Self::Home => 57615,
            Self::Back => 57618,
            Self::Forward => 57617,
            Self::Favorite => 57619,
            Self::Camera => 57620,
            Self::Setting => 57621,
            Self::Find => 57626,
            Self::Help => 57627,
            Self::Mail => 57625,
            Self::Send => 57634,
            Self::Copy => 57711,
            Self::Cut => 57710,
            Self::Paste => 57709,
            Self::Play => 57602,
            Self::Pause => 57603,
            Self::Download => 57624,
            Self::Upload => 57628,
            Self::Sync => 57623,
            Self::People => 57637,
            Self::Flag => 57641,
            Self::World => 57640,
            Self::Brightness => 0xE706,
            Self::ClearNight => 0xE708,
        }
    }
}
