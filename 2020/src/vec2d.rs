use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::iter::FromIterator;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Vec2d<T>(Vec<Vec<T>>);

impl<T> Vec2d<T> {
    pub fn row_count(&self) -> usize {
        self.0.len()
    }

    pub fn column_count(&self) -> usize {
        if self.0.len() == 0 { 0 } else { self.0[0].len() }
    }

    pub fn items(&self) -> impl Iterator<Item=&T> {
        self.0.iter().flat_map(|row| row.iter())
    }

    pub fn indices(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.0.iter().enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().map(move |(j, _)| (i, j))
            })
    }
}

impl<T, I: Iterator<Item=T>> FromIterator<I> for Vec2d<T> {
    fn from_iter<I2: IntoIterator<Item=I>>(iter: I2) -> Self {
        Vec2d(iter.into_iter().map(|row| row.collect()).collect())
    }
}

impl<T> From<Vec<Vec<T>>> for Vec2d<T> {
    fn from(contents: Vec<Vec<T>>) -> Self {
        Vec2d(contents)
    }
}

impl<T> Deref for Vec2d<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vec2d<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.0[row][column]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row][column]
    }
}

impl<T> Index<usize> for Vec2d<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Vec2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
