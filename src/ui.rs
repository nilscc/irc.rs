mod app;

#[cfg(test)]
mod test;

pub fn main() {
    yew::Renderer::<app::App>::new().render();
}
