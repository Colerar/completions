// https://github.com/strukturag/libheif

use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(name = "heif-enc", bin_name = "heif-enc")]
pub struct Args {
  /// show help
  #[arg(short = 'h', long = "help", num_args = 0)]
  help: bool,
  /// set output quality (0-100) for lossy compression
  #[arg(short = 'q', long = "quality", num_args = 1)]
  #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
  quality: u8,
  /// generate lossless output (-q has no effect)
  #[arg(short = 'L', long = "lossless", num_args = 0)]
  lossless: bool,
  /// generate thumbnail with maximum size (default: off)
  #[arg(short = 't', long = "thumb", num_args = 0)]
  thumb: bool,
  /// do not save alpha channel
  #[arg(long = "no-alpha", num_args = 0)]
  no_alpha: bool,
  /// do not save alpha channel in thumbnail image
  #[arg(long = "no-thumb-alpha", num_args = 0)]
  no_thumb_alpha: bool,
  /// enable logging output (more -v will increase logging level)
  #[arg(short = 'v', long = "verbose", num_args = 0..)]
  verbose: u8,
  /// show all encoder parameters
  #[arg(short = 'P', long = "params", num_args = 0)]
  params: bool,
  #[arg(short = 'b', long = "bit_depth", num_args = 1)]
  #[arg(value_parser = clap::value_parser!(u8).range(0..=16))]
  bit_depth: u8,
  /// set encoder parameter (NAME=VALUE)
  #[arg(short = 'p', num_args = 1)]
  encoder_param: String,
  /// encode as AVIF
  #[arg(short = 'A', long = "avif", num_args = 0)]
  avif: bool,
  /// list all available encoders for the selected output format
  #[arg(long = "list-encoders", num_args = 0)]
  list_encoders: bool,
  /// select encoder to use (the IDs can be listed with --list-encoders)
  #[arg(short = 'e', long = "encoder", num_args = 1)]
  #[arg(value_name = "ID")]
  encoder: u8,
  /// load all codec plugins in the directory
  #[arg(long = "plugin-directory")]
  #[arg(value_name = "DIR")]
  #[arg(value_hint = ValueHint::DirPath, num_args = 1)]
  plugin_directory: PathBuf,
  /// [deprecated] crop images to even width and height (odd sizes are not
  /// decoded correctly by some software)
  #[arg(short = 'E', long = "even-size", num_args = 0)]
  even_size: bool,
  /// nclx profile: color conversion matrix coefficients, default=6 (see h.273)
  #[arg(long = "matrix_coefficients", num_args = 1)]
  matrix_coefficients: u8,
  /// nclx profile: color primaries (see h.273)
  #[arg(long = "colour_primaries", num_args = 1)]
  colour_primaries: String,
  /// nclx profile: transfer characteristics (see h.273)
  #[arg(long = "transfer_characteristic", num_args = 1)]
  transfer_characteristic: String,
  /// nclx profile: full range flag, default: 1
  #[arg(long = "full_range_flag", num_args = 1)]
  full_range_flag: String,
  /// will write both an ICC and an nclx color profile if both are present
  #[arg(long = "enable-two-colr-boxes", num_args = 0)]
  enable_two_colr_boxes: bool,
  /// input image has premultiplied alpha
  #[arg(long = "premultiplied-alpha", num_args = 1)]
  premultiplied_alpha: String,
  /// enable XMP metadata compression (experimental)
  #[arg(long = "enable-metadata-compression", num_args = 0)]
  enable_metadata_compression: bool,
  /// measure encoding time, PSNR, and output file size
  #[arg(long = "benchmark", num_args = 0)]
  benchmark: bool,
  /// output filename (optional)
  #[arg(short = 'o', long = "output", value_name = "FILE", num_args = 1)]
  output: PathBuf,
  /// What file do you want to process?
  #[arg(value_name = "INPUT_FILE", num_args = 1)]
  #[arg(value_hint = ValueHint::FilePath, num_args = 1)]
  input: PathBuf,
}
