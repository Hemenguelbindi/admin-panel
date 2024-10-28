use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login::LoginPage;
use crate::pages::dashbord::Dashbord;
use crate::pages::setting::Settings;
use crate::pages::users::Users;


#[derive(Clone, Routable, PartialEq)]
pub enum Route{
    #[at("/")]
    Login,
    #[at("/dashboard")]
    Dashboard,
    #[at("/users")]
    Users,
    #[at("/settings")]
    Settings,
}


pub fn switch(route: Route) -> Html{
    match route{
        Route::Login => html!{<LoginPage/>},
        Route::Dashboard => html!{<Dashbord/>},
        Route::Users => html!{<Users/>},
        Route::Settings => html!{<Settings/>},
    }
}