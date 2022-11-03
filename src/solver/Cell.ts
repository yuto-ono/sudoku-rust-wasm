/**
 * それぞれのマスを表すクラス
 * そのマスの候補のリストとしても機能する
 * 連結リストのアイテムとしても機能する
 */
class Cell {
  pos: number
  value: number
  length: number
  candidates: number
  prev: Cell
  next: Cell
  relatedCells: Cell[] = []
  changedCells: Cell[] = []

  constructor(pos: number, value: number) {
    this.pos = pos
    this.value = value
    this.prev = this
    this.next = this

    if (value !== 0) {
      this.length = 0
      this.candidates = 0
    } else {
      this.length = 9
      this.candidates = 0x3fe
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
      this.setCandidates()
    }

    return true
  }

  /**
   * 数字をセット 関連セルの候補も更新
   * 矛盾が生じたら false を返す
   */
  setValue(value: number): boolean {
    const mask = 1 << value
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
  addRelatedCell(cells: Cell[], row: number, col: number): void {
    const pos = row * 9 + col
    if (pos != this.pos) {
      const cell = cells[pos]
      if (!this.relatedCells.includes(cell)) {
        this.relatedCells.push(cell)
      }
    }
  }

  /**
   * 候補リストをセット
   */
  setCandidates(): void {
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
}

export default Cell
