//! 诊断类型定义
//!
//! 定义诊断、诊断严重级别等类型

use crate::symbol::Location;

/// 诊断严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// 诊断相关信息
#[derive(Debug, Clone)]
pub struct DiagnosticRelatedInfo {
    pub location: Location,
    pub message: String,
}

/// 诊断
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 诊断范围
    pub range: Location,
    /// 严重级别
    pub severity: DiagnosticSeverity,
    /// 诊断代码
    pub code: Option<String>,
    /// 来源（如 "babel-lsp-sv"）
    pub source: String,
    /// 诊断消息
    pub message: String,
    /// 相关信息
    pub related_info: Vec<DiagnosticRelatedInfo>,
}

impl Diagnostic {
    /// 创建新诊断
    pub fn new(range: Location, severity: DiagnosticSeverity, message: String) -> Self {
        Self {
            range,
            severity,
            code: None,
            source: "babel-lsp".to_string(),
            message,
            related_info: vec![],
        }
    }

    /// 创建错误诊断
    pub fn error(range: Location, message: String) -> Self {
        Self::new(range, DiagnosticSeverity::Error, message)
    }

    /// 创建警告诊断
    pub fn warning(range: Location, message: String) -> Self {
        Self::new(range, DiagnosticSeverity::Warning, message)
    }

    /// 设置诊断代码
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// 设置来源
    pub fn with_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    /// 添加相关信息
    pub fn add_related_info(mut self, location: Location, message: String) -> Self {
        self.related_info
            .push(DiagnosticRelatedInfo { location, message });
        self
    }
}

/// 诊断缓存
pub struct DiagnosticCache {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCache {
    pub fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn by_file(&self, uri: &str) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.range.uri == uri)
            .collect()
    }
}

impl Default for DiagnosticCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Position;

    #[test]
    fn test_diagnostic_creation() {
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(0, 10),
        };
        let diag = Diagnostic::error(loc, "syntax error".to_string());
        assert_eq!(diag.severity, DiagnosticSeverity::Error);
        assert_eq!(diag.message, "syntax error");
    }

    #[test]
    fn test_diagnostic_cache() {
        let mut cache = DiagnosticCache::new();
        let loc = Location {
            uri: "file:///test.sv".to_string(),
            start: Position::new(0, 0),
            end: Position::new(0, 10),
        };
        cache.add(Diagnostic::error(loc.clone(), "error 1".to_string()));
        cache.add(Diagnostic::warning(loc, "warning 1".to_string()));
        assert_eq!(cache.diagnostics().len(), 2);
    }
}
