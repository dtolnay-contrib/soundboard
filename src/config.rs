extern crate anyhow;
extern crate clap;
extern crate hotkey;
extern crate log;
extern crate serde;
extern crate toml;

use anyhow::{anyhow, Context, Result};
use clap::{crate_authors, crate_version, App, Arg};
use log::{error, info, trace, warn};
use regex;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use strum;
use strum_macros;

#[derive(Debug, Deserialize, Default, Clone, Serialize)]
pub struct Config {
  pub input_device: Option<usize>,
  pub output_device: Option<usize>,
  pub loopback_device: Option<usize>,
  pub stop_hotkey: Option<String>,
  pub sounds: Option<Vec<SoundConfig>>,
}

#[derive(Debug, Deserialize, Copy, Clone, Serialize, strum_macros::EnumString, PartialEq)]
pub enum Modifier {
  ALT = hotkey::modifiers::ALT as isize,
  CTRL = hotkey::modifiers::CONTROL as isize,
  SHIFT = hotkey::modifiers::SHIFT as isize,
  SUPER = hotkey::modifiers::SUPER as isize,
}

impl fmt::Display for Modifier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
  }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Copy, Clone, Serialize, strum_macros::EnumString, PartialEq)]
pub enum Key {
  BACKSPACE = hotkey::keys::BACKSPACE as isize,
  TAB = hotkey::keys::TAB as isize,
  ENTER = hotkey::keys::ENTER as isize,
  CAPS_LOCK = hotkey::keys::CAPS_LOCK as isize,
  ESCAPE = hotkey::keys::ESCAPE as isize,
  SPACEBAR = hotkey::keys::SPACEBAR as isize,
  PAGE_UP = hotkey::keys::PAGE_UP as isize,
  PAGE_DOWN = hotkey::keys::PAGE_DOWN as isize,
  END = hotkey::keys::END as isize,
  HOME = hotkey::keys::HOME as isize,
  ARROW_LEFT = hotkey::keys::ARROW_LEFT as isize,
  ARROW_RIGHT = hotkey::keys::ARROW_RIGHT as isize,
  ARROW_UP = hotkey::keys::ARROW_UP as isize,
  ARROW_DOWN = hotkey::keys::ARROW_DOWN as isize,
  PRINT_SCREEN = hotkey::keys::PRINT_SCREEN as isize,
  INSERT = hotkey::keys::INSERT as isize,
  DELETE = hotkey::keys::DELETE as isize,
  KEY_1 = '1' as isize,
  KEY_2 = '2' as isize,
  KEY_3 = '3' as isize,
  KEY_4 = '4' as isize,
  KEY_5 = '5' as isize,
  KEY_6 = '6' as isize,
  KEY_7 = '7' as isize,
  KEY_8 = '8' as isize,
  KEY_9 = '9' as isize,
  A = 'A' as isize,
  B = 'B' as isize,
  C = 'C' as isize,
  D = 'D' as isize,
  E = 'E' as isize,
  F = 'F' as isize,
  G = 'G' as isize,
  H = 'H' as isize,
  I = 'I' as isize,
  J = 'J' as isize,
  K = 'K' as isize,
  L = 'L' as isize,
  M = 'M' as isize,
  N = 'N' as isize,
  O = 'O' as isize,
  P = 'P' as isize,
  Q = 'Q' as isize,
  R = 'R' as isize,
  S = 'S' as isize,
  T = 'T' as isize,
  V = 'V' as isize,
  X = 'X' as isize,
  Y = 'Y' as isize,
  Z = 'Z' as isize,
}

impl fmt::Display for Key {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
  }
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct SoundConfig {
  pub name: String,
  pub path: String,
  pub hotkey: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct Hotkey {
  pub modifier: Vec<Modifier>,
  pub key: Key,
}

impl fmt::Display for Hotkey {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let modifier_string = self
      .modifier
      .clone()
      .into_iter()
      .fold(String::new(), |all, one| {
        if !all.is_empty() {
          format!("{}-{}", all, one)
        } else {
          one.to_string()
        }
      });
    let hotkey_string = {
      if !modifier_string.is_empty() {
        format!("{}-{}", modifier_string, self.key.to_string())
      } else {
        self.key.to_string()
      }
    };
    write!(f, "{}", hotkey_string)
  }
}

pub fn parse_hotkey(hotkey_string: &str) -> Result<Hotkey> {
  let re = regex::Regex::new(
    r"^(?i)(?:(CTRL|SHIFT|ALT|SUPER)-){0,1}(?:(CTRL|SHIFT|ALT|SUPER)-){0,1}(?:(CTRL|SHIFT|ALT|SUPER)-){0,1}(?:(CTRL|SHIFT|ALT|SUPER)-){0,1}(\w+)$",
  )?;
  let caps: regex::Captures = re
    .captures(hotkey_string)
    .ok_or(anyhow!("No valid hotkey match"))?;
  let mut modifier = Vec::new();
  let mut key: Option<Key> = None;
  for caps in caps.iter().skip(1) {
    if caps.is_some() {
      let mut mat = caps.unwrap().as_str().to_uppercase();
      if mat.parse::<usize>().is_ok() {
        mat = format!("KEY_{}", mat);
      }
      let modifier_try = Modifier::from_str(&mat);
      if modifier_try.is_ok() {
        modifier.push(modifier_try.unwrap());
        continue;
      }
      if key.is_some() {
        return Err(anyhow!("hotkey has alread a key specified"));
      }
      let key_try = Key::from_str(&mat);
      if key_try.is_ok() {
        key = Some(key_try.unwrap());
      }
    }
  }
  if key.is_none() {
    return Err(anyhow!("hotkey has no key specified"));
  }
  Ok(Hotkey {
    modifier: modifier,
    key: key.unwrap(),
  })
}

