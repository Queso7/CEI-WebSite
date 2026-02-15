use yew::prelude::*;

struct NavItem {
    name: &'static str,
    href: &'static str,
}

struct NavCategory {
    title: &'static str,
    items: Vec<NavItem>,
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let menu_open = use_state(|| false);
    let active_mobile_submenu = use_state(|| Option::<&'static str>::None);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(false))
    };

    let categories = vec![
        NavCategory {
            title: "Oferta Académica",
            items: vec![
                NavItem { name: "Idiomas y cursos", href: "#" },
                NavItem { name: "Talleres (Regulares / Sabatino)", href: "#" },
                NavItem { name: "Formación (Material didáctico)", href: "#" },
                NavItem { name: "Mocks (Exámenes de requisito)", href: "#" },
            ],
        },
        NavCategory {
            title: "Inscripciones",
            items: vec![
                NavItem { name: "Registro (Informes y servicios)", href: "#" },
                NavItem { name: "Exámenes de certificación", href: "#" },
                NavItem { name: "Informes generales", href: "#" },
            ],
        },
        NavCategory {
            title: "Comunidad",
            items: vec![
                NavItem { name: "Difusión cultural", href: "#" },
                NavItem { name: "Selita", href: "#" },
                NavItem { name: "Podcast", href: "#" },
            ],
        },
        NavCategory {
            title: "Nosotros",
            items: vec![
                NavItem { name: "¿Quiénes somos?", href: "#" },
                NavItem { name: "Departamentos (Ing / Ita / Jp)", href: "#" },
            ],
        },
    ];

    html! {
        <header class="sticky top-0 z-50 w-full">
            <nav class="bg-[#0b2545]/95 backdrop-blur-md shadow-lg border-b border-white/10 px-6 py-4">
                <div class="max-w-7xl mx-auto flex justify-between items-center">
                    <div class="flex items-center cursor-pointer">
                        <h1 class="text-2xl font-black text-white tracking-tight">
                            { "CEI" }
                        </h1>
                    </div>

                    // --- DESKTOP ---
                    <div class="hidden lg:flex gap-6 font-semibold">
                        { for categories.iter().map(|cat| html! {
                            <div class="group relative py-2">
                                <button class="flex items-center gap-1 text-white/90 group-hover:text-white transition-colors duration-300">
                                    { cat.title }
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 opacity-50 group-hover:rotate-180 transition-transform duration-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                                    </svg>
                                </button>
                                
                                <div class="absolute top-full left-0 mt-2 w-64 opacity-0 translate-y-2 invisible group-hover:opacity-100 group-hover:translate-y-0 group-hover:visible transition-all duration-300 ease-in-out">
                                    <div class="bg-white rounded-2xl shadow-xl border border-slate-100 py-2 overflow-hidden">
                                        { for cat.items.iter().map(|item| html! {
                                            <a href={item.href} class="block px-5 py-2 text-sm text-slate-700 hover:bg-blue-50 hover:text-blue-700 transition-colors duration-300 font-medium">
                                                { item.name }
                                            </a>
                                        })}
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>

                    <button onclick={toggle_menu} class="lg:hidden text-white text-3xl font-bold focus:outline-none transition-all duration-300 ease-in-out active:scale-90">
                        <span class="block transition-opacity duration-300 ease-in-out">
                            { if *menu_open { "✕" } else { "≡" } }
                        </span>
                    </button>
                </div>

                // --- MOBILE ---
                <div 
                    class={classes!(
                        "lg:hidden", "absolute", "top-full", "left-0", "w-full", 
                        "bg-[#0b2545]/95", "backdrop-blur-xl", "text-white", 
                        "flex", "flex-col", "px-8", "py-6", "gap-2", 
                        "shadow-2xl", "border-t", "border-white/10",
                        "transition-all", "duration-300", "ease-in-out",
                        if *menu_open { "opacity-100 translate-y-0 visible" } else { "opacity-0 -translate-y-4 invisible" }
                    )}
                >
                    { for categories.iter().map(|cat| {
                        let title = cat.title;
                        let is_active = *active_mobile_submenu == Some(title);
                        let toggle = {
                            let active = active_mobile_submenu.clone();
                            Callback::from(move |_| {
                                if is_active { active.set(None) } else { active.set(Some(title)) }
                            })
                        };

                        html! {
                            <div class="flex flex-col">
                                <button onclick={toggle} class="flex justify-between items-center py-3 text-lg font-bold hover:text-white/80 transition-colors duration-300">
                                    { cat.title }
                                    <span class={classes!("transition-transform", "duration-300", "ease-in-out", if is_active { "rotate-180" } else { "" })}>
                                        {"↓"}
                                    </span>
                                </button>

                                <div class={classes!(
                                    "flex", "flex-col", "ml-4", "border-l", "border-white/20", "pl-4", "overflow-hidden", "transition-all", "duration-300", "ease-in-out",
                                    if is_active { "max-h-64 opacity-100 mb-2" } else { "max-h-0 opacity-0 pointer-events-none" }
                                )}>
                                    { for cat.items.iter().map(|item| {
                                        let on_link_click = close_menu.clone();
                                        html! {
                                            <a 
                                                href={item.href} 
                                                onclick={on_link_click}
                                                class="py-2 text-white/70 hover:text-white transition-colors duration-300"
                                            >
                                                { item.name }
                                            </a>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })}
                </div>
            </nav>
        </header>
    }
}