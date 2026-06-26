use std::cmp::min;

use waclay::*;
use waclay_wasi::{AsWasiP2Ctx, WasiP2Ctx, WasiP2InputStream, WasiP2OutputStream};

mod bindings;

// The bytes of the component.
const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/example_stdio_guest.wasm");

struct StoreData {
    ctx: WasiP2Ctx,
}

impl AsWasiP2Ctx for StoreData {
    fn as_wasi_ref(&self) -> &WasiP2Ctx {
        &self.ctx
    }

    fn as_wasi_mut(&mut self) -> &mut WasiP2Ctx {
        &mut self.ctx
    }
}

pub fn main() {
    // Prepare store, component and exports

    let engine = Engine::new(wasmi_runtime_layer::Engine::default());
    let mut store = Store::new(
        &engine,
        StoreData {
            ctx: waclay_wasi::WasiP2Ctx::new(),
        },
    );

    let mut linker = Linker::default();
    waclay_wasi::add_to_linker(&mut linker, &mut store).unwrap();

    let component = Component::new(&engine, WASM).unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();

    let print_stdout = bindings::exports_funcs::get_print_stdout(&instance, &mut store).unwrap();
    let print_stderr = bindings::exports_funcs::get_print_stderr(&instance, &mut store).unwrap();
    let read_stdin = bindings::exports_funcs::get_read_stdin(&instance, &mut store).unwrap();

    // Ignoring std io and pretending it's closed is already the default behaviour,
    // but you can call "clear" methods to make sure in case wasi ctx was modified before.

    print_stdout
        .call(&mut store, "Voided message to stdout".to_string())
        .unwrap();

    print_stderr
        .call(&mut store, "Voided message to stderr".to_string())
        .unwrap();

    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: ");

    // Redirect / inherit stdio from the host
    store
        .data_mut()
        .as_wasi_mut()
        .clear_all()
        .inherit_stdin()
        .inherit_stdout()
        .inherit_stderr();

    print_stdout
        .call(&mut store, "Redirected message to stdout".to_string())
        .unwrap();

    print_stderr
        .call(&mut store, "Redirected message to stderr".to_string())
        .unwrap();

    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: line1");
    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: line2");
    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: ");

    // Capture stdio with a custom stream

    store
        .data_mut()
        .as_wasi_mut()
        .clear_all()
        .set_stdin(Box::new(PreparedInputStream {
            data: "prepared1\nprepared2".as_bytes().to_vec(),
            offset: 0,
        }))
        .set_stdout(Box::new(CapturingOutputStream(vec![])))
        .set_stderr(Box::new(CapturingOutputStream(vec![])));

    print_stdout
        .call(&mut store, "Captured message to stdout".to_string())
        .unwrap();

    print_stderr
        .call(&mut store, "Captured message to stderr".to_string())
        .unwrap();

    let stdout_captured_bytes = &store
        .data()
        .as_wasi_ref()
        .stdout
        .as_any()
        .downcast_ref::<CapturingOutputStream>()
        .unwrap()
        .0;
    assert_eq!(
        str::from_utf8(stdout_captured_bytes).unwrap(),
        "[Rust guest writing to stdout]: Captured message to stdout\n"
    );

    let stderr_captured_bytes = &store
        .data()
        .as_wasi_ref()
        .stderr
        .as_any()
        .downcast_ref::<CapturingOutputStream>()
        .unwrap()
        .0;
    assert_eq!(
        str::from_utf8(stderr_captured_bytes).unwrap(),
        "[Rust guest writing to stderr]: Captured message to stderr\n"
    );

    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: prepared1");
    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: prepared2");
    let result = read_stdin.call(&mut store, ()).unwrap();
    assert_eq!(result, "[Rust guest reading stdin]: ");

    // Test exit here, because I don't want to add another example just for it
    store.data_mut().as_wasi_mut().clear_all().inherit_stdout();

    let exit = bindings::exports_funcs::get_exit(&instance, &mut store).unwrap();
    exit.call(&mut store, ()).unwrap_err(); // should return an error

    // WARNING! Calling `exit` doesn't actually crash the instance, it just returns early!
    // It is up to the user to never use an instance again if it returned an error unexpectedly.
    // The following call will happen to work and even print the message ,
    // but accessing an instance that called `exit` is a very bad idea!
    // print_stdout.call(&mut store, "Hello?".to_string()).unwrap(); // <- will still work
}

#[derive(Debug)]
struct CapturingOutputStream(Vec<u8>);

impl WasiP2OutputStream for CapturingOutputStream {
    fn output_stream_check_write(&mut self) -> Result<u64, waclay_wasi::bindings::StreamError> {
        Ok(4 * 1024 * 1024)
    }

    fn output_stream_write(
        &mut self,
        mut contents: Vec<u8>,
    ) -> Result<(), waclay_wasi::bindings::StreamError> {
        self.0.append(&mut contents);
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
struct PreparedInputStream {
    data: Vec<u8>,
    offset: usize,
}

impl WasiP2InputStream for PreparedInputStream {
    fn input_stream_blocking_read(
        &mut self,
        len: u64,
    ) -> Result<Vec<u8>, waclay_wasi::bindings::StreamError> {
        let len = len as usize;
        let end = min(self.offset + len, self.data.len());

        let result = self.data[self.offset..end].to_vec();

        self.offset = end;
        Ok(result)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
