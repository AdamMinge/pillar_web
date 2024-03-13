use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::components;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AppPageProps {
    pub children: Children,
}

#[function_component(AppPage)]
pub fn app_page(props: &AppPageProps) -> Html {
    let brand = html! {
        <MastheadBrand>
            <Brand src="/public/images/brand.png" alt="Flow brand" />
        </MastheadBrand>
    };
    let sidebar = html_nested! {
        <PageSidebar>
            <components::Navigation/>
        </PageSidebar>
    };
    let tools = html! {
        <components::ToolsToolbar />
    };

    html! (
        <Page {brand} {sidebar} {tools}>
            { for props.children.iter() }
        </Page>
    )
}
