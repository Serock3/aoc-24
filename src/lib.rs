pub mod template;

// Use this file to add helper functions and additional modules.

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn parse_char_matrix(input: &str) -> Array2<char> {
    let cols = input
        .lines()
        .map(|l| l.len())
        .reduce(|prev, next| {
            assert_eq!(prev, next);
            next
        })
        .unwrap();
    let rows = input.lines().count();
    let shape = (rows, cols);

    let chars = input.chars().filter(|c| *c != '\n');
    Array::from_iter(chars)
        .into_shape_clone((shape, Order::RowMajor))
        .unwrap()
}

pub fn parse_int_matrix<T>(input: &str) -> Array2<T>
where
    T: TryFrom<u32, Error: std::fmt::Debug> + Clone,
{
    let cols = input
        .lines()
        .map(|l| l.len())
        .reduce(|prev, next| {
            assert_eq!(prev, next, "Inconsistent row lengths: {} != {}", prev, next);
            next
        })
        .unwrap();
    let rows = input.lines().count();
    let shape = (rows, cols);

    let chars = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c.to_digit(10) {
            Some(digit @ ..10) => digit.try_into().unwrap(),
            _ => panic!("Invalid digit: {}", c),
        });
    Array::from_iter(chars)
        .into_shape_clone((shape, Order::RowMajor))
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use std::ops::{Add, Neg, Sub};

use ndarray::{Array, Array2, Order};
use Direction::*;

pub const DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos<T>(pub T, pub T);

impl<T: Copy> From<[T; 2]> for Pos<T> {
    fn from(value: [T; 2]) -> Self {
        Pos(value[0], value[1])
    }
}

impl<T> From<Pos<T>> for [T; 2] {
    fn from(val: Pos<T>) -> Self {
        [val.0, val.1]
    }
}

impl<T> From<Pos<T>> for (T, T) {
    fn from(val: Pos<T>) -> Self {
        (val.0, val.1)
    }
}

impl<T: Add<Output = T>> Add for Pos<T> {
    type Output = Self;

    fn add(self, rhs: Pos<T>) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Pos<usize> {
    /// Add a signed position to an unsigned position, saturating on overflow.
    pub fn add_checked_with_bounds(
        self,
        rhs: impl Into<Pos<isize>>,
        bounds: impl Into<Pos<usize>>,
    ) -> Option<Self> {
        let new = self + rhs;
        if new.in_bounds(bounds) {
            Some(Pos(new.0 as usize, new.1 as usize))
        } else {
            None
        }
    }

    pub fn add_saturating(self, rhs: impl Into<Pos<isize>>) -> Self {
        let rhs: Pos<isize> = rhs.into();
        Pos(
            self.0.saturating_add_signed(rhs.0),
            self.1.saturating_add_signed(rhs.1),
        )
    }

    pub fn tuple(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl Pos<isize> {
    pub fn in_bounds(self, bounds: impl Into<Pos<usize>>) -> bool {
        let bounds: Pos<usize> = bounds.into();
        self.0 >= 0 && self.0 < bounds.0 as isize && self.1 >= 0 && self.1 < bounds.1 as isize
    }
}

impl<T: Into<Pos<isize>>> Add<T> for Pos<usize> {
    type Output = Pos<isize>;

    /// Add a signed position to an unsigned position, saturating on overflow.
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Pos(
            rhs.0.wrapping_add_unsigned(self.0),
            rhs.1.wrapping_add_unsigned(self.1),
        )
    }
}

impl<T: TryInto<isize, Error: std::fmt::Debug>> Sub<Pos<T>> for Pos<usize> {
    type Output = Pos<isize>;

    /// Subtract a signed position to an unsigned position, saturating on overflow.
    fn sub(self, rhs: Pos<T>) -> Self::Output {
        let x: isize = rhs.0.try_into().unwrap();
        let y: isize = rhs.1.try_into().unwrap();
        Pos(
            x.neg().wrapping_add_unsigned(self.0),
            y.neg().wrapping_add_unsigned(self.1),
        )
    }
}

impl From<Direction> for Pos<isize> {
    fn from(value: Direction) -> Self {
        match value {
            North => Pos(-1, 0),
            East => Pos(0, 1),
            South => Pos(1, 0),
            West => Pos(0, -1),
        }
    }
}