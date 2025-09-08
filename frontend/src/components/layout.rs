use yew::prelude::*;
use yew_router::prelude::*;
use super::{time_tracker::TimeTracker, AuthContext};

// Define Route here for now, will be moved to a routes module later
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext not found");
    let current_route = use_route::<Route>().unwrap_or(Route::Dashboard);
    
    let is_active = |route: &Route| -> &'static str {
        if route == &current_route {
            "bg-blue-900 text-white px-3 py-2 rounded-md text-sm font-medium"
        } else {
            "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
        }
    };
    
    html! {
        <div class="min-h-screen bg-gray-50">
            // Modern sidebar navigation
            <div class="flex">
                // Sidebar
                <div class="w-64 bg-gray-800 min-h-screen">
                    // Logo
                    <div class="flex items-center px-6 py-4 border-b border-gray-700">
                        <div class="flex items-center space-x-3">
                            <div class="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
                                <span class="text-white font-bold text-lg">{"G"}</span>
                            </div>
                            <span class="text-white text-xl font-bold">{"GhostHub"}</span>
                        </div>
                    </div>
                    
                    // Navigation
                    <nav class="mt-6 px-3">
                        <div class="space-y-1">
                            <Link<Route> to={Route::Dashboard} classes={is_active(&Route::Dashboard)}>
                                <div class="flex items-center">
                                    <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"/>
                                    </svg>
                                    {"Dashboard"}
                                </div>
                            </Link<Route>>
                            
                            <Link<Route> to={Route::Clients} classes={is_active(&Route::Clients)}>
                                <div class="flex items-center">
                                    <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M9 6a3 3 0 11-6 0 3 3 0 016 0zM17 6a3 3 0 11-6 0 3 3 0 016 0zM12.93 17c.046-.327.07-.66.07-1a6.97 6.97 0 00-1.5-4.33A5 5 0 0119 16v1h-6.07zM6 11a5 5 0 015 5v1H1v-1a5 5 0 015-5z"/>
                                    </svg>
                                    {"Clients"}
                                </div>
                            </Link<Route>>
                            
                            <Link<Route> to={Route::Tickets} classes={is_active(&Route::Tickets)}>
                                <div class="flex items-center">
                                    <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                                    </svg>
                                    {"Tickets"}
                                </div>
                            </Link<Route>>
                            
                            <Link<Route> to={Route::TimeTracking} classes={is_active(&Route::TimeTracking)}>
                                <div class="flex items-center">
                                    <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"/>
                                    </svg>
                                    {"Time Tracking"}
                                </div>
                            </Link<Route>>
                            
                            <Link<Route> to={Route::Assets} classes={is_active(&Route::Assets)}>
                                <div class="flex items-center">
                                    <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"/>
                                    </svg>
                                    {"Assets"}
                                </div>
                            </Link<Route>>
                            
                            <Link<Route> to={Route::Projects} classes={is_active(&Route::Projects)}>
                                <div class="flex items-center">
                                    <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M7 3a1 1 0 000 2h6a1 1 0 100-2H7zM4 7a1 1 0 011-1h10a1 1 0 110 2H5a1 1 0 01-1-1zM2 11a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4z"/>
                                    </svg>
                                    {"Projects"}
                                </div>
                            </Link<Route>>
                            
                            <Link<Route> to={Route::Invoices} classes={is_active(&Route::Invoices)}>
                                <div class="flex items-center">
                                    <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M4 4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2H4zM18 9H2v5a2 2 0 002 2h12a2 2 0 002-2V9zM4 13a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zm5-1a1 1 0 100 2h1a1 1 0 100-2H9z"/>
                                    </svg>
                                    {"Invoices"}
                                </div>
                            </Link<Route>>
                        </div>
                        
                        // Time tracker in sidebar
                        <div class="mt-8 px-3">
                            <div class="bg-gray-700 rounded-lg p-4">
                                <h4 class="text-sm font-medium text-gray-300 mb-3">{"Quick Timer"}</h4>
                                <TimeTracker compact={true} />
                            </div>
                        </div>
                    </nav>
                </div>
                
                // Main content area
                <div class="flex-1">
                    // Top header
                    <div class="bg-white shadow-sm border-b">
                        <div class="px-6 py-4">
                            <div class="flex justify-between items-center">
                                <div>
                                    // Breadcrumb or page title will go here
                                </div>
                                
                                // User menu
                                <div class="flex items-center space-x-4">
                                    <div class="text-sm text-gray-600">
                                        {"Welcome, "}{auth_ctx.user.as_ref().map(|u| format!("{} {}", u.first_name, u.last_name)).unwrap_or_else(|| "User".to_string())}
                                    </div>
                                    <button
                                        onclick={auth_ctx.logout.reform(|_| ())}
                                        class="bg-gray-100 hover:bg-gray-200 px-3 py-1 rounded text-sm"
                                    >
                                        {"Logout"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    // Page content
                    <main class="p-6">
                        { props.children.clone() }
                    </main>
                </div>
            </div>
        </div>
    }
}