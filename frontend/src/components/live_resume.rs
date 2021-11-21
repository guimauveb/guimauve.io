#[cfg(feature = "editable")]
use {
    crate::{
        components::resume_project::ResumeProject,
        entities::interfaces::{IProject, Status},
        service::{future::handle_future, projects::get_resume_projects},
    },
    std::collections::HashMap,
    yew::html,
    yew_functional::{function_component, use_effect_with_deps, use_state},
};

#[cfg(feature = "editable")]
#[function_component(LiveResume)]
pub fn resume() -> Html {
    let (resume_projects, set_resume_projects) = use_state(HashMap::default);

    use_effect_with_deps(
        move |_| {
            let future = async { get_resume_projects().await };
            handle_future(
                future,
                move |data: Result<HashMap<i32, IProject>, Status>| {
                    if let Ok(projects) = data {
                        set_resume_projects(projects);
                    }
                },
            );
            || {}
        },
        (),
    );
    html! {
        //<div style="display: flex; justify-content: center; flex: 1;">
        //    <div style="flex: 1; max-width: 1024px" flex_direction="column">
        //        <div display: flex; margin-bottom:"20px" flex_direction="column">

        //            // DETAILS
        //            <div style="margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p>{"Guillaume Bournel"}</p>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>
        //                <div display: flex; flex: 1">
        //                    <div display: flex; flex_direction="column" flex: 0 328px">
        //                        <div display: flex; margin-bottom:"8px">
        //                            <i style="margin-right: 8px;" class="fa fa-stack-overflow"/>
        //                            <p>{"/users/11676080/guimauve"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"8px">
        //                            <i style="margin-right: 8px;" class="fa fa-linkedin"/>
        //                            <p>{"/in/guillaume-b-4a167b19b"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"8px">
        //                            <i style="margin-right: 8px;" class="fa fa-envelope-o"/>
        //                            <p>{"contact@guimauve.io"}</p>
        //                        </div>
        //                    </div>
        //                    <div display: flex; flex_direction="column" flex: 0">
        //                        <div display: flex; margin-bottom:"8px">
        //                            <i style="margin-right: 8px;" class="fa fa-github"/>
        //                            <p>"/guimauveb"</p>
        //                        </div>
        //                        <p>{"guimauve.io"}</p>
        //                    </div>

        //                    <div display: flex; flex: 1" justify-content: flex-end">
        //                        <p>{"Nantes, FR"}</p>
        //                    </div>
        //                </div>
        //            </div>

        //            // ABOUT ME
        //            <div margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p style="font-weight: bold;">{"ABOUT ME"}</p>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>
        //                <p>{"I am a passionate software developer who enjoys spending time in the various fields of programming. I am self-educated, and always keeping up to date and strengthening my skills play a big role in my everyday life. I have found programming to be a lifestyle rather than a career choice.\nThe languages I have used the most include Rust, Typescript, Javascript, Python, C++, Bash script. I also tinker with Haskell from time to time. I have a strong interest in web development as well as OS development, embedded development, game development and compilers, even though most of my experience comes from web development. I am currently focusing on Rust and functional programming."}</p>
        //            </div>

        //            // OBJECTIVE
        //            <div margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p style="font-weight: bold;">{"OBJECTIVE"}</p>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>
        //                <p>{"A position developing software and services."}</p>
        //            </div>

        //            // SKILLS
        //            <div margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p style="font-weight: bold;">{"SKILLS"}</p>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>

        //                <div display: flex; flex: 1">
        //                    // Languages
        //                    <div display: flex; flex: 1" flex_direction="column">
        //                        <div margin-bottom:"8px">
        //                            <p>{"Languages"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p>{"Rust"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p>{"Typescript/Javascript"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p>{"Python"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p>{"C++"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p>{"C"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p>{"Bash script"}</p>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p>{"Haskell"}</p>
        //                        </div>
        //                    </div>

        //                    // Libraries/Frameworks
        //                    <div display: flex; flex: 1" flex_direction="column">
        //                        <div margin-bottom:"8px">
        //                            <p value="Libraries and frameworks"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="React/React Native (mostly using Typescript)"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Redux"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Django"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Actix"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Yew"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="curses/ncurses"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="curl/curlpp"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Flask"/>
        //                        </div>
        //                        <div display: flex; margin-bottom:"2px">
        //                            <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="jQuery"/>
        //                        </div>
        //                    </div>

        //                    // DevOps/Services
        //                    <div display: flex; flex: 1" flex_direction="column" align-items:"flex-end">
        //                        <div>
        //                            <div margin-bottom:"8px">
        //                                <p value="DevOps/Services"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Git"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Github"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="AWS"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Integromat"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="nginx"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Postgres"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="MySQL"/>
        //                            </div>
        //                            <div display: flex; margin-bottom:"2px">
        //                                <i style="margin-right: 8px;" class="fa fa-dot-circle-o"/><p value="Docker"/>
        //                            </div>
        //                        </div>
        //                    </div>
        //                </div>
        //            </div>

        //            // SOFTWARE DEVELOPER EXPERIENCE
        //            <div margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p font_weight="bold" value="SOFTWARE DEVELOPER EXPERIENCE"/>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>

        //                // Dashdoc
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Full stack developer - Dashdoc"/>
        //                        <p font_weight="bold" value="January 2021 - Present"/>
        //                    </div>
        //                    <p font_size="0.8em" value="Working on every end of the codebase (back-end - integrations - front-end - mobile application) playing various developer roles. From implementing quick but much needed features (such as small API or UI changes needed for a new customer) to larger projects involving all ends of the product (such as bigger features that have been requested by a few clients for some time).\n\nSince I am the one to whom customer support goes first in case of trouble, I also play an important role in quick and critical bug resolution.\n\nI write a lot of React using Typescript, whether it be by taking part in the refactoring of the UI by creating 'ui-kit' generic and reusable components using functional components, or simply by adding new features. I also work on back-end features, such as adding and updating API endpoints and creating in-house tools to help customer support and operations team.\n\nI also work with Integromat to create new deployment scenarios, fix existing ones, add new modules and keep them up to date.\n\nThe languages and frameworks I have been using the most are Typescript/React and Python/Django."/>
        //                </div>

        //                // guimauve
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Full stack developer - guimauve"/>
        //                        <p font_weight="bold" value="August 2019 - Present"/>
        //                    </div>
        //                    <p font_size="0.8em" value="Creating all kinds of projects, consistently learning new technologies. Currently focusing on Rust and functional programming."/>
        //                </div>

        //                // guimauve.io
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Blogger - guimauve.io"/>
        //                        <p font_weight="bold" value="June 2020 - Present"/>
        //                    </div>
        //                    <p font_size="0.8em" value="I maintain an active blog discussing anything related to what I encounter during project development. I also write tutorials and guides."/>
        //                </div>

        //                // Institut Sylvie
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Full stack web developer - institut-sylvie.fr"/>
        //                        <p font_weight="bold" value="January 2020 - May 2020"/>
        //                    </div>
        //                    <ResumeProject
        //                        project={match resume_projects.get(&1) {
        //                            Some(institut_project) => institut_project.clone(),
        //                            _ => IProject::default(),
        //                        }}
        //                    />
        //                </div>
        //            </div>

        //            // OTHER EXPERIENCE
        //            <div margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p font_weight="bold" value="OTHER EXPERIENCE"/>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>

        //                // Front Desk - Kyriad Hotel Nantes
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Front Desk - Kyriad Hotel - Nantes"/>
        //                        <p font_weight="bold" value="December 2019 - July 2020"/>
        //                    </div>
        //                    <p font_size="0.8em" value="• Welcoming international guests in a warm and friendly manner.\n• Answering questions and addressing complaints\n• Knowing all essential aspects of the hotel operations.\n• Registering guests and managing the rooming chart.\n• Collecting payments by accepting cash, check, or charge payments from customers."/>
        //                    </div>

        //                // Auto Technician - Norauto Saint-Herblain
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Auto Technician - Norauto - Saint-Herblain"/>
        //                        <p font_weight="bold" value="June 2018 - September 2019"/>
        //                    </div>
        //                    <p font_size="0.8em" value="Performed maintenance and repairs on all makes and models, including:\n\n• Oil changes\n• Tire rotations and replacement\n• Transmission flushes\n• Front-end alignments\n• Battery installations\n• Headlight/taillight installations\n\nConducted necessary repairs and took vehicles for test drives to verify soundness." />
        //                </div>
        //            </div>

        //            // PROJECTS
        //            <div margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p font_weight="bold" value="PROJECTS (See more projects at guimauve.io/projects)"/>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>
        //                <div>
        //                    {for resume_projects.iter().filter(|(&id, _)| id != 1).  map(|(_, project)| {
        //                            html! {
        //                                <ResumeProject project={project} />
        //                            }
        //                        })
        //                    }
        //                </div>
        //            </div>

        //            // EDUCATION
        //            <div margin-bottom:"20px">
        //                <div margin-bottom:"8px">
        //                    <p font_weight="bold" value="EDUCATION"/>
        //                </div>
        //                <div margin-bottom:"8px">
        //                    <<hr style="border: 0; border-top: 1px solid rgb(41, 41, 41);"/>
        //                </div>

        //                // Law
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Bachelor of Law - University of Poitiers"/>
        //                    </div>
        //                    <p font_size="0.8em" value="Public law/Tax law"/>
        //                </div>

        //                // MIP
        //                <div margin-bottom:"20px">
        //                    <div display: flex; justify-content: space-between" margin-bottom:"12px">
        //                        <p font_weight="bold" value="Mathematics - Physics - Computer Science - University of Nantes"/>
        //                    </div>
        //                    <p font_size="0.8em" value="Mathematics/Computer science"/>
        //                </div>

        //            </div>

        //        </div>
        //    </div>
        //</div>
    }
}
