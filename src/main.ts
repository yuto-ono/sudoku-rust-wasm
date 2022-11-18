import App from "./App.svelte"
import "./globals.scss"
import { greet } from "../wasm/pkg"

greet()

const el = document.getElementById("app")

if (el != null) {
  new App({ target: el })
}
