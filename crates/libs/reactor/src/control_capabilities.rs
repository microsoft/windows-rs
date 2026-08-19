macro_rules! native_control_catalog {
    ($callback:ident) => {
        $callback! {
            // Kind => [UIElement, text, enabled, toggle, attachment], [public builders]
            Border => [Ui, NoText, NoEnabled, NoToggle, Content], [],
            BreadcrumbBar => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            AutoSuggestBox => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            Button => [Ui, Text, Enabled, NoToggle, Content], [Enabled, Text],
            AppBarButton => [NoUi, NoText, NoEnabled, NoToggle, None], [],
            AppBarSeparator => [NoUi, NoText, NoEnabled, NoToggle, None], [],
            AppBarToggleButton => [NoUi, NoText, NoEnabled, NoToggle, None], [],
            CommandBar => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            CommandBarFlyout => [NoUi, NoText, NoEnabled, NoToggle, None], [],
            CompositionHost => [Ui, NoText, NoEnabled, NoToggle, None], [],
            #[cfg(feature = "canvas")]
            CanvasImage => [Ui, NoText, NoEnabled, NoToggle, None], [],
            DropDownButton => [Ui, Text, Enabled, NoToggle, Content], [Enabled, Text],
            SplitButton => [Ui, Text, Enabled, NoToggle, Content], [Enabled, Text],
            Flyout => [NoUi, NoText, NoEnabled, NoToggle, Content], [],
            FlipView => [Ui, NoText, Enabled, NoToggle, Items], [Enabled],
            TabView => [Ui, NoText, Enabled, NoToggle, Items], [Enabled],
            TabViewItem => [Ui, NoText, NoEnabled, NoToggle, Content], [],
            SelectorBar => [Ui, NoText, Enabled, NoToggle, Items], [Enabled],
            SelectorBarItem => [Ui, NoText, NoEnabled, NoToggle, None], [],
            MenuFlyout => [NoUi, NoText, NoEnabled, NoToggle, None], [],
            MenuBar => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            HyperlinkButton => [Ui, Text, Enabled, NoToggle, Content], [Enabled, Text],
            Image => [Ui, NoText, NoEnabled, NoToggle, None], [],
            SymbolIcon => [Ui, NoText, NoEnabled, NoToggle, None], [],
            FontIcon => [Ui, NoText, NoEnabled, NoToggle, None], [],
            BitmapIcon => [Ui, NoText, NoEnabled, NoToggle, None], [],
            ImageIcon => [Ui, NoText, NoEnabled, NoToggle, None], [],
            PathIcon => [Ui, NoText, NoEnabled, NoToggle, None], [],
            NavigationView => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            NavigationViewItem => [Ui, NoText, NoEnabled, NoToggle, None], [],
            Rectangle => [Ui, NoText, NoEnabled, NoToggle, None], [],
            Ellipse => [Ui, NoText, NoEnabled, NoToggle, None], [],
            Line => [Ui, NoText, NoEnabled, NoToggle, None], [],
            InfoBadge => [Ui, NoText, NoEnabled, NoToggle, None], [],
            InfoBar => [Ui, NoText, NoEnabled, NoToggle, None], [],
            PersonPicture => [Ui, NoText, NoEnabled, NoToggle, None], [],
            RepeatButton => [Ui, Text, Enabled, NoToggle, Content], [Enabled, Text],
            ToggleButton => [Ui, Text, Enabled, Toggle, Content], [Enabled, Text],
            ToggleSwitch => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            ProgressBar => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            ProgressRing => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            Slider => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            NumberBox => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            RatingControl => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            ColorPicker => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            DatePicker => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            CalendarDatePicker => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            TimePicker => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            CalendarView => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            RichEditBox => [Ui, Text, Enabled, NoToggle, None], [Enabled, Text],
            RichTextBlock => [Ui, NoText, NoEnabled, NoToggle, None], [],
            TreeView => [Ui, NoText, Enabled, NoToggle, None], [Enabled],
            Canvas => [Ui, NoText, NoEnabled, NoToggle, Children], [],
            CheckBox => [Ui, Text, Enabled, Toggle, Content], [Enabled, Text],
            RadioButton => [Ui, Text, Enabled, Toggle, Content], [Enabled, Text],
            PasswordBox => [Ui, Text, Enabled, NoToggle, None], [Enabled, Text],
            Pivot => [Ui, NoText, Enabled, NoToggle, Items], [Enabled],
            PivotItem => [Ui, NoText, NoEnabled, NoToggle, Content], [],
            Grid => [Ui, NoText, NoEnabled, NoToggle, Children], [],
            GridView => [Ui, NoText, NoEnabled, NoToggle, None], [],
            ListBox => [Ui, Text, Enabled, NoToggle, None], [Enabled, Text],
            ComboBox => [Ui, Text, Enabled, NoToggle, None], [Enabled, Text],
            ContentDialog => [Ui, NoText, NoEnabled, NoToggle, HeaderContent], [],
            RadioButtons => [Ui, Text, Enabled, NoToggle, None], [Enabled, Text],
            ListView => [Ui, Text, Enabled, NoToggle, None], [],
            RelativePanel => [Ui, NoText, NoEnabled, NoToggle, Children], [],
            StackPanel => [Ui, NoText, NoEnabled, NoToggle, Children], [],
            #[cfg(feature = "canvas")]
            SwapChainCanvas => [Ui, NoText, NoEnabled, NoToggle, None], [],
            #[cfg(feature = "canvas")]
            SwapChainHost => [Ui, NoText, NoEnabled, NoToggle, None], [],
            #[cfg(feature = "webview")]
            WebViewHost => [Ui, NoText, NoEnabled, NoToggle, None], [],
            TextBlock => [Ui, Text, NoEnabled, NoToggle, None], [Text],
            TextBox => [Ui, Text, Enabled, NoToggle, None], [Enabled, Text],
            Viewbox => [Ui, NoText, NoEnabled, NoToggle, Content], [],
            ScrollViewer => [Ui, NoText, Enabled, NoToggle, Content], [Enabled],
            ScrollView => [Ui, NoText, Enabled, NoToggle, Content], [Enabled],
            SplitView => [Ui, NoText, Enabled, NoToggle, ContentPane], [Enabled],
            Expander => [Ui, NoText, Enabled, NoToggle, HeaderContent], [Enabled],
            TeachingTip => [Ui, NoText, NoEnabled, NoToggle, None], [],
            TitleBar => [Ui, NoText, NoEnabled, NoToggle, ContentPane], [],
            ToolTip => [Ui, NoText, NoEnabled, NoToggle, Content], [],
        }
    };
}

