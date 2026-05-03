import * as fs from "fs";

const k = Number(fs.readFileSync(0, "utf8").trim());
let ansSum = 0;

for (let a = 1; a <= k; a++) {
	for (let b = 1; b <= k; b++) {
		for (let c = 1; c <= k; c++) {
			ansSum += gcd(gcd(a, b), c);
		}
	}
}

console.log(ansSum);

function gcd(a: number, b: number): number {
	if (b === 0) {
		return a;
	}
	return gcd(b, a % b);
}

