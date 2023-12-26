//! Spin on your `embedded-hal-async` futures until they resolve.
//!
//! If you have drivers that implement `embedded-hal-async` traits, then
//! you can use these adapter types to get spinning, blocking I/O for free.
//! Same goes for `embedded-io-async` and `embedded-io`. By designing to
//! common interfaces, `embedded-spinny` provides adapters for multiple
//! hardware abstraction layers (HALs).
//!
//! To get started, select a Rust HAL that provides `embedded-hal-async`
//! driver implementations. Wrap those drivers in their corresponding `Spinny*`
//! adapter, and you'll have a driver that implements the blocking `embedded-hal`
//! traits.
//!
//! The adapters use a basic executor that spins on the future until
//! it produces a value. It's very good at spinning; however, the total cost
//! will depend on how your futures are implemented. If polling your futures is
//! costly, keep in mind that these adapters will poll that costly future
//! as quickly as possible. Additionally, the adapters will never cancel
//! futures; if your future never resolves, you're forever blocked.
//!
//! This package does not include any adapters for `digital` traits. There is
//! no blocking behavior in the `embedded-hal` interface, so there's nothing
//! to adapt.

#![no_std]

mod delay;
mod i2c;
mod io;
mod spi;

pub use delay::SpinnyDelayNs;
pub use i2c::SpinnyI2c;
pub use io::SpinnyIo;
pub use spi::{SpinnySpiBus, SpinnySpiDevice};

use core::future::Future;

/// Spin on the future until it resolves.
fn spin_on<F: Future>(fut: F) -> F::Output {
    let fut = core::pin::pin!(fut);
    cassette::Cassette::new(fut).block_on()
}
