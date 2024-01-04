import fs from "fs";

const contents = fs.readFileSync("./lines", 'utf-8');
contents
    .split('\n')
    .filter((_, i) => i%2===0)
    .filter((_, i) => i>1 && i<4)
    .forEach((line) => {
    console.log(line);
})
