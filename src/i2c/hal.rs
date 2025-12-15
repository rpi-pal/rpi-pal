use super::{Error, I2c};

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::ErrorType for I2c {
    type Error = Error;
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        if let Error::Io(e) = self {
            use std::io::ErrorKind::*;

            match e.kind() {
                /* ResourceBusy | */ InvalidData => embedded_hal::i2c::ErrorKind::Bus,
                WouldBlock => embedded_hal::i2c::ErrorKind::ArbitrationLoss,
                _ => embedded_hal::i2c::ErrorKind::Other,
            }
        } else {
            embedded_hal::i2c::ErrorKind::Other
        }
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::I2c for I2c {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation],
    ) -> Result<(), Self::Error> {
        self.set_slave_address(u16::from(address))?;
        for op in operations {
            match op {
                embedded_hal::i2c::Operation::Read(buff) => {
                    self.read(buff)?;
                }
                embedded_hal::i2c::Operation::Write(buff) => {
                    self.write(buff)?;
                }
            }
        }

        Ok(())
    }
}
