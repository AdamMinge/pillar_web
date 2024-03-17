use super::{AppAbout, AppAccount};
use crate::components::ThemeSwitch;

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AppToolbar)]
pub fn app_toolbar() -> Html {
    html! {
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <ThemeSwitch/>
                    </ToolbarItem>
                    <ToolbarItem>
                        <AppAbout/>
                    </ToolbarItem>
                    <ToolbarItem>
                        <AppAccount/>
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    }
}
