#[deny(missing_docs)]
use std::thread;

///Päälaskuri
fn laskin(kilpailu: u64, tiiviste: u64)
{
    let _ = kilpailu*tiiviste;
}
///Laskee tietyn kilpailun kaikki tiivisten omaavat syötteet omassa threadissaan.
///Ainoa FFI-funktio, koska se on kallista ja muita ei tarvita.,
#[no_mangle]
pub extern fn laske(kilpailu: u64, tiiviste: u64)
{
    thread::spawn(move || {laskin(kilpailu, tiiviste);});
}