pub fn load_and_parse_config(name: &str) -> Result<Config> {
  let mut path = std::env::current_exe()?;
  path.pop();
  path.push(name);
  let toml_str = fs::read_to_string(&path)?;
  let toml_config = toml::from_str(&toml_str)?;
  info!("Loaded config file from {}", path.display());
  Ok(toml_config)
}

pub fn save_config(config: Config, name: &str) -> Result<()> {
  let mut path = std::env::current_exe()?;
  path.pop();
  path.push(name);

  let pretty_string = toml::to_string_pretty(&config)?;
  fs::write(&path, pretty_string)?;
  info!("Saved config file at {}", &path.display());
  Ok(())
}

pub fn parse_arguments() -> clap::ArgMatches {
  let matches = App::new("soundboard")
    .version(crate_version!())
    .author(crate_authors!())
    .about("play sounds over your microphone")
    .arg(
      Arg::with_name("config-file")
        .short('c')
        .long("config")
        .value_name("FILE")
        .default_value("soundboard.toml")
        .about("sets a custom config file")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("input-device")
        .short('i')
        .long("input-device")
        .about("Sets the input device to use")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("output-device")
        .short('o')
        .long("output-device")
        .about("Sets the output device to use")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("loopback-device")
        .short('l')
        .long("loopback-device")
        .about("Sets the loopback device to use")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("verbose")
        .long("verbose")
        .takes_value(true)
        .about("Sets the level of verbosity"),
    )
    .arg(
      Arg::with_name("print-possible-devices")
        .long("print-possible-devices")
        .about("Print possible devices"),
    )
    .arg(Arg::with_name("no-gui").long("no-gui").about("Disable GUI"))
    .get_matches();

  matches
}

pub fn parse_devices(
  config: &Config,
  arguments: &clap::ArgMatches,
) -> Result<(Option<usize>, Option<usize>, usize)> {
  let input_device_index: Option<usize> = {
    if arguments.is_present("input-device") {
      Some(
        arguments
          .value_of("input-device")
          .expect("No input device specified")
          .parse()
          .expect("No number specified"),
      )
    } else if config.input_device.is_some() {
      config.input_device
    } else {
      None
    }
  };
  let output_device_index: Option<usize> = {
    if arguments.is_present("output-device") {
      Some(
        arguments
          .value_of("output-device")
          .expect("No ouput device specified")
          .parse()
          .expect("No number specified"),
      )
    } else if config.output_device.is_some() {
      config.output_device
    } else {
      None
    }
  };

  let loop_device_index: usize = {
    if arguments.is_present("loopback-device") {
      arguments
        .value_of("loopback-device")
        .expect("No loopback device specified")
        .parse()
        .expect("No number specified")
    } else if config.loopback_device.is_some() {
      config.loopback_device.unwrap()
    } else {
      return Err(anyhow!(
        "No loopback device specified in config or on command line"
      ));
    }
  };

  Ok((input_device_index, output_device_index, loop_device_index))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hotkey_parse() {
    assert_eq!(
      parse_hotkey("CTRL-P").unwrap(),
      Hotkey {
        modifier: vec![Modifier::CTRL],
        key: Key::P
      }
    );
    assert_eq!(
      parse_hotkey("CTRL-SHIFT-P").unwrap(),
      Hotkey {
        modifier: vec![Modifier::CTRL, Modifier::SHIFT],
        key: Key::P
      }
    );
    assert_eq!(
      parse_hotkey("S").unwrap(),
      Hotkey {
        modifier: vec![],
        key: Key::S
      }
    );
    assert_eq!(
      parse_hotkey("ALT-BACKSPACE").unwrap(),
      Hotkey {
        modifier: vec![Modifier::ALT],
        key: Key::BACKSPACE
      }
    );
    assert_eq!(
      parse_hotkey("SHIFT-SUPER-A").unwrap(),
      Hotkey {
        modifier: vec![Modifier::SHIFT, Modifier::SUPER],
        key: Key::A
      }
    );
    assert_eq!(
      parse_hotkey("SUPER-ARROW_RIGHT").unwrap(),
      Hotkey {
        modifier: vec![Modifier::SUPER],
        key: Key::ARROW_RIGHT
      }
    );
    assert_eq!(
      parse_hotkey("SUPER-CTRL-SHIFT-ALT-9").unwrap(),
      Hotkey {
        modifier: vec![
          Modifier::SUPER,
          Modifier::CTRL,
          Modifier::SHIFT,
          Modifier::ALT
        ],
        key: Key::KEY_9
      }
    );
    assert_eq!(
      parse_hotkey("super-ctrl-SHIFT-alt-ARROW_Up").unwrap(),
      Hotkey {
        modifier: vec![
          Modifier::SUPER,
          Modifier::CTRL,
          Modifier::SHIFT,
          Modifier::ALT
        ],
        key: Key::ARROW_UP
      }
    );

    assert_eq!(
      parse_hotkey("5").unwrap(),
      Hotkey {
        modifier: vec![
        ],
        key: Key::KEY_5
      }
    );

    assert_eq!(
      parse_hotkey("KEY_5").unwrap(),
      Hotkey {
        modifier: vec![
        ],
        key: Key::KEY_5
      }
    );

    assert_eq!(
      parse_hotkey("5-5").unwrap_err().to_string(),
      "No valid hotkey match"
    );

    assert_eq!(
      parse_hotkey("CTRL-").unwrap_err().to_string(),
      "No valid hotkey match"
    );

    assert_eq!(
      parse_hotkey("").unwrap_err().to_string(),
      "No valid hotkey match"
    );
  }
}
