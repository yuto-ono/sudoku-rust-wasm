//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use sudoku_wasm::constants::BOARD_NUM;
use sudoku_wasm::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn sample1() {
    let mut num_array = flatten(&sample1_array);
    let answer_array = flatten(&answer1_array);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Success as u32);
    assert_eq!(num_array, answer_array);
}

#[wasm_bindgen_test]
fn sample2() {
    let mut num_array = flatten(&sample2_array);
    let answer_array = flatten(&answer2_array);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Success as u32);
    assert_eq!(num_array, answer_array);
}

#[wasm_bindgen_test]
fn duplicated() {
    let mut num_array = flatten(&duplicated_array);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Duplicated as u32);
}

#[wasm_bindgen_test]
fn unsolvable() {
    let mut num_array = flatten(&unsolvable_array);
    let solve_status = solve(&mut num_array);
    assert_eq!(solve_status, SolveStatus::Unsolvable as u32);
}

fn flatten(array2d: &[[u32; 9]]) -> [u32; BOARD_NUM] {
    let mut num_array = [0; BOARD_NUM];
    for i in 0..BOARD_NUM {
        num_array[i] = array2d[i / 9][i % 9];
    }
    num_array
}

const sample1_array: [[u32; 9]; 9] = [
    [8, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 3, 6, 0, 0, 0, 0, 0],
    [0, 7, 0, 0, 9, 0, 2, 0, 0],
    [0, 5, 0, 0, 0, 7, 0, 0, 0],
    [0, 0, 0, 0, 4, 5, 7, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 3, 0],
    [0, 0, 1, 0, 0, 0, 0, 6, 8],
    [0, 0, 8, 5, 0, 0, 0, 1, 0],
    [0, 9, 0, 0, 0, 0, 4, 0, 0],
];

const answer1_array: [[u32; 9]; 9] = [
    [8, 1, 2, 7, 5, 3, 6, 4, 9],
    [9, 4, 3, 6, 8, 2, 1, 7, 5],
    [6, 7, 5, 4, 9, 1, 2, 8, 3],
    [1, 5, 4, 2, 3, 7, 8, 9, 6],
    [3, 6, 9, 8, 4, 5, 7, 2, 1],
    [2, 8, 7, 1, 6, 9, 5, 3, 4],
    [5, 2, 1, 9, 7, 4, 3, 6, 8],
    [4, 3, 8, 5, 2, 6, 9, 1, 7],
    [7, 9, 6, 3, 1, 8, 4, 5, 2],
];

const sample2_array: [[u32; 9]; 9] = [
    [0, 0, 5, 3, 0, 0, 0, 0, 0],
    [8, 0, 0, 0, 0, 0, 0, 2, 0],
    [0, 7, 0, 0, 1, 0, 5, 0, 0],
    [4, 0, 0, 0, 0, 5, 3, 0, 0],
    [0, 1, 0, 0, 7, 0, 0, 0, 6],
    [0, 0, 3, 2, 0, 0, 0, 8, 0],
    [0, 6, 0, 5, 0, 0, 0, 0, 9],
    [0, 0, 4, 0, 0, 0, 0, 3, 0],
    [0, 0, 0, 0, 0, 9, 7, 0, 0],
];

const answer2_array: [[u32; 9]; 9] = [
    [1, 4, 5, 3, 2, 7, 6, 9, 8],
    [8, 3, 9, 6, 5, 4, 1, 2, 7],
    [6, 7, 2, 9, 1, 8, 5, 4, 3],
    [4, 9, 6, 1, 8, 5, 3, 7, 2],
    [2, 1, 8, 4, 7, 3, 9, 5, 6],
    [7, 5, 3, 2, 9, 6, 4, 8, 1],
    [3, 6, 7, 5, 4, 2, 8, 1, 9],
    [9, 8, 4, 7, 6, 1, 2, 3, 5],
    [5, 2, 1, 8, 3, 9, 7, 6, 4],
];

const duplicated_array: [[u32; 9]; 9] = [
    [8, 8, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 3, 6, 0, 0, 0, 0, 0],
    [0, 7, 0, 0, 9, 0, 2, 0, 0],
    [0, 5, 0, 0, 0, 7, 0, 0, 0],
    [0, 0, 0, 0, 4, 5, 7, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 3, 0],
    [0, 0, 1, 0, 0, 0, 0, 6, 8],
    [0, 0, 8, 5, 0, 0, 0, 1, 0],
    [0, 9, 0, 0, 0, 0, 4, 0, 0],
];

const unsolvable_array: [[u32; 9]; 9] = [
    [8, 2, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 3, 6, 0, 0, 0, 0, 0],
    [0, 7, 0, 0, 9, 0, 2, 0, 0],
    [0, 5, 0, 0, 0, 7, 0, 0, 0],
    [0, 0, 0, 0, 4, 5, 7, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 3, 0],
    [0, 0, 1, 0, 0, 0, 0, 6, 8],
    [0, 0, 8, 5, 0, 0, 0, 1, 0],
    [0, 9, 0, 0, 0, 0, 4, 0, 0],
];
