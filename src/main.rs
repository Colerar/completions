use anyhow::{Context, Ok};
use clap::{Command, CommandFactory, ValueEnum};
use clap_complete::{Generator, Shell};
use std::{
  env,
  fs::{self, File},
  io::BufWriter,
  path::Path,
};

use crate::cmd::*;

mod cmd;

fn main() -> anyhow::Result<()> {
  let dir = env::current_dir().unwrap();
  println!("Current working dir: {}", dir.to_string_lossy());
  let output = dir.join("completions");
  let mut cmds = [heif_enc::Args::command()];
  for mut cmd in &mut cmds {
    generate_completions(output.as_path(), &mut cmd)?;
  }
  Ok(())
}

fn create_dir_all_if_not<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
  let path = path.as_ref();
  if !path.exists() {
    fs::create_dir_all(path)
      .with_context(|| format!("Failed to create dir: {}", path.to_string_lossy()))?;
  }
  Ok(())
}

fn generate_completions<P: AsRef<Path>>(output_dir: P, cmd: &mut Command) -> anyhow::Result<()> {
  let mut output_dir = output_dir.as_ref().to_path_buf();
  output_dir.push(cmd.get_name());

  println!(
    "#== Generating completions for command `{}`",
    cmd.get_name()
  );

  for shell in Shell::value_variants() {
    let shell_name = shell.to_string();
    println!(
      "  = [{}] Generating completion for shell `{}`",
      cmd.get_name(),
      shell_name.as_str()
    );
    let mut output_file_path = output_dir.join(shell_name.as_str());
    create_dir_all_if_not(&output_file_path)?;
    output_file_path.push(shell.file_name(cmd.get_name()));

    let file = File::create(&output_file_path).with_context(|| {
      format!(
        "Failed to open file `{}` with `write` and `create_new` mode",
        output_file_path.to_string_lossy()
      )
    })?;

    let mut buf = BufWriter::new(file);
    shell.generate(cmd, &mut buf);
  }
  Ok(())
}
