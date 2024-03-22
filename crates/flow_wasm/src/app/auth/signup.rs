use crate::routes::{AppRoute, AuthRoute};
use crate::validators::{make_email_validator, make_password_validator, make_username_validator};

use patternfly_yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[function_component(SignupPageFooter)]
fn signup_page_footer() -> Html {
    let links = ChildrenRenderer::new(vec![]);

    let band = ChildrenRenderer::new(vec![
        html! {<>{"Need an account? "}<Link<AuthRoute> to={AuthRoute::Login}>{ "Log in" }</Link<AuthRoute>></>},
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

#[function_component(SignupPageForm)]
fn signup_page_form() -> Html {
    let username = use_state_eq(String::new);
    let onchangeusername = {
        let username = username.clone();
        Callback::from(move |value| {
            username.set(value);
        })
    };

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

    let valid_email = use_state(move || false);
    let on_email_validated = {
        let valid_email = valid_email.clone();
        Callback::from(move |result: ValidationResult| {
            valid_email.set(result.state == InputState::Success);
        })
    };

    let valid_username = use_state(move || false);
    let on_username_validated = {
        let valid_username = valid_username.clone();
        Callback::from(move |result: ValidationResult| {
            valid_username.set(result.state == InputState::Success);
        })
    };

    let valid_password = use_state(move || false);
    let on_password_validated = {
        let valid_password = valid_password.clone();
        Callback::from(move |result: ValidationResult| {
            valid_password.set(result.state == InputState::Success);
        })
    };

    let signup_enabled = {
        let valid_email = valid_email.clone();
        let valid_username = valid_username.clone();
        let valid_password = valid_password.clone();
        move || *valid_email && *valid_username && *valid_password
    };

    let user_signup = {
        let client = flow_api::hooks::use_client();
        let username = username.clone();
        let email: UseStateHandle<String> = email.clone();
        let password = password.clone();

        use_async(async move {
            flow_api::services::signup(
                &mut client.unwrap(),
                flow_api::types::Signup {
                    username: (*username).clone(),
                    password: (*password).clone(),
                    email: (*email).clone(),
                },
            )
            .await
        })
    };

    let onsubmit = {
        let user_signup = user_signup.clone();

        Callback::from(move |_| {
            if !user_signup.loading {
                user_signup.run();
            }
        })
    };

    let navigator = use_navigator().unwrap();
    use_effect_with(user_signup.clone(), move |user_signup| {
        if let Some(_) = &user_signup.data {
            navigator.push(&AppRoute::Home);
        } else if let Some(_) = &user_signup.error {
        }
        || ()
    });

    html! {
        <>
            <Form {onsubmit} method="dialog">
                <FormGroupValidated<TextInput>
                    label="Username"
                    required=true
                    validator={make_username_validator()}
                    onvalidated={on_username_validated}
                >
                    <TextInput required=true onchange={onchangeusername} value={(*username).clone()} />
                </FormGroupValidated<TextInput>>
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
                    <Button
                        label="Sign up"
                        block=true
                        r#type={ButtonType::Submit}
                        variant={ButtonVariant::Primary}
                        disabled={!signup_enabled()}
                        loading={user_signup.loading}
                    />
                </ActionGroup>
            </Form>
        </>
    }
}

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let title = html_nested! {<Title size={Size::XXLarge}>{"Create your account"}</Title>};

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
                            <SignupPageForm/>
                        </LoginMainBody>
                        <SignupPageFooter/>
                    </LoginMain>
                </Login>
            </ToastViewer>
        </>
    }
}
