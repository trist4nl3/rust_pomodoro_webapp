use wasm_bindgen::JsCast;
use web_sys::{HtmlAudioElement, AudioContext, MediaElementAudioSourceNode};
use std::rc::Rc;
use std::cell::RefCell;

pub fn init_audio() -> (HtmlAudioElement, HtmlAudioElement, Rc<RefCell<Option<AudioContext>>>) {
    // Create audio elements for each sound
    let button_audio = HtmlAudioElement::new().unwrap();
    let alarm_audio = HtmlAudioElement::new().unwrap();

    // Set the source for each audio element
    button_audio.set_src("src/assets/button.wav");
    alarm_audio.set_src("src/assets/alarm.mp3");

    // Create an Rc<RefCell<Option<AudioContext>>> to allow mutable borrowing
    let audio_context: Rc<RefCell<Option<AudioContext>>> = Rc::new(RefCell::new(None));

    // Return the audio elements and AudioContext
    (button_audio, alarm_audio, audio_context)
}
