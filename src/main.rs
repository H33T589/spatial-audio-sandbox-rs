use rodio::{OutputStream, SpatialSink, source::SineWave};
use std::{thread, time::Duration};

// Moves a 440 Hz tone around the listener on the x/z plane.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, handle) = OutputStream::try_default()?;
    let left_ear  = [-0.2, 0.0, 0.0];
    let right_ear = [ 0.2, 0.0, 0.0];

    let mut angle = 0.0f32;
    let radius = 0.8f32;

    let sink = SpatialSink::try_new(&handle, left_ear, right_ear, [radius, 0.0, 0.0])?;
    sink.append(SineWave::new(440));
    sink.set_volume(0.2);

    for _ in 0..240 {
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        sink.set_emitter_position([x, 0.0, z]);
        angle += 0.05;
        thread::sleep(Duration::from_millis(33));
    }
    sink.stop();
    Ok(())
}
