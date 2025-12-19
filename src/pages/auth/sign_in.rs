use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::A;
use stylance::*;

import_style!(css, "sign_in.module.scss");

#[component]
pub fn SignIn() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Sign In"/>

        // <img src="/images/car-wreck.png" alt="background" />

        <div class={css::page}>
            <div class={css::content_container}>
                <div class={css::logo_container}>
                    <img class={css::logo} src="/images/logo-white.svg" alt="logo" />
                </div>
                <div class={css::title_container}>
                    // <h2 class={css::h2}>"Have a financial wreck?"</h2>
                    // <h2 class={css::h2}>"Manage your personal finances and become a"</h2>
                    // <h1 class={css::h1}>"Carefree Retiree!"</h1>
                    <h1 class={css::h1}>"Go from "<span class={css::bold}>"Financial Wreck"</span>" to"</h1>
                    <h1 class={css::h1}><span class={css::bold}>"Carefree Retiree"</span>"!"</h1>
                </div>
                <div class={css::btns_container}>
                    <div class={css::auth_btns}>
                        <div class="sign_in_btn">
                            <A href="dashboard">Sign In</A>
                            // <Link
                            //     href="__API_DOMAIN__/api/auth/login"
                            //     btnStyles
                            //     colors={primaryWithTransparency}
                            //     {sizes}
                            //     width="full"
                            //     disabled={!!activeLink}
                            //     onclick={() => activeLink = "signIn"}
                            // >
                            //     Sign In
                            //     <Icon icon="ri:login-circle-line" />
                            //     {#snippet disabledContent()}
                            //         Sign In
                            //         {#if activeLink === "signIn"}
                            //             <Icon icon="icomoon-free:spinner2" class="fp_spin" />
                            //         {:else}
                            //             <Icon icon="ri:login-circle-line" />
                            //         {/if}
                            //     {/snippet}
                            // </Link>
                        </div>
                        // <div class={css::register_btn}>
                        //     <Link
                        //         href="__API_DOMAIN__/api/auth/register"
                        //         btnStyles
                        //         colors={primaryWithTransparency}
                        //         {sizes}
                        //         width="full"
                        //         disabled={!!activeLink}
                        //         onclick={() => activeLink = "register"}
                        //     >
                        //         Register
                        //         <Icon icon="ri:account-circle-line" />
                        //         {#snippet disabledContent()}
                        //             Register
                        //             {#if activeLink === "register"}
                        //                 <Icon icon="icomoon-free:spinner2" class="fp_spin" />
                        //             {:else}
                        //                 <Icon icon="ri:account-circle-line" />
                        //             {/if}
                        //         {/snippet}
                        //     </Link>
                        // </div>
                    </div>
                    // <div class={css::mobile_btn}>
                    //     <Button
                    //     colors={secondaryWithTransparency}
                    //     {sizes}
                    //     width="full"
                    //     icon="ri:add-circle-line"
                    //     iconSide="right"
                    //     disabled={!!activeLink}
                    //     disabledIcon={activeLink === "addToHomeScreen" ? "icomoon-free:spinner2" : "ri:add-circle-line" }
                    //     disabledIconShouldSpin={activeLink === "addToHomeScreen"}
                    //     onclick={() => {
                    //         activeLink = "addToHomeScreen";
                    //         alert("Configure SvelteKit as a PWA: https://kit.svelte.dev/docs/service-workers. Then configure PWA installation: https://web.dev/learn/pwa/installation-prompt/");
                    //     }}
                    //     >
                    //     Add to Home Screen
                    //     </Button>
                    // </div>
                    // <div class={css::desktop_btn}>
                    //     <Button
                    //     colors={secondaryWithTransparency}
                    //     {sizes}
                    //     width="full"
                    //     icon="ri:download-cloud-2-line"
                    //     iconSide="right"
                    //     disabled={!!activeLink}
                    //     disabledIcon={activeLink === "installFinancialWreck" ? "icomoon-free:spinner2" : "ri:download-cloud-2-line" }
                    //     disabledIconShouldSpin={activeLink === "installFinancialWreck"}
                    //     onclick={() => {
                    //         activeLink = "installFinancialWreck";
                    //         alert("TODO: Configure SvelteKit as a PWA: https://kit.svelte.dev/docs/service-workers. Then configure PWA installation: https://web.dev/learn/pwa/installation-prompt/");
                    //     }}
                    //     >
                    //     Install Financial Wreck
                    //     </Button>
                    // </div>
                </div>
            </div>
        </div>
    }
}
