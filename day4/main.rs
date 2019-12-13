use std::convert::TryInto;

fn main() {
    day4();
}

fn day4() {
    let low = 197487;
    let high = 673251;
    let mut count = 0;

    for test in low..high {
        if !only_two_digits(test) { continue; }
        if !not_decrease(test) { continue; }
        count += 1;
    }

    println!("Total matches:{}", count);
}

fn num_to_vec(num: usize) -> Vec<usize> {
    let mut vec = vec![99; 6];
    let base: usize = 10;
    for i in 0..6 {
        vec[5-i] = (num / base.pow(i.try_into().unwrap())) % 10;
    }
    return vec;
}

fn only_two_digits(num: usize) -> bool {
    let vec = num_to_vec(num);
    let mut i = 0;
    loop {
        if i >= vec.len() - 1 { return false; }
        if vec[i] == vec[i+1] {
            let no_prev = i <= 0 || vec[i] != vec[i-1];
            let no_after = i >=  vec.len() - 2 || vec[i] != vec[i+2];
            if no_prev && no_after { return true; }
        }
        i += 1;
    }
}

fn not_decrease(num: usize) -> bool {
    let vec = num_to_vec(num);
    for i in 0..vec.len()-1 {
        if vec[i] > vec[i+1] {
            return false;
        }
    }
    return true;
}
