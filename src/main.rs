use ::hotkey as hotkeyExt;
use clap::{crate_authors, crate_version, App, Arg};
use cpal::traits::{DeviceTrait,  HostTrait};
use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Settings, Subscription,
    Text,
};

use std::path::{PathBuf};
use std::sync::mpsc::{Sender};
use std::thread::JoinHandle;

//use rodio;

mod config;
mod gui;
mod hotkey;
mod sound;

fn print_possible_devices() {
    let host = cpal::default_host();

    let devices = host.devices().expect("No available sound devices");

    println!("  Devices: ");
    for (device_index, device) in devices.enumerate() {
        println!("  {}. \"{}\"", device_index, device.name().unwrap());

        // Input configs
        if let Ok(conf) = device.default_input_format() {
            println!("    Default input stream format:\n      {:?}", conf);
        }

        // Output configs
        if let Ok(conf) = device.default_output_format() {
            println!("    Default output stream format:\n      {:?}", conf);
        }
    }
}

pub fn main() {
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

    if matches.is_present("print-possible-devices") {
        print_possible_devices();
        return;
    }

    let input_device_index: Option<usize> = {
        if matches.is_present("input-device") {
            Some(
                matches
                    .value_of("input-device")
                    .expect("No input device specified")
                    .parse()
                    .expect("No number specified"),
            )
        } else {
            None
        }
    };
    let output_device_index: Option<usize> = {
        if matches.is_present("output-device") {
            Some(
                matches
                    .value_of("output-device")
                    .expect("No ouput device specified")
                    .parse()
                    .expect("No number specified"),
            )
        } else {
            None
        }
    };
    let loop_device_index: usize = matches
        .value_of("loopback-device")
        .expect("No loopback device specified")
        .parse()
        .expect("No number specified");

    //Init Sound Module, get Sender to pass File Paths to
    let (tx, handle): (Sender<PathBuf>, JoinHandle<()>) =
        sound::init_sound(input_device_index, output_device_index, loop_device_index);

    std::thread::spawn(move || {
        let mut hk = hotkeyExt::Listener::new();
        hk.register_hotkey(hotkeyExt::modifiers::CONTROL, 'P' as u32, move || {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("resources/nicht-so-tief-rudiger.mp3");
            tx.send(file_path).unwrap();
        })
        .unwrap();

        hk.listen();
    });

    if matches.is_present("no-gui") {
        handle.join().expect("sound_thread join failed");
        return;
    }

    let mut settings = Settings::default();
    settings.window.size = (275, 150);
    Soundboard::run(settings);
}

#[derive(Default)]
struct Soundboard {
    play_sound_button: button::State,
    status_text: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    PlaySound,
}

impl Application for Soundboard {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Soundboard, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("soundboard")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlaySound => {
                self.status_text = "Start playing sound...".to_string();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(self.title()).size(32))
            .push(
                Button::new(&mut self.play_sound_button, Text::new("Play sound"))
                    .on_press(Message::PlaySound),
            )
            .padding(10)
            .push(Text::new(&self.status_text).size(10))
            .into()
    }
}
