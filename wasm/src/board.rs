mod cell;
use cell::Cell;
mod related_ids;
use super::constants::RELATED_LENGTH;
use related_ids::RELATED_IDS;

pub struct Board {
    pub is_valid: bool,
    cells: Vec<Cell>,
    pub length: u32,
}

impl Board {
    /**
     * Board のコンストラクタ
     */
    pub fn new(num_array: &[u32]) -> Board {
        let mut is_valid = true;
        let mut length: u32 = 0;
        let mut cells: Vec<Cell> = num_array
            .iter()
            .enumerate()
            .map(|(i, &num)| Cell::new(i, num))
            .collect();

        // セルの初期化
        for cell in cells.iter_mut() {
            if cell.num == 0 {
                length += 1
            }
            if !cell.init(num_array) {
                is_valid = false;
                break;
            }
        }

        Board {
            cells,
            is_valid,
            length,
        }
    }

    /**
     * 再帰呼び出しにより、数独を解く
     */
    pub fn solve(&mut self) -> bool {
        let mut selected_i: usize = 0;
        let mut min_length: u32 = 10;
        let mut changed_ids: [usize; RELATED_LENGTH] = [0; RELATED_LENGTH];
        let mut changed_length: usize = 0;

        // 空きマスのうち、最も候補が少ないものを選ぶ
        for cell in self.cells.iter() {
            if cell.num == 0 {
                if cell.length == 1 {
                    selected_i = cell.pos;
                    break;
                }
                if cell.length < min_length {
                    min_length = cell.length;
                    selected_i = cell.pos;
                }
            }
        }

        self.length -= 1;

        // 候補に上がっている数字を入れてみる
        for i in 1..10 {
            let mask: u32 = 1 << i;
            if (self.cells[selected_i].candidates & mask) != 0 {
                let mut duplicated = false;
                self.cells[selected_i].num = i;
                for &id in RELATED_IDS[selected_i].iter() {
                    let cell = &mut self.cells[id];
                    if cell.num == 0 && (cell.candidates & mask) != 0 {
                        if cell.length == 1 {
                            duplicated = true;
                            break;
                        }
                        cell.remove_candidate(mask);
                        changed_ids[changed_length] = id;
                        changed_length += 1;
                    }
                }
                if !duplicated && (self.length == 0 || self.solve()) {
                    return true; // 解けた！
                }

                // 変更したセルをもとに戻す
                for j in 0..changed_length {
                    self.cells[changed_ids[j]].add_candidate(mask);
                }
                changed_length = 0;
            }
        }

        // 解けなかったので、もとに戻してやり直し
        self.cells[selected_i].num = 0;
        self.length += 1;
        false
    }

    pub fn output_array(&self, num_array: &mut [u32]) {
        for cell in self.cells.iter() {
            if cell.pos < num_array.len() {
                num_array[cell.pos] = cell.num
            }
        }
    }
}
