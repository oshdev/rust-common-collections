pub fn main() {
    println!("8.1 Storing Lists of Values with Vectors");

    // If we don't assign any values, we need to specify the type of a generic vector.
    let _: Vec<i32> = Vec::new();
    // Typically we can infer the type of the vector at the time of creation.
    let _ = vec![1, 2, 3];

    // To add values to a vector we need to make it mutable as usual.
    // Rust is smart enough to deduce the type from the data pushed in further down.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Vector is freed when it goes out of scope.
    {
        let _v = vec![1, 2, 3, 4];
        // do stuff with _v
    } // <- _v goes out of scope and is freed here

    println!("Reading Element of Vectors.");
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // value pointer indexing
    println!("The third element is {}", third);
    match v.get(2) { // Vec::get method
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let _does_not_exist = &v[100];
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
    let _does_not_exist = v.get(100);

    /* error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
                 - immutable borrow occurs here
    v.push(6);
    ^^^^^^^^^ mutable borrow occurs here
    println!("The first element is: {}", first);
                                         ----- immutable borrow later used here
    */

    let v = vec![100, 32, 57];
    println!("Iterating over the Values in a Vector: {:?}", v);
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    println!("Iterating over mutable refs of a Vector (like mapping): {:?}",v );
    for i in &mut v {
        *i *= 10;
    }
    println!("{:?}", v);

    println!("Using an Enum to Store Multiple Types");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(6.47)
    ];
    println!("{:?}", row);
}
