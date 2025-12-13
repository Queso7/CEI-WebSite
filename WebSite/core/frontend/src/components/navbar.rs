use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="w-full py-4 px-6 bg-white shadow-md flex justify-between items-center">
            <h1 class="text-xl font-bold text-pink-600">{"CEI"}</h1>
            <button class="bg-pink-500 text-white px-4 py-2 rounded-xl">
                {"Login"}
            </button>
        </nav>
    }
}