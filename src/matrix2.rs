use std::env::var;
use std::ops::{Add, AddAssign, Mul};
use std::sync::mpsc;
use std::thread;
use oneshot::Receiver;
use crate::vector::{dot_product, Vector};

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    pub idx: usize,
    pub row: Vector<T>,
    pub col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    val: T,
}
impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct Msg<T> {
    input:MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}
const NUM_PRODUCER: usize = 4;

// 矩阵乘法
pub fn multiply<T>(a: &crate::matrix::Matrix<T>, b: &crate::matrix::Matrix<T>) -> anyhow::Result<crate::matrix::Matrix<T>>
    where T: Copy + Default + Add<Output=T> + AddAssign + Mul<Output=T> + Send + 'static,
{
    if a.row != b.col {
        return Err(anyhow::anyhow!("Matrix multiply error: a.col != b.row"));
    }

    // 将每次的点集作为一次计算。收集好行的vec和列的vec发送计算
    // 1.构建sender
    let senders = (0..NUM_PRODUCER)
        .map(|_| {

            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx  {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput{
                        idx:msg.input.idx,
                        val:value,
                    }) {
                        eprintln!("Send error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            return tx
        }).collect::<Vec<_>>();


    let matrix_len = a.row * b.col;
    let mut receiver:Vec<Receiver<MsgOutput<T>>> = Vec::with_capacity(matrix_len);

    for i in (0..a.row) {
        for j in 0..b.col {
            // 计算要点乘的每行和每列
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx,rx) = oneshot::channel();
            let msg = Msg::new(input,tx);
            // 发送到通道线程中
            if let Err(e) = senders[idx%NUM_PRODUCER].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            // 将返回的结果通道存储
            receiver.push( rx)
        }
    }
    // 计算后的平铺矩阵
    let mut data = vec![T::default(); matrix_len];
    for rx in receiver {
        let output = rx.recv()?;
        data[output.idx] = output.val
    }


    Ok(crate::matrix::Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> anyhow::Result<()> {
        let a = crate::matrix::Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = crate::matrix::Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {22 28, 49 64})");

        Ok(())
    }
}