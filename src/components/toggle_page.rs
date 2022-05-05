use yew::prelude::*;

pub enum TogglePageMsg {
    ToggleVisibility,
}

#[derive(Debug, PartialEq)]
pub struct TogglePage {
    pub inner_html: Html,
    pub is_visible: bool,
}
#[derive(Properties, PartialEq, Clone, Debug)]
pub struct TogglePageProps {
    pub inner_html: Html,
    pub is_visible: bool,
}

impl Component for TogglePage {
    type Message = TogglePageMsg;
    type Properties = TogglePageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            inner_html: ctx.props().inner_html.clone(),
            is_visible: ctx.props().is_visible,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TogglePageMsg::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.is_visible {
            self.inner_html.clone()
        } else {
            html! {}
        }
    }
}
