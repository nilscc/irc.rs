mod home;
mod route;
mod settings;

use gloo::{
    console,
    storage::{LocalStorage, Storage},
};
use home::HomePage;
use route::Route;
use settings::{Settings, SettingsPage};
use yew::prelude::*;
use yew_router::prelude::*;

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
