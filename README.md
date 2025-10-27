# spatial-audio-sandbox-rs  
Tiny Rust demo mapping **(x, y, z)** → perceived audio position using `rodio`.  
  
## Run locally  
1) Install Rust: https://rustup.rs  
2) `cargo run`  
  
## Notes  
- Uses `SpatialSink` with left/right ear at ±0.2 m.  
- Emitter moves on a circle in the x/z plane to demonstrate panning.  
  
## Next steps  
- CLI params (radius/speed/frequency)  
- Basic visualization (Bevy)  
- Web/WASM experiment 
