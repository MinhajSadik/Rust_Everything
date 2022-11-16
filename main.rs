use std::{cmp::Ordering, io};

// use std::mem;
use rand::Rng;

// fn analyze_slice(slice: &[i32]) {
//     println!("first element of the slice: {}", slice[0]);
//     println!("the slice has {} elements", slice.len());
// }
fn main() {
    println!("Guess your number!");

    let secreet_number = rand::thread_rng().gen_range(1..=100);

    print!("{}\n", secreet_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("please enter valid input");

        println!("Your Guess Number is {}", guess);

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secreet_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Yehh you are win!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }

    // // Fixed-size array (type signature is superfluous)
    // let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // // All elements can be initialized to the same value
    // let ys: [i32; 500] = [0; 500];

    // // Indexing starts at 0
    // println!("first element of the array: {}", xs[0]);
    // println!("second element of the array: {}", xs[1]);

    // // `len` returns the count of elements in the array
    // println!("number of elements in array: {}", xs.len());

    // // Arrays are stack allocated
    // println!("array occupies {} bytes", mem::size_of_val(&xs));

    // // Arrays can be automatically borrowed as slices
    // println!("borrow the whole array as a slice");
    // analyze_slice(&xs);

    // println!("borrow a section of the array as a slice");
    // analyze_slice(&ys[1..4]);

    // // Example of empty slice `&[]`
    // let empty_array: [u32; 0] = [];
    // assert_eq!(&empty_array, &[]);
    // assert_eq!(&empty_array, &[][..]); // same but more verbose

    // // message instead of happily continue.
    // for i in 0..xs.len() + 1 {
    //     // OOPS, one element too far
    //     match xs.get(i) {
    //         Some(xval) => println!("{}: {}", i, xval),
    //         None => println!("Slow down! {} is too far!", i),
    //     }
    // }
}
