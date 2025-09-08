use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use web_sys::window;
use gloo::timers::callback::Interval;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTimer {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ticket_id: Option<Uuid>,
    pub ticket_subject: Option<String>,
    pub project_id: Option<Uuid>,
    pub project_name: Option<String>,
    pub client_name: Option<String>,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub elapsed_minutes: i32,
    pub billable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStats {
    pub total_hours_today: f64,
    pub billable_hours_today: f64,
    pub total_hours_week: f64,
    pub billable_hours_week: f64,
    pub unbilled_amount: f64,
    pub active_timers: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerStartRequest {
    pub ticket_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub description: Option<String>,
    pub billable: Option<bool>,
}

#[derive(Properties, PartialEq)]
pub struct TimeTrackerProps {
    #[prop_or_default]
    pub compact: bool,
}

pub enum TimeTrackerMsg {
    LoadStats,
    LoadActiveTimers,
    StartTimer(TimerStartRequest),
    StopTimer(Option<Uuid>),
    SwitchTimer(TimerStartRequest),
    UpdateElapsedTime,
    StatsLoaded(TimeStats),
    ActiveTimersLoaded(Vec<ActiveTimer>),
    TimerStarted(ActiveTimer),
    TimerStopped,
    Error(String),
}

pub struct TimeTracker {
    stats: TimeStats,
    active_timers: Vec<ActiveTimer>,
    loading: bool,
    show_start_form: bool,
    _interval: Option<Interval>,
}

impl Component for TimeTracker {
    type Message = TimeTrackerMsg;
    type Properties = TimeTrackerProps;

    fn create(ctx: &Context<Self>) -> Self {
        // Start interval to update elapsed time every minute
        let link = ctx.link().clone();
        let interval = Interval::new(60_000, move || {
            link.send_message(TimeTrackerMsg::UpdateElapsedTime);
        });

        // Load initial data
        ctx.link().send_message(TimeTrackerMsg::LoadStats);
        ctx.link().send_message(TimeTrackerMsg::LoadActiveTimers);

        Self {
            stats: TimeStats {
                total_hours_today: 0.0,
                billable_hours_today: 0.0,
                total_hours_week: 0.0,
                billable_hours_week: 0.0,
                unbilled_amount: 0.0,
                active_timers: 0,
            },
            active_timers: Vec::new(),
            loading: true,
            show_start_form: false,
            _interval: Some(interval),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimeTrackerMsg::LoadStats => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match Request::get("/api/v1/time/stats").send().await {
                        Ok(response) => {
                            if let Ok(stats) = response.json::<TimeStats>().await {
                                link.send_message(TimeTrackerMsg::StatsLoaded(stats));
                            } else {
                                link.send_message(TimeTrackerMsg::Error("Failed to parse stats".to_string()));
                            }
                        }
                        Err(e) => {
                            link.send_message(TimeTrackerMsg::Error(format!("Failed to load stats: {:?}", e)));
                        }
                    }
                });
                false
            }
            TimeTrackerMsg::LoadActiveTimers => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match Request::get("/api/v1/time/timer/active").send().await {
                        Ok(response) => {
                            if let Ok(timers) = response.json::<Vec<ActiveTimer>>().await {
                                link.send_message(TimeTrackerMsg::ActiveTimersLoaded(timers));
                            } else {
                                link.send_message(TimeTrackerMsg::Error("Failed to parse timers".to_string()));
                            }
                        }
                        Err(e) => {
                            link.send_message(TimeTrackerMsg::Error(format!("Failed to load timers: {:?}", e)));
                        }
                    }
                });
                false
            }
            TimeTrackerMsg::StartTimer(request) => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match Request::post("/api/v1/time/timer/start")
                        .json(&request)
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if let Ok(timer) = response.json::<ActiveTimer>().await {
                                link.send_message(TimeTrackerMsg::TimerStarted(timer));
                            } else {
                                link.send_message(TimeTrackerMsg::Error("Failed to start timer".to_string()));
                            }
                        }
                        Err(e) => {
                            link.send_message(TimeTrackerMsg::Error(format!("Failed to start timer: {:?}", e)));
                        }
                    }
                });
                false
            }
            TimeTrackerMsg::StopTimer(timer_id) => {
                let link = ctx.link().clone();
                let request_body = if let Some(id) = timer_id {
                    serde_json::json!({"timer_id": id})
                } else {
                    serde_json::json!({})
                };
                
                spawn_local(async move {
                    match Request::post("/api/v1/time/timer/stop")
                        .json(&request_body)
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(_) => {
                            link.send_message(TimeTrackerMsg::TimerStopped);
                        }
                        Err(e) => {
                            link.send_message(TimeTrackerMsg::Error(format!("Failed to stop timer: {:?}", e)));
                        }
                    }
                });
                false
            }
            TimeTrackerMsg::UpdateElapsedTime => {
                // Update elapsed time for all active timers
                for timer in &mut self.active_timers {
                    let now = js_sys::Date::new_0().get_time() as i64;
                    let start_time = timer.start_time.timestamp_millis();
                    let elapsed_ms = now - start_time;
                    timer.elapsed_minutes = (elapsed_ms / 60000) as i32;
                }
                true
            }
            TimeTrackerMsg::StatsLoaded(stats) => {
                self.stats = stats;
                self.loading = false;
                true
            }
            TimeTrackerMsg::ActiveTimersLoaded(timers) => {
                self.active_timers = timers;
                self.loading = false;
                true
            }
            TimeTrackerMsg::TimerStarted(timer) => {
                self.active_timers.insert(0, timer);
                self.show_start_form = false;
                ctx.link().send_message(TimeTrackerMsg::LoadStats);
                true
            }
            TimeTrackerMsg::TimerStopped => {
                ctx.link().send_message(TimeTrackerMsg::LoadActiveTimers);
                ctx.link().send_message(TimeTrackerMsg::LoadStats);
                true
            }
            TimeTrackerMsg::Error(error) => {
                // TODO: Show error toast/notification
                web_sys::console::error_1(&format!("Time Tracker Error: {}", error).into());
                false
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().compact {
            self.view_compact(ctx)
        } else {
            self.view_full(ctx)
        }
    }
}

