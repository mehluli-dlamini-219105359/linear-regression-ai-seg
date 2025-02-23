use rand::Rng;
struct Person {
    name: String,
    age: f64,    // Using float 64 for floating point values (with noise)
    height: f64, // Using float 64 for floating point values (with noise)
}

fn main() {
    //random number generator (noise)
    let mut rng = rand::thread_rng();

    let people: Vec<Person> = vec![
        Person { name: "Mehluli".to_string(), age: 26.11, height: 173.90 },
        Person { name: "Esihle".to_string(), age: 22.8, height: 154.78 },
        Person { name: "Luyanda".to_string(), age: 22.3, height: 160.70},
        Person { name: "Ndumiso".to_string(), age: 35.80, height: 163.45 },
        Person { name: "Belusiwe".to_string(), age: 28.10, height: 168.00 },
        Person { name: "Karabo".to_string(), age: 40.3, height: 176.98 },
        Person { name: "Lindiwe".to_string(), age: 32.3, height: 160.90 },
    ];
    // Add noise to each person's age and height
    let noisy_agent: Vec<Person> = people.into_iter().map(|person| {
        let age_noise = rng.gen_range(-2.0..2.0);  // Add noise to age
        let height_noise = rng.gen_range(-3.0..3.0);  // Add noise to height

        Person {
            name: person.name,
            age: person.age + age_noise,
            height: person.height + height_noise,
        }
    }).collect();

    // My Sample Dataset and add cargo v1.0.100
    for person in &noisy_agent {
        println!("Name: {}, Age: {:.2}, Height: {:.2} cm", person.name, person.age, person.height);
    }
}
