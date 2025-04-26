#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::Ui;
use rfd::FileDialog;
use winreg::{enums::*, RegKey};
use std::{io::Write, path::PathBuf, sync::Arc};

struct Installer {
  install_path: String,
  index: usize,
  show_error: bool,
  error_message: String
}
impl Default for Installer {
  fn default() -> Self {
    Self {
      install_path: String::from("C:\\Program Files\\rust-crate-version-checker"),
      index: 0,
      show_error: false,
      error_message: String::new()
    }
  }
}

impl Installer {
  fn show_error(&mut self, msg: &str) {
    self.error_message = String::from(msg);
    self.show_error = true;
  }

  fn show_error_window(&mut self, ctx: &egui::Context) {
    if self.show_error {
      egui::Window::new("错误")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .fixed_size(egui::vec2(300.0, 100.0))
        .show(ctx, |ui| {
          ui.add_space(10.0);
          ui.label(&self.error_message);
          ui.add_space(10.0);
          if ui.button("确定").clicked() {
            self.show_error = false
          }
        });
    }
  }
}

impl Installer {
  fn zero_page(&mut self, ui: &mut Ui) {
    ui.horizontal(|ui| {
      ui.label("安装路径：");
      ui.text_edit_singleline(&mut self.install_path);
      if ui.button("选择文件夹").clicked() {
        if let Some(path_buf) = FileDialog::new().set_title("请选择文件夹").pick_folder() {
          let mut path_buf = PathBuf::from(path_buf.display().to_string());
          path_buf.push("rust-crate-version-checker");
          self.install_path = path_buf.to_str().unwrap().to_string();
        }
      }
    });
    if ui.button("下一步").clicked() {
      if self.install_path.is_empty() {
        self.show_error("请选择文件夹！")
      } else {
        self.index = 1
      }
    }
  }

  fn one_page(&mut self, ui: &mut Ui) {
    if ui.button("上一步").clicked() {
      self.index = 0
    }
    if ui.button("安装").clicked() {
      let path = PathBuf::from(&self.install_path);
      if path.exists() {
        self.show_error("文件夹已存在，请选择其他文件夹！");
      } else {
        match std::fs::create_dir_all(&path) {
          Ok(_) => {
            if let Err(e) = install_app(path.clone()) {
              self.show_error(&format!("安装失败：{}", e));
            } else {
              if let Err(e) = add_to_path(path.to_str().unwrap()) {
                self.show_error(&format!("添加到环境变量失败：{}", e));
              }
              self.index = 2;
            }
          }
          Err(e) => self.show_error(&format!("创建文件夹失败：{}", e))
        }
      }
    }
  }

  fn two_page(&mut self, ui: &mut Ui) {
    ui.label("安装完成！");
    if ui.button("完成").clicked() {
      std::process::exit(0);
    }
  }

}

impl eframe::App for Installer {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    self.show_error_window(ctx);
    egui::CentralPanel::default().show(ctx, |ui| {
      match self.index {
        0 => self.zero_page(ui),
        1 => self.one_page(ui),
        2 => self.two_page(ui),
        _ => panic!("Invalid index")
      }
    });
  }
}

const APP: &'static [u8] = include_bytes!("../../../target/release/rust-crate-version-checker.exe");
const UNINSTALLER: &'static [u8] = include_bytes!("../../../target/release/uninstall-app.exe");

fn install_app(path: PathBuf) -> Result<(), std::io::Error> {
  let mut file = std::fs::File::create(path.join("rcvc.exe"))?;
  file.write_all(APP)?;
  let mut file = std::fs::File::create(path.join("rcvc-uninstall.exe"))?;
  file.write_all(UNINSTALLER)?;
  Ok(())
}

fn add_to_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
  let hkcu = RegKey::predef(HKEY_CURRENT_USER);
  let environment = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
  let mut current_path: String = environment.get_value("Path")?;
  if !current_path.ends_with(";") {
    current_path.push(';');
  }
  current_path.push_str(path);
  environment.set_value("Path", &current_path)?;
  Ok(())
}

fn set_font(cc: &eframe::CreationContext) {
  let mut fonts = egui::FontDefinitions::default();
  fonts.font_data.insert(
    "microsoft_yahei".to_string(),
    Arc::new(egui::FontData::from_static(include_bytes!("../msyh.ttc")))
  );

  fonts.families
    .entry(egui::FontFamily::Proportional)
    .or_default()
    .insert(0, "microsoft_yahei".to_string());

  fonts.families
    .entry(egui::FontFamily::Monospace)
    .or_default()
    .push("microsoft_yahei".to_string());

  cc.egui_ctx.set_fonts(fonts);
}

fn show_window() -> Result<(), eframe::Error> {
  let native_options = eframe::NativeOptions {
    ..Default::default()
  };

  eframe::run_native(
    "Installer",
    native_options,
    Box::new(
      |cc| {
        set_font(cc);

        Ok(Box::new(Installer::default()))
      }
    )
  )?;

  Ok(())
}

fn main() {
  show_window().unwrap();
}
