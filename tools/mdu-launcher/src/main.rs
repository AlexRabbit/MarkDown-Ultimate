//! Native Windows shell: embeddable Python serves dist/; wry webview shows the app (no external browser).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use tao::event_loop::{ControlFlow, EventLoop};
use tao::event::Event;
use tao::event::WindowEvent;
use tao::window::WindowBuilder;

const DEFAULT_PORT: &str = "8765";
const APP_PATH: &str = "/MarkDown-Ultimate/";

fn main() {
    if let Err(e) = run() {
        show_fatal(&e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let root = exe_root()?;
    env::set_current_dir(&root).map_err(|e| format!("set cwd: {e}"))?;

    let index = root.join("dist").join("index.html");
    if !index.is_file() {
        return Err(format!(
            "dist/index.html not found in {}. Re-run portable-bundle from repo root.",
            root.display()
        ));
    }

    let port = env::var("MDU_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
    let url = format!("http://127.0.0.1:{port}{APP_PATH}");

    start_server(&root, &port)?;

    for attempt in 0..40 {
        thread::sleep(Duration::from_millis(250));
        if port_open(&port) {
            break;
        }
        if attempt == 39 {
            return Err(format!("Server did not listen on port {port}"));
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("MarkDown Ultimate")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 840.0))
        .build(&event_loop)
        .map_err(|e| format!("window: {e}"))?;

    let _webview = wry::WebViewBuilder::new()
        .with_url(&url)
        .build(&window)
        .map_err(|e| format!("webview: {e}"))?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn port_open(port: &str) -> bool {
    use std::net::{SocketAddr, TcpStream};
    let addr: SocketAddr = match format!("127.0.0.1:{port}").parse() {
        Ok(a) => a,
        Err(_) => return false,
    };
    TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_ok()
}

fn start_server(root: &Path, port: &str) -> Result<(), String> {
    let serve = root.join("tools").join("serve_dist.py");
    if !serve.is_file() {
        return Err(format!("Missing {}", serve.display()));
    }
    let python = resolve_python(root)?;
    env::set_var("MDU_PORT", port);
    env::set_var("MDU_QUIET", "1");

    Command::new(&python.exe)
        .args(&python.args)
        .arg(&serve)
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("start server ({:?}): {e}", python.exe))?;
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
        "Python not found next to this app (python/ folder). Re-run tools/portable-bundle.ps1."
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

fn show_fatal(msg: &str) {
    let _ = Command::new("cmd")
        .args(["/C", "start", "", "mshta", &format!(
            "javascript:alert('MarkDown Ultimate:\\n{}');close()",
            msg.replace('\'', "\\'").replace('\n', "\\n")
        )])
        .spawn();
}
