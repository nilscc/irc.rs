use gloo::{
    console,
    storage::{LocalStorage, Storage},
};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Settings {
    host: String,
}

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/settings")]
    Settings,
}

#[derive(Debug, PartialEq, Properties)]
struct HomeProps {
    settings: Option<Settings>,
}

#[function_component]
fn HomePage(props: &HomeProps) -> Html {
    let nav = use_navigator().unwrap();

    if props.settings.is_none() {
        nav.push(&Route::Settings);
        html! {
            <p>{ "Go to settings." }</p>
        }
    } else {
        html! {
            <h1 class="">{ "Hello world." }</h1>
        }
    }
}

#[derive(Debug, PartialEq, Clone, Properties)]
struct SettingsProps {
    #[prop_or_default]
    settings: Option<Settings>,
    #[prop_or_default]
    set_settings: Callback<Settings>,
}

#[function_component]
fn SettingsPage(props: &SettingsProps) -> Html {
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

#[function_component]
pub fn App() -> Html {
    // load settings from local storage (if available)
    let settings = use_state_eq(|| match LocalStorage::get("settings") {
        Ok(settings) => Some(settings),
        Err(err) => {
            console::error!(format!(
                "Error retrieving settings from local storage:\n{err}"
            ));
            None
        }
    });

    // store new settings in local storage and update UI state
    let set_settings = Callback::from({
        let settings = settings.clone();
        move |new_settings: Settings| {
            console::log!(format!("new settings: {new_settings:?}"));

            // set local storage
            if let Err(err) = LocalStorage::set("settings", &new_settings) {
                console::error!(format!("Error storing settings:\n{err}"));
            }

            // update state
            settings.set(Some(new_settings));
        }
    });

    // main application routing
    let switch = Callback::from({
        move |route: Route| {
            let settings = (*settings).clone();
            let set_settings = set_settings.clone();
            match route {
                Route::Home => html! {
                    <HomePage {settings} />
                },
                Route::Settings => html! {
                    <SettingsPage {settings} {set_settings} />
                },
            }
        }
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
