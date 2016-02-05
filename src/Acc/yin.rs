extern crate libc;

use libc::{int16_t, c_float, size_t, c_int};
use Acc::audioData::audio;
use std::ptr;

#[repr(C)]
struct yin_result_t{
    pitch:c_float,
    buffer_length:c_int,
    probability:c_float
}

#[link(name = "Yin")]
extern{
    fn Yin_estimate_pitch(yin: *mut int16_t) -> yin_result_t;
}

pub fn testWithFile(){
    unsafe {
        let mut mut_audio = audio;
        let result = Yin_estimate_pitch(mut_audio.as_mut_ptr());
        println!("Pitch is found to be {} with buffer length {} and probablity {} \n", result.pitch as f32, result.buffer_length as i32, result.probability as f32);
    }
}

