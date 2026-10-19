# ContentDialog theme

Run `cargo run -p reactor-content-dialog-theme`.

Choose Light, Dark, or System, then open the dialog. The dialog follows its window without a
separate theme setting. Use the buttons inside the dialog to change the window theme while it is
open; its background, text, and buttons should update together.

Close the dialog, choose another theme, and reopen it to check that the retained dialog follows
the current theme. System uses the window's actual color scheme.
