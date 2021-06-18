import { fibonacci, pedersen_prove, blake_prove } from "wasm-prover";

const pre = document.getElementById("wasm-prover");

// compute the median of an array
const median = arr => {
    const mid = Math.floor(arr.length / 2),
      nums = [...arr].sort((a, b) => a - b);
    return arr.length % 2 !== 0 ? nums[mid] : (nums[mid - 1] + nums[mid]) / 2;
  };

const repeat = 5;
const input = 27;
const perf = Array.from(
    {length: repeat},
    (_, i) => {
        const t0 = performance.now();
        pedersen_prove();
        const t1 = performance.now();
        return t1 - t0;
    }
);
pre.textContent = `median time of running zk_prove(): ${median(perf)} ms`;
