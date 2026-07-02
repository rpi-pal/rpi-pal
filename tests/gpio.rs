use rpi_pal::gpio::{Gpio, Level, OutputPin, InputPin};
use std::time::Duration;
use std::thread::sleep;

fn pretty_print_msg(msg: u8) -> String {
    format!("0b{:04b} (0x{:02x})", msg, msg)
}

const TXIO: u8 = 17;
const RXIO: u8 = 27;

#[test]
fn gpio_marco_polo() {
    let test_byte: u8 = 0b1001;
    let success_byte = test_byte ^ 0xF;
    let mut echo_byte = 0;

    println!("Test    : {}", pretty_print_msg(test_byte));
    println!("Expected: {}", pretty_print_msg(success_byte));

    let gpio = Gpio::new().expect("GPIO failed to init");
    let mut tx_io: OutputPin = gpio.get(TXIO).expect("Failed to get out pin").into_output();
    let rx_io: InputPin = gpio.get(RXIO).expect("Failed to get in pin").into_input();

    for i in 0..4 {
        let bit = (test_byte >> i) & 1;

        tx_io.write(Level::from(bit));
        println!("  Sent    : {}", pretty_print_msg(tx_io.is_set_high() as u8));
        sleep(Duration::from_micros(5));

        echo_byte = (echo_byte << 1) | rx_io.read() as u8;
        println!("  Received: {}", pretty_print_msg(rx_io.is_high() as u8));
        sleep(Duration::from_micros(5));
    }


    println!("Reconstructed: {}", pretty_print_msg(echo_byte));

    assert_eq!(echo_byte, success_byte, "Gpio Bad Echo");
}