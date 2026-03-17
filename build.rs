use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=frontend/**/*");
    let status = Command::new("bun")
        .args(["build", "frontend/index.ts", "--outdir", "public/frontend/"])
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => eprintln!("bun build exited with: {}", s),
        Err(e) => eprintln!("bun not found, skipping frontend build: {}", e),
    }
}
