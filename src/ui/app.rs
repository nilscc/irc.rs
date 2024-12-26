use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::irc;

#[derive(Debug, Clone, PartialEq)]
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
        move |e: SubmitEvent| {
            e.prevent_default();
            let host = host_ref.cast::<HtmlInputElement>().unwrap().value();
            set_settings.emit(Settings { host })
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

fn switch_route(route: Route, settings: &Option<Settings>) -> Html {
    match route {
        Route::Home => html! {
            <HomePage settings={settings.clone()}/>
        },
        Route::Settings => html! {
            <SettingsPage />
        },
    }
}

#[function_component]
pub fn App() -> Html {
    let settings = use_state(|| None::<Settings>);

    html! {
        <BrowserRouter>
            <Switch<Route> render={move |r| switch_route(r, &settings)}/>
        </BrowserRouter>
    }
}
