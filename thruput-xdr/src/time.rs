//! Arithmetic on [`TimePoint`], on [`TimeSpan`], the length of time between
//! two timepoints, and on [`BlockHeight`].
//!
//! ```
//! use thruput_xdr::types::{BlockHeight, TimePoint, TimeSpan};
//!
//! let now = TimePoint::UNIX_EPOCH + TimeSpan::from_secs(1_700_000_000);
//! let deadline = now + TimeSpan::from_days(7);
//! assert!(now < deadline);
//! assert_eq!(deadline - now, TimeSpan::from_hours(7 * 24));
//!
//! let height = BlockHeight::new(1_000);
//! let expiry = height + 100;
//! assert_eq!(expiry - height, 100);
//! ```
//!
//! `TimePoint`, `TimeSpan` and `BlockHeight` are `Copy`, so a struct that
//! holds one can derive `Copy`. The operators also accept references.
//!
//! # Overflow
//!
//! Every `+`, `-` and `*` operator here checks for overflow in every build
//! profile and panics when it happens, unlike the integer primitives. A
//! deadline that wrapped into the past would be a security bug. The
//! `checked_` and `saturating_` methods handle the edge without a panic.
//! Use them on values a caller supplies.
//!
//! `later - earlier` on timepoints saturates to [`TimeSpan::ZERO`] when
//! `earlier` is in fact later; [`TimePoint::checked_duration_since`] detects
//! that case. A difference of heights is signed.
//!
//! # Resolution
//!
//! A `TimeSpan` counts in the unit of `TimePoint`, which is the nanosecond
//! today, and encodes as that count in a `u64`. Build spans from
//! [`TimeSpan::from_secs`] and its siblings, and read them with
//! [`TimeSpan::as_secs`], so that code does not depend on the unit.
//! [`TimeSpan::RESOLUTION`] is the shortest span that is not zero.
//!
//! # Constants
//!
//! The `TimeSpan` constructors and [`TimePoint::from_unix_secs`] are
//! `const`. The operators and the other methods are not.
//!
//! ```
//! use thruput_xdr::types::{TimePoint, TimeSpan};
//!
//! const START: TimePoint = TimePoint::from_unix_secs(2_500_000_000);
//! const HOUR: TimeSpan = TimeSpan::from_hours(1);
//! assert!(START < START + HOUR);
//! ```

use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use core::time::Duration;

use crate::types::{BlockHeight, TimePoint, TimeSpan};

// Each is one integer. `types.rs` is generated and derives `Clone` alone, so
// `Copy` is added here: a struct that holds one can then derive `Copy` too,
// and an operand need not be borrowed or cloned.
impl Copy for TimePoint {}
impl Copy for TimeSpan {}
impl Copy for BlockHeight {}

impl Default for TimeSpan {
    fn default() -> TimeSpan {
        TimeSpan::ZERO
    }
}

/// How many of the unit of [`TimePoint`] make a second. The one place the
/// unit is stated: everything else in this module is derived from it.
const TICKS_PER_SEC: u64 = 1_000_000_000;

const NANOS_PER_TICK: u128 = 1_000_000_000 / TICKS_PER_SEC as u128;

/// A [`core::time::Duration`] too long for a [`TimeSpan`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeSpanOverflow;

impl core::fmt::Display for TimeSpanOverflow {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("duration is too long for a time span")
    }
}

impl core::error::Error for TimeSpanOverflow {}

const fn scaled(count: u64, ticks_each: u64) -> TimeSpan {
    match count.checked_mul(ticks_each) {
        Some(ticks) => TimeSpan::new(ticks),
        None => panic!("overflow in TimeSpan constructor"),
    }
}

impl TimeSpan {
    /// No time at all.
    pub const ZERO: TimeSpan = TimeSpan::new(0);

    /// The shortest span that is not zero: one of the unit of [`TimePoint`].
    pub const RESOLUTION: TimeSpan = TimeSpan::new(1);

    /// The longest span. About 584 years at nanosecond resolution.
    pub const MAX: TimeSpan = TimeSpan::new(u64::MAX);

    /// A span of `secs` seconds. Panics past [`TimeSpan::MAX`].
    pub const fn from_secs(secs: u64) -> TimeSpan {
        scaled(secs, TICKS_PER_SEC)
    }

