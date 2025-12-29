use yew::prelude::*;
use yew::functional::*;
mod components;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="min-h-screen flex flex-col bg-white">
            <Navbar />

            <main class="flex-1 flex items-center justify-center px-4">
            
            </main>

            <Footer />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
