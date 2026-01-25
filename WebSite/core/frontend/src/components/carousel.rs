use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub images: Vec<String>,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let active_idx = use_state(|| 0);
    let len = props.images.len();

    {
        let active_idx_handle = active_idx.clone();
        let current_pos = *active_idx;

        use_effect_with(active_idx.clone(), move |_| {
            let mut timeout_handle = None;
            if len > 1 {
                let handle_clone = active_idx_handle.clone();
                timeout_handle = Some(Timeout::new(4000, move || {
                    let next_val = (current_pos + 1) % len;
                    handle_clone.set(next_val);
                }));
            }
            move || {
                if let Some(t) = timeout_handle {
                    drop(t);
                }
            }
        });
    }

    let on_prev = {
        let active_idx = active_idx.clone();
        move |_| { if len > 0 { active_idx.set((*active_idx + len - 1) % len); } }
    };

    let on_next = {
        let active_idx = active_idx.clone();
        move |_| { if len > 0 { active_idx.set((*active_idx + 1) % len); } }
    };

    html! {
        <div class="max-w-7xl mx-auto px-2 sm:px-4">
            <div class={classes!("relative", "w-full", "h-[250px]", "sm:h-[400px]", "md:h-[500px]", "lg:h-[600px]", "overflow-hidden", "rounded-2xl", "md:rounded-3xl", "bg-gray-900", "group", "shadow-2xl")}>
                {
                    for props.images.iter().enumerate().map(|(i, url)| {
                        let is_active = i == *active_idx;
                        html! {
                            <div key={url.clone()} class={classes!(
                                "absolute", "inset-0", "transition-all", "duration-1000", "ease-in-out",
                                if is_active { "opacity-100" } else { "opacity-0" },
                                if is_active { "scale-100" } else { "scale-105" },
                                if is_active { "z-10" } else { "z-0" }
                            )}>
                                <img 
                                    src={url.clone()} 
                                    class="w-full h-full object-cover select-none" 
                                    alt={format!("Slide {}", i)}
                                />
                                <div class={classes!("absolute", "inset-0", "bg-gradient-to-t", "from-black/40", "via-transparent", "to-transparent", "pointer-events-none")} />
                            </div>
                        }
                    })
                }

                <button 
                    onclick={on_prev} 
                    class={classes!("absolute", "left-2", "md:left-4", "top-1/2", "-translate-y-1/2", "z-30", "p-2", "md:p-3", "rounded-full", "bg-white/10", "hover:bg-white/30", "text-white", "backdrop-blur-sm", "opacity-0", "group-hover:opacity-100", "transition-all", "duration-300")}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-5 h-5 md:w-6 md:h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
                    </svg>
                </button>

                <button 
                    onclick={on_next} 
                    class={classes!("absolute", "right-2", "md:right-4", "top-1/2", "-translate-y-1/2", "z-30", "p-2", "md:p-3", "rounded-full", "bg-white/10", "hover:bg-white/30", "text-white", "backdrop-blur-sm", "opacity-0", "group-hover:opacity-100", "transition-all", "duration-300")}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-5 h-5 md:w-6 md:h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
                    </svg>
                </button>

                <div class={classes!("absolute", "bottom-4", "md:bottom-8", "left-1/2", "-translate-x-1/2", "flex", "space-x-2", "md:space-x-3", "z-20")}>
                    {
                        for (0..len).map(|i| {
                            let is_active = i == *active_idx;
                            html! {
                                <button 
                                    onclick={
                                        let active_idx = active_idx.clone();
                                        move |_| active_idx.set(i)
                                    }
                                    class={classes!(
                                        "h-1.5", "md:h-2", "transition-all", "duration-500", "rounded-full", "shadow-sm",
                                        if is_active { "w-8" } else { "w-1.5" },
                                        if is_active { "md:w-12" } else { "md:w-2" },
                                        if is_active { "bg-white" } else { "bg-white/50" },
                                        if !is_active { "hover:bg-white/80" } else { "" }
                                    )} 
                                />
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}