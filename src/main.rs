#![allow(unused_imports)]

extern crate clap;
extern crate cpal;
extern crate iced;
extern crate log;

extern crate strum;
#[macro_use]
extern crate strum_macros;


use anyhow::{anyhow, Context, Result};
use log::{error, info, trace, warn};

use ::hotkey as hotkeyExt;
use clap::{crate_authors, crate_version, App, Arg};
use cpal::traits::{DeviceTrait, HostTrait};
use iced::{
  button, executor, Align, Application, Button, Column, Command, Container, Element, Length, Row,
  Settings, Subscription, Text,
};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod config;
mod download;
mod gui;
mod hotkey;
mod sound;
mod utils;

pub fn main() -> Result<()> {
  env_logger::builder()
    .filter_module("soundboard", log::LevelFilter::Info)
    .init();
  info!("Parsing arguments");
  let arguments = config::parse_arguments();

  if arguments.is_present("print-possible-devices") {
    sound::print_possible_devices();
    return Ok(());
  }

  let config_file = config::load_and_parse_config(arguments.value_of("config-file").unwrap())?;
  // println!("{:#?}", config_file);

  let (tx, rx): (Sender<sound::Message>, Receiver<sound::Message>) = mpsc::channel();

  let (input_device_index, output_device_index, loop_device_index) =
    config::parse_devices(&config_file, &arguments)?;

  std::thread::spawn(move || -> Result<()> {
    sound::init_sound(
      rx,
      input_device_index,
      output_device_index,
      loop_device_index,
    )
  });

  let tx_clone = tx.clone();
  let hotkey_thread = std::thread::spawn(move || -> Result<()> {
    let mut hk = hotkeyExt::Listener::new();

    let tx_clone_1 = tx_clone.clone();
    let stop_hotkey = {
      if config_file.stop_hotkey.is_some() {
        config::parse_hotkey(&config_file.stop_hotkey.as_ref().unwrap())?
      } else {
        config::Hotkey {
          modifier: vec![config::Modifier::CTRL],
          key: config::Key::S
        }
      }
    };
    hk.register_hotkey(stop_hotkey.modifier.iter().fold(0, |acc, x| acc | (*x as u32)) as u32, stop_hotkey.key as u32, move || {
      let _result = tx_clone_1.send(sound::Message::StopAll);
    })
    .map_err(|_s| anyhow!("register key"))?;

    let tx_clone = tx_clone.clone();
    for sound in config_file.sounds.unwrap_or_default() {
      if sound.hotkey.is_none() {
        continue;
      }
      let hotkey = config::parse_hotkey(&sound.hotkey.as_ref().unwrap())?;
      let tx_clone = tx_clone.clone();
      let _result = hk
        .register_hotkey(
          hotkey.modifier.iter().fold(0, |acc, x| acc | (*x as u32)) as u32,
          hotkey.key as u32,
          move || {
            if let Err(err) = tx_clone.send(sound::Message::PlaySound(
              sound.path.clone(),
              sound::SoundDevices::Both,
            )) {
              error!("failed to play sound {}", err);
            };
          },
        )
        .map_err(|_s| anyhow!("register key"));
    }
    hk.listen();
    Ok(())
  });

  if arguments.is_present("no-gui") {
    let _result = hotkey_thread.join().expect("sound thread join failed");
    return Ok(());
  }

  let config_file = config::load_and_parse_config(arguments.value_of("config-file").unwrap())?;
  let tx_clone = tx;
  let mut settings = Settings::with_flags((tx_clone, config_file));
  settings.window.size = (500, 350);
  gui::Soundboard::run(settings);
  Ok(())
}
