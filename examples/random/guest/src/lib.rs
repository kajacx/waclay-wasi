mod bindings {
    wit_bindgen::generate!({
        path: "../random.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct GuestComponent;

impl bindings::exports::waclay_wasi::examples::funcs::Guest for GuestComponent {
    fn get_random_bytes(len: u32) -> Vec<u8> {
        let mut bytes = vec![0u8; len as usize];
        getrandom::fill(&mut bytes).unwrap();
        bytes
    }
}
