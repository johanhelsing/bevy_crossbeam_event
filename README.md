# bevy_crossbeam_event

[![crates.io](https://img.shields.io/crates/v/bevy_crossbeam_event.svg)](https://crates.io/crates/bevy_crossbeam_event)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![crates.io](https://img.shields.io/crates/d/bevy_crossbeam_event.svg)](https://crates.io/crates/bevy_crossbeam_event)
[![docs.rs](https://img.shields.io/docsrs/bevy_crossbeam_event)](https://docs.rs/bevy_crossbeam_event)

Fire Bevy events from crossbeam channels.

Useful if you need to handle callbacks in 3rd party libraries etc. like
`steamworks-rs`, or getting events out of `tracing` layers.

## Usage

Add add events to your app using `.add_crossbeam_event::<EventType>`:

```rust ignore
#[derive(Clone, Debug)]
struct LobbyJoined(Lobby);

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_crossbeam_event::<LobbyJoined>();
        app.add_startup_system(setup);
        app.add_system(handle_lobby_joined);
    }
}
```

Fire events by using `Res<CrossbeamEventSender<EventType>>` (which can be
cloned and sent into callbacks):

```rust ignore
fn setup(service: Res<ThirdPartyCode>, sender: Res<CrossbeamEventSender<LobbyJoined>>) {
    let sender = sender.clone();
    service.join_lobby(id, move |lobby| {
        sender.send(LobbyJoined(lobby));
    });
}
```

Handle the events just like normal Bevy events (which they are):

```rust ignore
fn handle_lobby_joined(mut lobby_joined_events: EventReader<LobbyJoined>) {
    for lobby in lobby_joined_events.iter() {
        info!("lobby joined: {lobby:?}");
    }
}
```

## Bevy Version Support

The `main` branch targets the latest bevy release.

|bevy|bevy_crossbeam_event|
|----|--------------------|
|0.14| 0.6, main          |
|0.13| 0.5                |
|0.12| 0.3                |
|0.11| 0.2                |
|0.10| 0.1                |

## License

`bevy_crossbeam_event` is dual-licensed under either

- MIT License (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

## Contributions

PRs welcome!
