use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html!(
        <Bullseye>
            <AboutModal
            background_image_src="/public/images/background.png"
            brand_image_src="/public/images/logo.png"
            brand_image_alt="Flow logo"
            product_name="Flow"
            trademark="Copyright © 2020, 2023 Flow"
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
}
