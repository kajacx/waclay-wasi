mod bindings {
    wit_bindgen::generate!({
        path: "../env.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

/// Struct off of which the implementation will hang
///
/// The name of this struct is not significant.
struct GuestComponent;

impl bindings::exports::waclay_wasi::examples::funcs::Guest for GuestComponent {
    fn get_env_var_all() -> Vec<(String, String)> {
        std::env::vars().collect()
    }

    fn get_env_var(name: String) -> Option<String> {
        std::env::var(name).ok()
    }

    fn get_program_name() -> String {
        std::env::args().next().unwrap_or_default()
    }

    fn get_cli_args() -> Vec<String> {
        std::env::args().skip(1).collect()
    }
}
