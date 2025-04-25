use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize)]
struct CrateResponse {
  #[serde(rename = "crate")]
  crate_info: CrateInfo
}

#[derive(Debug, Deserialize)]
struct CrateInfo {
  max_version: String,
  newest_version: String,
  default_version: String,
  max_stable_version: String
}

fn get_crate_info(name: &str) -> Result<CrateResponse> {
  let encoded_name = urlencoding::encode(name).into_owned();
  let url = format!("https://crates.io/api/v1/crates/{}", encoded_name);

  let client = Client::new();
  let response = client
    .get(url)
    .header("User-Agent", "rust-crate-version-checker")
    .send()?
    .json()?;

  Ok(response)
}

fn print_crate_info(name: &str, info: &CrateInfo) {
  println!("{} 包的版本信息:", name);
  println!("- 默认版本：{}", info.default_version);
  println!("- 最新版本：{}", info.newest_version);
  println!("- 最大版本：{}", info.max_version);
  println!("- 最大稳定版本：{}", info.max_stable_version);
}

fn main() {
  let crate_name = std::env::args()
    .skip(1)
    .collect::<Vec<String>>()
    .join(" ");

  if crate_name.is_empty() {
    eprintln!("请提供包名！");
    std::process::exit(1);
  }

  match get_crate_info(&crate_name) {
    Ok(response) => print_crate_info(&crate_name, &response.crate_info),
    Err(err) => eprintln!("错误: {}", err)
  }
}

