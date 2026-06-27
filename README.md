# Wasi preview 2 binding for waclay Rust host

This is an attempt to add wasi preview 2.6.0 host bindings to [waclay](https://github.com/HemantKArya/waclay) so it can run wasi components.

This is an independent project and I am not affiliated with the maintainers of waclay.

## Capabilities

Here is an quick overview of implemented functions:

Feature | Empty impl | Inherit impl | Custom impl
--- | --- | --- | ---
Std IO | ✔️ | ✔️ | ✔️
Env variables | ✔️ | ✔️ | ✔️
Cli arguments | ✔️ | ✔️ | ✔️
Random generation | ✔️ | ✔️ | ✔️
Clocks | ✔️ | ✔️ | ✔️
Terminal* | ✔️ | ❌ | ❌
Filesystem | ❌ | ❌ | ❌
Networking | ❌ | ❌ | ❌

*) Terminal is NOT std in/out/err. It is a separate interface for detecting if a process is connected to a real terminal screen
for the purpose for setting colors, formatting output based on screen width, etc.

For now, `waclay-wasi` will just tell the component that it is not connected to any terminal.

### Legend

- **Empty impl** - Completely sandboxed mode - no file exists, network is disconnected, stdio is completely voided, time is stuck at 0, etc.
- **Inherit impl** - Functionality is inherited from the host - stdout is redirected to host's stdout, real file access, real env variables, etc.
- **Custom impl** - User-defined behaviour - arbitrary bytes as input to stdin, seeded random generator, custom cli arguments, etc.

## Goals

Current target is to have Filesystem and Networking in the "Empty impl" state, and everything else fully implemented.

### Tasks

- Make the WasiP2 interfaces fallible (returning `anyhow::Result`) so users can return a trap to the host.

## Usage

It is too early to use this crate unfortunately, as it requires a custom fork of waclay that I haven't even published yet. But soon I will make a release once I test it in my own project.
