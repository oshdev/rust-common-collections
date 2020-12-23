pub fn main() {
    println!("8.2 Storing UTF-8 Encoded Text with Strings");

    println!("Creating a String");
    // Creating a new, empty String.
    let mut _s = String::new();

    // Using the `to_string` method to create a String from a string literal.
    let data = "initial contents";
    let _ = data.to_string();
    // The method also works on a literal directly.
    let _ = "initial contents".to_string();
    // The following is equivalent to the above.
    let _ = String::from("initial contents");

    // Storing greeting in different languages in strings
    let _ = String::from("السلام عليكم");
    let _ = String::from("Dobrý den");
    let _ = String::from("Hello");
    let _ = String::from("שָׁלוֹם");
    let _ = String::from("नमस्ते");
    let _ = String::from("こんにちは");
    let _ = String::from("안녕하세요");
    let _ = String::from("你好");
    let _ = String::from("Olá");
    let _ = String::from("Здравствуйте");
    let _ = String::from("Hola");

    println!("Updating a String");
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // take a string slice to not take the ownership of s2
    println!("s2 is {}", s2);
    s.push('!'); // add a single character
    println!("s is {}", s);

    println!("Concatenating a string");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _ = s1 + &s2; // s1 has been moved here and can no longer be used.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _ = format!("{}-{}-{}", s1, s2, s3);

    println!("Indexing into String is not supported for safety reasons.");
    println!("1. Strings are stored as UTF-8, therefore it's impossible to know ahead
    how many bytes each _letter_ takes (1 or more).
2. Bytes, Scalar Values, Grapheme Clusters.
3. Indexing operations are expected to always the constant time O(1),
    which is impossible to achieve with String type.");

    println!("Slicing strings is support, but risky - splitting at chat boundary is an error");
    /*
    let hello = "ø";
    let _ = &hello[0..1];
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'ø' (bytes 0..2) of `ø`'
     */

    println!("Methods for Iterating Over Strings.");
    print!("Chars: ");
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    print!("\nBytes: ");
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!("
Getting grapheme clusters from strings is complex,
therefore not provided in the standard library.
Crates are available on crates.io if needed.");
}
