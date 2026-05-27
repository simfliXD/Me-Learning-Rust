// Compound Data Types
// arrays, tuples, slices and strings (slice string)

// Arrays
// Sammling av samma datatyp
pub fn compound_data_types() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // [datatype; size]
    println!("Number Array: {:?}", arr);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    // Tuples
    let human: (String, u8, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Kratos", 21, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5] - Good for memory
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Gorilla"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"Harry Potter".to_string(),
        &"Lord of the Rings".to_string(),
        &"The Hobbit".to_string(),
    ];
    println!("Book Slice: {:?}", book_slices);

    // Strings VS String Slices (&str)
    // Strings [growable, mutable, owned] - Slow (Heap)
    // String Slices (&str) [immutable, borrowed] - Fast (Stack)
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Now Says {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello,World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);

    print_slice(slice);
}

fn print_slice(slice: &str) {
    println!("Slice: {}", slice);
}
