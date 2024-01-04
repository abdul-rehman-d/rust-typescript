import fs from "fs";

const filename = process.argv[2];

if (!filename) {
    process.exit(1);
}

fs.readFileSync(filename)
    .toString()
    .split('\n')
    .forEach(
        line => {
            const print = parseInt(line);
            if (isNaN(print)) {
                console.log("Line not a number");
            } else {
                console.log(print);
            }
        }
    );