    /// A span of `secs` seconds, or `None` past [`TimeSpan::MAX`]. For a
    /// number of seconds from outside the program.
    pub const fn checked_from_secs(secs: u64) -> Option<TimeSpan> {
        match secs.checked_mul(TICKS_PER_SEC) {
            Some(ticks) => Some(TimeSpan::new(ticks)),
            None => None,
        }
    }

    /// A span of `mins` minutes. Panics past [`TimeSpan::MAX`].
    pub const fn from_mins(mins: u64) -> TimeSpan {
        scaled(mins, 60 * TICKS_PER_SEC)
    }

    /// A span of `hours` hours. Panics past [`TimeSpan::MAX`].
    pub const fn from_hours(hours: u64) -> TimeSpan {
        scaled(hours, 3_600 * TICKS_PER_SEC)
    }

    /// A span of `days` 24-hour days. Panics past [`TimeSpan::MAX`].
    pub const fn from_days(days: u64) -> TimeSpan {
        scaled(days, 86_400 * TICKS_PER_SEC)
    }

    /// The span as long as `duration`, rounded down to
    /// [`TimeSpan::RESOLUTION`], or `None` past [`TimeSpan::MAX`].
    ///
    /// The `const` form of `TimeSpan::try_from(duration)`. Use it for a
    /// span shorter than a second: `from_duration(Duration::from_millis(250))`.
    pub const fn from_duration(duration: Duration) -> Option<TimeSpan> {
        let ticks = duration.as_nanos() / NANOS_PER_TICK;
        if ticks > u64::MAX as u128 {
            None
        } else {
            Some(TimeSpan::new(ticks as u64))
        }
    }

    /// The same length of time as a [`core::time::Duration`].
    pub fn as_duration(self) -> Duration {
        Duration::new(
            *self / TICKS_PER_SEC,
            ((*self % TICKS_PER_SEC) as u128 * NANOS_PER_TICK) as u32,
        )
    }

    /// The whole seconds in the span.
    pub fn as_secs(self) -> u64 {
        *self / TICKS_PER_SEC
    }

    /// The span in nanoseconds. Never overflows, whatever the resolution.
    ///
    /// For arithmetic that leaves the time types, such as a rate multiplied
    /// by a time in a wider integer.
    pub fn as_nanos(self) -> u128 {
        *self as u128 * NANOS_PER_TICK
    }

    /// Whether the span is [`TimeSpan::ZERO`].
    pub fn is_zero(self) -> bool {
        *self == 0
    }

    /// `self + other`, or `None` past [`TimeSpan::MAX`].
    pub fn checked_add(self, other: TimeSpan) -> Option<TimeSpan> {
        (*self).checked_add(*other).map(TimeSpan::new)
    }

    /// `self - other`, or `None` if `other` is the longer.
    pub fn checked_sub(self, other: TimeSpan) -> Option<TimeSpan> {
        (*self).checked_sub(*other).map(TimeSpan::new)
    }

    /// `self * count`, or `None` past [`TimeSpan::MAX`].
    pub fn checked_mul(self, count: u64) -> Option<TimeSpan> {
        (*self).checked_mul(count).map(TimeSpan::new)
    }

    /// How many whole `period`s fit in the span, and what is left over.
    /// `None` if `period` is zero.
    ///
    /// See [`TimePoint::period_since`] to count periods from an origin.
    pub fn checked_div_rem(self, period: TimeSpan) -> Option<(u64, TimeSpan)> {
        if *period == 0 {
            return None;
        }
        Some((*self / *period, TimeSpan::new(*self % *period)))
    }
}

impl From<TimeSpan> for Duration {
    fn from(span: TimeSpan) -> Duration {
        span.as_duration()
    }
}

impl TryFrom<Duration> for TimeSpan {
    type Error = TimeSpanOverflow;

    fn try_from(duration: Duration) -> Result<TimeSpan, TimeSpanOverflow> {
        TimeSpan::from_duration(duration).ok_or(TimeSpanOverflow)
    }
}

/// Where a timepoint falls on a clock of equal periods. See
/// [`TimePoint::period_since`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Period {
    /// How many whole periods have passed: 0 through the first period.
    pub index: u64,

    /// How far into period `index` the timepoint is: less than the period.
    pub elapsed: TimeSpan,
}

impl TimePoint {
    /// The Unix epoch, 1970-01-01T00:00:00Z: the timepoint every other one
    /// is counted from.
    ///
    /// `TimePoint::UNIX_EPOCH + TimeSpan::from_secs(secs)` builds a timepoint
    /// from a Unix seconds value, and
    /// `t.duration_since(&TimePoint::UNIX_EPOCH)` reads one back.
    pub const UNIX_EPOCH: TimePoint = TimePoint::new(0);

