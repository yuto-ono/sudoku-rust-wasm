use wasm_bindgen::prelude::*;

pub const CHUNK_NUM: usize = 3;
pub const COL_NUM: usize = 9;
pub const BOARD_NUM: usize = COL_NUM * COL_NUM;

/**
 * ビットボードの配列の長さ
 */
const BITBOARD_LEN: usize = COL_NUM + 1;

/**
 * 各マスのビットマスク
 */
const MASKS: [u128; BOARD_NUM] = {
    let mut masks = [0u128; BOARD_NUM];
    let mut i = 0;
    while i < BOARD_NUM {
        let row = i / COL_NUM;
        let col = i % COL_NUM;
        let area33top = (row / CHUNK_NUM) * CHUNK_NUM;
        let area33left = (col / CHUNK_NUM) * CHUNK_NUM;
        let mut j = 0;
        while j < COL_NUM {
            let row33 = area33top + (j / CHUNK_NUM);
            let col33 = area33left + (j % CHUNK_NUM);
            masks[i] |= 1 << (row * COL_NUM + j);
            masks[i] |= 1 << (j * COL_NUM + col);
            masks[i] |= 1 << (row33 * COL_NUM + col33);
            j += 1
        }
        i += 1;
    }
    masks
};

pub enum SolveStatus {
    Success,       // 解けた
    InvalidLength, // 配列の長さが違う
    NoEmpty,       // 空きマスがない
    Duplicated,    // 重複がある
    Unsolvable,    // 解けない
}

/**
 * 数独を解く
 */
#[wasm_bindgen]
pub fn solve(num_array: &mut [u32]) -> u32 {
    if num_array.len() != BOARD_NUM {
        return SolveStatus::InvalidLength as u32; // 配列の長さが違う
    }

    let mut board = [0u128; BITBOARD_LEN];

    if !num_array_to_bitboard(&mut board, num_array) {
        return SolveStatus::Duplicated as u32; // 重複がある
    }
    if board[0] == 0 {
        return SolveStatus::NoEmpty as u32; // 空きマスがない
    }
    if !solve_recursive(&mut board, 0, 1) {
        return SolveStatus::Unsolvable as u32; // 解くことができない
    }
    output_array(&board, num_array);
    SolveStatus::Success as u32 // 解けた
}

/**
 * 配列からビットボードを生成
 * 重複があれば false を返す
 */
fn num_array_to_bitboard(board: &mut [u128], num_array: &mut [u32]) -> bool {
    for (i, &num) in num_array.iter().enumerate() {
        if num != 0 && (board[num as usize] & MASKS[i]) != 0 {
            return false;
        }
        board[num as usize] |= 1 << i;
    }
    true
}

/**
 * solve の内部処理（再帰関数）
 * 解けたら true, 解けなかったら false を返す
 */
fn solve_recursive(board: &mut [u128], mut pos: usize, mut bit: u128) -> bool {
    // 空きマスを探す
    loop {
        if pos == BOARD_NUM {
            return true;
        }
        if (board[0] & bit) != 0 {
            break;
        }
        pos += 1;
        bit <<= 1;
    }

    // 数字を入れてみる
    for i in 1..BITBOARD_LEN {
        if (board[i] & MASKS[pos]) == 0 {
            board[i] |= bit;
            // 再帰的に探索
            if solve_recursive(board, pos + 1, bit << 1) {
                return true;
            }
            board[i] ^= bit;
        }
    }

    false
}

/**
 * ビットボードを配列に出力
 */
fn output_array(board: &[u128], num_array: &mut [u32]) {
    for i in 0..BOARD_NUM {
        let bit = 1 << i;
        for num in 1..BITBOARD_LEN {
            if (board[num] & bit) != 0 {
                num_array[i] = num as u32;
                break;
            }
        }
    }
}
