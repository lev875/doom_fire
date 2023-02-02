import init, { Fire } from "./wasm/doom_fire"
import wasm from "url:./wasm/doom_fire_bg.wasm"
import { debounced } from "./util"

const getWidth = height => {
  let aspectRatio = window.innerWidth / window.innerHeight
  return Math.ceil(height * aspectRatio)
}

const height  = 150
const width   = getWidth(height)

const canvas  = document.getElementById("canvas")
canvas.width  = width
canvas.height = height
const context = canvas.getContext("2d")

init(wasm).then(() => {

  let fire = new Fire(width, height, true)

  const frameRate = 10
  const frameDiff = 1000 / frameRate

  const draw = previous => current => {
    if (current - previous < frameDiff) {
      requestAnimationFrame(draw(previous))
    } else {
      fire.tick()
      context.putImageData(new ImageData(fire.get_output(), fire.width), 0, 0)
      requestAnimationFrame(draw(current))
    }
  }

  requestAnimationFrame(draw(0))

  canvas.addEventListener(
    "click",
    () => fire.is_running ? fire.stop() : fire.start()
  )

  const resize = () => {
    let newWidth = getWidth(height)
    canvas.width = newWidth
    fire = new Fire(newWidth, height, true)
  }

  window.addEventListener("resize", debounced(100)(resize))

})
