import React from "react";
import "./App.css";
import { greet } from "bdk-wasm";
import { DescriptorInput } from "./components/DescriptorInput";
import { Button } from "./components/Button";

function App() {
  return (
    <div className="App max-w-8xl mx-auto px-4 sm:px-6 md:px-8 py-8 ">
      <div className="grid grid-cols-4 gap-4">
        <div className="col-span-4">
          <DescriptorInput />
        </div>
        <div className="col-span-2 col-start-4">
          <Button onClick={greet}>Test a Wasm function</Button>
        </div>
      </div>
    </div>
  );
}

export default App;
