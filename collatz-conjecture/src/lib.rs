pub fn collatz(n: u64) -> Option<u64> {
    let mut step_count : u64 = 0;
    let mut num = n;
    if num == 0 {
        None
    } else {
        while num != 1 {
            step_count += 1;
            if (num & 1) == 1 {
                num = (3 * num) + 1;
            } else {
                num = num / 2;
            }
        }
        Some(step_count)
    }
}
