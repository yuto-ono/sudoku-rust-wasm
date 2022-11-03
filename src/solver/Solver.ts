import Cell from "./Cell"
import EmptyList from "./EmptyList"

const CELL_NUMBER = 81

/**
 * 数独の盤面を管理し、解くクラス
 */
class Solver {
  isValid = true
  private cells: Cell[] = []
  private emptyList: EmptyList

  constructor(data: number[]) {
    this.emptyList = new EmptyList()

    if (data.length !== CELL_NUMBER) {
      this.isValid = false
      return
    }

    this.cells = data.map((num, i) => new Cell(i, num))

    // セルの初期化と空きマスリストの作成
    for (const cell of this.cells) {
      if (!cell.init(this.cells)) {
        this.isValid = false
        return
      }
      if (cell.value === 0) {
        this.emptyList.push(cell)
      }
    }
  }

  /**
   * 数独を解く（再帰呼び出し）
   * 解けたら true, 解けなかったら false を返す
   * 改良バックトラック
   */
  solve(): boolean {
    if (this.emptyList.length === 0) {
      return true // 解けた！
    }

    // 空きマスのうち、最も候補が少ないものを選ぶ
    const cell = this.emptyList.popMin()

    // 候補に上がっている数字を入れてみる
    for (let i = 1; i <= 9; i++) {
      if (cell.setValue(i)) {
        if (this.solve()) {
          return true
        }
        cell.resetValue()
      }
    }

    // 解けなかったので、もとに戻してやり直し
    this.emptyList.restore(cell)
    return false
  }

  /**
   * 現在の盤面のデータを取得
   */
  getNumberArray(): number[] {
    return this.cells.map((cell) => cell.value)
  }
}

export default Solver