    /// The timepoint `secs` seconds after the Unix epoch. Panics past the
    /// end of the representable range.
    ///
    /// A `const fn`, for a constant timepoint.
    pub const fn from_unix_secs(secs: u64) -> TimePoint {
        match secs.checked_mul(TICKS_PER_SEC) {
            Some(ticks) => TimePoint::new(ticks),
            None => panic!("overflow in TimePoint::from_unix_secs"),
        }
    }

    /// The timepoint `secs` seconds after the Unix epoch, or `None` past
    /// the end of the representable range. For a Unix timestamp from
    /// outside the program.
    pub const fn checked_from_unix_secs(secs: u64) -> Option<TimePoint> {
        match secs.checked_mul(TICKS_PER_SEC) {
            Some(ticks) => Some(TimePoint::new(ticks)),
            None => None,
        }
    }

    /// `self + span`, or `None` if that is past the end of the
    /// representable range.
    pub fn checked_add(&self, span: TimeSpan) -> Option<TimePoint> {
        (**self).checked_add(*span).map(TimePoint::new)
    }

    /// `self - span`, or `None` if that is before the Unix epoch.
    pub fn checked_sub(&self, span: TimeSpan) -> Option<TimePoint> {
        (**self).checked_sub(*span).map(TimePoint::new)
    }

    /// `self + span`, or the latest representable timepoint if that
    /// overflows.
    ///
    /// The natural form for a deadline that a caller-supplied span may push
    /// past the end of the range: the result then simply reads as "never".
    pub fn saturating_add(&self, span: TimeSpan) -> TimePoint {
        TimePoint::new((**self).saturating_add(*span))
    }

    /// `self - span`, or [`TimePoint::UNIX_EPOCH`] if that would precede it.
    pub fn saturating_sub(&self, span: TimeSpan) -> TimePoint {
        TimePoint::new((**self).saturating_sub(*span))
    }

    /// The time from `earlier` to `self`, or [`TimeSpan::ZERO`] if `earlier`
    /// is in fact later.
    ///
    /// The same as `self - earlier`.
    pub fn duration_since(&self, earlier: &TimePoint) -> TimeSpan {
        TimeSpan::new((**self).saturating_sub(**earlier))
    }

    /// The time from `earlier` to `self`, or `None` if `earlier` is in fact
    /// later.
    pub fn checked_duration_since(
        &self,
        earlier: &TimePoint,
    ) -> Option<TimeSpan> {
        (**self).checked_sub(**earlier).map(TimeSpan::new)
    }

    /// The period `self` falls in, on a clock of equal `period`s that starts
    /// at `origin`.
    ///
    /// Period `n` runs from `origin + n * period` up to, and not including,
    /// `origin + (n + 1) * period`. Returns `None` if `self` is before
    /// `origin`, or `period` is zero.
    ///
    /// ```
    /// use thruput_xdr::time::Period;
    /// use thruput_xdr::types::{TimePoint, TimeSpan};
    ///
    /// let origin = TimePoint::UNIX_EPOCH + TimeSpan::from_days(30_000);
    /// let now = origin + TimeSpan::from_secs(150);
    /// assert_eq!(
    ///     now.period_since(&origin, TimeSpan::from_mins(1)),
    ///     Some(Period { index: 2, elapsed: TimeSpan::from_secs(30) })
    /// );
    /// ```
    pub fn period_since(
        &self,
        origin: &TimePoint,
        period: TimeSpan,
    ) -> Option<Period> {
        let (index, elapsed) = self
            .checked_duration_since(origin)?
            .checked_div_rem(period)?;
        Some(Period { index, elapsed })
    }
}

// The operators. Each by-value impl holds the logic; the forms that take a
// reference forward to it.

impl Add<TimeSpan> for TimePoint {
    type Output = TimePoint;

    fn add(self, rhs: TimeSpan) -> TimePoint {
        self.checked_add(rhs)
            .expect("overflow when adding time span to timepoint")
    }
}

impl Add<TimeSpan> for &TimePoint {
    type Output = TimePoint;

    fn add(self, rhs: TimeSpan) -> TimePoint {
        *self + rhs
    }
}

