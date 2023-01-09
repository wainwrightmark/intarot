// use std::{fmt::Display, num::ParseIntError, ops::Range, str::FromStr};

// use rand::{rngs::ThreadRng, Rng};

// #[derive(
//     Copy, Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default,
// )]
// pub struct Ordering(pub u128);

// impl FromStr for Ordering {
//     type Err = ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         u128::from_str(s).map(|x| Self(x))
//     }
// }

// impl Display for Ordering {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }

// impl From<u128> for Ordering {
//     fn from(value: u128) -> Self {
//         Self(value)
//     }
// }

// impl Ordering {
//     pub fn gen(count: usize) -> Self {
//         Self(ThreadRng::default().gen_range(Ordering::get_range(&count)))
//     }

//     ///Reorder an array in sorted order to this ordering
//     pub fn reorder<T>(&self, arr: &mut [T]) {
//         let mut rem = self.0;

//         for i in 0..arr.len() {
//             let index = (rem % ((arr.len() - i) as u128)) as usize;
//             arr.swap(i, index + i);
//             rem /= (arr.len() - i) as u128;
//         }
//     }

//     ///Get the range of legal values for arrays of length l
//     pub fn get_range(l: &usize) -> Range<u128> {
//         if *l == 0 {
//             return 0..1;
//         }
//         let num: u128 = *l as u128;

//         0..(1..=num).product()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::collections::HashSet;

//     use crate::data::prelude::*;
//     use anyhow::Ok;
//     use itertools::Itertools;
//     use ntest::test_case;

//     #[test_case(0, "0123")]
//     #[test_case(1, "1023")]
//     #[test_case(2, "2103")]
//     pub fn should_order_correctly(o: u128, expected: &str) -> Result<(), anyhow::Error> {
//         let ordering = Ordering(o);

//         let mut arr = [0, 1, 2, 3];

//         ordering.reorder(&mut arr);

//         let actual = arr.into_iter().map(|x| x.to_string()).join("");

//         assert_eq!(expected, actual);

//         Ok(())
//     }

//     #[test]
//     pub fn all_possible_orderings_are_unique() -> Result<(), anyhow::Error> {
//         let mut set: HashSet<[i32; 4]> = Default::default();

//         for o in Ordering::get_range(&4) {
//             let ordering = Ordering(o);
//             let mut arr = [0, 1, 2, 3];
//             ordering.reorder(&mut arr);

//             let added = set.insert(arr);
//             assert!(added);
//         }

//         assert_eq!(set.len(), 24);

//         Ok(())
//     }
// }
