mod buffer_view;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::irc::client::{Buffer, Client};

use super::{Route, Settings};
use buffer_view::BufferView;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Properties)]
pub struct HomeProps {
    pub settings: Option<Settings>,
}

#[function_component]
pub fn HomePage(props: &HomeProps) -> Html {
    let nav = use_navigator().unwrap();

    let client = use_state_eq(|| Client {
        capabilities: vec![],
        buffers: vec![Buffer {}],
    });

    if props.settings.is_none() {
        nav.push(&Route::Settings);
        html! {
            <p>{ "Go to settings." }</p>
        }
    } else {
        html! {
            <BufferView buffers={client.buffers.clone()} />
        }
    }
}
