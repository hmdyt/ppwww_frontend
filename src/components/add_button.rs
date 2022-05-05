use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct AddButton {
    value: usize,
}

impl Component for AddButton {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <button onclick={link.callback(|_| Msg::AddOne)}> {"+1"} </button>
                <span>{self.value}</span>
            </>
        }
    }
}
