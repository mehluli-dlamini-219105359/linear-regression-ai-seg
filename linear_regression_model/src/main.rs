struct Person {
    name: String,
    age: i32,
    height: i32,
}

fn main() {
    let people: Vec<Person> = vec![
        Person { name: "Mehluli".to_string(), age: 26, height: 173 },
        Person { name: "Esihle".to_string(), age: 22, height: 154 },
        Person { name: "Luyanda".to_string(), age: 22, height: 160 },
        Person { name: "Ndumiso".to_string(), age: 35, height: 163 },
        Person { name: "Belusiwe".to_string(), age: 28, height: 168 },
        Person { name: "Karabo".to_string(), age: 40, height: 176 },
        Person { name: "Lindiwe".to_string(), age: 32, height: 160 },
    ];

    // Print the dataset
    for person in &people {
        println!("Name: {}, Age: {}, Height: {} cm", person.name, person.age, person.height);
    }
}