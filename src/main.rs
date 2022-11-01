use std::sync::mpsc;
use std::thread;

use rdev::{listen, Event, EventType};

mod emulator;
mod replacement;

use emulator::Emulator;
use replacement::{Replacement, Replacements};

fn main() {
  let mut replacements: Vec<Replacement> = vec![];

  if Replacements::file_path().exists() {
    replacements = Replacements::load();
    println!("{:?}", replacements);
  } else {
    // if file does not exist, write sample file
    Replacements::write_sample_file();
    println!("Created replacements file. Go make edits. {}", Replacements::filename());
    std::process::exit(0);
  }

  let mut presses: Vec<String> = vec![];

  let mut handles = vec![];
  let (tx, rx) = mpsc::channel();

  handles.push(
    thread::spawn(move || {
      let callback = move |event: Event| {
        match event.event_type {
          EventType::KeyPress(_key) => {
            let tx = tx.clone();

            // Send the event and handle any errors from sending
            if let Err(err) = tx.send(event) {
              println!("error sending event: {}", err);
            };
          },
          _ => { } // Ignore all other events
        }
      };

      if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
      }
    })
  );

  handles.push(
    thread::spawn(move || {
      for event in rx {
        match event.name {
          Some(key) => {

            for replacement in &mut replacements {
              let matches_idx = replacement.matches.len();

              let mut temp_match_char_str = String::from("");
              let chars: Vec<char> = replacement.from.chars().collect();
              let char = chars[matches_idx];
              temp_match_char_str.push(char);

              if temp_match_char_str == key {
                replacement.matches.push_str(&key);
                // println!("{} matches {}", temp_match_char_str, &replacement.matches);

                if replacement.matches.clone() == replacement.from.clone() {
                  // println!("Matched entire replacement!: {}", &replacement.from);

                  // Remove typed letters from `replacement.from`
                  for _ in 0..=replacement.from.len() {
                    Emulator::backspace();
                  }

                  // Emulate letters from `replacement.to`
                  for key in replacement.to.chars() {
                    Emulator::emulate(key);
                  }

                  // reset logic
                  replacement.clear();
                  break;
                }
              } else {
                replacement.clear();
              }
            }
          },
          _ => {}
        };
      }
    })
  );

  for handle in handles {
    handle.join().unwrap()
  }

}
