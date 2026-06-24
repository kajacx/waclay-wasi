mod bindings {
    wit_bindgen::generate!({
        path: "../stdio.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct GuestComponent;

impl bindings::exports::waclay_wasi::examples::funcs::Guest for GuestComponent {
    fn print_stdout(text: String) {
        println!("[Rust guest writing to stdout]: {text}");
    }

    fn print_stderr(text: String) {
        eprintln!("[Rust guest writing to stderr]: {text}");
    }

    fn read_stdin() -> String {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("read line from stdin inside of the Rust guest");
        let line = buffer.trim_end();
        format!("[Rust guest reading stdin]: {line}")
    }
}
