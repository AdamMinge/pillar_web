use crate::routes::{switch_app_route, AppRoute};

use dotenv_codegen::dotenv;
use patternfly_yew::prelude::*;
use url::Url;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

#[function_component(App)]
pub fn app() -> Html {
    let config = flow_api::api::Config {
        api_root: Url::parse(dotenv!("API_ROOT")).unwrap(),
        api_token: Some(dotenv!("API_KEY").to_string()),
    };

    html! {
        <flow_api::components::Api config={config}>
            <BackdropViewer>
                <ToastViewer>
                    <Router<AppRoute> default={AppRoute::Index}>
                        <RouterSwitch<AppRoute> render={switch_app_route} />
                    </Router<AppRoute>>
                </ToastViewer>
            </BackdropViewer>
        </flow_api::components::Api>
    }
}
