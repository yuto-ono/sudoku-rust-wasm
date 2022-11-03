import Cell from "./Cell"

/**
 * 空きマスリスト
 * 双方向連結リストによる実装
 */
class EmptyList {
  head: Cell
  length = 0

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
    let minLength = 10

    while (cell.next !== this.head) {
      cell = cell.next
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
