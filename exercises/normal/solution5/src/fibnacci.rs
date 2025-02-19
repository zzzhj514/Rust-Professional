pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut index = 0;
    let mut res = 0;
    while fib(index) < threshold{
        if fib(index) % 2 == 1{
            res += fib(index);
        }
        index += 1;
    }
    res
}
pub fn fib(n:u32)->u32
{
    if n == 0 {
        return 0;
    }else if n == 1 {
        return 1;
    }else {
        return fib(n-2) + fib(n-1);
    }
}