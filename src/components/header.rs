use yew::prelude::*;

#[derive(Debug)]
pub struct HeaderButton {
    is_selected: bool,
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct HeaderButtonProps {
    pub is_selected: bool,
    pub text: String,
    pub toggle_handler: Callback<MouseEvent>,
}

pub enum HeaderButtonMsg {
    Click,
}

impl Component for HeaderButton {
    type Message = HeaderButtonMsg;
    type Properties = HeaderButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_selected: ctx.props().is_selected,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderButtonMsg::Click => {
                self.is_selected = !self.is_selected;
            }
        };
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let click_callback = &ctx.props().toggle_handler;
        if self.is_selected {
            html! {
                <a href="#">
                    {format!("{} {}", ctx.props().text, self.is_selected)}
                </a>
            }
        } else {
            html! {
                <a href="#" onclick={click_callback}>
                    {format!("{} {}", ctx.props().text, self.is_selected)}
                </a>
            }
        }
    }
}
