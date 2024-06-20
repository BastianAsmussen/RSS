use anyhow::Result;
use tracing::info;
use video_rs::{
    encode::Settings, ffmpeg::format::Pixel, DecoderBuilder, Encoder, Location, Options,
};

pub fn compress(
    source: impl Into<Location>,
    destination: impl Into<Location>,
    new_dims: (u32, u32),
) -> Result<()> {
    let (width, height) = new_dims;

    let mut decoder = DecoderBuilder::new(source)
        .with_resize(video_rs::Resize::Fit(width, height))
        .with_options(&Options::preset_h264())
        .build()?;
    let (width, height) = decoder.size_out();

    info!("Encoding resolution: {width}x{height}");

    let settings = Settings::preset_h264_custom(
        width as usize,
        height as usize,
        Pixel::YUV420P,
        Options::preset_h264(),
    );
    let mut encoder = Encoder::new(destination, settings)?;
    for (i, frame) in decoder.decode_iter().enumerate() {
        let Ok((time, frame)) = frame else {
            break;
        };

        info!("Resolution={width}x{height}, Frame={i}, Time={time}");
        encoder.encode(&frame, time)?;
    }

    encoder.finish()?;

    Ok(())
}
