// Import the csv crate so the program can read CSV files
use csv::ReaderBuilder;

// Import serde so CSV rows can automatically be converted into Rust structs
use serde::Deserialize;

// HashMap allows us to store key/value pairs (used for grouping values by city)
use std::collections::HashMap;

// Standard Rust error handling trait
use std::error::Error;


// This struct represents a single row in the housing dataset.
//
// The struct fields must match the column names in the CSV file
// so that serde can correctly deserialize each row.
#[derive(Debug, Deserialize, Clone)]
struct House {
    city: String,      // City where the house is located
    price: f64,        // Price of the house
    bedrooms: u32,     // Number of bedrooms
    bathrooms: u32,    // Number of bathrooms
    sqft: f64,         // Size of the house in square feet
    year_built: u32,   // Year the house was built
}


// Function: load_data
//
// Reads a CSV file and converts each row into a House struct.
// Returns a vector containing all houses in the dataset.
fn load_data(path: &str) -> Result<Vec<House>, Box<dyn Error>> {

    // Create a CSV reader using the file path provided
    let mut reader = ReaderBuilder::new().from_path(path)?;

    // Vector that will store all parsed house records
    let mut houses = Vec::new();

    // Iterate through each row of the CSV file
    for result in reader.deserialize() {

        // Deserialize the row into a House struct
        // If an error occurs, it will be returned
        let record: House = result?;

        // Add the parsed record to our dataset
        houses.push(record);
    }

    // Return the vector of houses
    Ok(houses)
}


// Function: filter_by_city
//
// Returns a new vector containing only houses
// that belong to the specified city.
fn filter_by_city(houses: &Vec<House>, city_name: &str) -> Vec<House> {

    houses
        .iter()                        // Iterate over the list of houses
        .filter(|h| h.city == city_name) // Keep only houses where city matches
        .cloned()                      // Clone each result so we can return ownership
        .collect()                     // Collect results into a new Vec<House>
}


// Function: sort_by_price
//
// Sorts houses from lowest price to highest price.
// Takes ownership of the vector so it can modify it.
fn sort_by_price(mut houses: Vec<House>) -> Vec<House> {

    // Sort the vector by comparing house prices
    // partial_cmp is required because f64 values are floating-point
    houses.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    // Return the sorted vector
    houses
}


// Function: average_price_by_city
//
// Calculates the average home price for each city.
// Returns a HashMap where:
// key   = city name
// value = average home price
fn average_price_by_city(houses: &Vec<House>) -> HashMap<String, f64> {

    // Temporary map to store:
    // city -> (total_price, number_of_houses)
    let mut totals: HashMap<String, (f64, u32)> = HashMap::new();

    // Loop through every house
    for house in houses {

        // Get the entry for this city.
        // If it doesn't exist yet, insert (0.0 total price, 0 count).
        let entry = totals.entry(house.city.clone()).or_insert((0.0, 0));

        // Add the current house price to the city's total
        entry.0 += house.price;

        // Increment the count of houses in that city
        entry.1 += 1;
    }

    // Now convert totals into averages
    let mut averages = HashMap::new();

    for (city, (total, count)) in totals {

        // average = total price / number of houses
        averages.insert(city, total / count as f64);
    }

    // Return the computed averages
    averages
}


// Function: average_price_per_sqft
//
// Calculates the average price per square foot for each city.
fn average_price_per_sqft(houses: &Vec<House>) -> HashMap<String, f64> {

    // city -> (total_price_per_sqft, number_of_houses)
    let mut totals: HashMap<String, (f64, u32)> = HashMap::new();

    for house in houses {

        // Compute price per square foot for this house
        let price_per_sqft = house.price / house.sqft;

        // Get or create the entry for the city
        let entry = totals.entry(house.city.clone()).or_insert((0.0, 0));

        // Add to running total
        entry.0 += price_per_sqft;

        // Increment house count
        entry.1 += 1;
    }

    // Convert totals into averages
    let mut averages = HashMap::new();

    for (city, (total, count)) in totals {

        averages.insert(city, total / count as f64);
    }

    averages
}


// Main function: the entry point of the program
fn main() -> Result<(), Box<dyn Error>> {

    // Load housing data from CSV file
    let houses = load_data("data/housing.csv")?;

    // Print the full dataset
    println!("\nFULL DATASET");
    for house in &houses {
        println!("{:?}", house);
    }

    // ----------------------------------------------------
    // FILTER EXAMPLE
    // ----------------------------------------------------

    println!("\nFILTER: Homes in Boise");

    // Create a subset of houses located in Boise
    let boise = filter_by_city(&houses, "Boise");

    for house in &boise {
        println!("{:?}", house);
    }

    // ----------------------------------------------------
    // SORT EXAMPLE
    // ----------------------------------------------------

    println!("\nSORT: Homes by Price");

    // Clone the dataset so we can sort without losing the original
    let sorted = sort_by_price(houses.clone());

    for house in &sorted {
        println!("{:?}", house);
    }

    // ----------------------------------------------------
    // AGGREGATION EXAMPLE
    // ----------------------------------------------------

    println!("\nAGGREGATION: Average Price by City");

    // Compute average prices
    let averages = average_price_by_city(&houses);

    for (city, avg) in &averages {
        println!("{}: ${:.2}", city, avg);
    }

    // ----------------------------------------------------
    // QUESTION 1
    // ----------------------------------------------------

    println!("\nQuestion 1: Which city has the highest average home price?");

    let mut highest_city = "";
    let mut highest_price = 0.0;

    // Loop through the averages map to find the maximum value
    for (city, price) in &averages {

        if *price > highest_price {

            highest_price = *price;
            highest_city = city;
        }
    }

    println!("Answer: {} (${:.2})", highest_city, highest_price);

    // ----------------------------------------------------
    // QUESTION 2
    // ----------------------------------------------------

    println!("\nQuestion 2: Average price per square foot by city");

    let avg_sqft = average_price_per_sqft(&houses);

    for (city, value) in &avg_sqft {
        println!("{}: ${:.2} per sqft", city, value);
    }

    Ok(())
}