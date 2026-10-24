//! Arithmetic on `TimePoint` and `BlockHeight`. The rules are documented,
//! with an example, in `docs/types.md`, which renders as the `types` module
//! documentation; this module is private.

use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration;

use crate::types::{BlockHeight, TimePoint};

/// The nanoseconds in `duration`, or `None` if they do not fit `u64` (about
/// 584 years).
const fn duration_nanos(duration: Duration) -> Option<u64> {
    let nanos = duration.as_nanos();
    if nanos > u64::MAX as u128 {
        None
    } else {
        Some(nanos as u64)
    }
}

impl TimePoint {
    /// The Unix epoch, 1970-01-01T00:00:00Z: the timepoint every other one
    /// counts nanoseconds from.
    ///
    /// `TimePoint::UNIX_EPOCH + Duration::from_secs(secs)` builds a timepoint
    /// from a Unix seconds value, and
    /// `t.duration_since(&TimePoint::UNIX_EPOCH)` reads one back.
    pub const UNIX_EPOCH: TimePoint = TimePoint::new(0);

    /// `self + duration`, or `None` if that is past the end of the
    /// representable range.
    pub fn checked_add(&self, duration: Duration) -> Option<TimePoint> {
        duration_nanos(duration)
            .and_then(|nanos| (**self).checked_add(nanos))
            .map(TimePoint::new)
    }

    /// `self - duration`, or `None` if that is before the Unix epoch.
    pub fn checked_sub(&self, duration: Duration) -> Option<TimePoint> {
        duration_nanos(duration)
            .and_then(|nanos| (**self).checked_sub(nanos))
            .map(TimePoint::new)
    }

    /// `self + duration`, or the latest representable timepoint if that
    /// overflows.
    ///
    /// The natural form for a deadline that a caller-supplied span may push
    /// past the end of the range: the result then simply reads as "never".
    pub fn saturating_add(&self, duration: Duration) -> TimePoint {
        match self.checked_add(duration) {
            Some(sum) => sum,
            None => TimePoint::new(u64::MAX),
        }
    }

    /// `self - duration`, or [`TimePoint::UNIX_EPOCH`] if that would
    /// precede it.
    pub fn saturating_sub(&self, duration: Duration) -> TimePoint {
        match self.checked_sub(duration) {
            Some(difference) => difference,
            None => TimePoint::UNIX_EPOCH,
        }
    }

    /// The time from `earlier` to `self`, or `Duration::ZERO` if `earlier`
    /// is in fact later.
    ///
    /// The same as `self - earlier`.
    pub fn duration_since(&self, earlier: &TimePoint) -> Duration {
        Duration::from_nanos((**self).saturating_sub(**earlier))
    }

    /// The time from `earlier` to `self`, or `None` if `earlier` is in fact
    /// later.
    pub fn checked_duration_since(
        &self,
        earlier: &TimePoint,
    ) -> Option<Duration> {
        (**self).checked_sub(**earlier).map(Duration::from_nanos)
    }
}

impl Add<Duration> for &TimePoint {
    type Output = TimePoint;

    fn add(self, rhs: Duration) -> TimePoint {
        self.checked_add(rhs)
            .expect("overflow when adding duration to timepoint")
    }
}

impl Add<Duration> for TimePoint {
    type Output = TimePoint;

    fn add(self, rhs: Duration) -> TimePoint {
        &self + rhs
    }
}

impl AddAssign<Duration> for TimePoint {
    fn add_assign(&mut self, rhs: Duration) {
        *self = &*self + rhs;
    }
}

impl Sub<Duration> for &TimePoint {
    type Output = TimePoint;

    fn sub(self, rhs: Duration) -> TimePoint {
        self.checked_sub(rhs)
            .expect("overflow when subtracting duration from timepoint")
    }
}

impl Sub<Duration> for TimePoint {
    type Output = TimePoint;

    fn sub(self, rhs: Duration) -> TimePoint {
        &self - rhs
    }
}

impl SubAssign<Duration> for TimePoint {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = &*self - rhs;
    }
}

impl Sub<&TimePoint> for &TimePoint {
    type Output = Duration;

    fn sub(self, rhs: &TimePoint) -> Duration {
        self.duration_since(rhs)
    }
}

impl Sub<TimePoint> for &TimePoint {
    type Output = Duration;

    fn sub(self, rhs: TimePoint) -> Duration {
        self.duration_since(&rhs)
    }
}

impl Sub<&TimePoint> for TimePoint {
    type Output = Duration;

    fn sub(self, rhs: &TimePoint) -> Duration {
        self.duration_since(rhs)
    }
}

impl Sub<TimePoint> for TimePoint {
    type Output = Duration;

    fn sub(self, rhs: TimePoint) -> Duration {
        self.duration_since(&rhs)
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
        &self + rhs
    }
}

