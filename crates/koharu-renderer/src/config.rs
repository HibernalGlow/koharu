use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[serde(default)]
pub struct TypesettingConfig {
    pub font_families: Vec<String>,
    /// 自动适配时的**可读性下限**：排版在 `[min_font_size, 自动上限]` 里找「能放下的最大字号」，
    /// 放不下就掉到这里（宁可溢出也不缩到看不清）。默认 9.0 与上游一致。
    pub min_font_size: f32,
}

impl Default for TypesettingConfig {
    fn default() -> Self {
        Self {
            font_families: vec!["CCWildWords".to_owned(), "Adobe 黑体 Std".to_owned()],
            min_font_size: crate::renderer::MINIMUM_FONT_SIZE,
        }
    }
}

impl TypesettingConfig {
    pub fn load() -> anyhow::Result<koharu_config::Config<Self>> {
        koharu_config::load("typesetting")
    }
}
