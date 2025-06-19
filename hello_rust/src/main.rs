use std::usize;

fn main() {
    println!("Hello, world!");

    let x = 5; // Immutable, type inferred as i32 (32-bit integer)
    println!("x is {}", x);

    let mut y = 10; // Mutable with `mut`
    println!("y is {}", y);
    y = 15; // Changing the value
    println!("now y is {}", y);

    show_ownership();

    control_flows();

    println!("sum function: {}", add(2, 4));

    println!("is even? {}, {}", 2, is_even(2));
    println!("is even? {}, {}", 3, is_even(3));

    let search_me = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

    println!("Binary search: {:?} found {} at {:?}", search_me, 1, binary_search(&search_me, 1));
    println!("Binary search: {:?} found {} at {:?}", search_me, 5, binary_search(&search_me, 5));
    println!("Binary search: {:?} found {} at {:?}", search_me, 19, binary_search(&search_me, 19));
    println!("Binary search: {:?} don't find {} {:?}", search_me, 2, binary_search(&search_me, 2));
    println!("Binary search: {:?} don't find {} {:?}", search_me, -2, binary_search(&search_me, -2));
    println!("Binary search: {:?} don't find {} {:?}", search_me, 200, binary_search(&search_me, 200));

}

fn show_ownership() {
    let s1 = String::from("hello"); // s1 owns the string
    let s2 = s1; // Ownership moves to s2, s1 is no longer valid
    // println!("{}", s1); // This would error!
    println!("{}", s2);

    let s3 = String::from("hello2");
    let s4 = &s3; // Borrow s1 immutably
    println!("s3: {}, s4: {}", s3, s4);
}

fn control_flows() {
    let number = 3;
    if number > 0 {
        println!("Positive");
    } else {
        println!("Zero or negative");
    }

    // Loop with a break
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!("Counter stopped at {}", counter);

    // For loop over a range
    for i in 0..5 {
        println!("i is {}", i);
    }
}

fn add(a: i32, b: i32) -> i32 { // `->` specifies return type
    a + b // No semicolon means this is returned
}

fn is_even(num: i32) -> bool {
    return num % 2 == 0
}

/// assumes sorted asc vec
fn binary_search(arr: &Vec<i64>, target: i64) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None
    }

    let mut left: usize = 0;
    let mut right = len - 1;

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    if arr[left] == target {
        Some(left)
    } else {
        None 
    }
}

