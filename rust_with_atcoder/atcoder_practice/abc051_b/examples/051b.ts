import * as fs from "fs";

const [k, s] = fs.readFileSync(0, "utf8").trim().split(/\s+/).map(Number);

let count = 0;

for (let x = 0; x <= k; x++) {
	for (let y = 0; y <= k; y++) {
		const z = s - x - y;
		if (0 <= z && z <= k) {
			count += 1;
		}
	}
}

console.log(count);

