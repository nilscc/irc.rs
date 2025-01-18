#[cfg(test)]
mod test;

use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct UsersProps {}

#[function_component]
pub fn Users(_props: &UsersProps) -> Html {
    html!(
        <>
        </>
    )
}
