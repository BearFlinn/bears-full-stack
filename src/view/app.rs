use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use leptos_use::use_media_query;

use crate::view::components::*;
use crate::view::pages::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta description, etc.
    provide_meta_context();

    let is_mobile = use_media_query("(max-width: 768px)");

    view! {
        // Sets the page title (Text on the tab)
        <Title text="Bear's Portfolio"/>

        // style, icon, and font imports:
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/pkg/bears-leptos-project.css"/>

            // import google icons
            <Link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet"/>

            // import google fonts
            <Link rel="preconnect" href="https://fonts.googleapis.com"/>
            <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="crossorigin"/>
            <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap"/>

            // import social icons
            <Script src="https://kit.fontawesome.com/9bb7da5a3b.js" crossorigin="anonymous"></Script>

        <Router>
            <div class="app">
                <div class={move || {if is_mobile.get() {"app-mobile"} else {"app-desktop"}}}>
                    {move || {if is_mobile.get() {view! {<MobileNav/>}} else {view! {<Sidebar/>}}}}
                    <div class="pages">
                        <div class="mobile-warning" style={move || {if is_mobile.get() {"display: block"} else {"display: none"}}}>
                            <h3>"Warning: WIP"</h3>
                            <p>"The mobile version of this site is under development, please use the desktop version for the best experience."</p>
                        </div>
                        <div class="page-content">
                            <Routes>
                                <Route path="" view=Home/>
                                <Route path="portfolio" view=Portfolio/>
                                <Route path="resume" view=Resume/>
                                <Route path="contact" view=Contact/>
                                <Route path="playground" view=Playground/>
                            </Routes>
                        </div>
                        <Footer/>
                    </div>
                </div>
            </div>
        </Router>
    }
}