//! Adapters for blocking delays.

/// A blocking delay from an async delay.
///
/// The wrapped type `D` is your async delay driver. Your async-aware
/// HAL provides this implementation.
///
/// ```
/// use embedded_spinny::SpinnyDelayNs;
/// use embedded_hal::delay::DelayNs;
/// # use embedded_hal_async::delay::DelayNs as AsyncDelayNs;
/// # struct YourHalDelayNs;
/// # impl AsyncDelayNs for YourHalDelayNs {
/// #   async fn delay_ns(&mut self, _: u32) {}
/// # }
///
/// let your_delay_ns = // Provided by your driver package...
/// #   YourHalDelayNs;
/// let mut spinny_delay = SpinnyDelayNs(your_delay_ns);
///
/// fn needs_delay<D: DelayNs>(delay: &mut D) {
///     delay.delay_ns(1234);
/// }
///
/// needs_delay(&mut spinny_delay);
/// ```
pub struct SpinnyDelayNs<D>(pub D);

impl<D> embedded_hal::delay::DelayNs for SpinnyDelayNs<D>
where
    D: embedded_hal_async::delay::DelayNs,
{
    fn delay_ns(&mut self, ns: u32) {
        crate::spin_on(embedded_hal_async::delay::DelayNs::delay_ns(
            &mut self.0,
            ns,
        ))
    }
}
