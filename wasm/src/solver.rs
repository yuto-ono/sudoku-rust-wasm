use super::constants::BOARD_NUM;
mod masks;
use masks::MASKS;

const NO_EMPTY: u128 = (1 << BOARD_NUM) - 1;

#[derive(Debug, PartialEq)]
pub enum SolveStatus {
    Success,
    InvalidLength,
    NoEmpty,
    Duplicated,
    Unsolvable,
}

pub fn solve(num_array: &mut [u32]) -> SolveStatus {
    if num_array.len() != BOARD_NUM {
        return SolveStatus::InvalidLength; // 配列の長さが違う
    }

    let mut board = [0u128; 10];

    if !set_num_array(&mut board, num_array) {
        return SolveStatus::Duplicated; // 重複がある
    }
    if board[0] == NO_EMPTY {
        return SolveStatus::NoEmpty; // 空きマスがない
    }
    if !solve_recursive(&mut board, 0, 1) {
        return SolveStatus::Unsolvable; // 解くことができない
    }
    output_array(&board, num_array);
    SolveStatus::Success // 解けた
}

fn set_num(board: &mut [u128], pos: usize, bit: u128, num: usize) -> bool {
    if (board[num] & MASKS[pos]) != 0 {
        return false;
    }
    board[0] |= bit;
    board[num] |= bit;
    true
}

fn set_num_array(board: &mut [u128], num_array: &mut [u32]) -> bool {
    for (i, &num) in num_array.iter().enumerate() {
        if num != 0 {
            let bit: u128 = 1 << i;
            if !set_num(board, i, bit, num as usize) {
                return false;
            }
        }
    }
    true
}

fn solve_recursive(board: &mut [u128], mut pos: usize, mut bit: u128) -> bool {
    loop {
        if pos == BOARD_NUM {
            return true;
        }
        if (board[0] & bit) == 0 {
            break;
        }
        pos += 1;
        bit <<= 1;
    }

    for i in 1..10 {
        if set_num(board, pos, bit, i) {
            if solve_recursive(board, pos + 1, bit << 1) {
                return true;
            }
            board[0] ^= bit;
            board[i] ^= bit;
        }
    }

    false
}

fn output_array(board: &[u128], num_array: &mut [u32]) {
    for i in 0..BOARD_NUM {
        let bit = 1 << i;
        num_array[i] = 0;
        for num in 1..10 {
            if (board[num] & bit) != 0 {
                num_array[i] = num as u32;
                break;
            }
        }
    }
}
