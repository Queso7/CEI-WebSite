use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    html! {
        <>
            <nav class="bg-white/80 backdrop-blur-md shadow-lg px-6 py-4 flex justify-between items-center">
                <h1 class="text-2xl font-bold text-pink-700">
                    { "CEI" }
                </h1>

                <button
                    onclick={toggle_menu}
                    class="md:hidden text-pink-600 text-3xl font-bold focus:outline-none"
                >
                    { "≡" }
                </button>

                <div class="hidden md:flex gap-6 text-pink-700 font-semibold">
                    <a href="#">{"Departamentos"}</a>
                    <a href="#">{"Idiomas y cursos"}</a>
                    <a href="#">{"Talleres"}</a>
                </div>
            </nav>

            {
                if *menu_open {
                    html! {
                        <div class="md:hidden bg-white/70 backdrop-blur-md text-pink-700
                                    flex flex-col px-6 py-4 gap-4 shadow-md">
                            <a href="#">{"Departamentos"}</a>
                            <a href="#">{"Idiomas y cursos"}</a>
                            <a href="#">{"Talleres"}</a>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
