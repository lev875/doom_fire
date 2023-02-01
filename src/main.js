import { bufferToCanvas, spread } from "./fire"

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
