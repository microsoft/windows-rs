# ContentDialog default button

Run `cargo run -p reactor-content-dialog`.

Choose a default button, then press Enter in the dialog. WinUI highlights the default button and
normally focuses it when the dialog opens. The result is shown after the dialog closes; Cancel
returns `ContentDialogResult::None`.

The editor case shows the focus exception: a multiline TextBox receives focus and consumes Enter
to insert a line instead of closing the dialog. Tab moves focus to the buttons.

`ContentDialogButton::None` removes the default-button designation, not the buttons' own keyboard
behavior. A focused button can still respond to Enter.
