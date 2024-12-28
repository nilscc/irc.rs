use super::route::Route;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub host: String,
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct SettingsProps {
    #[prop_or_default]
    pub settings: Option<Settings>,
    #[prop_or_default]
    pub set_settings: Callback<Settings>,
}

#[function_component]
pub fn SettingsPage(props: &SettingsProps) -> Html {
    let host_ref = use_node_ref();

    let onsubmit = Callback::from({
        let host_ref = host_ref.clone();
        let set_settings = props.set_settings.clone();
        let nav = use_navigator().unwrap();
        move |e: SubmitEvent| {
            e.prevent_default();
            let host = host_ref.cast::<HtmlInputElement>().unwrap().value();
            set_settings.emit(Settings { host });
            nav.push(&Route::Home);
        }
    });

    let host = match &props.settings {
        Some(settings) => settings.host.clone(),
        _ => "".into(),
    };

    html! {
        <form {onsubmit}>
            <h1>{ "Settings" }</h1>
            <label for="host">{ "Host name:" }</label>
            <input ref={host_ref} id="host" type="text" value={host} />
            <input type="submit" />
        </form>
    }
}
