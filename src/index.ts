function multiplyBy5(nums: number[], idx: number): number {
    return (nums[idx] ?? idx) * 5;
}

const nums: number[] = [1,2,3];

console.log(multiplyBy5(nums, 1));
console.log(multiplyBy5(nums, 3));
