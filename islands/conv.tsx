import { JSX } from "preact";
import { useEffect, useState } from "preact/hooks";

import { instantiate, latin_to_greek } from "../static/rs_lib.generated.js";

const iliad = `mh=nin a)/eide qea\\ Phlhi"a/dew A)cilh=os
oy)lome/nhn, h(\\ myri/' A)caioi=s a)/lge' e)/qhke,
polla\\s d' i)fqi/moys jyca\\s A)/i"di proi"/ajen
h(rw/wn, ay)toy\\s de\\ e(lw/ria tey=ce ky/nessin
oi)wnoi=si/ te pa=si, Dio\\s d' e)telei/eto boylh/,
e)x oy(= dh\\ ta\\ prw=ta diasth/thn e)ri/sante
A)trei"/dhs te a)/nax a)ndrw=n kai\\ di=os A)cilley/s.`;

type DetailsInProps = {
  hidden: boolean;
  setInput: (input: string) => void;
};

const DetailsInner = ({ hidden, setInput }: DetailsInProps) => {
  if (hidden) {
    return <></>;
  }

  const fillIliad = () => {
    setInput(iliad);
  };

  return (
    <div class="flex justify-center mb-4">
      <div class="flex-none text-stone-700 text-balance w-2/3">
        <div>
          Converter from ASCII printable characters to Greek characters (in
          UTF-8). It follows the convention of &#32;
          <a
            href="http://www.perseus.tufts.edu/hopper/search"
            target="_blank"
            class="underline decoration-2 decoration-rose-500 hover:text-zinc-500 hover:decoration-pink-500"
          >
            Perseus Digital Library
          </a>, except that:
          <ul class="list-disc list-inside ml-2">
            <li>
              <span class="greek">ξ</span> (xi) = x
            </li>
            <li>
              <span class="greek">υ</span> (ypsilon) = y
            </li>
            <li>
              <span class="greek">χ</span> (chi) = c
            </li>
            <li>
              <span class="greek">ψ</span> (psi) = j
            </li>
            <li>
              <span class="greek">ϊ, ϋ</span>{" "}
              (diaereses) = " (double quotation mark)
            </li>
            <li>
              <span class="greek">᾽</span> (koronis) = ' (single quotation mark)
            </li>
            <li>capital Greek = capital Latin</li>
          </ul>
        </div>
        <div class="mt-2">
          Any characters that are NOT converted to Greek are displayed as-is,
          and are regarded as "end-of-word" tokens; i.e., a sigma in lower case
          is transformed to its final form.
        </div>
        <div class="mt-2">
          Click &#32;
          <button
            type="button"
            class="underline decoration-2 decoration-rose-500 hover:text-zinc-500 hover:decoration-pink-500"
            onClick={fillIliad}
          >
            here
          </button>
          &#32; to try the first six lines of Homer's Iliad.
        </div>
      </div>
    </div>
  );
};

type DetailsProps = {
  setInput: (input: string) => void;
};

const Details = ({ setInput }: DetailsProps) => {
  const [hidden, setHidden] = useState(true);

  const toggle = () => {
    setHidden(!hidden);
  };

  return (
    <>
      <div class="flex justify-center mt-3">
        <div class="flex-none text-sm text-emerald-600" onClick={toggle}>
          [Details]
        </div>
      </div>
      <DetailsInner hidden={hidden} setInput={setInput} />
    </>
  );
};

const Header = ({ setInput }: DetailsProps) => {
  return (
    <>
      <div class="flex justify-center">
        <h1 class="extrabold text-3xl underline decoration-4 decoration-rose-500 text-stone-700 flex-none">
          <span class="text-sm">&nbsp;</span>
          Latin to Greek
          <span class="text-sm">&nbsp;</span>
        </h1>
      </div>
      <Details setInput={setInput} />
    </>
  );
};

type InputProps = {
  value: string;
  onInput: (e: JSX.TargetedEvent<HTMLInputElement, Event>) => void;
};

const Input = ({ value, onInput }: InputProps) => {
  return (
    <div class="flex justify-center mt-2">
      <input
        class="rounded-lg ring ring-rose-300 focus:outline-none hover:ring-pink-400 text-stone-700 placeholder-stone-400 caret-rose-500 flex-none w-3/5 h-10 px-3"
        value={value}
        onInput={onInput}
        placeholder="a)/nqrwpo/s e)stin."
      />
    </div>
  );
};

type OutputProps = {
  greek: string[];
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
          {greek.map((line, i) => <div key={i}>{line}</div>)}
        </div>
      </div>
    );
  }
};

const toGreekPerLine = (text: string) => {
  if (text.length === 0) {
    return [];
  }

  const lines = text.replaceAll("\n", "\n\0").split("\0");
  return lines.map(latin_to_greek);
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

  const greek = wasmReady ? toGreekPerLine(input) : [];
  const greekph = wasmReady ? latin_to_greek("fai/nei e)nqa/de.") : "";

  const onInput = (e: JSX.TargetedEvent<HTMLInputElement, Event>) => {
    setInput(e.currentTarget.value);
  };

  return (
    <div class="container max-w-full">
      <div class="bg-rose-100 pt-32 pb-8">
        <Header setInput={setInput} />
        <Input value={input} onInput={onInput} />
      </div>

      <Output greek={greek} placeholder={greekph} />
    </div>
  );
}
