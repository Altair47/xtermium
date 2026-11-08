//! The past conversations of an AI agent session's tool in its working folder (the session
//! history panel): read from the tool's own session files, which stay as the tool wrote them. A
//! rename is kept by xtermium (`agent-titles`, in the data folder), and a delete removes the
//! tool's transcript, as asked by hand, nothing else.

use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One past conversation of a tool in a folder.
#[derive(Serialize, Clone, Debug, PartialEq, Default)]
pub struct Conversation {
    /// What the tool goes on with it by (`Kind::agent_argv`, resume).
    pub id: String,
    pub title: String,
    /// When it started and when it last changed, Unix milliseconds.
    pub created: u64,
    pub used: u64,
    /// The transcript's size.
    pub bytes: u64,
    /// The transcript file: what a rename is kept against and a delete removes.
    pub path: String,
}

/// How much of a transcript is read: its head for when it started and the first prompt, its tail
/// for the title the tool wrote last. Whole files can be hundreds of megabytes.
const HEAD: u64 = 64 * 1024;
const TAIL: u64 = 256 * 1024;

/// The conversations of `tool` in `dir` (empty: the home folder), latest used first.
pub fn list(tool: &str, dir: &str) -> Vec<Conversation> {
    let Some(home) = home() else { return vec![] };
    let dir = match dir.trim() {
        "" => home.clone(),
        d => crate::paths::expand(d),
    };
    let mut out = match tool {
        "claude" => claude(&home, &dir),
        "codex" => codex(&home, &dir),
        "omp" => omp(&home, &dir),
        _ => vec![],
    };
    let titles = titles();
    for c in &mut out {
        if let Some(t) = titles.get(&c.path) {
            c.title = t.clone();
        }
    }
    out.sort_by_key(|c| std::cmp::Reverse(c.used));
    out
}

/// Keeps `title` for the conversation at `path`, in xtermium's own file: the tool's stays as it is.
pub fn rename(path: &str, title: &str) -> anyhow::Result<()> {
    let mut all = titles();
    let title = crate::clean::text(title);
    if title.is_empty() {
        all.remove(path);
    } else {
        all.insert(path.to_string(), title.chars().take(200).collect());
    }
    crate::config::write("agent-titles", &serde_json::to_string(&all)?)
}

/// Removes the conversation's transcript, when it is one of `tool`'s (inside its sessions folder),
/// and the title kept for it.
pub fn delete(tool: &str, path: &str) -> anyhow::Result<()> {
    let Some(home) = home() else { anyhow::bail!("no home folder") };
    let root = match tool {
        "claude" => claude_home(&home).join("projects"),
        "codex" => codex_home(&home).join("sessions"),
        "omp" => omp_home(&home).join("sessions"),
        _ => anyhow::bail!("not a tool with a history"),
    };
    let file = Path::new(path);
    let inside = file.canonicalize().ok().zip(root.canonicalize().ok()).is_some_and(|(f, r)| f.starts_with(&r));
    if !inside || file.extension().is_none_or(|e| e != "jsonl") {
        anyhow::bail!("not one of the tool's conversations");
    }
    std::fs::remove_file(file)?;
    let mut all = titles();
    if all.remove(path).is_some() {
        crate::config::write("agent-titles", &serde_json::to_string(&all)?)?;
    }
    Ok(())
}

/// The titles given here, by transcript path.
fn titles() -> BTreeMap<String, String> {
    crate::config::read_if("agent-titles").and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

fn home() -> Option<PathBuf> {
    directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf())
}

fn claude_home(home: &Path) -> PathBuf {
    std::env::var_os("CLAUDE_CONFIG_DIR").map_or_else(|| home.join(".claude"), PathBuf::from)
}

fn codex_home(home: &Path) -> PathBuf {
    std::env::var_os("CODEX_HOME").map_or_else(|| home.join(".codex"), PathBuf::from)
}

fn omp_home(home: &Path) -> PathBuf {
    std::env::var_os("PI_CODING_AGENT_DIR").map_or_else(|| home.join(".omp").join("agent"), PathBuf::from)
}

