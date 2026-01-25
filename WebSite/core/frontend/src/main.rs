mod components;
mod pages;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::footer::Footer;

use crate::router::{switch, Route}; 

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen flex flex-col bg-white font-sans">
                <Navbar /> 
                <main class="flex-1 w-full">
                    <Switch<Route> render={switch} />
                </main>
                <Footer />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}