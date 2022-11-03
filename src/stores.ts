import { derived, writable } from "svelte/store"
import { createCells } from "./cellsStore"

export const CELL_NUMBER = 81

export const cells = createCells()

export const cellsIsEmpty = derived(cells, ($cells) => {
  return !$cells.some((cell) => cell.num !== 0)
})

export const solved = writable(false)

export const time = writable<number>()