/// Claude Code keeps `<id>.jsonl` per conversation under `projects/<folder with every character
/// but letters and digits as ->/`; `agent-<id>.jsonl` beside them are its subagents' transcripts.
/// The title it shows is the last `ai-title` or `custom-title` line (a rename in Claude), else the
/// first prompt.
fn claude(home: &Path, dir: &Path) -> Vec<Conversation> {
    let folder: String = dir.to_string_lossy().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    // Claude's prompt history names each conversation's first prompt, for the ones whose
    // transcript starts with pages of hook output and never got a title.
    let mut prompts: BTreeMap<String, String> = BTreeMap::new();
    for line in std::fs::read_to_string(claude_home(home).join("history.jsonl")).unwrap_or_default().lines() {
        if let Ok(v) = serde_json::from_str::<Value>(line)
            && let (Some(id), Some(text)) = (v["sessionId"].as_str(), v["display"].as_str())
            && !text.trim_start().starts_with('/')
        {
            prompts.entry(id.to_string()).or_insert_with(|| one_line(text));
        }
    }
    let mut out = vec![];
    for e in std::fs::read_dir(claude_home(home).join("projects").join(folder)).into_iter().flatten().flatten() {
        let path = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if !name.ends_with(".jsonl") || name.starts_with("agent-") {
            continue;
        }
        let Some(mut c) = stat(&path) else { continue };
        c.id = name.trim_end_matches(".jsonl").to_string();
        let (head, tail) = ends(&path);
        for line in head.lines().chain(tail.lines()) {
            let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
            if c.created == 0 && let Some(t) = v["timestamp"].as_str() {
                c.created = millis(t);
            }
            match v["type"].as_str() {
                Some("ai-title") if c.title.is_empty() => c.title = v["aiTitle"].as_str().unwrap_or_default().to_string(),
                Some("custom-title") => c.title = v["customTitle"].as_str().unwrap_or_default().to_string(),
                _ => {}
            }
        }
        // Claude writes the title again and again: the last one in the tail is the newest.
        for line in tail.lines().rev() {
            let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
            if matches!(v["type"].as_str(), Some("ai-title" | "custom-title")) {
                c.title = v["customTitle"].as_str().or(v["aiTitle"].as_str()).unwrap_or_default().to_string();
                break;
            }
        }
        if c.title.is_empty() {
            c.title = first_prompt(&head, |v| (v["type"] == "user" && v["isMeta"] != true).then(|| v["message"]["content"].clone())).or_else(|| prompts.get(&c.id).cloned()).unwrap_or_default();
        }
        out.push(c);
    }
    out
}

/// Codex keeps `sessions/<year>/<month>/<day>/rollout-<time>-<id>.jsonl`, each starting with a
/// `session_meta` line naming its folder (`cwd`), id and start; subagents' threads say so and are
/// left out. Names given in Codex are in `session_index.jsonl`.
fn codex(home: &Path, dir: &Path) -> Vec<Conversation> {
    let codex = codex_home(home);
    let mut names: BTreeMap<String, String> = BTreeMap::new();
    for line in std::fs::read_to_string(codex.join("session_index.jsonl")).unwrap_or_default().lines() {
        if let Ok(v) = serde_json::from_str::<Value>(line)
            && let (Some(id), Some(name)) = (v["id"].as_str(), v["thread_name"].as_str())
        {
            names.insert(id.to_string(), name.to_string());
        }
    }
    let mut files = vec![];
    walk(&codex.join("sessions"), &mut files, 4);
    let mut out = vec![];
    for path in files {
        let (head, _) = ends(&path);
        let Some(meta) = head.lines().next().and_then(|l| serde_json::from_str::<Value>(l).ok()).filter(|v| v["type"] == "session_meta") else { continue };
        let p = &meta["payload"];
        if p["thread_source"] == "subagent" || !p["cwd"].as_str().is_some_and(|c| crate::agent_mcp::same_dir(Path::new(c), dir)) {
            continue;
        }
        let Some(mut c) = stat(&path) else { continue };
        c.id = p["id"].as_str().unwrap_or_default().to_string();
        c.created = p["timestamp"].as_str().map(millis).unwrap_or(c.used);
        c.title = names.get(&c.id).cloned().unwrap_or_default();
        if c.title.is_empty() {
            // The first message is often the AGENTS.md text Codex sends itself; the person's comes after.
            let user = |v: &Value| (v["type"] == "response_item" && v["payload"]["type"] == "message" && v["payload"]["role"] == "user").then(|| v["payload"]["content"].clone());
            c.title = first_prompt(&head, user).unwrap_or_default();
        }
        out.push(c);
    }
    out
}

/// omp (as pi, which it comes from) keeps `sessions/<folder>/<time>_<id>.jsonl`, each starting with
/// a line naming its folder (`cwd`) and id; a name given in omp is a `session_info` line.
/// ponytail: from omp's documentation, not run against it yet.
fn omp(home: &Path, dir: &Path) -> Vec<Conversation> {
    let mut files = vec![];
    walk(&omp_home(home).join("sessions"), &mut files, 2);
    let mut out = vec![];
    for path in files {
        let (head, tail) = ends(&path);
        let Some(first) = head.lines().next().and_then(|l| serde_json::from_str::<Value>(l).ok()) else { continue };
        let cwd = first["cwd"].as_str().or(first["workingDir"].as_str());
        if !cwd.is_some_and(|c| crate::agent_mcp::same_dir(Path::new(c), dir)) {
            continue;
        }
        let Some(mut c) = stat(&path) else { continue };
        c.id = first["id"].as_str().unwrap_or_default().to_string();
        c.created = first["timestamp"].as_str().map(millis).unwrap_or(c.used);
        for line in tail.lines().rev().chain(head.lines().rev()) {
            if let Ok(v) = serde_json::from_str::<Value>(line)
                && v["type"] == "session_info"
                && let Some(n) = v["name"].as_str()
            {
                c.title = n.to_string();
                break;
            }
        }
        if c.title.is_empty() {
            c.title = first_prompt(&head, |v| (v["type"] == "message" && v["message"]["role"] == "user").then(|| v["message"]["content"].clone())).unwrap_or_default();
        }
        out.push(c);
    }
    out
}

