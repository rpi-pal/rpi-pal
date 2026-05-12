 // I need to think about this some more.
 // How to marco polo when the pi doesn't have analog_read()?
 // I could use serial, or I could use network.
 //
 // I can't use another peripheral, because I can't guarentee that peripheral can be
 // trusted since `cargo test` runs asynchronously.
 //
 // Maybe there is another way...
