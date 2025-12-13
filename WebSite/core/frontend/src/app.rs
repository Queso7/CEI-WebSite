use yew::prelude::*;
use crate::components::footer::Footer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <main class="min-h-screen flex flex-col items-center justify-center bg-pink-50">
                <h1 class="text-4xl font-bold text-pink-700 mb-4">
                    { "Bienvenido al CEI UwU" }
                </h1>
                <p class="text-lg text-pink-500">
                    { "Frontend hecho en Rust con Yew" }
                </p>
            </main>

            <Footer />
        </>
    }
}
