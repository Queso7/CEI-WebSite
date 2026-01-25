use yew::prelude::*;
mod components;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::carousel::Carousel;

#[function_component(App)]
fn app() -> Html {
    // Manana lo deshardcodeamos, pijes
    let slides = use_memo((), |_| vec![
        "https://images.unsplash.com/photo-1441986300917-64674bd600d8?w=1000&q=80".to_string(),
        "https://images.unsplash.com/photo-1441984904996-e0b6ba687e04?w=1000&q=80".to_string(),
        "https://images.unsplash.com/photo-1472851294608-062f824d29cc?w=1000&q=80".to_string(),
    ]);

    html! {
        <div class="min-h-screen flex flex-col bg-white font-sans">
            <Navbar />
            <main class="flex-1 container mx-auto px-4 py-8">
                <Carousel images={(*slides).clone()} />
                
                <div class="mt-12 text-center">
                    <h2 class="text-4xl font-extrabold text-gray-900">{"Bienvenido al Centro de Idiomas de la FES-Acatlan"}</h2>
                    <p class="text-lg text-gray-600 mt-4">{"UwU"}</p>
                </div>
            </main>
            <Footer />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}