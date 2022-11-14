import { describe, expect, it } from "@jest/globals"
import Solver from "./Solver"

describe("Solver", () => {
  it("sample 1", () => {
    const solver = new Solver(sample1)
    expect(solver.solve()).toBe(true)
    expect(solver.getNumberArray()).toStrictEqual(answer1)
  })
  it("sample 2", () => {
    const solver = new Solver(sample2)
    expect(solver.solve()).toBe(true)
    expect(solver.getNumberArray()).toStrictEqual(answer2)
  })
})

const toNumberArray = (text: string): number[] => {
  const result: number[] = []
  text.split("").forEach((char) => {
    if (/^\d$/.test(char)) {
      result.push(Number(char))
    }
  })
  return result
}

const sample1_text = `
800000000
003600000
070090200
050007000
000045700
000100030
001000068
008500010
090000400`

const answer1_text = `
812753649
943682175
675491283
154237896
369845721
287169534
521974368
438526917
796318452`

const sample1 = toNumberArray(sample1_text)
const answer1 = toNumberArray(answer1_text)

const sample2_text = `
005300000
800000020
070010500
400005300
010070006
003200080
060500009
004000030
000009700`

const answer2_text = `
145327698
839654127
672918543
496185372
218473956
753296481
367542819
984761235
521839764`

const sample2 = toNumberArray(sample2_text)
const answer2 = toNumberArray(answer2_text)
