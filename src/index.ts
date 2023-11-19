import fs from "fs";

fs.readFileSync("lines")
  .toString()
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .filter((_, i) => i > 1 && i < 4)
  .forEach((line) => console.log(line));
