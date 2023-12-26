//! Adapters for blocking I2C controllers.

/// A blocking I2C controller from an async I2C controller.
///
/// The wrapped type `I` is an async-capable I2C controller, provided by your HAL.
///
/// ```
/// use embedded_spinny::SpinnyI2c;
/// use embedded_hal::i2c::I2c;
/// # use embedded_hal_async::i2c::{I2c as AsyncI2c, ErrorType, SevenBitAddress, Operation};
/// # struct YourI2c; impl ErrorType for YourI2c { type Error = core::convert::Infallible; }
/// # impl AsyncI2c<SevenBitAddress> for YourI2c {
/// #   async fn transaction(&mut self, _: SevenBitAddress, _: &mut [Operation<'_>]) -> Result<(), Self::Error> {
/// #       Ok(())
/// #   }
/// # }
///
/// let your_i2c = // Provided by your driver package...
/// #   YourI2c;
/// let mut spinny_i2c = SpinnyI2c(your_i2c);
///
/// fn needs_i2c<I: I2c>(_: &mut I) { /* ... */}
///
/// needs_i2c(&mut spinny_i2c);
/// ```
pub struct SpinnyI2c<I>(pub I);

impl<I> embedded_hal_async::i2c::ErrorType for SpinnyI2c<I>
where
    I: embedded_hal_async::i2c::ErrorType,
{
    type Error = <I as embedded_hal_async::i2c::ErrorType>::Error;
}

impl<I, A> embedded_hal::i2c::I2c<A> for SpinnyI2c<I>
where
    I: embedded_hal_async::i2c::I2c<A>,
    A: embedded_hal_async::i2c::AddressMode,
{
    fn transaction(
        &mut self,
        address: A,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        crate::spin_on(embedded_hal_async::i2c::I2c::transaction(
            &mut self.0,
            address,
            operations,
        ))
    }
}
