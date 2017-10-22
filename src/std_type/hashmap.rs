use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry",
        "645-7689" => "hello",
        _ => "who?",
    }
}

pub fn test() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't hve Daniel's number."),
    }

    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("calling Ashley : {}", call(number)),
        _ => println!("Don't have"),
    }

    contacts.remove(&"Ashley");

    for (contact, &number) in contacts.iter() {
        println!("calling {} : {}", contact, call(number));
    }
}