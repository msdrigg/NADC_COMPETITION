use crate::segment_tree::PointSegment;

use std::collections::HashMap;
use std::io;
use std::str;

#[derive(Copy, Clone)]
struct Employee {
    wage: i32,
    employee_count: usize,
}

/// Reads white-space separated tokens one at a time.
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

pub struct EmployeeTree {
    pub wage: i32,
    pub employees: Vec<EmployeeTree>,
}

struct TreeIterator {
    stack: Vec<EmployeeTree>,
    next: Option<(i32, usize)>,
}
impl Iterator for TreeIterator {
    type Item = (i32, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next.take() {
            return Some(next);
        }

        if let Some(EmployeeTree {
            wage, employees, ..
        }) = self.stack.pop()
        {
            if let Some(employees) = employees {
                let box right = right;
                self.stack.push(right);
            }
            return Some(wage);
        }

        None
    }
}

impl IntoIterator for EmployeeTree {
    type Item = (i32, usize);
    type IntoIter = TreeIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = Vec::new();

        let smallest = pop_smallest(self, &mut stack);

        TreeIterator {
            stack: stack,
            next: Some(smallest),
        }
    }
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// This function should be marked unsafe, but noone has time for that in a
    /// programming contest. Use at your own risk!
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    /// Use "turbofish" syntax token::<T>() to select data type of next token.
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

pub fn method3<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let employees = scan.token::<usize>();
    let changes = scan.token::<usize>();
    let ceo_wage = scan.token::<i32>();

    let employee_tree = EmployeeTree {
        wage: ceo_wage,
        employees: Vec::new(),
    };

    let employee_map = Vec::<*mut EmployeeTree>::new();

    for _ in 2..=employees {
        let wage = scan.token::<i32>();
        let boss_id = scan.token::<usize>();
        let boss = unsafe { **employee_map.get(boss_id - 1).unwrap() };
        boss.employees.push(EmployeeTree {
            wage,
            employees: Vec::new(),
        });
        employee_map.push(boss.employees.last_mut().unwrap() as *mut _);
    }

    let mut employee_locator = Vec::<usize>::with_capacity(employees);

    let employee_vec: Vec<_> = employee_tree
        .into_iter()
        .enumerate()
        .map(|(loc, (wage, employee_count))| {
            employee_locator.push(loc);
            Employee {
                wage,
                employee_count,
            }
        })
        .collect();

    let employee_wages: Vec<_> = employee_vec.iter().copied().map(|x| x.wage).collect();
    let mut seg_tree = PointSegment::build(employee_wages, crate::ops::Add);

    for _ in 0..changes {
        let next_action = scan.token::<char>();
        if next_action == 'u' {
            let employee = scan.token::<usize>();
            writeln!(
                out,
                "{}",
                seg_tree.query(*employee_locator.get(employee - 1).unwrap()),
            )
            .ok();
            // queries.push(Query { employee, time: i });
        } else {
            let employee = scan.token::<usize>();
            let wage_change = scan.token::<i32>();

            let employer_location = *employee_locator.get(employee - 1).unwrap();
            let employer = employee_vec.get(employer_location).unwrap();
            if employer.employee_count != 0 {
                seg_tree.modify(
                    employer_location + 1,
                    employer_location + 1 + employer.employee_count,
                    wage_change,
                );
            }
        }
    }
}

// fn increment_employee_count(
//     employee_vec: &mut Vec<Employee>,
//     employee_locator: &Vec<usize>,
//     boss_loc: usize,
// ) {
//     let boss = employee_vec.get_mut(boss_loc).unwrap();
//     boss.employee_count += 1;
//     if boss.boss_id == 0 {
//         return;
//     };
//     let bosses_boss_loc = *employee_locator.get(boss.boss_id - 1).unwrap();
//     increment_employee_count(employee_vec, employee_locator, bosses_boss_loc);
// }
