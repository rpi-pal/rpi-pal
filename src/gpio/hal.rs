use core::convert::Infallible;

use super::{InputPin, IoPin, Level, OutputPin, Pin};

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::ErrorType for Pin {
    type Error = Infallible;
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::InputPin for Pin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::read(self) == Level::High)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::read(self) == Level::Low)
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::ErrorType for InputPin {
    type Error = Infallible;
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::InputPin for InputPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_low())
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::ErrorType for IoPin {
    type Error = Infallible;
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::InputPin for IoPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_low())
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::ErrorType for OutputPin {
    type Error = Infallible;
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::InputPin for OutputPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Self::is_set_low(self))
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::OutputPin for OutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        OutputPin::set_low(self);

        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        OutputPin::set_high(self);

        Ok(())
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::StatefulOutputPin for OutputPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(OutputPin::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(OutputPin::is_set_low(self))
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        OutputPin::toggle(self);

        Ok(())
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::OutputPin for IoPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        IoPin::set_low(self);

        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        IoPin::set_high(self);

        Ok(())
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::StatefulOutputPin for IoPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(IoPin::is_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(IoPin::is_low(self))
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        IoPin::toggle(self);

        Ok(())
    }
}

