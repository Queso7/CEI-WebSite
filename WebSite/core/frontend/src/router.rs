use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { 
            <div class="flex-1 min-h-[70vh] flex flex-col items-center justify-center text-center px-4">
                <h1 class="text-9xl font-black text-gray-100 absolute z-0 select-none">{"404"}</h1>
                <div class="z-10">
                    <h2 class="text-3xl font-bold text-[#0b2545] mb-4">{"¡Ups! Página no encontrada"}</h2>
                    <p class="text-gray-600 mb-8 max-w-md">
                        {"Parece que la página se movió de lugar."}
                    </p>
                    <Link<Route> to={Route::Home} classes="px-6 py-3 bg-[#0b2545] text-white rounded-lg font-bold hover:bg-opacity-90 transition-all shadow-lg">
                        {"Volver al Inicio"}
                    </Link<Route>>
                </div>
            </div>
        },
    }
}