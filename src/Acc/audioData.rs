
extern crate hound;

use std::io::{Read, BufReader};
use std::path::Path;

pub static FILENAME:&'static str = "/Users/user/Dropbox/cours/SI/WEBAPP/accordare/src/Acc/openE.wav";

pub fn sample_from_file<P: AsRef<Path>>(filename:P) -> Vec<i16> {
    let result = hound::WavReader::open(filename);
    sample(result)
}

pub fn sample_from_buf(buf:Vec<u8>) -> Vec<i16> {
    let slice_buf = &buf[..];
    let result = hound::WavReader::new( slice_buf );
    sample(result)
}

    pub fn sample<R:Read>(res : hound::Result<hound::WavReader<R>>) -> Vec<i16>
{
    match res {
        Err(err)=> {
            println!("openWave error{:?}",err); 
            vec![]
        },
        Ok(mut wave)=> {
            println!("wave Size{:?}", wave.len()); 
            let sample:Vec<i16> = wave.samples::<i16>()
                .filter(|value|{ match value{
                    &Err(ref err) => {
                        println!("sample Error{:?}", err);
                        false
                    },
                    &Ok(..) => true 
                }})
            .map(|value| {
                let x:i32 = 2;
                let max16Bit = x.pow(15);
                let maxDesired = x.pow(10) / x;
                ((value.unwrap() as f32 / max16Bit as f32 ) * (maxDesired as f32)) as i16
            }).collect();
            println!("sample Size{:?}",sample.len()); 
            sample
        }
    }
}
