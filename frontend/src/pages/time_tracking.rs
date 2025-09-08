use yew::prelude::*;
use crate::components::time_tracker::TimeTracker;

#[function_component(TimeTrackingPage)]
pub fn time_tracking_page() -> Html {
    html! {
        <div>
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-gray-900">{"Time Tracking"}</h1>
                <p class="mt-2 text-gray-600">{"Track your time, manage projects, and monitor productivity"}</p>
            </div>
            
            // Main time tracker component
            <TimeTracker compact={false} />
            
            // TODO: Add timesheet table, recent entries, etc.
        </div>
    }
}