use reqwest;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubDomain {
    #[serde(rename = "issuer_ca_id")]
    pub issuer_ca_id: i64,
    #[serde(rename = "issuer_name")]
    pub issuer_name: String,
    #[serde(rename = "common_name")]
    pub common_name: String,
    #[serde(rename = "name_value")]
    pub name_value: String,
    pub id: i64,
    #[serde(rename = "entry_timestamp")]
    pub entry_timestamp: String,
    #[serde(rename = "not_before")]
    pub not_before: String,
    #[serde(rename = "not_after")]
    pub not_after: String,
    #[serde(rename = "serial_number")]
    pub serial_number: String,
}

pub type SubDomains = Vec<SubDomain>;

#[tokio::main]
async fn main() {
    let mut maindomain = String::new();
    let stdin = io::stdin();
    println!("Enter domain : ");
    stdin
        .read_line(&mut maindomain)
        .expect("Error in reading from console");
    let subdomainsearch = format!("https://crt.sh/?q={}&output=json", maindomain);
    getsubdomains(&subdomainsearch).await;
}

async fn getsubdomains(subdomainsearch: &str) {
    let client = reqwest::Client::new();
    let resp = client
        .get(subdomainsearch)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)",
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    /*let txtres = &resp.text().await;
    match txtres {
        Ok(got) => println!("{}", got),
        Err(err) => println!("Error"),
    }*/
    let jsonres = resp.json::<SubDomains>().await;
    match jsonres {
        Ok(gotlist) => {
            println!("Sub Dommains");
            println!("_________________");
            for subdomain in gotlist {
                println!("{}", subdomain.common_name);
            }
        }
        Err(err) => println!("Something went wrong {}", err),
    };
}
