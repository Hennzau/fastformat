use eyre::Context;

use std::path::Path;

fn main() -> eyre::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::env::set_current_dir(root).wrap_err("failed to set working dir")?;

    let file = Path::new("src/main.py");

    let uv = which::which("uv")?;
    let mut cmd = std::process::Command::new(&uv);

    cmd.arg("run").arg(file);

    if !cmd.status().wrap_err("failed to run python")?.success() {
        println!("python run failed");
    }

    Ok(())
}