/// A conversation with the file's size and last change filled in.
fn stat(path: &Path) -> Option<Conversation> {
    let m = std::fs::metadata(path).ok()?;
    let used = m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_millis() as u64);
    Some(Conversation { used, bytes: m.len(), path: path.display().to_string(), ..Default::default() })
}

/// The first `HEAD` bytes of a file and its last `TAIL` bytes (the whole file twice when it is
/// smaller), cut to whole lines.
fn ends(path: &Path) -> (String, String) {
    use std::io::{Read, Seek, SeekFrom};
    let Ok(mut f) = std::fs::File::open(path) else { return (String::new(), String::new()) };
    let len = f.metadata().map(|m| m.len()).unwrap_or(0);
    let mut head = vec![0; HEAD.min(len) as usize];
    let _ = f.read_exact(&mut head);
    let mut tail = vec![0; TAIL.min(len) as usize];
    let _ = f.seek(SeekFrom::Start(len.saturating_sub(TAIL))).and_then(|_| f.read_exact(&mut tail));
    let head = String::from_utf8_lossy(&head).into_owned();
    let mut tail = String::from_utf8_lossy(&tail).into_owned();
    if len > TAIL && let Some(nl) = tail.find('\n') {
        tail.drain(..=nl); // a line cut in the middle
    }
    (head, tail)
}

/// The first line of `text` for which `pick` gives a user message's content, as a one-line title:
/// a string, or the text parts of a list, skipping what tools and hooks put there (`<…>` blocks,
/// instructions headed `#`).
fn first_prompt(text: &str, pick: impl Fn(&Value) -> Option<Value>) -> Option<String> {
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
        let Some(content) = pick(&v) else { continue };
        let s = match &content {
            Value::String(s) => s.clone(),
            Value::Array(parts) => parts.iter().filter_map(|p| p["text"].as_str()).collect::<Vec<_>>().join(" "),
            _ => continue,
        };
        let s = s.trim();
        if s.is_empty() || s.starts_with('<') || s.starts_with('#') {
            continue;
        }
        return Some(one_line(s));
    }
    None
}

/// A prompt's first line, cut to 80 characters, as a title.
fn one_line(s: &str) -> String {
    let line = s.trim().lines().next().unwrap_or_default().trim();
    if line.chars().count() > 80 { format!("{}…", line.chars().take(79).collect::<String>()) } else { line.to_string() }
}

/// An RFC 3339 time as Unix milliseconds; 0 when it isn't one.
fn millis(t: &str) -> u64 {
    chrono::DateTime::parse_from_rfc3339(t).map(|d| d.timestamp_millis().max(0) as u64).unwrap_or(0)
}

