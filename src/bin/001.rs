fn ap_sum(first_nu: u64, last_nu: u64, diff: u64) -> u64{
    return (first_nu + last_nu)*((last_nu - first_nu)/diff+1)/2;
}

fn solve(n: u64, a: u64, b: u64) -> u64{
    return ap_sum(a, n-(n%a), a) + ap_sum(b, n-(n%b), b) - ap_sum(a*b, n-n%(a*b), a*b);
}
fn main(){
    let n: u64=1000;let a: u64=3;let b: u64=5;
    println!("{}",solve(n-1,a,b));
}
