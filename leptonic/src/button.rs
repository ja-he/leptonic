use std::fmt::{Display, Formatter};

use leptos::{ev::MouseEvent, *};
use leptos_icons::BsIcon;
use leptos_router::{State, ToHref, A};
use leptos_use::on_click_outside;

use crate::{
    hooks::button::*,
    icon::Icon,
    prelude::{AriaExpanded, AriaHasPopup, Consumer},
    OptionalMaybeSignal,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
    #[default]
    Filled,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Flat => "flat",
            ButtonVariant::Outlined => "outlined",
            ButtonVariant::Filled => "filled",
        }
    }
}

impl Display for ButtonVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warn,
    Danger,
}

impl ButtonColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonColor::Primary => "primary",
            ButtonColor::Secondary => "secondary",
            ButtonColor::Success => "success",
            ButtonColor::Info => "info",
            ButtonColor::Warn => "warn",
            ButtonColor::Danger => "danger",
        }
    }
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Normal => "normal",
            ButtonSize::Big => "big",
        }
    }
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[component]
pub fn Button(
    #[prop(into)] on_click: Consumer<MouseEvent>,
    #[prop(into, optional)] variant: OptionalMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptionalMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptionalMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] variations: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionalMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_haspopup: OptionalMaybeSignal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: OptionalMaybeSignal<AriaExpanded>,
    children: Children,
) -> impl IntoView {
    let btn = use_button(InitialButtonProps {
        disabled: disabled.or(false),
        aria_haspopup: aria_haspopup.or_default(),
        aria_expanded: aria_expanded.or_default(),
    });

    let has_variations = variations.0.as_ref().is_some();

    let variations = move || {
        if has_variations {
            let (dropdown_open, set_dropdown_open): (ReadSignal<bool>, WriteSignal<bool>) =
                create_signal(false);
            let dropdown_trigger = create_node_ref::<html::Div>();
            let _ = on_click_outside(dropdown_trigger, move |_| {
                set_dropdown_open.set(false);
            });
            Some(
                view! {
                    <div class="dropdown-trigger" node_ref=dropdown_trigger on:click=move |e| {
                        if !disabled.get_untracked() {
                            set_dropdown_open.update(|it| *it = !*it);
                            e.stop_propagation();
                        }
                    }>
                        { move || {
                            let icon = match dropdown_open.get() {
                                true => BsIcon::BsCaretUp,
                                false => BsIcon::BsCaretDown,
                            };
                            view! {
                                <Icon icon=icon/>
                            }
                        }}
                    </div>

                    <div class="dropdown" class:active=move || dropdown_open.get() && !disabled.get()>
                        { variations.get() }
                    </div>
                }.into_view(),
            )
        } else {
            None
        }
    };

    view! {
        <button
            {..btn.props}
            id=id
            class=move || class.0.as_ref().map(|it| format!("{} leptonic-btn", it.get())).unwrap_or("leptonic-btn".to_string())
            class:has-variations=has_variations
            class:active=move || active.get()
            style=style
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().as_str()
            data-size=move || size.get().as_str()
            on:click=move |e| {
                if !disabled.get_untracked() {
                    e.stop_propagation();
                    on_click.consume(e);
                }
            }
        >
            <div class="name">
                { children() }
            </div>

            { variations }
        </button>
    }
}

#[component]
pub fn ButtonGroup(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-group>
            { children() }
        </leptonic-btn-group>
    }
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-wrapper>
            { children() }
        </leptonic-btn-wrapper>
    }
}

#[component]
pub fn LinkButton<H>(
    href: H,
    #[prop(into, optional)] variant: OptionalMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptionalMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptionalMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionalMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[allow(unused)] // TODO: Remove this when leptos's A component supports the title attribute.
    #[prop(into, optional)]
    title: Option<AttributeValue>, // TODO: This should be limited to string attributes...
    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,
    /// An object of any type that will be pushed to router state
    #[prop(optional)]
    state: Option<State>,
    /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    /// will skip this page.)
    #[prop(optional)]
    replace: bool,
    children: Children,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    view! {
        <leptonic-link
            id=id
            class=move || {
                let user = class.get();
                let active = active.get().then(|| "active").unwrap_or_default();
                format!("leptonic-btn {user} {active}")
            }
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().as_str()
            data-size=move || size.get().as_str()
            aria-disabled=move || disabled.get()
            style=style
        >
            <A href=href exact=exact state=state.unwrap_or_default() replace=replace>
                <div class="name">
                    { children() }
                </div>
            </A>
        </leptonic-link>
    }
}
