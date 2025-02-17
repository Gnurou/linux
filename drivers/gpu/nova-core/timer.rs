// SPDX-License-Identifier: GPL-2.0

//! Nova Core Timer subdevice

use core::fmt::Display;
use core::ops::{Add, Sub};
use core::time::Duration;

use kernel::devres::Devres;
use kernel::num::U64Ext;
use kernel::prelude::*;

use crate::driver::Bar0;
use crate::regs;

/// A timestamp with nanosecond granularity obtained from the GPU timer.
///
/// A timestamp can also be substracted to another in order to obtain a [`Duration`].
///
/// TODO: add Kunit tests!
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Timestamp(u64);

impl Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(mut self, rhs: Duration) -> Self::Output {
        let mut nanos = rhs.as_nanos();
        while nanos > u64::MAX as u128 {
            self.0 = self.0.wrapping_add(nanos as u64);
            nanos -= u64::MAX as u128;
        }

        Timestamp(self.0.wrapping_add(nanos as u64))
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration::from_nanos(self.0.wrapping_sub(rhs.0))
    }
}

pub(crate) struct Timer {}

impl Timer {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Read the current timer timestamp.
    pub(crate) fn read(&self, bar: &Bar0) -> Timestamp {
        loop {
            let hi = regs::PtimerTime1::read(bar);
            let lo = regs::PtimerTime0::read(bar);

            if hi.hi() == regs::PtimerTime1::read(bar).hi() {
                return Timestamp(u64::from_u32s(hi.hi(), lo.lo()));
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn time(bar: &Bar0, time: u64) {
        regs::PtimerTime1::default()
            .set_hi(time.upper_32_bits())
            .write(bar);
        regs::PtimerTime0::default()
            .set_lo(time.lower_32_bits())
            .write(bar);
    }

    /// Wait until `cond` is true or `timeout` elapsed, based on GPU time.
    ///
    /// When `cond` evaluates to `Some`, its return value is returned.
    ///
    /// `Err(ETIMEDOUT)` is returned if `timeout` has been reached without `cond` evaluating to
    /// `Some`, or if the timer device is stuck for some reason.
    pub(crate) fn wait_on<R, F: Fn() -> Option<R>>(
        &self,
        bar: &Devres<Bar0>,
        timeout: Duration,
        cond: F,
    ) -> Result<R> {
        // Number of consecutive time reads after which we consider the timer frozen if it hasn't
        // moved forward.
        const MAX_STALLED_READS: usize = 16;

        let (mut cur_time, mut prev_time, deadline) = {
            let cur_time = with_bar!(bar, |b| self.read(b))?;
            let deadline = cur_time + timeout;

            (cur_time, cur_time, deadline)
        };
        let mut num_reads = 0;

        loop {
            if let Some(ret) = cond() {
                return Ok(ret);
            }

            (|| {
                cur_time = with_bar!(bar, |b| self.read(b))?;

                /* Check if the timer is frozen for some reason. */
                if cur_time == prev_time {
                    if num_reads >= MAX_STALLED_READS {
                        return Err(ETIMEDOUT);
                    }
                    num_reads += 1;
                } else {
                    if cur_time >= deadline {
                        return Err(ETIMEDOUT);
                    }

                    num_reads = 0;
                    prev_time = cur_time;
                }

                Ok(())
            })()?;
        }
    }
}
