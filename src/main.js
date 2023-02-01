import init, { Fire } from "./wasm/doom_fire"
import wasm from "url:./wasm/doom_fire_bg.wasm"

const height  = 150
const width   = height * 4

const canvas  = document.getElementById("canvas")
canvas.width  = width
canvas.height = height
const context = canvas.getContext("2d")

init(wasm).then(() => {

  const fire = new Fire(width, height)
  // fire.stop()

  const frameDiff = 1000 / 60

  const draw = previous => current => {
    if (current - previous < frameDiff * 6) {
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
})
