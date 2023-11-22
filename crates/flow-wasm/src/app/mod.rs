pub mod about;
pub mod home;

use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};
use yew_nested_router::Target;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Home,
}

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Home}>
                    <RouterSwitch<AppRoute> render={switch_app_route} />
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Home => html! {<AppPage><home::Home/></AppPage>},
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(AppPage)]
fn page(props: &PageProps) -> Html {
    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavList>
                </NavList>
            </Nav>
        </PageSidebar>
    };

    let brand = html! (
        <MastheadBrand>
            <Brand src="https://www.patternfly.org/assets/images/PF-Masthead-Logo.svg" alt="Patternfly Logo" />
        </MastheadBrand>
    );

    let backdropper = use_backdrop();
    let onabout = use_callback((), move |_, ()| {
        if let Some(backdropper) = &backdropper {
            backdropper.open(html!(<about::About/>));
        }
    });

    let onthemeswitch = use_callback((), |state, ()| match state {
        true => gloo_utils::document_element().set_class_name("pf-v5-theme-dark"),
        false => gloo_utils::document_element().set_class_name(""),
    });

    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <patternfly_yew::prelude::Switch onchange={onthemeswitch} label="Dark Theme" />
                    </ToolbarItem>
                    <ToolbarItem>
                        <Dropdown
                            position={Position::Right}
                            icon={Icon::QuestionCircle}
                            variant={MenuToggleVariant::Plain}
                        >
                            <MenuAction onclick={onabout}>{"About"}</MenuAction>
                        </Dropdown>
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );

    html! (
        <Page {brand} {sidebar} {tools}>
            { for props.children.iter() }
        </Page>
    )
}
