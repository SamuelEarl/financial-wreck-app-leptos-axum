# Dialogs

---

{{
    <Dialog>
        <DialogTrigger>
            <Button
                variant={BtnVariant::Primary}
            >
                "Open Terms of Service"
            </Button>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>"Terms of Service"</DialogTitle>
                <DialogDescription>
                    "Please read these terms carefully before agreeing."
                </DialogDescription>
            </DialogHeader>
            <DialogBody>
                <ScrollArea>
                    <p>"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."</p>

                    <p>"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."</p>
                    
                    <p>"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."</p>
                    
                    <p>"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."</p>
                    
                    <p>"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."</p>
                    
                    <p>"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."</p>
                    
                    <p>"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."</p>
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
}}

<br />

The height of the dialog adjusts to fit to the content. However, if there is more content than the dialog body can hold, then the dialog height will fill the available screen space and the dialog content will scroll so it doesn't overflow the available space.

```rust
use crate::components::{
    colors_and_sizes::BtnVariant,
    buttons::button::Button,
};
use crate::components::dialogs::dialog::{ 
    Dialog, DialogTrigger, DialogContent, DialogBody, DialogHeader, 
    DialogTitle, DialogDescription, ScrollArea, DialogFooter, DialogClose
};

<Dialog>
    <DialogTrigger>
        <Button
            variant={BtnVariant::Primary}
        >
            "Open Terms of Service"
        </Button>
    </DialogTrigger>
    <DialogContent>
        <DialogHeader>
            <DialogTitle>"Terms of Service"</DialogTitle>
            <DialogDescription>
                "Please read these terms carefully before agreeing."
            </DialogDescription>
        </DialogHeader>
        <DialogBody>
            <ScrollArea>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
                <p style="margin-bottom: 10px; color: #555;">
                    "Paragraph: This content is inside the scroll area."
                    "It will scroll automatically if the screen is too short."
                </p>
            </ScrollArea>
        </DialogBody>
        <DialogFooter>
            <DialogClose>
                <button style="padding: 8px 16px;">"Decline"</button>
            </DialogClose>

            // Note: Since this is "Accept", you might want custom logic here
            // besides just closing, but for now we just wrap a button.
            <DialogClose>
                <button style="background: black; color: white; padding: 8px 16px;">
                    "Accept"
                </button>
            </DialogClose>
        </DialogFooter>
    </DialogContent>
</Dialog>
```
