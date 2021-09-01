use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
};

use rand::{prelude::SliceRandom, Rng};

#[macro_use]
extern crate dmoj;

pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    use std::cmp::min;
    use std::mem::swap;

    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both u and v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} is even", u);
        debug_assert!(v % 2 == 1, "v = {} is even", v);

        // Swap if necessary so u <= v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
        v -= u;

        // Identity 1: gcd(u, 0) = u
        // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
        if v == 0 {
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) (u is known to be odd)
        v >>= v.trailing_zeros();
    }
}

type Line = (i32, i32, i32, i32);
type Slope = (i32, i32);

fn main() {
    let amount = scan!(i32);
    let mut slopes: HashMap<Slope, Vec<(f64, Line)>> = HashMap::new();

    let mut square_counter = 0;
    for _ in 0..amount {
        let (x1, y1, x2, y2) = scan!(i32, i32, i32, i32);
        let xs = x2 - x1;
        let ys = y2 - y1;

        let mut slope_divisor: i32 =
            gcd(xs.abs().try_into().unwrap(), ys.abs().try_into().unwrap())
                .try_into()
                .unwrap();
        if ys.is_negative() {
            slope_divisor = -slope_divisor;
        }

        let mut main_slope = (xs / slope_divisor, ys / slope_divisor);
        if main_slope.1 == 0 {
            main_slope.0 = main_slope.0.abs();
        }

        let main_line = (x1, y1, x2, y2);
        let main_dist = distance_perp(main_slope, main_line);

        let matching_lines = slopes.entry(main_slope).or_default();
        let insertion = match matching_lines.binary_search_by(|v| {
            v.0.partial_cmp(&main_dist)
                .expect("Couldn't compare values")
        }) {
            Ok(v) => v,
            Err(v) => v,
        };
        matching_lines.insert(insertion, (main_dist, main_line));

        if matching_lines.len() == 1 {
            continue;
        }

        let opposing_slope = if !main_slope.0.is_positive() {
            (main_slope.1, -main_slope.0)
        } else {
            (-main_slope.1, main_slope.0)
        };

        if let Some(opposing_lines) = slopes.get(&opposing_slope) {
            if opposing_lines.len() < 2 {
                continue;
            }
            for matching_line in slopes.get(&main_slope).unwrap().into_iter() {
                if matching_line.0 == main_dist {
                    continue;
                }
                for (idx, opposing_line_0) in opposing_lines.into_iter().enumerate() {
                    for opposing_line_1 in opposing_lines[idx + 1..].into_iter() {
                        if ((opposing_line_0.0 - opposing_line_1.0).abs()
                            - (matching_line.0 - main_dist).abs())
                        .abs()
                            < 1.0 / 2000.0
                        {
                            square_counter += 1;
                        }
                    }
                }

                //     let matching_dist = (matching_line.0 - main_dist).abs();
                //     let mut matched_pairs: HashSet<(usize, usize)> = HashSet::new();
                //     for (first_index, line_o) in opposing_lines.iter().enumerate() {
                //         let index = match opposing_lines.binary_search_by(|v| {
                //             v.0.partial_cmp(&(matching_dist - line_o.0).abs())
                //                 .expect("Couldn't compare values")
                //         }) {
                //             Ok(v) => v,
                //             Err(v) => v,
                //         };

                //         let index_neg = match opposing_lines.binary_search_by(|v| {
                //             v.0.partial_cmp(&-(matching_dist - line_o.0).abs())
                //                 .expect("Couldn't compare values")
                //         }) {
                //             Ok(v) => v,
                //             Err(v) => v,
                //         };

                //         if !matched_pairs.contains(&(index, first_index)) {
                //             if let Some(line_o_2) = opposing_lines.get(index) {
                //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs() < 1.0 / 2000.0
                //                 {
                //                     square_counter += 1;
                //                     matched_pairs.insert((index, first_index));
                //                     matched_pairs.insert((first_index, index));
                //                 }
                //             }
                //         }

                //         if index > 0 && !matched_pairs.contains(&(index - 1, first_index)) {
                //             if let Some(line_o_2) = opposing_lines.get(index - 1) {
                //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs() < 1.0 / 2000.0
                //                 {
                //                     square_counter += 1;
                //                     matched_pairs.insert((index - 1, first_index));
                //                     matched_pairs.insert((first_index, index - 1));
                //                 }
                //             }
                //         }

                //         if !matched_pairs.contains(&(index_neg, first_index)) {
                //             if let Some(line_o_2) = opposing_lines.get(index_neg) {
                //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs() < 1.0 / 2000.0
                //                 {
                //                     square_counter += 1;
                //                     matched_pairs.insert((index_neg, first_index));
                //                     matched_pairs.insert((first_index, index_neg));
                //                 }
                //             }
                //         }

                //         if index_neg > 0 && !matched_pairs.contains(&(index_neg - 1, first_index)) {
                //             if let Some(line_o_2) = opposing_lines.get(index_neg - 1) {
                //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs() < 1.0 / 2000.0
                //                 {
                //                     square_counter += 1;
                //                     matched_pairs.insert((index_neg - 1, first_index));
                //                     matched_pairs.insert((first_index, index_neg - 1));
                //                 }
                //             }
                //         }
                //     }
            }
        }
    }
    println!("{}", square_counter);
}

