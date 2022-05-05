use super::components::header::{HeaderButton, HeaderButtonProps};
use super::components::toggle_page::{TogglePage, TogglePageProps};
use yew::{callback, prelude::*};

#[derive(Debug, PartialEq)]
struct PageController {
    header_button_props: Vec<HeaderButtonProps>,
    toggle_page_props: Vec<TogglePageProps>,
    static_header_elements: Vec<Html>,
}
#[derive(Properties, PartialEq, Debug)]
struct PageControllerProps {
    header_buttons: Vec<HeaderButtonProps>,
    toggle_pages: Vec<TogglePageProps>,
    static_header_text: Vec<String>,
    static_header_link: Vec<String>,
}
enum PageControllerMsg {
    ToggleVisibility(usize),
}
impl Component for PageController {
    type Message = PageControllerMsg;
    type Properties = PageControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        // check logic
        let num_selected_buttons = ctx
            .props()
            .header_buttons
            .iter()
            .filter(|header_button| header_button.is_selected == true)
            .count();
        let num_visible_pages = ctx
            .props()
            .toggle_pages
            .iter()
            .filter(|toggle_page| toggle_page.is_visible == true)
            .count();
        match num_selected_buttons {
            1 => (),
            0 => panic!("No header button is selected!"),
            _ => panic!("More than one header button is selected!"),
        }
        match num_visible_pages {
            1 => (),
            0 => panic!("No toggle page is visible!"),
            _ => panic!("More than one toggle page is visible!"),
        }
        let mut static_header_elements = Vec::new();
        for i in 0..ctx.props().static_header_text.len() {
            static_header_elements.push(html! {
                <a href={ctx.props().static_header_link[i].clone()}>
                    {ctx.props().static_header_text[i].clone()}
                </a>
            });
        }
        let mut header_button_props = Vec::new();
        for i in 0..ctx.props().header_buttons.len() {
            header_button_props.push(HeaderButtonProps {
                is_selected: ctx.props().header_buttons[i].is_selected,
                text: ctx.props().header_buttons[i].text.clone(),
                toggle_handler: ctx
                    .link()
                    .callback(move |_| PageControllerMsg::ToggleVisibility(i)),
            });
        }
        Self {
            header_button_props,
            toggle_page_props: ctx.props().toggle_pages.clone(),
            static_header_elements,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PageControllerMsg::ToggleVisibility(i) => {
                for j in 0..self.toggle_page_props.len() {
                    self.header_button_props[j].is_selected = false;
                    self.toggle_page_props[j].is_visible = false;
                }
                self.header_button_props[i].is_selected = true;
                self.toggle_page_props[i].is_visible = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_page = self
            .toggle_page_props
            .iter()
            .filter(|tp| tp.is_visible == true)
            .collect::<Vec<_>>();
        match current_page.len() {
            1 => (),
            0 => panic!("No toggle page is visible!"),
            _ => panic!("More than one toggle page is visible!"),
        }

        html! {
            <>
                <nav class="menu">
                    <div class="inner">
                        {
                            self.header_button_props
                                .iter()
                                .enumerate()
                                .map(|(i, hbp)| {
                                    html! {
                                        <HeaderButton
                                            is_selected={hbp.is_selected}
                                            text={hbp.text.clone()}
                                            toggle_handler={|_| PageControllerMsg::ToggleVisibility(i)}
                                        />
                                    }
                                })
                                .collect::<Html>()
                        }
                        {self.static_header_elements.clone()}
                    </div>
                </nav>
                <TogglePage
                    is_visible={current_page[0].is_visible}
                    inner_html={current_page[0].inner_html.clone()}
                />
            </>
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <PageController
            static_header_text={vec!["GitHub".to_string()]}
            static_header_link={vec!["https://github.com/hmdyt".to_string()]}
            header_buttons={vec![
                HeaderButtonProps {
                    is_selected: true,
                    text: "AboutMe".to_string(),
                    toggle_handler: Callback::noop(),
                },
                HeaderButtonProps {
                    is_selected: false,
                    text: "Archives".to_string(),
                    toggle_handler: Callback::noop(),
                },
            ]}
            toggle_pages={vec![
                TogglePageProps {
                    is_visible: true,
                    inner_html: html!{<>{"aboutme text"}</>},
                },
                TogglePageProps {
                    is_visible: false,
                    inner_html: html!{<>{"archives text"}</>},
                },
            ]}
        />
    }
}
