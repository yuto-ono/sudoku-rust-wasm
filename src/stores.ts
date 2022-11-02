import { writable } from "svelte/store"

export type CellType = {
  num: number
  inputed: boolean
  element?: HTMLInputElement
}

export const CELL_NUMBER = 81

export const cells = writable<CellType[]>(
  [...Array(CELL_NUMBER)].map(() => ({
    num: 0,
    inputed: false,
  }))
)