impl AddAssign<TimeSpan> for TimePoint {
    fn add_assign(&mut self, rhs: TimeSpan) {
        *self = *self + rhs;
    }
}

impl Sub<TimeSpan> for TimePoint {
    type Output = TimePoint;

    fn sub(self, rhs: TimeSpan) -> TimePoint {
        self.checked_sub(rhs)
            .expect("overflow when subtracting time span from timepoint")
    }
}

impl Sub<TimeSpan> for &TimePoint {
    type Output = TimePoint;

    fn sub(self, rhs: TimeSpan) -> TimePoint {
        *self - rhs
    }
}

impl SubAssign<TimeSpan> for TimePoint {
    fn sub_assign(&mut self, rhs: TimeSpan) {
        *self = *self - rhs;
    }
}

impl Sub<TimePoint> for TimePoint {
    type Output = TimeSpan;

    fn sub(self, rhs: TimePoint) -> TimeSpan {
        self.duration_since(&rhs)
    }
}

impl Sub<&TimePoint> for TimePoint {
    type Output = TimeSpan;

    fn sub(self, rhs: &TimePoint) -> TimeSpan {
        self.duration_since(rhs)
    }
}

impl Sub<TimePoint> for &TimePoint {
    type Output = TimeSpan;

    fn sub(self, rhs: TimePoint) -> TimeSpan {
        self.duration_since(&rhs)
    }
}

impl Sub<&TimePoint> for &TimePoint {
    type Output = TimeSpan;

    fn sub(self, rhs: &TimePoint) -> TimeSpan {
        self.duration_since(rhs)
    }
}

impl Add for TimeSpan {
    type Output = TimeSpan;

    fn add(self, rhs: TimeSpan) -> TimeSpan {
        self.checked_add(rhs)
            .expect("overflow when adding time spans")
    }
}

impl AddAssign for TimeSpan {
    fn add_assign(&mut self, rhs: TimeSpan) {
        *self = *self + rhs;
    }
}

impl Sub for TimeSpan {
    type Output = TimeSpan;

    fn sub(self, rhs: TimeSpan) -> TimeSpan {
        self.checked_sub(rhs)
            .expect("overflow when subtracting time spans")
    }
}

impl SubAssign for TimeSpan {
    fn sub_assign(&mut self, rhs: TimeSpan) {
        *self = *self - rhs;
    }
}

impl Mul<u64> for TimeSpan {
    type Output = TimeSpan;

    fn mul(self, rhs: u64) -> TimeSpan {
        self.checked_mul(rhs)
            .expect("overflow when multiplying time span")
    }
}

impl Mul<TimeSpan> for u64 {
    type Output = TimeSpan;

    fn mul(self, rhs: TimeSpan) -> TimeSpan {
        rhs * self
    }
}

impl Div<u64> for TimeSpan {
    type Output = TimeSpan;

    /// Panics if `rhs` is zero.
    fn div(self, rhs: u64) -> TimeSpan {
        TimeSpan::new(*self / rhs)
    }
}

impl BlockHeight {
    /// `self + blocks`, or `None` on overflow.
    pub fn checked_add(&self, blocks: i64) -> Option<BlockHeight> {
        (**self).checked_add(blocks).map(BlockHeight::new)
    }

    /// `self - blocks`, or `None` on overflow.
    pub fn checked_sub(&self, blocks: i64) -> Option<BlockHeight> {
        (**self).checked_sub(blocks).map(BlockHeight::new)
    }

    /// The blocks from `earlier` to `self`: negative if `earlier` is in fact
    /// later.
    ///
    /// The same as `self - earlier`. Panics if the difference overflows
    /// `i64`; see [`checked_blocks_since`](Self::checked_blocks_since).
    pub fn blocks_since(&self, earlier: &BlockHeight) -> i64 {
        match self.checked_blocks_since(earlier) {
            Some(blocks) => blocks,
            None => panic!("overflow when subtracting block heights"),
        }
    }

    /// The blocks from `earlier` to `self`, or `None` if the difference
    /// overflows `i64`.
    pub fn checked_blocks_since(&self, earlier: &BlockHeight) -> Option<i64> {
        (**self).checked_sub(**earlier)
    }
}

impl Add<i64> for &BlockHeight {
    type Output = BlockHeight;

    fn add(self, rhs: i64) -> BlockHeight {
        self.checked_add(rhs)
            .expect("overflow when adding blocks to height")
    }
}

impl Add<i64> for BlockHeight {
    type Output = BlockHeight;

