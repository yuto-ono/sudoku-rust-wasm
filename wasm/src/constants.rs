const CHUNK_NUM: usize = 3;
const COL_NUM: usize = 9;

pub const BOARD_NUM: usize = COL_NUM * COL_NUM;

// 各マスのビットマスク
pub const MASKS: [u128; BOARD_NUM] = {
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
