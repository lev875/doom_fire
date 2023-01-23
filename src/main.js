const palette = [
  [7, 7, 7, 255],
  [31, 7, 7, 255],
  [47, 15, 7, 255],
  [71, 15, 7, 255],
  [87, 23, 7, 255],
  [103, 31, 7, 255],
  [119, 31, 7, 255],
  [143, 39, 7, 255],
  [159, 47, 7, 255],
  [175, 63, 7, 255],
  [191, 71, 7, 255],
  [199, 71, 7, 255],
  [223, 79, 7, 255],
  [223, 87, 7, 255],
  [223, 87, 7, 255],
  [215, 95, 7, 255],
  [215, 103, 15, 255],
  [207, 111, 15, 255],
  [207, 119, 15, 255],
  [207, 127, 15, 255],
  [207, 135, 23, 255],
  [199, 135, 23, 255],
  [199, 143, 23, 255],
  [199, 151, 31, 255],
  [191, 159, 31, 255],
  [191, 159, 31, 255],
  [191, 167, 39, 255],
  [191, 167, 39, 255],
  [191, 175, 47, 255],
  [183, 175, 47, 255],
  [183, 183, 47, 255],
  [183, 183, 55, 255],
  [207, 207, 111, 255],
  [223, 223, 159, 255],
  [239, 239, 199, 255],
  [255, 255, 255, 255]
]

const cycle = arr => n => (
  function* () {
    let i = 0
    while (true) {
      if (i >= n) return
      yield arr[i++ % arr.length]
    }
  }
)()

const canvas = document.getElementById("canvas")

const width   = canvas.width
const height  = canvas.height

const context = canvas.getContext("2d")

const setPixel = array => ([r, g, b, a]) => i => {
  const n = i * 4
  array[n]      = r
  array[n + 1]  = g
  array[n + 2]  = b
  array[n + 3]  = a
  return array
}

const bufferToCanvas = array => buffer =>
  buffer
    .map(n => palette[n])
    .forEach( (p,i) => setPixel(array)(p)(i) )

const buffer = new Array(width * height)
buffer.fill(0, 0, width * (height - 1))
buffer.fill(35, width * (height - 1))

const spread = (buffer, height, width) => {
  for (let i = height - 2; i >= 0; i--)
    for (let j = 0; j < width - 1; j++) {
      const prev = buffer[(i + 1) * width + j]
      const random = Math.round(Math.random() * 3) & 3
      const wind = Math.round(Math.random() * 2) - 1
      let next = prev - 1 - (random & 1)
      if (next < 0)
        next = 0
      if (next >= palette.length)
        next = palette.length - 1
      buffer[i * width + j - wind] = next
    }
}

const array = new Uint8ClampedArray(width * height * 4)

const frameDiff = 1000 / 60

const draw = previous => current => {
  if (current - previous < frameDiff * 5) {
    requestAnimationFrame(draw(previous))
  } else {
    spread(buffer, height, width)
    bufferToCanvas(array)(buffer)
    context.putImageData(new ImageData(array, width), 0, 0)
    requestAnimationFrame(draw(current))
  }
}

requestAnimationFrame(draw(0))
