//initialize the cpal audio library

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result <(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    println!("Available audio devices: ");

    // Print out available audio devices
    for device in host.devices() {
        println!(" - {}", device.name()?);
    }
    Ok(())

    let device = host
        .default_input_device()
        .expect("Failed to get default input device, none available.");

    println!("Using default input device: {}", device.name()?);

    let config = device.default_input_config()?;

    println!("Default input config: {:?}", config);

    // Build and run the input stream based on the sample format
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                let max = data
                    .iter()
                    .map(|sample| sample.abs())
                    .fold(0.0_f32, f32::max);

                // Print the peak amplitude to the console
                println!("Peak amplitude: {:.4}", max);
            },
            move |err| {
                // Error handling for the audio stream
                eprintln!("Audio streaming error: {}", err);
            },
            None,
        )?,

        // Handle the i16 sample format
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data: &[i16], _| {
                let max = data
                    .iter()
                    .map(|sample| sample.abs() as f32 / i16::MAX as f32)
                    .fold(0.0_f32, f32::max);

                // Print the peak amplitude to the console
                println!("Peak amplitude: {:.4}", max);
            },
            move |err| {
                // Error handling for the audio stream
                eprintln!("Audio streaming error: {}", err);
            },
            None,
        )?,

        format => panic!("Unsupported sample format: {:?}", format),
    };

    // Initialize the audio stream
    stream.play()?;

    println!("Listening for audio input... Press Ctrl+C to stop.");

    // Loop to keep the main thread running
    // while streaming is active so the stream
    // can continue to process audio data.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}