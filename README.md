# linear-regression-ai
Software Engineering (SEG580S) Repo for the Rust Linear Regression Assignment

MACOS Set Up Steps
- Install rust using the MacOS cmnd line -> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Install RustRover (Non-commercial license for free use) -> Using your personal email or github for verification
- In Rust Rover open the terminal and run the following cmnds
  - cargo new linear_regression_model
  - cd linear_regression_model
 
Rust Cargo File (issues)
- Do not copy the contents of cargo file from Assignment pdf , issues came across were spacing and different style in quotation marks. Use a text editor to navigate through  that.

**HOW TO SOLVE A REAL WORLD PROBLEM (Predicting Height using a raw data set)**
- Understand what a dataset is -> https://towardsdatascience.com/rust-the-next-big-thing-in-data-science-319a03305883/
- Prepare a dataset , I used peoples Names , Ages and Height. This is not a linear regression model , a linear regression model.
- A linear regression model can be done manually or by using a crate (linkregress) which can be used to find the best fitting line and make predictions.
- [Install] add cargo v1.0.100 for the latest data set to be running optimally , run file and confirm with noise to age and height are being added.
- My model is currently raw and with the help of https://chatgpt.com/ , I searched how to make my model linear. Chatgpt suggested that I linear regression by implementing the least squares method , to fit the model and find the best fit line.
- This will give us the slope and intercept which inturn can be used to predict the height for a given age.
- (Image Below contains a screenshot of the predicted height for a given age (29).)
- ![image](https://github.com/user-attachments/assets/a711f188-51e0-41ae-b410-3d1b2171c54a)
  
- **To test the predicted age of your liking , make changes to this function** :
    let predicted_height = predict(slope, intercept, 29.0);
    println!("\nPredicted height for age 29: {:.2} cm", predicted_height);
- 
