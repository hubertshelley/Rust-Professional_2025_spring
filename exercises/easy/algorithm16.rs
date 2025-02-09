/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};
use std::mem;

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement the logic to rotate the matrix 90 degrees in place
    // 顺时针旋转90度 实现如下效果
    // 1 2 3       7 4 1
    // 4 5 6  -->  8 5 2
    // 7 8 9       9 6 3
    // 可将矩阵转置后再左右翻转实现
    // 1 2 3  转置  1 4 7  翻转  7 4 1
    // 4 5 6  -->  2 5 8  -->  8 5 2
    // 7 8 9       3 6 9       9 6 3
    let rows = matrix.len();
    let cols = matrix[0].len();
    let n = rows as i32 - cols as i32;
    let tran_matrix_rows = {
        if rows > cols {
            cols
        } else {
            rows
        }
    };
    // 方阵转置
    for i in 0..tran_matrix_rows {
        for j in i + 1..tran_matrix_rows {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
            // (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
        }
    }
    if n > 0 {
        // 再将右下角的元素转置
        for i in 0..n as usize {
            for j in 0..cols {
                let temp = matrix[i + cols][j];
                matrix[j].push(temp)
            }
        }
        for _ in 0..n {
            matrix.pop();
        }
    } else {
        // 再将左下角的元素转置
        for i in 0..-n as usize {
            matrix.push(Vec::new());
            for j in 0..rows {
                let temp = matrix[j].pop().unwrap();
                matrix[rows + i].push(temp);
            }
        }
    }
    // 左右翻转
    for r in matrix.iter_mut() {
        r.reverse();
    }
}

fn print_m(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![vec![1]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![1],]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_5() {
        // 1 2 3       4 1
        // 4 5 6  -->  5 2
        //             6 3
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![4, 1], vec![5, 2], vec![6, 3]]);
    }
}
