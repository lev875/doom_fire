export const getRandom = n => Math.round(Math.random() * n)

export const getRandomByte = () => {
  const array = new Uint8Array(1)
  crypto.getRandomValues(array)
  return array[0]
}

export const cycle = arr => n => (
  function* () {
    let i = 0
    while (true) {
      if (i >= n) return
      yield arr[i++ % arr.length]
    }
  }
)()
