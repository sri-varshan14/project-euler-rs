fn solve(n: u64) -> u128{
    let mut result: u128 = (n*(n+1)/2 as u64).pow(2) as u128;

    for i in 1..=n {
        result -= i.pow(2) as u128;
    }

    return result;
}
fn main(){
    let n = 100;
    println!("{}",solve(n));
}