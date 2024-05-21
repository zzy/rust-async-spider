mod sites;

#[tokio::main]
async fn main() {
    // this-week-in-rust.org
    match sites::this_week_in_rust_org::crawling().await {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching this-week-in-rust.org data."),
    }

    // lobste.rs
    match sites::lobste_rs::crawling().await {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching lobste.rs data."),
    }

    // datatau.net
    match sites::datatau_net::crawling().await {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching datatau.net data."),
    }
}
