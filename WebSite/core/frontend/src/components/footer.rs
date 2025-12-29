use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class=" w-full bg-blue-900 text-white text-sm py-6 border-t border-black 2">
            <div class="max-w-6xl mx-auto px-6 text-center space-y-2">
                <p class="font-semibold">
                    { "Universidad Nacional Autónoma de México (UNAM)" }
                </p>
                <p>
                    { "Hecho en México · © 2025 · Todos los derechos reservados" }
                </p>
                <p class="text-xs text-white">
                    { "Esta página puede reproducirse con fines no lucrativos siempre que se cite la fuente completa y su dirección electrónica." }
                </p>
            </div>
        </footer>
    }
}
