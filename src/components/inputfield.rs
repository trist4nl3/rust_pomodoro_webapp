use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub label: String,
    pub input_type: String,
    pub name: String,
    pub node_ref: NodeRef,
    pub placeholder: String,
}

#[function_component(FieldInput)]
pub fn field_input(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        label,
        input_type,
        name,
        node_ref,
        placeholder,
    } = props;

    html! {
    
        <label for = "dangerous-input"> // Taken from the yew documentation
            { label }
            < input
                type = { input_type.clone() }
                name = { name.clone() }
                placeholder = { placeholder.clone() }
                ref = { node_ref.clone() }
            />
        </label>

        
    }
}