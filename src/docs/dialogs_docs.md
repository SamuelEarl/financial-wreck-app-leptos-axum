# Dialogs

---

{{
    <Dialog>
        <DialogTrigger variant={BtnVariant::Primary}>
            "Open Terms of Service"
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>"Terms of Service"</DialogTitle>
                <DialogDescription>
                    "Please read these terms carefully before agreeing."
                </DialogDescription>
            </DialogHeader>
            <DialogBody>
                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>

                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
                
                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
                
                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
                
                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
                
                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
                
                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
            </DialogBody>
            <DialogFooter>
                <DialogClose 
                    colors=Some(Colors {
                        bg: "transparent".to_string(),
                        fg: "var(--secondary-bg)".to_string(),
                        br: "var(--secondary-bg)".to_string(),
                        ol: "var(--secondary-bg)".to_string(),
                    })
                >
                    "Decline"
                </DialogClose>
                <DialogClose variant={BtnVariant::Secondary}>
                    "Accept"
                </DialogClose>
            </DialogFooter>
        </DialogContent>
    </Dialog>
}}

<br />

The height of the dialog adjusts to fit to the content. However, if there is more content than the dialog body can hold, then the height of the dialog will fill the available screen space and the dialog content will scroll so it doesn't overflow the available space.

```rust
use crate::components::{
    colors_and_sizes::BtnVariant,
    buttons::button::Button,
};
use crate::components::dialogs::dialog::{ 
    Dialog, DialogTrigger, DialogContent, DialogBody, DialogHeader, 
    DialogTitle, DialogDescription, DialogFooter, DialogClose
};

<Dialog>
    // The button that sits on the page.
    <DialogTrigger variant={BtnVariant::Primary}>
        "Open Terms of Service"
    </DialogTrigger>

    // The dialog content
    <DialogContent>
        <DialogHeader>
            <DialogTitle>"Terms of Service"</DialogTitle>
            <DialogDescription>
                "Please read these terms carefully before agreeing."
            </DialogDescription>
        </DialogHeader>
        <DialogBody>
            <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
                
            <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
            
            <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."</p>
        </DialogBody>
        <DialogFooter>
            <DialogClose 
                colors=Some(Colors {
                    bg: "transparent".to_string(),
                    fg: "var(--secondary-bg)".to_string(),
                    br: "var(--secondary-bg)".to_string(),
                    ol: "var(--secondary-bg)".to_string(),
                })
            >
                "Decline"
            </DialogClose>

            // Note: Since this is "Accept", you might want custom logic here
            // besides just closing, but for this example we just wrap a button.
            <DialogClose variant={BtnVariant::Secondary}>
                "Accept"
            </DialogClose>
        </DialogFooter>
    </DialogContent>
</Dialog>
```
