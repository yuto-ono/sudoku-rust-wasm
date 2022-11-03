import { writable } from "svelte/store"
import { createCells } from "./cellsStore"

export const cells = createCells()

export const solved = writable(false)

export const time = writable<number>()
