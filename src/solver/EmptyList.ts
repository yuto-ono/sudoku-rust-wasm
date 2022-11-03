import Cell from "./Cell"

// 最小の要素を探すための初期値
const MIN_LENGTH_INITIAL = 10

/**
 * 空きマスリスト
 * 双方向連結リストによる実装
 */
class EmptyList {
  length = 0
  private head: Cell

  constructor() {
    this.head = new Cell(0, 0)
  }

  /**
   * 要素を最後尾に追加
   */
  push(cell: Cell): void {
    cell.prev = this.head.prev
    cell.next = this.head
    cell.prev.next = cell
    this.head.prev = cell
    this.length++
  }

  /**
   * リストの中の最小の要素を取り出す
   */
  popMin(): Cell {
    let cell = this.head
    let selectedCell = this.head
    let minLength = MIN_LENGTH_INITIAL

    while ((cell = cell.next) !== this.head) {
      if (cell.length === 1) {
        selectedCell = cell
        break
      }
      if (cell.length < minLength) {
        minLength = cell.length
        selectedCell = cell
      }
    }

    selectedCell.prev.next = selectedCell.next
    selectedCell.next.prev = selectedCell.prev
    this.length--
    return selectedCell
  }

  /**
   * 要素を復活
   */
  restore(cell: Cell): void {
    cell.prev.next = cell
    cell.next.prev = cell
    this.length++
  }
}

export default EmptyList
