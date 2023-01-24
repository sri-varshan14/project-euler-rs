fn is_prime(num: f64) -> bool{
    let n = (num.sqrt() + 1.0) as u64;
    for i in 2..=n{
        if num as u64 % i == 0{
            return false;
        }
    }
    return true;
}

fn solve(n: u64) -> u64{
    let mut sq_n = ((n as f64).sqrt() + 1.0) as u64;
    while sq_n as u64 > 1{
        if is_prime(sq_n as f64){
            if n % sq_n as u64 == 0{
                return sq_n as u64;
            }
        }
        sq_n -= 1;
    }
    return 1;
}

fn main(){
    let n: u64 = 600851475143;
    println!("{}",solve(n));
}
