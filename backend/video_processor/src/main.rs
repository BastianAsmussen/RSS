use std::{ops::DivAssign, path::Path};

use anyhow::Result;
use clap::Parser;
use tracing::info;
use video_rs::{encode::Settings, Decoder, Encoder, Url};

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
    video_rs::init().expect("Failed to initialize FFmpeg!");

    let args = Args::parse();

    let source = args.path.parse::<Url>()?;
    let mut decoder = Decoder::new(source)?;

    let (width, height) = decoder.size();
    info!("Video Resolution: {width}x{height}");

    let settings = Settings::preset_h264_yuv420p(640, 480, false);
    let mut encoder = Encoder::new(Path::new("low.mp4"), settings)?;

    for frame in decoder.decode_iter() {
        let Ok((time, mut frame)) = frame else {
            info!("No more frames.");
            break;
        };

        frame.div_assign(2);

        encoder.encode(&frame, time)?;
    }

    encoder.finish()?;

    Ok(())
}
