use serde::Deserialize;
use std::error::Error;

// added imports
use std::fmt; // for custom error display
use std::fs::{self, File}; // for creating folders and files
use std::io::copy; // for copying downloaded data into files

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

#[derive(Debug)] // custom error type for downloading images
enum DownloadError {
    Network(String),
    File(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DownloadError::Network(e) => write!(f, "Network error: {}", e),
            DownloadError::File(e) => write!(f, "File error: {}", e),
        }
    }
}

impl Error for DownloadError {} // allows DownloadError to work with ? and Result

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

// downloads the image from the URL and saves it locally
fn download_image(url: &str, filename: &str) -> Result<(), DownloadError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| DownloadError::Network(e.to_string()))?;
    let mut reader = response.into_reader();
    let mut file = File::create(filename)
        .map_err(|e| DownloadError::File(e.to_string()))?;
    copy(&mut reader, &mut file)
        .map_err(|e| DownloadError::File(e.to_string()))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    fs::create_dir_all("images")?; // creates the images folder

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                // save images to images folder
                let filename = format!("images/dog_{}.jpg", i); 
                match download_image(&dog_image.message, &filename) {
                    Ok(_) => println!("💾 Saved to: {}", filename),
                    Err(e) => println!("❌ Failed to download image: {}", e),
                }
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}
