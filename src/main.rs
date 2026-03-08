// Import the csv crate to read CSV files
use csv::ReaderBuilder;

// Import serde to automatically convert CSV rows into Rust structs
use serde::Deserialize;

// HashMap will help us perform aggregations by grouping values by city
use std::collections::HashMap;

// Standard Rust error handling
use std::error::Error;


// This struct represents one row of the housing dataset
// The field names must match the CSV column names
#[derive(Debug, Deserialize, Clone)]
struct House {
    city: String,
    price: f64,
    bedrooms: u32,
    bathrooms: u32,
    sqft: f64,
    year_built: u32,
}


// Function: load_data
// Reads the CSV file and converts each row into a House struct
fn load_data(path: &str) -> Result<Vec<House>, Box<dyn Error>> {

    // Create a CSV reader
    let mut reader = ReaderBuilder::new().from_path(path)?;

    // Vector to store all houses
    let mut houses = Vec::new();

    // Loop through each record in the CSV
    for result in reader.deserialize() {

        // Convert the row into a House struct
        let record: House = result?;

        // Add the record to the vector
        houses.push(record);
    }

    // Return the vector of houses
    Ok(houses)
}


// Function: filter_by_city
// Returns only houses that match a specific city
fn filter_by_city(houses: &Vec<House>, city_name: &str) -> Vec<House> {

    houses
        .iter()                       // iterate through the dataset
        .filter(|h| h.city == city_name) // keep houses matching the city
        .cloned()                     // clone the records so we can return them
        .collect()                    // collect results into a new vector
}


// Function: sort_by_price
// Sorts the houses by price from lowest to highest
fn sort_by_price(mut houses: Vec<House>) -> Vec<House> {

    houses.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    houses
}


// Function: average_price_by_city
// Calculates the average home price for each city
fn average_price_by_city(houses: &Vec<House>) -> HashMap<String, f64> {

    // Store totals and counts for each city
    let mut totals: HashMap<String, (f64, u32)> = HashMap::new();

    for house in houses {

        // Insert city if it does not exist yet
        let entry = totals.entry(house.city.clone()).or_insert((0.0, 0));

        // Add the house price to the total
        entry.0 += house.price;

        // Increase the number of houses counted
        entry.1 += 1;
    }

    // Now compute the average
    let mut averages = HashMap::new();

    for (city, (total, count)) in totals {

        averages.insert(city, total / count as f64);
    }

    averages
}


// Function: average_price_per_sqft
// Calculates the average price per square foot for each city
fn average_price_per_sqft(houses: &Vec<House>) -> HashMap<String, f64> {

    let mut totals: HashMap<String, (f64, u32)> = HashMap::new();

    for house in houses {

        // Calculate price per square foot
        let price_per_sqft = house.price / house.sqft;

        let entry = totals.entry(house.city.clone()).or_insert((0.0, 0));

        entry.0 += price_per_sqft;
        entry.1 += 1;
    }

    let mut averages = HashMap::new();

    for (city, (total, count)) in totals {

        averages.insert(city, total / count as f64);
    }

    averages
}


// Main function: program entry point
fn main() -> Result<(), Box<dyn Error>> {

    // Load the dataset from the CSV file
    let houses = load_data("data/housing.csv")?;

    // Display the entire dataset
    println!("\nFULL DATASET");
    for house in &houses {
        println!("{:?}", house);
    }

    // FILTER REQUIREMENT
    println!("\nFILTER: Homes in Boise");

    let boise = filter_by_city(&houses, "Boise");

    for house in &boise {
        println!("{:?}", house);
    }

    // SORT REQUIREMENT
    println!("\nSORT: Homes by Price");

    let sorted = sort_by_price(houses.clone());

    for house in &sorted {
        println!("{:?}", house);
    }

    // AGGREGATION REQUIREMENT
    println!("\nAGGREGATION: Average Price by City");

    let averages = average_price_by_city(&houses);

    for (city, avg) in &averages {
        println!("{}: ${:.2}", city, avg);
    }

    // QUESTION 1
    println!("\nQuestion 1: Which city has the highest average home price?");

    let mut highest_city = "";
    let mut highest_price = 0.0;

    for (city, price) in &averages {

        if *price > highest_price {

            highest_price = *price;
            highest_city = city;
        }
    }

    println!("Answer: {} (${:.2})", highest_city, highest_price);

    // QUESTION 2
    println!("\nQuestion 2: Average price per square foot by city");

    let avg_sqft = average_price_per_sqft(&houses);

    for (city, value) in &avg_sqft {
        println!("{}: ${:.2} per sqft", city, value);
    }

    Ok(())
}