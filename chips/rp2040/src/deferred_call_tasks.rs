//! Definition of Deferred Call tasks.
//!
//! Deferred calls also peripheral drivers to register pseudo interrupts.
//! These are the definitions of which deferred calls this chip needs.

use core::convert::Into;
use core::convert::TryFrom;

/// A type of task to defer a call for
#[derive(Copy, Clone)]
pub enum DeferredCallTask {
    Uart0 = 0,
    Uart1 = 1,
}

impl TryFrom<usize> for DeferredCallTask {
    type Error = ();

    fn try_from(value: usize) -> Result<DeferredCallTask, ()> {
        match value {
            0 => Ok(DeferredCallTask::Uart0),
            1 => Ok(DeferredCallTask::Uart1),
            _ => Err(()),
        }
    }
}

impl Into<usize> for DeferredCallTask {
    fn into(self) -> usize {
        self as usize
    }
}
