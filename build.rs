use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use vergen_gix::{BuildBuilder, CargoBuilder, Emitter, GixBuilder, RustcBuilder};

const DEFAULT_PI_MONO_ROOT: &str = "/data/projects/pi-mono";
const REPO_LOCAL_PI_MONO_ROOT: &str = "legacy_pi_mono_code/pi-mono";

fn candidate_models_generated_paths(manifest_dir: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(root) = env::var_os("PI_MONO_ROOT") {
        candidates.push(PathBuf::from(root).join("packages/ai/src/models.generated.ts"));
    }
    candidates
        .push(PathBuf::from(DEFAULT_PI_MONO_ROOT).join("packages/ai/src/models.generated.ts"));
    candidates.push(
        manifest_dir
            .join(REPO_LOCAL_PI_MONO_ROOT)
            .join("packages/ai/src/models.generated.ts"),
    );
    candidates
}

fn copy_models_generated_ts(
    out_dir: &Path,
    manifest_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=PI_MONO_ROOT");

    let candidates = candidate_models_generated_paths(manifest_dir);
    let source = candidates
        .iter()
        .find(|path| path.is_file())
        .cloned()
        .ok_or_else(|| {
            let checked = candidates
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("unable to locate pi-mono models.generated.ts; checked: {checked}")
        })?;

    println!("cargo:rerun-if-changed={}", source.display());
    fs::copy(source, out_dir.join("models.generated.ts"))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    copy_models_generated_ts(&out_dir, &manifest_dir)?;

    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let cargo = CargoBuilder::default().target_triple(true).build()?;
    let gix = GixBuilder::default().sha(true).dirty(true).build()?;
    let rustc = RustcBuilder::default().semver(true).build()?;

    let mut emitter = Emitter::default();
    // Offloaded builds can temporarily miss git objects and trigger fallback warnings.
    // Keep default env fallbacks, but suppress warning noise in build output.
    emitter
        .quiet()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&gix)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
