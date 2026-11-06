//! An AI agent session's MCP servers and skills: the ones its tool would load (read from the
//! tool's own configuration files and skill folders, never changed), and turning some off for that
//! session only, each tool its own way. What's off isn't loaded or listed to the model, so it
//! costs no tokens. Cursor's CLI has no way to turn a server off for one run, and Copilot's and
//! Cursor's none to turn a skill off, so those aren't offered there.

use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// The tools whose sessions can turn servers off (ui/lib/api.ts `MCP_TOOLS`).
pub const TOOLS: &[&str] = &["claude", "codex", "opencode", "copilot"];

/// The tools whose sessions can turn skills off (ui/lib/api.ts `SKILL_TOOLS`).
pub const SKILL_TOOLS: &[&str] = &["claude", "codex", "opencode"];

/// xtermium's own server, which agent sessions get while automation runs (`automation::agent_launch`).
pub const OWN: &str = "xtermium";

/// Copilot's built-in server, there without any configuration.
const COPILOT_BUILTIN: &str = "github-mcp-server";

/// The names of the MCP servers `tool` would start in `dir` (empty: the home folder), xtermium's
/// first, then the rest sorted. Only what its files say: a server the tool learns of elsewhere
/// (Claude's claude.ai connectors not yet toggled in its /mcp) can still be turned off by name.
pub fn servers(tool: &str, dir: &str) -> Vec<String> {
    if !TOOLS.contains(&tool) {
        return vec![];
    }
    let Some(home) = directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf()) else { return vec![OWN.into()] };
    let dir = match dir.trim() {
        "" => home.clone(),
        d => crate::paths::expand(d),
    };
    let mut names: Vec<String> = vec![];
    match tool {
        "claude" => claude(&home, &dir, &mut names),
        // Codex says itself, its plugins' servers included; its files when it can't.
        "codex" if codex_list(&dir, &mut names) => {}
        "codex" => {
            let codex = std::env::var_os("CODEX_HOME").map_or_else(|| home.join(".codex"), PathBuf::from);
            for f in [codex.join("config.toml"), dir.join(".codex").join("config.toml")] {
                let table = std::fs::read_to_string(f).ok().and_then(|s| s.parse::<toml::Table>().ok());
                if let Some(toml::Value::Table(t)) = table.as_ref().and_then(|t| t.get("mcp_servers")) {
                    names.extend(t.keys().cloned());
                }
            }
        }
        "opencode" => {
            let config = std::env::var_os("XDG_CONFIG_HOME").map_or_else(|| home.join(".config"), PathBuf::from).join("opencode");
            let mut files: Vec<PathBuf> = ["opencode.json", "opencode.jsonc", "config.json"].iter().map(|f| config.join(f)).collect();
            files.extend(std::env::var_os("OPENCODE_CONFIG").map(PathBuf::from));
            files.extend(["opencode.json", "opencode.jsonc"].iter().map(|f| dir.join(f)));
            for f in files {
                // A server has its `type` (local or remote); anything else under `mcp` isn't one.
                let mcp = read_json(&f)["mcp"].clone();
                names.extend(objects(&mcp).into_iter().filter(|n| mcp[n]["type"].is_string()));
            }
        }
        "copilot" => {
            let copilot = std::env::var_os("COPILOT_HOME").map_or_else(|| home.join(".copilot"), PathBuf::from);
            for f in [copilot.join("mcp-config.json"), dir.join(".mcp.json"), dir.join(".github").join("mcp.json")] {
                names.extend(objects(&read_json(&f)["mcpServers"]));
            }
            names.push(COPILOT_BUILTIN.into());
        }
        _ => {}
    }
    names.retain(|n| n != OWN);
    names.sort();
    names.dedup();
    names.insert(0, OWN.into());
    names
}

