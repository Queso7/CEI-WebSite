use yew::prelude::*;
use yew::functional::*;

#[function_component(App)]
fn app() -> Html {

    let menu_open = use_state(|| false);
    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    html! {
        <div class="min-h-screen flex flex-col bg-gradient-to-br from-pink-300 via-purple-400 to-blue-500">

            <nav class="bg-white/80 backdrop-blur-md shadow-lg px-6 py-4 flex justify-between items-center">

                <h1 class="text-2xl font-bold text-pink-700">
                    { "CEI" }
                </h1>

                <button onclick={toggle_menu.clone()}
                    class="md:hidden text-pink-600 text-3xl font-bold focus:outline-none">
                    { "≡" }
                </button>

                <div class="hidden md:flex gap-6 text-pink-700 font-semibold">
                            <a href="#" class="font-semibold">{"Departamentos"}</a>
                            <a href="#" class="font-semibold">{"Idiomas y cursos"}</a>
                            <a href="#" class="font-semibold">{"Talleres"}</a>
                </div>
            </nav>
            {
                if *menu_open {
                    html! {
                        <div class="md:hidden bg-white/70 backdrop-blur-md text-pink-700
                                    flex flex-col px-6 py-4 gap-4 shadow-md">
                            <a href="#" class="font-semibold">{"Departamentos"}</a>
                            <a href="#" class="font-semibold">{"Idiomas y cursos"}</a>
                            <a href="#" class="font-semibold">{"Talleres"}</a>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            <main class="flex-1 flex items-center justify-center px-4">
                <div class="bg-white/40 backdrop-blur-lg p-10 rounded-3xl shadow-xl max-w-2xl text-center">
                    <h2 class="text-4xl font-bold text-white drop-shadow">
                        { "Bienvenido al CEI UwU" }
                    </h2>

                    <p class="text-white/90 mt-4">
                        { "un saludo a la gente furra de corazon" }
                    </p>
                </div>
            </main>

            <footer class="bg-pink-100/70 backdrop-blur-md text-center py-3 text-sm text-pink-700">
                { "© 2025 CEI — Hecho con Yew UwU" }
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
