use rand::Rng;

struct Person {
    name: String,
    age: f64,    // Age (input feature)
    height: f64, // Height (output label)
}

// Forward pass (prediction)
fn forward_pass(slope: f64, intercept: f64, age: f64) -> f64 {
    // Using the linear regression model to predict height from age
    slope * age + intercept
}

// Mean Squared Error Loss function
fn compute_loss(predictions: &Vec<f64>, actuals: &Vec<f64>) -> f64 {
    let n = predictions.len();
    let squared_errors: f64 = predictions.iter()
        .zip(actuals.iter())
        .map(|(pred, actual)| (pred - actual).powi(2))
        .sum();
    squared_errors / n as f64
}

// Calculate the slope (beta_1)
fn calculate_slope(data: &Vec<Person>, n: usize) -> f64 {
    let sum_x = data.iter().map(|p| p.age).sum::<f64>();
    let sum_y = data.iter().map(|p| p.height).sum::<f64>();
    let sum_xy = data.iter().map(|p| p.age * p.height).sum::<f64>();
    let sum_x_squared = data.iter().map(|p| p.age * p.age).sum::<f64>();

    (n as f64 * sum_xy - sum_x * sum_y) / (n as f64 * sum_x_squared - sum_x * sum_x)
}

// Calculate the intercept (beta_0)
fn calculate_intercept(data: &Vec<Person>, slope: f64, n: usize) -> f64 {
    let sum_y = data.iter().map(|p| p.height).sum::<f64>();
    let sum_x = data.iter().map(|p| p.age).sum::<f64>();

    (sum_y - slope * sum_x) / (n as f64)
}

fn main() {
    // Random number generator (for adding noise)
    let mut rng = rand::thread_rng();

    // Original dataset
    let people: Vec<Person> = vec![
        Person { name: "Mehluli".to_string(), age: 26.11, height: 173.90 },
        Person { name: "Esihle".to_string(), age: 22.8, height: 154.78 },
        Person { name: "Luyanda".to_string(), age: 22.3, height: 160.70},
        Person { name: "Ndumiso".to_string(), age: 35.80, height: 163.45 },
        Person { name: "Belusiwe".to_string(), age: 28.10, height: 168.00 },
        Person { name: "Karabo".to_string(), age: 40.3, height: 176.98 },
        Person { name: "Lindiwe".to_string(), age: 32.3, height: 160.90 },
    ];

    // Add noise to each person's age and height to simulate real-world data variability
    let noisy_agent: Vec<Person> = people.into_iter().map(|person| {
        let age_noise = rng.gen_range(-2.0..2.0);  // Add noise to age
        let height_noise = rng.gen_range(-3.0..3.0);  // Add noise to height

        Person {
            name: person.name,
            age: person.age + age_noise,
            height: person.height + height_noise,
        }
    }).collect();

    // Number of data points
    let n = noisy_agent.len();

    // Training: Calculate the slope and intercept based on the noisy data
    let slope = calculate_slope(&noisy_agent, n);
    let intercept = calculate_intercept(&noisy_agent, slope, n);

    // Use the model to predict values
    let predictions: Vec<f64> = noisy_agent.iter()
        .map(|person| forward_pass(slope, intercept, person.age))
        .collect();

    // Get the actual values (height)
    let actuals: Vec<f64> = noisy_agent.iter()
        .map(|person| person.height)
        .collect();

    // Compute the loss (Mean Squared Error)
    let loss = compute_loss(&predictions, &actuals);

    // Output the linear regression model and loss
    println!("Linear Regression Model:");
    println!("y = {:.2} * x + {:.2}", slope, intercept);
    println!("\nMean Squared Error (Loss): {:.2}", loss);

    // Test the trained model by making a prediction for a new value (e.g., age = 29)
    let predicted_height = forward_pass(slope, intercept, 29.0);
    println!("\nPredicted height for age 29: {:.2} cm", predicted_height);
}