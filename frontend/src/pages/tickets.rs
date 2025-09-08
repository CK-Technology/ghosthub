use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketWithDetails {
    pub id: Uuid,
    pub number: i32,
    pub client_id: Uuid,
    pub client_name: String,
    pub contact_id: Option<Uuid>,
    pub contact_name: Option<String>,
    pub subject: String,
    pub status: String,
    pub priority: String,
    pub category_name: Option<String>,
    pub assigned_name: Option<String>,
    pub response_due_at: Option<DateTime<Utc>>,
    pub sla_breached: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketStats {
    pub total_tickets: i64,
    pub open_tickets: i64,
    pub overdue_tickets: i64,
    pub sla_breached: i64,
}

#[function_component(TicketsPage)]
pub fn tickets_page() -> Html {
    let tickets = use_state(|| Vec::<TicketWithDetails>::new());
    let stats = use_state(|| TicketStats {
        total_tickets: 0,
        open_tickets: 0,
        overdue_tickets: 0,
        sla_breached: 0,
    });
    let loading = use_state(|| true);
    let filter_status = use_state(|| "all".to_string());
    let filter_priority = use_state(|| "all".to_string());

    // Load tickets and stats
    {
        let tickets = tickets.clone();
        let stats = stats.clone();
        let loading = loading.clone();
        let filter_status = filter_status.clone();
        let filter_priority = filter_priority.clone();
        
        use_effect_with(((*filter_status).clone(), (*filter_priority).clone()), move |_| {
            spawn_local(async move {
                // Load stats
                if let Ok(response) = Request::get("/api/v1/tickets/stats").send().await {
                    if let Ok(data) = response.json::<TicketStats>().await {
                        stats.set(data);
                    }
                }
                
                // Build query parameters
                let mut query_params = vec![];
                if *filter_status != "all" {
                    query_params.push(format!("status={}", *filter_status));
                }
                if *filter_priority != "all" {
                    query_params.push(format!("priority={}", *filter_priority));
                }
                
                let query_string = if query_params.is_empty() {
                    String::new()
                } else {
                    format!("?{}", query_params.join("&"))
                };
                
                // Load tickets
                match Request::get(&format!("/api/v1/tickets{}", query_string))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(data) = response.json::<Vec<TicketWithDetails>>().await {
                            tickets.set(data);
                        }
                        loading.set(false);
                    }
                    Err(_) => {
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    let on_status_filter_change = {
        let filter_status = filter_status.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            filter_status.set(input.value());
        })
    };

    let on_priority_filter_change = {
        let filter_priority = filter_priority.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            filter_priority.set(input.value());
        })
    };

    let priority_badge_class = |priority: &str| -> &'static str {
        match priority {
            "critical" => "bg-red-100 text-red-800",
            "high" => "bg-orange-100 text-orange-800",
            "medium" => "bg-yellow-100 text-yellow-800",
            "low" => "bg-green-100 text-green-800",
            _ => "bg-gray-100 text-gray-800",
        }
    };

    let status_badge_class = |status: &str| -> &'static str {
        match status {
            "open" => "bg-blue-100 text-blue-800",
            "in_progress" => "bg-yellow-100 text-yellow-800",
            "resolved" => "bg-green-100 text-green-800",
            "closed" => "bg-gray-100 text-gray-800",
            _ => "bg-gray-100 text-gray-800",
        }
    };

    html! {
        <div>
            // Header with stats
            <div class="mb-8">
                <div class="sm:flex sm:items-center sm:justify-between">
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">{"Tickets"}</h1>
                        <p class="mt-2 text-sm text-gray-700">{"Manage support tickets and track SLAs"}</p>
                    </div>
                    <div class="mt-4 sm:mt-0">
                        <button type="button" class="block rounded-md bg-blue-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-sm hover:bg-blue-500">
                            {"New Ticket"}
                        </button>
                    </div>
                </div>

                // Stats cards
                <div class="mt-6 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
                    <div class="bg-white overflow-hidden shadow rounded-lg">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <div class="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
                                        <span class="text-white text-sm font-semibold">{"T"}</span>
                                    </div>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-sm font-medium text-gray-500 truncate">{"Total Tickets"}</dt>
                                        <dd class="text-lg font-medium text-gray-900">{stats.total_tickets}</dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white overflow-hidden shadow rounded-lg">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <div class="w-8 h-8 bg-green-500 rounded-lg flex items-center justify-center">
                                        <span class="text-white text-sm font-semibold">{"O"}</span>
                                    </div>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-sm font-medium text-gray-500 truncate">{"Open Tickets"}</dt>
                                        <dd class="text-lg font-medium text-gray-900">{stats.open_tickets}</dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white overflow-hidden shadow rounded-lg">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <div class="w-8 h-8 bg-orange-500 rounded-lg flex items-center justify-center">
                                        <span class="text-white text-sm font-semibold">{"⚠"}</span>
                                    </div>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-sm font-medium text-gray-500 truncate">{"Overdue"}</dt>
                                        <dd class="text-lg font-medium text-gray-900">{stats.overdue_tickets}</dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white overflow-hidden shadow rounded-lg">
                        <div class="p-5">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <div class="w-8 h-8 bg-red-500 rounded-lg flex items-center justify-center">
                                        <span class="text-white text-sm font-semibold">{"!"}</span>
                                    </div>
                                </div>
                                <div class="ml-5 w-0 flex-1">
                                    <dl>
                                        <dt class="text-sm font-medium text-gray-500 truncate">{"SLA Breached"}</dt>
                                        <dd class="text-lg font-medium text-gray-900">{stats.sla_breached}</dd>
                                    </dl>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Filters
            <div class="bg-white shadow rounded-lg p-4 mb-6">
                <div class="flex flex-wrap gap-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">{"Status"}</label>
                        <select onchange={on_status_filter_change} class="block w-full rounded-md border-gray-300 py-2 pl-3 pr-10 text-base focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm">
                            <option value="all">{"All Statuses"}</option>
                            <option value="open">{"Open"}</option>
                            <option value="in_progress">{"In Progress"}</option>
                            <option value="resolved">{"Resolved"}</option>
                            <option value="closed">{"Closed"}</option>
                        </select>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">{"Priority"}</label>
                        <select onchange={on_priority_filter_change} class="block w-full rounded-md border-gray-300 py-2 pl-3 pr-10 text-base focus:border-blue-500 focus:outline-none focus:ring-blue-500 sm:text-sm">
                            <option value="all">{"All Priorities"}</option>
                            <option value="critical">{"Critical"}</option>
                            <option value="high">{"High"}</option>
                            <option value="medium">{"Medium"}</option>
                            <option value="low">{"Low"}</option>
                        </select>
                    </div>
                </div>
            </div>

            // Tickets table
            <div class="bg-white shadow rounded-lg overflow-hidden">
                if *loading {
                    <div class="p-6">
                        <p class="text-gray-500">{"Loading tickets..."}</p>
                    </div>
                } else if tickets.is_empty() {
                    <div class="p-6">
                        <p class="text-gray-500">{"No tickets found matching your filters."}</p>
                    </div>
                } else {
                    <div class="overflow-x-auto">
                        <table class="min-w-full divide-y divide-gray-200">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Ticket"}</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Client"}</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Subject"}</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Status"}</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Priority"}</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Assigned"}</th>
                                    <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Due"}</th>
                                    <th class="relative px-6 py-3"><span class="sr-only">{"Actions"}</span></th>
                                </tr>
                            </thead>
                            <tbody class="bg-white divide-y divide-gray-200">
                                {tickets.iter().map(|ticket| {
                                    let sla_class = if ticket.sla_breached { "bg-red-50" } else { "" };
                                    html! {
                                        <tr key={ticket.id.to_string()} class={format!("hover:bg-gray-50 {}", sla_class)}>
                                            <td class="px-6 py-4 whitespace-nowrap">
                                                <div class="flex items-center">
                                                    <div class="text-sm font-medium text-gray-900">{format!("#{}", ticket.number)}</div>
                                                    if ticket.sla_breached {
                                                        <span class="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-100 text-red-800">
                                                            {"SLA Breach"}
                                                        </span>
                                                    }
                                                </div>
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{&ticket.client_name}</td>
                                            <td class="px-6 py-4">
                                                <div class="text-sm text-gray-900 truncate max-w-xs">{&ticket.subject}</div>
                                                if let Some(contact) = &ticket.contact_name {
                                                    <div class="text-sm text-gray-500">{contact}</div>
                                                }
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap">
                                                <span class={format!("inline-flex px-2 py-1 text-xs font-semibold rounded-full {}", status_badge_class(&ticket.status))}>
                                                    {&ticket.status}
                                                </span>
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap">
                                                <span class={format!("inline-flex px-2 py-1 text-xs font-semibold rounded-full {}", priority_badge_class(&ticket.priority))}>
                                                    {&ticket.priority}
                                                </span>
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {ticket.assigned_name.as_deref().unwrap_or("Unassigned")}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                if let Some(due_date) = &ticket.response_due_at {
                                                    {format!("{}", due_date.format("%m/%d %H:%M"))}
                                                } else {
                                                    {"-"}
                                                }
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                                <a href={format!("#/tickets/{}", ticket.id)} class="text-blue-600 hover:text-blue-900">
                                                    {"View"}
                                                </a>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()}
                            </tbody>
                        </table>
                    </div>
                }
            </div>
        </div>
    }
}