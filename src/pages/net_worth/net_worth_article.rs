use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{
    colors_and_sizes::{BtnVariant, Colors, Sizes},
    buttons::button::Button,
    dialogs::dialog::{ 
        Dialog, DialogTrigger, DialogContent, DialogBody, DialogHeader, 
        DialogTitle, DialogFooter, DialogClose
    },
};

#[component]
pub fn NetWorthArticle() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger
                colors=Some(Colors {
                    bg: "var(--warning-bg)".to_string(),
                    fg: "var(--warning-fg)".to_string(),
                    br: "var(--warning-bg)".to_string(),
                    ol: "var(--warning-bg)".to_string(),
                })
                sizes=Some(Sizes {
                    pv: Some(0),
                    ph: Some(2),
                    ..Default::default()
                })
            >
                "Learn more"
            </DialogTrigger>

            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"How to calculate your net worth"</DialogTitle>
                </DialogHeader>

                <DialogBody>
                    <p>"Calculating and monitoring your net worth is a valuable step on your financial to-do list. But what exactly does \"net worth\" mean? What should be included in the calculations? What items are commonly overlooked when calculating net worth?"</p>

                    <h3>"What is net worth?"</h3>
                    <p>"Net worth is defined by Investopedia as the \"amount by which assets exceed liabilities.\" Think of it as the total of everything that you own (assets) minus the total of everything that you owe (liabilities). As an example, you would calculate your net worth listing your home's current resale value as an asset, and the amount you owe on your mortgage as a liability."</p>

                    <h3>"Ballpark vs comprehensive estimates"</h3>
                    <p>"For most of us, a ballpark estimate of our net worth is easy to figure out because we have a rough idea of most of the big-ticket items. We know approximately how much our home and vehicle(s) are worth and what we owe on them. We know&mdash;or can easily look up&mdash;how much we have in the bank and in retirement funds. And we know how much we owe on our credit cards or student loans. But to get a true, comprehensive calculation of your net worth, you need to include all of your significant assets and liabilities, not just the five or six biggest ones that are typically top of mind."</p>

                    <p>"Each of us has to decide for ourselves the dollar amount that we think is significant enough to include in our net worth calculation. As an example, when I calculated my net worth, I didn't bother to include the value of my books. I have a lot of them, but they are mostly discount purchases and paperbacks bought at garage sales, so their resale value doesn't add up to much. But I have a friend who has complete sets of two coffee table book series that are out of print. They are worth enough that she should probably include them when estimating her net worth."</p>

                    <h3>"Commonly overlooked assets and liabilities"</h3>
                    <p>"Often, although people include their house as an asset, they forget to include the valuable items inside that house. If you have an antique dining room set that is worth a lot of money, its value needs to be included in your calculations, along with any other antiques or collectibles that are valuable. If woodworking is your hobby and you have a $1,000 lathe in your garage or basement, its resale value is an asset. If you're a mountain biker or a snowmobiler, include the value of your bike or snowmobile. If you are a quilter (like me) and have an expensive longarm quilting machine (lamentably, unlike me), include its vaue in your list of assets."</p>

                    <p>"Do you owe your brother-in-law for your share of the family's season ticket package for your favorite team? Did your sister pay for the trip that you and your siblings gave your parents as a anniversary present and you still owe her $300 for your share? And don't forget the amount you owe on your \"24 months same as cash\" furniture or appliance purchase. If you owe on it, you need to include it in your calculations."</p>

                    <h3>"Which assets should you include in your net worth estimate?"</h3>
                    <ul>
                        <li>"Savings account(s)"</li>
                        <li>"Health Savings Account (HSA)"</li>
                        <li>"Checking account(s)"</li>
                        <li>"Certificate(s)"</li>
                        <li>"Mutual fund(s) or other non-retirement accounts"</li>
                        <li>"Stocks"</li>
                        <li>"Bonds"</li>
                        <li>"Digital currency (bitcoins, etc.)"</li>
                        <li>"Value of your home"</li>
                        <li>"Value of second home or other property you own (Share a vacation cabin or condo with your siblings or parents? Include your stake in its value as an asset, and your portion of the mortgage, if any, as a liability.)"</li>
                        <li>"Value of vehicles you own (Including snowmobiles or boats or motorcycles, not just your cars and trucks.)"</li>
                        <li>"Value of personal items:"
                        <ul>
                            <li>"Antiques"</li>
                            <li>"Collections"</li>
                            <li>"Jewelry"</li>
                            <li>"Works of art"</li>
                            <li>"Furniture"</li>
                            <li>"Sports equipment"</li>
                            <li>"Tools or machinery for household work or hobbies"</li>
                        </ul>
                        </li>
                        <li>"Retirement savings:"
                        <ul>
                            <li>"401k"</li>
                            <li>"IRA"</li>
                            <li>"Other retirement savings"</li>
                        </ul>
                        </li>
                        <li>"Value of pension or annuity (Use today's value, not future expected income.)"</li>
                        <li>"Cash value of life insurance (Not sure of this amount? Check your policy or ask your agent.)"</li>
                        <li>"Value of your stake in a family business"</li>
                        <li>"Money owed to you (current loan balance only)"</li>
                    </ul>

                    <h3>"Which liabilities should you include in your net worth estimate?"</h3>
                    <ul>
                        <li>"Amount owed on home or other property"</li>
                        <li>"Auto loans"</li>
                        <li>"Other personal loans (Include loans from a bank or credit union and money owed to friends/family, if any.)"</li>
                        <li>"Credit card debt"</li>
                        <li>"Student loans"</li>
                        <li>"Amount owed on furniture or appliances, if any"</li>
                    </ul>

                    <h3>"Using a PFM to track your net worth"</h3>
                    <p>"An easy way to keep track of your net worth without doing a lot of math is to use a personal financial management (PFM) tool. Most PFMs have a section for tracking your net worth, and if you've already linked your bank accounts and credit cards to your PFM, you won't have to do anything to add them to your net worth estimate. Once you've entered your assets and liabilities into the PFM tool, linked accounts will update automatically. If you have assets or liabilities that are outside linked accounts (like a home, vehicles, personal effects), you can update the values manually when they have changed. Then the PFM system does the math for you!"</p>

                    <p>"For example, when I started researching this article, I revisited the net worth section of my PFM. I hadn't updated it in a while, so the value of my condo was no longer accurate given the current, hot real estate market. All I had to do was go to the line for my condo, click on the \"edit\" link and type in the new resale value. After that, the PFM tool did all the work of recalculating everything for me."</p>

                    <br />

                    <p><em>"Source: "<A href="https://www.alliantcreditunion.org/money-mentor/how-to-calculate-your-net-worth" target="_blank">"\"How to calculate your net worth\""</A>" by Pam Leibfried"</em></p>                            
                </DialogBody>

                <DialogFooter>
                    <DialogClose variant={BtnVariant::Primary}>
                        "Close"
                    </DialogClose>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
}