/// Every `.jsonl` under `root`, `depth` folders down at most.
fn walk(root: &Path, out: &mut Vec<PathBuf>, depth: u8) {
    for e in std::fs::read_dir(root).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            if depth > 0 {
                walk(&p, out, depth - 1);
            }
        } else if p.extension().is_some_and(|x| x == "jsonl") {
            out.push(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Claude's transcript gives its id, start, size and newest title; a subagent's file and one
    /// without a title (first prompt instead) are handled too. The title given here wins.
    #[test]
    fn claude_conversations_are_read_from_their_files() {
        crate::config::set_key([7; 32]); // the tests' key: a rename is kept in a sealed file
        let home = crate::config::dir().join("history-claude");
        let dir = home.join("proj");
        std::fs::create_dir_all(&dir).unwrap();
        unsafe { std::env::set_var("CLAUDE_CONFIG_DIR", home.join(".claude")) };
        let folder: String = dir.to_string_lossy().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
        let p = home.join(".claude").join("projects").join(folder);
        std::fs::create_dir_all(&p).unwrap();
        std::fs::write(p.join("aaa.jsonl"), concat!(
            "{\"type\":\"mode\",\"mode\":\"normal\"}\n",
            "{\"type\":\"user\",\"isMeta\":true,\"timestamp\":\"2026-11-08T10:00:00.000Z\",\"message\":{\"role\":\"user\",\"content\":\"<local-command-caveat>x</local-command-caveat>\"}}\n",
            "{\"type\":\"user\",\"timestamp\":\"2026-11-08T10:00:01.000Z\",\"message\":{\"role\":\"user\",\"content\":[{\"type\":\"text\",\"text\":\"fix the login bug\\nplease\"}]}}\n",
            "{\"type\":\"ai-title\",\"aiTitle\":\"Old title\"}\n",
            "{\"type\":\"ai-title\",\"aiTitle\":\"Login bug fix\"}\n",
        )).unwrap();
        std::fs::write(p.join("bbb.jsonl"), "{\"type\":\"user\",\"timestamp\":\"2026-11-07T09:00:00Z\",\"message\":{\"role\":\"user\",\"content\":\"deploy the site\"}}\n").unwrap();
        std::fs::write(p.join("agent-ccc.jsonl"), "{\"type\":\"user\",\"message\":{\"role\":\"user\",\"content\":\"a subagent\"}}\n").unwrap();
        let got = list("claude", &dir.to_string_lossy());
        let mut by_id: Vec<(String, String, u64)> = got.iter().map(|c| (c.id.clone(), c.title.clone(), c.created)).collect();
        by_id.sort();
        assert_eq!(by_id, [("aaa".to_string(), "Login bug fix".to_string(), 1794132000000), ("bbb".to_string(), "deploy the site".to_string(), 1794042000000)]);
        assert!(got.iter().all(|c| c.bytes > 0 && c.used > 0));

        let aaa = got.iter().find(|c| c.id == "aaa").unwrap();
        rename(&aaa.path, "  My login work ").unwrap();
        assert_eq!(list("claude", &dir.to_string_lossy()).iter().find(|c| c.id == "aaa").unwrap().title, "My login work");
        assert!(delete("claude", &home.join("elsewhere.jsonl").display().to_string()).is_err(), "only the tool's own files");
        delete("claude", &aaa.path).unwrap();
        assert_eq!(list("claude", &dir.to_string_lossy()).len(), 1);
        assert!(titles().is_empty(), "the title went with it");
        unsafe { std::env::remove_var("CLAUDE_CONFIG_DIR") };
        let _ = std::fs::remove_dir_all(&home);
    }

    /// Codex's rollouts are picked by their folder, named from its index, subagents' left out.
    #[test]
    fn codex_conversations_are_picked_by_folder() {
        let home = crate::config::dir().join("history-codex");
        let dir = home.join("proj");
        std::fs::create_dir_all(&dir).unwrap();
        let codex = home.join(".codex");
        unsafe { std::env::set_var("CODEX_HOME", &codex) };
        let day = codex.join("sessions").join("2026").join("11").join("08");
        std::fs::create_dir_all(&day).unwrap();
        let cwd = serde_json::to_string(&dir.to_string_lossy()).unwrap();
        let meta = |id: &str, source: &str| format!("{{\"timestamp\":\"2026-11-08T10:00:00.000Z\",\"type\":\"session_meta\",\"payload\":{{\"id\":\"{id}\",\"timestamp\":\"2026-11-08T10:00:00.000Z\",\"cwd\":{cwd},\"thread_source\":\"{source}\"}}}}\n");
        let user = "{\"type\":\"response_item\",\"payload\":{\"type\":\"message\",\"role\":\"user\",\"content\":[{\"type\":\"input_text\",\"text\":\"# AGENTS.md instructions\\n...\"}]}}\n{\"type\":\"response_item\",\"payload\":{\"type\":\"message\",\"role\":\"user\",\"content\":[{\"type\":\"input_text\",\"text\":\"rename the tunnels\"}]}}\n";
        std::fs::write(day.join("rollout-a.jsonl"), meta("id-a", "cli") + user).unwrap();
        std::fs::write(day.join("rollout-b.jsonl"), meta("id-b", "cli") + user).unwrap();
        std::fs::write(day.join("rollout-c.jsonl"), meta("id-c", "subagent") + user).unwrap();
        std::fs::write(day.join("rollout-d.jsonl"), "{\"type\":\"session_meta\",\"payload\":{\"id\":\"id-d\",\"cwd\":\"/somewhere/else\"}}\n").unwrap();
        std::fs::write(codex.join("session_index.jsonl"), "{\"id\":\"id-a\",\"thread_name\":\"Tunnel renames\"}\n").unwrap();
        let mut got: Vec<(String, String)> = list("codex", &dir.to_string_lossy()).into_iter().map(|c| (c.id, c.title)).collect();
        got.sort();
        assert_eq!(got, [("id-a".to_string(), "Tunnel renames".to_string()), ("id-b".to_string(), "rename the tunnels".to_string())]);
        unsafe { std::env::remove_var("CODEX_HOME") };
        let _ = std::fs::remove_dir_all(&home);
    }
}
