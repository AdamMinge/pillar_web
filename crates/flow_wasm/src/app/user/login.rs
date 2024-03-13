use patternfly_yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_nested_router::components::*;
use yew_nested_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct LoginPageProps<T: Target> {
    pub signup: T,
    pub recovery: T,
}

#[function_component(LoginPage)]
pub fn login_page<T: Target>(props: &LoginPageProps<T>) -> Html {
    let links = ChildrenRenderer::new(vec![]);

    let band = ChildrenRenderer::new(vec![
        html! {<>{"Need an account? "}<Link<T> active="active" target={props.signup.clone()}>{ "Sign up" }</Link<T>></>},
        html! {<Link<T> active="active" target={props.recovery.clone()}>{ "Forgot password?" }</Link<T>>},
    ]);

    let title = html_nested! {<Title size={Size::XXLarge}>{"Login to your account"}</Title>};

    let username = use_state_eq(String::new);
    let password = use_state_eq(String::new);

    let onchangeusername = {
        let username = username.clone();
        Callback::from(move |value| {
            username.set(value);
        })
    };

    let onchangepassword = {
        let password = password.clone();
        Callback::from(move |value| {
            password.set(value);
        })
    };

    let user_login = {
        let client = flow_api::hooks::use_client();
        let username = username.clone();
        let password = password.clone();
        use_async(async move { client.unwrap().login(&username, &password).await })
    };

    let onsubmit = {
        let user_login = user_login.clone();

        Callback::from(move |_| {
            user_login.run();
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
                                <FormGroup label="Username">
                                    <TextInput required=true name="username" onchange={onchangeusername} value={(*username).clone()} />
                                </FormGroup>
                                <FormGroup label="Password">
                                    <TextInput required=true name="password" r#type={TextInputType::Password} onchange={onchangepassword} value={(*password).clone()} />
                                </FormGroup>
                                <ActionGroup>
                                    <Button label="Log In" block=true r#type={ButtonType::Submit} variant={ButtonVariant::Primary}/>
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
