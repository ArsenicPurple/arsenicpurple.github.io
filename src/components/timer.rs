use leptos::prelude::*;
use leptos::{component, IntoView};
use std::time::Duration;

#[component]
pub fn Timer() -> impl IntoView {
    let (read_timer_done, set_timer_done) = signal(false);
    let (read_ticking, set_ticking) = signal(None::<IntervalHandle>);
    let (read_time_remaining, set_time_remaining) = signal(30u32);

    let set_time_remaining_copy = set_time_remaining.clone();
    let add_time = move |v| { set_time_remaining_copy.update(|d| { *d = d.saturating_add(v) }) };
    let set_time_remaining_copy = set_time_remaining.clone();
    let sub_time = move |v| { set_time_remaining_copy.update(|d| { *d = d.saturating_sub(v) }) };

    Effect::new(move || {
        if read_timer_done.get() {
            if let Some(interval_handle) = read_ticking.get() {
                interval_handle.clear();
            }
        }
    });

    view! {
        <div class="timer">
            <h3>{move || format_duration(read_time_remaining.get()) }</h3>
            <div class="timer-controls">
                <button on:click=move |_| { sub_time(30) } class="timer-add-time">"-30"</button>
                <button on:click=move |_| { sub_time(10) } class="timer-add-time">"-10"</button>
                <button
                    on:click=move |_| {
                        if let Some(interval_handle) = read_ticking.get() {
                            interval_handle.clear();
                            set_ticking.set(None);
                            set_timer_done.set(false);
                        } else {
                            set_ticking.set(Some(set_interval_with_handle(
                                move || {
                                    set_time_remaining.update(|t| if *t > 0 { *t -= 1 } else { set_timer_done.set(true) })
                                },
                                Duration::new(1, 0)
                            ).unwrap()));
                        }
                    }
                    class="timer-start-stop"
                >
                    { move || {
                        if read_ticking.get().is_some() { "Stop" } else { "Start" }
                    } }
                </button>
                <button on:click=move |_| { add_time(10) } class="timer-add-time">"+10"</button>
                <button on:click=move |_| { add_time(30) } class="timer-add-time">"+30"</button>
            </div>
        </div>
    }
}

fn format_duration(secs: u32) -> String {
    format_time(secs / 60) + ":" + &format_time(secs % 60)
}

fn format_time(value: u32) -> String {
    if value < 10 {
        return "0".to_owned() + &value.to_string()
    }
    value.to_string()
}