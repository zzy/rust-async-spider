mod sites;

#[async_std::main]
async fn main() {
    match sites::hacker_news().await {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching Hacker News data."),
    }

    match sites::datatau().await {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching DataTau data."),
    }

    match sites::lobsters().await {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching DataTau data."),
    }
}
