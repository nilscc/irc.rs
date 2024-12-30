mod buffer_view;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::irc::{
    client::{
        buffer::{Buffer, Line},
        Client,
    },
    parser::{Source, User},
};

use super::{Route, Settings};
use buffer_view::BufferView;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Properties)]
pub struct HomeProps {
    pub settings: Option<Settings>,
}

fn example_buffer() -> Buffer {
    Buffer {
        id: 0,
        name: "#helloworld".into(),
        motd: Some("Hello world! This is a strange place to be.".into()),
        lines: vec![
            Line {
                id: 0,
                source: Source::Host("localhost".into()),
                message: "moep".into(),
            },
            Line {
                id: 1,
                source: Source::User(User {
                    nick: "McManiaC".into(),
                    user: None,
                    host: None,
                }),
                message: "Hello world!".into(),
            },
        ],
    }
}

#[function_component]
pub fn HomePage(props: &HomeProps) -> Html {
    let nav = use_navigator().unwrap();

    let client = use_state_eq(|| Client {
        capabilities: vec![],
        buffers: vec![example_buffer()],
    });

    if props.settings.is_none() {
        nav.push(&Route::Settings);
        html! {
            <p>{ "Go to settings." }</p>
        }
    } else {
        html! {
            <main
                class="flex flex-row w-full h-full"
                >
                <BufferView buffer={client.buffers[0].clone()} />
            </main>
        }
    }
}
