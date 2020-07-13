use std::cmp::min;
use std::ops::Mul;
use crate::utils::vector::Vector4;
use crate::utils::scalar::RoundTo;

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<f32>,
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        let data: Vec<f32> = vec![0.0; rows * columns];
        Matrix{rows, columns, data}
    }

    pub fn identity(rows: usize, columns: usize) -> Self {
        let mut out = Self::new(rows, columns);
        for row in 0..rows {
            for column in 0..columns {
                if row == column {
                    out.set(row, column, 1.0);
                }
            }
        }
        out
    }

    pub fn fill(mut m: Self, data: Vec<f32>) -> Self {
        let length: usize = min(m.len(), data.len());
        for i in 0..length {
            m.data[i] = data[i];
        }
        m
    }

    pub fn submatrix(m: Self, row_to_remove: usize, column_to_remove: usize) -> Self {
        let mut out = Self::new(m.rows - 1, m.columns - 1);
        let mut current_row = 0;
        let mut current_column = 0;
        for row in 0..m.rows {
            if row != row_to_remove {
                for column in 0..m.columns {
                    if column != column_to_remove {
                        out.set(current_row, current_column, m.get(row, column));
                        current_column = current_column + 1;
                    }
                }
                current_row = current_row + 1;
                current_column = 0;
            }
        }
        out
    }

    pub fn minor(m: Self, row: usize, column: usize) -> f32 {
        Self::determinant(Self::submatrix(m, row, column))
    }

    pub fn cofactor(m: Self, row: usize, column: usize) -> f32 {
        let minor_value = Self::minor(m, row, column);
        if (row + column) % 2 == 0 {
            minor_value
        } else {
            minor_value * -1.0
        }
    }

    pub fn determinant(m: Self) -> f32 {
        let mut det: f32 = 0.0;
        if m.len() == 4 {
            det = m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0);
        } else {
            for column in 0..m.columns {
                det += m.get(0, column) * Self::cofactor(m.clone(), 0, column);
            }
        }
        det
    }

    pub fn inverse(m: Self) -> Self {
        let mut out = Self::new(m.rows, m.columns);
        let det = Self::determinant(m.clone());
        for row in 0..m.rows {
            for column in 0..m.columns {
                out.set(column, row, Self::cofactor(m.clone(), row, column) / det);
            }
        }
        out
    }

    pub fn round_to(m: Self, digits_after_comma: u8) -> Self {
        let mut out = Self::new(m.rows, m.columns);
        for row in 0..m.rows {
            for column in 0..m.columns {
                out.set(row, column, m.get(row, column).round_to(digits_after_comma));
            }
        }
        out
    }

    pub fn get(&self, row: usize, column: usize) -> f32 {
        self.data[row * self.columns + column]
    }

    pub fn set(&mut self, row: usize, column: usize, value: f32) -> &Self {
        self.data[row * self.rows + column] = value;
        self
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        if self.len() != rhs.len() {
            return false;
        } else {
            for i in 0..self.len() {
                if self.data[i] != rhs.data[i] {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Matrix::new(self.rows, self.columns);
        for row in 0..self.rows {
            for column in 0..self.columns {
                out.set(row, column,
                        self.get(row, 0) * rhs.get(0, column)
                        + self.get(row, 1) * rhs.get(1, column)
                        + self.get(row, 2) * rhs.get(2, column)
                        + self.get(row, 3) * rhs.get(3, column)
                );
            }
        }
        out
    }
}

impl Mul<Vector4> for Matrix {
    type Output = Vector4;

    fn mul(self, v: Vector4) -> Self::Output {
        let mut out = Vector4::new(0.0, 0.0, 0.0, 0.0);
        for row in 0..self.rows {
            out.set(row,
                    self.get(row, 0) * v.x
                    + self.get(row, 1) * v.y
                    + self.get(row, 2) * v.z
                    + self.get(row, 3) * v.w
            );
        }
        out
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        let mut out = Matrix::new(self.rows, self.columns);
        out.rows = self.rows;
        out.columns = self.columns;
        out.data = self.data.to_vec();
        out
    }
}

#[macro_export]
macro_rules! matrix {
    ($rows: expr, $columns: expr) => (
        Matrix::new($rows, $columns)
    );
}

#[macro_export]
macro_rules! identity_matrix {
    ($rows: expr, $columns: expr) => (
        Matrix::identity($rows, $columns)
    );
}
