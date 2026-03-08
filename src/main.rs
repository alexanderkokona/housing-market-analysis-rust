use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
struct House {
    city: String,
    price: f64,
    bedrooms: u32,
    bathrooms: u32,
    sqft: f64,
    year_built: u32,
}

fn load_data(path: &str) -> Result<Vec<House>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(path)?;
    let mut houses = Vec::new();

    for result in reader.deserialize() {
        let record: House = result?;
        houses.push(record);
    }

    Ok(houses)
}

fn filter_by_city(houses: &Vec<House>, city_name: &str) -> Vec<House> {
    houses
        .iter()
        .filter(|h| h.city == city_name)
        .cloned()
        .collect()
}

fn sort_by_price(mut houses: Vec<House>) -> Vec<House> {
    houses.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    houses
}

fn average_price_by_city(houses: &Vec<House>) -> HashMap<String, f64> {
    let mut totals: HashMap<String, (f64, u32)> = HashMap::new();

    for house in houses {
        let entry = totals.entry(house.city.clone()).or_insert((0.0, 0));
        entry.0 += house.price;
        entry.1 += 1;
    }

    let mut averages = HashMap::new();

    for (city, (total, count)) in totals {
        averages.insert(city, total / count as f64);
    }

    averages
}

fn average_price_per_sqft(houses: &Vec<House>) -> HashMap<String, f64> {
    let mut totals: HashMap<String, (f64, u32)> = HashMap::new();

    for house in houses {
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

fn main() -> Result<(), Box<dyn Error>> {

    let houses = load_data("data/housing.csv")?;

    println!("\nFULL DATASET");
    for house in &houses {
        println!("{:?}", house);
    }

    println!("\nFILTER: Homes in Boise");
    let boise = filter_by_city(&houses, "Boise");
    for house in &boise {
        println!("{:?}", house);
    }

    println!("\nSORT: Homes by Price");
    let sorted = sort_by_price(houses.clone());
    for house in &sorted {
        println!("{:?}", house);
    }

    println!("\nAGGREGATION: Average Price by City");
    let averages = average_price_by_city(&houses);
    for (city, avg) in &averages {
        println!("{}: ${:.2}", city, avg);
    }

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

    println!("\nQuestion 2: Average price per square foot by city");
    let avg_sqft = average_price_per_sqft(&houses);

    for (city, value) in &avg_sqft {
        println!("{}: ${:.2} per sqft", city, value);
    }

    Ok(())
}