pub mod template;
use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

use ndarray::{Array, Array2, Order};
use Direction::*;

// Use this file to add helper functions and additional modules.

/// Get the positions adjacent to a given position, if they are within the given bounds. Does not
/// count diagonals.
pub fn get_adjacent_positions(
    pos: impl Into<Pos<usize>>,
    bounds: impl Into<Pos<usize>>,
) -> impl Iterator<Item = Pos<usize>> + Clone {
    let pos = pos.into();
    let bounds = bounds.into();
    DIRECTIONS
        .iter()
        .filter_map(move |dir| (pos + *dir).in_bounds(bounds))
}

pub fn print_matrix(matrix: &Array2<char>) {
    for row in matrix.outer_iter() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

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

impl Direction {
    pub fn turn_right(&self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn turn_left(self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        use Direction::*;
        match c {
            '^' => Some(North),
            '>' => Some(East),
            'v' => Some(South),
            '<' => Some(West),
            _ => None,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos<T>(pub T, pub T);

impl<T: Display> Display for Pos<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({}, {})", self.0, self.1)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Pos<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({:?}, {:?})", self.0, self.1)
    }
}

impl std::ops::AddAssign<Direction> for Pos<isize> {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl<T> Mul<T> for Pos<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Pos(self.0 * rhs, self.1 * rhs)
    }
}

impl<T> From<(T, T)> for Pos<T> {
    fn from(value: (T, T)) -> Self {
        Pos(value.0, value.1)
    }
}

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
        new.in_bounds(bounds)
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
    pub fn tuple(self) -> (usize, usize) {
        (self.0.try_into().unwrap(), self.1.try_into().unwrap())
    }
}

impl Pos<isize> {
    /// Check if a position is within zero and the given bounds, returning the casted position if it
    /// is.
    pub fn in_bounds(self, bounds: impl Into<Pos<usize>>) -> Option<Pos<usize>> {
        let bounds: Pos<usize> = bounds.into();
        let in_bounds =
            self.0 >= 0 && self.0 < bounds.0 as isize && self.1 >= 0 && self.1 < bounds.1 as isize;
        if in_bounds {
            Some(Pos(self.0 as usize, self.1 as usize))
        } else {
            None
        }
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

impl Add<Direction> for Pos<isize> {
    type Output = Pos<isize>;

    /// Add a signed position to an unsigned position, saturating on overflow.
    fn add(self, rhs: Direction) -> Self::Output {
        let rhs: Pos<isize> = rhs.into();
        self + rhs
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

impl<D: std::borrow::Borrow<Direction>> From<D> for Pos<isize> {
    fn from(value: D) -> Self {
        match *value.borrow() {
            North => Pos(-1, 0),
            East => Pos(0, 1),
            South => Pos(1, 0),
            West => Pos(0, -1),
        }
    }
}
