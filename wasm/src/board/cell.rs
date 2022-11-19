use super::related_ids::RELATED_IDS;

// 1-9 すべて候補に上がっている状態の candidates
// ビットで候補を管理している（2進数で 1111111110）
const DEFAULT_CANDIDATES: u32 = 0x3fe;

/**
 * それぞれのマスを表す構造体
 * そのマスの候補のリストとしても機能する
 */
pub struct Cell {
    pub num: u32,
    pub pos: usize,
    pub candidates: u32,
}

impl Cell {
    /**
     * 新しいセルを作成
     */
    pub fn new(pos: usize, num: u32) -> Cell {
        Cell {
            num,
            pos,
            candidates: 0,
        }
    }

    /**
     * 初期化
     * 重複を発見したら false を返す
     */
    pub fn init(&mut self, num_array: &[u32]) -> bool {
        if self.num != 0 {
            // 重複チェック
            for &id in RELATED_IDS[self.pos].iter() {
                if self.num == num_array[id] {
                    return false;
                }
            }
        } else {
            // 候補リストの作成
            self.candidates = DEFAULT_CANDIDATES;
            for &id in RELATED_IDS[self.pos].iter() {
                let related_cell_num: u32 = num_array[id];
                if related_cell_num != 0 {
                    let mask = 1 << related_cell_num;
                    if (self.candidates & mask) != 0 {
                        self.candidates ^= mask;
                    }
                }
            }
        }
        true
    }
}
