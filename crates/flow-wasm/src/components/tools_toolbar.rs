use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::components;

#[function_component(ToolsToolbar)]
pub fn tools_toolbar() -> Html {
    html! {
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <components::ThemeSwitch/>
                    </ToolbarItem>
                    <ToolbarItem>
                        <components::AboutDropdown/>
                    </ToolbarItem>
                    <ToolbarItem>
                        <components::UserDropdown/>
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    }
}
