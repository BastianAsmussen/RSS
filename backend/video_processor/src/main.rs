mod video;

use anyhow::Result;
use clap::Parser;

/// Video processor for RSS.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the video file.
    #[arg(short, long)]
    path: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    gstreamer::init()?;

    video::temp()?;

    Ok(())
}
