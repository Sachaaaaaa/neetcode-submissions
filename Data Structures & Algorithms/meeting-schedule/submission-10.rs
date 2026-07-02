/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */


//  if meetings are sorted by their start time, then
//    we only need to check adjacent meetings for overlap
impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| v.start);

        for i in 1..intervals.len() {
            if intervals[i - 1].end > intervals[i].start {
                return false;
            }
        }
        true
    }
}
