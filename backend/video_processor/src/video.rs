use anyhow::Result;
use gstreamer::{Caps, ElementFactory, Pipeline};

pub fn convert(url: &str, new_dims: (u32, u32)) -> Result<()> {
    /* Command:
        * gst-launch-1.0 -e uridecodebin uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm name=decode ! \
    videoscale ! 'video/x-raw,width=360,height=240' ! \
    x264enc ! queue ! mp4mux name=mp4mux ! filesink location=0000.mp4 \
    decode. ! audioconvert ! lamemp3enc bitrate=128 ! queue ! mp4mux.
         */
    let pipeline = Pipeline::default();

    let decoder = ElementFactory::make("uridecodebin")
        .property("uri", url)
        .build()?;

    // let video_caps = VideoCapsBuilder::new().width(1920).height(1080).build();
    let scaler = ElementFactory::make("videoscale").build()?;

    Ok(())
}
