use super::{Error, Segment, Spi};

#[cfg(feature = "embedded-hal")]
impl embedded_hal::spi::ErrorType for Spi {
    type Error = Error;
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::spi::SpiBus<u8> for Spi {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        Spi::read(self, words)?;
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        Spi::write(self, words)?;
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        Spi::transfer(self, read, write)?;
        Ok(())
    }

    fn transfer_in_place(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let write_buffer = buffer.to_vec();
        self.transfer(buffer, &write_buffer)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(feature = "embedded-hal-nb")]
impl embedded_hal_nb::spi::FullDuplex<u8> for Spi {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        if let Some(last_read) = self.last_read.take() {
            Ok(last_read)
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        let mut read_buffer: [u8; 1] = [0];

        Spi::transfer(self, &mut read_buffer, &[byte])?;
        self.last_read = Some(read_buffer[0]);

        Ok(())
    }
}

/// Simple implementation of [embedded_hal::spi::SpiDevice]
///
/// You only need this when using the `embedded_hal` Spi trait interface.
///
/// Slave-select is currently handled at the bus level.
/// This no-op device implementation can be used to satisfy the trait.
// TODO: The underlying crate::spi::Spi shall be split up to support proper slave-select handling here.
pub struct SimpleHalSpiDevice {
    bus: Spi,
}

#[cfg(feature = "embedded-hal")]
impl SimpleHalSpiDevice {
    pub fn new(bus: Spi) -> SimpleHalSpiDevice {
        SimpleHalSpiDevice { bus }
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::spi::SpiDevice<u8> for SimpleHalSpiDevice {
    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Error> {
        let clock_speed = self.bus.clock_speed()?;
        let bits_per_word = self.bus.bits_per_word()?;

        // Map the hal spi operations to segments, so they all can be executed together as one transaction
        let segments = operations
            .into_iter()
            .map(|op| match op {
                embedded_hal::spi::Operation::Read(read_buff) => Segment::with_read(read_buff),
                embedded_hal::spi::Operation::Write(write_buff) => Segment::with_write(write_buff),
                embedded_hal::spi::Operation::Transfer(read_buff, write_buff) => {
                    Segment::new(read_buff, write_buff)
                }
                embedded_hal::spi::Operation::TransferInPlace(buff) => {
                    Segment::with_settings(Some(buff), None, clock_speed, 0, bits_per_word, false)
                }
                // Map a segment with no read or write buffer, just to handle the delay
                embedded_hal::spi::Operation::DelayNs(delay_ns) => Segment::with_settings(
                    None,
                    None,
                    clock_speed,
                    (*delay_ns / 1000) as u16,
                    bits_per_word,
                    false,
                ),
            })
            .collect::<Vec<Segment>>();
        self.bus.transfer_segments(&segments)
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::spi::ErrorType for SimpleHalSpiDevice {
    type Error = Error;
}
