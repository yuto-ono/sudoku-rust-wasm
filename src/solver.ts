export { solve } from "../wasm/pkg"

export const SolveStatus = {
  success: 0,
  invalidLength: 1,
  noEmpty: 2,
  duplicated: 3,
  unsolvable: 4,
  outOfRange: 5,
} as const
