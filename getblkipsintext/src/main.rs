use reqwest;

#[tokio::main]
async fn main() {
    println!("Entering the rusty world");

    let target = "https://feodotracker.abuse.ch/downloads/ipblocklist_recommended.txt";
    getblacklistedipsintext(target)
    .await;
}

async fn getblacklistedipsintext(target: &str) {
    let client = reqwest::Client::new();
    let resp = client
    .get(target)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246")
    .send()
    .await
    .unwrap()
    .text()
    .await;
match  resp {
    
    Ok(res) => {
        let allips: Vec<&str> = res.split("\n").collect();
        for ip in allips {
            if ! ip.starts_with("#") {
                println!("{}",ip);
            }
        }
    },
    _=> println!("Something went wrong"),
}

}

