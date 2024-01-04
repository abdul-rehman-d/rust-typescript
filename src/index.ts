type Custom = {
    age: number;
    name: string;
}

type Item = number | string | Custom;

function append(list: Item[]) {
    list.push("Hello, fem");
}

const items: Item[] = [];

console.log(items);
append(items);
console.log(items);

const nums: number[] = [1,2,3];

console.log(nums);
append(nums);
console.log(nums);
