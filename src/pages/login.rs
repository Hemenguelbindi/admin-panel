use yew::prelude::*;



#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html!{
        <div>
            <h1 class="text-sky-300 text-center pb-2 text-xl">{"Login"}</h1>
            <button class="bg-blue-500 hover::bg-blue-400 text-white font-bold py-2 px-4 border-blue-700 hover:border-blue-500">{"Login"}</button>
        </div>
    }
}
