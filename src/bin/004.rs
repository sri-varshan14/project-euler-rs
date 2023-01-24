use std::collections::HashMap;

fn is_prime(num: u64) -> bool{
    let temp = ((num as f64).sqrt() + 1.0) as u64;
    for i in 2..=temp {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn next_prime(num: u64) -> u64{
    if num == 2{
        return 3;
    }
    let mut a=num;
    loop {
        a += 2;
        if is_prime(a){
            return a;
        }
    }
}

fn update_mp(num: u64, mp: &mut HashMap<u64,u64>){
    let mut dup_num = num;
    let mut i: u64 = 2;
    while dup_num > 1 {
        let mut count=0;
        while dup_num % i == 0 {
            count+=1;
            dup_num /= i;
        }

        if mp.contains_key(&i) {
            if mp[&i] < count {
                mp.insert(i, count);
            }
        }
        else {
            mp.insert(i, count);
        }

        i = next_prime(i);
    }
}

fn solve(num :u64) -> u64{
    let mut mp: HashMap<u64, u64> = HashMap::new();

    for i in 2..=num {
        update_mp(i,&mut mp);
    }
    let mut result: u64 = 1;
    for k in mp.keys(){
        result *= k.pow(mp[k] as u32);
    }

    return result;
}
fn main(){
    let n: u64 = 20;
    println!("{}",solve(n));
}
