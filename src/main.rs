mod sites;

#[async_std::main]
async fn main() {

    // this-week-in-rust.org
    match sites::this_week_in_rust_org().await {
        Ok(site) => println!("{}", site),
        Err(_) => eprintln!("Error fetching this-week-in-rust.org data."),
    }

    // 其它站点
    // ……
}
