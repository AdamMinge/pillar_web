use crate::routes::UserRoute;
use crate::validators::{make_email_validator, make_password_validator};

use patternfly_yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_nested_router::components::*;

#[function_component(LoginPageFooter)]
fn login_page_footer() -> Html {
    let links = ChildrenRenderer::new(vec![]);

    let band = ChildrenRenderer::new(vec![
        html! {<>{"Need an account? "}<Link<UserRoute> active="active" target={UserRoute::Signup}>{ "Sign up" }</Link<UserRoute>></>},
        html! {<Link<UserRoute> active="active" target={UserRoute::PasswordRecovery}>{ "Forgot password?" }</Link<UserRoute>>},
    ]);

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

#[function_component(LoginPageForm)]
fn login_page_form() -> Html {
    let email = use_state_eq(String::new);
    let onchangeemail = {
        let email = email.clone();
        Callback::from(move |value| {
            email.set(value);
        })
    };

    let password = use_state_eq(String::new);
    let onchangepassword = {
        let password = password.clone();
        Callback::from(move |value| {
            password.set(value);
        })
    };

    let user_login = {
        let client = flow_api::hooks::use_client();
        let email = email.clone();
        let password = password.clone();
        use_async(async move { client.unwrap().login(&email, &password).await })
    };

    let onsubmit = {
        let user_login = user_login.clone();

        Callback::from(move |_| {
            user_login.run();
        })
    };

    let valid_email = use_state(move || false);
    let on_email_validated = {
        let valid_email = valid_email.clone();
        Callback::from(move |result: ValidationResult| {
            valid_email.set(result.state == InputState::Success);
        })
    };

    let valid_password = use_state(move || false);
    let on_password_validated = {
        let valid_password = valid_password.clone();
        Callback::from(move |result: ValidationResult| {
            valid_password.set(result.state == InputState::Success);
        })
    };

    let login_enabled = {
        let valid_email = valid_email.clone();
        let valid_password = valid_password.clone();
        move || *valid_email && *valid_password
    };

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

                <FormGroupValidated<TextInput>
                    label="Password"
                    required=true
                    validator={make_password_validator()}
                    onvalidated={on_password_validated}
                >
                    <TextInput required=true r#type={TextInputType::Password} onchange={onchangepassword} value={(*password).clone()} />
                </FormGroupValidated<TextInput>>

                <ActionGroup>
                    <Button label="Log In" block=true r#type={ButtonType::Submit} variant={ButtonVariant::Primary} disabled={!login_enabled()}/>
                </ActionGroup>
            </Form>
        </>
    }
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let title = html_nested! {<Title size={Size::XXLarge}>{"Login to your account"}</Title>};

    html! {
        <>
            <ToastViewer>
                <Background style="/public/images/background.png"/>
                <Login>
                    <LoginMain>
                        <LoginMainHeader
                            {title}
                            description="Enter the credentials to your account right here."
                        />
                        <LoginMainBody>
                            <LoginPageForm/>
                        </LoginMainBody>
                        <LoginPageFooter/>
                    </LoginMain>
                </Login>
            </ToastViewer>
        </>
    }
}
