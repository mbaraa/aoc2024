use std::{collections::HashSet, fs};

use crate::solve::Solution;

pub struct Day2Pt1;

enum SortDir {
    Incr(bool),
    Decr(bool),
    NotSafe,
}

impl Solution for Day2Pt1 {
    fn solve(&self) {
        let input = fs::read_to_string("./inputs/day2.txt").unwrap();

        let mut diffs: Vec<SortDir> = Vec::new();
        for report in input.split('\n') {
            if report.len() == 0 {
                continue;
            }

            let nums: Vec<i64> = report
                .split(' ')
                .into_iter()
                .map(|i| i.parse::<i64>().unwrap())
                .collect();

            let incr_or_decr = nums[0] < nums[1];
            let mut not_safe = false;
            for i in 0..(nums.len() - 1) {
                if (nums[i] < nums[i + 1]) != incr_or_decr {
                    not_safe = true;
                }
            }

            if not_safe {
                diffs.push(SortDir::NotSafe);
                continue;
            }

            let mut max_diff = 0;

            for i in 0..(nums.len() - 1) {
                let local_diff = (nums[i] - nums[i + 1]).abs();
                if local_diff > max_diff {
                    max_diff = local_diff;
                }
                if local_diff < 1 || local_diff > 3 {
                    not_safe = true;
                }
            }

            if nums[0] < nums[1] {
                diffs.push(SortDir::Incr(!not_safe));
            } else {
                diffs.push(SortDir::Decr(!not_safe));
            }
        }

        let safe_count: usize = diffs
            .into_iter()
            .filter(|i| match *i {
                SortDir::NotSafe => false,
                SortDir::Incr(safe) => safe,
                SortDir::Decr(safe) => safe,
            })
            .count();

        println!("{safe_count}");
    }
}

pub struct Day2Pt2;

impl Day2Pt2 {
    fn is_safe_level(&self, report: Vec<i64>) -> SortDir {
        let incr_or_decr = report[0] < report[1];
        let mut not_safe = false;
        for i in 0..(report.len() - 1) {
            if (report[i] < report[i + 1]) != incr_or_decr {
                not_safe = true;
            }
        }

        if not_safe {
            return SortDir::NotSafe;
        }

        let mut max_diff = 0;

        for i in 0..(report.len() - 1) {
            let local_diff = (report[i] - report[i + 1]).abs();
            if local_diff > max_diff {
                max_diff = local_diff;
            }
            if local_diff < 1 || local_diff > 3 {
                not_safe = true;
            }
        }

        if report[0] < report[1] {
            return SortDir::Incr(!not_safe);
        } else {
            return SortDir::Decr(!not_safe);
        }
    }
}

impl Solution for Day2Pt2 {
    fn solve(&self) {
        let input = fs::read_to_string("./inputs/day2.txt").unwrap();

        let mut diffs: Vec<SortDir> = Vec::new();
        for report in input.split('\n') {
            if report.len() == 0 {
                continue;
            }

            let nums: Vec<i64> = report
                .split(' ')
                .into_iter()
                .map(|i| i.parse::<i64>().unwrap())
                .collect();

            let level_safe = self.is_safe_level(nums.clone());
            if match level_safe {
                SortDir::NotSafe => false,
                SortDir::Incr(safe) => safe,
                SortDir::Decr(safe) => safe,
            } {
                diffs.push(level_safe);
                continue;
            }

            for i in 0..nums.len() {
                let mut new_nums = nums.clone();
                if i != nums.len() - 1 {
                    new_nums.remove(i);
                } else {
                    new_nums.pop();
                }
                let level_safe = self.is_safe_level(new_nums);
                if match level_safe {
                    SortDir::NotSafe => false,
                    SortDir::Incr(safe) => safe,
                    SortDir::Decr(safe) => safe,
                } {
                    diffs.push(level_safe);
                    break;
                }
            }
        }

        let safe_count: usize = diffs
            .into_iter()
            .filter(|i| match *i {
                SortDir::NotSafe => false,
                SortDir::Incr(safe) => safe,
                SortDir::Decr(safe) => safe,
            })
            .count();

        println!("{safe_count}");
    }
}