impl TimeTracker {
    fn view_compact(&self, ctx: &Context<Self>) -> Html {
        let on_start_quick_timer = ctx.link().callback(|_| {
            TimeTrackerMsg::StartTimer(TimerStartRequest {
                ticket_id: None,
                project_id: None,
                task_id: None,
                description: Some("Quick Timer".to_string()),
                billable: Some(true),
            })
        });

        let on_stop_timer = ctx.link().callback(|_| TimeTrackerMsg::StopTimer(None));

        html! {
            <div class="bg-white border-l-4 border-blue-500 p-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4">
                        <div class="text-sm">
                            <div class="font-medium text-gray-900">
                                {format!("{:.1}h Today", self.stats.total_hours_today)}
                            </div>
                            <div class="text-gray-500">
                                {format!("{:.1}h Billable", self.stats.billable_hours_today)}
                            </div>
                        </div>
                        
                        if !self.active_timers.is_empty() {
                            <div class="flex items-center space-x-2">
                                {for self.active_timers.iter().map(|timer| {
                                    let elapsed_hours = timer.elapsed_minutes as f64 / 60.0;
                                    html! {
                                        <div class="flex items-center space-x-2 bg-green-100 px-3 py-1 rounded-full">
                                            <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                                            <span class="text-sm font-medium text-green-800">
                                                {format!("{:.1}h", elapsed_hours)}
                                            </span>
                                            if let Some(ref subject) = timer.ticket_subject {
                                                <span class="text-xs text-green-600 truncate max-w-32">
                                                    {subject}
                                                </span>
                                            }
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    </div>
                    
                    <div class="flex items-center space-x-2">
                        if self.active_timers.is_empty() {
                            <button 
                                onclick={on_start_quick_timer}
                                class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500">
                                <span class="text-lg mr-1">{"▶"}</span>
                                {"Start Timer"}
                            </button>
                        } else {
                            <button 
                                onclick={on_stop_timer}
                                class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                                <span class="text-lg mr-1">{"⏹"}</span>
                                {"Stop Timer"}
                            </button>
                        }
                    </div>
                </div>
            </div>
        }
    }

    fn view_full(&self, ctx: &Context<Self>) -> Html {
        let on_toggle_start_form = ctx.link().callback(|_| TimeTrackerMsg::UpdateElapsedTime); // Placeholder

        html! {
            <div class="bg-white shadow rounded-lg">
                <div class="px-4 py-5 sm:p-6">
                    <div class="sm:flex sm:items-center sm:justify-between">
                        <div>
                            <h3 class="text-lg leading-6 font-medium text-gray-900">{"Time Tracking"}</h3>
                            <p class="mt-1 max-w-2xl text-sm text-gray-500">{"Track time and manage your workday"}</p>
                        </div>
                        <div class="mt-4 sm:mt-0">
                            <button 
                                onclick={on_toggle_start_form}
                                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
                                {"Start New Timer"}
                            </button>
                        </div>
                    </div>

                    // Stats cards
                    <div class="mt-6 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
                        <div class="bg-gray-50 overflow-hidden shadow rounded-lg">
                            <div class="p-5">
                                <div class="flex items-center">
                                    <div class="flex-shrink-0">
                                        <div class="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
                                            <span class="text-white text-sm font-semibold">{"⏰"}</span>
                                        </div>
                                    </div>
                                    <div class="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt class="text-sm font-medium text-gray-500 truncate">{"Hours Today"}</dt>
                                            <dd class="text-lg font-medium text-gray-900">{format!("{:.1}", self.stats.total_hours_today)}</dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-gray-50 overflow-hidden shadow rounded-lg">
                            <div class="p-5">
                                <div class="flex items-center">
                                    <div class="flex-shrink-0">
                                        <div class="w-8 h-8 bg-green-500 rounded-lg flex items-center justify-center">
                                            <span class="text-white text-sm font-semibold">{"💰"}</span>
                                        </div>
                                    </div>
                                    <div class="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt class="text-sm font-medium text-gray-500 truncate">{"Billable Today"}</dt>
                                            <dd class="text-lg font-medium text-gray-900">{format!("{:.1}h", self.stats.billable_hours_today)}</dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-gray-50 overflow-hidden shadow rounded-lg">
                            <div class="p-5">
                                <div class="flex items-center">
                                    <div class="flex-shrink-0">
                                        <div class="w-8 h-8 bg-purple-500 rounded-lg flex items-center justify-center">
                                            <span class="text-white text-sm font-semibold">{"📅"}</span>
                                        </div>
                                    </div>
                                    <div class="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt class="text-sm font-medium text-gray-500 truncate">{"Week Total"}</dt>
                                            <dd class="text-lg font-medium text-gray-900">{format!("{:.1}h", self.stats.total_hours_week)}</dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-gray-50 overflow-hidden shadow rounded-lg">
                            <div class="p-5">
                                <div class="flex items-center">
                                    <div class="flex-shrink-0">
                                        <div class="w-8 h-8 bg-yellow-500 rounded-lg flex items-center justify-center">
                                            <span class="text-white text-sm font-semibold">{"💵"}</span>
                                        </div>
                                    </div>
                                    <div class="ml-5 w-0 flex-1">
                                        <dl>
                                            <dt class="text-sm font-medium text-gray-500 truncate">{"Unbilled"}</dt>
                                            <dd class="text-lg font-medium text-gray-900">{format!("${:.0}", self.stats.unbilled_amount)}</dd>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Active timers
                    if !self.active_timers.is_empty() {
                        <div class="mt-6">
                            <h4 class="text-sm font-medium text-gray-900 mb-4">{"Active Timers"}</h4>
                            <div class="space-y-3">
                                {for self.active_timers.iter().map(|timer| {
                                    let elapsed_hours = timer.elapsed_minutes as f64 / 60.0;
                                    let on_stop = ctx.link().callback({
                                        let timer_id = timer.id;
                                        move |_| TimeTrackerMsg::StopTimer(Some(timer_id))
                                    });
                                    
                                    html! {
                                        <div class="bg-green-50 border border-green-200 rounded-lg p-4">
                                            <div class="flex items-center justify-between">
                                                <div class="flex items-center space-x-3">
                                                    <div class="w-3 h-3 bg-green-500 rounded-full animate-pulse"></div>
                                                    <div>
                                                        <div class="text-sm font-medium text-gray-900">
                                                            {timer.ticket_subject.as_deref().unwrap_or("General Time")}
                                                        </div>
                                                        if let Some(ref client) = timer.client_name {
                                                            <div class="text-xs text-gray-500">{client}</div>
                                                        }
                                                        if let Some(ref desc) = timer.description {
                                                            <div class="text-xs text-gray-600 mt-1">{desc}</div>
                                                        }
                                                    </div>
                                                </div>
                                                <div class="flex items-center space-x-4">
                                                    <div class="text-right">
                                                        <div class="text-lg font-semibold text-green-800">
                                                            {format!("{:.1}h", elapsed_hours)}
                                                        </div>
                                                        <div class="text-xs text-green-600">
                                                            {if timer.billable { "Billable" } else { "Non-billable" }}
                                                        </div>
                                                    </div>
                                                    <button 
                                                        onclick={on_stop}
                                                        class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-red-600 hover:bg-red-700">
                                                        {"Stop"}
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }
                </div>
            </div>
        }
    }

    fn format_elapsed_time(minutes: i32) -> String {
        let hours = minutes / 60;
        let mins = minutes % 60;
        if hours > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}m", mins)
        }
    }
}