use crate::routes::AppRoute;
use crate::validators::make_email_validator;

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
    let email = use_state_eq(String::new);
    let onchangeemail = {
        let email = email.clone();
        Callback::from(move |value| {
            email.set(value);
        })
    };

    let valid_email = use_state(move || false);
    let on_email_validated = {
        let valid_email = valid_email.clone();
        Callback::from(move |result: ValidationResult| {
            valid_email.set(result.state == InputState::Success);
        })
    };

    let recovery_enabled = {
        let valid_email = valid_email.clone();
        move || *valid_email
    };

    let password_recovery = {
        let client = flow_api::hooks::use_client();
        let email = email.clone();

        use_async(async move {
            flow_api::services::password_recovery(
                &mut client.unwrap(),
                flow_api::types::Email {
                    email: (*email).clone(),
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
                    label="Email"
                    required=true
                    validator={make_email_validator()}
                    onvalidated={on_email_validated}
                >
                    <TextInput required=true onchange={onchangeemail} value={(*email).clone()} />
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
                            description="If an account is created for this email address, we will send an recovery email to it."
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
