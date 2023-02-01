import { palette } from "./palette"
import { getRandomByte } from "./util"

const setPixel = array => ([r, g, b, a]) => i => {
  const n = i * 4
  array[n]      = r
  array[n + 1]  = g
  array[n + 2]  = b
  array[n + 3]  = a
  return array
}

export const bufferToCanvas = buffer => array =>
  buffer
    .map(n => palette[n])
    .forEach( (p,i) => setPixel(array)(p)(i) )

export const spread = (buffer, height, width) => {
  for (let i = height - 2; i >= 0; i--)
    for (let j = 0; j < width - 1; j++) {
      const prev    = buffer[(i + 1) * width + j]
      const random  = getRandomByte() & 0b11
      const wind    = (random & 0b10) - 1
      let next      = prev - (random & 1)
      if (next < 0)
        next = 0
      if (next >= palette.length)
        next = palette.length - 1
      buffer[i * width + j - wind] = next
    }
}
