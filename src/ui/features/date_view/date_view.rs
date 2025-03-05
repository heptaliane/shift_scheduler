use core::ops::Add;
use std::collections::HashMap;

use chrono::{Local, NaiveDate, TimeDelta};

use yew::{function_component, html, use_state_eq, Callback, Html, MouseEvent, Properties};

use super::date_input::DateInput;
use crate::ui::components::card::Card;
use crate::ui::data::DateConfig;

#[derive(Properties, PartialEq)]
pub struct DateViewProps {
    pub dates: Vec<DateConfig>,

    pub onchange: Callback<Vec<DateConfig>>,
}

fn update_date_lut(
    lut: &HashMap<NaiveDate, DateConfig>,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> HashMap<NaiveDate, DateConfig> {
    let mut current = start_date.clone();
    let mut lut = lut.clone();
    while current <= *end_date {
        if !lut.contains_key(&current) {
            lut.insert(
                current.clone(),
                DateConfig {
                    date: current.clone(),
                },
            );
            current = current.add(TimeDelta::days(1));
        }
    }
    lut
}

#[function_component]
pub fn DateView(props: &DateViewProps) -> Html {
    let start_date = use_state_eq(|| match props.dates.first() {
        Some(date) => date.date,
        _ => Local::now().date_naive(),
    });
    let end_date = use_state_eq(|| match props.dates.last() {
        Some(date) => date.date,
        _ => Local::now().date_naive(),
    });
    let dates = use_state_eq(|| {
        props
            .dates
            .iter()
            .map(|config| (config.date, config.clone()))
            .collect::<HashMap<NaiveDate, DateConfig>>()
    });
    let handle_start_change = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let dates = dates.clone();
        Callback::from(move |date: NaiveDate| {
            start_date.set(date);
            dates.set(update_date_lut(&dates, &start_date, &end_date));
        })
    };
    let handle_end_change = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let dates = dates.clone();
        Callback::from(move |date: NaiveDate| {
            end_date.set(date);
            dates.set(update_date_lut(&dates, &start_date, &end_date));
        })
    };
    let handle_submit = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let dates = dates.clone();
        Callback::from(move |_: MouseEvent| {
            let mut current = (*start_date).clone();
            let mut configs: Vec<DateConfig> = Vec::new();
            while current <= *end_date {
                configs.push(dates[&current].clone());
                current = current.add(TimeDelta::days(1));
            }
        })
    };

    html! {
        <Card>
            <div class="row g-3 align-items-center">
                <div class="col-auto">
                    {"Start date"}
                </div>
                <div class="col-auto">
                    <DateInput
                        value={(*start_date).clone()}
                        onchange={handle_start_change}
                    />
                </div>
                <div class="col-auto">
                    {"End date"}
                </div>
                <div class="col-auto">
                    <DateInput
                        value={(*end_date).clone()}
                        onchange={handle_end_change}
                    />
                </div>
            </div>
            <div class="d-grid">
                <button
                    type="button"
                    class="btn btn-primary"
                    onclick={handle_submit.clone()}
                >
                    {"Submit"}
                </button>
            </div>
        </Card>
    }
}