fn main_flame() {
    let vals: Vec<Line> = vec![
        (0, 0, 1, 0),
        (0, 1, 1, 1),
        (0, 2, 2, 2),
        (0, 0, 0, 4),
        (1, -1, 1, 0),
        (2, -2, 2, 2),
        (1, 1, 2, 2),
        (1, 2, 2, 3),
        (3, 0, 2, 1),
        (1, 1, 0, 2),
        (3, 1, 2, 2),
        (1, 3, 0, 2),
        (4, 0, 4, 4),
        (5, 0, 5, 4),
        (3, 0, 3, 4),
    ];
    for _ in 1..200 {
        let mut slopes: HashMap<Slope, Vec<(f64, Line)>> = HashMap::new();
        let mut square_counter = 0;
        let const2: i32 = rand::thread_rng().gen_range(-1000..1000);
        let mut const1: i32 = rand::thread_rng().gen_range(-1000..1000);
        if const1 == 0 {
            const1 = 1;
        }
        let mut new_vals = vals.clone();
        new_vals.shuffle(&mut rand::thread_rng());
        for val in new_vals {
            let (x1, y1, x2, y2) = val;
            let x1 = x1 * const1 + const2;
            let y1 = y1 * const1 + const2;
            let x2 = x2 * const1 + const2;
            let y2 = y2 * const1 + const2;
            let xs = x2 - x1;
            let ys = y2 - y1;

            let mut slope_divisor: i32 =
                gcd(xs.abs().try_into().unwrap(), ys.abs().try_into().unwrap())
                    .try_into()
                    .unwrap();
            if ys.is_negative() {
                slope_divisor = -slope_divisor;
            }

            let mut main_slope = (xs / slope_divisor, ys / slope_divisor);
            if main_slope.1 == 0 {
                main_slope.0 = main_slope.0.abs();
            }

            let main_line = (x1, y1, x2, y2);
            let main_dist = distance_perp(main_slope, main_line);

            let matching_lines = slopes.entry(main_slope).or_default();
            let insertion = match matching_lines.binary_search_by(|v| {
                v.0.partial_cmp(&main_dist)
                    .expect("Couldn't compare values")
            }) {
                Ok(v) => v,
                Err(v) => v,
            };
            matching_lines.insert(insertion, (main_dist, main_line));

            if matching_lines.len() == 1 {
                continue;
            }

            let opposing_slope = if !main_slope.0.is_positive() {
                (main_slope.1, -main_slope.0)
            } else {
                (-main_slope.1, main_slope.0)
            };

            if let Some(opposing_lines) = slopes.get(&opposing_slope) {
                if opposing_lines.len() < 2 {
                    continue;
                }
                for matching_line in slopes.get(&main_slope).unwrap().into_iter() {
                    if matching_line.0 == main_dist {
                        continue;
                    }
                    let matching_dist = (matching_line.0 - main_dist).abs();
                    let mut matched_pairs: HashSet<(usize, usize)> = HashSet::new();
                    for (first_index, line_o) in opposing_lines.iter().enumerate() {
                        let index = match opposing_lines.binary_search_by(|v| {
                            v.0.partial_cmp(&(matching_dist - line_o.0).abs())
                                .expect("Couldn't compare values")
                        }) {
                            Ok(v) => v,
                            Err(v) => v,
                        };

                        let index_neg = match opposing_lines.binary_search_by(|v| {
                            v.0.partial_cmp(&-(matching_dist - line_o.0).abs())
                                .expect("Couldn't compare values")
                        }) {
                            Ok(v) => v,
                            Err(v) => v,
                        };

                        if !matched_pairs.contains(&(index, first_index)) {
                            if let Some(line_o_2) = opposing_lines.get(index) {
                                if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                                    < 1.0 / 2000.0
                                {
                                    square_counter += 1;
                                    matched_pairs.insert((index, first_index));
                                    matched_pairs.insert((first_index, index));
                                }
                            }
                        }

                        if index > 0 && !matched_pairs.contains(&(index - 1, first_index)) {
                            if let Some(line_o_2) = opposing_lines.get(index - 1) {
                                if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                                    < 1.0 / 2000.0
                                {
                                    square_counter += 1;
                                    matched_pairs.insert((index - 1, first_index));
                                    matched_pairs.insert((first_index, index - 1));
                                }
                            }
                        }

                        if !matched_pairs.contains(&(index_neg, first_index)) {
                            if let Some(line_o_2) = opposing_lines.get(index_neg) {
                                if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                                    < 1.0 / 2000.0
                                {
                                    square_counter += 1;
                                    matched_pairs.insert((index_neg, first_index));
                                    matched_pairs.insert((first_index, index_neg));
                                }
                            }
                        }

                        if index_neg > 0 && !matched_pairs.contains(&(index_neg - 1, first_index)) {
                            if let Some(line_o_2) = opposing_lines.get(index_neg - 1) {
                                if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                                    < 1.0 / 2000.0
                                {
                                    square_counter += 1;
                                    matched_pairs.insert((index_neg - 1, first_index));
                                    matched_pairs.insert((first_index, index_neg - 1));
                                }
                            }
                        }
                    }
                }
            }
        }
        dbg!((square_counter, 10));
    }
}

