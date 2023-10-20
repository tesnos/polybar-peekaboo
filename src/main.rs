extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;
use i3ipc::event::inner::WindowChange;
use i3ipc::reply::NodeType;
use i3ipc::event::inner::WorkspaceChange;

use sysinfo::{System, SystemExt};

use std::process::Command;
use std::{thread, time};


fn workspace_is_empty(connection: &mut I3Connection) -> bool {
    let mut root = connection.get_tree().unwrap();
    while !root.focus.is_empty() {
        let id = root.focus[0];
        for node in &root.nodes {
            if node.id == id {
                root = node.clone();
                break;
            }
        }
    }

    root.nodetype == NodeType::Workspace
}

fn set_bar_hidden(state: bool) -> bool {
    let cmd = if state { "hide" } else { "show" };
    let _ = Command::new("polybar-msg").arg("cmd").arg(cmd).output();

    state
}


fn main() {
    let s = System::new_all();
    if s.processes_by_exact_name("polybar-peekabo").count() > 1 {
        panic!("polybar-peekaboo is already running!");
    }

    let mut connection = loop {
        match I3Connection::connect() {
            Ok(c) => break c,
            Err(_) => thread::sleep(time::Duration::from_millis(500)),
        }
    };
    let mut listener = loop {
        match I3EventListener::connect() {
            Ok(c) => break c,
            Err(_) => thread::sleep(time::Duration::from_millis(500)),
        }
    };
    let subs = [Subscription::Workspace, Subscription::Window];
    listener.subscribe(&subs).expect("Could not communicate with i3!");

    let mut hidden = false;

    for event in listener.listen() {
        match event.unwrap() {
            Event::WorkspaceEvent(e) => {
                hidden = set_bar_hidden(e.change == WorkspaceChange::Focus && workspace_is_empty(&mut connection));
            },
            Event::WindowEvent(e) => {
                if e.change == WindowChange::New && hidden {
                    hidden = set_bar_hidden(false);
                }
                if e.change == WindowChange::Close && !hidden && workspace_is_empty(&mut connection) {
                    hidden = set_bar_hidden(true);
                }
            },
            _ => unreachable!(),
        }
    }
}
