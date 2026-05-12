use rpi_pal::gpio::{Gpio, Level, Mode, IoPin, OutputPin};

const PINS: [u8; 4] = [5, 6, 13, 19];
const INTR: u8 = 26;

#[test]
fn gpio_marco_polo() {
    let test_byte: u8 = 0b1001;
    let success_byte = test_byte ^ 0xF;

    let gpio = Gpio::new().expect("GPIO failed to init");
    let mut pins: Vec<IoPin> = PINS
        .iter()
        .map(|&pin_num|
            gpio.get(pin_num).expect("Failed to get pin")
                .into_io(Mode::Output))
        .collect();

    let mut interupt: OutputPin = gpio.get(INTR).unwrap().into_output();
    interupt.set_low();

    for (pin, lv) in pins.iter_mut().zip(
        (0..PINS.len()).map(|shift| (test_byte >> shift) & 1)
    ) {
        pin.set_mode(Mode::Output);
        pin.write(Level::from(lv));
    }

    // a rising edge tells the pico we are ready to test the gpio output
    interupt.set_high();

    let mut result: u8 = 0;
    for (pin, i) in pins.iter_mut().zip(0..PINS.len()) {
        pin.set_mode(Mode::Input);
        if pin.read() == Level::High {
            result |= 1 << i; 
        }
    }
    assert_eq!(result, success_byte, "Gpio Bad Echo");
}
