/** Code written by Error/Metalblaze/Red-Lattice. Free to use however you like. */
use reqwest;
use std::fs;
use std::fs::{File, OpenOptions};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, StatusCode};
use std::io::{Write, BufReader, BufRead};
use std::time::{SystemTime, Duration};
use std::path::Path;
use std::io;

// Make this bigger for more funnies
const CHECK_AT_ONCE:usize = 200;

#[tokio::main]
async fn main()
{
    let now = SystemTime::now();
    check_for_assets_file();
    
    let start   = std::env::args().nth(1).expect("no start given");
    let end     = std::env::args().nth(2).expect("no amount given");

    let config_file = config_read();
    let first_line: String = BufReader::new(config_file).lines().next().unwrap_or(Ok("".to_string())).unwrap_or("".to_string());
    if first_line.len() > 0 {
        println!("Unfinished search detected. Would you like to resume? (Y/N)");

        if get_y_n_input() {
            get_range(first_line.parse::<u64>().unwrap(), end.parse().unwrap()).await;
        }
        else {
            get_range(start.parse().unwrap(), end.parse().unwrap()).await;
        }
    }
    else {
        get_range(start.parse().unwrap(), end.parse().unwrap()).await;
    }

    println!("\nCreations successfully gathered!");
    let _clear_file = config_init().set_len(0);
    println!("{:?}", now.elapsed().unwrap());
}

fn check_for_assets_file() {let _ = fs::create_dir_all("assets");}
fn config_init() -> File {OpenOptions::new().write(true).truncate(true).open("CONFIG.txt").unwrap()}
fn config_read() -> File {OpenOptions::new().read(true).open("CONFIG.txt").unwrap()}

/* Creates a new valid text file, based on the id */
fn manage_text_files(slice_1: String, slice_2: String, slice_3: String) -> File
{
    let file_name_string = format!("assets//{slice_1}//{slice_2}//{slice_3}.txt");
    let dir = format!("assets//{slice_1}//{slice_2}");

    if Path::new(&file_name_string).exists()
    {
        return OpenOptions::new().append(true).open(&file_name_string).unwrap();
    }
    let _ = fs::create_dir_all(dir);
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
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.blue/cyan}] {pos:>7}/{len:7} {msg} {percent}%   Estimated time remaining: {eta}")
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
                    let client = Client::builder().user_agent(APP_USER_AGENT).timeout(Duration::from_millis(15000)).build(); 
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
                        let client = Client::builder().user_agent(APP_USER_AGENT).timeout(Duration::from_millis(15000)).build();
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
                config_init().write_all(format!("{id}\n{id}").as_bytes()).unwrap();
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

fn get_y_n_input() -> bool
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed
    {
        "Y" => return true,
        "N" => return false,
        &_ => return failed_y_n_input(),
    };
}

fn failed_y_n_input() -> bool
{
    println!("\nPlease only enter Y or N");
    // If this hits recursion depth, it's user error at that point lmao
    return get_y_n_input();
}