import init from '../pkg/ecos_bg.wasm?init'

const fn = () => init({}).then((instance) => {
  return instance.exports as typeof import('../pkg/ecos_bg.wasm');
})

export async function setupCounter(element: HTMLButtonElement) {
  let counter = 0
  const setCounter = (count: number) => {
    counter = count
    element.innerHTML = `count is ${counter}`
  }
  fn().then(({add}) => {
    element.addEventListener('click', () => setCounter(add(counter ,1)))
    setCounter(0)
  })
  
}
