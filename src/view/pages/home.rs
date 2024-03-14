use leptos::*;
use crate::view::components::display_card::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home">
            <div class="hero">
                <div class="background">
                    <div class="tagline">
                        <h1>"Bear Flinn"</h1>
                        <h2>"Knowledgeable, Articulate, Adaptable."</h2>
                    </div>
                </div>
                <img class="profile" src="Pfp.jpg" alt="Bear Flinn Headshot"/>
            </div>
            <div class="about">
                <h1>"About Me"</h1>
                <div class="content">
                    <p class="description">
                        "Lifelong gamer and tech enthusiast with a proven track record in IT, esports, and technical communication. From building computers and Minecraft servers alongside my dad to competing in Overwatch esports, I've continuously refined my technical, leadership, and problem-solving skills. My experience at STEP honed my ability to research complex topics and break them down, while my time in esports developed team leadership and mentorship strengths.  Now, as a technical writer specializing in gaming and IT, I'm dedicated to translating complex concepts into clear, engaging documentation."
                    </p>
                    <DisplayCard
                        link = "/resume".to_string()
                        link_is_external = false
                        link_title = "Resume".to_string()
                    >
                        <h3 class="title">"Want to know more?"</h3>
                        <p> "My full background can be found on my resume page."</p>
                        <p class="click-me">"Click to Open"</p>
                    </DisplayCard>
                </div>
            </div>
            <div class="what-i-care-about">
                <h1>"What I Care About"</h1>
                <div class="content">
                    <div class="my-values">
                        <h2>"My Values"</h2>
                        <DisplayCard>
                            <div>
                                <h3>"Accessibility"</h3>
                                <p>"I believe everyone should have equal access to information, opportunities, and resources, regardless of their abilities. I strive to create inclusive experiences and advocate for accessibility in my work"</p>
                            </div>
                        </DisplayCard>
                        <DisplayCard>
                            <div>
                                <h3>"Communication"</h3>
                                <p>"I believe clear, honest communication is the foundation of success. I aim to listen actively, articulate ideas effectively, and collaborate towards shared goals."</p>
                            </div>
                        </DisplayCard>
                        <DisplayCard>
                            <div>
                                <h3>"Empathy"</h3>
                                <p>"I am a firm believer in empathy. I strive to understand and appreciate others' perspectives, and strive to be a part of their journey."</p>
                            </div>
                        </DisplayCard>
                    </div>
                    <img src="playing-dnd.jpg" alt="Bear Flinn Playing Dungeons and Dragons" class="dnd"/>
                </div>
            </div>

        </div>
    }
}