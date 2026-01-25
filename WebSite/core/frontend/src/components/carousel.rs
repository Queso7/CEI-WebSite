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
        <div class="max-w-6xl mx-auto px-4 sm:px-6">
            <div class={classes!(
                "relative", "w-full", 
                "h-[200px]", "sm:h-[300px]", "md:h-[400px]", "lg:h-[450px]", 
                "overflow-hidden", "rounded-2xl", "md:rounded-3xl", 
                "bg-gray-900", "group", "shadow-xl"
            )}>
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
                                <div class={classes!("absolute", "inset-0", "bg-gradient-to-t", "from-black/30", "via-transparent", "to-transparent", "pointer-events-none")} />
                            </div>
                        }
                    })
                }
                
                <button 
                    onclick={on_prev} 
                    class={classes!("absolute", "left-3", "top-1/2", "-translate-y-1/2", "z-30", "p-1.5", "md:p-2", "rounded-full", "bg-white/10", "hover:bg-white/20", "text-white", "backdrop-blur-sm", "opacity-0", "group-hover:opacity-100", "transition-all")}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-4 h-4 md:w-5 md:h-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
                    </svg>
                </button>

                <button 
                    onclick={on_next} 
                    class={classes!("absolute", "right-3", "top-1/2", "-translate-y-1/2", "z-30", "p-1.5", "md:p-2", "rounded-full", "bg-white/10", "hover:bg-white/20", "text-white", "backdrop-blur-sm", "opacity-0", "group-hover:opacity-100", "transition-all")}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-4 h-4 md:w-5 md:h-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
                    </svg>
                </button>

                <div class={classes!("absolute", "bottom-3", "md:bottom-5", "left-1/2", "-translate-x-1/2", "flex", "space-x-2", "z-20")}>
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
                                        "h-1", "md:h-1.5", "transition-all", "duration-500", "rounded-full",
                                        if is_active { "w-6 md:w-8 bg-white" } else { "w-1 md:w-1.5 bg-white/40" }
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