# test_pickers

Live tests for `windows-pickers`. Each test shows a real Shell dialog on a dedicated STA thread,
while the test thread owns the parent window and processes its messages. The harness finds the
dialog by title, checks its owner and commit button, and then accepts or cancels it.

```sh
cargo test -p test_pickers
```

The tests need an interactive desktop and run one dialog at a time.
