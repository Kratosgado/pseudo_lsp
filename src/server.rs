// use regex::Regex;
use std::fs;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};

#[derive(Debug)]
pub struct PseudoServer {
    pub client: Client,
    // hasConfigurationCapability: bool,
    // hasWorkspaceFolderCapability: bool,
    // hasDiagnosticRelatedInformationCapability: bool,
}

#[tower_lsp::async_trait]
impl LanguageServer for PseudoServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::INCREMENTAL),
                    will_save: Some(false),
                    will_save_wait_until: Some(false),
                    ..TextDocumentSyncOptions::default()
                },
            )),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(vec![".".to_string()]),
                ..CompletionOptions::default()
            }),
            diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
                inter_file_dependencies: false,
                workspace_diagnostics: false,
                ..DiagnosticOptions::default()
            })),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(false),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                ..WorkspaceServerCapabilities::default()
            }),
            ..ServerCapabilities::default()
        };
        let result = InitializeResult {
            capabilities,
            ..InitializeResult::default()
        };
        Ok(result)
    }
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "pseudo_lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem {
                label: "Hello".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("Some detail".to_string()),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "Bye".to_string(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("Some detail".to_string()),
                ..CompletionItem::default()
            },
        ])))
    }
    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }))
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        let text = params.content_changes[0].text.clone();

        let mut diagnostics = Vec::new();

        for (i, line) in text.lines().enumerate() {
            if line.contains("AAA") {
                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position {
                            line: i as u32,
                            character: line.find("AAA").unwrap() as u32,
                        },
                        end: Position {
                            line: i as u32,
                            character: line.find("AAA").unwrap() as u32 + 3,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: "Avoid using 'AAA'".to_string(),
                    ..Diagnostic::default()
                };

                diagnostics.push(diagnostic);
            }
        }

        self.client
            .publish_diagnostics(uri, diagnostics.clone(), None)
            .await;
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let uri = params.text_document.uri;
        let text = fs::read_to_string(uri.path()).unwrap();

        let mut diagnostics = Vec::new();

        for (i, line) in text.lines().enumerate() {
            if line.contains("AAA") {
                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position {
                            line: i as u32,
                            character: line.find("AAA").unwrap() as u32,
                        },
                        end: Position {
                            line: i as u32,
                            character: line.find("AAA").unwrap() as u32 + 3,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: "Avoid using 'AAA'".to_string(),
                    ..Diagnostic::default()
                };

                diagnostics.push(diagnostic);
            }
        }

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    items: diagnostics.clone(),
                    ..Default::default()
                },
                ..Default::default()
            }),
        ))
    }
}
