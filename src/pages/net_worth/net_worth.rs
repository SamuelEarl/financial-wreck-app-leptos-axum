use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos::logging::log;
use stylance::*;

use crate::components::{
    colors_and_sizes::BtnVariant,
    accordions::accordion::AccordionItem,
    buttons::button::Button,
    dialogs::dialog::{ 
        Dialog, DialogTrigger, DialogContent, DialogBody, DialogHeader, 
        DialogTitle, DialogDescription, ScrollArea, DialogFooter, DialogClose
    },
};
use crate::utils::format_currency::{format_currency};

import_style!(css, "net_worth.module.scss");

#[component]
pub fn NetWorth() -> impl IntoView {
    provide_meta_context();

    let (net_worth, set_net_worth) = signal(0);
    set_net_worth.set(1_000_000);

    view! {
        <Title text="Financial Wreck | Net Worth"/>

        <h1 class={css::underline}>"Net Worth"</h1>

        <h2>{move || format_currency(net_worth.get(), None)}</h2>

        <AccordionItem id="net_worth" title="What is net worth?">
            <p>"Your net worth is the difference between your assets and your liabilities. To calculate your net worth, add all your assets and liabilities to this page."</p>
            <p>
                <Button
                    variant={BtnVariant::Primary}
                    on:click=move |_| { log!("CLICKED"); }
                >
                    "Learn More"
                </Button>
            </p>
        </AccordionItem>

        <br />

        <p>"Add financial accounts, transfer money between accounts, and add bill pay alerts"</p>

        <h2 class={css::underline}>"Assets"</h2>
        
        <br />

        <h2 class={css::underline}>"Liabilities"</h2>

        <Dialog>
            // 1. The button that sits on your page
            <DialogTrigger>
                <Button
                    variant={BtnVariant::Primary}
                >
                    "Open Terms of Service"
                </Button>
            </DialogTrigger>

            // 2. The Modal Content
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Terms of Service"</DialogTitle>
                    <DialogDescription>
                        "Please read these terms carefully before agreeing."
                    </DialogDescription>
                </DialogHeader>

                <DialogBody>
                    <ScrollArea>
                        // <p style="margin-bottom: 10px; color: #555;">
                        //     "This content is inside the scroll area."
                        //     "It will scroll automatically if the screen is too short."
                        // </p>
                        {
                            (0..20).map(|i| view! {
                                <p style="margin-bottom: 10px; color: #555;">
                                    "Paragraph #" {i} ": This content is inside the scroll area. "
                                    "It will scroll automatically if the screen is too short."
                                </p>
                            }).collect_view()
                        }
                    </ScrollArea>
                </DialogBody>

                <DialogFooter>
                    <DialogClose>
                        <Button
                            variant={BtnVariant::Tertiary}
                        >
                            "Decline"
                        </Button>
                    </DialogClose>
                    
                    // Note: Since this is "Accept", you might want custom logic here
                    // besides just closing, but for now we just wrap a button.
                    <DialogClose>
                        <Button
                            variant={BtnVariant::Secondary}
                        >
                            "Accept"
                        </Button>
                    </DialogClose>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
