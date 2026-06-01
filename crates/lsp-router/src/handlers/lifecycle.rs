//! 生命周期处理器

use tower_lsp::lsp_types::*;

/// 构建服务器能力声明
pub fn build_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(false),
                })),
                ..Default::default()
            },
        )),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![".".to_string(), "(".to_string(), " ".to_string()]),
            resolve_provider: Some(false),
            ..Default::default()
        }),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        document_formatting_provider: Some(OneOf::Left(true)),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_capabilities() {
        let caps = build_server_capabilities();
        assert!(caps.completion_provider.is_some());
        assert!(caps.definition_provider.is_some());
        assert!(caps.hover_provider.is_some());
        assert!(caps.document_formatting_provider.is_some());
    }

    #[test]
    fn test_text_document_sync() {
        let caps = build_server_capabilities();
        match caps.text_document_sync {
            Some(TextDocumentSyncCapability::Options(opts)) => {
                assert_eq!(opts.open_close, Some(true));
                assert_eq!(opts.change, Some(TextDocumentSyncKind::FULL));
            },
            _ => panic!("unexpected sync type"),
        }
    }
}
