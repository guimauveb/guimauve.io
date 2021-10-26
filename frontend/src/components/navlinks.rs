use {
    crate::{
        components::{
            box_component::BoxComponent,
            navlink::NavLink,
            text::{Text, TextVariant},
        },
        routes::AppRoute,
        store::store::BlogStore,
        API_URL,
    },
    std::{collections::HashMap, rc::Rc},
    yew::{html, Callback, Properties},
    yew_functional::{function_component, use_context, use_state},
    yew_router::components::RouterAnchor,
};

#[derive(Clone, PartialEq)]
pub struct TNavLink<'a> {
    pub label: &'a str,
    pub on_hover_label: &'a str,
    pub route: Option<AppRoute>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct NavLinksProps {}

#[function_component(NavLinks)]
pub fn navlinks(_: &NavLinksProps) -> Html {
    let context = use_context::<Rc<BlogStore>>().expect("No context found!");
    let is_error = context.is_error;

    let nav_links: Rc<Vec<TNavLink>> = Rc::new(vec![
        TNavLink {
            label: "/articles",
            on_hover_label: "$ cd ~/articles",
            route: Some(AppRoute::Articles),
        },
        TNavLink {
            label: "/tags",
            on_hover_label: "$ cd ~/tags",
            route: Some(AppRoute::Tags),
        },
        TNavLink {
            label: "/projects",
            on_hover_label: "$ cd ~/projects",
            route: Some(AppRoute::Projects),
        },
        TNavLink {
            label: "/about",
            on_hover_label: "$ cd ~/about",
            route: Some(AppRoute::About),
        },
        TNavLink {
            label: "/resume",
            on_hover_label: "$ wget ~/resume.pdf",
            route: None,
        },
    ]);

    let (hovered_nav_links, set_hovered_nav_links) = {
        let nav_links = nav_links.clone();
        use_state(move || {
            nav_links
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

    html! {
        <>
            <BoxComponent display="flex" flex="0">
                <RouterAnchor<AppRoute> route={AppRoute::Home}>
                    <NavLink label="guimauve" />
                </RouterAnchor<AppRoute>>
                {if is_error {
                    html! {
                        <BoxComponent display="flex" flex="1" ml="12px" align_items="center" justify_content="center" >
                            <i style="color: rgb(178, 34, 34);" class="fa fa-exclamation-triangle"></i>
                            <BoxComponent ml="8px" align_items="baseline">
                                <Text variant={TextVariant::Caption} value="Error" />
                            </BoxComponent>
                        </BoxComponent>
                    }
                } else {
                    html! {}
                }}
            </BoxComponent>
            <BoxComponent display="flex" flex="1" justify_content="center">
                {for nav_links.iter().map(|nav_link| {
                    html! {
                        <BoxComponent ml="2px" mr="2px">
                            {match &nav_link.route {
                                Some(route) => html! {
                                    <RouterAnchor<AppRoute> route={route}>
                                        <NavLink
                                            label={&nav_link.label}
                                            on_hover={&on_label_hovered}
                                            on_hover_label={&nav_link.on_hover_label}
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
                                    <a href={API_URL.to_owned() + "/media/resume/resume_Guillaume_Bournel_2021.pdf"} target="_blank">
                                        <NavLink
                                            label={&nav_link.label}
                                            on_hover={&on_label_hovered}
                                            on_hover_label={&nav_link.on_hover_label}
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
                        </BoxComponent>
                        }
                    })
                }
            </BoxComponent>
        </>
    }
}
