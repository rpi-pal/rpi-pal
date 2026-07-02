use rpi_pal::uart::{Uart, Parity};
use std::path::Path;
use std::time::Duration;

const DEVICE_PATH: &str = "/dev/ttyAMA0";
const BAUD_RATE: u32 = 115_200;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

fn pretty_print_msg(msg: u8) -> String {
    format!("0b{:04b} (0x{:02x})", msg, msg)
}

#[test]
fn uart_marco_polo() {
    let test_byte: u8 = 0b1001;
    let success_byte: u8 = test_byte ^ 0xF;

    println!("Test    : {}", pretty_print_msg(test_byte));
    println!("Expected: {}", pretty_print_msg(success_byte));

    let dev_path: &Path = Path::new(DEVICE_PATH);

    let mut uart: Uart = Uart::with_path(
        dev_path,
        BAUD_RATE,
        Parity::None,
        DATA_BITS,
        STOP_BITS
    ).expect("UART failed to init");

    let timeout: Duration = Duration::default();
    uart.set_read_mode(1, timeout).expect("Failed to set read mode");

    let write_buffer: [u8; 1] = [test_byte];
    let mut read_buffer: [u8; 1] = [0u8; 1];

    println!("\n[Write] Sending test byte...");
    uart.write(&write_buffer).expect("UART Write Failed");
    println!("  Sent    : {}", pretty_print_msg(write_buffer[0]));

    uart.drain().expect("Drain Failed");

    println!("\n[Read] Getting response byte...");
    uart.read(&mut read_buffer).expect("UART Read Failed");
    println!("  Received: {}", pretty_print_msg(read_buffer[0]));

    assert_eq!(read_buffer[0], success_byte, "UART Bad Echo");
}