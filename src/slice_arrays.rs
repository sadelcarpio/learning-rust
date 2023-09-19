use std::mem;

fn analyze_slice(slice: &[i32]) {  // can only access first element and its length
    println!("First element of the slice: {}", slice[1]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];  // signature is [type, n_elements]
    let ys: [i32; 500] = [0; 500];  // initialize all 500 elements to 0
    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("Length of the array: {}", xs.len());
    // Arrays are allocated on the stack
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    // Arrays can be "borrowed" as slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);
    println!("Borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);  // like python indexing
    // Empty slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);  // Same but more verbose
    for i in 0..xs.len() + 1 {  // one element more
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            // allows to handle None in get method
            None => println!("Slow down! {} is too far!", i)
        }
    }
    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}
