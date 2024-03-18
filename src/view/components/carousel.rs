use std::time::Duration;

use leptos::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SlideAnimationState {
    Active,
    Inactive(Direction),
    Entering(Direction),
    Exiting(Direction),
    Reset(Direction),
}

impl SlideAnimationState {
    fn as_str(&self) -> &str {
        match self {
            SlideAnimationState::Active => "active",
            SlideAnimationState::Inactive(direction) => {
                match direction {
                    Direction::Left => "inactive-left",
                    Direction::Right => "inactive-right",
                }
            }
            SlideAnimationState::Entering(direction) => {
                match direction {
                    Direction::Left => "entering-left",
                    Direction::Right => "entering-right",
                }
            }
            SlideAnimationState::Exiting(direction) => {
                match direction {
                    Direction::Left => "exiting-left",
                    Direction::Right => "exiting-right",
                }
            }
            SlideAnimationState::Reset(direction) => {
                match direction {
                    Direction::Left => "reset-left",
                    Direction::Right => "reset-right",
                }
            }
        }
    }
}

#[component]
pub fn Slide (
    children: Children,
    id: usize
) -> impl IntoView {
    let animation_state = use_context::<ReadSignal<SlideAnimationState>>().expect("Animation state not found");
    view! {
        <div class={move || format!("slide-{}", animation_state.get().as_str())}>
            <div class={format!("slide-{}", id)}>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn Carousel(
    mut children: ChildrenFnMut,
) -> impl IntoView {
    let (active_slide, set_active_slide) = create_signal(0);
    let (animation_state, set_animation_state) = create_signal(SlideAnimationState::Active);

    provide_context(animation_state);

    let child_vec = children().nodes;
    let slide_count = child_vec.len();
    logging::log!("slide count: {}", slide_count);

    let visible_slide = create_memo(move |_| {
        child_vec.iter().enumerate().find_map(|(i, node)| {
            if i == active_slide.get() { Some((*node).clone()) } else { None }
        }).unwrap_or_default()
    });

    let set_next_slide = move |_| {
        set_active_slide.update(|Index| *Index = (*Index + 1) % slide_count);
    };
    let set_prev_slide = move |_| {
        set_active_slide.update(|Index| *Index = (*Index + slide_count - 1) % slide_count);
    };

    let delay = 300;

    let click_next = {
        let set_animation_state = set_animation_state;
    
        move |_| {
            spawn_local(async move {
                set_animation_state.set(SlideAnimationState::Exiting(Direction::Left));
                let _ = set_timeout_with_handle(move || set_animation_state.set(SlideAnimationState::Inactive(Direction::Left)), Duration::from_millis(delay));
    
                let _ = set_timeout_with_handle(move || set_animation_state.set(SlideAnimationState::Reset(Direction::Left)), Duration::from_millis(delay / 3));
    
                let _ = set_timeout_with_handle(move || {
                    set_next_slide(active_slide.get());
                    set_animation_state.set(SlideAnimationState::Entering(Direction::Right)); 
                    logging::log!("active slide: {}", active_slide.get());
                 }, Duration::from_millis(delay));
    
                let _ = set_timeout_with_handle(move || set_animation_state.set(SlideAnimationState::Active), Duration::from_millis(delay));
            });
        }
    };

    let click_prev = {
        let set_animation_state = set_animation_state;
    
        move |_| {
            spawn_local(async move {
                set_animation_state.set(SlideAnimationState::Exiting(Direction::Right));
                let _ = set_timeout_with_handle(move || set_animation_state.set(SlideAnimationState::Inactive(Direction::Right)), Duration::from_millis(delay));
    
                let _ = set_timeout_with_handle(move || set_animation_state.set(SlideAnimationState::Reset(Direction::Right)), Duration::from_millis(delay / 3));
    
                let _ = set_timeout_with_handle(move || {
                    set_prev_slide(active_slide.get());
                    set_animation_state.set(SlideAnimationState::Entering(Direction::Left)); 
                    logging::log!("active slide: {}", active_slide.get());
                 }, Duration::from_millis(delay));
    
                let _ = set_timeout_with_handle(move || set_animation_state.set(SlideAnimationState::Active), Duration::from_millis(delay));
            });
        }
    };

    view! {
        <div class="carousel">
            <button class="carousel-button" on:click=click_next>
                <i class="material-icons">"chevron_right"</i>
            </button>
            <div class="slides">
                {visible_slide}
            </div>
            <button class="carousel-button" on:click=click_prev>
                <i class="material-icons">"chevron_left"</i>
            </button>
        </div>
    }
}