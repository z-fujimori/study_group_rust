import * as fs from "node:fs";

const input = fs.readFileSync(0, "utf8").trim().split(/\s+/).map(Number);

const n = input[0];
const l = input[1];
const aList = input.slice(2, 2 + n);

let count = 0;
for (const a of aList) {
  if (a >= l) {
    count++;
  }
}

console.log(count);
