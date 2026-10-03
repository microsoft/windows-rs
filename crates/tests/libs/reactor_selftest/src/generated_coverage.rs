use reactor::*;
use windows_reactor as reactor;
pub struct CoverageCase {
    pub contract: &'static str,
    pub set: fn() -> View,
    pub clear: fn() -> View,
}
pub fn cases() -> Vec<CoverageCase> {
    vec![
        CoverageCase {
            contract: "attached.CanvasLeft",
            set: coverage_0_set,
            clear: coverage_0_clear,
        },
        CoverageCase {
            contract: "attached.CanvasTop",
            set: coverage_1_set,
            clear: coverage_1_clear,
        },
        CoverageCase {
            contract: "attached.GridRow",
            set: coverage_2_set,
            clear: coverage_2_clear,
        },
        CoverageCase {
            contract: "attached.GridColumn",
            set: coverage_3_set,
            clear: coverage_3_clear,
        },
        CoverageCase {
            contract: "attached.GridRowSpan",
            set: coverage_4_set,
            clear: coverage_4_clear,
        },
        CoverageCase {
            contract: "attached.GridColumnSpan",
            set: coverage_5_set,
            clear: coverage_5_clear,
        },
        CoverageCase {
            contract: "attached.RelativeAlignLeft",
            set: coverage_6_set,
            clear: coverage_6_clear,
        },
        CoverageCase {
            contract: "attached.RelativeAlignTop",
            set: coverage_7_set,
            clear: coverage_7_clear,
        },
        CoverageCase {
            contract: "attached.RelativeAlignRight",
            set: coverage_8_set,
            clear: coverage_8_clear,
        },
        CoverageCase {
            contract: "attached.RelativeAlignBottom",
            set: coverage_9_set,
            clear: coverage_9_clear,
        },
        CoverageCase {
            contract: "attached.RelativeAlignHorizontalCenter",
            set: coverage_10_set,
            clear: coverage_10_clear,
        },
        CoverageCase {
            contract: "attached.RelativeAlignVerticalCenter",
            set: coverage_11_set,
            clear: coverage_11_clear,
        },
        CoverageCase {
            contract: "attached.AutomationName",
            set: coverage_12_set,
            clear: coverage_12_clear,
        },
        CoverageCase {
            contract: "attached.AutomationId",
            set: coverage_13_set,
            clear: coverage_13_clear,
        },
        CoverageCase {
            contract: "attached.AutomationHeadingLevel",
            set: coverage_14_set,
            clear: coverage_14_clear,
        },
        CoverageCase {
            contract: "visual.Width",
            set: coverage_15_set,
            clear: coverage_15_clear,
        },
        CoverageCase {
            contract: "visual.Height",
            set: coverage_16_set,
            clear: coverage_16_clear,
        },
        CoverageCase {
            contract: "visual.MinWidth",
            set: coverage_17_set,
            clear: coverage_17_clear,
        },
        CoverageCase {
            contract: "visual.MaxWidth",
            set: coverage_18_set,
            clear: coverage_18_clear,
        },
        CoverageCase {
            contract: "visual.MinHeight",
            set: coverage_19_set,
            clear: coverage_19_clear,
        },
        CoverageCase {
            contract: "visual.MaxHeight",
            set: coverage_20_set,
            clear: coverage_20_clear,
        },
        CoverageCase {
            contract: "visual.Margin",
            set: coverage_21_set,
            clear: coverage_21_clear,
        },
        CoverageCase {
            contract: "visual.HorizontalAlignment",
            set: coverage_22_set,
            clear: coverage_22_clear,
        },
        CoverageCase {
            contract: "visual.VerticalAlignment",
            set: coverage_23_set,
            clear: coverage_23_clear,
        },
        CoverageCase {
            contract: "visual.Opacity",
            set: coverage_24_set,
            clear: coverage_24_clear,
        },
        CoverageCase {
            contract: "visual.Transitions",
            set: coverage_25_set,
            clear: coverage_25_clear,
        },
        CoverageCase {
            contract: "TextBlock.Text",
            set: coverage_26_set,
            clear: coverage_26_clear,
        },
        CoverageCase {
            contract: "TextBlock.FontSize",
            set: coverage_27_set,
            clear: coverage_27_clear,
        },
        CoverageCase {
            contract: "TextBlock.FontWeight",
            set: coverage_28_set,
            clear: coverage_28_clear,
        },
        CoverageCase {
            contract: "TextBlock.Foreground",
            set: coverage_29_set,
            clear: coverage_29_clear,
        },
        CoverageCase {
            contract: "TextBlock.Padding",
            set: coverage_30_set,
            clear: coverage_30_clear,
        },
        CoverageCase {
            contract: "TextBlock.TextWrapping",
            set: coverage_31_set,
            clear: coverage_31_clear,
        },
        CoverageCase {
            contract: "TextBlock.IsTextSelectionEnabled",
            set: coverage_32_set,
            clear: coverage_32_clear,
        },
        CoverageCase {
            contract: "TextBlock.TextTrimming",
            set: coverage_33_set,
            clear: coverage_33_clear,
        },
        CoverageCase {
            contract: "TextBlock.MaxLines",
            set: coverage_34_set,
            clear: coverage_34_clear,
        },
        CoverageCase {
            contract: "TextBox.Text",
            set: coverage_35_set,
            clear: coverage_35_clear,
        },
        CoverageCase {
            contract: "TextBox.PlaceholderText",
            set: coverage_36_set,
            clear: coverage_36_clear,
        },
        CoverageCase {
            contract: "TextBox.AcceptsReturn",
            set: coverage_37_set,
            clear: coverage_37_clear,
        },
        CoverageCase {
            contract: "TextBox.TextWrapping",
            set: coverage_38_set,
            clear: coverage_38_clear,
        },
        CoverageCase {
            contract: "TextBox.Background",
            set: coverage_39_set,
            clear: coverage_39_clear,
        },
        CoverageCase {
            contract: "TextBox.BorderBrush",
            set: coverage_40_set,
            clear: coverage_40_clear,
        },
        CoverageCase {
            contract: "TextBox.BorderThickness",
            set: coverage_41_set,
            clear: coverage_41_clear,
        },
        CoverageCase {
            contract: "TextBox.TextChanged",
            set: coverage_42_set,
            clear: coverage_42_clear,
        },
        CoverageCase {
            contract: "Button.Background",
            set: coverage_43_set,
            clear: coverage_43_clear,
        },
        CoverageCase {
            contract: "Button.IsEnabled",
            set: coverage_44_set,
            clear: coverage_44_clear,
        },
        CoverageCase {
            contract: "Button.HorizontalContentAlignment",
            set: coverage_45_set,
            clear: coverage_45_clear,
        },
        CoverageCase {
            contract: "Button.VerticalContentAlignment",
            set: coverage_46_set,
            clear: coverage_46_clear,
        },
        CoverageCase {
            contract: "Button.Resources",
            set: coverage_47_set,
            clear: coverage_47_clear,
        },
        CoverageCase {
            contract: "Button.Style",
            set: coverage_48_set,
            clear: coverage_48_clear,
        },
        CoverageCase {
            contract: "Button.KeyboardAccelerators",
            set: coverage_49_set,
            clear: coverage_49_clear,
        },
        CoverageCase {
            contract: "Button.Click",
            set: coverage_50_set,
            clear: coverage_50_clear,
        },
        CoverageCase {
            contract: "CheckBox.IsChecked",
            set: coverage_51_set,
            clear: coverage_51_clear,
        },
        CoverageCase {
            contract: "CheckBox.IsEnabled",
            set: coverage_52_set,
            clear: coverage_52_clear,
        },
        CoverageCase {
            contract: "CheckBox.Click",
            set: coverage_53_set,
            clear: coverage_53_clear,
        },
        CoverageCase {
            contract: "CheckBox.IsCheckedChanged",
            set: coverage_54_set,
            clear: coverage_54_clear,
        },
        CoverageCase {
            contract: "Border.Background",
            set: coverage_55_set,
            clear: coverage_55_clear,
        },
        CoverageCase {
            contract: "Border.BorderBrush",
            set: coverage_56_set,
            clear: coverage_56_clear,
        },
        CoverageCase {
            contract: "Border.BorderThickness",
            set: coverage_57_set,
            clear: coverage_57_clear,
        },
        CoverageCase {
            contract: "Border.CornerRadius",
            set: coverage_58_set,
            clear: coverage_58_clear,
        },
        CoverageCase {
            contract: "Border.Padding",
            set: coverage_59_set,
            clear: coverage_59_clear,
        },
        CoverageCase {
            contract: "Border.IsTabStop",
            set: coverage_60_set,
            clear: coverage_60_clear,
        },
        CoverageCase {
            contract: "Border.AllowFocusOnInteraction",
            set: coverage_61_set,
            clear: coverage_61_clear,
        },
        CoverageCase {
            contract: "Border.OpacityTransition",
            set: coverage_62_set,
            clear: coverage_62_clear,
        },
        CoverageCase {
            contract: "Border.Scale",
            set: coverage_63_set,
            clear: coverage_63_clear,
        },
        CoverageCase {
            contract: "Border.ScaleTransition",
            set: coverage_64_set,
            clear: coverage_64_clear,
        },
        CoverageCase {
            contract: "Border.CapturePointerOnPress",
            set: coverage_65_set,
            clear: coverage_65_clear,
        },
        CoverageCase {
            contract: "Border.FocusOnPointerRelease",
            set: coverage_66_set,
            clear: coverage_66_clear,
        },
        CoverageCase {
            contract: "Border.DropPolicy",
            set: coverage_67_set,
            clear: coverage_67_clear,
        },
        CoverageCase {
            contract: "Border.PointerPressed",
            set: coverage_68_set,
            clear: coverage_68_clear,
        },
        CoverageCase {
            contract: "Border.PointerMoved",
            set: coverage_69_set,
            clear: coverage_69_clear,
        },
        CoverageCase {
            contract: "Border.PointerEntered",
            set: coverage_70_set,
            clear: coverage_70_clear,
        },
        CoverageCase {
            contract: "Border.PointerExited",
            set: coverage_71_set,
            clear: coverage_71_clear,
        },
        CoverageCase {
            contract: "Border.PointerReleased",
            set: coverage_72_set,
            clear: coverage_72_clear,
        },
        CoverageCase {
            contract: "Border.PointerCaptureLost",
            set: coverage_73_set,
            clear: coverage_73_clear,
        },
        CoverageCase {
            contract: "Border.PointerCanceled",
            set: coverage_74_set,
            clear: coverage_74_clear,
        },
        CoverageCase {
            contract: "Border.PreviewKeyDown",
            set: coverage_75_set,
            clear: coverage_75_clear,
        },
        CoverageCase {
            contract: "Border.KeyUp",
            set: coverage_76_set,
            clear: coverage_76_clear,
        },
        CoverageCase {
            contract: "Border.CharacterReceived",
            set: coverage_77_set,
            clear: coverage_77_clear,
        },
        CoverageCase {
            contract: "Border.GotFocus",
            set: coverage_78_set,
            clear: coverage_78_clear,
        },
        CoverageCase {
            contract: "Border.LostFocus",
            set: coverage_79_set,
            clear: coverage_79_clear,
        },
        CoverageCase {
            contract: "Border.DragEnter",
            set: coverage_80_set,
            clear: coverage_80_clear,
        },
        CoverageCase {
            contract: "Border.DragOver",
            set: coverage_81_set,
            clear: coverage_81_clear,
        },
        CoverageCase {
            contract: "Border.DragLeave",
            set: coverage_82_set,
            clear: coverage_82_clear,
        },
        CoverageCase {
            contract: "Border.Drop",
            set: coverage_83_set,
            clear: coverage_83_clear,
        },
        CoverageCase {
            contract: "Grid.GridRows",
            set: coverage_84_set,
            clear: coverage_84_clear,
        },
        CoverageCase {
            contract: "Grid.GridColumns",
            set: coverage_85_set,
            clear: coverage_85_clear,
        },
        CoverageCase {
            contract: "Grid.RowSpacing",
            set: coverage_86_set,
            clear: coverage_86_clear,
        },
        CoverageCase {
            contract: "Grid.ColumnSpacing",
            set: coverage_87_set,
            clear: coverage_87_clear,
        },
        CoverageCase {
            contract: "Grid.Background",
            set: coverage_88_set,
            clear: coverage_88_clear,
        },
        CoverageCase {
            contract: "Grid.KeyboardAccelerators",
            set: coverage_89_set,
            clear: coverage_89_clear,
        },
        CoverageCase {
            contract: "StackPanel.Spacing",
            set: coverage_90_set,
            clear: coverage_90_clear,
        },
        CoverageCase {
            contract: "StackPanel.Orientation",
            set: coverage_91_set,
            clear: coverage_91_clear,
        },
        CoverageCase {
            contract: "ScrollViewer.HorizontalScrollBarVisibility",
            set: coverage_92_set,
            clear: coverage_92_clear,
        },
        CoverageCase {
            contract: "ScrollViewer.VerticalScrollBarVisibility",
            set: coverage_93_set,
            clear: coverage_93_clear,
        },
        CoverageCase {
            contract: "Viewbox.Stretch",
            set: coverage_94_set,
            clear: coverage_94_clear,
        },
        CoverageCase {
            contract: "Slider.Minimum",
            set: coverage_95_set,
            clear: coverage_95_clear,
        },
        CoverageCase {
            contract: "Slider.Maximum",
            set: coverage_96_set,
            clear: coverage_96_clear,
        },
        CoverageCase {
            contract: "Slider.Value",
            set: coverage_97_set,
            clear: coverage_97_clear,
        },
        CoverageCase {
            contract: "Slider.IsEnabled",
            set: coverage_98_set,
            clear: coverage_98_clear,
        },
        CoverageCase {
            contract: "Slider.Orientation",
            set: coverage_99_set,
            clear: coverage_99_clear,
        },
        CoverageCase {
            contract: "Slider.StepFrequency",
            set: coverage_100_set,
            clear: coverage_100_clear,
        },
        CoverageCase {
            contract: "Slider.ValueChanged",
            set: coverage_101_set,
            clear: coverage_101_clear,
        },
        CoverageCase {
            contract: "TreeView.SelectionMode",
            set: coverage_102_set,
            clear: coverage_102_clear,
        },
        CoverageCase {
            contract: "TreeView.ItemInvoked",
            set: coverage_103_set,
            clear: coverage_103_clear,
        },
        CoverageCase {
            contract: "ListView.SelectionMode",
            set: coverage_104_set,
            clear: coverage_104_clear,
        },
        CoverageCase {
            contract: "ListView.CanDragItems",
            set: coverage_105_set,
            clear: coverage_105_clear,
        },
        CoverageCase {
            contract: "ListView.CanReorderItems",
            set: coverage_106_set,
            clear: coverage_106_clear,
        },
        CoverageCase {
            contract: "ListView.AllowDrop",
            set: coverage_107_set,
            clear: coverage_107_clear,
        },
        CoverageCase {
            contract: "ListView.SelectedIndex",
            set: coverage_108_set,
            clear: coverage_108_clear,
        },
        CoverageCase {
            contract: "ListView.SelectionChanged",
            set: coverage_109_set,
            clear: coverage_109_clear,
        },
        CoverageCase {
            contract: "ListView.DragItemsCompleted",
            set: coverage_110_set,
            clear: coverage_110_clear,
        },
        CoverageCase {
            contract: "HyperlinkButton.NavigateUri",
            set: coverage_111_set,
            clear: coverage_111_clear,
        },
        CoverageCase {
            contract: "HyperlinkButton.IsEnabled",
            set: coverage_112_set,
            clear: coverage_112_clear,
        },
        CoverageCase {
            contract: "HyperlinkButton.Click",
            set: coverage_113_set,
            clear: coverage_113_clear,
        },
        CoverageCase {
            contract: "RepeatButton.IsEnabled",
            set: coverage_114_set,
            clear: coverage_114_clear,
        },
        CoverageCase {
            contract: "RepeatButton.Delay",
            set: coverage_115_set,
            clear: coverage_115_clear,
        },
        CoverageCase {
            contract: "RepeatButton.Interval",
            set: coverage_116_set,
            clear: coverage_116_clear,
        },
        CoverageCase {
            contract: "RepeatButton.Click",
            set: coverage_117_set,
            clear: coverage_117_clear,
        },
        CoverageCase {
            contract: "BreadcrumbBar.ItemsSource",
            set: coverage_118_set,
            clear: coverage_118_clear,
        },
        CoverageCase {
            contract: "BreadcrumbBar.ItemClicked",
            set: coverage_119_set,
            clear: coverage_119_clear,
        },
        CoverageCase {
            contract: "VariableSizedWrapGrid.Orientation",
            set: coverage_120_set,
            clear: coverage_120_clear,
        },
        CoverageCase {
            contract: "VariableSizedWrapGrid.ItemWidth",
            set: coverage_121_set,
            clear: coverage_121_clear,
        },
        CoverageCase {
            contract: "VariableSizedWrapGrid.ItemHeight",
            set: coverage_122_set,
            clear: coverage_122_clear,
        },
        CoverageCase {
            contract: "AutoSuggestBox.PlaceholderText",
            set: coverage_123_set,
            clear: coverage_123_clear,
        },
        CoverageCase {
            contract: "AutoSuggestBox.IsEnabled",
            set: coverage_124_set,
            clear: coverage_124_clear,
        },
        CoverageCase {
            contract: "AutoSuggestBox.ItemsSource",
            set: coverage_125_set,
            clear: coverage_125_clear,
        },
        CoverageCase {
            contract: "AutoSuggestBox.Text",
            set: coverage_126_set,
            clear: coverage_126_clear,
        },
        CoverageCase {
            contract: "AutoSuggestBox.QueryIcon",
            set: coverage_127_set,
            clear: coverage_127_clear,
        },
        CoverageCase {
            contract: "AutoSuggestBox.TextChanged",
            set: coverage_128_set,
            clear: coverage_128_clear,
        },
        CoverageCase {
            contract: "AutoSuggestBox.SuggestionChosen",
            set: coverage_129_set,
            clear: coverage_129_clear,
        },
        CoverageCase {
            contract: "PasswordBox.PlaceholderText",
            set: coverage_130_set,
            clear: coverage_130_clear,
        },
        CoverageCase {
            contract: "PasswordBox.PasswordRevealMode",
            set: coverage_131_set,
            clear: coverage_131_clear,
        },
        CoverageCase {
            contract: "PasswordBox.IsEnabled",
            set: coverage_132_set,
            clear: coverage_132_clear,
        },
        CoverageCase {
            contract: "PasswordBox.Password",
            set: coverage_133_set,
            clear: coverage_133_clear,
        },
        CoverageCase {
            contract: "PasswordBox.PasswordChanged",
            set: coverage_134_set,
            clear: coverage_134_clear,
        },
        CoverageCase {
            contract: "NumberBox.IsEnabled",
            set: coverage_135_set,
            clear: coverage_135_clear,
        },
        CoverageCase {
            contract: "NumberBox.Minimum",
            set: coverage_136_set,
            clear: coverage_136_clear,
        },
        CoverageCase {
            contract: "NumberBox.Maximum",
            set: coverage_137_set,
            clear: coverage_137_clear,
        },
        CoverageCase {
            contract: "NumberBox.Value",
            set: coverage_138_set,
            clear: coverage_138_clear,
        },
        CoverageCase {
            contract: "NumberBox.ValueChanged",
            set: coverage_139_set,
            clear: coverage_139_clear,
        },
        CoverageCase {
            contract: "NavigationView.IsEnabled",
            set: coverage_140_set,
            clear: coverage_140_clear,
        },
        CoverageCase {
            contract: "NavigationView.PaneDisplayMode",
            set: coverage_141_set,
            clear: coverage_141_clear,
        },
        CoverageCase {
            contract: "NavigationView.IsPaneToggleButtonVisible",
            set: coverage_142_set,
            clear: coverage_142_clear,
        },
        CoverageCase {
            contract: "NavigationView.IsBackButtonVisible",
            set: coverage_143_set,
            clear: coverage_143_clear,
        },
        CoverageCase {
            contract: "NavigationView.IsSettingsVisible",
            set: coverage_144_set,
            clear: coverage_144_clear,
        },
        CoverageCase {
            contract: "NavigationView.AlwaysShowHeader",
            set: coverage_145_set,
            clear: coverage_145_clear,
        },
        CoverageCase {
            contract: "NavigationView.PaneTitle",
            set: coverage_146_set,
            clear: coverage_146_clear,
        },
        CoverageCase {
            contract: "NavigationView.OpenPaneLength",
            set: coverage_147_set,
            clear: coverage_147_clear,
        },
        CoverageCase {
            contract: "NavigationView.IsPaneOpen",
            set: coverage_148_set,
            clear: coverage_148_clear,
        },
        CoverageCase {
            contract: "NavigationView.IsPaneOpenChanged",
            set: coverage_149_set,
            clear: coverage_149_clear,
        },
        CoverageCase {
            contract: "NavigationView.SelectionChanged",
            set: coverage_150_set,
            clear: coverage_150_clear,
        },
        CoverageCase {
            contract: "NavigationView.DisplayModeChanged",
            set: coverage_151_set,
            clear: coverage_151_clear,
        },
        CoverageCase {
            contract: "NavigationViewItem.IsSelected",
            set: coverage_152_set,
            clear: coverage_152_clear,
        },
        CoverageCase {
            contract: "NavigationViewItem.SelectsOnInvoked",
            set: coverage_153_set,
            clear: coverage_153_clear,
        },
        CoverageCase {
            contract: "NavigationViewItem.IsExpanded",
            set: coverage_154_set,
            clear: coverage_154_clear,
        },
        CoverageCase {
            contract: "NavigationViewItem.Tag",
            set: coverage_155_set,
            clear: coverage_155_clear,
        },
        CoverageCase {
            contract: "NavigationViewItem.Icon",
            set: coverage_156_set,
            clear: coverage_156_clear,
        },
        CoverageCase {
            contract: "SplitView.OpenPaneLength",
            set: coverage_157_set,
            clear: coverage_157_clear,
        },
        CoverageCase {
            contract: "SplitView.CompactPaneLength",
            set: coverage_158_set,
            clear: coverage_158_clear,
        },
        CoverageCase {
            contract: "SplitView.DisplayMode",
            set: coverage_159_set,
            clear: coverage_159_clear,
        },
        CoverageCase {
            contract: "SplitView.IsPaneOpen",
            set: coverage_160_set,
            clear: coverage_160_clear,
        },
        CoverageCase {
            contract: "SplitView.PaneClosed",
            set: coverage_161_set,
            clear: coverage_161_clear,
        },
        CoverageCase {
            contract: "ProgressBar.Minimum",
            set: coverage_162_set,
            clear: coverage_162_clear,
        },
        CoverageCase {
            contract: "ProgressBar.Maximum",
            set: coverage_163_set,
            clear: coverage_163_clear,
        },
        CoverageCase {
            contract: "ProgressBar.Value",
            set: coverage_164_set,
            clear: coverage_164_clear,
        },
        CoverageCase {
            contract: "ProgressBar.IsIndeterminate",
            set: coverage_165_set,
            clear: coverage_165_clear,
        },
        CoverageCase {
            contract: "ProgressBar.ShowError",
            set: coverage_166_set,
            clear: coverage_166_clear,
        },
        CoverageCase {
            contract: "ProgressBar.ShowPaused",
            set: coverage_167_set,
            clear: coverage_167_clear,
        },
        CoverageCase {
            contract: "ProgressBar.IsEnabled",
            set: coverage_168_set,
            clear: coverage_168_clear,
        },
        CoverageCase {
            contract: "ToggleSwitch.IsEnabled",
            set: coverage_169_set,
            clear: coverage_169_clear,
        },
        CoverageCase {
            contract: "ToggleSwitch.IsOn",
            set: coverage_170_set,
            clear: coverage_170_clear,
        },
        CoverageCase {
            contract: "ToggleSwitch.Toggled",
            set: coverage_171_set,
            clear: coverage_171_clear,
        },
        CoverageCase {
            contract: "ToggleButton.IsEnabled",
            set: coverage_172_set,
            clear: coverage_172_clear,
        },
        CoverageCase {
            contract: "ToggleButton.IsChecked",
            set: coverage_173_set,
            clear: coverage_173_clear,
        },
        CoverageCase {
            contract: "ToggleButton.IsCheckedChanged",
            set: coverage_174_set,
            clear: coverage_174_clear,
        },
        CoverageCase {
            contract: "RadioButton.GroupName",
            set: coverage_175_set,
            clear: coverage_175_clear,
        },
        CoverageCase {
            contract: "RadioButton.IsEnabled",
            set: coverage_176_set,
            clear: coverage_176_clear,
        },
        CoverageCase {
            contract: "RadioButton.IsChecked",
            set: coverage_177_set,
            clear: coverage_177_clear,
        },
        CoverageCase {
            contract: "RadioButton.Checked",
            set: coverage_178_set,
            clear: coverage_178_clear,
        },
        CoverageCase {
            contract: "RadioButtons.MaxColumns",
            set: coverage_179_set,
            clear: coverage_179_clear,
        },
        CoverageCase {
            contract: "RadioButtons.ItemsSource",
            set: coverage_180_set,
            clear: coverage_180_clear,
        },
        CoverageCase {
            contract: "RadioButtons.SelectedIndex",
            set: coverage_181_set,
            clear: coverage_181_clear,
        },
        CoverageCase {
            contract: "RadioButtons.SelectionChanged",
            set: coverage_182_set,
            clear: coverage_182_clear,
        },
        CoverageCase {
            contract: "InfoBadge.Value",
            set: coverage_183_set,
            clear: coverage_183_clear,
        },
        CoverageCase {
            contract: "InfoBar.Title",
            set: coverage_184_set,
            clear: coverage_184_clear,
        },
        CoverageCase {
            contract: "InfoBar.Message",
            set: coverage_185_set,
            clear: coverage_185_clear,
        },
        CoverageCase {
            contract: "InfoBar.Severity",
            set: coverage_186_set,
            clear: coverage_186_clear,
        },
        CoverageCase {
            contract: "InfoBar.IsOpen",
            set: coverage_187_set,
            clear: coverage_187_clear,
        },
        CoverageCase {
            contract: "InfoBar.IsClosable",
            set: coverage_188_set,
            clear: coverage_188_clear,
        },
        CoverageCase {
            contract: "InfoBar.Closed",
            set: coverage_189_set,
            clear: coverage_189_clear,
        },
        CoverageCase {
            contract: "PersonPicture.DisplayName",
            set: coverage_190_set,
            clear: coverage_190_clear,
        },
        CoverageCase {
            contract: "PersonPicture.Initials",
            set: coverage_191_set,
            clear: coverage_191_clear,
        },
        CoverageCase {
            contract: "ScrollView.HorizontalScrollBarVisibility",
            set: coverage_192_set,
            clear: coverage_192_clear,
        },
        CoverageCase {
            contract: "ScrollView.VerticalScrollBarVisibility",
            set: coverage_193_set,
            clear: coverage_193_clear,
        },
        CoverageCase {
            contract: "Image.Source",
            set: coverage_194_set,
            clear: coverage_194_clear,
        },
        CoverageCase {
            contract: "Image.Stretch",
            set: coverage_195_set,
            clear: coverage_195_clear,
        },
        CoverageCase {
            contract: "Image.ImageOpened",
            set: coverage_196_set,
            clear: coverage_196_clear,
        },
        CoverageCase {
            contract: "Image.ImageFailed",
            set: coverage_197_set,
            clear: coverage_197_clear,
        },
        CoverageCase {
            contract: "ProgressRing.Minimum",
            set: coverage_198_set,
            clear: coverage_198_clear,
        },
        CoverageCase {
            contract: "ProgressRing.Maximum",
            set: coverage_199_set,
            clear: coverage_199_clear,
        },
        CoverageCase {
            contract: "ProgressRing.Value",
            set: coverage_200_set,
            clear: coverage_200_clear,
        },
        CoverageCase {
            contract: "ProgressRing.IsIndeterminate",
            set: coverage_201_set,
            clear: coverage_201_clear,
        },
        CoverageCase {
            contract: "ProgressRing.IsActive",
            set: coverage_202_set,
            clear: coverage_202_clear,
        },
        CoverageCase {
            contract: "ProgressRing.IsEnabled",
            set: coverage_203_set,
            clear: coverage_203_clear,
        },
        CoverageCase {
            contract: "ListBox.IsEnabled",
            set: coverage_204_set,
            clear: coverage_204_clear,
        },
        CoverageCase {
            contract: "ListBox.SelectionChanged",
            set: coverage_205_set,
            clear: coverage_205_clear,
        },
        CoverageCase {
            contract: "Rectangle.Fill",
            set: coverage_206_set,
            clear: coverage_206_clear,
        },
        CoverageCase {
            contract: "Rectangle.Stroke",
            set: coverage_207_set,
            clear: coverage_207_clear,
        },
        CoverageCase {
            contract: "Rectangle.StrokeThickness",
            set: coverage_208_set,
            clear: coverage_208_clear,
        },
        CoverageCase {
            contract: "Rectangle.RadiusX",
            set: coverage_209_set,
            clear: coverage_209_clear,
        },
        CoverageCase {
            contract: "Rectangle.RadiusY",
            set: coverage_210_set,
            clear: coverage_210_clear,
        },
        CoverageCase {
            contract: "Ellipse.Fill",
            set: coverage_211_set,
            clear: coverage_211_clear,
        },
        CoverageCase {
            contract: "Ellipse.Stroke",
            set: coverage_212_set,
            clear: coverage_212_clear,
        },
        CoverageCase {
            contract: "Ellipse.StrokeThickness",
            set: coverage_213_set,
            clear: coverage_213_clear,
        },
        CoverageCase {
            contract: "Line.Stroke",
            set: coverage_214_set,
            clear: coverage_214_clear,
        },
        CoverageCase {
            contract: "Line.StrokeThickness",
            set: coverage_215_set,
            clear: coverage_215_clear,
        },
        CoverageCase {
            contract: "Line.X1",
            set: coverage_216_set,
            clear: coverage_216_clear,
        },
        CoverageCase {
            contract: "Line.Y1",
            set: coverage_217_set,
            clear: coverage_217_clear,
        },
        CoverageCase {
            contract: "Line.X2",
            set: coverage_218_set,
            clear: coverage_218_clear,
        },
        CoverageCase {
            contract: "Line.Y2",
            set: coverage_219_set,
            clear: coverage_219_clear,
        },
        CoverageCase {
            contract: "SymbolIcon.Symbol",
            set: coverage_220_set,
            clear: coverage_220_clear,
        },
        CoverageCase {
            contract: "ImageIcon.Source",
            set: coverage_221_set,
            clear: coverage_221_clear,
        },
        CoverageCase {
            contract: "FontIcon.Glyph",
            set: coverage_222_set,
            clear: coverage_222_clear,
        },
        CoverageCase {
            contract: "BitmapIcon.ShowAsMonochrome",
            set: coverage_223_set,
            clear: coverage_223_clear,
        },
        CoverageCase {
            contract: "BitmapIcon.UriSource",
            set: coverage_224_set,
            clear: coverage_224_clear,
        },
        CoverageCase {
            contract: "PathIcon.Data",
            set: coverage_225_set,
            clear: coverage_225_clear,
        },
        CoverageCase {
            contract: "ListBoxItem.IsSelected",
            set: coverage_226_set,
            clear: coverage_226_clear,
        },
        CoverageCase {
            contract: "ListBoxItem.Tag",
            set: coverage_227_set,
            clear: coverage_227_clear,
        },
        CoverageCase {
            contract: "RatingControl.Caption",
            set: coverage_228_set,
            clear: coverage_228_clear,
        },
        CoverageCase {
            contract: "RatingControl.IsReadOnly",
            set: coverage_229_set,
            clear: coverage_229_clear,
        },
        CoverageCase {
            contract: "RatingControl.MaxRating",
            set: coverage_230_set,
            clear: coverage_230_clear,
        },
        CoverageCase {
            contract: "RatingControl.Value",
            set: coverage_231_set,
            clear: coverage_231_clear,
        },
        CoverageCase {
            contract: "RatingControl.ValueChanged",
            set: coverage_232_set,
            clear: coverage_232_clear,
        },
        CoverageCase {
            contract: "Expander.IsExpanded",
            set: coverage_233_set,
            clear: coverage_233_clear,
        },
        CoverageCase {
            contract: "Expander.HorizontalContentAlignment",
            set: coverage_234_set,
            clear: coverage_234_clear,
        },
        CoverageCase {
            contract: "Expander.Resources",
            set: coverage_235_set,
            clear: coverage_235_clear,
        },
        CoverageCase {
            contract: "Expander.IsExpandedChanged",
            set: coverage_236_set,
            clear: coverage_236_clear,
        },
        CoverageCase {
            contract: "ComboBox.PlaceholderText",
            set: coverage_237_set,
            clear: coverage_237_clear,
        },
        CoverageCase {
            contract: "ComboBox.IsEditable",
            set: coverage_238_set,
            clear: coverage_238_clear,
        },
        CoverageCase {
            contract: "ComboBox.IsEnabled",
            set: coverage_239_set,
            clear: coverage_239_clear,
        },
        CoverageCase {
            contract: "ComboBox.ItemsSource",
            set: coverage_240_set,
            clear: coverage_240_clear,
        },
        CoverageCase {
            contract: "ComboBox.SelectedIndex",
            set: coverage_241_set,
            clear: coverage_241_clear,
        },
        CoverageCase {
            contract: "ComboBox.SelectionChanged",
            set: coverage_242_set,
            clear: coverage_242_clear,
        },
        CoverageCase {
            contract: "Pivot.Title",
            set: coverage_243_set,
            clear: coverage_243_clear,
        },
        CoverageCase {
            contract: "Pivot.SelectedIndex",
            set: coverage_244_set,
            clear: coverage_244_clear,
        },
        CoverageCase {
            contract: "Pivot.SelectionChanged",
            set: coverage_245_set,
            clear: coverage_245_clear,
        },
        CoverageCase {
            contract: "PivotItem.Header",
            set: coverage_246_set,
            clear: coverage_246_clear,
        },
        CoverageCase {
            contract: "FlipView.SelectedIndex",
            set: coverage_247_set,
            clear: coverage_247_clear,
        },
        CoverageCase {
            contract: "FlipView.SelectionChanged",
            set: coverage_248_set,
            clear: coverage_248_clear,
        },
        CoverageCase {
            contract: "SelectorBar.SelectionChanged",
            set: coverage_249_set,
            clear: coverage_249_clear,
        },
        CoverageCase {
            contract: "SelectorBarItem.Text",
            set: coverage_250_set,
            clear: coverage_250_clear,
        },
        CoverageCase {
            contract: "SelectorBarItem.IsSelected",
            set: coverage_251_set,
            clear: coverage_251_clear,
        },
        CoverageCase {
            contract: "SelectorBarItem.Icon",
            set: coverage_252_set,
            clear: coverage_252_clear,
        },
        CoverageCase {
            contract: "TabView.CanReorderTabs",
            set: coverage_253_set,
            clear: coverage_253_clear,
        },
        CoverageCase {
            contract: "TabView.IsAddTabButtonVisible",
            set: coverage_254_set,
            clear: coverage_254_clear,
        },
        CoverageCase {
            contract: "TabView.SelectedIndex",
            set: coverage_255_set,
            clear: coverage_255_clear,
        },
        CoverageCase {
            contract: "TabView.AddTabButtonClick",
            set: coverage_256_set,
            clear: coverage_256_clear,
        },
        CoverageCase {
            contract: "TabView.SelectionChanged",
            set: coverage_257_set,
            clear: coverage_257_clear,
        },
        CoverageCase {
            contract: "TabView.TabCloseRequested",
            set: coverage_258_set,
            clear: coverage_258_clear,
        },
        CoverageCase {
            contract: "TabView.TabItemsChanged",
            set: coverage_259_set,
            clear: coverage_259_clear,
        },
        CoverageCase {
            contract: "TabViewItem.IsClosable",
            set: coverage_260_set,
            clear: coverage_260_clear,
        },
        CoverageCase {
            contract: "TabViewItem.Header",
            set: coverage_261_set,
            clear: coverage_261_clear,
        },
        CoverageCase {
            contract: "TabViewItem.Tag",
            set: coverage_262_set,
            clear: coverage_262_clear,
        },
        CoverageCase {
            contract: "TeachingTip.Title",
            set: coverage_263_set,
            clear: coverage_263_clear,
        },
        CoverageCase {
            contract: "TeachingTip.Subtitle",
            set: coverage_264_set,
            clear: coverage_264_clear,
        },
        CoverageCase {
            contract: "TeachingTip.IsOpen",
            set: coverage_265_set,
            clear: coverage_265_clear,
        },
        CoverageCase {
            contract: "TeachingTip.IsLightDismissEnabled",
            set: coverage_266_set,
            clear: coverage_266_clear,
        },
        CoverageCase {
            contract: "TeachingTip.PreferredPlacement",
            set: coverage_267_set,
            clear: coverage_267_clear,
        },
        CoverageCase {
            contract: "TeachingTip.ActionButtonContent",
            set: coverage_268_set,
            clear: coverage_268_clear,
        },
        CoverageCase {
            contract: "TeachingTip.CloseButtonContent",
            set: coverage_269_set,
            clear: coverage_269_clear,
        },
        CoverageCase {
            contract: "TeachingTip.Closed",
            set: coverage_270_set,
            clear: coverage_270_clear,
        },
        CoverageCase {
            contract: "TeachingTip.ActionButtonClick",
            set: coverage_271_set,
            clear: coverage_271_clear,
        },
        CoverageCase {
            contract: "DropDownButton.IsEnabled",
            set: coverage_272_set,
            clear: coverage_272_clear,
        },
        CoverageCase {
            contract: "DropDownButton.Click",
            set: coverage_273_set,
            clear: coverage_273_clear,
        },
        CoverageCase {
            contract: "AppBarButton.Label",
            set: coverage_274_set,
            clear: coverage_274_clear,
        },
        CoverageCase {
            contract: "AppBarButton.IsEnabled",
            set: coverage_275_set,
            clear: coverage_275_clear,
        },
        CoverageCase {
            contract: "AppBarButton.Icon",
            set: coverage_276_set,
            clear: coverage_276_clear,
        },
        CoverageCase {
            contract: "AppBarButton.Click",
            set: coverage_277_set,
            clear: coverage_277_clear,
        },
        CoverageCase {
            contract: "MenuBarItem.Title",
            set: coverage_278_set,
            clear: coverage_278_clear,
        },
        CoverageCase {
            contract: "SplitButton.IsEnabled",
            set: coverage_279_set,
            clear: coverage_279_clear,
        },
        CoverageCase {
            contract: "SplitButton.Click",
            set: coverage_280_set,
            clear: coverage_280_clear,
        },
        CoverageCase {
            contract: "ColorPicker.Color",
            set: coverage_281_set,
            clear: coverage_281_clear,
        },
        CoverageCase {
            contract: "ColorPicker.IsAlphaEnabled",
            set: coverage_282_set,
            clear: coverage_282_clear,
        },
        CoverageCase {
            contract: "ColorPicker.IsHexInputVisible",
            set: coverage_283_set,
            clear: coverage_283_clear,
        },
        CoverageCase {
            contract: "ColorPicker.IsColorSliderVisible",
            set: coverage_284_set,
            clear: coverage_284_clear,
        },
        CoverageCase {
            contract: "ColorPicker.IsColorChannelTextInputVisible",
            set: coverage_285_set,
            clear: coverage_285_clear,
        },
        CoverageCase {
            contract: "ColorPicker.IsEnabled",
            set: coverage_286_set,
            clear: coverage_286_clear,
        },
        CoverageCase {
            contract: "ColorPicker.ColorChanged",
            set: coverage_287_set,
            clear: coverage_287_clear,
        },
        CoverageCase {
            contract: "DatePicker.DayVisible",
            set: coverage_288_set,
            clear: coverage_288_clear,
        },
        CoverageCase {
            contract: "DatePicker.MonthVisible",
            set: coverage_289_set,
            clear: coverage_289_clear,
        },
        CoverageCase {
            contract: "DatePicker.YearVisible",
            set: coverage_290_set,
            clear: coverage_290_clear,
        },
        CoverageCase {
            contract: "DatePicker.IsEnabled",
            set: coverage_291_set,
            clear: coverage_291_clear,
        },
        CoverageCase {
            contract: "DatePicker.SelectedDateChanged",
            set: coverage_292_set,
            clear: coverage_292_clear,
        },
        CoverageCase {
            contract: "TimePicker.IsEnabled",
            set: coverage_293_set,
            clear: coverage_293_clear,
        },
        CoverageCase {
            contract: "TimePicker.MinuteIncrement",
            set: coverage_294_set,
            clear: coverage_294_clear,
        },
        CoverageCase {
            contract: "TimePicker.ClockIdentifier",
            set: coverage_295_set,
            clear: coverage_295_clear,
        },
        CoverageCase {
            contract: "TimePicker.SelectedTimeChanged",
            set: coverage_296_set,
            clear: coverage_296_clear,
        },
        CoverageCase {
            contract: "CalendarDatePicker.PlaceholderText",
            set: coverage_297_set,
            clear: coverage_297_clear,
        },
        CoverageCase {
            contract: "CalendarDatePicker.IsTodayHighlighted",
            set: coverage_298_set,
            clear: coverage_298_clear,
        },
        CoverageCase {
            contract: "CalendarDatePicker.IsCalendarOpen",
            set: coverage_299_set,
            clear: coverage_299_clear,
        },
        CoverageCase {
            contract: "CalendarDatePicker.IsEnabled",
            set: coverage_300_set,
            clear: coverage_300_clear,
        },
        CoverageCase {
            contract: "CalendarDatePicker.DateChanged",
            set: coverage_301_set,
            clear: coverage_301_clear,
        },
        CoverageCase {
            contract: "CalendarView.IsTodayHighlighted",
            set: coverage_302_set,
            clear: coverage_302_clear,
        },
        CoverageCase {
            contract: "CalendarView.IsGroupLabelVisible",
            set: coverage_303_set,
            clear: coverage_303_clear,
        },
        CoverageCase {
            contract: "CalendarView.IsEnabled",
            set: coverage_304_set,
            clear: coverage_304_clear,
        },
        CoverageCase {
            contract: "CalendarView.SelectedDatesChanged",
            set: coverage_305_set,
            clear: coverage_305_clear,
        },
        CoverageCase {
            contract: "ListViewItem.Tag",
            set: coverage_306_set,
            clear: coverage_306_clear,
        },
        CoverageCase {
            contract: "GridView.CanDragItems",
            set: coverage_307_set,
            clear: coverage_307_clear,
        },
        CoverageCase {
            contract: "GridView.CanReorderItems",
            set: coverage_308_set,
            clear: coverage_308_clear,
        },
        CoverageCase {
            contract: "GridView.AllowDrop",
            set: coverage_309_set,
            clear: coverage_309_clear,
        },
        CoverageCase {
            contract: "GridView.SelectedIndex",
            set: coverage_310_set,
            clear: coverage_310_clear,
        },
        CoverageCase {
            contract: "GridView.SelectionChanged",
            set: coverage_311_set,
            clear: coverage_311_clear,
        },
        CoverageCase {
            contract: "GridView.DragItemsCompleted",
            set: coverage_312_set,
            clear: coverage_312_clear,
        },
        CoverageCase {
            contract: "GridViewItem.Tag",
            set: coverage_313_set,
            clear: coverage_313_clear,
        },
        CoverageCase {
            contract: "RichEditBox.Document",
            set: coverage_314_set,
            clear: coverage_314_clear,
        },
        CoverageCase {
            contract: "RichEditBox.PlaceholderText",
            set: coverage_315_set,
            clear: coverage_315_clear,
        },
        CoverageCase {
            contract: "RichEditBox.IsReadOnly",
            set: coverage_316_set,
            clear: coverage_316_clear,
        },
        CoverageCase {
            contract: "RichEditBox.IsEnabled",
            set: coverage_317_set,
            clear: coverage_317_clear,
        },
        CoverageCase {
            contract: "RichEditBox.TextChanged",
            set: coverage_318_set,
            clear: coverage_318_clear,
        },
        CoverageCase {
            contract: "RichTextBlock.Blocks",
            set: coverage_319_set,
            clear: coverage_319_clear,
        },
        CoverageCase {
            contract: "RichTextBlock.IsTextSelectionEnabled",
            set: coverage_320_set,
            clear: coverage_320_clear,
        },
        CoverageCase {
            contract: "RichTextBlock.TextWrapping",
            set: coverage_321_set,
            clear: coverage_321_clear,
        },
        CoverageCase {
            contract: "RichTextBlock.FontSize",
            set: coverage_322_set,
            clear: coverage_322_clear,
        },
    ]
}
fn coverage_0_set() -> View {
    TextBlock::new().canvas_left(1.0).into()
}
fn coverage_0_clear() -> View {
    TextBlock::new().into()
}
fn coverage_1_set() -> View {
    TextBlock::new().canvas_top(1.0).into()
}
fn coverage_1_clear() -> View {
    TextBlock::new().into()
}
fn coverage_2_set() -> View {
    TextBlock::new().grid_row(1).into()
}
fn coverage_2_clear() -> View {
    TextBlock::new().into()
}
fn coverage_3_set() -> View {
    TextBlock::new().grid_column(1).into()
}
fn coverage_3_clear() -> View {
    TextBlock::new().into()
}
fn coverage_4_set() -> View {
    TextBlock::new().grid_row_span(1).into()
}
fn coverage_4_clear() -> View {
    TextBlock::new().into()
}
fn coverage_5_set() -> View {
    TextBlock::new().grid_column_span(1).into()
}
fn coverage_5_clear() -> View {
    TextBlock::new().into()
}
fn coverage_6_set() -> View {
    TextBlock::new().relative_align_left().into()
}
fn coverage_6_clear() -> View {
    TextBlock::new().into()
}
fn coverage_7_set() -> View {
    TextBlock::new().relative_align_top().into()
}
fn coverage_7_clear() -> View {
    TextBlock::new().into()
}
fn coverage_8_set() -> View {
    TextBlock::new().relative_align_right().into()
}
fn coverage_8_clear() -> View {
    TextBlock::new().into()
}
fn coverage_9_set() -> View {
    TextBlock::new().relative_align_bottom().into()
}
fn coverage_9_clear() -> View {
    TextBlock::new().into()
}
fn coverage_10_set() -> View {
    TextBlock::new().relative_align_horizontal_center().into()
}
fn coverage_10_clear() -> View {
    TextBlock::new().into()
}
fn coverage_11_set() -> View {
    TextBlock::new().relative_align_vertical_center().into()
}
fn coverage_11_clear() -> View {
    TextBlock::new().into()
}
fn coverage_12_set() -> View {
    TextBlock::new().automation_name("coverage").into()
}
fn coverage_12_clear() -> View {
    TextBlock::new().into()
}
fn coverage_13_set() -> View {
    TextBlock::new().automation_id("coverage").into()
}
fn coverage_13_clear() -> View {
    TextBlock::new().into()
}
fn coverage_14_set() -> View {
    TextBlock::new()
        .automation_heading_level(AutomationHeadingLevel::Level1)
        .into()
}
fn coverage_14_clear() -> View {
    TextBlock::new().into()
}
fn coverage_15_set() -> View {
    TextBlock::new().width(1.0).into()
}
fn coverage_15_clear() -> View {
    TextBlock::new().into()
}
fn coverage_16_set() -> View {
    TextBlock::new().height(1.0).into()
}
fn coverage_16_clear() -> View {
    TextBlock::new().into()
}
fn coverage_17_set() -> View {
    TextBlock::new().min_width(1.0).into()
}
fn coverage_17_clear() -> View {
    TextBlock::new().into()
}
fn coverage_18_set() -> View {
    TextBlock::new().max_width(1.0).into()
}
fn coverage_18_clear() -> View {
    TextBlock::new().into()
}
fn coverage_19_set() -> View {
    TextBlock::new().min_height(1.0).into()
}
fn coverage_19_clear() -> View {
    TextBlock::new().into()
}
fn coverage_20_set() -> View {
    TextBlock::new().max_height(1.0).into()
}
fn coverage_20_clear() -> View {
    TextBlock::new().into()
}
fn coverage_21_set() -> View {
    TextBlock::new().margin(Thickness::uniform(1.0)).into()
}
fn coverage_21_clear() -> View {
    TextBlock::new().into()
}
fn coverage_22_set() -> View {
    TextBlock::new()
        .horizontal_alignment(HorizontalAlignment::Center)
        .into()
}
fn coverage_22_clear() -> View {
    TextBlock::new().into()
}
fn coverage_23_set() -> View {
    TextBlock::new()
        .vertical_alignment(VerticalAlignment::Center)
        .into()
}
fn coverage_23_clear() -> View {
    TextBlock::new().into()
}
fn coverage_24_set() -> View {
    TextBlock::new().opacity(0.5).into()
}
fn coverage_24_clear() -> View {
    TextBlock::new().into()
}
fn coverage_25_set() -> View {
    TextBlock::new()
        .transitions([ThemeTransition::Reposition])
        .into()
}
fn coverage_25_clear() -> View {
    TextBlock::new().into()
}
fn coverage_26_set() -> View {
    TextBlock::new().text("coverage").into()
}
fn coverage_26_clear() -> View {
    TextBlock::new().into()
}
fn coverage_27_set() -> View {
    TextBlock::new().font_size(1.0).into()
}
fn coverage_27_clear() -> View {
    TextBlock::new().into()
}
fn coverage_28_set() -> View {
    TextBlock::new().font_weight(FontWeight::SEMI_BOLD).into()
}
fn coverage_28_clear() -> View {
    TextBlock::new().into()
}
fn coverage_29_set() -> View {
    TextBlock::new().foreground(Color::rgb(1, 2, 3)).into()
}
fn coverage_29_clear() -> View {
    TextBlock::new().into()
}
fn coverage_30_set() -> View {
    TextBlock::new().padding(Thickness::uniform(1.0)).into()
}
fn coverage_30_clear() -> View {
    TextBlock::new().into()
}
fn coverage_31_set() -> View {
    TextBlock::new().text_wrapping(TextWrapping::Wrap).into()
}
fn coverage_31_clear() -> View {
    TextBlock::new().into()
}
fn coverage_32_set() -> View {
    TextBlock::new().is_text_selection_enabled(true).into()
}
fn coverage_32_clear() -> View {
    TextBlock::new().into()
}
fn coverage_33_set() -> View {
    TextBlock::new()
        .text_trimming(TextTrimming::CharacterEllipsis)
        .into()
}
fn coverage_33_clear() -> View {
    TextBlock::new().into()
}
fn coverage_34_set() -> View {
    TextBlock::new().max_lines(1).into()
}
fn coverage_34_clear() -> View {
    TextBlock::new().into()
}
fn coverage_35_set() -> View {
    TextBox::new("Text").into()
}
fn coverage_35_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_36_set() -> View {
    TextBox::new("Text").placeholder_text("coverage").into()
}
fn coverage_36_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_37_set() -> View {
    TextBox::new("Text").accepts_return(true).into()
}
fn coverage_37_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_38_set() -> View {
    TextBox::new("Text")
        .text_wrapping(TextWrapping::Wrap)
        .into()
}
fn coverage_38_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_39_set() -> View {
    TextBox::new("Text").background(Color::rgb(1, 2, 3)).into()
}
fn coverage_39_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_40_set() -> View {
    TextBox::new("Text")
        .border_brush(Color::rgb(1, 2, 3))
        .into()
}
fn coverage_40_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_41_set() -> View {
    TextBox::new("Text")
        .border_thickness(Thickness::uniform(1.0))
        .into()
}
fn coverage_41_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_42_set() -> View {
    TextBox::new("Text").on_text_changed(|_| {}).into()
}
fn coverage_42_clear() -> View {
    TextBox::new("Text").into()
}
fn coverage_43_set() -> View {
    Button::new().background(Color::rgb(1, 2, 3)).into()
}
fn coverage_43_clear() -> View {
    Button::new().into()
}
fn coverage_44_set() -> View {
    Button::new().is_enabled(true).into()
}
fn coverage_44_clear() -> View {
    Button::new().into()
}
fn coverage_45_set() -> View {
    Button::new()
        .horizontal_content_alignment(HorizontalAlignment::Center)
        .into()
}
fn coverage_45_clear() -> View {
    Button::new().into()
}
fn coverage_46_set() -> View {
    Button::new()
        .vertical_content_alignment(VerticalAlignment::Center)
        .into()
}
fn coverage_46_clear() -> View {
    Button::new().into()
}
fn coverage_47_set() -> View {
    Button::new()
        .resource_overrides(ResourceOverrides::new())
        .into()
}
fn coverage_47_clear() -> View {
    Button::new().into()
}
fn coverage_48_set() -> View {
    Button::new().style(ButtonStyle::Default).into()
}
fn coverage_48_clear() -> View {
    Button::new().into()
}
fn coverage_49_set() -> View {
    Button::new()
        .key_accelerators(KeyAccelerators::default())
        .into()
}
fn coverage_49_clear() -> View {
    Button::new().into()
}
fn coverage_50_set() -> View {
    Button::new().on_click(|| {}).into()
}
fn coverage_50_clear() -> View {
    Button::new().into()
}
fn coverage_51_set() -> View {
    CheckBox::new().is_checked(Some(true)).into()
}
fn coverage_51_clear() -> View {
    CheckBox::new().into()
}
fn coverage_52_set() -> View {
    CheckBox::new().is_enabled(true).into()
}
fn coverage_52_clear() -> View {
    CheckBox::new().into()
}
fn coverage_53_set() -> View {
    CheckBox::new().on_click(|| {}).into()
}
fn coverage_53_clear() -> View {
    CheckBox::new().into()
}
fn coverage_54_set() -> View {
    CheckBox::new().on_is_checked_changed(|_| {}).into()
}
fn coverage_54_clear() -> View {
    CheckBox::new().into()
}
fn coverage_55_set() -> View {
    Border::new().background(Color::rgb(1, 2, 3)).into()
}
fn coverage_55_clear() -> View {
    Border::new().into()
}
fn coverage_56_set() -> View {
    Border::new().border_brush(Color::rgb(1, 2, 3)).into()
}
fn coverage_56_clear() -> View {
    Border::new().into()
}
fn coverage_57_set() -> View {
    Border::new()
        .border_thickness(Thickness::uniform(1.0))
        .into()
}
fn coverage_57_clear() -> View {
    Border::new().into()
}
fn coverage_58_set() -> View {
    Border::new()
        .corner_radius(CornerRadius::uniform(1.0))
        .into()
}
fn coverage_58_clear() -> View {
    Border::new().into()
}
fn coverage_59_set() -> View {
    Border::new().padding(Thickness::uniform(1.0)).into()
}
fn coverage_59_clear() -> View {
    Border::new().into()
}
fn coverage_60_set() -> View {
    Border::new().is_tab_stop(true).into()
}
fn coverage_60_clear() -> View {
    Border::new().into()
}
fn coverage_61_set() -> View {
    Border::new().allow_focus_on_interaction(true).into()
}
fn coverage_61_clear() -> View {
    Border::new().into()
}
fn coverage_62_set() -> View {
    Border::new()
        .opacity_transition(std::time::Duration::from_millis(1))
        .into()
}
fn coverage_62_clear() -> View {
    Border::new().into()
}
fn coverage_63_set() -> View {
    Border::new().scale(1.0).into()
}
fn coverage_63_clear() -> View {
    Border::new().into()
}
fn coverage_64_set() -> View {
    Border::new()
        .scale_transition(std::time::Duration::from_millis(1))
        .into()
}
fn coverage_64_clear() -> View {
    Border::new().into()
}
fn coverage_65_set() -> View {
    Border::new().capture_pointer_on_press(true).into()
}
fn coverage_65_clear() -> View {
    Border::new().into()
}
fn coverage_66_set() -> View {
    Border::new().focus_on_pointer_release(true).into()
}
fn coverage_66_clear() -> View {
    Border::new().into()
}
fn coverage_67_set() -> View {
    Border::new().drop_policy(DragDropPolicy::new()).into()
}
fn coverage_67_clear() -> View {
    Border::new().into()
}
fn coverage_68_set() -> View {
    Border::new().on_pointer_pressed(|_| {}).into()
}
fn coverage_68_clear() -> View {
    Border::new().into()
}
fn coverage_69_set() -> View {
    Border::new().on_pointer_moved(|_| {}).into()
}
fn coverage_69_clear() -> View {
    Border::new().into()
}
fn coverage_70_set() -> View {
    Border::new().on_pointer_entered(|_| {}).into()
}
fn coverage_70_clear() -> View {
    Border::new().into()
}
fn coverage_71_set() -> View {
    Border::new().on_pointer_exited(|_| {}).into()
}
fn coverage_71_clear() -> View {
    Border::new().into()
}
fn coverage_72_set() -> View {
    Border::new().on_pointer_released(|_| {}).into()
}
fn coverage_72_clear() -> View {
    Border::new().into()
}
fn coverage_73_set() -> View {
    Border::new().on_pointer_capture_lost(|| {}).into()
}
fn coverage_73_clear() -> View {
    Border::new().into()
}
fn coverage_74_set() -> View {
    Border::new().on_pointer_canceled(|| {}).into()
}
fn coverage_74_clear() -> View {
    Border::new().into()
}
fn coverage_75_set() -> View {
    Border::new()
        .on_preview_key_down(RoutedCallback::new(|_| false))
        .into()
}
fn coverage_75_clear() -> View {
    Border::new().into()
}
fn coverage_76_set() -> View {
    Border::new()
        .on_key_up(RoutedCallback::new(|_| false))
        .into()
}
fn coverage_76_clear() -> View {
    Border::new().into()
}
fn coverage_77_set() -> View {
    Border::new()
        .on_character_received(RoutedCallback::new(|_| false))
        .into()
}
fn coverage_77_clear() -> View {
    Border::new().into()
}
fn coverage_78_set() -> View {
    Border::new().on_got_focus(|_| {}).into()
}
fn coverage_78_clear() -> View {
    Border::new().into()
}
fn coverage_79_set() -> View {
    Border::new().on_lost_focus(|_| {}).into()
}
fn coverage_79_clear() -> View {
    Border::new().into()
}
fn coverage_80_set() -> View {
    Border::new().on_drag_enter(|_| {}).into()
}
fn coverage_80_clear() -> View {
    Border::new().into()
}
fn coverage_81_set() -> View {
    Border::new().on_drag_over(|_| {}).into()
}
fn coverage_81_clear() -> View {
    Border::new().into()
}
fn coverage_82_set() -> View {
    Border::new().on_drag_leave(|| {}).into()
}
fn coverage_82_clear() -> View {
    Border::new().into()
}
fn coverage_83_set() -> View {
    Border::new().on_drop(|_| {}).into()
}
fn coverage_83_clear() -> View {
    Border::new().into()
}
fn coverage_84_set() -> View {
    Grid::new().rows([GridLength::Auto]).into()
}
fn coverage_84_clear() -> View {
    Grid::new().into()
}
fn coverage_85_set() -> View {
    Grid::new().columns([GridLength::Auto]).into()
}
fn coverage_85_clear() -> View {
    Grid::new().into()
}
fn coverage_86_set() -> View {
    Grid::new().row_spacing(1.0).into()
}
fn coverage_86_clear() -> View {
    Grid::new().into()
}
fn coverage_87_set() -> View {
    Grid::new().column_spacing(1.0).into()
}
fn coverage_87_clear() -> View {
    Grid::new().into()
}
fn coverage_88_set() -> View {
    Grid::new().background(Color::rgb(1, 2, 3)).into()
}
fn coverage_88_clear() -> View {
    Grid::new().into()
}
fn coverage_89_set() -> View {
    Grid::new()
        .key_accelerators(KeyAccelerators::default())
        .into()
}
fn coverage_89_clear() -> View {
    Grid::new().into()
}
fn coverage_90_set() -> View {
    StackPanel::new().spacing(1.0).into()
}
fn coverage_90_clear() -> View {
    StackPanel::new().into()
}
fn coverage_91_set() -> View {
    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .into()
}
fn coverage_91_clear() -> View {
    StackPanel::new().into()
}
fn coverage_92_set() -> View {
    ScrollViewer::new()
        .horizontal_scroll_bar_visibility(ScrollBarVisibility::Visible)
        .into()
}
fn coverage_92_clear() -> View {
    ScrollViewer::new().into()
}
fn coverage_93_set() -> View {
    ScrollViewer::new()
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Visible)
        .into()
}
fn coverage_93_clear() -> View {
    ScrollViewer::new().into()
}
fn coverage_94_set() -> View {
    Viewbox::new().stretch(Stretch::Uniform).into()
}
fn coverage_94_clear() -> View {
    Viewbox::new().into()
}
fn coverage_95_set() -> View {
    Slider::new().minimum(1.0).into()
}
fn coverage_95_clear() -> View {
    Slider::new().into()
}
fn coverage_96_set() -> View {
    Slider::new().maximum(1.0).into()
}
fn coverage_96_clear() -> View {
    Slider::new().into()
}
fn coverage_97_set() -> View {
    Slider::new().value(1.0).into()
}
fn coverage_97_clear() -> View {
    Slider::new().into()
}
fn coverage_98_set() -> View {
    Slider::new().is_enabled(true).into()
}
fn coverage_98_clear() -> View {
    Slider::new().into()
}
fn coverage_99_set() -> View {
    Slider::new().orientation(Orientation::Horizontal).into()
}
fn coverage_99_clear() -> View {
    Slider::new().into()
}
fn coverage_100_set() -> View {
    Slider::new().step_frequency(1.0).into()
}
fn coverage_100_clear() -> View {
    Slider::new().into()
}
fn coverage_101_set() -> View {
    Slider::new().on_value_changed(|_| {}).into()
}
fn coverage_101_clear() -> View {
    Slider::new().into()
}
fn coverage_102_set() -> View {
    TreeView::new()
        .selection_mode(TreeViewSelectionMode::Single)
        .into()
}
fn coverage_102_clear() -> View {
    TreeView::new().into()
}
fn coverage_103_set() -> View {
    TreeView::new().on_item_invoked(|_| {}).into()
}
fn coverage_103_clear() -> View {
    TreeView::new().into()
}
fn coverage_104_set() -> View {
    ListView::new()
        .selection_mode(ListViewSelectionMode::Single)
        .into()
}
fn coverage_104_clear() -> View {
    ListView::new().into()
}
fn coverage_105_set() -> View {
    ListView::new().can_drag_items(true).into()
}
fn coverage_105_clear() -> View {
    ListView::new().into()
}
fn coverage_106_set() -> View {
    ListView::new().can_reorder_items(true).into()
}
fn coverage_106_clear() -> View {
    ListView::new().into()
}
fn coverage_107_set() -> View {
    ListView::new().allow_drop(true).into()
}
fn coverage_107_clear() -> View {
    ListView::new().into()
}
fn coverage_108_set() -> View {
    ListView::new().selected_index(Some(0)).into()
}
fn coverage_108_clear() -> View {
    ListView::new().into()
}
fn coverage_109_set() -> View {
    ListView::new().on_selection_changed(|_| {}).into()
}
fn coverage_109_clear() -> View {
    ListView::new().into()
}
fn coverage_110_set() -> View {
    ListView::new().on_reordered(|_| {}).into()
}
fn coverage_110_clear() -> View {
    ListView::new().into()
}
fn coverage_111_set() -> View {
    HyperlinkButton::new()
        .navigate_uri("https://example.com")
        .unwrap()
        .into()
}
fn coverage_111_clear() -> View {
    HyperlinkButton::new().into()
}
fn coverage_112_set() -> View {
    HyperlinkButton::new().is_enabled(true).into()
}
fn coverage_112_clear() -> View {
    HyperlinkButton::new().into()
}
fn coverage_113_set() -> View {
    HyperlinkButton::new().on_click(|| {}).into()
}
fn coverage_113_clear() -> View {
    HyperlinkButton::new().into()
}
fn coverage_114_set() -> View {
    RepeatButton::new().is_enabled(true).into()
}
fn coverage_114_clear() -> View {
    RepeatButton::new().into()
}
fn coverage_115_set() -> View {
    RepeatButton::new().delay(1).into()
}
fn coverage_115_clear() -> View {
    RepeatButton::new().into()
}
fn coverage_116_set() -> View {
    RepeatButton::new().interval(1).into()
}
fn coverage_116_clear() -> View {
    RepeatButton::new().into()
}
fn coverage_117_set() -> View {
    RepeatButton::new().on_click(|| {}).into()
}
fn coverage_117_clear() -> View {
    RepeatButton::new().into()
}
fn coverage_118_set() -> View {
    BreadcrumbBar::new().items_source(["coverage"]).into()
}
fn coverage_118_clear() -> View {
    BreadcrumbBar::new().into()
}
fn coverage_119_set() -> View {
    BreadcrumbBar::new().on_item_clicked(|_| {}).into()
}
fn coverage_119_clear() -> View {
    BreadcrumbBar::new().into()
}
fn coverage_120_set() -> View {
    VariableSizedWrapGrid::new()
        .orientation(Orientation::Horizontal)
        .into()
}
fn coverage_120_clear() -> View {
    VariableSizedWrapGrid::new().into()
}
fn coverage_121_set() -> View {
    VariableSizedWrapGrid::new().item_width(1.0).into()
}
fn coverage_121_clear() -> View {
    VariableSizedWrapGrid::new().into()
}
fn coverage_122_set() -> View {
    VariableSizedWrapGrid::new().item_height(1.0).into()
}
fn coverage_122_clear() -> View {
    VariableSizedWrapGrid::new().into()
}
fn coverage_123_set() -> View {
    AutoSuggestBox::new().placeholder_text("coverage").into()
}
fn coverage_123_clear() -> View {
    AutoSuggestBox::new().into()
}
fn coverage_124_set() -> View {
    AutoSuggestBox::new().is_enabled(true).into()
}
fn coverage_124_clear() -> View {
    AutoSuggestBox::new().into()
}
fn coverage_125_set() -> View {
    AutoSuggestBox::new().items_source(["coverage"]).into()
}
fn coverage_125_clear() -> View {
    AutoSuggestBox::new().into()
}
fn coverage_126_set() -> View {
    AutoSuggestBox::new().text("coverage").into()
}
fn coverage_126_clear() -> View {
    AutoSuggestBox::new().into()
}
fn coverage_127_set() -> View {
    AutoSuggestBox::new().query_icon(Symbol::Previous).into()
}
fn coverage_127_clear() -> View {
    AutoSuggestBox::new().into()
}
fn coverage_128_set() -> View {
    AutoSuggestBox::new().on_text_changed(|_| {}).into()
}
fn coverage_128_clear() -> View {
    AutoSuggestBox::new().into()
}
fn coverage_129_set() -> View {
    AutoSuggestBox::new().on_suggestion_chosen(|_| {}).into()
}
fn coverage_129_clear() -> View {
    AutoSuggestBox::new().into()
}
fn coverage_130_set() -> View {
    PasswordBox::new().placeholder_text("coverage").into()
}
fn coverage_130_clear() -> View {
    PasswordBox::new().into()
}
fn coverage_131_set() -> View {
    PasswordBox::new()
        .password_reveal_mode(PasswordRevealMode::Visible)
        .into()
}
fn coverage_131_clear() -> View {
    PasswordBox::new().into()
}
fn coverage_132_set() -> View {
    PasswordBox::new().is_enabled(true).into()
}
fn coverage_132_clear() -> View {
    PasswordBox::new().into()
}
fn coverage_133_set() -> View {
    PasswordBox::new().password("coverage").into()
}
fn coverage_133_clear() -> View {
    PasswordBox::new().into()
}
fn coverage_134_set() -> View {
    PasswordBox::new().on_password_changed(|_| {}).into()
}
fn coverage_134_clear() -> View {
    PasswordBox::new().into()
}
fn coverage_135_set() -> View {
    NumberBox::new().is_enabled(true).into()
}
fn coverage_135_clear() -> View {
    NumberBox::new().into()
}
fn coverage_136_set() -> View {
    NumberBox::new().minimum(1.0).into()
}
fn coverage_136_clear() -> View {
    NumberBox::new().into()
}
fn coverage_137_set() -> View {
    NumberBox::new().maximum(1.0).into()
}
fn coverage_137_clear() -> View {
    NumberBox::new().into()
}
fn coverage_138_set() -> View {
    NumberBox::new().value(Some(1.0)).into()
}
fn coverage_138_clear() -> View {
    NumberBox::new().into()
}
fn coverage_139_set() -> View {
    NumberBox::new().on_value_changed(|_| {}).into()
}
fn coverage_139_clear() -> View {
    NumberBox::new().into()
}
fn coverage_140_set() -> View {
    NavigationView::new().is_enabled(true).into()
}
fn coverage_140_clear() -> View {
    NavigationView::new().into()
}
fn coverage_141_set() -> View {
    NavigationView::new()
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .into()
}
fn coverage_141_clear() -> View {
    NavigationView::new().into()
}
fn coverage_142_set() -> View {
    NavigationView::new()
        .is_pane_toggle_button_visible(true)
        .into()
}
fn coverage_142_clear() -> View {
    NavigationView::new().into()
}
fn coverage_143_set() -> View {
    NavigationView::new()
        .is_back_button_visible(NavigationViewBackButtonVisible::Visible)
        .into()
}
fn coverage_143_clear() -> View {
    NavigationView::new().into()
}
fn coverage_144_set() -> View {
    NavigationView::new().is_settings_visible(true).into()
}
fn coverage_144_clear() -> View {
    NavigationView::new().into()
}
fn coverage_145_set() -> View {
    NavigationView::new().always_show_header(true).into()
}
fn coverage_145_clear() -> View {
    NavigationView::new().into()
}
fn coverage_146_set() -> View {
    NavigationView::new().pane_title("coverage").into()
}
fn coverage_146_clear() -> View {
    NavigationView::new().into()
}
fn coverage_147_set() -> View {
    NavigationView::new().open_pane_length(1.0).into()
}
fn coverage_147_clear() -> View {
    NavigationView::new().into()
}
fn coverage_148_set() -> View {
    NavigationView::new().is_pane_open(true).into()
}
fn coverage_148_clear() -> View {
    NavigationView::new().into()
}
fn coverage_149_set() -> View {
    NavigationView::new().on_is_pane_open_changed(|_| {}).into()
}
fn coverage_149_clear() -> View {
    NavigationView::new().into()
}
fn coverage_150_set() -> View {
    NavigationView::new().on_selected_tag_changed(|_| {}).into()
}
fn coverage_150_clear() -> View {
    NavigationView::new().into()
}
fn coverage_151_set() -> View {
    NavigationView::new().on_display_mode_changed(|_| {}).into()
}
fn coverage_151_clear() -> View {
    NavigationView::new().into()
}
fn coverage_152_set() -> View {
    NavigationViewItem::new().is_selected(true).into()
}
fn coverage_152_clear() -> View {
    NavigationViewItem::new().into()
}
fn coverage_153_set() -> View {
    NavigationViewItem::new().selects_on_invoked(true).into()
}
fn coverage_153_clear() -> View {
    NavigationViewItem::new().into()
}
fn coverage_154_set() -> View {
    NavigationViewItem::new().is_expanded(true).into()
}
fn coverage_154_clear() -> View {
    NavigationViewItem::new().into()
}
fn coverage_155_set() -> View {
    NavigationViewItem::new().tag("coverage").into()
}
fn coverage_155_clear() -> View {
    NavigationViewItem::new().into()
}
fn coverage_156_set() -> View {
    NavigationViewItem::new().icon(Symbol::Previous).into()
}
fn coverage_156_clear() -> View {
    NavigationViewItem::new().into()
}
fn coverage_157_set() -> View {
    SplitView::new().open_pane_length(1.0).into()
}
fn coverage_157_clear() -> View {
    SplitView::new().into()
}
fn coverage_158_set() -> View {
    SplitView::new().compact_pane_length(1.0).into()
}
fn coverage_158_clear() -> View {
    SplitView::new().into()
}
fn coverage_159_set() -> View {
    SplitView::new()
        .display_mode(SplitViewDisplayMode::Inline)
        .into()
}
fn coverage_159_clear() -> View {
    SplitView::new().into()
}
fn coverage_160_set() -> View {
    SplitView::new().is_pane_open(true).into()
}
fn coverage_160_clear() -> View {
    SplitView::new().into()
}
fn coverage_161_set() -> View {
    SplitView::new().on_pane_closed(|_| {}).into()
}
fn coverage_161_clear() -> View {
    SplitView::new().into()
}
fn coverage_162_set() -> View {
    ProgressBar::new().minimum(1.0).into()
}
fn coverage_162_clear() -> View {
    ProgressBar::new().into()
}
fn coverage_163_set() -> View {
    ProgressBar::new().maximum(1.0).into()
}
fn coverage_163_clear() -> View {
    ProgressBar::new().into()
}
fn coverage_164_set() -> View {
    ProgressBar::new().value(1.0).into()
}
fn coverage_164_clear() -> View {
    ProgressBar::new().into()
}
fn coverage_165_set() -> View {
    ProgressBar::new().is_indeterminate(true).into()
}
fn coverage_165_clear() -> View {
    ProgressBar::new().into()
}
fn coverage_166_set() -> View {
    ProgressBar::new().show_error(true).into()
}
fn coverage_166_clear() -> View {
    ProgressBar::new().into()
}
fn coverage_167_set() -> View {
    ProgressBar::new().show_paused(true).into()
}
fn coverage_167_clear() -> View {
    ProgressBar::new().into()
}
fn coverage_168_set() -> View {
    ProgressBar::new().is_enabled(true).into()
}
fn coverage_168_clear() -> View {
    ProgressBar::new().into()
}
fn coverage_169_set() -> View {
    ToggleSwitch::new().is_enabled(true).into()
}
fn coverage_169_clear() -> View {
    ToggleSwitch::new().into()
}
fn coverage_170_set() -> View {
    ToggleSwitch::new().is_on(true).into()
}
fn coverage_170_clear() -> View {
    ToggleSwitch::new().into()
}
fn coverage_171_set() -> View {
    ToggleSwitch::new().on_toggled(|_| {}).into()
}
fn coverage_171_clear() -> View {
    ToggleSwitch::new().into()
}
fn coverage_172_set() -> View {
    ToggleButton::new().is_enabled(true).into()
}
fn coverage_172_clear() -> View {
    ToggleButton::new().into()
}
fn coverage_173_set() -> View {
    ToggleButton::new().is_checked(Some(true)).into()
}
fn coverage_173_clear() -> View {
    ToggleButton::new().into()
}
fn coverage_174_set() -> View {
    ToggleButton::new().on_is_checked_changed(|_| {}).into()
}
fn coverage_174_clear() -> View {
    ToggleButton::new().into()
}
fn coverage_175_set() -> View {
    RadioButton::new().group_name("coverage").into()
}
fn coverage_175_clear() -> View {
    RadioButton::new().into()
}
fn coverage_176_set() -> View {
    RadioButton::new().is_enabled(true).into()
}
fn coverage_176_clear() -> View {
    RadioButton::new().into()
}
fn coverage_177_set() -> View {
    RadioButton::new().is_checked(Some(true)).into()
}
fn coverage_177_clear() -> View {
    RadioButton::new().into()
}
fn coverage_178_set() -> View {
    RadioButton::new().on_checked(|_| {}).into()
}
fn coverage_178_clear() -> View {
    RadioButton::new().into()
}
fn coverage_179_set() -> View {
    RadioButtons::new().max_columns(1).into()
}
fn coverage_179_clear() -> View {
    RadioButtons::new().into()
}
fn coverage_180_set() -> View {
    RadioButtons::new().items_source(["coverage"]).into()
}
fn coverage_180_clear() -> View {
    RadioButtons::new().into()
}
fn coverage_181_set() -> View {
    RadioButtons::new().selected_index(Some(0)).into()
}
fn coverage_181_clear() -> View {
    RadioButtons::new().into()
}
fn coverage_182_set() -> View {
    RadioButtons::new().on_selection_changed(|_| {}).into()
}
fn coverage_182_clear() -> View {
    RadioButtons::new().into()
}
fn coverage_183_set() -> View {
    InfoBadge::new().value(1).into()
}
fn coverage_183_clear() -> View {
    InfoBadge::new().into()
}
fn coverage_184_set() -> View {
    InfoBar::new().title("coverage").into()
}
fn coverage_184_clear() -> View {
    InfoBar::new().into()
}
fn coverage_185_set() -> View {
    InfoBar::new().message("coverage").into()
}
fn coverage_185_clear() -> View {
    InfoBar::new().into()
}
fn coverage_186_set() -> View {
    InfoBar::new()
        .severity(InfoBarSeverity::Informational)
        .into()
}
fn coverage_186_clear() -> View {
    InfoBar::new().into()
}
fn coverage_187_set() -> View {
    InfoBar::new().is_open(false).into()
}
fn coverage_187_clear() -> View {
    InfoBar::new().into()
}
fn coverage_188_set() -> View {
    InfoBar::new().is_closable(true).into()
}
fn coverage_188_clear() -> View {
    InfoBar::new().into()
}
fn coverage_189_set() -> View {
    InfoBar::new().on_closed(|| {}).into()
}
fn coverage_189_clear() -> View {
    InfoBar::new().into()
}
fn coverage_190_set() -> View {
    PersonPicture::new().display_name("coverage").into()
}
fn coverage_190_clear() -> View {
    PersonPicture::new().into()
}
fn coverage_191_set() -> View {
    PersonPicture::new().initials("coverage").into()
}
fn coverage_191_clear() -> View {
    PersonPicture::new().into()
}
fn coverage_192_set() -> View {
    ScrollView::new()
        .horizontal_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible)
        .into()
}
fn coverage_192_clear() -> View {
    ScrollView::new().into()
}
fn coverage_193_set() -> View {
    ScrollView::new()
        .vertical_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible)
        .into()
}
fn coverage_193_clear() -> View {
    ScrollView::new().into()
}
fn coverage_194_set() -> View {
    Image::new().source("https://example.com").unwrap().into()
}
fn coverage_194_clear() -> View {
    Image::new().into()
}
fn coverage_195_set() -> View {
    Image::new().stretch(Stretch::Uniform).into()
}
fn coverage_195_clear() -> View {
    Image::new().into()
}
fn coverage_196_set() -> View {
    Image::new().on_opened(|| {}).into()
}
fn coverage_196_clear() -> View {
    Image::new().into()
}
fn coverage_197_set() -> View {
    Image::new().on_failed(|| {}).into()
}
fn coverage_197_clear() -> View {
    Image::new().into()
}
fn coverage_198_set() -> View {
    ProgressRing::new().minimum(1.0).into()
}
fn coverage_198_clear() -> View {
    ProgressRing::new().into()
}
fn coverage_199_set() -> View {
    ProgressRing::new().maximum(1.0).into()
}
fn coverage_199_clear() -> View {
    ProgressRing::new().into()
}
fn coverage_200_set() -> View {
    ProgressRing::new().value(1.0).into()
}
fn coverage_200_clear() -> View {
    ProgressRing::new().into()
}
fn coverage_201_set() -> View {
    ProgressRing::new().is_indeterminate(true).into()
}
fn coverage_201_clear() -> View {
    ProgressRing::new().into()
}
fn coverage_202_set() -> View {
    ProgressRing::new().is_active(true).into()
}
fn coverage_202_clear() -> View {
    ProgressRing::new().into()
}
fn coverage_203_set() -> View {
    ProgressRing::new().is_enabled(true).into()
}
fn coverage_203_clear() -> View {
    ProgressRing::new().into()
}
fn coverage_204_set() -> View {
    ListBox::new().is_enabled(true).into()
}
fn coverage_204_clear() -> View {
    ListBox::new().into()
}
fn coverage_205_set() -> View {
    ListBox::new().on_selected_tag_changed(|_| {}).into()
}
fn coverage_205_clear() -> View {
    ListBox::new().into()
}
fn coverage_206_set() -> View {
    Rectangle::new().fill(Color::rgb(1, 2, 3)).into()
}
fn coverage_206_clear() -> View {
    Rectangle::new().into()
}
fn coverage_207_set() -> View {
    Rectangle::new().stroke(Color::rgb(1, 2, 3)).into()
}
fn coverage_207_clear() -> View {
    Rectangle::new().into()
}
fn coverage_208_set() -> View {
    Rectangle::new().stroke_thickness(1.0).into()
}
fn coverage_208_clear() -> View {
    Rectangle::new().into()
}
fn coverage_209_set() -> View {
    Rectangle::new().radius_x(1.0).into()
}
fn coverage_209_clear() -> View {
    Rectangle::new().into()
}
fn coverage_210_set() -> View {
    Rectangle::new().radius_y(1.0).into()
}
fn coverage_210_clear() -> View {
    Rectangle::new().into()
}
fn coverage_211_set() -> View {
    Ellipse::new().fill(Color::rgb(1, 2, 3)).into()
}
fn coverage_211_clear() -> View {
    Ellipse::new().into()
}
fn coverage_212_set() -> View {
    Ellipse::new().stroke(Color::rgb(1, 2, 3)).into()
}
fn coverage_212_clear() -> View {
    Ellipse::new().into()
}
fn coverage_213_set() -> View {
    Ellipse::new().stroke_thickness(1.0).into()
}
fn coverage_213_clear() -> View {
    Ellipse::new().into()
}
fn coverage_214_set() -> View {
    Line::new().stroke(Color::rgb(1, 2, 3)).into()
}
fn coverage_214_clear() -> View {
    Line::new().into()
}
fn coverage_215_set() -> View {
    Line::new().stroke_thickness(1.0).into()
}
fn coverage_215_clear() -> View {
    Line::new().into()
}
fn coverage_216_set() -> View {
    Line::new().x1(1.0).into()
}
fn coverage_216_clear() -> View {
    Line::new().into()
}
fn coverage_217_set() -> View {
    Line::new().y1(1.0).into()
}
fn coverage_217_clear() -> View {
    Line::new().into()
}
fn coverage_218_set() -> View {
    Line::new().x2(1.0).into()
}
fn coverage_218_clear() -> View {
    Line::new().into()
}
fn coverage_219_set() -> View {
    Line::new().y2(1.0).into()
}
fn coverage_219_clear() -> View {
    Line::new().into()
}
fn coverage_220_set() -> View {
    SymbolIcon::new().symbol(Symbol::Previous).into()
}
fn coverage_220_clear() -> View {
    SymbolIcon::new().into()
}
fn coverage_221_set() -> View {
    ImageIcon::new()
        .source("https://example.com")
        .unwrap()
        .into()
}
fn coverage_221_clear() -> View {
    ImageIcon::new().into()
}
fn coverage_222_set() -> View {
    FontIcon::new().glyph("coverage").into()
}
fn coverage_222_clear() -> View {
    FontIcon::new().into()
}
fn coverage_223_set() -> View {
    BitmapIcon::new().show_as_monochrome(true).into()
}
fn coverage_223_clear() -> View {
    BitmapIcon::new().into()
}
fn coverage_224_set() -> View {
    BitmapIcon::new()
        .uri_source("https://example.com")
        .unwrap()
        .into()
}
fn coverage_224_clear() -> View {
    BitmapIcon::new().into()
}
fn coverage_225_set() -> View {
    PathIcon::new().data("M 0,0 L 1,1").into()
}
fn coverage_225_clear() -> View {
    PathIcon::new().into()
}
fn coverage_226_set() -> View {
    ListBoxItem::new().is_selected(true).into()
}
fn coverage_226_clear() -> View {
    ListBoxItem::new().into()
}
fn coverage_227_set() -> View {
    ListBoxItem::new().tag("coverage").into()
}
fn coverage_227_clear() -> View {
    ListBoxItem::new().into()
}
fn coverage_228_set() -> View {
    RatingControl::new().caption("coverage").into()
}
fn coverage_228_clear() -> View {
    RatingControl::new().into()
}
fn coverage_229_set() -> View {
    RatingControl::new().is_read_only(true).into()
}
fn coverage_229_clear() -> View {
    RatingControl::new().into()
}
fn coverage_230_set() -> View {
    RatingControl::new().max_rating(1).into()
}
fn coverage_230_clear() -> View {
    RatingControl::new().into()
}
fn coverage_231_set() -> View {
    RatingControl::new().value(Some(1.0)).into()
}
fn coverage_231_clear() -> View {
    RatingControl::new().into()
}
fn coverage_232_set() -> View {
    RatingControl::new().on_value_changed(|_| {}).into()
}
fn coverage_232_clear() -> View {
    RatingControl::new().into()
}
fn coverage_233_set() -> View {
    Expander::new().is_expanded(true).into()
}
fn coverage_233_clear() -> View {
    Expander::new().into()
}
fn coverage_234_set() -> View {
    Expander::new()
        .horizontal_content_alignment(HorizontalAlignment::Center)
        .into()
}
fn coverage_234_clear() -> View {
    Expander::new().into()
}
fn coverage_235_set() -> View {
    Expander::new()
        .resource_overrides(ResourceOverrides::new())
        .into()
}
fn coverage_235_clear() -> View {
    Expander::new().into()
}
fn coverage_236_set() -> View {
    Expander::new().on_is_expanded_changed(|_| {}).into()
}
fn coverage_236_clear() -> View {
    Expander::new().into()
}
fn coverage_237_set() -> View {
    ComboBox::new().placeholder_text("coverage").into()
}
fn coverage_237_clear() -> View {
    ComboBox::new().into()
}
fn coverage_238_set() -> View {
    ComboBox::new().is_editable(true).into()
}
fn coverage_238_clear() -> View {
    ComboBox::new().into()
}
fn coverage_239_set() -> View {
    ComboBox::new().is_enabled(true).into()
}
fn coverage_239_clear() -> View {
    ComboBox::new().into()
}
fn coverage_240_set() -> View {
    ComboBox::new().items_source(["coverage"]).into()
}
fn coverage_240_clear() -> View {
    ComboBox::new().into()
}
fn coverage_241_set() -> View {
    ComboBox::new().selected_index(Some(0)).into()
}
fn coverage_241_clear() -> View {
    ComboBox::new().into()
}
fn coverage_242_set() -> View {
    ComboBox::new().on_selection_changed(|_| {}).into()
}
fn coverage_242_clear() -> View {
    ComboBox::new().into()
}
fn coverage_243_set() -> View {
    Pivot::new().title("coverage").into()
}
fn coverage_243_clear() -> View {
    Pivot::new().into()
}
fn coverage_244_set() -> View {
    Pivot::new().selected_index(Some(0)).into()
}
fn coverage_244_clear() -> View {
    Pivot::new().into()
}
fn coverage_245_set() -> View {
    Pivot::new().on_selection_changed(|_| {}).into()
}
fn coverage_245_clear() -> View {
    Pivot::new().into()
}
fn coverage_246_set() -> View {
    PivotItem::new().header("coverage").into()
}
fn coverage_246_clear() -> View {
    PivotItem::new().into()
}
fn coverage_247_set() -> View {
    FlipView::new().selected_index(Some(0)).into()
}
fn coverage_247_clear() -> View {
    FlipView::new().into()
}
fn coverage_248_set() -> View {
    FlipView::new().on_selection_changed(|_| {}).into()
}
fn coverage_248_clear() -> View {
    FlipView::new().into()
}
fn coverage_249_set() -> View {
    SelectorBar::new().on_selected_text_changed(|_| {}).into()
}
fn coverage_249_clear() -> View {
    SelectorBar::new().into()
}
fn coverage_250_set() -> View {
    SelectorBarItem::new().text("coverage").into()
}
fn coverage_250_clear() -> View {
    SelectorBarItem::new().into()
}
fn coverage_251_set() -> View {
    SelectorBarItem::new().is_selected(true).into()
}
fn coverage_251_clear() -> View {
    SelectorBarItem::new().into()
}
fn coverage_252_set() -> View {
    SelectorBarItem::new().icon(Symbol::Previous).into()
}
fn coverage_252_clear() -> View {
    SelectorBarItem::new().into()
}
fn coverage_253_set() -> View {
    TabView::new().can_reorder_tabs(true).into()
}
fn coverage_253_clear() -> View {
    TabView::new().into()
}
fn coverage_254_set() -> View {
    TabView::new().is_add_tab_button_visible(true).into()
}
fn coverage_254_clear() -> View {
    TabView::new().into()
}
fn coverage_255_set() -> View {
    TabView::new().selected_index(Some(0)).into()
}
fn coverage_255_clear() -> View {
    TabView::new().into()
}
fn coverage_256_set() -> View {
    TabView::new().on_add_tab_button_click(|| {}).into()
}
fn coverage_256_clear() -> View {
    TabView::new().into()
}
fn coverage_257_set() -> View {
    TabView::new().on_selection_changed(|_| {}).into()
}
fn coverage_257_clear() -> View {
    TabView::new().into()
}
fn coverage_258_set() -> View {
    TabView::new().on_close_requested(|_| {}).into()
}
fn coverage_258_clear() -> View {
    TabView::new().into()
}
fn coverage_259_set() -> View {
    TabView::new().on_reordered(|_| {}).into()
}
fn coverage_259_clear() -> View {
    TabView::new().into()
}
fn coverage_260_set() -> View {
    TabViewItem::new().is_closable(true).into()
}
fn coverage_260_clear() -> View {
    TabViewItem::new().into()
}
fn coverage_261_set() -> View {
    TabViewItem::new().header("coverage").into()
}
fn coverage_261_clear() -> View {
    TabViewItem::new().into()
}
fn coverage_262_set() -> View {
    TabViewItem::new().tag("coverage").into()
}
fn coverage_262_clear() -> View {
    TabViewItem::new().into()
}
fn coverage_263_set() -> View {
    TeachingTip::new().title("coverage").into()
}
fn coverage_263_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_264_set() -> View {
    TeachingTip::new().subtitle("coverage").into()
}
fn coverage_264_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_265_set() -> View {
    TeachingTip::new().is_open(false).into()
}
fn coverage_265_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_266_set() -> View {
    TeachingTip::new().is_light_dismiss_enabled(true).into()
}
fn coverage_266_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_267_set() -> View {
    TeachingTip::new()
        .preferred_placement(TeachingTipPlacementMode::Top)
        .into()
}
fn coverage_267_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_268_set() -> View {
    TeachingTip::new().action_button_content("coverage").into()
}
fn coverage_268_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_269_set() -> View {
    TeachingTip::new().close_button_content("coverage").into()
}
fn coverage_269_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_270_set() -> View {
    TeachingTip::new().on_closed(|| {}).into()
}
fn coverage_270_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_271_set() -> View {
    TeachingTip::new().on_action_button_click(|| {}).into()
}
fn coverage_271_clear() -> View {
    TeachingTip::new().into()
}
fn coverage_272_set() -> View {
    DropDownButton::new().is_enabled(true).into()
}
fn coverage_272_clear() -> View {
    DropDownButton::new().into()
}
fn coverage_273_set() -> View {
    DropDownButton::new().on_click(|| {}).into()
}
fn coverage_273_clear() -> View {
    DropDownButton::new().into()
}
fn coverage_274_set() -> View {
    AppBarButton::new().label("coverage").into()
}
fn coverage_274_clear() -> View {
    AppBarButton::new().into()
}
fn coverage_275_set() -> View {
    AppBarButton::new().is_enabled(true).into()
}
fn coverage_275_clear() -> View {
    AppBarButton::new().into()
}
fn coverage_276_set() -> View {
    AppBarButton::new().icon(Symbol::Previous).into()
}
fn coverage_276_clear() -> View {
    AppBarButton::new().into()
}
fn coverage_277_set() -> View {
    AppBarButton::new().on_click(|| {}).into()
}
fn coverage_277_clear() -> View {
    AppBarButton::new().into()
}
fn coverage_278_set() -> View {
    MenuBarItem::new().title("coverage").into()
}
fn coverage_278_clear() -> View {
    MenuBarItem::new().into()
}
fn coverage_279_set() -> View {
    SplitButton::new().is_enabled(true).into()
}
fn coverage_279_clear() -> View {
    SplitButton::new().into()
}
fn coverage_280_set() -> View {
    SplitButton::new().on_click(|| {}).into()
}
fn coverage_280_clear() -> View {
    SplitButton::new().into()
}
fn coverage_281_set() -> View {
    ColorPicker::new().color(Color::rgb(1, 2, 3)).into()
}
fn coverage_281_clear() -> View {
    ColorPicker::new().into()
}
fn coverage_282_set() -> View {
    ColorPicker::new().is_alpha_enabled(true).into()
}
fn coverage_282_clear() -> View {
    ColorPicker::new().into()
}
fn coverage_283_set() -> View {
    ColorPicker::new().is_hex_input_visible(true).into()
}
fn coverage_283_clear() -> View {
    ColorPicker::new().into()
}
fn coverage_284_set() -> View {
    ColorPicker::new().is_color_slider_visible(true).into()
}
fn coverage_284_clear() -> View {
    ColorPicker::new().into()
}
fn coverage_285_set() -> View {
    ColorPicker::new()
        .is_color_channel_text_input_visible(true)
        .into()
}
fn coverage_285_clear() -> View {
    ColorPicker::new().into()
}
fn coverage_286_set() -> View {
    ColorPicker::new().is_enabled(true).into()
}
fn coverage_286_clear() -> View {
    ColorPicker::new().into()
}
fn coverage_287_set() -> View {
    ColorPicker::new().on_color_changed(|_| {}).into()
}
fn coverage_287_clear() -> View {
    ColorPicker::new().into()
}
fn coverage_288_set() -> View {
    DatePicker::new().day_visible(true).into()
}
fn coverage_288_clear() -> View {
    DatePicker::new().into()
}
fn coverage_289_set() -> View {
    DatePicker::new().month_visible(true).into()
}
fn coverage_289_clear() -> View {
    DatePicker::new().into()
}
fn coverage_290_set() -> View {
    DatePicker::new().year_visible(true).into()
}
fn coverage_290_clear() -> View {
    DatePicker::new().into()
}
fn coverage_291_set() -> View {
    DatePicker::new().is_enabled(true).into()
}
fn coverage_291_clear() -> View {
    DatePicker::new().into()
}
fn coverage_292_set() -> View {
    DatePicker::new().on_selected_date_changed(|_| {}).into()
}
fn coverage_292_clear() -> View {
    DatePicker::new().into()
}
fn coverage_293_set() -> View {
    TimePicker::new().is_enabled(true).into()
}
fn coverage_293_clear() -> View {
    TimePicker::new().into()
}
fn coverage_294_set() -> View {
    TimePicker::new().minute_increment(1).into()
}
fn coverage_294_clear() -> View {
    TimePicker::new().into()
}
fn coverage_295_set() -> View {
    TimePicker::new().clock_identifier("24HourClock").into()
}
fn coverage_295_clear() -> View {
    TimePicker::new().into()
}
fn coverage_296_set() -> View {
    TimePicker::new().on_selected_time_changed(|_| {}).into()
}
fn coverage_296_clear() -> View {
    TimePicker::new().into()
}
fn coverage_297_set() -> View {
    CalendarDatePicker::new()
        .placeholder_text("coverage")
        .into()
}
fn coverage_297_clear() -> View {
    CalendarDatePicker::new().into()
}
fn coverage_298_set() -> View {
    CalendarDatePicker::new().is_today_highlighted(true).into()
}
fn coverage_298_clear() -> View {
    CalendarDatePicker::new().into()
}
fn coverage_299_set() -> View {
    CalendarDatePicker::new().is_calendar_open(false).into()
}
fn coverage_299_clear() -> View {
    CalendarDatePicker::new().into()
}
fn coverage_300_set() -> View {
    CalendarDatePicker::new().is_enabled(true).into()
}
fn coverage_300_clear() -> View {
    CalendarDatePicker::new().into()
}
fn coverage_301_set() -> View {
    CalendarDatePicker::new().on_date_changed(|_| {}).into()
}
fn coverage_301_clear() -> View {
    CalendarDatePicker::new().into()
}
fn coverage_302_set() -> View {
    CalendarView::new().is_today_highlighted(true).into()
}
fn coverage_302_clear() -> View {
    CalendarView::new().into()
}
fn coverage_303_set() -> View {
    CalendarView::new().is_group_label_visible(true).into()
}
fn coverage_303_clear() -> View {
    CalendarView::new().into()
}
fn coverage_304_set() -> View {
    CalendarView::new().is_enabled(true).into()
}
fn coverage_304_clear() -> View {
    CalendarView::new().into()
}
fn coverage_305_set() -> View {
    CalendarView::new().on_selected_dates_changed(|| {}).into()
}
fn coverage_305_clear() -> View {
    CalendarView::new().into()
}
fn coverage_306_set() -> View {
    ListViewItem::new().tag("coverage").into()
}
fn coverage_306_clear() -> View {
    ListViewItem::new().into()
}
fn coverage_307_set() -> View {
    GridView::new().can_drag_items(true).into()
}
fn coverage_307_clear() -> View {
    GridView::new().into()
}
fn coverage_308_set() -> View {
    GridView::new().can_reorder_items(true).into()
}
fn coverage_308_clear() -> View {
    GridView::new().into()
}
fn coverage_309_set() -> View {
    GridView::new().allow_drop(true).into()
}
fn coverage_309_clear() -> View {
    GridView::new().into()
}
fn coverage_310_set() -> View {
    GridView::new().selected_index(Some(0)).into()
}
fn coverage_310_clear() -> View {
    GridView::new().into()
}
fn coverage_311_set() -> View {
    GridView::new().on_selection_changed(|_| {}).into()
}
fn coverage_311_clear() -> View {
    GridView::new().into()
}
fn coverage_312_set() -> View {
    GridView::new().on_reordered(|_| {}).into()
}
fn coverage_312_clear() -> View {
    GridView::new().into()
}
fn coverage_313_set() -> View {
    GridViewItem::new().tag("coverage").into()
}
fn coverage_313_clear() -> View {
    GridViewItem::new().into()
}
fn coverage_314_set() -> View {
    RichEditBox::new().text("coverage").into()
}
fn coverage_314_clear() -> View {
    RichEditBox::new().into()
}
fn coverage_315_set() -> View {
    RichEditBox::new().placeholder_text("coverage").into()
}
fn coverage_315_clear() -> View {
    RichEditBox::new().into()
}
fn coverage_316_set() -> View {
    RichEditBox::new().is_read_only(true).into()
}
fn coverage_316_clear() -> View {
    RichEditBox::new().into()
}
fn coverage_317_set() -> View {
    RichEditBox::new().is_enabled(true).into()
}
fn coverage_317_clear() -> View {
    RichEditBox::new().into()
}
fn coverage_318_set() -> View {
    RichEditBox::new().on_text_changed(|_| {}).into()
}
fn coverage_318_clear() -> View {
    RichEditBox::new().into()
}
fn coverage_319_set() -> View {
    RichTextBlock::new()
        .paragraphs(RichText::new([RichTextParagraph::new([
            RichTextInline::Run(RichTextRun::plain("coverage")),
        ])]))
        .into()
}
fn coverage_319_clear() -> View {
    RichTextBlock::new().into()
}
fn coverage_320_set() -> View {
    RichTextBlock::new().is_text_selection_enabled(true).into()
}
fn coverage_320_clear() -> View {
    RichTextBlock::new().into()
}
fn coverage_321_set() -> View {
    RichTextBlock::new()
        .text_wrapping(TextWrapping::Wrap)
        .into()
}
fn coverage_321_clear() -> View {
    RichTextBlock::new().into()
}
fn coverage_322_set() -> View {
    RichTextBlock::new().font_size(1.0).into()
}
fn coverage_322_clear() -> View {
    RichTextBlock::new().into()
}
