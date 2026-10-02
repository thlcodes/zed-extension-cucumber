use std::{env, fs};

use zed::lsp::{Completion, Symbol};
use zed::settings::LspSettings;
use zed::{CodeLabel, CodeLabelSpan, Worktree};
use zed_extension_api as zed;

struct CucumberExtension {
    did_find_server: bool,
}

const SERVER_BINARY: &str = "cucumber-language-server";
const SERVER_PATH: &str =
    "node_modules/@cucumber/language-server/bin/cucumber-language-server.cjs";
const PACKAGE_NAME: &str = "@cucumber/language-server";

/// Step keywords mapped to their tree-sitter highlight group.
const STEP_KEYWORDS: &[(&str, &str)] = &[
    ("Given ", "keyword.import"),
    ("When ", "function"),
    ("Then ", "type"),
    ("And ", "keyword"),
    ("But ", "keyword"),
    ("* ", "keyword"),
];

/// Structural keywords used in document symbols.
const SYMBOL_KEYWORDS: &[&str] = &[
    "Scenario Outline",
    "Scenario Template",
    "Scenario",
    "Feature",
    "Background",
    "Rule",
    "Examples",
    "Scenarios",
];

impl CucumberExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, id: &zed::LanguageServerId) -> zed::Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for CucumberExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_args = vec!["--stdio".into()];
        let (command, args) = match worktree.which(SERVER_BINARY) {
            Some(command) => (command, lsp_args),
            None => {
                let script_path = self.server_script_path(language_server_id)?;
                let mut args = lsp_args.clone();
                args.insert(
                    0,
                    env::current_dir()
                        .unwrap()
                        .join(&script_path)
                        .to_string_lossy()
                        .to_string(),
                );
                (zed::node_binary_path()?, args)
            }
        };
        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree("cucumber", worktree)
            .ok()
            .and_then(|s| s.initialization_options.clone()))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree("cucumber", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        let mut config = match settings {
            zed::serde_json::Value::Object(map) => map,
            _ => zed::serde_json::Map::new(),
        };

        if !config.contains_key("features") {
            config.insert(
                "features".to_string(),
                zed::serde_json::json!([
                    "features/**/*.feature",
                    "src/test/**/*.feature",
                    "tests/**/*.feature",
                    "**/*.feature"
                ]),
            );
        }

        if !config.contains_key("glue") {
            config.insert(
                "glue".to_string(),
                zed::serde_json::json!([
                    "features/**/*.{ts,js,rb,py,java,kt}",
                    "src/test/**/*.{java,kt}",
                    "step_definitions/**/*.{ts,js,rb,py}",
                    "**/*.steps.{ts,js,rb,py}"
                ]),
            );
        }

        Ok(Some(zed::serde_json::json!({
            "cucumber": config
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        let label = &completion.label;

        for &(keyword, highlight) in STEP_KEYWORDS {
            if let Some(rest) = label.strip_prefix(keyword) {
                return Some(CodeLabel {
                    code: label.clone(),
                    spans: vec![
                        CodeLabelSpan::literal(
                            keyword.trim_end(),
                            Some(highlight.to_string()),
                        ),
                        CodeLabelSpan::literal(" ", None),
                        CodeLabelSpan::literal(rest, None),
                    ],
                    filter_range: (0..label.len()).into(),
                });
            }
        }

        None
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        let name = &symbol.name;

        if name.is_empty() || !SYMBOL_KEYWORDS.iter().any(|kw| name.starts_with(kw)) {
            return None;
        }

        for keyword in SYMBOL_KEYWORDS {
            if let Some(rest) = name.strip_prefix(keyword) {
                if let Some(title) = rest.strip_prefix(": ") {
                    return Some(CodeLabel {
                        code: name.clone(),
                        spans: vec![
                            CodeLabelSpan::literal(*keyword, Some("keyword".to_string())),
                            CodeLabelSpan::literal(": ", None),
                            CodeLabelSpan::literal(title, None),
                        ],
                        filter_range: (0..name.len()).into(),
                    });
                }

                if rest.is_empty() {
                    return Some(CodeLabel {
                        code: name.clone(),
                        spans: vec![CodeLabelSpan::literal(
                            *keyword,
                            Some("keyword".to_string()),
                        )],
                        filter_range: (0..name.len()).into(),
                    });
                }
            }
        }

        None
    }
}

zed::register_extension!(CucumberExtension);
