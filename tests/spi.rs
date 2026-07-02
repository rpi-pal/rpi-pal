use rpi_pal::spi::{Spi, Mode, Bus, SlaveSelect};
use std::thread::sleep;
use std::time::Duration;

const CLK_RTE: u32 = 100_000;

fn pretty_print_msg(msg: u8) -> String {
    format!("0b{:04b} (0x{:02x})", msg, msg)
}

#[test]
fn spi_marco_polo() {
    let test_byte: u8 = 0b1001;
    let success_byte: u8 = test_byte ^ 0xF;

    println!("Test    : {}", pretty_print_msg(test_byte));
    println!("Expected: {}", pretty_print_msg(success_byte));

    let spi = Spi::new(
        Bus::Spi0,
        SlaveSelect::Ss0,
        CLK_RTE,
        Mode::Mode0
    ).expect("SPI failed to init");

    sleep(Duration::from_millis(5));

    let write_buffer = [test_byte];
    let mut read_buffer = [0u8; 1];

    println!("\n[Transaction 1] Sending test byte...");
    spi.transfer(&mut read_buffer, &write_buffer).expect("SPI Transfer Failed");

    println!("  Sent    : {}", pretty_print_msg(test_byte));
    println!("  Received: {} <- garbage", pretty_print_msg(read_buffer[0]));

    let dummy = [0x00];
    let mut echo = [0u8; 1];

    println!("\n[Transaction 2] Getting response from slave...");
    spi.transfer(&mut echo, &dummy).expect("SPI Transfer Failed");

    println!("  Sent    : {} <- dummy", pretty_print_msg(dummy[0]));
    println!("  Received: {}", pretty_print_msg(echo[0]));

    assert_eq!(echo[0], success_byte, "SPI Bad Echo");
}