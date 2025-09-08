use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod services;
mod utils;

use components::{layout::Layout, auth::{AuthProvider, LoginForm, AuthContext}};
use pages::{clients::ClientsPage, dashboard::DashboardPage, tickets::TicketsPage, time_tracking::TimeTrackingPage};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/login")]
    Login,
    #[at("/clients")]
    Clients,
    #[at("/tickets")]
    Tickets,
    #[at("/time")]
    TimeTracking,
    #[at("/assets")]
    Assets,
    #[at("/invoices")]
    Invoices,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Clients => html! { <ClientsPage /> },
        Route::Tickets => html! { <TicketsPage /> },
        Route::TimeTracking => html! { <TimeTrackingPage /> },
        Route::Assets => html! { <div class="p-6"><h1 class="text-2xl font-bold">{"Assets"}</h1><p>{"Asset management coming soon..."}</p></div> },
        Route::Invoices => html! { <div class="p-6"><h1 class="text-2xl font-bold">{"Invoices"}</h1><p>{"Invoice management coming soon..."}</p></div> },
        Route::Projects => html! { <div class="p-6"><h1 class="text-2xl font-bold">{"Projects"}</h1><p>{"Project management coming soon..."}</p></div> },
        Route::NotFound => html! { 
            <div class="min-h-screen flex items-center justify-center">
                <div class="text-center">
                    <h1 class="text-6xl font-bold text-gray-900">{"404"}</h1>
                    <p class="text-xl text-gray-600 mt-4">{"Page Not Found"}</p>
                </div>
            </div>
        },
    }
}

#[function_component(LoginPage)]
fn login_page() -> Html {
    let navigator = use_navigator().unwrap();
    
    let on_login = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Dashboard);
        })
    };
    
    html! {
        <LoginForm {on_login} />
    }
}

#[function_component(AppRouter)]
fn app_router() -> Html {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext not found");
    
    // If not authenticated, show login page
    if auth_ctx.user.is_none() {
        return html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        };
    }
    
    // If authenticated, show main app with layout
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <AuthProvider>
            <AppRouter />
        </AuthProvider>
    }
}

fn main() {
    // Set up Tailwind CSS
    let document = web_sys::window().unwrap().document().unwrap();
    let head = document.head().unwrap();
    
    let link = document.create_element("link").unwrap();
    link.set_attribute("href", "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css").unwrap();
    link.set_attribute("rel", "stylesheet").unwrap();
    head.append_child(&link).unwrap();
    
    yew::Renderer::<App>::new().render();
}