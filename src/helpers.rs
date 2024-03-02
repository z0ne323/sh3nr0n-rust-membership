use std::fs;
use std::io::{self, Read};
use reqwest::blocking::Response;
use reqwest::Error as ReqwestError;

pub fn file_exists(path: &str) -> bool {
    /*
    Description:
        Checks if a file or path exists.
    Parameters:
        path (&str): The path to the file or directory.
    Returns:
        bool: Returns true if the file or path exists, false otherwise.
    */
    fs::metadata(path).is_ok()
}


pub fn read_api_key(file_path: &str) -> Result<String, io::Error> {
    /*
    Description:
        Reads the content of a file and returns it as a String, representing an API key.
    Parameters:
        file_path (&str): The path to the file containing the API key.
    Returns:
        Result<String, std::io::Error>: 
            Ok(String): Returns the content of the file as a String.
            Err(std::io::Error): Returns an error if reading the file fails.
    */
    if !file_exists(file_path) {
        return Err(io::Error::new(io::ErrorKind::NotFound, "[-] File not found"));
    }

    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn handle_error_shodan(result: Result<Response, ReqwestError>) {
    /*
    Description:
        Handles errors specific to Shodan module, providing a centralized error handling mechanism.
    Parameters:
        result (Result<Response, ReqwestError>): The result of a Shodan module function call, containing either a successful response or an error.
    Returns:
        None: The function does not return a value. It prints relevant information about the response or error.
    */
    match result {
        Ok(response) => {
            // Handle successful response
            println!("Status: {}", response.status());
            let body = response.text().expect("Failed to read response body");
            println!("Body:\n{}", body);
        }
        Err(err) => {
            // Handle generic error
            eprintln!("Error: {:?}", err);
        }
    }
}