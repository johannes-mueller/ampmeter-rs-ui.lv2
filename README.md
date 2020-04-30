# ampmeter-rs-ui.lv2 – a UI prototype for a simple amplifier plugin

[rust-lv2](https://github.com/RustAudio/rust-lv2) are Rust bindings to write
[LV2 Plugins](https://lv2plug.in). The crate in the repo
[lv2-ui](https://github.com/johannes-mueller/lv2-ui) aims to fill the gap in
rust-lv2 for LV2 GUIs. This repo is the UI part of a LV2 plugin written in
Rust serving as a test plugin for LV2 UI in Rust.

## Build and installation

As in a Rust crate only one library is allowed, LV2 plugins with a GUI have to
come in two crates, one for the DSP part of the plugin, and one for the GUI.

Therefore the plugin has to be build in two steps.

### DSP part

The DSP part is in
[ampmeter-rs.lv2](https://github.com/johannes-mueller/ampmeter-rs.lv2).

The DSP part is the easy one. Just clone it and run `cargo build`.


### UI Part (this repo)

This is more tricky as it depends on three other crates that are not yet in
crates.io.

* [pugl-sys](https://github.com/johannes-mueller/pugl-sys), a rust sys-crate
  for [pugl](https://github.com/lv2/pugl/).

* [pugl-ui](https://github.com/johannes-mueller/pugl-sys), a stub of a GUI-toolkit

* [lv2-ui](https://github.com/johannes-mueller/lv2-ui), lv2 ui bindings

Clone all of these and this repo into one directory next to each other. In
pugl-sys, you need to fetch the submodules by
```
git submodule --init --update
```

Then run `cargo build` in ampmeter-rs-ui.lv2. It might work.


# Installation

Make the directory `~/.lv2/ampmeter-rs.lv2` and copy (or symlink) the following
files into it.

* `ampmeter-rs.lv2/manifest.ttl`
* `ampmeter-rs.lv2/ampmeter.ttl`
* `ampmeter-rs.lv2/target/debug/libampmeter_rs.so`
* `ampmeter-rs-ui.lv2/target/debug/libampmeter_rs_ui.so`

Then you can use a plugin host like carla or jalv to run the plugin
`https://johannes-mueller.org/lv2/ampmeter-rs#lv2`.
