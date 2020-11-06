use std::cmp::max;

fn main() {
    println!("Iteratively:");
    print_100(find_smallest_i_iteratively);
    println!("Recursively:");
    print_100(find_smallest_i_recursively);
}

fn print_100(f: fn(u32) -> (u32, u32)) {
    for n in 1..=100 {
        let (i, largest_n) = f(n);
        println!(
            "for n = {:3}, smallest i: {:3}, largest n: {:4}",
            n, i, largest_n
        );
    }
}

fn find_smallest_i_iteratively(n: u32) -> (u32, u32) {
    let mut current_n = n;
    let mut largest_n = current_n;
    let mut i = 0;
    while current_n != 1 {
        current_n = f(current_n);
        if current_n > largest_n {
            largest_n = current_n;
        }
        i += 1;
    }
    (i, largest_n)
}

fn find_smallest_i_recursively(n: u32) -> (u32, u32) {
    if n == 1 {
        (0, 1)
    } else {
        let next_n = f(n);
        let (i, largest_n) = find_smallest_i_recursively(next_n);
        (i + 1, max(n, largest_n))
    }
}

fn f(n: u32) -> u32 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}
