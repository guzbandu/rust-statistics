use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut map = HashMap::new();
    for i in 1..30 {
	let random_number = rand::thread_rng().gen_range(1..=10);
        println!("The {i} number is: {random_number}");
        v.push(random_number);
        let count = map.entry(random_number).or_insert(0);
        *count += 1; 
    }
    //bubble sort
    let k = &v.len();
    let mut swap = true;
    while swap {
        swap = false;
        for i in 0..(k-1) {
            if v[i]>v[i+1] {
                let temp=v[i];
                v[i]=v[i+1];
                v[i+1]=temp;
                swap=true;
            }
        }
    }
    //println!("{:?}", map);
    let mut max = 0;
    let mut mode = -1;
    for (rdm_nmbr, cnt) in map {
        println!("{rdm_nmbr} has count {cnt}");
        if cnt > max {
            max = cnt;
            mode = rdm_nmbr
        }
    }
    let k = &v.len();
    let midway = *k / 2;
    println!("The median is element {midway} which is {}", v[midway]);
    println!("The mode is number {mode} which occured {max} times");
}
