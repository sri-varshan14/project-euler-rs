fn solve(n: u128) -> u128{
    let mut result: u128 = 0;
    let mut a=1;let mut b=2;
    while b < n {
        if b & 1 == 0 {
            result += b;
        }
        let sum = a+b;
        a = b;
        b = sum;
    }
    return result;
}

fn main(){
    let n: u128 = 4_000_000;
    println!("{}",solve(n));
}
