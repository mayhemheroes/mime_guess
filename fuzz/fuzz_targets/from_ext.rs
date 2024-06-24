#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|value: String| {
    mime_guess::from_ext(&value);
});