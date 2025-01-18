use yew::prelude::*;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Properties)]
pub struct ChannelsProps {
    pub names: Vec<AttrValue>,
}

#[function_component]
pub fn Channels(props: &ChannelsProps) -> Html {
    let names = props
        .names
        .iter()
        .map(|s| html!(<ChannelName name={s.clone()} />))
        .collect::<Vec<Html>>();

    html!(
        <div class="m-4">
            { names }
        </div>
    )
}

#[derive(Debug, PartialEq, Properties)]
struct ChannelNameProps {
    name: AttrValue,
}

#[function_component]
fn ChannelName(props: &ChannelNameProps) -> Html {
    html!(
        <p>{ props.name.clone() }</p>
    )
}
