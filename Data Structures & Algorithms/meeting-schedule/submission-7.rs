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

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
        for i in 0..intervals.len() {
            for j in i+1.. intervals.len() {
                if intervals[i].start <= intervals[j].start && intervals[j].start < intervals[i].end{
                    return false;
                }
                if intervals[j].start <= intervals[i].start && intervals[i].start < intervals[j].end{
                    return false;
                }
            }
        }

        true
    }
}