    fn add(self, rhs: i64) -> BlockHeight {
        <&BlockHeight as Add<i64>>::add(&self, rhs)
    }
}

impl AddAssign<i64> for BlockHeight {
    fn add_assign(&mut self, rhs: i64) {
        *self = <&BlockHeight as Add<i64>>::add(self, rhs);
    }
}

impl Sub<i64> for &BlockHeight {
    type Output = BlockHeight;

    fn sub(self, rhs: i64) -> BlockHeight {
        self.checked_sub(rhs)
            .expect("overflow when subtracting blocks from height")
    }
}

impl Sub<i64> for BlockHeight {
    type Output = BlockHeight;

    fn sub(self, rhs: i64) -> BlockHeight {
        <&BlockHeight as Sub<i64>>::sub(&self, rhs)
    }
}

impl SubAssign<i64> for BlockHeight {
    fn sub_assign(&mut self, rhs: i64) {
        *self = <&BlockHeight as Sub<i64>>::sub(self, rhs);
    }
}

impl Sub<&BlockHeight> for &BlockHeight {
    type Output = i64;

    fn sub(self, rhs: &BlockHeight) -> i64 {
        self.blocks_since(rhs)
    }
}

impl Sub<BlockHeight> for &BlockHeight {
    type Output = i64;

    fn sub(self, rhs: BlockHeight) -> i64 {
        self.blocks_since(&rhs)
    }
}

impl Sub<&BlockHeight> for BlockHeight {
    type Output = i64;

    fn sub(self, rhs: &BlockHeight) -> i64 {
        self.blocks_since(rhs)
    }
}

impl Sub<BlockHeight> for BlockHeight {
    type Output = i64;

    fn sub(self, rhs: BlockHeight) -> i64 {
        self.blocks_since(&rhs)
    }
}

#[cfg(test)]
// The operators on references are part of what is tested.
#[allow(clippy::op_ref)]
mod tests {
    use super::*;

    const DAY: TimeSpan = TimeSpan::from_days(1);
    const TICK: TimeSpan = TimeSpan::RESOLUTION;

    fn timepoint(ticks: u64) -> TimePoint {
        TimePoint::new(ticks)
    }

    fn height(blocks: i64) -> BlockHeight {
        BlockHeight::new(blocks)
    }

    #[test]
    fn timepoints_spans_and_heights_are_copy() {
        #[derive(Clone, Copy)]
        struct Terms {
            end: TimePoint,
            period: TimeSpan,
            expiry: BlockHeight,
        }

        let terms = Terms {
            end: timepoint(1_000),
            period: DAY,
            expiry: height(10),
        };
        let copy = terms;
        // Operands by value, and still usable afterwards.
        assert_eq!(terms.end + terms.period - copy.period, copy.end);
        assert_eq!(terms.end - copy.end, TimeSpan::ZERO);
        assert_eq!(terms.expiry + 1 - copy.expiry, 1);
        assert_eq!(terms.expiry, copy.expiry);
    }

    #[test]
    fn spans_build_from_units() {
        assert_eq!(TimeSpan::from_secs(60), TimeSpan::from_mins(1));
        assert_eq!(TimeSpan::from_mins(60), TimeSpan::from_hours(1));
        assert_eq!(TimeSpan::from_hours(24), DAY);
        assert_eq!(DAY.as_secs(), 86_400);
        assert_eq!(DAY.as_nanos(), 86_400_000_000_000);
        assert_eq!(TimeSpan::default(), TimeSpan::ZERO);
        assert!(TimeSpan::ZERO.is_zero());
        assert!(!TICK.is_zero());
        assert!(TimeSpan::ZERO < TICK && TICK <= TimeSpan::from_secs(1));
        assert!(DAY < TimeSpan::MAX);
        // One second is a whole number of the unit.
        assert_eq!(
            TimeSpan::from_secs(1).checked_div_rem(TICK),
            Some((TICKS_PER_SEC, TimeSpan::ZERO))
        );
    }

    #[test]
    #[should_panic(expected = "overflow in TimeSpan constructor")]
    fn a_span_past_the_range_panics() {
        let _ = TimeSpan::from_days(u64::MAX);
    }

