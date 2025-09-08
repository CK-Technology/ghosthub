use yew::prelude::*;
use yew_hooks::{use_effect_once, use_state_eq};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DashboardStats {
    pub overview: OverviewStats,
    pub tickets: TicketStats,
    pub time: TimeStats,
    pub invoices: InvoiceStats,
    pub clients: ClientStats,
    pub assets: AssetStats,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OverviewStats {
    pub total_clients: i64,
    pub active_tickets: i64,
    pub monthly_revenue: Decimal,
    pub unbilled_time: Decimal,
    pub overdue_invoices: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TicketStats {
    pub open: i64,
    pub in_progress: i64,
    pub pending: i64,
    pub resolved_today: i64,
    pub sla_breached: i64,
    pub avg_response_time_hours: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TimeStats {
    pub hours_today: Decimal,
    pub billable_hours_today: Decimal,
    pub hours_this_week: Decimal,
    pub active_timers: i64,
    pub team_utilization: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InvoiceStats {
    pub outstanding_amount: Decimal,
    pub overdue_amount: Decimal,
    pub draft_count: i64,
    pub paid_this_month: Decimal,
    pub collection_ratio: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ClientStats {
    pub total_clients: i64,
    pub new_this_month: i64,
    pub top_clients_by_revenue: Vec<TopClient>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TopClient {
    pub name: String,
    pub revenue: Decimal,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AssetStats {
    pub total_assets: i64,
    pub critical_alerts: i64,
    pub warranty_expiring: i64,
    pub online_percentage: Option<f64>,
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let stats = use_state(|| None::<DashboardStats>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    // Fetch dashboard stats on component mount
    {
        let stats = stats.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get("http://localhost:8080/api/v1/dashboard")
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                match response.json::<DashboardStats>().await {
                                    Ok(data) => {
                                        stats.set(Some(data));
                                        loading.set(false);
                                    }
                                    Err(e) => {
                                        error.set(Some(format!("Failed to parse response: {}", e)));
                                        loading.set(false);
                                    }
                                }
                            } else {
                                error.set(Some(format!("Request failed: {}", response.status())));
                                loading.set(false);
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Network error: {}", e)));
                            loading.set(false);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }
    
    if *loading {
        return html! {
            <div class="flex justify-center items-center h-64">
                <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-500"></div>
            </div>
        };
    }
    
    if let Some(error_msg) = (*error).clone() {
        return html! {
            <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                {"Error loading dashboard: "}{error_msg}
            </div>
        };
    }
    
    let stats_data = match (*stats).clone() {
        Some(data) => data,
        None => return html! { <div>{"No data available"}</div> },
    };
    
    html! {
        <div class="space-y-6">
            // Header
            <div class="bg-white shadow rounded-lg">
                <div class="px-6 py-4">
                    <h1 class="text-2xl font-bold text-gray-900">{"Dashboard"}</h1>
                    <p class="text-gray-600">{"Overview of your MSP operations"}</p>
                </div>
            </div>
            
            // Key metrics cards
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <MetricCard
                    title="Active Clients"
                    value={stats_data.overview.total_clients.to_string()}
                    icon="users"
                    color="blue"
                />
                <MetricCard
                    title="Open Tickets"
                    value={stats_data.overview.active_tickets.to_string()}
                    icon="ticket"
                    color="yellow"
                />
                <MetricCard
                    title="Monthly Revenue"
                    value={format!("${}", stats_data.overview.monthly_revenue)}
                    icon="dollar"
                    color="green"
                />
                <MetricCard
                    title="Unbilled Time"
                    value={format!("${}", stats_data.overview.unbilled_time)}
                    icon="clock"
                    color="orange"
                />
            </div>
            
            // Charts and detailed stats
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // Ticket stats
                <div class="bg-white shadow rounded-lg">
                    <div class="px-6 py-4 border-b border-gray-200">
                        <h3 class="text-lg font-medium text-gray-900">{"Tickets"}</h3>
                    </div>
                    <div class="p-6">
                        <div class="grid grid-cols-2 gap-4">
                            <div class="text-center">
                                <div class="text-2xl font-bold text-blue-600">{stats_data.tickets.open}</div>
                                <div class="text-sm text-gray-500">{"Open"}</div>
                            </div>
                            <div class="text-center">
                                <div class="text-2xl font-bold text-yellow-600">{stats_data.tickets.in_progress}</div>
                                <div class="text-sm text-gray-500">{"In Progress"}</div>
                            </div>
                            <div class="text-center">
                                <div class="text-2xl font-bold text-green-600">{stats_data.tickets.resolved_today}</div>
                                <div class="text-sm text-gray-500">{"Resolved Today"}</div>
                            </div>
                            <div class="text-center">
                                <div class="text-2xl font-bold text-red-600">{stats_data.tickets.sla_breached}</div>
                                <div class="text-sm text-gray-500">{"SLA Breached"}</div>
                            </div>
                        </div>
                    </div>
                </div>
                
                // Time tracking stats
                <div class="bg-white shadow rounded-lg">
                    <div class="px-6 py-4 border-b border-gray-200">
                        <h3 class="text-lg font-medium text-gray-900">{"Time Tracking"}</h3>
                    </div>
                    <div class="p-6">
                        <div class="space-y-4">
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Hours Today"}</span>
                                <span class="font-medium">{format!("{:.1}h", stats_data.time.hours_today)}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Billable Today"}</span>
                                <span class="font-medium text-green-600">{format!("{:.1}h", stats_data.time.billable_hours_today)}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"This Week"}</span>
                                <span class="font-medium">{format!("{:.1}h", stats_data.time.hours_this_week)}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Active Timers"}</span>
                                <span class="font-medium text-blue-600">{stats_data.time.active_timers}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            // Invoice and client stats
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-white shadow rounded-lg">
                    <div class="px-6 py-4 border-b border-gray-200">
                        <h3 class="text-lg font-medium text-gray-900">{"Invoicing"}</h3>
                    </div>
                    <div class="p-6">
                        <div class="space-y-4">
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Outstanding"}</span>
                                <span class="font-medium">{format!("${}", stats_data.invoices.outstanding_amount)}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Overdue"}</span>
                                <span class="font-medium text-red-600">{format!("${}", stats_data.invoices.overdue_amount)}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Draft Invoices"}</span>
                                <span class="font-medium">{stats_data.invoices.draft_count}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Paid This Month"}</span>
                                <span class="font-medium text-green-600">{format!("${}", stats_data.invoices.paid_this_month)}</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="bg-white shadow rounded-lg">
                    <div class="px-6 py-4 border-b border-gray-200">
                        <h3 class="text-lg font-medium text-gray-900">{"Assets & Clients"}</h3>
                    </div>
                    <div class="p-6">
                        <div class="space-y-4">
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Total Assets"}</span>
                                <span class="font-medium">{stats_data.assets.total_assets}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Critical Alerts"}</span>
                                <span class="font-medium text-red-600">{stats_data.assets.critical_alerts}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"Warranty Expiring"}</span>
                                <span class="font-medium text-yellow-600">{stats_data.assets.warranty_expiring}</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-gray-600">{"New Clients (Month)"}</span>
                                <span class="font-medium text-green-600">{stats_data.clients.new_this_month}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MetricCardProps {
    pub title: String,
    pub value: String,
    pub icon: String,
    pub color: String,
}

#[function_component(MetricCard)]
pub fn metric_card(props: &MetricCardProps) -> Html {
    let color_classes = match props.color.as_str() {
        "blue" => "bg-blue-500",
        "green" => "bg-green-500",
        "yellow" => "bg-yellow-500",
        "orange" => "bg-orange-500",
        "red" => "bg-red-500",
        _ => "bg-gray-500",
    };
    
    html! {
        <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="p-5">
                <div class="flex items-center">
                    <div class="flex-shrink-0">
                        <div class={format!("h-8 w-8 rounded-md {} flex items-center justify-center", color_classes)}>
                            <svg class="h-5 w-5 text-white" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z"/>
                                <path d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z"/>
                            </svg>
                        </div>
                    </div>
                    <div class="ml-5 w-0 flex-1">
                        <dl>
                            <dt class="text-sm font-medium text-gray-500 truncate">{&props.title}</dt>
                            <dd class="text-lg font-medium text-gray-900">{&props.value}</dd>
                        </dl>
                    </div>
                </div>
            </div>
        </div>
    }
}