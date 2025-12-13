use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="w-full py-6 text-center text-pink-600 bg-pink-100 mt-10 rounded-t-2xl shadow-inner">
            { "© 2025 CEI — Hecho con Yew + Tailwind UwU " }
        </footer>
    }
}
