use yew::{function_component, Html, html};

#[function_component]
fn App() -> Html {
    let welcome_text = "Hello, World!";

    html! {
        <div>
            <p>{ welcome_text }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
