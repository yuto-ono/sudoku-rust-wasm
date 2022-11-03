// 1-9 すべて候補に上がっている状態の candidates
// ビットで候補を管理している（2進数で 1111111110）
const DEFAULT_CANDIDATES = 0x3fe

/**
 * それぞれのマスを表すクラス
 * そのマスの候補のリストとしても機能する
 * 連結リストのアイテムとしても機能する
 */
class Cell {
  value: number
  length = 0
  prev: Cell
  next: Cell
  private pos: number
  private candidates = 0
  private relatedCells: Cell[] = []
  private changedCells: Cell[] = []

  constructor(pos: number, value: number) {
    this.pos = pos
    this.value = value
    this.prev = this
    this.next = this

    if (value === 0) {
      this.length = 9
      this.candidates = DEFAULT_CANDIDATES
    }
  }

  /**
   * 初期化
   * セルのリストが完成したあとに呼ぶ想定
   * 矛盾を発見したら false を返す
   */
  init(cells: Cell[]): boolean {
    const row = Math.floor(this.pos / 9)
    const col = this.pos % 9
    const area33top = Math.floor(row / 3) * 3
    const area33left = Math.floor(col / 3) * 3

    // 関連セルの追加
    for (let i = 0; i < 9; i++) {
      const row33 = area33top + Math.floor(i / 3)
      const col33 = area33left + (i % 3)
      this.addRelatedCell(cells, row, i)
      this.addRelatedCell(cells, i, col)
      this.addRelatedCell(cells, row33, col33)
    }

    if (this.value !== 0) {
      // 重複チェック
      if (this.relatedCells.some((cell) => cell.value === this.value)) {
        return false
      }
    } else {
      // 候補リストの作成
      this.relatedCells.forEach((cell) => {
        if (cell.value !== 0) {
          const mask = 1 << cell.value
          if (this.candidates & mask) {
            this.candidates ^= mask
            this.length--
          }
        }
      })
    }

    return true
  }

  /**
   * 数字をセット 関連セルの候補も更新
   * 矛盾が生じたら false を返す
   */
  setValue(value: number): boolean {
    const mask = 1 << value
    if ((this.candidates & mask) === 0) {
      return false
    }
    this.value = value
    for (const cell of this.relatedCells) {
      if (cell.value === 0 && cell.candidates & mask) {
        if (cell.length === 1) {
          this.resetValue()
          return false
        }
        cell.candidates ^= mask
        cell.length--
        this.changedCells.push(cell)
      }
    }
    return true
  }

  /**
   * リセット（空きマスに戻す）
   * changedCells をもとに、関連セルの候補も元に戻す
   */
  resetValue(): void {
    const mask = 1 << this.value
    this.changedCells.forEach((cell) => {
      cell.candidates ^= mask
      cell.length++
    })
    this.changedCells.splice(0)
    this.value = 0
  }

  /**
   * 指定位置のセルを関連セルのリストに追加
   */
  private addRelatedCell(cells: Cell[], row: number, col: number): void {
    const pos = row * 9 + col
    if (pos !== this.pos) {
      const cell = cells[pos]
      if (!this.relatedCells.includes(cell)) {
        this.relatedCells.push(cell)
      }
    }
  }
}

export default Cell
