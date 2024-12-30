mod buffer_view;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::irc::{
    client::{
        buffer::{Buffer, Line},
        Client,
    },
    parser::Source,
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
        name: "test".to_string(),
        lines: vec![Line {
            source: Source::Host("localhost".to_string()),
            message: "moep".into(),
        }],
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
                class="flex flex-row w-screen h-screen"
                >
                <BufferView buffer={client.buffers[0].clone()} />
            </main>
        }
    }
}
