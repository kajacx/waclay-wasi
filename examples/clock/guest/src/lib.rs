use std::time::{Instant, SystemTime};

mod bindings {
    wit_bindgen::generate!({
        path: "../clock.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct GuestComponent;

impl bindings::exports::waclay_wasi::examples::funcs::Guest for GuestComponent {
    fn get_wall_clock() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn get_time_elapsed() -> u64 {
        let start = Instant::now();
        println!("This message will be voided, but it will still take time to travel to the host.");
        start.elapsed().as_nanos() as u64
    }
}
