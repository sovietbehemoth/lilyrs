


let sample = `
000000000000000100000000000000000000000000000100000000000000000100000000000000000000000000010000000000000
000000000000000000010000000000000000000000000000000000000100000000000000000000000001000000000000000000010
`;

let repl = "";

for (i in sample) {
    if (i == "1") {
        repl += "ONE";
    } else {
        repl += i;
    }
}

let nums = [
    1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 
    64.0, 81.0, 100.0, 121.0, 144.0, 
    169.0, 196.0, 225.0, 256.0
];

for (let j = 0; j < 200; j++) {
    for (let i = 0; i < nums.length; i++) {
        Math.sqrt(nums[i]);
    }
}

console.log("JavaScript: Completed all iterations.");