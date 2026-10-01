use std::{env, fs};

use zed::settings::LspSettings;
use zed::Worktree;
use zed_extension_api as zed;

struct CucumberExtension {
    did_find_server: bool,
}

const SERVER_BINARY: &str = "cucumber-language-server";
const SERVER_PATH: &str = "node_modules/@cucumber/language-server/bin/cucumber-language-server.cjs";

/// Starts the server the way its bin does, but with the grammar `.wasm` files
/// taken from the hoisted `node_modules/@cucumber/language-service/dist`.
/// The bin looks for them under `language-server/node_modules/...`, which npm
/// does not create, so every glue file failed to parse and every step was
/// reported as undefined (#12).
const SERVER_LAUNCHER: &str = r#"
const modules = process.argv[1];
const { startStandaloneServer } = require(modules + "/@cucumber/language-server/dist/cjs/src/wasm/startStandaloneServer");
const { NodeFiles } = require(modules + "/@cucumber/language-server/dist/cjs/src/node/NodeFiles");
const { connection } = startStandaloneServer(modules + "/@cucumber/language-service/dist", (rootUri) => new NodeFiles(rootUri));
process.on("unhandledRejection", (reason) => connection.console.error("Unhandled rejection: " + reason));
"#;
const PACKAGE_NAME: &str = "@cucumber/language-server";

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
                self.server_script_path(language_server_id)?;
                let node_modules = env::current_dir()
                    .unwrap()
                    .join("node_modules")
                    .to_string_lossy()
                    .to_string();
                let mut args = vec!["-e".into(), SERVER_LAUNCHER.into(), node_modules];
                args.extend(lsp_args);
                (zed::node_binary_path()?, args)
            }
        };
        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
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

        Ok(Some(zed::serde_json::json!({
            "cucumber": settings
        })))
    }
}

zed::register_extension!(CucumberExtension);
