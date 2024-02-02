import { JSX } from "preact";
import { useEffect, useState } from "preact/hooks";

import { instantiate, latin_to_greek } from "../static/rs_lib.generated.js";

type InputProps = {
  value: string;
  onInput: (e: JSX.TargetedEvent<HTMLInputElement, Event>) => void;
};

const Input = ({ value, onInput }: InputProps) => {
  return (
    <div class="flex justify-center">
      <input
        class="rounded-lg ring ring-rose-300 flex-none w-3/5 h-10 px-3"
        value={value}
        onInput={onInput}
        placeholder="a)/nqrwpo/s e)stin."
      />
    </div>
  );
};

type OutputProps = {
  greek: string;
  placeholder: string;
};

const Output = ({ greek, placeholder }: OutputProps) => {
  if (greek.length === 0) {
    return (
      <div class="greek flex justify-center">
        <div class="text-balance overflow-x-hidden border-b border-rose-300 text-stone-500 flex-none w-3/5 px-3 pt-20">
          {placeholder}
        </div>
      </div>
    );
  } else {
    return (
      <div class="greek flex justify-center">
        <div class="text-balance overflow-x-hidden border-b border-rose-300 text-stone-700 flex-none w-3/5 px-3 pt-20">
          {greek}
        </div>
      </div>
    );
  }
};

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
  const greekph = wasmReady ? latin_to_greek("fai/nei e)nqa/de.") : "";

  const onInput = (e: JSX.TargetedEvent<HTMLInputElement, Event>) => {
    setInput(e.currentTarget.value);
  };

  return (
    <div class="container max-w-full">
      <div class="bg-rose-100 pt-32 pb-8">
        <Input value={input} onInput={onInput} />
      </div>

      <Output greek={greek} placeholder={greekph} />
    </div>
  );
}
