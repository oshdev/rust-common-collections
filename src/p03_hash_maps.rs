use std::collections::HashMap;

pub fn main() {
    println!("8.3 Storing Keys with Associated Values in Hash Maps");

    println!("Creating a New Hash Map");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Collecting Iterator Values into a Hash Map");
    let teams = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 50];
    let _: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    println!("Hash Maps and Ownership");
    let key = String::from("colour");
    let value = String::from("Red");
    let mut map = HashMap::new();
    map.insert(key, value);

    /* error[E0382]: borrow of moved value: `key`
    println!("key = {:#?}", key);

    let key = String::from("colour");
        --- move occurs because `key` has type `String`, which does not implement the `Copy` trait
    ...
    map.insert(key, value);
               ---value moved here
    println!("key = {:#?}", key);
                            ^^^ value borrowed here after move
    */

    println!("Accessing Values in a Hash Map");
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);

    let team_name = String::from("Blue");
    let _ = scores.get(&team_name);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    println!("Updating a Hash Map");

    println!("Overwriting a Value");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("scores = {:?}", scores);

    println!("Only Inserting a Value if the Key Has No Value");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores = {:?}", scores);

    println!("Updating a Value Based on the Previous Value");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    println!("Hashing Functions");
    println!("HashMap uses a strong hashing function that you can switch to another one");
}
