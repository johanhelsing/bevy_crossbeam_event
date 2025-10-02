use bevy::{ecs::message::Message, prelude::*};
use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError};

#[derive(Resource, Clone, Debug)]
pub struct CrossbeamEventSender<T: Message>(Sender<T>);

impl<T: Message> CrossbeamEventSender<T> {
    pub fn send(&self, event: impl Into<T>) {
        let event = event.into();
        if let Err(err) = self.0.try_send(event) {
            match err {
                // we have an unbounded channel, so this would only happen if we're out of memory
                TrySendError::Full(_) => panic!("unable to send event, channel full"),
                // This should only happen if callbacks happen as the app is shutting down, so we ignore it
                TrySendError::Disconnected(_) => {}
            }
        };
    }
}

#[derive(Resource)]
struct CrossbeamEventReceiver<T: Message>(Receiver<T>);

pub trait CrossbeamEventApp {
    fn add_crossbeam_event<T: Message>(&mut self) -> &mut Self;
}

impl CrossbeamEventApp for App {
    fn add_crossbeam_event<T: Message>(&mut self) -> &mut Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        self.insert_resource(CrossbeamEventSender::<T>(sender));
        self.insert_resource(CrossbeamEventReceiver::<T>(receiver));
        self.add_message::<T>();
        self.add_systems(PreUpdate, process_crossbeam_messages::<T>);
        self
    }
}

fn process_crossbeam_messages<T: Message>(
    receiver: Res<CrossbeamEventReceiver<T>>,
    mut events: MessageWriter<T>,
) {
    loop {
        match receiver.0.try_recv() {
            Ok(msg) => {
                events.write(msg);
            }
            Err(TryRecvError::Disconnected) => {
                panic!("sender resource dropped")
            }
            Err(TryRecvError::Empty) => {
                break;
            }
        }
    }
}
