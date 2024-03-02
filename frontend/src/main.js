import './app.css'
import App from './App.svelte'

let name = "Henning";

const app = new App({
  target: document.getElementById('app'),
})

export default app
