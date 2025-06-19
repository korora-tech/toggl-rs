use super::ids::{ProjectId, TagId, TaskId, TimeEntryId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags(pub HashMap<String, serde_json::Value>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaFeature {
    pub code: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_timeline_display_activity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_timeline_grouping_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_timeline_grouping_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_timeline_recording_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_timeline_sync_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_features: Option<Vec<AlphaFeature>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_advanced_filters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tracker_delay_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tracker_delay_in_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tagging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autotracking_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beginning_of_week: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_snap_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_snap_initial_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_visible_hours_end: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_visible_hours_start: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_zoom_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_swipe_actions_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charts_view_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_detailed_report_entries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_time_entries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards_view_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_separator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_project_id: Option<ProjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_id: Option<TaskId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_density: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_rates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_format_on_timer_duration_field: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_popup_integration_timer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_send_error_reports: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_send_usage_statistics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_business_promo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_app_on_time_entry_started: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_app_on_time_entry_stopped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haptic_feedback_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_keyboard_shortcut: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_sidebar_right: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_detection_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_detection_interval_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactivity_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_is_goals_view_shown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_goals_view_expanded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_goals_view_shown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_summary_total_view_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_mini_timer_on_top: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_window_on_top: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_increment_timer_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcuts_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcuts_share_time_entries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_is_goals_view_shown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_entry_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_mode_overlay_seen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_on_start_time_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_time_zone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_auto_start_break: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_auto_start_focus: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_break_interval_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_break_project_id: Option<ProjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_break_start_sound_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_break_tag_id: Option<TagId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_countdown_timer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_focus_interval_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_focus_sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_global_sound_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_interval_end_sound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_interval_end_volume: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_longer_break_duration_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_prevent_screen_lock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_rounds_before_longer_break: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_session_start_sound_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_show_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_stop_timer_at_interval_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_track_breaks_as_time_entries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_dashboard_activity_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_shortcut_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_timeline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_last_project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_days: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_interval_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_snoozing_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_rounding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_rounding_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_rounding_step_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports_hide_weekends: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_app_on_startup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_entry_warning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_timer_notification_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_follow_modal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_footer_popup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_project_dashboard_overlay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_toggl_button_modal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_daily_project_invites: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_product_emails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_timer_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_weekly_report: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_shortcut_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_all_entries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_changelog: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_events_in_calendar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_timeline_in_day_view: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_total_billable_hours: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_weekend_on_timer_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_automatically: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_shortcut_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_automatically: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_entry_on_shutdown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_report_amounts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_report_distinct_rates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_report_grouping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_report_sort_asc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_report_sort_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_report_sub_grouping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_total_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_shortcut_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_entry_display_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_entry_ghost_suggestions_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_entry_invitations_notification_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_entry_start_stop_input_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeofday_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_view: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_view_mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_accept_needed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_mini_timer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_time_entry_started: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_time_entry_stopped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_report_grouping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_report_value_to_show: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workout_default_project_id: Option<ProjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workout_default_tag_id: Option<TagId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_features: Option<Vec<AlphaFeature>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushService {
    pub id: i64,
    pub token: String,
    pub platform: String,
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    pub total: i64,
    pub used: i64,
    pub available: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryConstraints {
    pub description_required: bool,
    pub project_required: bool,
    pub tag_required: bool,
    pub task_required: bool,
    pub time_entry_constraints_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTimeEntry {
    pub time_entry_id: TimeEntryId,
    pub user_id: UserId,
    pub user_name: String,
    pub user_email: String,
    pub shared_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
    pub logo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_approvals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_expenses: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_timesheet_view: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_start_end_times: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inc_tos_accepted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inc_tos_accepted_by: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_pricing_plan: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_locked_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_sign_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_requested_at: Option<String>,
}
