use std::path::Path;
use std::process::Command;

/// Run a command and return its trimmed stdout, or None if it fails.
fn command_output(program: &str, args: &[&str]) -> Option<String> {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn main() {
    // Get git commit SHA of the source the build was made from
    let git_sha = command_output("git", &["rev-parse", "--short", "HEAD"])
        .unwrap_or_else(|| "unknown".into());

    // Get build timestamp. build-all.sh passes a fresh one on every build, so
    // the stamp shows when this UI was built.
    let timestamp = std::env::var("IBM1130_BUILD_TIMESTAMP")
        .ok()
        .or_else(|| command_output("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]))
        .unwrap_or_else(|| "unknown".into());

    // Get hostname
    let hostname = command_output("hostname", &["-s"])
        .or_else(|| command_output("hostname", &[]))
        .unwrap_or_else(|| "unknown".into());

    println!("cargo:rustc-env=VERGEN_GIT_SHA_SHORT={}", git_sha);
    println!("cargo:rustc-env=VERGEN_BUILD_TIMESTAMP={}", timestamp);
    println!("cargo:rustc-env=VERGEN_BUILD_HOST={}", hostname);

    // Re-run on every build-all.sh build, and when the checked-out commit
    // changes. .git/HEAD alone only changes when switching branches, so also
    // watch the branch ref it points to and the index (updated on commit).
    println!("cargo:rerun-if-env-changed=IBM1130_BUILD_TIMESTAMP");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
    if let Ok(head) = std::fs::read_to_string(".git/HEAD")
        && let Some(reference) = head.trim().strip_prefix("ref: ")
    {
        let ref_path = Path::new(".git").join(reference);
        if ref_path.exists() {
            println!("cargo:rerun-if-changed={}", ref_path.display());
        } else {
            // Ref is packed; watch packed-refs instead
            println!("cargo:rerun-if-changed=.git/packed-refs");
        }
    }
}
