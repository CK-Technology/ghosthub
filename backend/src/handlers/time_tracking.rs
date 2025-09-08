use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, patch, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use rust_decimal::Decimal;
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct TimeEntryCreate {
    pub ticket_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub description: Option<String>,
    pub billable: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntryUpdate {
    pub ticket_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub description: Option<String>,
    pub billable: Option<bool>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ManualTimeEntry {
    pub ticket_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub billable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntryQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub user_id: Option<Uuid>,
    pub ticket_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub billable: Option<bool>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntryWithDetails {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub ticket_id: Option<Uuid>,
    pub ticket_number: Option<i32>,
    pub ticket_subject: Option<String>,
    pub project_id: Option<Uuid>,
    pub project_name: Option<String>,
    pub task_id: Option<Uuid>,
    pub task_name: Option<String>,
    pub client_id: Option<Uuid>,
    pub client_name: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub description: Option<String>,
    pub billable: bool,
    pub billed: bool,
    pub hourly_rate: Option<Decimal>,
    pub total_amount: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct TimeStats {
    pub total_hours_today: Decimal,
    pub billable_hours_today: Decimal,
    pub total_hours_week: Decimal,
    pub billable_hours_week: Decimal,
    pub unbilled_amount: Decimal,
    pub active_timers: i32,
}

pub fn time_tracking_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/entries", get(list_time_entries).post(create_manual_entry))
        .route("/entries/:id", get(get_time_entry).put(update_time_entry).delete(delete_time_entry))
        .route("/timer/start", post(start_timer))
        .route("/timer/stop", post(stop_timer))
        .route("/timer/active", get(get_active_timers))
        .route("/timer/switch", post(switch_timer))
        .route("/stats", get(get_time_stats))
        .route("/timesheet", get(get_timesheet))
}

async fn list_time_entries(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TimeEntryQuery>,
) -> Result<Json<Vec<TimeEntryWithDetails>>, StatusCode> {
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    
    // TODO: Add proper filtering based on query parameters
    match sqlx::query_as!(
        TimeEntryWithDetails,
        "SELECT 
            te.id, te.user_id, u.first_name || ' ' || u.last_name as user_name,
            te.ticket_id, t.number as ticket_number, t.subject as ticket_subject,
            te.project_id, p.name as project_name,
            te.task_id, tk.name as task_name,
            COALESCE(t.client_id, p.client_id) as client_id,
            c.name as client_name,
            te.start_time, te.end_time, te.duration_minutes,
            te.description, te.billable, te.billed,
            te.hourly_rate, te.total_amount,
            te.created_at, te.updated_at
         FROM time_entries te
         LEFT JOIN users u ON te.user_id = u.id
         LEFT JOIN tickets t ON te.ticket_id = t.id
         LEFT JOIN projects p ON te.project_id = p.id
         LEFT JOIN tasks tk ON te.task_id = tk.id
         LEFT JOIN clients c ON COALESCE(t.client_id, p.client_id) = c.id
         ORDER BY te.start_time DESC
         LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(entries) => Ok(Json(entries)),
        Err(e) => {
            tracing::error!("Error fetching time entries: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn start_timer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TimeEntryCreate>,
) -> Result<(StatusCode, Json<ActiveTimer>), StatusCode> {
    let entry_id = Uuid::new_v4();
    // TODO: Get current user from auth context
    let current_user_id = Uuid::new_v4();
    
    let now = Utc::now();
    let billable = payload.billable.unwrap_or(true);
    
    // Stop any existing active timer for this user
    let _ = sqlx::query!(
        "UPDATE time_entries SET 
         end_time = NOW(),
         duration_minutes = EXTRACT(EPOCH FROM (NOW() - start_time)) / 60
         WHERE user_id = $1 AND end_time IS NULL",
        current_user_id
    )
    .execute(&state.db_pool)
    .await;
    
    // Start new timer
    match sqlx::query!(
        "INSERT INTO time_entries (
            id, user_id, ticket_id, project_id, task_id, 
            start_time, description, billable
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        entry_id,
        current_user_id,
        payload.ticket_id,
        payload.project_id,
        payload.task_id,
        now,
        payload.description,
        billable
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => {
            // Fetch the active timer details
            match get_active_timer_by_id(&state, entry_id).await {
                Ok(timer) => Ok((StatusCode::CREATED, Json(timer))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(e) => {
            tracing::error!("Error starting timer: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn stop_timer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<TimeEntryWithDetails>, StatusCode> {
    // TODO: Get current user from auth context
    let current_user_id = Uuid::new_v4();
    
    let timer_id = payload.get("timer_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());
    
    let end_time = Utc::now();
    
    // If timer_id is provided, stop that specific timer, otherwise stop the user's active timer
    let query = if let Some(id) = timer_id {
        sqlx::query!(
            "UPDATE time_entries SET 
             end_time = $2,
             duration_minutes = EXTRACT(EPOCH FROM ($2 - start_time)) / 60
             WHERE id = $1 AND user_id = $3 AND end_time IS NULL
             RETURNING id",
            id,
            end_time,
            current_user_id
        )
    } else {
        sqlx::query!(
            "UPDATE time_entries SET 
             end_time = $2,
             duration_minutes = EXTRACT(EPOCH FROM ($2 - start_time)) / 60
             WHERE user_id = $1 AND end_time IS NULL
             RETURNING id",
            current_user_id,
            end_time
        )
    };
    
    match query.fetch_optional(&state.db_pool).await {
        Ok(Some(row)) => {
            // Calculate billable amount
            let _ = calculate_and_update_billing(&state, row.id).await;
            
            // Fetch the updated entry
            match get_time_entry_by_id(&state, row.id).await {
                Ok(entry) => Ok(Json(entry)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Error stopping timer: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_active_timers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ActiveTimer>>, StatusCode> {
    // TODO: Get current user from auth context or return all active timers for admins
    let current_user_id = Uuid::new_v4();
    
    match sqlx::query!(
        "SELECT 
            te.id, te.user_id, te.ticket_id, t.subject as ticket_subject,
            te.project_id, p.name as project_name,
            c.name as client_name, te.description, te.start_time, te.billable,
            EXTRACT(EPOCH FROM (NOW() - te.start_time)) / 60 as elapsed_minutes
         FROM time_entries te
         LEFT JOIN tickets t ON te.ticket_id = t.id
         LEFT JOIN projects p ON te.project_id = p.id
         LEFT JOIN clients c ON COALESCE(t.client_id, p.client_id) = c.id
         WHERE te.user_id = $1 AND te.end_time IS NULL
         ORDER BY te.start_time DESC",
        current_user_id
    )
    .fetch_all(&state.db_pool)
    .await
    {
        Ok(rows) => {
            let timers = rows.into_iter().map(|row| ActiveTimer {
                id: row.id,
                user_id: row.user_id,
                ticket_id: row.ticket_id,
                ticket_subject: Some(row.ticket_subject),
                project_id: row.project_id,
                project_name: Some(row.project_name),
                client_name: Some(row.client_name),
                description: row.description,
                start_time: row.start_time,
                elapsed_minutes: row.elapsed_minutes.map(|d| d.to_i32().unwrap_or(0)).unwrap_or(0),
                billable: row.billable.unwrap_or(false),
            }).collect();
            
            Ok(Json(timers))
        }
        Err(e) => {
            tracing::error!("Error fetching active timers: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn switch_timer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TimeEntryCreate>,
) -> Result<Json<ActiveTimer>, StatusCode> {
    // TODO: Get current user from auth context
    let current_user_id = Uuid::new_v4();
    
    // Stop current timer if any
    let _ = sqlx::query!(
        "UPDATE time_entries SET 
         end_time = NOW(),
         duration_minutes = EXTRACT(EPOCH FROM (NOW() - start_time)) / 60
         WHERE user_id = $1 AND end_time IS NULL",
        current_user_id
    )
    .execute(&state.db_pool)
    .await;
    
    // Start new timer
    let entry_id = Uuid::new_v4();
    let now = Utc::now();
    let billable = payload.billable.unwrap_or(true);
    
    match sqlx::query!(
        "INSERT INTO time_entries (
            id, user_id, ticket_id, project_id, task_id, 
            start_time, description, billable
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        entry_id,
        current_user_id,
        payload.ticket_id,
        payload.project_id,
        payload.task_id,
        now,
        payload.description,
        billable
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => {
            match get_active_timer_by_id(&state, entry_id).await {
                Ok(timer) => Ok(Json(timer)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(e) => {
            tracing::error!("Error switching timer: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_manual_entry(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ManualTimeEntry>,
) -> Result<(StatusCode, Json<TimeEntryWithDetails>), StatusCode> {
    let entry_id = Uuid::new_v4();
    // TODO: Get current user from auth context
    let current_user_id = Uuid::new_v4();
    
    let duration = payload.end_time.signed_duration_since(payload.start_time);
    let duration_minutes = duration.num_minutes() as i32;
    
    match sqlx::query!(
        "INSERT INTO time_entries (
            id, user_id, ticket_id, project_id, task_id,
            start_time, end_time, duration_minutes, description, billable
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        entry_id,
        current_user_id,
        payload.ticket_id,
        payload.project_id,
        payload.task_id,
        payload.start_time,
        payload.end_time,
        duration_minutes,
        payload.description,
        payload.billable
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => {
            // Calculate billing
            let _ = calculate_and_update_billing(&state, entry_id).await;
            
            match get_time_entry_by_id(&state, entry_id).await {
                Ok(entry) => Ok((StatusCode::CREATED, Json(entry))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(e) => {
            tracing::error!("Error creating manual time entry: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_time_entry(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<TimeEntryWithDetails>, StatusCode> {
    match get_time_entry_by_id(&state, id).await {
        Ok(entry) => Ok(Json(entry)),
        Err(StatusCode::NOT_FOUND) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_time_entry(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<TimeEntryUpdate>,
) -> Result<Json<TimeEntryWithDetails>, StatusCode> {
    // Calculate duration if start and end times are provided
    let duration = if let (Some(start), Some(end)) = (&payload.start_time, &payload.end_time) {
        Some(end.signed_duration_since(*start).num_minutes() as i32)
    } else {
        payload.duration_minutes
    };
    
    match sqlx::query!(
        "UPDATE time_entries SET 
         ticket_id = COALESCE($2, ticket_id),
         project_id = COALESCE($3, project_id),
         task_id = COALESCE($4, task_id),
         description = COALESCE($5, description),
         billable = COALESCE($6, billable),
         start_time = COALESCE($7, start_time),
         end_time = COALESCE($8, end_time),
         duration_minutes = COALESCE($9, duration_minutes),
         updated_at = NOW()
         WHERE id = $1",
        id,
        payload.ticket_id,
        payload.project_id,
        payload.task_id,
        payload.description,
        payload.billable,
        payload.start_time,
        payload.end_time,
        duration
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                // Recalculate billing
                let _ = calculate_and_update_billing(&state, id).await;
                
                match get_time_entry_by_id(&state, id).await {
                    Ok(entry) => Ok(Json(entry)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Error updating time entry: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_time_entry(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match sqlx::query!("DELETE FROM time_entries WHERE id = $1", id)
        .execute(&state.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Error deleting time entry: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_time_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TimeStats>, StatusCode> {
    // TODO: Get current user from auth context
    let current_user_id = Uuid::new_v4();
    
    let stats = match sqlx::query!(
        "SELECT 
            COALESCE(SUM(duration_minutes) FILTER (WHERE start_time::date = CURRENT_DATE), 0) / 60.0 as hours_today,
            COALESCE(SUM(duration_minutes) FILTER (WHERE start_time::date = CURRENT_DATE AND billable = true), 0) / 60.0 as billable_hours_today,
            COALESCE(SUM(duration_minutes) FILTER (WHERE start_time >= date_trunc('week', CURRENT_DATE)), 0) / 60.0 as hours_week,
            COALESCE(SUM(duration_minutes) FILTER (WHERE start_time >= date_trunc('week', CURRENT_DATE) AND billable = true), 0) / 60.0 as billable_hours_week,
            COALESCE(SUM(total_amount) FILTER (WHERE billable = true AND billed = false), 0) as unbilled_amount,
            COUNT(*) FILTER (WHERE end_time IS NULL) as active_timers
         FROM time_entries 
         WHERE user_id = $1",
        current_user_id
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(row) => TimeStats {
            total_hours_today: Decimal::from_f64_retain(row.hours_today.unwrap_or(0.0)).unwrap_or_default(),
            billable_hours_today: Decimal::from_f64_retain(row.billable_hours_today.unwrap_or(0.0)).unwrap_or_default(),
            total_hours_week: Decimal::from_f64_retain(row.hours_week.unwrap_or(0.0)).unwrap_or_default(),
            billable_hours_week: Decimal::from_f64_retain(row.billable_hours_week.unwrap_or(0.0)).unwrap_or_default(),
            unbilled_amount: row.unbilled_amount.unwrap_or_default(),
            active_timers: row.active_timers.unwrap_or(0) as i32,
        },
        Err(e) => {
            tracing::error!("Error fetching time stats: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(stats))
}

async fn get_timesheet(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TimeEntryQuery>,
) -> Result<Json<Vec<TimeEntryWithDetails>>, StatusCode> {
    // TODO: Implement timesheet view with date grouping
    list_time_entries(State(state), Query(params)).await
}

// Helper functions

async fn get_time_entry_by_id(state: &AppState, id: Uuid) -> Result<TimeEntryWithDetails, StatusCode> {
    match sqlx::query_as!(
        TimeEntryWithDetails,
        "SELECT 
            te.id, te.user_id, u.first_name || ' ' || u.last_name as user_name,
            te.ticket_id, t.number as ticket_number, t.subject as ticket_subject,
            te.project_id, p.name as project_name,
            te.task_id, tk.name as task_name,
            COALESCE(t.client_id, p.client_id) as client_id,
            c.name as client_name,
            te.start_time, te.end_time, te.duration_minutes,
            te.description, te.billable, te.billed,
            te.hourly_rate, te.total_amount,
            te.created_at, te.updated_at
         FROM time_entries te
         LEFT JOIN users u ON te.user_id = u.id
         LEFT JOIN tickets t ON te.ticket_id = t.id
         LEFT JOIN projects p ON te.project_id = p.id
         LEFT JOIN tasks tk ON te.task_id = tk.id
         LEFT JOIN clients c ON COALESCE(t.client_id, p.client_id) = c.id
         WHERE te.id = $1",
        id
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(entry) => Ok(entry),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Error fetching time entry: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_active_timer_by_id(state: &AppState, id: Uuid) -> Result<ActiveTimer, StatusCode> {
    match sqlx::query!(
        "SELECT 
            te.id, te.user_id, te.ticket_id, t.subject as ticket_subject,
            te.project_id, p.name as project_name,
            c.name as client_name, te.description, te.start_time, te.billable,
            EXTRACT(EPOCH FROM (NOW() - te.start_time)) / 60 as elapsed_minutes
         FROM time_entries te
         LEFT JOIN tickets t ON te.ticket_id = t.id
         LEFT JOIN projects p ON te.project_id = p.id
         LEFT JOIN clients c ON COALESCE(t.client_id, p.client_id) = c.id
         WHERE te.id = $1",
        id
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(row) => Ok(ActiveTimer {
            id: row.id,
            user_id: row.user_id,
            ticket_id: row.ticket_id,
            ticket_subject: Some(row.ticket_subject),
            project_id: row.project_id,
            project_name: Some(row.project_name),
            client_name: Some(row.client_name),
            description: row.description,
            start_time: row.start_time,
            elapsed_minutes: row.elapsed_minutes.map(|d| d.to_i32().unwrap_or(0)).unwrap_or(0),
            billable: row.billable.unwrap_or(false),
        }),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Error fetching active timer: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn calculate_and_update_billing(state: &AppState, entry_id: Uuid) -> Result<(), sqlx::Error> {
    // TODO: Get user's hourly rate or project/client rate
    let default_rate = Decimal::from(75); // $75/hour default
    
    sqlx::query!(
        "UPDATE time_entries SET 
         hourly_rate = COALESCE(hourly_rate, $2),
         total_amount = CASE WHEN billable THEN 
                           COALESCE(hourly_rate, $2) * (duration_minutes::decimal / 60)
                        ELSE 0 END
         WHERE id = $1",
        entry_id,
        default_rate
    )
    .execute(&state.db_pool)
    .await?;
    
    Ok(())
}