use anyhow::{anyhow, Result};
use gstreamer::{glib::object::Cast, Pipeline};

pub fn convert(url: &str, new_dims: (u32, u32)) -> Result<Pipeline> {
    let (width, height) = new_dims;
    let pipeline = gstreamer::parse::launch(&format!(
        "uridecodebin uri={url} ! videoscale ! video/x-raw,width={width},height={height} ! matroskamux ! filesink location={width}x{height}",
    ))?;
    let pipeline = pipeline
        .dynamic_cast::<Pipeline>()
        .map_err(|_| anyhow!("Failed to dynamically cast pipeline!"))?;

    Ok(pipeline)
}
