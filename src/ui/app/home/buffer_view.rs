#[cfg(test)]
mod test;

use yew::prelude::*;

use crate::irc::{
    client::buffer::{Buffer, Line},
    parser::Source,
};

#[derive(Debug, PartialEq, Properties)]
pub struct BufferViewProps {
    pub buffer: Buffer,
}

#[function_component]
pub fn BufferView(props: &BufferViewProps) -> Html {
    let lines = props
        .buffer
        .lines
        .iter()
        .map(|line| html! { <LineItem line={line.clone()} /> })
        .collect::<Html>();

    let onclick = Callback::from(move |_| {});

    html! {
        <div
            class="grow flex flex-col"
            >
            // buffer title
            <Title name={props.buffer.name.clone()} motd={props.buffer.motd.clone()} />
            // buffer line view
            <div
                class="grow"
                >
                { lines }
            </div>
            // buffer input
            <div class="flex flex-row">
                <input type="text"
                    class="grow bg-slate-900 p-2 rounded-md outline-none focus:ring-slate-600 focus:ring-1"
                    />
                <button {onclick} value="Send"
                    class="rounded-md bg-slate-600 px-4 py-2 ml-4 hover:bg-slate-500"
                    >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor"
                        class="size-5"
                        >
                        <path strokeLinecap="round" strokeLinejoin="round" d="M6 12 3.269 3.125A59.769 59.769 0 0 1 21.485 12 59.768 59.768 0 0 1 3.27 20.875L5.999 12Zm0 0h7.5" />
                    </svg>
                </button>
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
struct TitleProps {
    name: AttrValue,
    #[prop_or_default]
    motd: Option<AttrValue>,
}

#[function_component]
fn Title(props: &TitleProps) -> Html {
    let name = html! {
        { props.name.clone() }
    };

    let motd = match &props.motd {
        Some(motd) => html! { <span class="text-slate-400">{ motd }</span> },
        None => html! {},
    };

    html! {
        <div class="bg-slate-700 rounded-md p-2 pr-4 flex flex-row">
            <p class="grow">{ motd }</p>
            <p class="">{ name }</p>
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
struct LineItemProps {
    line: Line,
}

#[function_component]
fn LineItem(props: &LineItemProps) -> Html {
    let inner = match &props.line.source {
        Source::Host(name) => html! {
            <p class="text-slate-500">{ name }{ ": "}{ props.line.message.clone() }</p>
        },
        Source::User(user) => {
            html! {
                <p>{ "<" }{ user.nick.clone() }{ "> " }{ props.line.message.clone() } </p>
            }
        }
    };

    html! {
        <div key={props.line.id}>
            { inner }
        </div>
    }
}
