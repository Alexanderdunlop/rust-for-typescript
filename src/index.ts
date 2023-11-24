import fs from "fs";

const fileName = process.argv[2];

if (fileName) {
  fs.readFileSync(fileName)
    .toString()
    .split("\n")
    .forEach((line) => {
      const print = parseInt(line);
      if (isNaN(print)) {
        console.log("Line not a number");
      } else {
        console.log(print);
      }
    });
}

// Prints
// 1
// 5
// 9
// 33
// Line not a number
