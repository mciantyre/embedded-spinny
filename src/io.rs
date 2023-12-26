//! An adapter for embedded-io readers, writers, seekers, etc.

/// Adapt your async IO driver for blocking IO.
///
/// This conditionally implements the `embedded-io` blocking (and non-blocking)
/// traits. The support depends on what's available for your async IO driver.
///
/// The wrapped type `I` is your `embedded-io-async`-aware driver.
///
/// ```
/// use embedded_spinny::SpinnyIo;
/// use embedded_io::{Read, Write};
/// # use embedded_io_async::{Read as AsyncRead, Write as AsyncWrite, ErrorType};
/// # struct YourIo; impl ErrorType for YourIo { type Error = core::convert::Infallible; }
/// # impl AsyncRead for YourIo {
/// #   async fn read(&mut self, _: &mut [u8]) -> Result<usize, Self::Error> {
/// #       Ok(0)
/// #   }
/// # }
/// # impl AsyncWrite for YourIo {
/// #   async fn write(&mut self, _: &[u8]) -> Result<usize, Self::Error> {
/// #       Ok(0)
/// #   }
/// # }
///
/// let your_io = // Provided by your driver package...
/// #   YourIo;
/// let mut spinny_io = SpinnyIo(your_io);
///
/// fn needs_read<R: Read>(_: &mut R) { /* ... */}
/// fn needs_write<W: Write>(_: &mut W) { /* ... */}
/// fn needs_read_write<I: Read + Write>(_: &mut I) { /* ... */}
///
/// needs_read(&mut spinny_io);
/// needs_write(&mut spinny_io);
/// needs_read_write(&mut spinny_io);
/// ```
pub struct SpinnyIo<I>(pub I);

impl<R> embedded_io_async::ErrorType for SpinnyIo<R>
where
    R: embedded_io_async::ErrorType,
{
    type Error = <R as embedded_io_async::ErrorType>::Error;
}

impl<R> embedded_io::Read for SpinnyIo<R>
where
    R: embedded_io_async::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        crate::spin_on(embedded_io_async::Read::read(&mut self.0, buf))
    }
}

impl<W> embedded_io::Write for SpinnyIo<W>
where
    W: embedded_io_async::Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        crate::spin_on(embedded_io_async::Write::write(&mut self.0, buf))
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        crate::spin_on(embedded_io_async::Write::flush(&mut self.0))
    }
}

impl<S> embedded_io::Seek for SpinnyIo<S>
where
    S: embedded_io_async::Seek,
{
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<u64, Self::Error> {
        crate::spin_on(embedded_io_async::Seek::seek(&mut self.0, pos))
    }
}

impl<B> embedded_io::BufRead for SpinnyIo<B>
where
    B: embedded_io_async::BufRead,
{
    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        crate::spin_on(embedded_io_async::BufRead::fill_buf(&mut self.0))
    }

    fn consume(&mut self, amt: usize) {
        embedded_io_async::BufRead::consume(&mut self.0, amt);
    }
}

impl<R> embedded_io::ReadReady for SpinnyIo<R>
where
    R: embedded_io_async::ReadReady,
{
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        embedded_io_async::ReadReady::read_ready(&mut self.0)
    }
}

impl<W> embedded_io::WriteReady for SpinnyIo<W>
where
    W: embedded_io_async::WriteReady,
{
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        embedded_io_async::WriteReady::write_ready(&mut self.0)
    }
}
