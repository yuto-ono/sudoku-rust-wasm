import { writable, type Readable } from "svelte/store"
import { CELL_NUMBER } from "./constants"

export type CellType = {
  num: number
  inputed: boolean
  element?: HTMLInputElement
}

export type CellsStore = Readable<CellType[]> & {
  updateCell: (index: number, payload: Partial<CellType>) => void
  setArray: (nums: number[]) => void
  setSolvedArray: (nums: number[]) => void
  reset: () => void
  undo: () => void
}

export const createCells = (): CellsStore => {
  const generateInitalCells = (): CellType[] => {
    return [...Array(CELL_NUMBER)].map(() => ({
      num: 0,
      inputed: false,
    }))
  }
  const { subscribe, set, update } = writable(generateInitalCells())

  const updateCell = (index: number, payload: Partial<CellType>) => {
    if (index < CELL_NUMBER) {
      update((cells) => {
        const cell = cells[index]
        cell.num = payload.num ?? cell.num
        cell.inputed = payload.inputed ?? cell.inputed
        cell.element = payload.element ?? cell.element
        return cells
      })
    }
  }

  const setArray = (nums: number[]) => {
    if (nums.length === CELL_NUMBER) {
      set(
        nums.map((num) => ({
          num,
          inputed: num !== 0,
        }))
      )
    }
  }

  const setSolvedArray = (nums: number[]) => {
    if (nums.length === CELL_NUMBER) {
      update((cells) => {
        cells.forEach((cell, i) => {
          cell.num = nums[i] ?? 0
        })
        return cells
      })
    }
  }

  const undo = () => {
    update((cells) => {
      cells.forEach((cell) => {
        if (!cell.inputed) {
          cell.num = 0
        }
      })
      return cells
    })
  }

  return {
    subscribe,
    updateCell,
    setArray,
    setSolvedArray,
    reset: () => set(generateInitalCells()),
    undo,
  }
}
