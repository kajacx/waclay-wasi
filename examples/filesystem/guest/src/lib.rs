use std::fs::{File, create_dir_all};

mod bindings {
    wit_bindgen::generate!({
        path: "../filesystem.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct GuestComponent;

impl bindings::exports::waclay_wasi::examples::funcs::Guest for GuestComponent {
    fn test_filesystem() -> () {
        File::open("./file.txt").unwrap_err();

        File::create("./file.txt").unwrap_err();

        create_dir_all("./directory").unwrap_err();
    }
}
