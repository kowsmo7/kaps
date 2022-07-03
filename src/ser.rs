use anyhow::{Context, anyhow};
use clap::ValueEnum;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufWriter;
use crate::image::Image;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ImageFormat {
    Png,
    Infer,
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImageFormat::*;
        write!(f, "{}", match self {
            Png => "png",
            Infer => "infer",
        })
    }
}

fn inferred_image_format(path: &Path) -> Option<ImageFormat> {
    path
        .extension()
        .and_then(|ext| match ext.to_string_lossy().as_ref() {
            "png" => Some(ImageFormat::Png),
            _ => None,
        })
}

fn write(file: File, data: &[u8], width: u32, height: u32) -> anyhow::Result<()> {
    let w = BufWriter::new(file);
    let mut enc = png::Encoder::new(w, width, height);
    enc.set_color(png::ColorType::Rgba);

    enc.write_header()?.write_image_data(data)?;

    Ok(())
}

pub fn save_to_file(
    filename_fullpath: &str,
    image_format: ImageFormat,
    screenshot: Image,
) -> anyhow::Result<()> {
    let path = PathBuf::try_from(filename_fullpath)?;

    // essentially, we don't care about the image format
    let _image_format = match image_format {
        ImageFormat::Infer => inferred_image_format(&path)
            .ok_or_else(|| anyhow!("Unknown image format."))?,
        f => f,
    };

    let file = File::create(&path)
        .with_context(|| {
            format!("Failed to open file {}", path.to_string_lossy())
        })?;
    
    write(file, &screenshot.data, screenshot.width, screenshot.height)?;

    Ok(())
}
