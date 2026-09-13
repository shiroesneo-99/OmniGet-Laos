//! Runtime extensions of the asset protocol scope.
//!
//! `tauri.conf.json` only allows app-owned directories and the standard user
//! media folders statically. Everything the webview legitimately previews
//! through `convertFileSrc` outside of those (course folders, music/book
//! roots, a download folder on another drive, files a plugin command hands
//! back) is allowed here, at runtime, from sources the Rust side trusts:
//! the saved settings, the study plugin database and plugin command results.
//!
//! Folders picked through the dialog plugin are already added to the asset
//! scope by `tauri-plugin-dialog` itself, so no command needs to do that.

use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use tauri::{AppHandle, Manager, Runtime};

/// Name of a file that never exists, used to ask the scope whether a
/// directory is already covered without touching the filesystem.
const PROBE: &str = "__omniget_asset_scope_probe__";

/// Max JSON nesting walked when scanning plugin results.
const MAX_JSON_DEPTH: usize = 32;

fn seen() -> &'static Mutex<HashSet<(PathBuf, bool)>> {
    static SEEN: OnceLock<Mutex<HashSet<(PathBuf, bool)>>> = OnceLock::new();
    SEEN.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Comparable form of a path: `\\?\` stripped, separators unified and, on
/// Windows, lowercased (the filesystem is case-insensitive there).
fn normalized(path: &Path) -> String {
    let s = path.to_string_lossy();
    let s = s.strip_prefix(r"\\?\").unwrap_or(&s);
    let s = s.replace('\\', "/");
    let s = s.trim_end_matches('/').to_string();
    if cfg!(windows) {
        s.to_lowercase()
    } else {
        s
    }
}

/// True when `dir` is the user's home directory or one of its ancestors
/// (including the filesystem root holding it).
fn covers_home(dir: &Path) -> bool {
    let Some(home) = dirs::home_dir() else {
        return false;
    };
    let home = normalized(&home);
    let dir = normalized(dir);
    if dir.is_empty() {
        return true;
    }
    home == dir || home.starts_with(&format!("{dir}/"))
}

fn is_fs_root(dir: &Path) -> bool {
    !dir.components().any(|c| matches!(c, Component::Normal(_)))
}

/// Adds `dir` to the asset protocol scope unless it is already covered.
fn allow_dir<R: Runtime>(app: &AppHandle<R>, dir: &Path, recursive: bool) {
    if dir.as_os_str().is_empty() || !dir.is_absolute() {
        return;
    }
    let key = (dir.to_path_buf(), recursive);
    {
        let mut seen = seen().lock().unwrap_or_else(|e| e.into_inner());
        if !seen.insert(key) {
            return;
        }
    }
    let scope = app.asset_protocol_scope();
    let probe = if recursive {
        dir.join(PROBE).join(PROBE)
    } else {
        dir.join(PROBE)
    };
    if scope.is_allowed(&probe) {
        return;
    }
    if let Err(e) = scope.allow_directory(dir, recursive) {
        tracing::warn!("[asset-scope] could not allow {}: {}", dir.display(), e);
    } else {
        tracing::debug!(
            "[asset-scope] allowed {} (recursive: {})",
            dir.display(),
            recursive
        );
    }
}

/// Allows a folder the user configured explicitly (download folder, library
/// root). These come from persisted state, so they are allowed as-is; the
/// static `deny` list still applies on top.
pub fn allow_user_dir<R: Runtime>(app: &AppHandle<R>, dir: &Path) {
    allow_dir(app, dir, true);
}

/// Startup wiring: app data dir (covers portable mode), sensitive files in
/// it, the configured download folder and every persisted study root.
pub fn init<R: Runtime>(app: &AppHandle<R>, download_dir: &Path) {
    let scope = app.asset_protocol_scope();
    if let Some(data_dir) = crate::core::paths::app_data_dir() {
        // Forbid first: forbidden patterns win over allowed ones.
        for file in [
            "settings.json",
            "chrome-extension-cookies.txt",
            "telegram.session",
            "queue.wal",
        ] {
            let _ = scope.forbid_file(data_dir.join(file));
        }
        let _ = scope.forbid_directory(data_dir.join("cookies"), true);
        let _ = scope.forbid_directory(data_dir.join("chrome-native-host"), true);

        // Equal to `$APPDATA` in a normal install; in portable mode
        // (`OMNIGET_DATA_DIR`) it lives next to the executable instead.
        allow_dir(app, &data_dir, true);

        allow_study_roots(app, &data_dir.join("plugins").join("study").join("data"));
    }

    if download_dir.is_absolute() {
        allow_user_dir(app, download_dir);
    }
}

/// Reads the study plugin database (read-only, best effort) for the folders
/// the user added to the course, book and music libraries, so media restored
/// by the frontend before any plugin command runs (e.g. the music queue) can
/// still load after a restart.
fn allow_study_roots<R: Runtime>(app: &AppHandle<R>, study_data_dir: &Path) {
    let db_path = study_data_dir.join("study.db");
    if !db_path.is_file() {
        return;
    }
    let conn = match rusqlite::Connection::open_with_flags(
        &db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("[asset-scope] could not open study db: {}", e);
            return;
        }
    };
    let _ = conn.busy_timeout(std::time::Duration::from_millis(500));

    let query_paths = |sql: &str| -> Vec<String> {
        let Ok(mut stmt) = conn.prepare(sql) else {
            return Vec::new();
        };
        let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(0)) else {
            return Vec::new();
        };
        rows.flatten().collect()
    };

    // Folders allowed recursively.
    for sql in [
        "SELECT path FROM library_roots WHERE enabled != 0",
        "SELECT path FROM read_roots WHERE enabled != 0",
        "SELECT path FROM music_roots WHERE enabled != 0",
        "SELECT path FROM library_pinned_paths",
        "SELECT source_path FROM courses",
    ] {
        for p in query_paths(sql) {
            let p = PathBuf::from(p);
            if p.is_absolute() {
                allow_user_dir(app, &p);
            }
        }
    }

    // Downloaded tracks can sit outside any music root.
    let mut parents = HashSet::new();
    for p in query_paths("SELECT local_path FROM music_dedup") {
        if let Some(parent) = Path::new(&p).parent() {
            parents.insert(parent.to_path_buf());
        }
    }
    for parent in parents {
        if !is_fs_root(&parent) && !covers_home(&parent) {
            allow_dir(app, &parent, false);
        }
    }
}

fn looks_like_local_path(s: &str) -> bool {
    (3..4096).contains(&s.len())
        && !s.contains("://")
        && !s.contains('\n')
        && !s.starts_with("data:")
        && Path::new(s).is_absolute()
}

fn collect_paths<'a>(value: &'a serde_json::Value, depth: usize, out: &mut Vec<&'a str>) {
    if depth > MAX_JSON_DEPTH {
        return;
    }
    match value {
        serde_json::Value::String(s) if looks_like_local_path(s) => out.push(s),
        serde_json::Value::Array(items) => {
            for v in items {
                collect_paths(v, depth + 1, out);
            }
        }
        serde_json::Value::Object(map) => {
            for v in map.values() {
                collect_paths(v, depth + 1, out);
            }
        }
        _ => {}
    }
}

/// Allows the local files a plugin command returned (lesson videos,
/// subtitles, covers, book chapters, browse listings...).
///
/// Only the directory holding each file is allowed, non-recursively. HTML
/// documents (HTML books, extracted EPUB chapters) also get their parent and
/// grandparent recursively so relative stylesheets and images resolve.
/// Commands managing library roots return folders, which are allowed
/// recursively. Home directories and filesystem roots are never widened from
/// here, since plugin arguments come from the webview.
pub fn allow_from_plugin_result<R: Runtime>(
    app: &AppHandle<R>,
    command: &str,
    value: &serde_json::Value,
) {
    let mut found = Vec::new();
    collect_paths(value, 0, &mut found);
    if found.is_empty() {
        return;
    }
    let roots_command = command.contains("roots");
    let mut dirs_done: HashSet<(&Path, bool)> = HashSet::new();

    for s in found {
        let path = Path::new(s);
        if roots_command {
            if !is_fs_root(path) && !covers_home(path) && dirs_done.insert((path, true)) {
                allow_dir(app, path, true);
            }
            continue;
        }
        let Some(parent) = path.parent() else {
            continue;
        };
        if is_fs_root(parent) || covers_home(parent) {
            continue;
        }
        let is_html = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| matches!(e.to_ascii_lowercase().as_str(), "html" | "htm" | "xhtml"))
            .unwrap_or(false);
        if is_html {
            let top = match parent.parent() {
                Some(gp) if !is_fs_root(gp) && !covers_home(gp) => gp,
                _ => parent,
            };
            if dirs_done.insert((top, true)) {
                allow_dir(app, top, true);
            }
        } else if dirs_done.insert((parent, false)) {
            allow_dir(app, parent, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_local_paths() {
        assert!(!looks_like_local_path("https://example.com/a.jpg"));
        assert!(!looks_like_local_path("data:image/png;base64,AAAA"));
        assert!(!looks_like_local_path("relative/file.mp4"));
        assert!(!looks_like_local_path("Some title"));
        if cfg!(windows) {
            assert!(looks_like_local_path(r"D:\Courses\a\lesson.mp4"));
            assert!(looks_like_local_path(r"\\?\D:\Courses\a\lesson.mp4"));
        } else {
            assert!(looks_like_local_path("/home/u/Courses/lesson.mp4"));
        }
    }

    #[test]
    fn collects_nested_strings() {
        let sample = if cfg!(windows) {
            serde_json::json!({"a": [{"video_path": r"D:\x\y.mp4"}, "text"], "n": 1})
        } else {
            serde_json::json!({"a": [{"video_path": "/x/y.mp4"}, "text"], "n": 1})
        };
        let mut out = Vec::new();
        collect_paths(&sample, 0, &mut out);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn home_and_roots_are_broad() {
        if let Some(home) = dirs::home_dir() {
            assert!(covers_home(&home));
            if let Some(parent) = home.parent() {
                assert!(covers_home(parent));
            }
            assert!(!covers_home(&home.join("Music")));
        }
        let root = if cfg!(windows) { r"C:\" } else { "/" };
        assert!(is_fs_root(Path::new(root)));
    }
}
