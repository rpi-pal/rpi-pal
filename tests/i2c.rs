use rpi_pal::i2c::I2c;

const SLV_ADD: u16 = 0x08;

#[test]
fn i2c_marco_polo() {
    let test_byte: u8 = 0b1001;
    let success_byte: u8 = test_byte ^ 0xF;

    let mut i2c = I2c::new().expect("I2C failed to init");
    i2c.set_slave_address(SLV_ADD).expect("Bad slave addr");

    let write_buffer: [u8; 1] = [test_byte];
    let mut read_buffer: [u8; 1] = [0u8; 1];

    i2c.write_read(&write_buffer, &mut read_buffer).expect("i2c Transfer Failed");

    assert_eq!(read_buffer[0], success_byte, "i2c Bad Echo");
}
