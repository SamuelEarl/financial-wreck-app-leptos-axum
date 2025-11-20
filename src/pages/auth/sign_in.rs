use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::A;
use stylance::*;

import_style!(style, "sign_in.module.scss");

/// Shows progress toward a goal.
#[component]
pub fn SignIn() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Financial Wreck | Sign In"/>

        // <img src="/images/car-wreck.png" alt="background" />

        <div class=style::page>
            <div class=style::content_container>
                <div class=style::logo_container>
                    <img class=style::logo src="/images/logo-white.svg" alt="logo" />
                </div>
                <div class=style::title_container>
                    <h1>Manage Your<br/>Personal Finances</h1>
                </div>
                <div class=style::btns_container>
                    <div class=style::auth_btns>
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
                            //             <Icon icon="icomoon-free:spinner2" class=style::fp_spin" />
                            //         {:else}
                            //             <Icon icon="ri:login-circle-line" />
                            //         {/if}
                            //     {/snippet}
                            // </Link>
                        </div>
                        // <div class=style::register_btn>
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
                        //                 <Icon icon="icomoon-free:spinner2" class=style::fp_spin />
                        //             {:else}
                        //                 <Icon icon="ri:account-circle-line" />
                        //             {/if}
                        //         {/snippet}
                        //     </Link>
                        // </div>
                    </div>
                    // <div class=style::mobile_btn>
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
                    // <div class=style::desktop_btn>
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
