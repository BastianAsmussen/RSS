mod video;

use anyhow::Result;
use clap::Parser;
use gstreamer::{
    prelude::{ElementExt, GstObjectExt},
    ClockTime, MessageView, State,
};

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

    let pipeline = video::convert(
        "https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm",
        (192, 144),
    )?;
    let bus = pipeline.bus().expect("Failed to get bus!");
    pipeline.set_state(State::Playing)?;

    for msg in bus.iter_timed(ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(GstObjectExt::path_string),
                    err.error(),
                    err.debug()
                );

                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(State::Null)?;

    Ok(())
}
