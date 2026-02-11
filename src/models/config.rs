use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the data directory where ideas and projects are stored
    pub data_dir: PathBuf,

    /// Default output format (json, table, yaml)
    pub default_format: OutputFormat,

    /// Whether to show timestamps by default in table output
    pub show_timestamps: bool,

    /// Maximum number of items to show in list commands (None for no limit)
    pub max_list_items: Option<usize>,

    /// Whether to use colors in output
    pub use_colors: bool,

    /// Default editor for editing ideas/projects
    pub default_editor: Option<String>,

    /// Backup configuration
    pub backup: BackupConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Whether to enable automatic backups
    pub enabled: bool,

    /// Number of backups to keep
    pub max_backups: usize,

    /// Backup interval in hours
    pub interval_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum OutputFormat {
    Json,
    #[default]
    Table,
    Yaml,
}

impl Config {
    pub fn new() -> Self {
        Self {
            data_dir: PathBuf::from("~/.ideavault/data"),
            default_format: OutputFormat::Table,
            show_timestamps: true,
            max_list_items: Some(50),
            use_colors: true,
            default_editor: None,
            backup: BackupConfig {
                enabled: true,
                max_backups: 10,
                interval_hours: 24,
            },
        }
    }

    pub fn with_data_dir<P: Into<PathBuf>>(mut self, data_dir: P) -> Self {
        self.data_dir = data_dir.into();
        self
    }

    pub fn with_format(mut self, format: OutputFormat) -> Self {
        self.default_format = format;
        self
    }

    pub fn with_timestamps(mut self, show: bool) -> Self {
        self.show_timestamps = show;
        self
    }

    pub fn with_max_list_items(mut self, max: Option<usize>) -> Self {
        self.max_list_items = max;
        self
    }

    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }

    pub fn with_editor<S: Into<String>>(mut self, editor: S) -> Self {
        self.default_editor = Some(editor.into());
        self
    }

    pub fn with_backup_config(mut self, backup: BackupConfig) -> Self {
        self.backup = backup;
        self
    }

    /// Get the ideas file path
    pub fn ideas_file(&self) -> PathBuf {
        self.data_dir.join("ideas.json")
    }

    /// Get the projects file path
    pub fn projects_file(&self) -> PathBuf {
        self.data_dir.join("projects.json")
    }

    /// Get the tags file path
    pub fn tags_file(&self) -> PathBuf {
        self.data_dir.join("tags.json")
    }

    /// Get the backup directory path
    pub fn backup_dir(&self) -> PathBuf {
        self.data_dir.join("backups")
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_backups: 10,
            interval_hours: 24,
        }
    }
}
