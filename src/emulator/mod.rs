use rdev::{simulate, EventType, Key};
use std::{thread, time};

pub struct Emulator {}

impl Emulator {
  pub fn emulate(key: char) {
    // TODO:
    //  capital letters
    //  symbols
    match key {
      ' ' => { send(&EventType::KeyPress(Key::Space)) },
      'a' => { send(&EventType::KeyPress(Key::KeyA)) },
      'b' => { send(&EventType::KeyPress(Key::KeyB)) },
      'c' => { send(&EventType::KeyPress(Key::KeyC)) },
      'd' => { send(&EventType::KeyPress(Key::KeyD)) },
      'e' => { send(&EventType::KeyPress(Key::KeyE)) },
      'f' => { send(&EventType::KeyPress(Key::KeyF)) },
      'g' => { send(&EventType::KeyPress(Key::KeyG)) },
      'h' => { send(&EventType::KeyPress(Key::KeyH)) },
      'i' => { send(&EventType::KeyPress(Key::KeyI)) },
      'j' => { send(&EventType::KeyPress(Key::KeyJ)) },
      'k' => { send(&EventType::KeyPress(Key::KeyK)) },
      'l' => { send(&EventType::KeyPress(Key::KeyL)) },
      'm' => { send(&EventType::KeyPress(Key::KeyM)) },
      'n' => { send(&EventType::KeyPress(Key::KeyN)) },
      'o' => { send(&EventType::KeyPress(Key::KeyO)) },
      'p' => { send(&EventType::KeyPress(Key::KeyP)) },
      'q' => { send(&EventType::KeyPress(Key::KeyQ)) },
      'r' => { send(&EventType::KeyPress(Key::KeyR)) },
      's' => { send(&EventType::KeyPress(Key::KeyS)) },
      't' => { send(&EventType::KeyPress(Key::KeyT)) },
      'u' => { send(&EventType::KeyPress(Key::KeyU)) },
      'v' => { send(&EventType::KeyPress(Key::KeyV)) },
      'w' => { send(&EventType::KeyPress(Key::KeyW)) },
      'x' => { send(&EventType::KeyPress(Key::KeyX)) },
      'y' => { send(&EventType::KeyPress(Key::KeyY)) },
      'z' => { send(&EventType::KeyPress(Key::KeyZ)) },
      _ => {}
    }
  }

  pub fn backspace() {
    send(&EventType::KeyPress(Key::Backspace));
  }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {}
    }
    // Let ths OS catch up (at least MacOS)
    thread::sleep(delay);
}
