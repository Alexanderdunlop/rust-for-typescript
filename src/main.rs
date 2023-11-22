fn practice(nums: Vec<usize>, idx: usize) -> usize {
    return nums.get(idx).unwrap_or(&idx) * 5;
}

fn main() {
    
}