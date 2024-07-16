use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AppAbout)]
pub fn app_about() -> Html {
    let backdropper = use_backdrop();
    let onabout = use_callback((), move |_, ()| {
        if let Some(backdropper) = &backdropper {
            backdropper.open(
                html!(
                    <Bullseye>
                        <AboutModal
                            background_image_src="/public/images/background.png"
                            brand_image_src="/public/images/logo.png"
                            brand_image_alt="Pillar logo"
                            product_name="Pillar"
                            trademark="Copyright © 2023 Pillar"
                        >
            
                        <p>{ env!("CARGO_PKG_DESCRIPTION") }</p>
                        <br />
            
                        <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                            <DescriptionGroup term="Name">{env!("CARGO_PKG_NAME")}</DescriptionGroup>
                            <DescriptionGroup term="Version">{env!("CARGO_PKG_VERSION")}</DescriptionGroup>
                            <DescriptionGroup term="License">{env!("CARGO_PKG_LICENSE")}</DescriptionGroup>
                        </DescriptionList>
            
                        </AboutModal>
                    </Bullseye>
                )
            );
        }
    });

    html! {
        <Dropdown
            position={Position::Right}
            icon={Icon::QuestionCircle}
            variant={MenuToggleVariant::Plain}
        >
            <MenuAction onclick={onabout}>{"About"}</MenuAction>
        </Dropdown>
    }
}