    #[test]
    fn spans_convert_to_and_from_durations() {
        assert_eq!(Duration::from(DAY), Duration::from_secs(86_400));
        assert_eq!(TimeSpan::try_from(Duration::from_secs(86_400)), Ok(DAY));
        assert_eq!(
            TimeSpan::from_duration(Duration::from_millis(1_500))
                .unwrap()
                .as_duration(),
            Duration::from_millis(1_500)
        );
        assert_eq!(
            TimeSpan::try_from(TimeSpan::MAX.as_duration()),
            Ok(TimeSpan::MAX)
        );
        assert_eq!(
            TimeSpan::try_from(Duration::from(TICK)),
            Ok(TimeSpan::RESOLUTION)
        );
        assert_eq!(
            TimeSpan::try_from(
                Duration::from(TimeSpan::MAX) + Duration::from(TICK)
            ),
            Err(TimeSpanOverflow)
        );
        assert_eq!(TimeSpan::from_duration(Duration::MAX), None);
    }

    #[test]
    fn spans_add_scale_and_divide() {
        let minute = TimeSpan::from_mins(1);
        let mut span = minute + TimeSpan::from_secs(30);
        assert_eq!(span, TimeSpan::from_secs(90));
        span -= TimeSpan::from_secs(30);
        span += minute;
        assert_eq!(span, 2 * minute);
        assert_eq!(minute * 2, 2 * minute);
        assert_eq!(minute / 4, TimeSpan::from_secs(15));
        assert_eq!(
            TimeSpan::from_secs(150).checked_div_rem(minute),
            Some((2, TimeSpan::from_secs(30)))
        );
        assert_eq!(minute.checked_div_rem(TimeSpan::ZERO), None);

        assert_eq!(TimeSpan::MAX.checked_add(TICK), None);
        assert_eq!(TimeSpan::ZERO.checked_sub(TICK), None);
        assert_eq!(TimeSpan::MAX.checked_mul(2), None);
        assert_eq!(minute.checked_mul(60), Some(TimeSpan::from_hours(1)));
    }

    #[test]
    #[should_panic(expected = "overflow when adding time spans")]
    fn a_sum_of_spans_past_the_range_panics() {
        let _ = TimeSpan::MAX + TICK;
    }

    #[test]
    #[should_panic(expected = "overflow when subtracting time spans")]
    fn a_negative_span_panics() {
        let _ = TimeSpan::ZERO - TICK;
    }

    #[test]
    #[should_panic(expected = "overflow when multiplying time span")]
    fn a_scaled_span_past_the_range_panics() {
        let _ = TimeSpan::MAX * 2;
    }

    #[test]
    fn timepoint_plus_and_minus_span() {
        let now = timepoint(1_000);
        let deadline = now + 7 * TICK;
        assert_eq!(deadline, timepoint(1_007));
        assert!(now < deadline);
        assert_eq!(deadline - 7 * TICK, now);
        assert_eq!(&deadline - 7 * TICK, now);
        assert_eq!(&now + 7 * TICK, deadline);

        let mut moving = now;
        moving += 10 * TICK;
        moving -= 3 * TICK;
        assert_eq!(moving, timepoint(1_007));

        assert_eq!(
            TimePoint::UNIX_EPOCH + TimeSpan::from_secs(5),
            timepoint(5 * TICKS_PER_SEC)
        );
        assert_eq!(TimePoint::from_unix_secs(5), timepoint(5 * TICKS_PER_SEC));
        assert_eq!(
            TimePoint::checked_from_unix_secs(5),
            Some(timepoint(5 * TICKS_PER_SEC))
        );
        assert_eq!(TimePoint::checked_from_unix_secs(u64::MAX), None);
        assert_eq!(TimeSpan::checked_from_secs(u64::MAX), None);
        assert_eq!(
            TimeSpan::checked_from_secs(5),
            Some(TimeSpan::from_secs(5))
        );
    }

    #[test]
    fn timepoint_difference_saturates_at_zero() {
        let start = timepoint(100);
        let end = timepoint(250);
        assert_eq!(end - start, 150 * TICK);
        assert_eq!(&end - &start, 150 * TICK);
        assert_eq!(end - &start, 150 * TICK);
        assert_eq!(&end - start, 150 * TICK);
        assert_eq!(end.duration_since(&start), 150 * TICK);
        assert_eq!(end.checked_duration_since(&start), Some(150 * TICK));

        assert_eq!(start - end, TimeSpan::ZERO);
        assert_eq!(start.duration_since(&end), TimeSpan::ZERO);
        assert_eq!(start.checked_duration_since(&end), None);
    }

