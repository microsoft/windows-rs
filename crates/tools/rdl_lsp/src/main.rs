use lsp_server::*;
use lsp_types::*;
use std::{collections::HashMap, time::Duration};
use windows_rdl::Reader;

fn main() {
    let (connection, _io_threads) = Connection::stdio();

    let capabilities = serde_json::json!(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(true),
                })),
                ..Default::default()
            },
        )),
        ..Default::default()
    });

    let initialize_params = connection.initialize(capabilities).unwrap();
    let initialize_params: InitializeParams = serde_json::from_value(initialize_params).unwrap();

    #[allow(clippy::mutable_key_type)]
    let mut pending = HashMap::<Uri, String>::new();
    let mut debug = false;
    let mut shutdown = false;

    if let Some(init_options) = initialize_params.initialization_options
        && let Some(value) = get_debug_setting(&init_options)
    {
        debug = value;
    }

    loop {
        match connection.receiver.recv_timeout(Duration::from_secs(2)) {
            Ok(msg) => {
                if debug {
                    dbg!(&msg);
                }

                match msg {
                    Message::Request(request) => {
                        if request.method == "shutdown" {
                            let response = Response::new_ok(request.id, ());
                            connection.sender.send(Message::Response(response)).unwrap();
                            shutdown = true;
                        }
                    }

                    Message::Notification(notification) => match notification.method.as_str() {
                        "exit" => {
                            if shutdown {
                                break;
                            }
                        }

                        "workspace/didChangeConfiguration" => {
                            let params: DidChangeConfigurationParams =
                                serde_json::from_value(notification.params).unwrap();
                            if let Some(value) = get_debug_setting(&params.settings) {
                                debug = value;
                            }
                        }

                        "textDocument/didChange" => {
                            let params: DidChangeTextDocumentParams =
                                serde_json::from_value(notification.params).unwrap();
                            let uri = params.text_document.uri;
                            pending
                                .insert(uri, params.content_changes.last().unwrap().text.clone());
                        }

                        "textDocument/didOpen" => {
                            let params: DidOpenTextDocumentParams =
                                serde_json::from_value(notification.params).unwrap();
                            let uri = params.text_document.uri;
                            let text = params.text_document.text;
                            pending.remove(&uri);
                            validate_document(&connection, &uri, &text);
                        }

                        "textDocument/didSave" => {
                            let params: DidSaveTextDocumentParams =
                                serde_json::from_value(notification.params).unwrap();
                            let uri = params.text_document.uri;
                            let text = params.text.unwrap();
                            pending.remove(&uri);
                            validate_document(&connection, &uri, &text);
                        }

                        method => {
                            if debug {
                                eprintln!("Unimplemented method: {}", method);
                            }
                        }
                    },
                    Message::Response(_) => {}
                }
            }
            Err(_) => {
                for (uri, text) in pending.drain() {
                    validate_document(&connection, &uri, &text);
                }
            }
        }
    }
}

fn get_debug_setting(settings: &serde_json::Value) -> Option<bool> {
    settings
        .get("windows-rs")
        .and_then(|value| value.get("rdl"))
        .and_then(|value| value.get("lsp"))
        .and_then(|value| value.get("debug"))
        .and_then(|value| value.as_bool())
}

fn validate_document(connection: &Connection, uri: &Uri, text: &str) {
    let temp_path = std::env::temp_dir().join("temp.rdl");
    std::fs::write(&temp_path, text).unwrap();

    let result = Reader::new()
        .input(temp_path.as_os_str().to_str().unwrap())
        .output("NUL")
        .write();

    let diagnostics = match result {
        Ok(_) => vec![],
        Err(e) => vec![Diagnostic {
            range: Range {
                start: Position {
                    line: (e.line.saturating_sub(1)) as u32,
                    character: (e.column.saturating_sub(1)) as u32,
                },
                end: Position {
                    line: (e.line.saturating_sub(1)) as u32,
                    character: 100,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            message: e.message.clone(),
            source: Some("windows-rdl".to_string()),
            ..Default::default()
        }],
    };

    let params = PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics,
        version: None,
    };

    let notification = Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(params).unwrap(),
    };

    connection
        .sender
        .send(Message::Notification(notification))
        .unwrap();
}
