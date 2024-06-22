use anyhow::Result;
use core::convert::TryFrom;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};

use super::browser;

pub fn prepare_input() -> Result<KeyState> {
    let (tx, rx) = unbounded();
    let keydown_tx = Rc::new(RefCell::new(tx));
    let keyup_tx = Rc::clone(&keydown_tx);
    let on_keydown = Closure::wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        if let Ok(key) = Key::try_from(keycode.code().as_str()) {
            let _ = keydown_tx.borrow_mut().start_send(KeyPress::KeyDown(key));
        }
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    let on_keyup = Closure::wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        if let Ok(key) = Key::try_from(keycode.code().as_str()) {
            let _ = keyup_tx.borrow_mut().start_send(KeyPress::KeyUp(key));
        }
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    let window = browser::window()?;
    window.set_onkeydown(Some(on_keydown.as_ref().unchecked_ref()));
    window.set_onkeyup(Some(on_keyup.as_ref().unchecked_ref()));
    on_keydown.forget();
    on_keyup.forget();
    Ok(KeyState::new(rx))
}

pub struct KeyState {
    receiver: UnboundedReceiver<KeyPress>,
    pub pressed_keys: HashMap<Key, KeyPress>,
}

impl KeyState {
    pub fn new(receiver: UnboundedReceiver<KeyPress>) -> Self {
        KeyState {
            receiver,
            pressed_keys: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        loop {
            match self.receiver.try_next() {
                Ok(None) | Err(_) => break,
                Ok(Some(event)) => match event {
                    KeyPress::KeyUp(key) => self.set_released(key),
                    KeyPress::KeyDown(key) => self.set_prssed(key, event),
                },
            }
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains_key(&key)
    }

    fn set_prssed(&mut self, key: Key, event: KeyPress) {
        self.pressed_keys.insert(key, event);
    }

    fn set_released(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Space,
}

impl TryFrom<&str> for Key {
    type Error = ();

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            "Space" => Ok(Key::Space),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum KeyPress {
    KeyUp(Key),
    KeyDown(Key),
}
