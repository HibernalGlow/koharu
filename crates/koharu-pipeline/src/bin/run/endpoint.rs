//! 只加「翻译后端」这一档，不动上游的管线本身。
//!
//! 上游 `run.rs` 把 `Provider::Local` 写死了，于是无头 CLI 只能用它编译期钉死的
//! 那张模型表（连绝对路径都不认）。这里把那一处挑成可选项：
//!
//! - `local`：保持上游行为（进程内 llama.cpp + catalog 里的 id，一字未改）
//! - `openai-compatible`：走任意 OpenAI 兼容端点 —— 本机 llama-server / LM Studio
//!   也好，在线 API（DeepSeek 等）也好，都只是 base_url + model 两个字符串
//!
//! 单独成文件是为了让 `run.rs` 的改动只剩「挂载点」，将来和上游同步时冲突面最小。
//! key 的来路见 `koharu-secrets`：环境变量优先，其次才是系统钥匙串。

use anyhow::{Result, bail};
use clap::ValueEnum;
use koharu_translator::{ModelSelection, Provider, ProvidersConfig};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum TranslationBackend {
    /// 上游行为：进程内 llama.cpp，模型 id 取自编译期钉死的 catalog。
    Local,
    /// 任意 OpenAI 兼容端点：`--base-url` + `--llm <端点上的模型名>`。
    #[value(name = "openai-compatible")]
    OpenAiCompatible,
}

/// 翻译模型选择。`local` 那一支与上游逐字相同（含 `vision: true` / `reasoning: true`）。
pub(crate) fn selection(
    backend: TranslationBackend,
    model: &str,
    vision: bool,
) -> ModelSelection {
    match backend {
        TranslationBackend::Local => ModelSelection {
            provider: Provider::Local,
            model: Some(model.to_owned()),
            quantization: None,
            vision: true,
            reasoning: true,
        },
        TranslationBackend::OpenAiCompatible => ModelSelection {
            provider: Provider::OpenAiCompatible,
            model: Some(model.to_owned()),
            quantization: None,
            // 纯文本端点（DeepSeek 这类）收到图片会直接报错，所以默认不发画面；
            // 端点那边是多模态模型时用 `--vision` 打开。
            vision,
            reasoning: false,
        },
    }
}

/// providers 配置。`local` 那一支仍是上游的 `ProvidersConfig::default()`。
pub(crate) fn providers(
    backend: TranslationBackend,
    base_url: Option<&str>,
) -> Result<ProvidersConfig> {
    let mut providers = ProvidersConfig::default();
    if backend == TranslationBackend::OpenAiCompatible {
        // 明确报错而不是悄悄回落到默认的 localhost:11434 —— 静默回落会让人以为
        // 「用的就是我在线的那个端点」，实际打去本机一个不存在的服务。
        let Some(url) = base_url else {
            bail!("--provider openai-compatible 必须同时给 --base-url（例如 https://api.deepseek.com/v1）");
        };
        providers.openai_compatible.base_url = Some(
            url.parse()
                .map_err(|error| anyhow::anyhow!("--base-url 不是合法 URL（{url}）：{error}"))?,
        );
    }
    Ok(providers)
}
