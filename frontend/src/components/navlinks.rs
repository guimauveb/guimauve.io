use {
    crate::{components::navlink::NavLink, routes::AppRoute, store::store::BlogStore, API_URL},
    std::{collections::HashMap, rc::Rc},
    yew::{html, Callback},
    yew_functional::{function_component, use_context, use_state},
    yew_router::components::RouterAnchor,
};

#[derive(Clone, PartialEq)]
pub struct TNavLink {
    pub label: &'static str,
    pub label_on_hover: &'static str,
    pub route: Option<AppRoute>,
}

const NAV_LINKS: &[TNavLink] = &[
    TNavLink {
        label: "/articles",
        label_on_hover: "$ cd ~/articles",
        route: Some(AppRoute::Articles),
    },
    TNavLink {
        label: "/tags",
        label_on_hover: "$ cd ~/tags",
        route: Some(AppRoute::Tags),
    },
    TNavLink {
        label: "/projects",
        label_on_hover: "$ cd ~/projects",
        route: Some(AppRoute::Projects),
    },
    TNavLink {
        label: "/about",
        label_on_hover: "$ cd ~/about",
        route: Some(AppRoute::About),
    },
    TNavLink {
        label: "/resume",
        label_on_hover: "$ wget ~/resume.pdf",
        route: None,
    },
];

const RESUME_SLUG: &str = "/media/resume/resume_Guillaume_Bournel_2021.pdf";

#[function_component(NavLinks)]
pub fn navlinks() -> Html {
    let context = use_context::<Rc<BlogStore>>().expect("No context found!");
    let is_error = context.is_error;

    let (hovered_nav_links, set_hovered_nav_links) = {
        use_state(move || {
            NAV_LINKS
                .iter()
                .map(|nav_link| (nav_link.label, false))
                .collect::<HashMap<&str, bool>>()
        })
    };

    let on_label_hovered: Callback<(&str, bool)> = {
        let (hovered_nav_links, set_hovered_nav_links) =
            (hovered_nav_links.clone(), set_hovered_nav_links.clone());
        Callback::from(move |(updated_label, is_hovered): (&str, bool)| {
            // Could be improved using `use_previous` like hook
            let mut updated_hovered_nav_links = (*hovered_nav_links).clone();
            for (label, hovered) in updated_hovered_nav_links.iter_mut() {
                if *label == updated_label {
                    *hovered = is_hovered;
                } else {
                    *hovered = false;
                }
            }
            set_hovered_nav_links(updated_hovered_nav_links);
        })
    };

    // Make completely const?
    let mut resume_url = String::with_capacity(API_URL.len() + RESUME_SLUG.len());
    resume_url.push_str(API_URL);
    resume_url.push_str(RESUME_SLUG);

    html! {
        <>
            <div style="display: flex; flex: 1;">
                <RouterAnchor<AppRoute> route={AppRoute::Home}>
                    <NavLink label="guimauve" />
                </RouterAnchor<AppRoute>>
                {if is_error {
                    html! {
                        <div style="display: flex; flex: 1; margin-left: 12px; align-items: center; justify-content: center;" >
                            <i style="color: rgb(178, 34, 34);" class="fa fa-exclamation-triangle"></i>
                            <div style="margin-left: 8px; align-items: baseline;">
                                <p style="font-size: .7rem;">{"Error"}</p>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
            <div style="display: flex; flex: 1; justify-content: center;">
                {for NAV_LINKS.iter().map(|nav_link| {
                    html! {
                        <div style="margin-left: 2px; margin-right: 2px;">
                            {match &nav_link.route {
                                Some(route) => html! {
                                    <RouterAnchor<AppRoute> route={route}>
                                        <NavLink
                                            label={&nav_link.label}
                                            on_hover={&on_label_hovered}
                                            label_on_hover={&nav_link.label_on_hover}
                                            is_other_link_hovered={hovered_nav_links.iter().fold(false, |mut acc, (label, hovered)| {
                                                if *hovered && label != &nav_link.label {
                                                    acc = true;
                                                }
                                                acc
                                               })
                                            }
                                            hovered={hovered_nav_links.get(&nav_link.label).expect("Could not find nav link")}
                                        />
                                    </RouterAnchor<AppRoute>>
                                },
                                None => html! {
                                    <a href={&resume_url} target="_blank">
                                        <NavLink
                                            label={&nav_link.label}
                                            on_hover={&on_label_hovered}
                                            label_on_hover={&nav_link.label_on_hover}
                                            is_other_link_hovered={hovered_nav_links.iter().fold(false, |mut acc, (label, hovered)| {
                                                if *hovered && label != &nav_link.label {
                                                    acc = true;
                                                }
                                                acc
                                               })
                                            }
                                            hovered={hovered_nav_links.get(&nav_link.label).expect("Could not find nav link")}
                                        />
                                    </a>
                                    }
                                }
                            }
                        </div>
                        }
                    })
                }
            </div>
        </>
    }
}
