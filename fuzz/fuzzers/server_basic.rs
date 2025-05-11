#![no_main]

#[macro_use]
extern crate libfuzzer_sys;

use std::io::BufRead;

use multipart::mock::ServerRequest;
use multipart::server::Multipart;

use log::info;

mod logger;

const BOUNDARY: &'static str = "--12--34--56";

fuzz_target!(|data: &[u8]| {
    logger::init();

    info!("Fuzzing started! Data len: {}", data.len());

    do_fuzz(data);

    info!("Finished fuzzing iteration");
});

fn do_fuzz(data: &[u8]) {
    if data.len() < BOUNDARY.len() {
        return;
    }

    let req = ServerRequest::new(data, BOUNDARY);

    info!("Request constructed!");

    let mut multipart = if let Ok(multi) = Multipart::from_request(req) {
        multi
    } else {
        panic!("This shouldn't have failed")
    };

    // A lot of requests will be malformed
    while let Ok(Some(mut entry)) = multipart.read_entry() {
        info!("read_entry() loop!");
        loop {
            let consume = entry.data.fill_buf().expect("This shouldn't fail").len();

            info!("Consume amt: {}", consume);

            if consume == 0 {
                break;
            }
            entry.data.consume(consume);
        }
    }
}
