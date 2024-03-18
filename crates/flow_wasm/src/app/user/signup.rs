use crate::routes::UserRoute;

use patternfly_yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew_nested_router::components::*;

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let links = ChildrenRenderer::new(vec![]);

    let band = ChildrenRenderer::new(vec![
        html! {<>{"Already have an account? "}<Link<UserRoute> active="active" target={UserRoute::Login}>{ "Log in" }</Link<UserRoute>></>},
    ]);

    let title = html_nested! {<Title size={Size::XXLarge}>{"Create your account"}</Title>};
    let toaster = use_toaster();

    let username = use_state_eq(String::new);
    let email = use_state_eq(String::new);
    let password = use_state_eq(String::new);

    let onchangeusername = {
        let username = username.clone();
        Callback::from(move |value| {
            username.set(value);
        })
    };

    let onchangeemail = {
        let email = email.clone();
        Callback::from(move |value| {
            email.set(value);
        })
    };

    let onchangepassword = {
        let password = password.clone();
        Callback::from(move |value| {
            password.set(value);
        })
    };

    let onsubmit = {
        let toaster = toaster.clone();
        let username = username.clone();
        let email: UseStateHandle<String> = email.clone();
        let password = password.clone();
        Callback::from(move |_| {
            if let Some(toaster) = &toaster {
                toaster.toast(format!(
                    "Sign up - Username: {}, Email: {}, Password: {}",
                    &*username, &*email, &*password
                ));
            }
        })
    };

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
                            <Form {onsubmit} method="dialog">
                                <FormGroup label="Username" required=true>
                                    <TextInput required=true name="username" onchange={onchangeusername} value={(*username).clone()} />
                                </FormGroup>
                                <FormGroup label="Email" required=true>
                                    <TextInput required=true name="email" onchange={onchangeemail} value={(*email).clone()} />
                                </FormGroup>
                                <FormGroup label="Password" required=true>
                                    <TextInput required=true name="password" r#type={TextInputType::Password} onchange={onchangepassword} value={(*password).clone()} />
                                </FormGroup>
                                <ActionGroup>
                                    <Button label="Sign up" block=true r#type={ButtonType::Submit} variant={ButtonVariant::Primary}/>
                                </ActionGroup>
                            </Form>
                        </LoginMainBody>
                        <LoginMainFooter
                            {links}
                            {band}
                        >
                        </LoginMainFooter>
                    </LoginMain>
                </Login>
            </ToastViewer>
        </>
    }
}
