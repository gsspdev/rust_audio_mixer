use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;

const host : HostTrait = cpal::default_host();
const device : DeviceTrait= host.default_output_device().expect("no output device available");


fn main() {
    
    const stream : DeviceTrait.stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // react to stream events and read or write stream data here.
        },
        move |err| {
            println!("the audio stream is offline") // react to errors here.
        },
        None // None=blocking, Some(Duration)=timeout
    );
    println!("Hello, world!");
}