impl AddAssign<i64> for BlockHeight {
    fn add_assign(&mut self, rhs: i64) {
        *self = &*self + rhs;
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
        &self - rhs
    }
}

impl SubAssign<i64> for BlockHeight {
    fn sub_assign(&mut self, rhs: i64) {
        *self = &*self - rhs;
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
mod tests {
    use super::*;

    const DAY: Duration = Duration::from_hours(24);

    fn timepoint(nanos: u64) -> TimePoint {
        TimePoint::new(nanos)
    }

    fn height(blocks: i64) -> BlockHeight {
        BlockHeight::new(blocks)
    }

    #[test]
    fn timepoint_plus_and_minus_duration() {
        let now = timepoint(1_000);
        let deadline = &now + Duration::from_nanos(7);
        assert_eq!(deadline, timepoint(1_007));
        assert!(now < deadline);
        assert_eq!(&deadline - Duration::from_nanos(7), now);
        assert_eq!(deadline.clone() - Duration::from_nanos(7), now);

        let mut moving = now.clone();
        moving += Duration::from_nanos(10);
        moving -= Duration::from_nanos(3);
        assert_eq!(moving, timepoint(1_007));

        assert_eq!(
            TimePoint::UNIX_EPOCH + Duration::from_secs(5),
            timepoint(5_000_000_000)
        );
        // Sub-nanosecond precision is not a thing: whole nanoseconds only,
        // and `core` cannot express less than one either.
        assert_eq!(
            &now + 7 * DAY,
            timepoint(1_000 + 7 * 86_400 * 1_000_000_000)
        );
    }

    #[test]
    fn timepoint_difference_saturates_at_zero() {
        let start = timepoint(100);
        let end = timepoint(250);
        assert_eq!(&end - &start, Duration::from_nanos(150));
        assert_eq!(end.clone() - &start, Duration::from_nanos(150));
        assert_eq!(&end - start.clone(), Duration::from_nanos(150));
        assert_eq!(end.clone() - start.clone(), Duration::from_nanos(150));
        assert_eq!(end.duration_since(&start), Duration::from_nanos(150));
        assert_eq!(
            end.checked_duration_since(&start),
            Some(Duration::from_nanos(150))
        );

        assert_eq!(&start - &end, Duration::ZERO);
        assert_eq!(start.duration_since(&end), Duration::ZERO);
        assert_eq!(start.checked_duration_since(&end), None);
    }

    #[test]
    fn timepoint_checked_and_saturating_forms() {
        let late = timepoint(u64::MAX - 1);
        assert_eq!(
            late.checked_add(Duration::from_nanos(1)),
            Some(timepoint(u64::MAX))
        );
        assert_eq!(late.checked_add(Duration::from_nanos(2)), None);
        assert_eq!(late.saturating_add(DAY), timepoint(u64::MAX));
        // A duration too long for u64 nanoseconds at all.
        assert_eq!(timepoint(0).checked_add(Duration::MAX), None);
        assert_eq!(
            timepoint(0).saturating_add(Duration::MAX),
            timepoint(u64::MAX)
        );

        let early = timepoint(1);
        assert_eq!(
            early.checked_sub(Duration::from_nanos(1)),
            Some(TimePoint::UNIX_EPOCH)
        );
        assert_eq!(early.checked_sub(Duration::from_nanos(2)), None);
        assert_eq!(early.saturating_sub(DAY), TimePoint::UNIX_EPOCH);
        assert_eq!(early.checked_sub(Duration::MAX), None);
    }

    #[test]
    #[should_panic(expected = "overflow when adding duration to timepoint")]
    fn timepoint_add_overflow_panics() {
        let _ = timepoint(u64::MAX) + Duration::from_nanos(1);
    }

    #[test]
    #[should_panic(
        expected = "overflow when subtracting duration from timepoint"
    )]
    fn timepoint_sub_before_epoch_panics() {
        let _ = TimePoint::UNIX_EPOCH - Duration::from_nanos(1);
    }

    #[test]
    fn height_plus_and_minus_blocks() {
        let now = height(1_000);
        let expiry = &now + 100;
        assert_eq!(expiry, height(1_100));
        assert!(now < expiry);
        assert_eq!(&expiry - 100, now);
        assert_eq!(expiry.clone() - 100, now);

        let mut moving = now.clone();
        moving += 10;
        moving -= 3;
        assert_eq!(moving, height(1_007));
    }

    #[test]
    fn height_difference_is_signed() {
        let earlier = height(100);
        let later = height(250);
        assert_eq!(&later - &earlier, 150);
        assert_eq!(later.clone() - &earlier, 150);
        assert_eq!(&later - earlier.clone(), 150);
        assert_eq!(later.clone() - earlier.clone(), 150);
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
