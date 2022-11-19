mod cell;
use cell::Cell;
mod related_ids;
use super::constants::*;
use related_ids::RELATED_IDS;

pub struct Board {
    pub is_valid: bool,
    cells: Vec<Cell>,
    pub empty_len: u32,
}

impl Board {
    /**
     * Board のコンストラクタ
     */
    pub fn new(num_array: &[u32]) -> Board {
        let mut is_valid = true;
        let mut empty_len: u32 = 0;
        let mut cells: Vec<Cell> = Vec::with_capacity(BOARD_NUM);

        for (i, &num) in num_array.iter().enumerate() {
            let mut cell = Cell::new(i, num);
            if !cell.init(num_array) {
                is_valid = false;
                break;
            }
            if cell.num == 0 {
                empty_len += 1
            }
            cells.push(cell);
        }

        Board {
            cells,
            is_valid,
            empty_len,
        }
    }

    /**
     * 再帰呼び出しにより、数独を解く
     */
    pub fn solve(&mut self) -> bool {
        let mut changed_ids = [0; RELATED_LENGTH];
        let mut changed_length: usize = 0;
        let selected_i = match self.cells.iter().find(|cell| cell.num == 0) {
            Some(cell) => cell.pos,
            None => 0,
        };

        self.empty_len -= 1;

        // 候補に上がっている数字を入れてみる
        for i in 1..10 {
            let mask: u32 = 1 << i;
            if (self.cells[selected_i].candidates & mask) != 0 {
                let mut duplicated = false;
                self.cells[selected_i].num = i;

                // 関連セルの候補を更新
                for &id in RELATED_IDS[selected_i].iter() {
                    let cell = &mut self.cells[id];
                    if cell.num == 0 && (cell.candidates & mask) != 0 {
                        if cell.candidates == mask {
                            duplicated = true;
                            break;
                        }
                        cell.candidates ^= mask;
                        changed_ids[changed_length] = id;
                        changed_length += 1;
                    }
                }

                // 再帰呼び出し
                if !duplicated && (self.empty_len == 0 || self.solve()) {
                    return true; // 解けた！
                }

                // 変更したセルをもとに戻す
                for j in 0..changed_length {
                    self.cells[changed_ids[j]].candidates ^= mask;
                }
                changed_length = 0;
            }
        }

        // 解けなかったので、もとに戻してやり直し
        self.cells[selected_i].num = 0;
        self.empty_len += 1;
        false
    }

    /**
     * ソルブ結果を配列に書き出す
     */
    pub fn output_array(&self, num_array: &mut [u32]) {
        if num_array.len() == self.cells.len() {
            for (i, cell) in self.cells.iter().enumerate() {
                num_array[i] = cell.num
            }
        }
    }
}
