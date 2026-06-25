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
        let mut args_all: Vec<_> = std::env::args().collect();
        args_all.remove(0)
    }

    fn get_cli_args() -> Vec<String> {
        let mut args_all: Vec<_> = std::env::args().collect();
        args_all.remove(0);
        args_all
    }
}
