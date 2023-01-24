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

const bufferToCanvas = buffer => array =>
  buffer
    .map(n => palette[n])
    .forEach( (p,i) => setPixel(array)(p)(i) )

const spread = (buffer, height, width) => {
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

const height  = 150
const width   = height * 4

const canvas  = document.getElementById("canvas")
canvas.width  = width
canvas.height = height
const context = canvas.getContext("2d")

const buffer = new Array(width * height)
buffer.fill(0, 0, width * (height - 1))
buffer.fill(35, width * (height - 1))

const array = new Uint8ClampedArray(width * height * 4)

const frameDiff = 1000 / 60

const draw = previous => current => {
  if (current - previous < frameDiff * 6) {
    requestAnimationFrame(draw(previous))
  } else {
    spread(buffer, height, width)
    bufferToCanvas(buffer)(array)
    context.putImageData(new ImageData(array, width), 0, 0)
    requestAnimationFrame(draw(current))
  }
}

requestAnimationFrame(draw(0))

let isOn = true
const turnOn = () => {
  isOn = true
  buffer.fill(35, width * (height - 1))
}

const turnOff = () => {
  isOn = false
  buffer.fill(0, width * (height - 1))
}

canvas.addEventListener(
  "click",
  () => {
    if (isOn) turnOff()
    else turnOn()
  }
)
