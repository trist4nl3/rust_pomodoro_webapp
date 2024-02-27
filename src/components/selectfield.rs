use yew::prelude::*;

pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub is_selected: bool,
}

impl SelectOption {
    pub fn new(value: &str, label: &str, is_selected: bool) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            is_selected,
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub id: String,
    pub label: String,
    pub options: Vec<SelectOption>,
    pub on_change: Callback<String>,
}

pub fn select(props: &Props) -> Html {
    let stylesheet = Style::new(css!(
        r#"
          label {
            font-size: 24px;
          }

          select {
            font-size: 24px;
            width: 100%;
          }
    "#
    ))
    .unwrap();
    let onchange = {
        let callback = props.on_change.clone();
        Callback::from(move |event: ChangeData| {
            if let ChangeData::Select(elem) = event {
                let value = elem.value();
                callback.emit(value);
            }
        })
    };
    html! {
        <div class={stylesheet}>
            <label for={props.id.clone()}>{&props.label}</label>
            <select
                id={props.id.clone()}
                data-test={props.data_test.clone()}
                onchange={onchange}
                {create_option_tag(props.options.clone())}
            </select>
        </div>
    }
}

fn create_option_tag(select_options: Vec<SelectOption>) -> Vec<Html> {
    select_options
        .iter()
        .map(|select_option| {
            html! {
              <option
              value={select_option.value.clone()}
              selected={select_option.is_selected}>
                {&select_option.label}
              </option>
            }
        })
        .collect()
}
