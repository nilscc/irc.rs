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
        <div
            class="grow flex flex-col"
            >
            // buffer title
            <div class="bg-slate-600 rounded-md p-2">
                { format!("{}", props.buffer.name) }
            </div>
            // buffer line view
            <div
                class="grow"
                >
                <p>{"Lines..."}</p>
            </div>
            // buffer input
            <input type="text"
                class="bg-slate-900 p-2 rounded-md focus:ring-slate-950"
                />
        </div>
    }
}
