import { useEffect } from "react";
import "./App.css";
import init from "../pkg/wgpu_boids.js";

function App() {
  useEffect(() => {
    init().then(() => {
      console.log("WASM initialized");
    });
  });

  return <div id="wasm-canvas"></div>;
}

export default App;
