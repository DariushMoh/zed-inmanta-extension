use zed_extension_api::{self as zed, serde_json, settings::LspSettings};

struct InmantaExtension;

impl InmantaExtension {
    /// Strip single-line (//) and multi-line (/* */) comments from JSONC
    fn strip_comments(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        let mut in_string = false;
        let mut in_line_comment = false;
        let mut in_block_comment = false;

        while let Some(c) = chars.next() {
            if in_line_comment {
                if c == '\n' {
                    in_line_comment = false;
                    result.push(c);
                }
                continue;
            }
            if in_block_comment {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    in_block_comment = false;
                }
                continue;
            }
            if in_string {
                if c == '\\' {
                    result.push(c);
                    if let Some(next) = chars.next() {
                        result.push(next);
                    }
                    continue;
                }
                if c == '"' {
                    in_string = false;
                }
                result.push(c);
                continue;
            }
            if c == '"' {
                in_string = true;
                result.push(c);
                continue;
            }
            if c == '/' {
                match chars.peek() {
                    Some(&'/') => {
                        chars.next();
                        in_line_comment = true;
                        continue;
                    }
                    Some(&'*') => {
                        chars.next();
                        in_block_comment = true;
                        continue;
                    }
                    _ => {}
                }
            }
            result.push(c);
        }
        result
    }

    /// Remove trailing commas before } or ] to handle JSONC trailing commas
    fn strip_trailing_commas(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        for i in 0..len {
            if chars[i] == ',' {
                // Look ahead for next non-whitespace character
                let mut j = i + 1;
                while j < len && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < len && (chars[j] == '}' || chars[j] == ']') {
                    // Skip this trailing comma
                    continue;
                }
            }
            result.push(chars[i]);
        }
        result
    }

    fn python_path(worktree: &zed::Worktree) -> Option<String> {
        // Try LspSettings first (global settings.json)
        if let Some(path) = LspSettings::for_worktree("inmanta-language-server", worktree)
            .ok()
            .and_then(|s| s.settings)
            .and_then(|s| {
                s.get("pythonPath")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
        {
            return Some(path);
        }

        // Fallback: read .zed/settings.json via worktree API, handle JSONC
        if let Ok(content) = worktree.read_text_file(".zed/settings.json") {
            let stripped = Self::strip_comments(&content);
            let stripped = Self::strip_trailing_commas(&stripped);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stripped) {
                if let Some(path) = json
                    .pointer("/lsp/inmanta-language-server/settings/pythonPath")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                {
                    return Some(path);
                }
            }
        }

        None
    }
}

impl zed::Extension for InmantaExtension {
    fn new() -> Self {
        InmantaExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let python_path = Self::python_path(worktree).ok_or_else(|| {
            "Inmanta: please set 'pythonPath' in .zed/settings.json or global settings.json \
             under lsp.inmanta-language-server.settings.\n\
             Example:\n\
             {\n\
               \"lsp\": {\n\
                 \"inmanta-language-server\": {\n\
                   \"settings\": {\n\
                     \"pythonPath\": \"/path/to/venv/bin/python\"\n\
                   }\n\
                 }\n\
               }\n\
             }\n\
             The venv must have inmantals installed: pip install inmantals setuptools wheel"
                .to_string()
        })?;

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
        let python_path = Self::python_path(worktree).unwrap_or_else(|| worktree.root_path());

        let compiler_venv = python_path
            .strip_suffix("/bin/python3.12")
            .or_else(|| python_path.strip_suffix("/bin/python3"))
            .or_else(|| python_path.strip_suffix("/bin/python"))
            .map(|s| s.to_string())
            .unwrap_or_else(|| python_path.clone());

        Ok(Some(serde_json::json!({
            "pip": {
                "use_system_config": false,
                "index_url": "https://artifacts.internal.inmanta.com/inmanta/dev",
                "pre": true,
            },
            "compilerVenv": compiler_venv
        })))
    }
}

zed::register_extension!(InmantaExtension);