pub fn distance_perp(slope: Slope, line: Line) -> f64 {
    if slope.0 == 0 {
        return line.0.into();
    }

    let descriminant: f64 = (slope.0.pow(2) + slope.1.pow(2)).into();
    let y_intercept: f64 = (line.1 - slope.1 / slope.0 * line.0).into();
    let abs_dist = y_intercept / descriminant.sqrt();

    abs_dist
}

#[cfg(test)]
pub mod tests {
    use std::{
        collections::{HashMap, HashSet},
        convert::TryInto,
    };

    use rand::{prelude::SliceRandom, Rng};

    use crate::{distance_perp, gcd, Line, Slope};

    #[test]
    fn test_distance_perp() {
        debug_assert!(
            (distance_perp((-1, 1), (0, 2, 2, 0)) - (2.0_f64).sqrt()).abs() < 10.0_f64.powf(-9.0)
        );
        debug_assert!(
            (distance_perp((-1, 1), (0, 1, 1, 0)) - (2.0_f64).sqrt() / 2.0).abs()
                < 10.0_f64.powf(-9.0)
        );
    }

    #[test]
    pub fn test_squares() {
        let vals: Vec<Line> = vec![
            (0, 0, 1, 0),
            (0, 1, 1, 1),
            (0, 2, 2, 2),
            (0, 0, 0, 4),
            (1, -1, 1, 0),
            (2, -2, 2, 2),
            (1, 1, 2, 2),
            (1, 2, 2, 3),
            (3, 0, 2, 1),
            (1, 1, 0, 2),
            (3, 1, 2, 2),
            (1, 3, 0, 2),
            (4, 0, 4, 4),
            (5, 0, 5, 4),
            (3, 0, 3, 4),
        ];
        for _ in 1..200 {
            let mut slopes: HashMap<Slope, Vec<(f64, Line)>> = HashMap::new();
            let mut square_counter = 0;
            let const2: i32 = rand::thread_rng().gen_range(-1000..1000);
            let mut const1: i32 = rand::thread_rng().gen_range(-1000..1000);
            if const1 == 0 {
                const1 = 1;
            }
            let mut new_vals = vals.clone();
            new_vals.shuffle(&mut rand::thread_rng());
            for val in new_vals {
                let (x1, y1, x2, y2) = val;
                let x1 = x1 * const1 + const2;
                let y1 = y1 * const1 + const2;
                let x2 = x2 * const1 + const2;
                let y2 = y2 * const1 + const2;
                let xs = x2 - x1;
                let ys = y2 - y1;

                let mut slope_divisor: i32 =
                    gcd(xs.abs().try_into().unwrap(), ys.abs().try_into().unwrap())
                        .try_into()
                        .unwrap();
                if ys.is_negative() {
                    slope_divisor = -slope_divisor;
                }

                let mut main_slope = (xs / slope_divisor, ys / slope_divisor);
                if main_slope.1 == 0 {
                    main_slope.0 = main_slope.0.abs();
                }

                let main_line = (x1, y1, x2, y2);
                let main_dist = distance_perp(main_slope, main_line);

                let matching_lines = slopes.entry(main_slope).or_default();
                let insertion = match matching_lines.binary_search_by(|v| {
                    v.0.partial_cmp(&main_dist)
                        .expect("Couldn't compare values")
                }) {
                    Ok(v) => v,
                    Err(v) => v,
                };
                matching_lines.insert(insertion, (main_dist, main_line));

                if matching_lines.len() == 1 {
                    continue;
                }

                let opposing_slope = if !main_slope.0.is_positive() {
                    (main_slope.1, -main_slope.0)
                } else {
                    (-main_slope.1, main_slope.0)
                };

                if let Some(opposing_lines) = slopes.get(&opposing_slope) {
                    if opposing_lines.len() < 2 {
                        continue;
                    }
                    for matching_line in slopes.get(&main_slope).unwrap().into_iter() {
                        if matching_line.0 == main_dist {
                            continue;
                        }
                        for (idx, opposing_line_0) in opposing_lines.into_iter().enumerate() {
                            for opposing_line_1 in opposing_lines[idx + 1..].into_iter() {
                                if ((opposing_line_0.0 - opposing_line_1.0).abs()
                                    - (matching_line.0 - main_dist).abs())
                                .abs()
                                    < 1.0 / 2000.0
                                {
                                    square_counter += 1;
                                }
                            }
                        }
                    }
                    // for matching_line in slopes.get(&main_slope).unwrap().into_iter() {
                    //     if matching_line.0 == main_dist {
                    //         continue;
                    //     }
                    //     let matching_dist = (matching_line.0 - main_dist).abs();
                    //     let mut matched_pairs: HashSet<(usize, usize)> = HashSet::new();
                    //     for (first_index, line_o) in opposing_lines.iter().enumerate() {
                    //         let index = match opposing_lines.binary_search_by(|v| {
                    //             v.0.partial_cmp(&(matching_dist - line_o.0).abs())
                    //                 .expect("Couldn't compare values")
                    //         }) {
                    //             Ok(v) => v,
                    //             Err(v) => v,
                    //         };

                    //         let index_neg = match opposing_lines.binary_search_by(|v| {
                    //             v.0.partial_cmp(&-(matching_dist - line_o.0).abs())
                    //                 .expect("Couldn't compare values")
                    //         }) {
                    //             Ok(v) => v,
                    //             Err(v) => v,
                    //         };

                    //         if !matched_pairs.contains(&(index, first_index)) {
                    //             if let Some(line_o_2) = opposing_lines.get(index) {
                    //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                    //                     < 1.0 / 2000.0
                    //                 {
                    //                     square_counter += 1;
                    //                     matched_pairs.insert((index, first_index));
                    //                     matched_pairs.insert((first_index, index));
                    //                 }
                    //             }
                    //         }

                    //         if index > 0 && !matched_pairs.contains(&(index - 1, first_index)) {
                    //             if let Some(line_o_2) = opposing_lines.get(index - 1) {
                    //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                    //                     < 1.0 / 2000.0
                    //                 {
                    //                     square_counter += 1;
                    //                     matched_pairs.insert((index - 1, first_index));
                    //                     matched_pairs.insert((first_index, index - 1));
                    //                 }
                    //             }
                    //         }

                    //         if !matched_pairs.contains(&(index_neg, first_index)) {
                    //             if let Some(line_o_2) = opposing_lines.get(index_neg) {
                    //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                    //                     < 1.0 / 2000.0
                    //                 {
                    //                     square_counter += 1;
                    //                     matched_pairs.insert((index_neg, first_index));
                    //                     matched_pairs.insert((first_index, index_neg));
                    //                 }
                    //             }
                    //         }

                    //         if index_neg > 0
                    //             && !matched_pairs.contains(&(index_neg - 1, first_index))
                    //         {
                    //             if let Some(line_o_2) = opposing_lines.get(index_neg - 1) {
                    //                 if ((line_o_2.0 - line_o.0).abs() - matching_dist).abs()
                    //                     < 1.0 / 2000.0
                    //                 {
                    //                     square_counter += 1;
                    //                     matched_pairs.insert((index_neg - 1, first_index));
                    //                     matched_pairs.insert((first_index, index_neg - 1));
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                }
            }
            assert_eq!(square_counter, 19);
        }
    }
}
