
use failure::Error;

use alto::{Alto, Buffer, Context, Mono, Stereo, StreamingSource};

use lewton::inside_ogg::OggStreamReader;
use std::fs::File;

pub fn initialize_context() -> Result<Context, Error> {
    let alto = Alto::load_default()?;

    for s in alto.enumerate_outputs() {
        println!("Found device: {}", s.to_str()?);
    }

    let device = alto.open(None)?; // Opens the default audio device
    Ok(device.new_context(None)?) // Creates a default context
}


const NUM_BUFFER_POOL: i32 = 100;


/// This could be useful for music, or other long audio files to reduce memory footprint
pub struct StreamingOggSource{
    sample_rate: i32,
    streaming_source: StreamingSource,
    streaming_reader: OggStreamReader<File>,
}

impl StreamingOggSource{
    pub fn new(file_path: &str, context: Context) -> Result<Self, Error>{
        let f = File::open(file_path)?;

        // Prepare the reading
        let srr = OggStreamReader::new(f)?;

        // Prepare the playback.
        let str_src = context.new_streaming_source()?;
        let sample_rate = srr.ident_hdr.audio_sample_rate as i32;

        if srr.ident_hdr.audio_channels > 2 {
            // the openal crate can't process these many channels directly
            println!("Stream error: {} channels are too many!", srr.ident_hdr.audio_channels);
        }

        Ok(StreamingOggSource{
            sample_rate,
            streaming_reader: srr,
            streaming_source: str_src
        })
    }


    pub fn stream(&mut self, context: &Context) -> Result<(), Error>{
        if let Some(pck_samples) = (self.streaming_reader.read_dec_packet_itl())? {

            let buf = if self.streaming_source.buffers_queued() < NUM_BUFFER_POOL{
                 match self.streaming_reader.ident_hdr.audio_channels {
                    1 => context.new_buffer::<Mono<i16>,_>(&pck_samples, self.sample_rate),
                    2 => context.new_buffer::<Stereo<i16>,_>(&pck_samples, self.sample_rate),
                    n => panic!("unsupported number of channels: {}", n),
                }?
            }else{
                let mut buf = self.streaming_source.unqueue_buffer()?;

                    match self.streaming_reader.ident_hdr.audio_channels {
                        1 => {buf.set_data::<Mono<i16>, &[i16]>(&pck_samples, self.sample_rate)?;},
                        2 => {buf.set_data::<Stereo<i16>, &[i16]>(&pck_samples, self.sample_rate)?;},
                        n => panic!("unsupported number of channels: {}", n),
                    }
                    buf
            };
            self.streaming_source.queue_buffer(buf)?;
        }
        Ok(())
    }
}


pub fn load_buffer_from_ogg_file(file_path: &str, context: &Context) -> Result<Buffer, Error> {
    let f = File::open(file_path)?;

    // Prepare the reading
    let mut srr = OggStreamReader::new(f)?;

    let sample_rate = srr.ident_hdr.audio_sample_rate as i32;

    if srr.ident_hdr.audio_channels > 2 {
        // the openal crate can't process these many channels directly
        println!("Stream error: {} channels are too many!", srr.ident_hdr.audio_channels);
    }


    let mut buffer_data = Vec::new();

    while let Some(pck_samples) = srr.read_dec_packet_itl()?{
        buffer_data.extend(pck_samples);
    }

    let buffer = match srr.ident_hdr.audio_channels {
        1 => context.new_buffer::<Mono<i16>,_>(&buffer_data, sample_rate),
        2 => context.new_buffer::<Stereo<i16>,_>(&buffer_data, sample_rate),
        n => panic!("unsupported number of channels: {}", n),
    }?;
    Ok(buffer)
}
