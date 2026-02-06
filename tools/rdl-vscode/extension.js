const vscode = require("vscode");
const { LanguageClient, TransportKind } = require("vscode-languageclient/node");
const path = require("path");
let client;

function activate(context) {
  const serverPath = path.join(__dirname, "tool_rdl_lsp.exe");

  const serverOptions = {
    command: serverPath,
    transport: TransportKind.stdio,
  };

  const outputChannel = vscode.window.createOutputChannel(
    "Windows Metadata Language (RDL) Server",
  );

  const clientOptions = {
    documentSelector: [{ language: "windows-rdl" }],
    outputChannel: outputChannel,
    initializationOptions: () => {
      const config = vscode.workspace.getConfiguration("windows-rs.rdl");
      return {
        "windows-rs": {
          rdl: {
            ...config
          }
        }
      };
    },
    synchronize: { configurationSection: "windows-rs.rdl" },
  };

  client = new LanguageClient(
    "rdl",
    "RDL Language Server",
    serverOptions,
    clientOptions,
  );
  context.subscriptions.push(outputChannel);
  context.subscriptions.push(client.start());
}

function deactivate() {
  if (client) {
    return client.stop();
  }
}

module.exports = { activate, deactivate };
