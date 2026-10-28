//! Starts the bundled or system Python server and opens the local web app.
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const DEFAULT_PORT: &str = "8765";
const APP_PATH: &str = "/MarkDown-Ultimate/";

fn main() {
    if let Err(e) = run() {
        eprintln!("MarkDown Ultimate launcher error: {e}");
        eprintln!("Press Enter to exit.");
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let root = exe_root()?;
    env::set_current_dir(&root).map_err(|e| format!("set cwd: {e}"))?;

    let index = root.join("dist").join("index.html");
    if !index.is_file() {
        return Err(format!(
            "dist/index.html not found under {}. Run Trunk build first.",
            root.display()
        ));
    }

    let port = env::var("MDU_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
    let url = format!("http://127.0.0.1:{port}{APP_PATH}");

    let serve = root.join("tools").join("serve_dist.py");
    if !serve.is_file() {
        return Err(format!("Missing {}", serve.display()));
    }

    let python = resolve_python(&root)?;
    env::set_var("MDU_PORT", &port);
    env::set_var("MDU_QUIET", "1");

    Command::new(&python.exe)
        .args(python.args)
        .arg(&serve)
        .current_dir(&root)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("start server ({:?}): {e}", python.exe))?;

    thread::sleep(Duration::from_secs(3));

    Command::new("cmd")
        .args(["/C", "start", "", &url])
        .spawn()
        .map_err(|e| format!("open browser: {e}"))?;

    println!("MarkDown Ultimate: {url}");
    println!("Leave this window open while you use the app.");
    Ok(())
}

fn exe_root() -> Result<PathBuf, String> {
    env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .ok_or_else(|| "Could not resolve launcher directory".to_string())
}

struct PythonCmd {
    exe: PathBuf,
    args: Vec<String>,
}

fn resolve_python(root: &Path) -> Result<PythonCmd, String> {
    let candidates: [(&str, Vec<String>); 5] = [
        ("python\\pythonw.exe", vec![]),
        ("python\\python.exe", vec![]),
        ("pythonw", vec![]),
        ("python", vec![]),
        ("py", vec!["-3".to_string()]),
    ];

    for (name, extra) in candidates {
        if name.contains('\\') {
            let p = root.join(name);
            if p.is_file() {
                return Ok(PythonCmd {
                    exe: p,
                    args: extra,
                });
            }
        } else if which_on_path(name) {
            return Ok(PythonCmd {
                exe: PathBuf::from(name),
                args: extra,
            });
        }
    }

    Err(
        "Python not found. Use the portable bundle (embeddable python/) or install Python 3."
            .to_string(),
    )
}

fn which_on_path(name: &str) -> bool {
    Command::new("where")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
