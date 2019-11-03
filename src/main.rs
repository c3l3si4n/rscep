extern crate clap;
extern crate reqwest;
extern crate serde_json;

use std::io::Read;

use clap::{App, Arg};
use reqwest::Response;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    // Tool Info, Arguments and Data
    let matches = App::new("RustCep")
        .version("0.1.0")
        .author("Daniel Matsumoto <insidetf2@gmail.com>")
        .about("Makes CEP (brazilian zip code) consults with Rust!")
        .arg(
            Arg::with_name("cep")
                .short("c")
                .long("cep")
                .required(true)
                .takes_value(true)
                .help("CEP code to consult"),
        )
        .get_matches();

    // Gets the cep argument
    let cep_argument = matches.value_of("cep").unwrap();

    // Input Validation

    // Checks if input is numeric
    let test = cep_argument.parse::<i64>();
    match test {
        Err(_e) => {
            println!("The cep needs to be numeric.");
            std::process::exit(1);
        }
        _ => {}
    }

    // Checks if input has correct length
    if cep_argument.chars().count() != 8 {
        println!("The cep needs to have 8 digits.");
        std::process::exit(1);
    }

    // Makes a request to our API
    let uri = format!("https://viacep.com.br/ws/{}/json/", cep_argument);

    // Captures response object
    let mut res: Response = reqwest::get(&uri[..])?;

    // Initializes and assings the API's response body to the body variable
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // Makes Javascript Object from the response data
    let json: serde_json::Value = serde_json::from_str(&body[..])?;


    // Parses JSON to it's variables
    let cep_code = json["cep"].as_str().unwrap();
    let formatted_address = json["logradouro"].as_str().unwrap();
    let additional = json["complemento"].as_str().unwrap();
    let neighborhood = json["bairro"].as_str().unwrap();
    let city = json["localidade"].as_str().unwrap();
    let state = json["uf"].as_str().unwrap();
    let ibge = json["ibge"].as_str().unwrap();
    let gia = json["gia"].as_str().unwrap();


    println!("CEP: {}", cep_code);
    println!("Address: {}", formatted_address);
    println!("Neighborhood: {}", neighborhood);
    println!("Additional: {}", additional);

    println!("City: {}", city);
    println!("State: {}", state);
    println!("IBGE: {}", ibge);
    println!("GIA: {}", gia);

    Ok(())
}
