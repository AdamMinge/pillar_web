use patternfly_yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

#[function_component(PasswordRecoveryPage)]
pub fn password_recovery_page() -> Html {
    let links = ChildrenRenderer::new(vec![]);

    let band = ChildrenRenderer::new(vec![]);

    let title = html_nested! {<Title size={Size::XXLarge}>{"Recovery your password"}</Title>};
    let toaster = use_toaster();

    let email = use_state_eq(String::new);

    let onchangeemail = {
        let email = email.clone();
        Callback::from(move |value| {
            email.set(value);
        })
    };

    let onsubmit = {
        let toaster = toaster.clone();
        let email = email.clone();
        Callback::from(move |_| {
            if let Some(toaster) = &toaster {
                toaster.toast(format!("Recovery - email: {}", &*email));
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
                            description="If an account is created for this email address, we will send an recovery email to it."
                        />
                        <LoginMainBody>
                            <Form {onsubmit} method="dialog">
                                <FormGroup label="Email">
                                    <TextInput required=true name="email" onchange={onchangeemail} value={(*email).clone()} />
                                </FormGroup>

                                <ActionGroup>
                                    <Button label="Recovery" block=true r#type={ButtonType::Submit} variant={ButtonVariant::Primary}/>
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