/// The names of the skills `tool` would list in `dir` (empty: the home folder), sorted: the user's
/// skill folders, then the project's, from `dir` up to its repository's top (only `dir` outside a
/// repository). Plugins' skills aren't here: Claude can't turn one off on its own.
pub fn skills(tool: &str, dir: &str) -> Vec<String> {
    if !SKILL_TOOLS.contains(&tool) {
        return vec![];
    }
    let Some(home) = directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf()) else { return vec![] };
    let dir = match dir.trim() {
        "" => home.clone(),
        d => crate::paths::expand(d),
    };
    let (mut roots, project): (Vec<PathBuf>, &[&str]) = match tool {
        "claude" => (vec![home.join(".claude").join("skills")], &[".claude"]),
        "codex" => {
            let codex = std::env::var_os("CODEX_HOME").map_or_else(|| home.join(".codex"), PathBuf::from);
            (vec![home.join(".agents").join("skills"), codex.join("skills")], &[".agents"])
        }
        _ => {
            let config = std::env::var_os("XDG_CONFIG_HOME").map_or_else(|| home.join(".config"), PathBuf::from);
            (vec![config.join("opencode").join("skills"), home.join(".claude").join("skills"), home.join(".agents").join("skills")], &[".opencode", ".claude", ".agents"])
        }
    };
    let top = dir.ancestors().position(|d| d.join(".git").exists()).unwrap_or(0);
    for d in dir.ancestors().take(top + 1) {
        roots.extend(project.iter().map(|p| d.join(p).join("skills")));
    }
    let mut names = vec![];
    for root in roots {
        for e in std::fs::read_dir(root).into_iter().flatten().flatten() {
            if let Ok(text) = std::fs::read_to_string(e.path().join("SKILL.md")) {
                names.push(skill_name(&text).unwrap_or_else(|| e.file_name().to_string_lossy().into_owned()));
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

/// A SKILL.md's `name:` from its front matter; None without one (the folder names it then).
fn skill_name(text: &str) -> Option<String> {
    let mut lines = text.trim_start_matches('\u{feff}').lines();
    if lines.next()?.trim() != "---" {
        return None;
    }
    let name = lines.take_while(|l| l.trim() != "---").find_map(|l| l.strip_prefix("name:"))?;
    Some(name.trim().trim_matches(['"', '\'']).to_string()).filter(|n| !n.is_empty())
}

/// `codex mcp list --json` run in `dir` (its project's .codex/config.toml counts): every server it
/// would start, plugins' too, without connecting to any. False when Codex isn't there or says nothing.
/// ponytail: no time limit; it answers in a few seconds, and the field shows "Looking…" meanwhile.
fn codex_list(dir: &Path, names: &mut Vec<String>) -> bool {
    if cfg!(test) {
        return false; // tests read files; they don't run anyone's Codex
    }
    // Through cmd on Windows: an npm-installed codex is a .cmd shim, which only cmd resolves.
    let mut cmd = if cfg!(windows) { std::process::Command::new("cmd") } else { std::process::Command::new("codex") };
    if cfg!(windows) {
        cmd.args(["/c", "codex"]);
    }
    cmd.args(["mcp", "list", "--json"]).stdin(std::process::Stdio::null()).stderr(std::process::Stdio::null());
    if dir.is_dir() {
        cmd.current_dir(dir);
    }
    #[cfg(windows)]
    crate::term::no_window(&mut cmd);
    let Ok(out) = cmd.output() else { return false };
    let list: Value = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    let found: Vec<String> = list.as_array().into_iter().flatten().filter_map(|s| s["name"].as_str().map(String::from)).collect();
    names.extend(found.iter().cloned());
    out.status.success() && !found.is_empty()
}

/// Claude Code's servers: user scope (`mcpServers` in ~/.claude.json), local scope (that file's
/// entry for the folder), the folder's .mcp.json, and enabled plugins' servers, which it names
/// `plugin:<plugin>:<server>`. claude.ai connectors come from claude.ai; the ones its /mcp panel
/// has turned on or off somewhere are in ~/.claude.json, by the names `deniedMcpServers` matches.
fn claude(home: &Path, dir: &Path, names: &mut Vec<String>) {
    let state = read_json(&home.join(".claude.json"));
    names.extend(objects(&state["mcpServers"]));
    if let Some(projects) = state["projects"].as_object() {
        for (path, p) in projects {
            if same_dir(Path::new(path), dir) {
                names.extend(objects(&p["mcpServers"]));
            }
            for list in ["disabledMcpServers", "enabledMcpServers"] {
                let seen = p[list].as_array().into_iter().flatten().filter_map(Value::as_str);
                names.extend(seen.filter(|n| n.starts_with("claude.ai ")).map(String::from));
            }
        }
    }
    names.extend(objects(&read_json(&dir.join(".mcp.json"))["mcpServers"]));
    let plugins = home.join(".claude").join("plugins");
    let enabled = read_json(&home.join(".claude").join("settings.json"))["enabledPlugins"].clone();
    let installed = read_json(&plugins.join("installed_plugins.json"));
    for (id, entries) in installed["plugins"].as_object().into_iter().flatten() {
        if enabled[id] != json!(true) {
            continue;
        }
        let plugin = id.split('@').next().unwrap_or(id);
        let Some(root) = entries.as_array().and_then(|e| e.first()).and_then(|e| e["installPath"].as_str()) else { continue };
        names.extend(plugin_servers(Path::new(root)).into_iter().map(|s| format!("plugin:{plugin}:{s}")));
    }
}

/// A plugin's servers: its .mcp.json, and `mcpServers` in its manifest, inline or a file's path.
fn plugin_servers(root: &Path) -> Vec<String> {
    let mut out = servers_file(&read_json(&root.join(".mcp.json")));
    for manifest in [root.join(".claude-plugin").join("plugin.json"), root.join("plugin.json")] {
        match &read_json(&manifest)["mcpServers"] {
            Value::String(p) => out.extend(servers_file(&read_json(&root.join(p)))),
            m @ Value::Object(_) => out.extend(objects(m)),
            _ => {}
        }
    }
    out
}

/// A .mcp.json's servers: under `mcpServers`, or (as some plugins write it) at its top.
fn servers_file(v: &Value) -> Vec<String> {
    if v.get("mcpServers").is_some() { objects(&v["mcpServers"]) } else { objects(v) }
}

/// The keys of an object whose values are objects (servers' definitions).
fn objects(v: &Value) -> Vec<String> {
    v.as_object().into_iter().flatten().filter(|(_, d)| d.is_object()).map(|(k, _)| k.clone()).collect()
}

/// The same folder: Claude writes its project paths with forward slashes, and Windows ignores case.
fn same_dir(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| {
        let s = p.to_string_lossy().replace('\\', "/");
        let s = s.trim_end_matches('/').to_string();
        if cfg!(windows) { s.to_lowercase() } else { s }
    };
    norm(a) == norm(b)
}

/// A JSON or JSONC file (comments, trailing commas), or `null` when it's missing or unreadable.
fn read_json(path: &Path) -> Value {
    std::fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&strip_jsonc(&s)).ok()).unwrap_or(Value::Null)
}

/// JSONC as plain JSON: `//` and `/* */` comments and commas before `}` or `]` go, strings stay.
fn strip_jsonc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    let mut in_string = false;
    while let Some(c) = chars.next() {
        if in_string {
            out.push(c);
            if c == '\\' {
                out.extend(chars.next());
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match (c, chars.peek()) {
            ('"', _) => {
                in_string = true;
                out.push(c);
            }
            ('/', Some('/')) => {
                while chars.next().is_some_and(|c| c != '\n') {}
                out.push('\n');
            }
            ('/', Some('*')) => {
                chars.next();
                let mut last = ' ';
                for c in chars.by_ref() {
                    if last == '*' && c == '/' {
                        break;
                    }
                    last = c;
                }
            }
            (',', _) => {
                // A trailing comma: the next thing that isn't blank closes the object or array.
                let rest: String = chars.clone().collect();
                if !matches!(rest.trim_start().chars().next(), Some('}' | ']')) {
                    out.push(c);
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// A name that can go on a command line as it is: through an npm `.cmd` shim on Windows, cmd.exe
/// reads the arguments again, so quotes and shell characters never go there. Codex also takes it
/// as a bare TOML key. ponytail: a server named otherwise isn't turned off for Codex or Copilot.
fn plain(name: &str) -> bool {
    !name.is_empty() && name.len() <= 128 && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
}

/// Turns `off` servers and `skills` off for this run of `tool`: Claude through a settings file
/// with `deniedMcpServers` (any server, plugins' and claude.ai connectors' too) and
/// `skillOverrides` "off", Codex with one `-c mcp_servers.<name>.enabled=false` each and a
/// `skills.config` list, OpenCode with `"enabled": false` servers and denied skills in
/// OPENCODE_CONFIG_CONTENT (merged over its other files), Copilot with `--disable-mcp-server`.
/// xtermium's own server is left out by `agent_launch` instead, and turned off here as well in case
/// the tool has one of its own by that name.
pub fn turn_off(tool: &str, off: &[String], skills: &[String], argv: &mut Vec<String>, env: &mut Vec<(String, String)>) {
    if (off.is_empty() && skills.is_empty()) || argv.is_empty() {
        return;
    }
    match tool {
        "claude" => {
            let mut settings = json!({});
            if !off.is_empty() {
                settings["deniedMcpServers"] = off.iter().map(|n| json!({ "serverName": n })).collect();
            }
            if !skills.is_empty() {
                settings["skillOverrides"] = skills.iter().map(|n| (n.clone(), json!("off"))).collect::<serde_json::Map<_, _>>().into();
            }
            static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if let Some(path) = crate::config::temp().map(|t| t.join(format!("mcp-off-{n}.json")))
                && std::fs::write(&path, settings.to_string()).is_ok()
            {
                argv.extend(["--settings".into(), path.display().to_string()]);
            }
        }
        "codex" => {
            let mut flags: Vec<String> = off.iter().filter(|n| plain(n)).flat_map(|n| ["-c".to_string(), format!("mcp_servers.{n}.enabled=false")]).collect();
            let named: Vec<String> = skills.iter().filter(|n| plain(n)).map(|n| format!("{{name={},enabled=false}}", json!(n))).collect();
            if !named.is_empty() {
                flags.extend(["-c".into(), format!("skills.config=[{}]", named.join(","))]);
            }
            argv.splice(1..1, flags);
        }
        "copilot" => argv.extend(off.iter().filter(|n| plain(n)).map(|n| format!("--disable-mcp-server={n}"))),
        "opencode" => {
            const VAR: &str = "OPENCODE_CONFIG_CONTENT";
            let at = env.iter().position(|(k, _)| k == VAR);
            let mut config = at.and_then(|i| serde_json::from_str::<Value>(&env[i].1).ok()).unwrap_or_else(|| json!({}));
            for n in off {
                config["mcp"][n] = json!({ "enabled": false });
            }
            for n in skills {
                config["permission"]["skill"][n] = json!("deny");
            }
            let text = config.to_string();
            match at {
                Some(i) => env[i].1 = text,
                None => env.push((VAR.into(), text)),
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jsonc_reads_as_json() {
        let s = r#"{
  // a comment, "with quotes"
  "mcp": { "a": { "url": "http://x/y", }, /* gone */ "b": { "cmd": "say \"//hi\"" }, },
  "list": [1, 2,],
}"#;
        let v: Value = serde_json::from_str(&strip_jsonc(s)).unwrap();
        assert_eq!(objects(&v["mcp"]), ["a", "b"]);
        assert_eq!(v["mcp"]["a"]["url"], "http://x/y", "// inside a string stays");
        assert_eq!(v["mcp"]["b"]["cmd"], r#"say "//hi""#);
        assert_eq!(v["list"], json!([1, 2]));
    }

    #[test]
    fn each_tool_turns_servers_off_its_own_way() {
        let off = vec!["context7".to_string(), "claude.ai Gmail".to_string(), "x\" & calc".to_string()];
        let (mut argv, mut env) = (vec!["codex".to_string(), "--model".into(), "m".into()], vec![]);
        turn_off("codex", &off, &[], &mut argv, &mut env);
        assert_eq!(argv, ["codex", "-c", "mcp_servers.context7.enabled=false", "--model", "m"], "names unsafe on a command line are skipped");

        let mut argv = vec!["copilot".to_string()];
        turn_off("copilot", &off, &[], &mut argv, &mut env);
        assert_eq!(argv, ["copilot", "--disable-mcp-server=context7"]);

        let mut env = vec![("OPENCODE_CONFIG_CONTENT".to_string(), json!({ "mcp": { OWN: { "type": "remote" } } }).to_string())];
        turn_off("opencode", &off, &[], &mut vec!["opencode".into()], &mut env);
        let config: Value = serde_json::from_str(&env[0].1).unwrap();
        assert_eq!(config["mcp"][OWN]["type"], "remote", "xtermium's own entry stays");
        assert_eq!(config["mcp"]["claude.ai Gmail"], json!({ "enabled": false }));

        let mut argv = vec!["claude".to_string()];
        turn_off("claude", &off, &[], &mut argv, &mut vec![]);
        assert_eq!(argv[1], "--settings");
        let settings: Value = serde_json::from_str(&std::fs::read_to_string(&argv[2]).unwrap()).unwrap();
        let _ = std::fs::remove_file(&argv[2]);
        assert_eq!(settings["deniedMcpServers"][1], json!({ "serverName": "claude.ai Gmail" }), "any name: it's in a file");

        let mut argv = vec!["claude".to_string()];
        turn_off("claude", &[], &[], &mut argv, &mut vec![]);
        assert_eq!(argv, ["claude"], "nothing off: nothing added");
    }

    #[test]
    fn skills_turn_off_and_are_named() {
        let skills = vec!["deploy".to_string(), "x\" & calc".to_string()];
        let mut argv = vec!["codex".to_string()];
        turn_off("codex", &[], &skills, &mut argv, &mut vec![]);
        assert_eq!(argv, ["codex", "-c", r#"skills.config=[{name="deploy",enabled=false}]"#], "names unsafe on a command line are skipped");

        let mut env = vec![];
        turn_off("opencode", &[], &skills, &mut vec!["opencode".into()], &mut env);
        let config: Value = serde_json::from_str(&env[0].1).unwrap();
        assert_eq!(config["permission"]["skill"]["deploy"], "deny");

        let mut argv = vec!["claude".to_string()];
        turn_off("claude", &["context7".into()], &skills, &mut argv, &mut vec![]);
        let settings: Value = serde_json::from_str(&std::fs::read_to_string(&argv[2]).unwrap()).unwrap();
        let _ = std::fs::remove_file(&argv[2]);
        assert_eq!(settings["skillOverrides"]["deploy"], "off");
        assert_eq!(settings["deniedMcpServers"][0], json!({ "serverName": "context7" }), "one settings file for both");

        assert_eq!(skill_name("\u{feff}---\nname: \"deploy\"\ndescription: x\n---\nbody"), Some("deploy".into()));
        assert_eq!(skill_name("---\ndescription: x\n---\nname: not this"), None, "only the front matter names it");
        assert_eq!(skill_name("# no front matter"), None);
    }

    #[test]
    fn claude_paths_match_however_written() {
        if cfg!(windows) {
            assert!(same_dir(Path::new("C:/Users/me/proj"), Path::new(r"c:\Users\me\proj\")));
        }
        assert!(same_dir(Path::new("/home/me/proj/"), Path::new("/home/me/proj")));
    }
}
