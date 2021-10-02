#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let reader = std::io::Cursor::new(data);
    let mut de = jpeg_decoder::Decoder::new(reader);
    de.decode();
});
