use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    html! {
        <header class="sticky top-0 z-50 w-full">
            <div class="relative">
                <nav
                    class="
                        bg-[#0b2545]/95 backdrop-blur-md shadow-lg
                        px-6 py-4 flex justify-between items-center
                        border-b border-white/10
                    "
                >
                    <h1 class="text-2xl font-bold text-white tracking-tight">
                        { "CEI" }
                    </h1>

                    <button
                        onclick={toggle_menu}
                        class="md:hidden text-white text-3xl font-bold focus:outline-none"
                    >
                        { if *menu_open { "✕" } else { "≡" } }
                    </button>

                    <div class="hidden md:flex gap-8 font-semibold">
                        <a class="text-white/90 hover:text-white transition">
                            { "Departamentos" }
                        </a>
                        <a class="text-white/90 hover:text-white transition">
                            { "Idiomas y cursos" }
                        </a>
                        <a class="text-white/90 hover:text-white transition">
                            { "Talleres" }
                        </a>
                    </div>
                </nav>

                if *menu_open {
                    <div
                        class={classes!(
                            "md:hidden", "absolute", "top-full", "left-0", "w-full",
                            "bg-[#0b2545]/75", "backdrop-blur-xl",
                            "text-white", "flex", "flex-col",
                            "px-8", "py-8", "gap-6",
                            "shadow-2xl", "border-t", "border-white/10",
                            "animate-in", "fade-in", "slide-in-from-top-2", "duration-300"
                        )}
                    >
                        <a class="text-lg font-bold hover:text-white/80 transition">
                            { "Departamentos" }
                        </a>
                        <a class="text-lg font-bold hover:text-white/80 transition">
                            { "Idiomas y cursos" }
                        </a>
                        <a class="text-lg font-bold hover:text-white/80 transition">
                            { "Talleres" }
                        </a>
                    </div>
                }
            </div>
        </header>
    }
}
