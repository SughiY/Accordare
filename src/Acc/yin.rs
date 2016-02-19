extern crate libc;

use libc::{int16_t, c_float, size_t, c_int};
use Acc::audioData::{FILENAME, sample_from_buf, sample_from_file};
use std::ptr;

#[repr(C)]
pub struct yin_result_t{
    pitch:c_float,
    buffer_length:c_int,
    probability:c_float
}

impl yin_result_t {
    pub fn get_pitch(&self) -> f32 { self.pitch as f32 }
}

#[link(name = "Yin")]
extern{
    fn Yin_estimate_pitch(yin: *mut int16_t, buffer_length: int16_t) -> yin_result_t;
}

pub fn testWithFile(){
        let mut mut_audio = &mut sample_from_file(FILENAME)[1000 .. 2500];
        let result = unsafe{ Yin_estimate_pitch(mut_audio.as_mut_ptr(), 1500 as int16_t) };
        println!("Pitch is found to be {} with buffer length {} and probablity {} \n", result.pitch as f32, result.buffer_length as i32, result.probability as f32);
}

pub fn estimate_pitch(buffer: Vec<u8>) -> yin_result_t{
    let mut_audio = &mut sample_from_buf(buffer)[..];
    println!("got sample from buffer!");
    let result = unsafe {Yin_estimate_pitch(mut_audio.as_mut_ptr(), 1100 as int16_t)};
    if result.pitch as f32 > -2 as f32  { println!("Pitch is found to be {} with buffer length {} and probablity {} \n", result.pitch as f32, result.buffer_length as i32, result.probability as f32); }
    result
}
