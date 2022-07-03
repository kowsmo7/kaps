//! kaps
//! Simple and performant screenshot application for Xorg.
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    usage = "kaps [OPTIONS] -- <slop-args>",
)]
struct Args {
    #[clap(
        arg_enum,
        short,
        long,
        value_parser,
        default_value_t = ser::ImageFormat::Infer,
        display_order(1),
        help = "Output format, inferring from the file name by default",
    )]
    image_format: ser::ImageFormat,

    #[clap(
        short,
        long,
        value_parser,
        display_order(2),
        help = "Filepath to which the screenshot will be saved, relative to your home folder",
    )]
    file_path: String, 

    #[clap(hide(true))]
    slop_args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    slop::ensure_arguments(&args.slop_args)?;

    let display = x11::connect()?;
    let size = x11::display_size(&display)?;
    let x_image = x11::root_image(&display, size)?;

    let selection = slop::get_slop_selection(args.slop_args)?;
    let screenshot = image::Image::from_x11(x_image, selection);

    ser::save_to_file(&args.file_path, args.image_format, screenshot)?;

    Ok(())
}

mod ser;
mod x11;
mod image;
mod slop;
