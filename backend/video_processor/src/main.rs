mod compressor;

use std::path::Path;

use anyhow::Result;
use clap::Parser;
use compressor::compress;
use video_rs::Url;

use tracing::{error, info};

/// Video processor for RSS.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the video file.
    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    video_rs::init().expect("Failed to initialize FFmpeg!");

    let args = Args::parse();

    let source = args.path.parse::<Url>()?;
    let target_resolutions = [
        (1920, 1080),
        (1280, 720),
        (640, 480),
        (480, 360),
        (426, 240),
        (192, 144),
    ];

    let mut tasks = Vec::new();
    for (width, height) in target_resolutions {
        let source = source.clone();
        let handle = tokio::spawn(async move {
            let destination = format!("video/{width}x{height}.mp4");
            let destination = Path::new(&destination);

            compress(source, destination, (width, height))
        });

        tasks.push(((width, height), handle));
    }

    for ((width, height), task) in tasks {
        match task.await {
            Ok(_) => info!("Finished generating resolution {width}x{height}."),
            Err(why) => error!("Failed to generate resolution {width}x{height}: {why}"),
        }
    }

    Ok(())
}
