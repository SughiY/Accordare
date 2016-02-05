
extern crate hound;

pub fn sample() -> Vec<i16> {
    match hound::WavReader::open("/Users/user/Dropbox/cours/SI/WEBAPP/accordare/src/Acc/openE.wav") {
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
