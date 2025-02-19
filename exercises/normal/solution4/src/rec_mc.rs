pub fn dp_rec_mc(mut amount: u32) -> u32 {
    // TODO: 这里写逻辑
    const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut a = 0;
    let mut b = 0;
    let mut res = 0;
    for i in 0..8{
        a = amount%CASHES[7-i];
        b = (amount - a)/CASHES[7-i];
        res += b;
        amount = a;
    }
    res
}
