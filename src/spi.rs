//! Adapters for blocking SPI buses and devices.

/// A blocking SPI bus from an async SPI bus.
///
/// The wrapped type `S` is an async-capable SPI bus, provided by your HAL.
pub struct SpinnySpiBus<S>(pub S);

impl<S> embedded_hal_async::spi::ErrorType for SpinnySpiBus<S>
where
    S: embedded_hal_async::spi::ErrorType,
{
    type Error = <S as embedded_hal_async::spi::ErrorType>::Error;
}

impl<S, W> embedded_hal::spi::SpiBus<W> for SpinnySpiBus<S>
where
    W: Copy + 'static,
    S: embedded_hal_async::spi::SpiBus<W>,
{
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        crate::spin_on(embedded_hal_async::spi::SpiBus::read(&mut self.0, words))
    }

    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        crate::spin_on(embedded_hal_async::spi::SpiBus::write(&mut self.0, words))
    }

    fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Self::Error> {
        crate::spin_on(embedded_hal_async::spi::SpiBus::transfer(
            &mut self.0,
            read,
            write,
        ))
    }

    fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        crate::spin_on(embedded_hal_async::spi::SpiBus::transfer_in_place(
            &mut self.0,
            words,
        ))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        crate::spin_on(embedded_hal_async::spi::SpiBus::flush(&mut self.0))
    }
}

/// A blocking SPI device from an async SPI device.
///
/// The wrapped type `S` is an async-capable SPI device, provided by your HAL
/// or another package.
pub struct SpinnySpiDevice<S>(pub S);

impl<S> embedded_hal_async::spi::ErrorType for SpinnySpiDevice<S>
where
    S: embedded_hal_async::spi::ErrorType,
{
    type Error = <S as embedded_hal_async::spi::ErrorType>::Error;
}

impl<S, W> embedded_hal::spi::SpiDevice<W> for SpinnySpiDevice<S>
where
    W: Copy + 'static,
    S: embedded_hal_async::spi::SpiDevice<W>,
{
    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, W>],
    ) -> Result<(), Self::Error> {
        crate::spin_on(embedded_hal_async::spi::SpiDevice::transaction(
            &mut self.0,
            operations,
        ))
    }
}
