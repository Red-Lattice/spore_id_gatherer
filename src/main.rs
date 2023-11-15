/** Code written by Error/Metalblaze/Red-Lattice. Free to use however you like. */
extern crate reqwest;
use std::{fs, io};
use std::fs::{File, OpenOptions};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, StatusCode};
use std::path::Path;
use std::io::Write;
use std::time::SystemTime;

// Make this bigger for more funnies
const CHECK_AT_ONCE:usize = 2000;

#[tokio::main]
async fn main() 
{
    let now = SystemTime::now();

    check_for_file();
    println!("\nWelcome to error/metalblaze/red lattice's id getter!\nPlease enter a starting ID to begin your range");

    let start = input_value();
    println!("\nHow many ID's after this would you like to search? (inclusive)");
    let end = input_value();
    get_range(start, end).await;

    println!("\nCreations successfully gathered!");

    println!("{:?}", now.elapsed().unwrap());
}

fn check_for_file() {let _ = fs::create_dir_all("assets");}

/* Creates a new valid text file, based on the id */
fn manage_text_files(slice_1: String, slice_2: String, slice_3: String) -> File
{
    let file_name_string = format!("assets//{slice_1}-{slice_2}-{slice_3}.txt");

    if Path::new(&file_name_string).exists()
    {
        return OpenOptions::new().append(true).open(&file_name_string).unwrap();
    }

    let _ = std::fs::File::create(&file_name_string).unwrap();
    return OpenOptions::new().append(true).open(&file_name_string).unwrap();
}

/* Gets the ids of creations within a range */
async fn get_range(start: u64, count: u64)
{
    
    let id_slice_1 = (start / 1000000000).to_string(); // First slice does not need to be cleaned as it always has a leading 5
    let id_slice_2 = clean_id((start / 1000000) % 1000);
    let id_slice_3 = clean_id((start / 1000) % 1000);
    let mut sub_id = (start / 1000) % 1000; // The 9 digits that make up the subids

    let end = start + count;
    let mut file = manage_text_files(id_slice_1, id_slice_2, id_slice_3);
    let bar = ProgressBar::new(count);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.blue/cyan} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    for i in (start..=end).step_by(CHECK_AT_ONCE)
    {
        let urls = (0..CHECK_AT_ONCE).map(|j| {
            let i = i + j as u64;
            let url = url_builder(i);
            (url, i)
        });
    
        let results = futures::future::join_all(urls.map(|(url, id)|
            async move 
                { 
                    let client = Client::builder().user_agent(APP_USER_AGENT).build(); 
                    (client.expect("REASON").head(url).send().await, id) 
                }
        )).await;
    
        for (result, id) in results.into_iter()
        {
            let result = match result {
                Ok(result) => result,
                Err(_) => 
                {
                    loop // If there was an error in fetching the request, it just retries until it works.
                    {
                        let client = Client::builder().user_agent(APP_USER_AGENT).build();
                        let url = url_builder(id);
                        let result = client.expect("REASON").head(url).send().await;
                        if let Ok(result) = result {
                            break result
                        }
                    }
                }
            };
            if (result).status() == StatusCode::OK
            {
                if sub_id != id % 1000
                {
                    file = manage_text_files((id / 1000000000).to_string(), 
                        clean_id((id / 1000000) % 1000), 
                        clean_id((id / 1000) % 1000));
                    sub_id = id % 1000;
                }
                file.write_all(format!("{id}\n").as_bytes()).unwrap();
            }
            bar.inc(1);
        }
    }
}

static APP_USER_AGENT: &str = "Sporepedia Archival Team | contact at: err.error.found@gmail.com";

/* Abstracted because it's ugly. This just builds the static url for a given creation. */
fn url_builder(id: u64) -> String
{
    let id_slice_1 = (id / 1000000000).to_string();
    let id_slice_2 = clean_id((id / 1000000) % 1000);
    let id_slice_3 = clean_id((id / 1000) % 1000);

    let url = "http://static.spore.com/static/thumb/".to_owned() + &id_slice_1 
        + "/" + &id_slice_2
        + "/" + &id_slice_3 
        + "/" + &id.to_string() + ".png";
    return url;
}

/* This gathers an input from the user and returns it as an integer */
fn input_value() -> u64
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse::<u64>()
    {
        Ok(i) =>  return i,
        Err(..) => println!("\nthis was not a valid ID: {}", trimmed),
    };
    return input_value();
}

/* This prevents leading zeros from being dropped, and also converts the 
   ints into strings for concatenation */
fn clean_id(input: u64) -> String
{
    if input > 99
    {
        return input.to_string();
    }
    if input > 9
    {
        return "0".to_owned() + &input.to_string();
    }
    return "00".to_owned() + &input.to_string();
}