use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event as CtEvent, KeyEvent, MouseEvent};

/// Terminal events.
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let cloned_sender = sender.clone();
        let handler = thread::spawn(move || main_loop(cloned_sender, tick_rate));
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function blocks the current thread when there is no data available.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}

/// Main loop.
///
/// Continuously poll for new events and send them to the given sender.
fn main_loop(sender: mpsc::Sender<Event>, tick_rate: Duration) {
    let mut last_tick = Instant::now();
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(tick_rate);
        if event::poll(timeout).expect("no events available") {
            match event::read().expect("unable to read event") {
                CtEvent::Key(e) if e.kind == event::KeyEventKind::Press => {
                    sender.send(Event::Key(e))
                }
                CtEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                CtEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                _ => Ok(()),
            }
            .expect("failed to send terminal event")
        }

        if last_tick.elapsed() >= tick_rate {
            sender.send(Event::Tick).expect("failed to send tick event");
            last_tick = Instant::now();
        }
    }
}