    #[test]
    fn timepoint_checked_and_saturating_forms() {
        let late = timepoint(u64::MAX - 1);
        assert_eq!(late.checked_add(TICK), Some(timepoint(u64::MAX)));
        assert_eq!(late.checked_add(2 * TICK), None);
        assert_eq!(late.saturating_add(DAY), timepoint(u64::MAX));
        assert_eq!(
            timepoint(0).saturating_add(TimeSpan::MAX),
            timepoint(u64::MAX)
        );

        let early = timepoint(1);
        assert_eq!(early.checked_sub(TICK), Some(TimePoint::UNIX_EPOCH));
        assert_eq!(early.checked_sub(2 * TICK), None);
        assert_eq!(early.saturating_sub(DAY), TimePoint::UNIX_EPOCH);
    }

    #[test]
    #[should_panic(expected = "overflow when adding time span to timepoint")]
    fn timepoint_add_overflow_panics() {
        let _ = timepoint(u64::MAX) + TICK;
    }

    #[test]
    #[should_panic(
        expected = "overflow when subtracting time span from timepoint"
    )]
    fn timepoint_sub_before_epoch_panics() {
        let _ = TimePoint::UNIX_EPOCH - TICK;
    }

    #[test]
    fn period_since_splits_time_from_an_origin() {
        let origin = TimePoint::from_unix_secs(2_500_000_000);
        let minute = TimeSpan::from_mins(1);
        assert_eq!(
            origin.period_since(&origin, minute),
            Some(Period {
                index: 0,
                elapsed: TimeSpan::ZERO
            })
        );
        assert_eq!(
            (origin + TimeSpan::from_secs(150)).period_since(&origin, minute),
            Some(Period {
                index: 2,
                elapsed: TimeSpan::from_secs(30)
            })
        );
        // Before the origin, and with no period to count in.
        assert_eq!((origin - TICK).period_since(&origin, minute), None);
        assert_eq!(origin.period_since(&origin, TimeSpan::ZERO), None);
        // The whole range, in the shortest period.
        assert_eq!(
            timepoint(u64::MAX).period_since(&TimePoint::UNIX_EPOCH, TICK),
            Some(Period {
                index: u64::MAX,
                elapsed: TimeSpan::ZERO
            })
        );
    }

    #[cfg(feature = "xdrs")]
    #[test]
    fn a_span_encodes_as_a_timepoint_does() {
        let span = TimeSpan::from_secs(90);
        let bytes = xdrs::cxs::encode(&span).unwrap();
        assert_eq!(
            bytes,
            xdrs::cxs::encode(&(TimePoint::UNIX_EPOCH + span)).unwrap()
        );
        assert_eq!(xdrs::cxs::decode::<TimeSpan>(&bytes).unwrap(), span);
    }

    #[test]
    fn height_plus_and_minus_blocks() {
        let now = height(1_000);
        let expiry = &now + 100;
        assert_eq!(expiry, height(1_100));
        assert!(now < expiry);
        assert_eq!(&expiry - 100, now);
        assert_eq!(expiry - 100, now);

        let mut moving = now;
        moving += 10;
        moving -= 3;
        assert_eq!(moving, height(1_007));
    }

    #[test]
    fn height_difference_is_signed() {
        let earlier = height(100);
        let later = height(250);
        assert_eq!(&later - &earlier, 150);
        assert_eq!(later - &earlier, 150);
        assert_eq!(&later - earlier, 150);
        assert_eq!(later - earlier, 150);
        assert_eq!(later.blocks_since(&earlier), 150);
        assert_eq!(&earlier - &later, -150);
        assert_eq!(earlier.checked_blocks_since(&later), Some(-150));
        assert_eq!(height(i64::MAX).checked_blocks_since(&height(-1)), None);
    }

    #[test]
    fn height_checked_forms() {
        let top = height(i64::MAX);
        assert_eq!(top.checked_add(1), None);
        assert_eq!(top.checked_sub(1), Some(height(i64::MAX - 1)));
        assert_eq!(height(i64::MIN).checked_sub(1), None);
    }

    #[test]
    #[should_panic(expected = "overflow when adding blocks to height")]
    fn height_add_overflow_panics() {
        let _ = height(i64::MAX) + 1;
    }

    #[test]
    #[should_panic(expected = "overflow when subtracting block heights")]
    fn height_difference_overflow_panics() {
        let _ = height(i64::MAX) - height(-1);
    }
}
