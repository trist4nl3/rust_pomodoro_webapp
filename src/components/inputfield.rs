use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub label: String,
    pub input_type: String,
    pub name: String,
    pub node_ref: NodeRef,
}

#[function_component(FieldInput)]
pub fn field_input(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        label,
        input_type,
        name,
        node_ref,
    } = props;

    html! {
        <div>
            <label for = "dangerous-input"> // Taken from the yew documentation
                { label }
                < input
                    type = { input_type }
                    name = { name }
                    ref = { node_ref.clone() }
            </label>
    }
}