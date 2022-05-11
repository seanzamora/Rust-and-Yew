use yew::{html, Component, Context, Html,  Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct DashboardView;

impl Component for DashboardView {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }     

  
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { 
            <div class="flex">
            <p class="text-2xl">{"Test"}</p>
            </div>

        }
    }
}