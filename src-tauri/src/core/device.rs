use rdev::{listen, Event, EventType};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread::spawn,
};
use tauri::{AppHandle, Emitter};

static IS_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize)]
pub enum DeviceKind {
    MousePress,
    MouseRelease,
    MouseMove,
    KeyboardPress,
    KeyboardRelease,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceEvent {
    kind: DeviceKind,
    value: Value,
}

pub fn start_listening(app_handle: AppHandle) {
    if IS_RUNNING.load(Ordering::SeqCst) {
        return;
    }

    IS_RUNNING.store(true, Ordering::SeqCst);

    spawn(move || {
        let callback = move |event: Event| {
            let device = match event.event_type {
                EventType::ButtonPress(button) => DeviceEvent {
                    kind: DeviceKind::MousePress,
                    value: json!(format!("{:?}", button)),
                },
                EventType::ButtonRelease(button) => DeviceEvent {
                    kind: DeviceKind::MouseRelease,
                    value: json!(format!("{:?}", button)),
                },
                EventType::MouseMove { x, y } => DeviceEvent {
                    kind: DeviceKind::MouseMove,
                    value: json!({ "x": x, "y": y }),
                },
                EventType::KeyPress(key) => DeviceEvent {
                    kind: DeviceKind::KeyboardPress,
                    value: json!(format!("{:?}", key)),
                },
                EventType::KeyRelease(key) => DeviceEvent {
                    kind: DeviceKind::KeyboardRelease,
                    value: json!(format!("{:?}", key)),
                },
                _ => return,
            };

            if let Err(e) = app_handle.emit("change", device) {
                eprintln!("Failed to emit event: {:?}", e);
            }
        };

        if let Err(e) = listen(callback) {
            eprintln!("Device listening error: {:?}", e);
        }
    });
}
