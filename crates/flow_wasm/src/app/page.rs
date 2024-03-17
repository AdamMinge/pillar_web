use super::{AppNavigation, AppToolbar};

use patternfly_yew::prelude::*;
use yew::prelude::*;

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
            <AppNavigation/>
        </PageSidebar>
    };
    let tools = html! {
        <AppToolbar />
    };

    html! (
        <Page {brand} {sidebar} {tools}>
            { for props.children.iter() }
        </Page>
    )
}
