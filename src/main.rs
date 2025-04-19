use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use image::{ImageFormat, GenericImageView};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

#[derive(Parser)]
#[command(name = "climageproc")]
#[command(about = "A CLI tool for batch image processing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Resize images while maintaining aspect ratio
    Resize {
        /// Input file or directory
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory
        #[arg(short, long)]
        output: PathBuf,
        
        /// New width in pixels
        #[arg(short, long)]
        width: Option<u32>,
        
        /// New height in pixels
        #[arg(short, long)]
        height: Option<u32>,
    },
    
    /// Convert images to a different format
    Convert {
        /// Input file or directory
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory
        #[arg(short, long)]
        output: PathBuf,
        
        /// Target format (jpg, png, etc.)
        #[arg(short, long)]
        format: String,
    },
}

fn process_image(input_path: &Path, output_path: &Path, command: &Commands) -> Result<()> {
    let img = image::open(input_path)
        .with_context(|| format!("Failed to open image: {}", input_path.display()))?;

    let processed_img = match command {
        Commands::Resize { width, height, .. } => {
            if let (Some(w), Some(h)) = (width, height) {
                img.resize_exact(*w, *h, image::imageops::FilterType::Lanczos3)
            } else if let Some(w) = width {
                // Calculate height to maintain aspect ratio
                let ratio = img.height() as f32 / img.width() as f32;
                let h = (*w as f32 * ratio).round() as u32;
                img.resize(*w, h, image::imageops::FilterType::Lanczos3)
            } else if let Some(h) = height {
                // Calculate width to maintain aspect ratio
                let ratio = img.width() as f32 / img.height() as f32;
                let w = (*h as f32 * ratio).round() as u32;
                img.resize(w, *h, image::imageops::FilterType::Lanczos3)
            } else {
                img
            }
        }
        Commands::Convert { .. } => img,
    };

    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?
    }

    match command {
        Commands::Convert { format, .. } => {
            let format = match format.to_lowercase().as_str() {
                "jpg" | "jpeg" => ImageFormat::Jpeg,
                "png" => ImageFormat::Png,
                "gif" => ImageFormat::Gif,
                "webp" => ImageFormat::WebP,
                _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
            };
            processed_img.save_with_format(output_path, format)?
        }
        _ => {
            processed_img.save(output_path)?
        }
    }

    Ok(())
}

fn process_directory(input: &Path, output: &Path, command: &Commands) -> Result<()> {
    let entries: Vec<_> = std::fs::read_dir(input)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp"))
                .unwrap_or(false)
        })
        .collect();

    let progress_bar = ProgressBar::new(entries.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
    );

    entries.par_iter().try_for_each(|entry| {
        let input_path = entry.path();
        let file_name = input_path.file_name().unwrap();
        let mut output_path = PathBuf::from(output);
        output_path.push(file_name);

        if let Commands::Convert { format, .. } = command {
            output_path.set_extension(format);
        }

        let result = process_image(&input_path, &output_path, command);
        progress_bar.inc(1);
        result
    })?;

    progress_bar.finish_with_message("Done!");
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        cmd @ (Commands::Resize { input, output, .. } | Commands::Convert { input, output, .. }) => {
            if input.is_dir() {
                process_directory(input, output, cmd)?
            } else {
                process_image(input, output, cmd)?
            }
        }
    }

    Ok(())
}