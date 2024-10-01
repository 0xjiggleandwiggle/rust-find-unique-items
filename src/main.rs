
fn get_unique_items<T: PartialEq>(items: Vec<T>) -> Vec<T> {
    let mut tmp: Vec<T> = Vec::new();
    for item in items {
        if !tmp.contains(&item) {
            tmp.push(item);
        }
    }
    tmp
}

fn main() {

    //strings
    let vec1 = vec!["Hi", "Hi", "Bye", "Greetings", "Apple", "Banana","Bye"];
    assert_eq!(get_unique_items(vec1), vec!["Hi", "Bye", "Greetings", "Apple", "Banana"]);
    
    // i32
    let vec2 = vec![10, 100, 37, 37, 110, 23, 23, 23, 1];
    assert_eq!(get_unique_items(vec2), vec![10, 100, 37, 110, 23, 1]);
    println!("Hoooray all tests passed");
}
