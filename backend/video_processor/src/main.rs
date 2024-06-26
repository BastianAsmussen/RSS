mod video;

use anyhow::Result;
use clap::Parser;
use gstreamer::{
    prelude::{ElementExt, GstObjectExt},
    ClockTime, MessageView, State,
};

use tracing::{debug, error, info};

/// Video processor for RSS.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to the video.
    #[arg(short, long)]
    url: String,
    // The desired video width.
    #[arg(long)]
    width: u32,
    // The desired video height.
    #[arg(long)]
    height: u32,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    gstreamer::init()?;

    let args = Args::parse();

    let pipeline = video::convert(&args.url, (args.width, args.height))?;
    let bus = pipeline.bus().expect("Failed to get bus!");
    pipeline.set_state(State::Playing)?;

    for msg in bus.iter_timed(ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => {
                info!("Finished rescaling video.");

                break;
            }
            MessageView::Error(err) => {
                error!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(GstObjectExt::path_string),
                    err.error(),
                    err.debug()
                );

                break;
            }
            MessageView::StateChanged(state_changed) => {
                if !state_changed.src().map(|s| s == &pipeline).unwrap_or(false) {
                    continue;
                }

                debug!(
                    "State changed! ({:?} -> {:?})",
                    state_changed.old(),
                    state_changed.current()
                );
            }
            MessageView::Buffering(buffer) => {
                debug!("Buffering... ({}%)", buffer.percent());
            }
            _ => (),
        }
    }

    pipeline.set_state(State::Null)?;

    Ok(())
}