macro_rules! framework_elements {
    ($callback:ident) => {
        $callback! {
            (
                Border,
                ElementKind::Border(border) => &border.props.framework,
                MountedKind::Border(props) => &props.framework
            ),
            (
                Button,
                ElementKind::Button { props, .. }
                    | ElementKind::ButtonFlyout { button: props, .. }
                    | ElementKind::ButtonMenuFlyout { button: props, .. }
                    | ElementKind::ButtonCommandBarFlyout { button: props, .. }
                    => &props.framework,
                MountedKind::Button(props)
                    | MountedKind::ButtonFlyout(props)
                    | MountedKind::ButtonMenuFlyout(props)
                    | MountedKind::ButtonCommandBarFlyout(props)
                    => &props.framework
            ),
            (
                DropDownButton,
                ElementKind::DropDownButton(drop_down) => &drop_down.props.framework,
                MountedKind::DropDownButton(props) | MountedKind::DropDownMenuFlyout(props) => {
                    &props.framework
                }
            ),
            (
                SplitButton,
                ElementKind::SplitButton { props, .. }
                    | ElementKind::SplitButtonFlyout { button: props, .. }
                    => &props.framework,
                MountedKind::SplitButton(props) | MountedKind::SplitButtonFlyout(props) => {
                    &props.framework
                }
            ),
            (
                MenuBar,
                ElementKind::MenuBar(props) => &props.framework,
                MountedKind::MenuBar(props) => &props.framework
            ),
            (
                CommandBar,
                ElementKind::CommandBar(props) => &props.framework,
                MountedKind::CommandBar { framework, .. } => framework
            ),
            (
                HyperlinkButton,
                ElementKind::HyperlinkButton { props, .. } => &props.framework,
                MountedKind::HyperlinkButton(props) => &props.framework
            ),
            (
                Image,
                ElementKind::Image(props) => &props.framework,
                MountedKind::Image { props, .. } => &props.framework
            ),
            (
                Shape,
                ElementKind::Shape(props) => &props.framework,
                MountedKind::Shape(props) => &props.framework
            ),
            (
                RepeatButton,
                ElementKind::RepeatButton { props, .. } => &props.framework,
                MountedKind::RepeatButton(props) => &props.framework
            ),
            (
                ToggleButton,
                ElementKind::ToggleButton { props, .. } => &props.framework,
                MountedKind::ToggleButton(props) => &props.framework
            ),
            (
                ToggleSwitch,
                ElementKind::ToggleSwitch(props) => &props.framework,
                MountedKind::ToggleSwitch(props) => &props.framework
            ),
            (
                InfoBadge,
                ElementKind::InfoBadge(props) => &props.framework,
                MountedKind::InfoBadge(props) => &props.framework
            ),
            (
                InfoBar,
                ElementKind::InfoBar(props) => &props.framework,
                MountedKind::InfoBar(props) => &props.framework
            ),
            (
                PersonPicture,
                ElementKind::PersonPicture(props) => &props.framework,
                MountedKind::PersonPicture(props) => &props.framework
            ),
            (
                ProgressBar,
                ElementKind::ProgressBar(props) => &props.framework,
                MountedKind::ProgressBar(props) => &props.framework
            ),
            (
                ProgressRing,
                ElementKind::ProgressRing(props) => &props.framework,
                MountedKind::ProgressRing(props) => &props.framework
            ),
            (
                Slider,
                ElementKind::Slider(props) => &props.framework,
                MountedKind::Slider(props) => &props.framework
            ),
            (
                NumberBox,
                ElementKind::NumberBox(props) => &props.framework,
                MountedKind::NumberBox(props) => &props.framework
            ),
            (
                RatingControl,
                ElementKind::RatingControl(props) => &props.framework,
                MountedKind::RatingControl(props) => &props.framework
            ),
            (
                ColorPicker,
                ElementKind::ColorPicker(props) => &props.framework,
                MountedKind::ColorPicker(props) => &props.framework
            ),
            (
                DatePicker,
                ElementKind::DatePicker(props) => &props.framework,
                MountedKind::DatePicker(props) => &props.framework
            ),
            (
                CalendarDatePicker,
                ElementKind::CalendarDatePicker(props) => &props.framework,
                MountedKind::CalendarDatePicker(props) => &props.framework
            ),
            (
                TimePicker,
                ElementKind::TimePicker(props) => &props.framework,
                MountedKind::TimePicker(props) => &props.framework
            ),
            (
                CalendarView,
                ElementKind::CalendarView(props) => &props.framework,
                MountedKind::CalendarView(props) => &props.framework
            ),
            (
                NavigationView,
                ElementKind::NavigationView(value) => &value.props.framework,
                MountedKind::NavigationView(props) => &props.framework
            ),
            (
                FlipView,
                ElementKind::FlipView(props) => &props.framework,
                MountedKind::FlipView(props) => &props.framework
            ),
            (
                TabView,
                ElementKind::TabView(props) => &props.framework,
                MountedKind::TabView(props) => &props.framework
            ),
            (
                SelectorBar,
                ElementKind::SelectorBar(props) => &props.framework,
                MountedKind::SelectorBar(props) => &props.framework
            ),
            (
                BreadcrumbBar,
                ElementKind::BreadcrumbBar(props) => &props.framework,
                MountedKind::BreadcrumbBar(props) => &props.framework
            ),
            (
                AutoSuggestBox,
                ElementKind::AutoSuggestBox(props) => &props.framework,
                MountedKind::AutoSuggestBox(props) => &props.framework
            ),
            (
                Pivot,
                ElementKind::Pivot(props) => &props.framework,
                MountedKind::Pivot(props) => &props.framework
            ),
            (
                ListBox,
                ElementKind::ListBox(props) => &props.framework,
                MountedKind::ListBox(props) => &props.framework
            ),
            (
                ComboBox,
                ElementKind::ComboBox(props) => &props.framework,
                MountedKind::ComboBox(props) => &props.framework
            ),
            (
                RadioButtons,
                ElementKind::RadioButtons(props) => &props.framework,
                MountedKind::RadioButtons(props) => &props.framework
            ),
            (
                CheckBox,
                ElementKind::CheckBox { props, .. } => &props.framework,
                MountedKind::CheckBox(props) => &props.framework
            ),
            (
                RadioButton,
                ElementKind::RadioButton { props, .. } => &props.framework,
                MountedKind::RadioButton(props) => &props.framework
            ),
            (
                PasswordBox,
                ElementKind::PasswordBox(props) => &props.framework,
                MountedKind::PasswordBox(props) => &props.framework
            ),
            (
                TextBlock,
                ElementKind::TextBlock(props) => &props.framework,
                MountedKind::TextBlock(props) => &props.framework
            ),
            (
                TextBox,
                ElementKind::TextBox(props) => &props.framework,
                MountedKind::TextBox(props) => &props.framework
            ),
            (
                RichEditBox,
                ElementKind::RichEditBox(props) => &props.framework,
                MountedKind::RichEditBox(props) => &props.framework
            ),
            (
                RichTextBlock,
                ElementKind::RichTextBlock(props) => &props.framework,
                MountedKind::RichTextBlock(props) => &props.framework
            ),
            (
                TreeView,
                ElementKind::TreeView(props) => &props.framework,
                MountedKind::TreeView(props) => &props.framework
            ),
            (
                StackPanel,
                ElementKind::StackPanel(props) => &props.framework,
                MountedKind::StackPanel(props) => &props.framework
            ),
            (
                Grid,
                ElementKind::Grid(props) => &props.framework,
                MountedKind::Grid(props) => &props.framework
            ),
            (
                Canvas,
                ElementKind::Canvas(props) => &props.framework,
                MountedKind::Canvas(props) => props
            ),
            (
                RelativePanel,
                ElementKind::RelativePanel(props) => &props.framework,
                MountedKind::RelativePanel(props) => props
            ),
            (
                Viewbox,
                ElementKind::Viewbox { props, .. } => &props.framework,
                MountedKind::Viewbox(props) => &props.framework
            ),
            (
                ScrollViewer,
                ElementKind::ScrollViewer { props, .. } => &props.framework,
                MountedKind::ScrollViewer(props) => &props.framework
            ),
            (
                ScrollView,
                ElementKind::ScrollView { props, .. } => &props.framework,
                MountedKind::ScrollView(props) => &props.framework
            ),
            (
                SplitView,
                ElementKind::SplitView(split) => &split.props.framework,
                MountedKind::SplitView(props) => &props.framework
            ),
            (
                Expander,
                ElementKind::Expander(expander) => &expander.props.framework,
                MountedKind::Expander(props) => &props.framework
            ),
            (
                CompositionHost,
                ElementKind::CompositionHost(props) => &props.framework,
                MountedKind::CompositionHost(props) => &props.framework
            ),
            #[cfg(feature = "canvas")]
            (
                CanvasImage,
                ElementKind::CanvasImage(props) => &props.framework,
                MountedKind::CanvasImage(props) => &props.framework
            ),
            #[cfg(feature = "canvas")]
            (
                SwapChainCanvas,
                ElementKind::SwapChainCanvas(props) => &props.framework,
                MountedKind::SwapChainCanvas(props) => &props.framework
            ),
            #[cfg(feature = "canvas")]
            (
                SwapChainHost,
                ElementKind::SwapChainHost(props) => &props.framework,
                MountedKind::SwapChainHost(props) => &props.framework
            ),
            #[cfg(feature = "webview")]
            (
                WebViewHost,
                ElementKind::WebViewHost(props) => &props.framework,
                MountedKind::WebViewHost(props) => &props.framework
            ),
        }
    };
}
