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

export const sample1 = toNumberArray(sample1_text)

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

export const sample2 = toNumberArray(sample2_text)
