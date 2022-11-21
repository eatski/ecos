import './style.css'
import init,{calculate} from '../pkg'

await init()
const canvas = document.querySelector<HTMLCanvasElement>('#canvas')
if (canvas) {
  const ctx = canvas.getContext('2d')
  if (ctx) {
    let state = {
      points: [
        {x: 100,y: 100},
      ]
    }
    setInterval(() => {
      state = calculate(state)
      ctx.clearRect(0, 0, canvas.width, canvas.height)
      state.points.forEach(({x,y}) => {
        ctx.fillRect(x, y, 1, 1)
      })
    },1)
    ctx.fillStyle = 'red'
    ctx.fillRect(0, 0, 1, 1)
  }
}