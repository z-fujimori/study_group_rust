import * as fs from "fs";

const [n, y] = fs
  .readFileSync(0, "utf8")
  .trim()
  .split(/\s+/)
  .map(Number);

for (let a = 0; a <= n; a++) {
  for (let b = 0; b <= n - a; b++) {
    const c = n - a - b;
    if (10000 * a + 5000 * b + 1000 * c === y) {
      console.log(`${a} ${b} ${c}`);
      process.exit(0);
    }
  }
}

console.log("-1 -1 -1");
