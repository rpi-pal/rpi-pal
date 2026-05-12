use rpi_pal::spi::{Spi, Mode, Bus, SlaveSelect};

const BUS: Bus = Bus::Spi0;
const SLV_SEL: SlaveSelect = SlaveSelect::Ss0;
const CLK_RTE: u32 = 8_000_000;
const MODE: Mode = Mode::Mode0;

#[test]
fn spi_marco_polo() {
    let test_byte: u8 = 0b1001;
    let success_byte: u8 = test_byte ^ 0xF;

    let spi: Spi = Spi::new(
        BUS,
        SLV_SEL,
        CLK_RTE,
        MODE
    ).expect("SPI failed to init");

    let write_buffer: [u8; 1] = [test_byte];
    let mut read_buffer: [u8; 1] = [0u8; 1];

    spi.transfer(&mut read_buffer, &write_buffer).expect("SPI Tranfer Failed");

    assert_eq!(read_buffer[0], success_byte, "SPI Bad Echo");
}
