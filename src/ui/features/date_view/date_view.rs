use chrono::NaiveDate;

use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

use super::date_input::DateInput;
use crate::ui::components::card::Card;
use crate::ui::data::DateConfig;

#[derive(Properties, PartialEq)]
pub struct DateViewProps {
    pub dates: Vec<DateConfig>,

    pub onchange: Callback<Vec<DateConfig>>,
}

#[function_component]
pub fn DateView(props: &DateViewProps) -> Html {
    let dates = use_state_eq(|| props.dates.clone());
    let handle_start_change = {
        let dates = dates.clone();
        Callback::from(move |date: NaiveDate| {
            let mut new_dates = (*dates).clone();
            new_dates[0] = DateConfig { date };
            dates.set(new_dates)
        })
    };
    let handle_end_change = {
        let dates = dates.clone();
        Callback::from(move |date: NaiveDate| {
            let mut new_dates = (*dates).clone();
            new_dates[dates.len() - 1] = DateConfig { date };
            dates.set(new_dates)
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
                        value={dates.first().unwrap().date}
                        onchange={handle_start_change}
                    />
                </div>
                <div class="col-auto">
                    {"End date"}
                </div>
                <div class="col-auto">
                    <DateInput
                        value={dates.last().unwrap().date}
                        onchange={handle_end_change}
                    />
                </div>
            </div>
        </Card>
    }
}
