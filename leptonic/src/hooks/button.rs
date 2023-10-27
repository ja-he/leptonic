use crate::{aria::*, MaybeSignalExt};
use leptos::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct InitialButtonProps {
    pub disabled: MaybeSignal<bool>,
    pub aria_haspopup: MaybeSignal<AriaHasPopup>,
    pub aria_expanded: MaybeSignal<AriaExpanded>,
}

#[derive(Debug, Clone)]
pub struct UseButtonReturn {
    /// Spread these props onto your button using the spreac syntax: `<button {..props}>...`
    pub props: HashMap<&'static str, Attribute>,
}

pub fn use_button(initial_props: InitialButtonProps) -> UseButtonReturn {
    let mut props = HashMap::<&'static str, Attribute>::new();
    props.insert("role", Attribute::String(Oco::Borrowed("button")));
    props.insert(
        "tabindex",
        initial_props
            .disabled
            .map(|it| match it {
                true => Attribute::Option(None),
                false => Attribute::String(Oco::Borrowed("0")),
            })
            .into_attribute(),
    );
    props.insert("disabled", initial_props.disabled.into_attribute());
    props.insert(
        "aria-disabled",
        initial_props
            .disabled
            .map(|it| match it {
                true => "true",
                false => "false",
            })
            .into_attribute(),
    );
    props.insert(
        "aria-haspopup",
        initial_props.aria_haspopup.into_attribute(),
    );

    // From https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded
    // A button that opens a widget should have aria-controls set to the id of the expandable widget and aria-expanded set to the current state of the widget.

    props.insert(
        "aria-expanded",
        initial_props.aria_expanded.into_attribute(),
    );
    //props.insert(
    //    "aria-controls",
    //    initial_props.aria_controls.into_attribute(),
    //);
    //props.insert(
    //    "aria-pressed",
    //    initial_props.aria_pressed.into_attribute(),
    //);
    UseButtonReturn { props }
}
