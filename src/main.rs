
fn multiply_by_5(nums: &Vec<usize>, idx: usize) -> usize {    
    return nums.get(idx).unwrap_or(&idx) * 5;
}

fn main() {
    let v = vec![1,2,3];

    println!("{:?}", multiply_by_5(&v, 1));
    println!("{:?}", multiply_by_5(&v, 3));
}
