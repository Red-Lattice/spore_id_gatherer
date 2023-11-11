extern crate reqwest;
use std::fs;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use reqwest::Client;
use std::time::SystemTime;

// Make this bigger for more funnies
const CHECK_AT_ONCE:usize = 100;

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
fn manage_text_files() -> File
{
    let mut million_set = 1;
    let mut file_name_string = format!("id_pile//million_{million_set}.txt");
    while Path::new(&file_name_string).exists()
    {
        million_set += 1;
        file_name_string = format!("id_pile//million_{million_set}.txt");
    }
    return std::fs::File::create(&file_name_string).unwrap();
}

async fn get_range(start: u64, count: u64)
{
    let end = start + count;
    let mut line_count:u64 = 0;
    let mut file = manage_text_files();

    for i in (start..=end).step_by(CHECK_AT_ONCE)
    {
        if line_count > 1000000
        {
            file = manage_text_files();
            line_count = 0;
        }

        let urls = (0..CHECK_AT_ONCE).map(|j| {
            let i = i + j as u64;
            let id_slice_1 = (i / 1000000000).to_string();
            let id_slice_2 = clean_id((i / 1000000) % 1000);
            let id_slice_3 = clean_id((i / 1000) % 1000);

            let url = "http://static.spore.com/static/thumb/".to_owned() + &id_slice_1 
                + "/" + &id_slice_2
                + "/" + &id_slice_3 
                + "/" + &i.to_string() + ".png";
            (url, i)
        });
    
        let results = futures::future::join_all(urls.map(|(url, id)|
            async move { 
                let client = Client::new(); (client.get(url).send().await, id) }))
        .await;
    
        for (result, id) in results.into_iter() {
            // If a png is too small, it gets deleted because it's not a real creation
            if result.unwrap().content_length().unwrap() > 500
            {
                file.write_all(format!("{id}\n").as_bytes()).unwrap();
                line_count += 1;
                //println!("{id}");
            }
        }
    }
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