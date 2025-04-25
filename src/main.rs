use reqwest::blocking::Client;
use serde::Deserialize;


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

fn main() {
  let args = std::env::args();
  let data: Vec<String> = args.collect();
  let mut args = vec![];
  for (i, str) in data.iter().enumerate() {
    if i != 0 {
      args.push(str.clone())
    }
  }
  let arg = args.join(" ");
  let crate_name = urlencoding::encode(&arg).into_owned();
  let url = format!("https://crates.io/api/v1/crates/{}", &crate_name);

  let client = Client::new();
  let response = client.get(url).header("User-Agent", "rust-crate-version-checker").send();
  match response {
    Ok(res_data) => {
      let data:Result<CrateResponse, reqwest::Error>  = res_data.json();
      match data {
        Ok(data) => {
          println!("{} 包的默认版本为：{}", &crate_name, &data.crate_info.default_version);
          println!("{} 包的最新版本为：{}", &crate_name, &data.crate_info.newest_version);
          println!("{} 包的最大版本为：{}", &crate_name, &data.crate_info.max_version);
          println!("{} 包的最大稳定版本为：{}", &crate_name, &data.crate_info.max_stable_version)
        },
        Err(err) => {
          println!("在解析数据时发生错误：\n{}", err)
        }
      }
    },
    Err(err) => {
      println!("在网络请求时发生错误：\n{}", err)
    }
  }
}
