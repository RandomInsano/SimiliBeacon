use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{process, thread::sleep, time::Duration};

fn main() {
    
    let m: i8 = 60; // Default time in minutes to wait between each URL Request
    let uri = vec!["CONTRIBUTING.md", "CodeQL.yml", "LICENSE.txt", "README.md", "SECURITY.md", "cglicenses.json", "cgmanifest.json", "gulpfile.js", "package.json", "product.json"];
    let mut x: i8 = 0; // count the number of itteration to the loop
    let mut y: i8 = thread_rng().gen_range(1,11); // Number of loop to do before changing the URI
    let fqdn = "https://github.com/microsoft/vscode/".to_string().to_owned(); // Domain we use for our C2 config
    let mut url = fqdn.clone() + uri.choose(&mut rand::thread_rng()).unwrap(); // Full URL where to grab the C2 instructions

    loop {
        let jit: f32 = thread_rng().gen_range(0.0,2.0);
        let ttw8: u64 = (m as f32 * 60.0 * jit) as u64; // Time to wait in minutes
        let minutes = Duration::from_secs(ttw8.into());

        println!("Minutes to wait: {} URL to Fetch: {}", ttw8/60, url);
        sleep(minutes);
        if reqwest::blocking::get(url.clone()).is_err()  {
            println!("Request failed. Unable to get {url}");
            process::exit(1);
        }

        x+=1;

        if x == y {
            y = thread_rng().gen_range(1,11);
            x = 0;
            url = fqdn.clone() + uri.choose(&mut rand::thread_rng()).unwrap();
        }
    }
}
