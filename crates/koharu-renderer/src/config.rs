use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[serde(default)]
pub struct TypesettingConfig {
    pub font_families: Vec<String>,
    /// 自动适配时的**可读性下限**：排版在 `[min_font_size, 自动上限]` 里找「能放下的最大字号」，
    /// 放不下就掉到这里（宁可溢出也不缩到看不清）。默认 9.0 与上游一致。
    pub min_font_size: f32,
    /// **框外文字的强制字号**：只作用于「不属于任何气泡」的文字（旁白、拟声、裸字）。
    /// 设了它就把这一类文字的自动适配关掉，按这个字号渲染 —— 放不下也照放，
    /// 会压出原来的检测框之外（渲染不裁切）。
    /// 气泡文字不受它影响，仍然自动适配到气泡高度内：对气泡强制字号只会撑爆画面。
    /// `None`（默认）= 上游行为。
    pub font_size: Option<f32>,
}

impl Default for TypesettingConfig {
    fn default() -> Self {
        Self {
            font_families: vec!["CCWildWords".to_owned(), "Adobe 黑体 Std".to_owned()],
            min_font_size: crate::renderer::MINIMUM_FONT_SIZE,
            font_size: None,
        }
    }
}

impl TypesettingConfig {
    pub fn load() -> anyhow::Result<koharu_config::Config<Self>> {
        koharu_config::load("typesetting")
    }
}
