use yew::prelude::*;
use crate::components::carousel::Carousel;

#[function_component(Home)]
pub fn home() -> Html {
    let slides = use_memo((), |_| vec![
        "https://images.unsplash.com/photo-1441986300917-64674bd600d8?w=1000&q=80".to_string(),
        "https://images.unsplash.com/photo-1441984904996-e0b6ba687e04?w=1000&q=80".to_string(),
        "https://images.unsplash.com/photo-1472851294608-062f824d29cc?w=1000&q=80".to_string(),
    ]);

    html! {
        <div class="flex flex-col w-full pb-20 bg-slate-50/30">
            <section class="max-w-5xl mx-auto mt-8 px-4 w-full">
                <div class="relative overflow-hidden bg-white rounded-3xl border border-slate-100 shadow-xl shadow-blue-900/5 p-8 md:p-10 text-center">
                    <div class="absolute top-0 right-0 -mr-16 -mt-16 w-64 h-64 bg-blue-50 rounded-full blur-3xl opacity-60" />
                    <div class="relative z-10">
                        <h2 class="text-2xl md:text-4xl font-black text-[#0b2545] leading-tight italic">
                            {"Bienvenido al "}
                            <span class="text-blue-700">{"Centro de Enseñanza de Idiomas"}</span>
                        </h2>
                    </div>
                </div>
            </section>

            <section class="w-full mt-10">
                <div class="max-w-[900px] mx-auto px-4">
                    <div class="rounded-3xl overflow-hidden shadow-lg">
                        <Carousel images={(*slides).clone()} />
                    </div>
                </div>
            </section>

            <section class="w-full mt-16 px-4">
                <div class="max-w-7xl mx-auto">
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                        
                        <div class="group p-8 rounded-3xl bg-blue-50/50 border border-blue-100 hover:bg-blue-100/50 hover:shadow-xl transition-all duration-300">
                            <div class="w-12 h-12 bg-blue-500 rounded-2xl flex items-center justify-center text-white mb-6 group-hover:rotate-6 transition-transform shadow-lg shadow-blue-200">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25" />
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-blue-900 mb-4">{"Cursos"}</h3>
                            <ul class="space-y-2 text-blue-800/70 text-sm font-medium">
                                <li class="hover:text-blue-600 cursor-pointer transition-colors">{"• Idiomas y cursos"}</li>
                                <li class="hover:text-blue-600 cursor-pointer transition-colors">{"• Talleres (Reg/Sab)"}</li>
                                <li class="hover:text-blue-600 cursor-pointer transition-colors font-bold">{"• Registro e informes"}</li>
                            </ul>
                        </div>

                        <div class="group p-8 rounded-3xl bg-purple-50/50 border border-purple-100 hover:bg-purple-100/50 hover:shadow-xl transition-all duration-300">
                            <div class="w-12 h-12 bg-purple-500 rounded-2xl flex items-center justify-center text-white mb-6 group-hover:rotate-6 transition-transform shadow-lg shadow-purple-200">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M10.34 15.84c-.688-.06-1.386-.09-2.09-.09H7.5a4.5 4.5 0 110-9h.75c.704 0 1.402-.03 2.09-.09m0 9.18c.253.962.584 1.892.985 2.783.247.55.06 1.21-.463 1.511l-.657.38c-.551.318-1.26.117-1.527-.461a13.247 13.247 0 01-1.022-2.384m2.684-2.007V15.75m1.192-9.423a11.421 11.421 0 003.587 4.582m1.192 4.841a11.42 11.42 0 003.587-4.582m0 0a11.454 11.454 0 01-3.587-4.582m3.587 4.582h.75a2.25 2.25 0 012.25 2.25v.75a2.25 2.25 0 01-2.25 2.25h-.75" />
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-purple-900 mb-4">{"Eventos"}</h3>
                            <ul class="space-y-2 text-purple-800/70 text-sm font-medium">
                                <li class="hover:text-purple-600 cursor-pointer transition-colors">{"• Difusión Cultural"}</li>
                            </ul>
                        </div>

                        <div class="group p-8 rounded-3xl bg-amber-50/50 border border-amber-100 hover:bg-amber-100/50 hover:shadow-xl transition-all duration-300">
                            <div class="w-12 h-12 bg-amber-500 rounded-2xl flex items-center justify-center text-white mb-6 group-hover:rotate-6 transition-transform shadow-lg shadow-amber-200">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-amber-900 mb-4">{"Avisos"}</h3>
                            <ul class="space-y-2 text-amber-800/70 text-sm font-medium">
                                <li class="hover:text-amber-600 cursor-pointer transition-colors">{"• Convocatorias"}</li>
                                <li class="hover:text-amber-600 cursor-pointer transition-colors">{"• Certificaciones"}</li>
                                <li class="hover:text-amber-600 cursor-pointer transition-colors">{"• Exámenes requisito"}</li>
                            </ul>
                        </div>

                        <div class="group p-8 rounded-3xl bg-emerald-50/50 border border-emerald-100 hover:bg-emerald-100/50 hover:shadow-xl transition-all duration-300">
                            <div class="w-12 h-12 bg-emerald-500 rounded-2xl flex items-center justify-center text-white mb-6 group-hover:rotate-6 transition-transform shadow-lg shadow-emerald-200">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-emerald-900 mb-4">{"Más"}</h3>
                            <ul class="space-y-2 text-emerald-800/70 text-sm font-medium">
                                <li class="hover:text-emerald-600 cursor-pointer transition-colors">{"• Departamentos"}</li>
                                <li class="hover:text-emerald-600 cursor-pointer transition-colors font-bold italic">{"• ¿Quiénes somos?"}</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}