#[cfg(any(feature = "embedded-hal-nb"))]
use super::{Error, Queue, Uart};

#[cfg(feature = "embedded-hal-nb")]
impl embedded_hal_nb::serial::ErrorType for Uart {
    type Error = Error;
}

#[cfg(feature = "embedded-hal-nb")]
impl embedded_hal_nb::serial::Error for Error {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        embedded_hal_nb::serial::ErrorKind::Other
    }
}

#[cfg(feature = "embedded-hal-nb")]
impl embedded_hal_nb::serial::Read<u8> for Uart {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        let mut buffer = [0u8; 1];
        if Uart::read(self, &mut buffer)? == 0 {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        } else {
            Ok(buffer[0])
        }
    }
}

#[cfg(feature = "embedded-hal-nb")]
impl embedded_hal_nb::serial::Write<u8> for Uart {
    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        if Uart::write(self, &[word])? == 0 {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        Uart::flush(self, Queue::Output)?;

        Ok(())
    }
}

