import init,{add} from '../pkg'

export async function setupCounter(element: HTMLButtonElement) {
  let counter = 0
  const setCounter = (count: number) => {
    counter = count
    element.innerHTML = `count is ${counter}`
  }
  init().then(()=> {
    element.addEventListener('click', () => setCounter(add(counter ,1)))
    setCounter(0)
  })
  
}
