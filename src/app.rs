use yew::{function_component, Html, html};
use yew_router::prelude::*;
use crate::routes::{switch, Route};


#[function_component(AdminPanel)]
pub fn app () -> Html{
    html!{
        <div class="w-screen h-screen flex flex-col items-center justify-center dark:bg-slate-800 bg-gray-100">
            <BrowserRouter>
                <Switch <Route> render={switch}/>
            </BrowserRouter>
        </div>
    }
}
