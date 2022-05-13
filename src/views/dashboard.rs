use yew::{html, Component, Context, Html,  Properties};

use crate::services::subscriber::Subscriber;
use crate::services::producers::Producer;
use crate::services::event_bus::EventBus;

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
            <>
            <Producer />
            <Subscriber />
            </>
        }
    }
}