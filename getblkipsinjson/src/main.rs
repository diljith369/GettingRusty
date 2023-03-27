use reqwest;
use serde::{Serialize, Deserialize}; 
//struct json to rust : https://transform.tools/json-to-rust-serde
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IPBlockList {
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    pub port: i64,
    pub status: String,
    pub hostname: Option<String>,
    #[serde(rename = "as_number")]
    pub as_number: i64,
    #[serde(rename = "as_name")]
    pub as_name: String,
    pub country: String,
    #[serde(rename = "first_seen")]
    pub first_seen: String,
    #[serde(rename = "last_online")]
    pub last_online: String,
    pub malware: String,
}
pub type IPBlockLists = Vec<IPBlockList>;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let target = "https://feodotracker.abuse.ch/downloads/ipblocklist_recommended.json";
    let resp = client
    .get(target)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246")
    .header("Content-Type", "application/json")    
    .send()
    .await
    .unwrap();
    
    let jsonres = resp
                .json::<IPBlockLists>()
                .await
                .unwrap();

    
    for ipblocklist in jsonres {
        println!("IP : {} ", ipblocklist.ip_address);
        match ipblocklist.hostname {
            Some(hostname) =>   println!("Host Name : {} ", hostname),
            None =>  println!("Host Name Missing"),
        }

    }   

}
