//! 日志配置
//!
//! tracing 双模式：Normal（文件） / Debug（终端）

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// 日志模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogMode {
    /// 正常模式：仅写入文件
    Normal,
    /// 调试模式：终端 + 文件
    Debug,
}

/// 日志配置
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: String,
    /// 日志文件路径
    pub file_path: Option<String>,
    /// 日志模式
    pub mode: LogMode,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: None,
            mode: LogMode::Normal,
        }
    }
}

/// 初始化日志系统
pub fn init_logging(config: &LoggingConfig) -> anyhow::Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.level));

    match config.mode {
        LogMode::Normal => {
            // 仅文件输出
            if let Some(path) = &config.file_path {
                let file_appender = tracing_appender::rolling::daily(path, "babel-lsp.log");
                let file_layer = fmt::layer().with_writer(file_appender).with_ansi(false);

                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(file_layer)
                    .try_init()
                    .map_err(|e| anyhow::anyhow!("Failed to init logging: {}", e))?;
            } else {
                // 无文件路径时使用标准输出
                tracing_subscriber::fmt()
                    .with_env_filter(env_filter)
                    .try_init()
                    .map_err(|e| anyhow::anyhow!("Failed to init logging: {}", e))?;
            }
        },
        LogMode::Debug => {
            // 终端 + 文件（双输出）
            let stdout_layer = fmt::layer().with_writer(std::io::stderr).with_ansi(true);

            if let Some(path) = &config.file_path {
                let file_appender = tracing_appender::rolling::daily(path, "babel-lsp.log");
                let file_layer = fmt::layer().with_writer(file_appender).with_ansi(false);

                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(stdout_layer)
                    .with(file_layer)
                    .try_init()
                    .map_err(|e| anyhow::anyhow!("Failed to init logging: {}", e))?;
            } else {
                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(stdout_layer)
                    .try_init()
                    .map_err(|e| anyhow::anyhow!("Failed to init logging: {}", e))?;
            }
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.mode, LogMode::Normal);
        assert!(config.file_path.is_none());
    }

    #[test]
    fn test_log_mode_variants() {
        assert_eq!(LogMode::Normal, LogMode::Normal);
        assert_eq!(LogMode::Debug, LogMode::Debug);
        assert_ne!(LogMode::Normal, LogMode::Debug);
    }

    #[test]
    fn test_log_mode_debug_trait() {
        let mode = LogMode::Debug;
        let debug_str = format!("{:?}", mode);
        assert!(debug_str.contains("Debug"));
    }

    #[test]
    fn test_log_mode_copy_trait() {
        let mode1 = LogMode::Normal;
        let mode2 = mode1; // Copy
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_logging_config_custom() {
        let config = LoggingConfig {
            level: "debug".to_string(),
            file_path: Some("/var/log".to_string()),
            mode: LogMode::Debug,
        };
        assert_eq!(config.level, "debug");
        assert_eq!(config.file_path, Some("/var/log".to_string()));
        assert_eq!(config.mode, LogMode::Debug);
    }

    #[test]
    fn test_logging_config_clone() {
        let config = LoggingConfig {
            level: "warn".to_string(),
            file_path: Some("/tmp/logs".to_string()),
            mode: LogMode::Normal,
        };
        let cloned = config.clone();
        assert_eq!(config.level, cloned.level);
        assert_eq!(config.file_path, cloned.file_path);
        assert_eq!(config.mode, cloned.mode);
    }

    #[test]
    fn test_logging_config_with_different_levels() {
        for level in &["error", "warn", "info", "debug", "trace"] {
            let config = LoggingConfig {
                level: level.to_string(),
                file_path: None,
                mode: LogMode::Normal,
            };
            assert_eq!(config.level, *level);
        }
    }

    #[test]
    fn test_logging_config_mode_variants() {
        let normal_config = LoggingConfig {
            level: "info".to_string(),
            file_path: None,
            mode: LogMode::Normal,
        };
        assert_eq!(normal_config.mode, LogMode::Normal);

        let debug_config = LoggingConfig {
            level: "debug".to_string(),
            file_path: None,
            mode: LogMode::Debug,
        };
        assert_eq!(debug_config.mode, LogMode::Debug);
    }

    /// 验证 init_logging 在 Normal 模式（无文件路径）不 panic
    /// 注意：tracing subscriber 全局只能初始化一次
    #[test]
    fn test_init_logging_normal_no_file() {
        let config = LoggingConfig {
            level: "info".to_string(),
            file_path: None,
            mode: LogMode::Normal,
        };
        // 验证函数可执行，不 panic
        let _ = init_logging(&config);
    }

    /// 验证 init_logging 在 Debug 模式（无文件路径）不 panic
    #[test]
    fn test_init_logging_debug_no_file() {
        let config = LoggingConfig {
            level: "debug".to_string(),
            file_path: None,
            mode: LogMode::Debug,
        };
        let _ = init_logging(&config);
    }

    /// 验证 EnvFilter 默认行为
    #[test]
    fn test_env_filter_fallback() {
        let level = "warn";
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
        // 验证 filter 创建成功
        let _ = env_filter;
    }

    /// 验证配置组合：Debug 模式 + 文件路径
    #[test]
    fn test_logging_config_debug_with_file() {
        let config = LoggingConfig {
            level: "debug".to_string(),
            file_path: Some("/tmp/babel-lsp".to_string()),
            mode: LogMode::Debug,
        };
        assert_eq!(config.mode, LogMode::Debug);
        assert!(config.file_path.is_some());
    }

    /// 验证配置组合：Normal 模式 + 文件路径
    #[test]
    fn test_logging_config_normal_with_file() {
        let config = LoggingConfig {
            level: "info".to_string(),
            file_path: Some("/var/log/babel-lsp".to_string()),
            mode: LogMode::Normal,
        };
        assert_eq!(config.mode, LogMode::Normal);
        assert!(config.file_path.is_some());
    }

    /// 验证 init_logging Normal 模式 + 文件路径
    #[test]
    fn test_init_logging_normal_with_file() {
        use tempfile::tempdir;
        let temp_dir = tempdir().unwrap();
        let config = LoggingConfig {
            level: "info".to_string(),
            file_path: Some(temp_dir.path().to_string_lossy().to_string()),
            mode: LogMode::Normal,
        };
        // try_init 可能因已初始化而失败，但这不是 bug
        let _ = init_logging(&config);
    }

    /// 验证 init_logging Debug 模式 + 文件路径
    #[test]
    fn test_init_logging_debug_with_file() {
        use tempfile::tempdir;
        let temp_dir = tempdir().unwrap();
        let config = LoggingConfig {
            level: "debug".to_string(),
            file_path: Some(temp_dir.path().to_string_lossy().to_string()),
            mode: LogMode::Debug,
        };
        let _ = init_logging(&config);
    }
}
