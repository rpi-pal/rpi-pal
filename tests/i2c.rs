use rpi_pal::i2c::I2c;
use std::time::Duration;
use std::thread::sleep;

const SLV_ADD: u16 = 0x08;

fn pretty_print_msg(msg: u8) -> String {
    format!("0b{:04b} (0x{:02x})", msg, msg)
}

#[test]
fn i2c_marco_polo() {
    let test_byte: u8 = 0b1001;
    let success_byte: u8 = test_byte ^ 0xF;

    println!("Test    : {}", pretty_print_msg(test_byte));
    println!("Expected: {}", pretty_print_msg(success_byte));

    let mut i2c = I2c::new().expect("I2C failed to init");
    i2c.set_slave_address(SLV_ADD).expect("Bad slave addr");

    let write_buffer: [u8; 1] = [test_byte];
    let mut read_buffer: [u8; 1] = [0u8; 1];
    sleep(Duration::from_millis(5));


    println!("\n[WriteRead] Echo!");
    i2c.write_read(&write_buffer, &mut read_buffer).expect("i2c Transfer Failed");

    read_buffer[0] = read_buffer[0] & 0xF;

    println!("  Sent    : {}", pretty_print_msg(write_buffer[0]));
    println!("  Received: {}", pretty_print_msg(read_buffer[0]));

    assert_eq!(read_buffer[0], success_byte, "i2c Bad Echo");
}