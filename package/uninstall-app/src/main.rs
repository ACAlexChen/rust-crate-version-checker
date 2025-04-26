use std::{fs, path::PathBuf};
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;
use std::collections::HashSet;

fn remove_from_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
  let hkcu = RegKey::predef(HKEY_CURRENT_USER);
  let environment = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

  let current_path: String = environment.get_value("Path")?;
  let mut path_set: HashSet<String> = current_path.split(';')
      .map(|p| p.to_string())
      .collect();

  let path_to_remove = Path::new(path).to_str().unwrap_or(path);
  path_set.remove(path_to_remove);

  let new_path = path_set.into_iter().collect::<Vec<_>>().join(";");
  environment.set_value("Path", &new_path)?;

  Ok(())
}

fn clean_up(dir: &PathBuf) -> Result<(), std::io::Error> {
  fs::remove_dir_all(dir)?;
  Ok(())
}

fn main() {
  let dir = std::env::current_dir().unwrap();
  remove_from_path(dir.to_str().unwrap()).unwrap();
  clean_up(&dir).unwrap();
}
