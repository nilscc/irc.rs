mod buffer_view;
mod channel_list;
mod user_list;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::irc::{
    client::{
        buffer::{Buffer, Line},
        Client,
    },
    parser::{Source, User},
};

use buffer_view::BufferView;
use channel_list::Channels;
use user_list::Users;

use super::{Route, Settings};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Properties)]
pub struct HomeProps {
    pub settings: Option<Settings>,
}

fn example_buffers() -> Vec<Buffer> {
    vec![
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
        },
        Buffer {
            id: 1,
            name: "#test".into(),
            motd: None,
            lines: vec![],
        },
    ]
}

#[function_component]
pub fn HomePage(props: &HomeProps) -> Html {
    let nav = use_navigator().unwrap();

    let client = use_state_eq(|| {
        let mut c = Client::new();
        c.buffers.append(&mut example_buffers());
        c
    });

    let channel_names = client
        .buffers
        .iter()
        .map(|b| b.name.clone())
        .collect::<Vec<AttrValue>>();

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
                <Channels names={channel_names}/>
                <BufferView buffer={client.buffers[0].clone()} />
                <Users />
            </main>
        }
    }
}
