import { JSX } from "preact";
import { useEffect, useState } from "preact/hooks";

import { instantiate, latin_to_greek } from "../static/rs_lib.generated.js";

export default function Conv() {
  const [wasmReady, setWasmReady] = useState(false);
  const [input, setInput] = useState("");

  useEffect(() => {
    instantiate({ url: new URL("/rs_lib_bg.wasm", location.origin) }).then(
      () => {
        setWasmReady(true);
      },
    );
  }, []);

  const greek = wasmReady ? latin_to_greek(input) : "";

  const onInput = (e: JSX.TargetedEvent<HTMLInputElement, Event>) => {
    setInput(e.currentTarget.value);
  };

  return (
    <div>
      <input
        value={input}
        onInput={onInput}
        placeholder="a)/nqrwpo/s e)stin."
      />

      <div class="greek">
        {greek}
      </div>
    </div>
  );
}
