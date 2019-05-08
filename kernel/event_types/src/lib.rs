#![no_std]

extern crate keycodes_ascii;
extern crate alloc;

use keycodes_ascii::{KeyEvent};
use alloc::string::String;


#[derive(Debug, Clone)]
pub enum Event {
    /// An input event from a keyboard
    InputEvent(KeyboardInputEvent),
    /// An event from another entity that wishes to print a message
    OutputEvent(PrintOutputEvent),
    /// Tells an application that the window manager has resized that application's window
    /// so that it knows to perform any necessary tasks related to window size, such as text reflow.
    ResizeEvent(WindowResizeEvent),
    ExitEvent,
}

impl Event {
    pub fn new_input_event(kev: KeyEvent) -> Event {
        Event::InputEvent(KeyboardInputEvent::new(kev))
    }

    pub fn new_output_event<S>(s: S) -> Event where S: Into<String> {
        Event::OutputEvent(PrintOutputEvent::new(s.into()))
    }

    pub fn new_resize_event(x: usize, y: usize, width: usize, height: usize) -> Event {
        Event::ResizeEvent(WindowResizeEvent::new(x,y,width, height))
    }

}

/// use this to deliver input events (such as keyboard input) to the input_event_manager.
#[derive(Debug, Clone)]
pub struct KeyboardInputEvent {
    pub key_event: KeyEvent,
}

impl KeyboardInputEvent {
    pub fn new(kev: KeyEvent) -> KeyboardInputEvent {
        KeyboardInputEvent {
            key_event: kev,
        }
    }
}

/// use this to queue up a formatted string that should be printed to the input_event_manager. 
#[derive(Debug, Clone)]
pub struct PrintOutputEvent {
    pub text: String,
}

impl PrintOutputEvent {
    pub fn new(s: String) -> PrintOutputEvent {
        PrintOutputEvent {
            text: s,
        }
    }
}

//Use this to inform the window manager to adjust the sizes of existing windows
#[derive(Debug, Clone)]
pub struct WindowResizeEvent {
    pub x: usize,
    pub y: usize,
    pub width: usize, 
    pub height: usize, 
}
 impl WindowResizeEvent {
    pub fn new(x: usize, y: usize, width: usize, height:usize) -> WindowResizeEvent {
        WindowResizeEvent {
            x: x,
            y: y,
            width: width, 
            height: height,
        }
    }
} 