import React, {useEffect} from "react";
import ReactDOM from "react-dom";
import { greet } from 'bdk-wasm'

const App = () => {
  useEffect(() => {
    greet()
  }, [])
  return (
    <h1>My React and TypeScript App!</h1>
  )
};

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
