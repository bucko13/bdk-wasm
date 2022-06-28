import { extractPolicy } from "bdk-wasm";
import React, { useState } from "react";
import { Button } from "./Button";

export const DescriptorInput = () => {
  const [descriptor, setDescriptor] = useState("");
  const [policy, setPolicy] = useState("");
  const [error, setError] = useState("");

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setPolicy("");
    setError("");
    setDescriptor(event.target.value);
  };
  const parseDescriptor = () => {
    try {
      setPolicy(extractPolicy(descriptor));
    } catch (e: any) {
      setError(e.message);
    }
  };
  return (
    <div>
      <label htmlFor="enterDescriptor" className="form-label inline-block mb-2 text-gray-700">
        Enter Descriptor
      </label>
      <div className="grid grid-cols-4 gap-2">
        <div className="col-span-3">
          <form
            onSubmit={e => {
              e.preventDefault();
              parseDescriptor();
            }}
          >
            <input
              type="text"
              className="
            form-control
            block
            w-full
            px-3
            py-1.5
            text-base
            font-normal
            text-gray-700
            bg-white bg-clip-padding
            border border-solid border-gray-300
            rounded
            transition
            ease-in-out
            m-0
            focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none
          "
              id="enterDescriptor"
              placeholder="wsh(1, xpub..., xpub...)"
              onChange={handleChange}
            />
          </form>
        </div>
        <div className="col-span-1">
          <Button onClick={parseDescriptor} style={{ width: "100%" }}>
            Parse
          </Button>
        </div>
      </div>
      <div>{policy && <code>{policy}</code>}</div>
      {error.length ? (
        <div
          className="bg-red-100 rounded-lg py-5 px-6 mb-4 text-base text-red-700 mb-3"
          role="alert"
        >
          {error}
        </div>
      ) : null}
    </div>
  );
};
