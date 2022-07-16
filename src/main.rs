use chrono::{Datelike, Local, TimeZone, Timelike};
use nix::unistd::Uid;
use std::process::Command;
use std::{
    env,
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use surf::Url;
use surf::{Client, Config};
use tide::Request;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        std::process::exit(0);
    }
    if !args.contains(&"--serve".to_string()) {
        if !Uid::effective().is_root() {
            eprintln!("You must run this executable with root permissions");
            std::process::exit(1);
        }
        let client: Client = Config::new()
            .set_base_url(
                Url::parse(format!("http://{}", args[1]).as_str()).expect("invalid server"),
            )
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()?;
        let result = match client.get("/").await?.body_string().await?.parse() {
            Ok(v) => v,
            Err(_) => {
                eprint!("Invalid response from server");
                std::process::exit(1);
            }
        };
        let dt = Local.timestamp_millis(result);
        let date_string = format!(
            "{}-{}-{} {}:{}:{}",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second()
        );
        Command::new("date").args(["--set", &date_string]).spawn()?;
        std::process::exit(0);
    }
    if args.len() < 3 {
        eprintln!("missing port");
        std::process::exit(1);
    }
    let mut app = tide::new();
    app.at("/").get(current_time);
    app.listen(format!(
        "0.0.0.0:{}",
        args[2].parse::<u16>().expect("invalid port")
    ))
    .await?;
    Ok(())
}

async fn current_time(_: Request<()>) -> tide::Result {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_millis()
        .to_string()
        .into())
}
