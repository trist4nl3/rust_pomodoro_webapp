use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub label: String,
    pub input_type: String,
    pub name: String,
    pub node_ref: NodeRef,
    pub value: String,
}

#[function_component(FieldInput)]
pub fn field_input(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        label,
        input_type,
        name,
        node_ref,
        value,
    } = props;

    html! {
    
        <label for = "dangerous-input"> // Taken from the yew documentation
            { label }
            < input
                type = { input_type.clone() }
                name = { name.clone() }
                value = { value.clone() }
                ref = { node_ref.clone() }
            />
        </label>

        
    }
}