use zed_extension_api::{self as zed, serde_json, settings::LspSettings};

struct InmantaExtension;

impl zed::Extension for InmantaExtension {
    fn new() -> Self {
        InmantaExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let settings = LspSettings::for_worktree("inmanta-language-server", worktree)
            .ok()
            .and_then(|s| s.settings);

        let python_path = settings
            .as_ref()
            .and_then(|s| s.get("pythonPath"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                "Inmanta: please set 'pythonPath' in settings.json under \
                 lsp.inmanta-language-server.settings. \
                 Point it to the Python binary in a virtualenv that has 'inmantals' installed. \
                 Example: ~/.virtualenvs/inmantals/bin/python. \
                 Install with: pip install inmantals"
                    .to_string()
            })?
            .to_string();

        Ok(zed::Command {
            command: python_path,
            args: vec!["-m".into(), "inmantals.pipeserver".into()],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("inmanta-language-server", worktree)
            .ok()
            .and_then(|s| s.settings);

        let default_venv = worktree.root_path();
        let compiler_venv = settings
            .as_ref()
            .and_then(|s| s.get("compilerVenv").or_else(|| s.get("pythonPath")))
            .and_then(|v| v.as_str())
            .map(|p| {
                if let Some(venv) = p
                    .strip_suffix("/bin/python")
                    .or_else(|| p.strip_suffix("/bin/python3"))
                    .or_else(|| p.strip_suffix("/bin/python3.12"))
                {
                    venv.to_string()
                } else {
                    p.to_string()
                }
            })
            .unwrap_or_else(|| default_venv.clone());

        Ok(Some(serde_json::json!({
            "pip": {
                "use_system_config": true
            },
            "compilerVenv": compiler_venv,
            "logFile": "/tmp/inmanta-ls.log"
        })))
    }
}

zed::register_extension!(InmantaExtension);
