use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign,Mul};

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

// 矩阵乘法
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
    where T: Copy + Default + Add<Output=T> + AddAssign + Mul<Output=T>
{
    if a.row != b.col {
        return Err(anyhow::anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let mut data: Vec<T> = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> fmt::Display for Matrix<T> where T: fmt::Display {
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
    where
        T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> anyhow::Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {22 28, 49 64})");

        Ok(())
    }
}