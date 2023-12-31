use clap::Parser;
use std::error::Error;
use std::fmt;
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::energy::{electronvolt, gigaelectronvolt, joule, kiloelectronvolt, megaelectronvolt};
use uom::si::f32::*;
use uom::si::length::{centimeter, kilometer, meter};
use uom::si::mass::kilogram;
use uom::si::time::{microsecond, millisecond, nanosecond, second};

// Define custom error type for unsupported units
#[derive(Debug)]
enum UnsupportedUnitError {
    Length(String),
    Time(String),
    Energy(String),
}

impl fmt::Display for UnsupportedUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnsupportedUnitError::Length(unit) => write!(f, "Unsupported length unit: {}", unit),
            UnsupportedUnitError::Time(unit) => write!(f, "Unsupported time unit: {}", unit),
            UnsupportedUnitError::Energy(unit) => write!(f, "Unsupported energy unit: {}", unit),
        }
    }
}

impl Error for UnsupportedUnitError {}

#[derive(Debug)]
enum DivideByZeroError {
    LengthIsZero,
    TimeIsZero,
}

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DivideByZeroError::LengthIsZero => write!(f, "Length is zero, cannot divide by zero"),
            DivideByZeroError::TimeIsZero => write!(f, "Time is zero, cannot divide by zero"),
        }
    }
}

impl std::error::Error for DivideByZeroError {}

fn calculate_energy(time: Time, length: Length) -> Result<Energy, DivideByZeroError> {
    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);

    // Check if either time or length is zero
    if time == Time::new::<second>(0.0) {
        return Err(DivideByZeroError::TimeIsZero);
    }
    if length == Length::new::<meter>(0.0) {
        return Err(DivideByZeroError::LengthIsZero);
    }

    let energy: Energy = m * (length * length) / (2.0 * time * time);

    Ok(energy)
}

// Functions to parse length and time inputs
fn parse_length(quantity: f32, unit: &str) -> Result<Length, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "cm" | "centimeter" | "centimeters" => Ok(Length::new::<centimeter>(quantity)),
        "m" | "meter" | "meters" => Ok(Length::new::<meter>(quantity)),
        "km" | "kilometer" | "kilometers" => Ok(Length::new::<kilometer>(quantity)),
        _ => Err(UnsupportedUnitError::Length(unit.to_string())),
    }
}

fn parse_time(quantity: f32, unit: &str) -> Result<Time, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ns" | "nanosecond" | "nanoseconds" => Ok(Time::new::<nanosecond>(quantity)),
        "mus" | "us" | "microsecond" | "microseconds" => Ok(Time::new::<microsecond>(quantity)),
        "ms" | "millisecond" | "milliseconds" => Ok(Time::new::<millisecond>(quantity)),
        "s" | "second" | "seconds" => Ok(Time::new::<second>(quantity)),
        _ => Err(UnsupportedUnitError::Time(unit.to_string())),
    }
}

// Enum for supported units
enum EnergyUnit {
    Electronvolt,
    Kiloelectronvolt,
    Megaelectronvolt,
    Gigaelectronvolt,
    Joule,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Flight path length from source to target in units of (cm, m, km).
    #[arg(required = true, short, long, num_args(2), value_names = ["LENGTH","UNIT"])]
    length_of_flight_path: Vec<String>,

    /// Time-of-Flight to convert to neutron Energy in units of (ns, us, ms, s).
    #[arg(required = true, short, long, num_args(2), value_names = ["TIME","UNIT"])]
    time_of_flight: Vec<String>,

    /// Desired neutron energy units of (eV, KeE, MeV, J).
    #[arg(short, long, num_args(1), default_value = "eV")]
    unit: Option<String>,
}

// Function to parse energy unit input
fn parse_energy_unit(unit: &str) -> Result<EnergyUnit, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ev" | "electronvolt" | "electronvolts" => Ok(EnergyUnit::Electronvolt),
        "kev" | "kiloelectronvolt" | "kiloelectronvolts" => Ok(EnergyUnit::Kiloelectronvolt),
        "mev" | "megaelectronvolt" | "megaelectronvolts" => Ok(EnergyUnit::Megaelectronvolt),
        "gev" | "gigaelectronvolt" | "gigaelectronvolts" => Ok(EnergyUnit::Gigaelectronvolt),
        "j" | "joule" | "joules" => Ok(EnergyUnit::Joule),
        _ => Err(UnsupportedUnitError::Energy(unit.to_string())),
    }
}

fn main() {
    let cli = Cli::parse();

    // Parse length value
    let input_length_value: f32 = match cli.length_of_flight_path[0].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error: parsing length value: {}", err);
            return;
        }
    };

    // Parse time value
    let input_time_value: f32 = match cli.time_of_flight[0].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error: parsing time value: {}", err);
            return;
        }
    };

    let input_length_unit: &str = &cli.length_of_flight_path[1].trim().to_lowercase();

    let input_time_unit: &str = &cli.time_of_flight[1].trim().to_lowercase();

    let length_quantity =
        parse_length(input_length_value, input_length_unit).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });

    let time_quantity = parse_time(input_time_value, input_time_unit).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let output_energy_unit: &str = &cli.unit.unwrap().trim().to_lowercase();

    let energy_quantity: Energy =
        calculate_energy(time_quantity, length_quantity).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });

    let energy_unit = parse_energy_unit(output_energy_unit).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    match energy_unit {
        EnergyUnit::Electronvolt => {
            println!(
                "Energy = {}",
                energy_quantity.into_format_args(electronvolt, Abbreviation)
            )
        }
        EnergyUnit::Kiloelectronvolt => {
            println!(
                "Energy = {}",
                energy_quantity.into_format_args(kiloelectronvolt, Abbreviation)
            )
        }
        EnergyUnit::Megaelectronvolt => {
            println!(
                "Energy = {}",
                energy_quantity.into_format_args(megaelectronvolt, Abbreviation)
            )
        }
        EnergyUnit::Gigaelectronvolt => {
            println!(
                "Energy = {}",
                energy_quantity.into_format_args(gigaelectronvolt, Abbreviation)
            )
        }
        EnergyUnit::Joule => println!(
            "Energy = {}",
            energy_quantity.into_format_args(joule, Abbreviation)
        ),
    }
}
