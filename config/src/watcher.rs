use {
    crate::{
        Config,
        attach::{self, AttachConfig},
    },
    notify::{Event, EventKind, RecursiveMode, Watcher},
    std::{
        fs,
        sync::{self},
    },
};

pub fn watch() -> notify::Result<()> {
    let (tx, rx) = sync::mpsc::channel::<Event>();

    let attach_path = AttachConfig::get_path().unwrap();
    if !attach_path.exists() {
        fs::File::create(&attach_path)?;
    }

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        let Ok(event) = res else {
            return;
        };

        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => (),
            _ => return,
        }

        let _ = tx.send(event);
    })?;

    watcher.watch(&attach_path, RecursiveMode::NonRecursive)?;

    while let Ok(event) = rx.recv() {
        for path in event.paths {
            if path == attach_path {
                *attach::CONFIG.write().unwrap() = AttachConfig::read().unwrap_or_default();
            }
        }
    }
    Ok(())
}
