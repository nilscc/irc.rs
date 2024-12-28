#[cfg(test)]
mod test;

use yew::prelude::*;

use crate::irc::client::Buffer;

#[derive(Debug, PartialEq, Properties)]
pub struct BufferViewProps {
    pub buffers: Vec<Buffer>,
}

#[function_component]
pub fn BufferView(props: &BufferViewProps) -> Html {
    html! {
        <p>{ format!("Buffer view! Number of buffers: {}", props.buffers.len()) }</p>
    }
}
