#[cfg(test)]
mod test;

use yew::prelude::*;

use crate::irc::client::buffer::Buffer;

#[derive(Debug, PartialEq, Properties)]
pub struct BufferViewProps {
    pub buffer: Buffer,
}

#[function_component]
pub fn BufferView(props: &BufferViewProps) -> Html {
    html! {
        <p>{ format!("Buffer view! {:?}", props.buffer) }</p>
    }
}
