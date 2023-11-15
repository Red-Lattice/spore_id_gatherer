extern crate reqwest;
use std::fs;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use reqwest::Client;
use std::time::SystemTime;
use reqwest::StatusCode;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::fs::OpenOptions;

// Make this bigger for more funnies
const CHECK_AT_ONCE:usize = 2000;

#[tokio::main]
async fn main() 
{
    let now = SystemTime::now();

    check_for_file();
    println!("\nWelcome to error/metalblaze/red lattice's id getter!");
    run().await;

    println!("{:?}", now.elapsed().unwrap());
}

async fn run()
{
    println!("\nPlease enter a starting ID to begin your range");
    let start = input_value();
    println!("\nHow many ID's after this would you like to search? (inclusive)");
    let end = input_value();
    get_range(start, end).await;
    println!("\nCreations successfully gathered!");
    return;
}

fn check_for_file() {let _ = fs::create_dir_all("id_pile");}

/* Creates a new valid text file, based on million line counts. */
fn manage_text_files(slice_1: String, slice_2: String, slice_3: String) -> File
{
    let file_name_string = format!("id_pile//{slice_1}-{slice_2}-{slice_3}.txt");
    if Path::new(&file_name_string).exists()
    {
        return OpenOptions::new().write(true).open(&file_name_string).unwrap();
    }
    let _ = std::fs::File::create(&file_name_string).unwrap();
    return OpenOptions::new().write(true).open(&file_name_string).unwrap();
}

async fn get_range(start: u64, count: u64)
{
    let id_slice_1 = (start / 1000000000).to_string();
    let id_slice_2 = clean_id((start / 1000000) % 1000);
    let id_slice_3 = clean_id((start / 1000) % 1000);

    let end = start + count;
    let mut line_count:u64 = 0;
    let mut file = manage_text_files(id_slice_1, id_slice_2, id_slice_3);
    let bar = ProgressBar::new(count);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.blue/cyan} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    for i in (start..=end).step_by(CHECK_AT_ONCE)
    {
        let id_slice_1 = (i / 1000000000).to_string();
        let id_slice_2 = clean_id((i / 1000000) % 1000);
        let id_slice_3 = clean_id((i / 1000) % 1000);

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
                if line_count > 1000 // I want it to break up the ID's into chunks of 1 million.
                {
                    file = manage_text_files(id_slice_1.clone(), id_slice_2.clone(), id_slice_3.clone());
                    line_count = 0;
                }
                file.write_all(format!("{id}\n").as_bytes()).unwrap();
            }
            line_count += 1;
            bar.inc(1);
        }
    }
}

static APP_USER_AGENT: &str = "Sporepedia Archival Team | contact at: err.error.found@gmail.com";

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
    return 500000000000;
}

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