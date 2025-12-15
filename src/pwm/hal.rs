#[cfg(feature = "embedded-hal")]
use super::{Error, Pwm};

#[cfg(feature = "embedded-hal")]
impl embedded_hal::pwm::ErrorType for Pwm {
    type Error = Error;
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::pwm::Error for Error {
    fn kind(&self) -> embedded_hal::pwm::ErrorKind {
        embedded_hal::pwm::ErrorKind::Other
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::pwm::SetDutyCycle for Pwm {
    fn max_duty_cycle(&self) -> u16 {
        u16::MAX
    }

    fn set_duty_cycle(&mut self, duty: u16) -> std::result::Result<(), Self::Error> {
        let _ = Pwm::set_duty_cycle(self, (duty as f64) / (self.max_duty_cycle() as f64));
        Ok(())
    }
}
