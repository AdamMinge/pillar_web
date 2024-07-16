use crate::routes::{AppRoute, AuthRoute};
use crate::validators::make_password_validator;

use patternfly_yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[function_component(PasswordRecoveryPageFooter)]
fn password_recovery_page_footer() -> Html {
    let links = ChildrenRenderer::new(vec![]);
    let band = ChildrenRenderer::new(vec![]);

    html! {
        <>
            <LoginMainFooter
                {links}
                {band}
            >
            </LoginMainFooter>
        </>
    }
}

#[function_component(PasswordRecoveryPageForm)]
fn password_recovery_page_form() -> Html {
    let password = use_state_eq(String::new);
    let onchangepassword = {
        let password = password.clone();
        Callback::from(move |value| {
            password.set(value);
        })
    };

    let valid_password = use_state(move || false);
    let on_password_validated = {
        let valid_password = valid_password.clone();
        Callback::from(move |result: ValidationResult| {
            valid_password.set(result.state == InputState::Success);
        })
    };

    let recovery_enabled = {
        let valid_password = valid_password.clone();
        move || *valid_password
    };

    let password_recovery = {
        let client = pillar_api::hooks::use_client();
        let password = password.clone();
        let token = use_route::<AuthRoute>().and_then(|route| match route {
            AuthRoute::PasswordRecovery { token } => Some(token),
            _ => None,
        });

        use_async(async move {
            pillar_api::services::recovery(
                &mut client.unwrap(),
                pillar_api::types::Password {
                    password: ((*password).clone()),
                },
                pillar_api::types::Token {
                    token: token.unwrap(),
                },
            )
            .await
        })
    };

    let onsubmit = {
        let password_recovery = password_recovery.clone();
        Callback::from(move |_| {
            if !password_recovery.loading {
                password_recovery.run();
            }
        })
    };

    let navigator = use_navigator().unwrap();
    use_effect_with(password_recovery.clone(), move |password_recovery| {
        if let Some(_) = &password_recovery.data {
            navigator.push(&AppRoute::Home);
        } else if let Some(_) = &password_recovery.error {
        }
        || ()
    });

    html! {
        <>
            <Form {onsubmit} method="dialog">
                <FormGroupValidated<TextInput>
                    label="Password"
                    required=true
                    validator={make_password_validator()}
                    onvalidated={on_password_validated}
                >
                    <TextInput required=true r#type={TextInputType::Password} onchange={onchangepassword} value={(*password).clone()} />
                </FormGroupValidated<TextInput>>
                <ActionGroup>
                    <Button
                        label="Recovery"
                        block=true
                        r#type={ButtonType::Submit}
                        variant={ButtonVariant::Primary}
                        disabled={!recovery_enabled()}
                        loading={password_recovery.loading}
                    />
                </ActionGroup>
            </Form>
        </>
    }
}

#[function_component(PasswordRecoveryPage)]
pub fn password_recovery_page() -> Html {
    let title = html_nested! {<Title size={Size::XXLarge}>{"Recovery your password"}</Title>};

    html! {
        <>
            <ToastViewer>
                <Background style="/public/images/background.png"/>
                <Login>
                    <LoginMain>
                        <LoginMainHeader
                            {title}
                            description="Enter a new password for an account."
                        />
                        <LoginMainBody>
                            <PasswordRecoveryPageForm/>
                        </LoginMainBody>
                        <PasswordRecoveryPageFooter/>
                    </LoginMain>
                </Login>
            </ToastViewer>
        </>
    }
}
