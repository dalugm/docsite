#[derive(
    Clone,
    Copy,
    dioxus_router::prelude::Routable,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize
)]
pub enum BookRoute {
    #[route("/")]
    Index {},
    #[route("/introduction/roadmap")]
    IntroductionRoadmap {},
    #[route("/guide")]
    GuideIndex {},
    #[route("/guide/tooling")]
    GuideTooling {},
    #[route("/guide/new_app")]
    GuideNewApp {},
    #[route("/guide/component")]
    GuideComponent {},
    #[route("/guide/assets")]
    GuideAssets {},
    #[route("/guide/state")]
    GuideState {},
    #[route("/guide/routing")]
    GuideRouting {},
    #[route("/guide/fetching")]
    GuideFetching {},
    #[route("/guide/multiplatform")]
    GuideMultiplatform {},
    #[route("/guide/backend")]
    GuideBackend {},
    #[route("/guide/deploy")]
    GuideDeploy {},
    #[route("/guide/next_steps")]
    GuideNextSteps {},
    #[route("/essentials")]
    EssentialsIndex {},
    #[route("/essentials/lifecycle")]
    EssentialsLifecycleIndex {},
    #[route("/essentials/state")]
    EssentialsStateIndex {},
    #[route("/essentials/breaking")]
    EssentialsBreakingIndex {},
    #[route("/essentials/structure")]
    EssentialsStructureIndex {},
    #[route("/essentials/rsx")]
    EssentialsRsxIndex {},
    #[route("/reference")]
    ReferenceIndex {},
    #[route("/router")]
    RouterIndex {},
    #[route("/router/reference/routing-update-callback")]
    RouterReferenceRoutingUpdateCallback {},
    #[route("/reference/assets")]
    ReferenceAssets {},
    #[route("/reference/web")]
    ReferenceWebIndex {},
    #[route("/reference/desktop")]
    ReferenceDesktopIndex {},
    #[route("/reference/mobile")]
    ReferenceMobileIndex {},
    #[route("/reference/mobile/apis")]
    ReferenceMobileApis {},
    #[route("/reference/ssr")]
    ReferenceSsr {},
    #[route("/reference/fullstack")]
    ReferenceFullstackIndex {},
    #[route("/reference/fullstack/server_functions")]
    ReferenceFullstackServerFunctions {},
    #[route("/reference/fullstack/extractors")]
    ReferenceFullstackExtractors {},
    #[route("/reference/fullstack/middleware")]
    ReferenceFullstackMiddleware {},
    #[route("/reference/fullstack/authentication")]
    ReferenceFullstackAuthentication {},
    #[route("/reference/fullstack/routing")]
    ReferenceFullstackRouting {},
    #[route("/cookbook/publishing")]
    CookbookPublishing {},
    #[route("/cookbook/antipatterns")]
    CookbookAntipatterns {},
    #[route("/cookbook/error_handling")]
    CookbookErrorHandling {},
    #[route("/cookbook/integrations")]
    CookbookIntegrationsIndex {},
    #[route("/cookbook/integrations/logging")]
    CookbookIntegrationsLogging {},
    #[route("/cookbook/integrations/internationalization")]
    CookbookIntegrationsInternationalization {},
    #[route("/cookbook/state")]
    CookbookStateIndex {},
    #[route("/cookbook/state/external")]
    CookbookStateExternalIndex {},
    #[route("/cookbook/state/custom_hooks")]
    CookbookStateCustomHooksIndex {},
    #[route("/cookbook/testing")]
    CookbookTesting {},
    #[route("/cookbook/examples")]
    CookbookExamples {},
    #[route("/cookbook/tailwind")]
    CookbookTailwind {},
    #[route("/cookbook/optimizing")]
    CookbookOptimizing {},
    #[route("/migration")]
    MigrationIndex {},
    #[route("/migration/hooks")]
    MigrationHooks {},
    #[route("/migration/state")]
    MigrationState {},
    #[route("/migration/fermi")]
    MigrationFermi {},
    #[route("/migration/props")]
    MigrationProps {},
    #[route("/contributing")]
    ContributingIndex {},
    #[route("/contributing/guiding_principles")]
    ContributingGuidingPrinciples {},
    #[route("/contributing/roadmap")]
    ContributingRoadmap {},
}
impl BookRoute {
    pub fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        &self.page().sections
    }
    pub fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        LAZY_BOOK.get_page(self)
    }
    pub fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        match self {
            BookRoute::Index {} => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::IntroductionRoadmap {} => {
                use_mdbook::mdbook_shared::PageId(1usize)
            }
            BookRoute::GuideIndex {} => use_mdbook::mdbook_shared::PageId(2usize),
            BookRoute::GuideTooling {} => use_mdbook::mdbook_shared::PageId(3usize),
            BookRoute::GuideNewApp {} => use_mdbook::mdbook_shared::PageId(4usize),
            BookRoute::GuideComponent {} => use_mdbook::mdbook_shared::PageId(5usize),
            BookRoute::GuideAssets {} => use_mdbook::mdbook_shared::PageId(6usize),
            BookRoute::GuideState {} => use_mdbook::mdbook_shared::PageId(7usize),
            BookRoute::GuideRouting {} => use_mdbook::mdbook_shared::PageId(8usize),
            BookRoute::GuideFetching {} => use_mdbook::mdbook_shared::PageId(9usize),
            BookRoute::GuideMultiplatform {} => {
                use_mdbook::mdbook_shared::PageId(10usize)
            }
            BookRoute::GuideBackend {} => use_mdbook::mdbook_shared::PageId(11usize),
            BookRoute::GuideDeploy {} => use_mdbook::mdbook_shared::PageId(12usize),
            BookRoute::GuideNextSteps {} => use_mdbook::mdbook_shared::PageId(13usize),
            BookRoute::EssentialsIndex {} => use_mdbook::mdbook_shared::PageId(14usize),
            BookRoute::EssentialsLifecycleIndex {} => {
                use_mdbook::mdbook_shared::PageId(15usize)
            }
            BookRoute::EssentialsStateIndex {} => {
                use_mdbook::mdbook_shared::PageId(16usize)
            }
            BookRoute::EssentialsBreakingIndex {} => {
                use_mdbook::mdbook_shared::PageId(17usize)
            }
            BookRoute::EssentialsStructureIndex {} => {
                use_mdbook::mdbook_shared::PageId(18usize)
            }
            BookRoute::EssentialsRsxIndex {} => {
                use_mdbook::mdbook_shared::PageId(19usize)
            }
            BookRoute::ReferenceIndex {} => use_mdbook::mdbook_shared::PageId(20usize),
            BookRoute::RouterIndex {} => use_mdbook::mdbook_shared::PageId(21usize),
            BookRoute::RouterReferenceRoutingUpdateCallback {} => {
                use_mdbook::mdbook_shared::PageId(22usize)
            }
            BookRoute::ReferenceAssets {} => use_mdbook::mdbook_shared::PageId(23usize),
            BookRoute::ReferenceWebIndex {} => use_mdbook::mdbook_shared::PageId(24usize),
            BookRoute::ReferenceDesktopIndex {} => {
                use_mdbook::mdbook_shared::PageId(25usize)
            }
            BookRoute::ReferenceMobileIndex {} => {
                use_mdbook::mdbook_shared::PageId(26usize)
            }
            BookRoute::ReferenceMobileApis {} => {
                use_mdbook::mdbook_shared::PageId(27usize)
            }
            BookRoute::ReferenceSsr {} => use_mdbook::mdbook_shared::PageId(28usize),
            BookRoute::ReferenceFullstackIndex {} => {
                use_mdbook::mdbook_shared::PageId(29usize)
            }
            BookRoute::ReferenceFullstackServerFunctions {} => {
                use_mdbook::mdbook_shared::PageId(30usize)
            }
            BookRoute::ReferenceFullstackExtractors {} => {
                use_mdbook::mdbook_shared::PageId(31usize)
            }
            BookRoute::ReferenceFullstackMiddleware {} => {
                use_mdbook::mdbook_shared::PageId(32usize)
            }
            BookRoute::ReferenceFullstackAuthentication {} => {
                use_mdbook::mdbook_shared::PageId(33usize)
            }
            BookRoute::ReferenceFullstackRouting {} => {
                use_mdbook::mdbook_shared::PageId(34usize)
            }
            BookRoute::CookbookPublishing {} => {
                use_mdbook::mdbook_shared::PageId(35usize)
            }
            BookRoute::CookbookAntipatterns {} => {
                use_mdbook::mdbook_shared::PageId(36usize)
            }
            BookRoute::CookbookErrorHandling {} => {
                use_mdbook::mdbook_shared::PageId(37usize)
            }
            BookRoute::CookbookIntegrationsIndex {} => {
                use_mdbook::mdbook_shared::PageId(38usize)
            }
            BookRoute::CookbookIntegrationsLogging {} => {
                use_mdbook::mdbook_shared::PageId(39usize)
            }
            BookRoute::CookbookIntegrationsInternationalization {} => {
                use_mdbook::mdbook_shared::PageId(40usize)
            }
            BookRoute::CookbookStateIndex {} => {
                use_mdbook::mdbook_shared::PageId(41usize)
            }
            BookRoute::CookbookStateExternalIndex {} => {
                use_mdbook::mdbook_shared::PageId(42usize)
            }
            BookRoute::CookbookStateCustomHooksIndex {} => {
                use_mdbook::mdbook_shared::PageId(43usize)
            }
            BookRoute::CookbookTesting {} => use_mdbook::mdbook_shared::PageId(44usize),
            BookRoute::CookbookExamples {} => use_mdbook::mdbook_shared::PageId(45usize),
            BookRoute::CookbookTailwind {} => use_mdbook::mdbook_shared::PageId(46usize),
            BookRoute::CookbookOptimizing {} => {
                use_mdbook::mdbook_shared::PageId(47usize)
            }
            BookRoute::MigrationIndex {} => use_mdbook::mdbook_shared::PageId(48usize),
            BookRoute::MigrationHooks {} => use_mdbook::mdbook_shared::PageId(49usize),
            BookRoute::MigrationState {} => use_mdbook::mdbook_shared::PageId(50usize),
            BookRoute::MigrationFermi {} => use_mdbook::mdbook_shared::PageId(51usize),
            BookRoute::MigrationProps {} => use_mdbook::mdbook_shared::PageId(52usize),
            BookRoute::ContributingIndex {} => use_mdbook::mdbook_shared::PageId(53usize),
            BookRoute::ContributingGuidingPrinciples {} => {
                use_mdbook::mdbook_shared::PageId(54usize)
            }
            BookRoute::ContributingRoadmap {} => {
                use_mdbook::mdbook_shared::PageId(55usize)
            }
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::Index {}
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> = use_mdbook::Lazy::new(||
{
    {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages
            .push((
                0usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Introduction".to_string(),
                        url: BookRoute::Index {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Introduction".to_string(),
                                id: "introduction".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Features".to_string(),
                                id: "features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Multiplatform".to_string(),
                                id: "multiplatform".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Stability".to_string(),
                                id: "stability".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(0usize),
                    }
                },
            ));
        page_id_mapping
            .insert(BookRoute::Index {}, ::use_mdbook::mdbook_shared::PageId(0usize));
        pages
            .push((
                1usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Roadmap".to_string(),
                        url: BookRoute::IntroductionRoadmap {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Roadmap".to_string(),
                                id: "roadmap".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Features".to_string(),
                                id: "features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Roadmap".to_string(),
                                id: "roadmap".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Core".to_string(),
                                id: "core".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "SSR".to_string(),
                                id: "ssr".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Desktop".to_string(),
                                id: "desktop".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Mobile".to_string(),
                                id: "mobile".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Bundling (CLI)".to_string(),
                                id: "bundling-(cli)".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Essential hooks".to_string(),
                                id: "essential-hooks".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Work in Progress".to_string(),
                                id: "work-in-progress".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Build Tool".to_string(),
                                id: "build-tool".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Server Component Support".to_string(),
                                id: "server-component-support".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Native rendering".to_string(),
                                id: "native-rendering".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(1usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::IntroductionRoadmap {},
                ::use_mdbook::mdbook_shared::PageId(1usize),
            );
        pages
            .push((
                2usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Tutorial".to_string(),
                        url: BookRoute::GuideIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus Guide".to_string(),
                                id: "dioxus-guide".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Introduction".to_string(),
                                id: "introduction".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(2usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideIndex {},
                ::use_mdbook::mdbook_shared::PageId(2usize),
            );
        pages
            .push((
                3usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Tooling setup".to_string(),
                        url: BookRoute::GuideTooling {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Setting up Tooling".to_string(),
                                id: "setting-up-tooling".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Prerequisites".to_string(),
                                id: "prerequisites".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "An Editor".to_string(),
                                id: "an-editor".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Rust".to_string(),
                                id: "rust".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Platform-specific dependencies".to_string(),
                                id: "platform-specific-dependencies".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus CLI".to_string(),
                                id: "dioxus-cli".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(3usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideTooling {},
                ::use_mdbook::mdbook_shared::PageId(3usize),
            );
        pages
            .push((
                4usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Creating a new app".to_string(),
                        url: BookRoute::GuideNewApp {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Create a new project".to_string(),
                                id: "create-a-new-project".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Running the project".to_string(),
                                id: "running-the-project".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conclusion".to_string(),
                                id: "conclusion".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(4usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideNewApp {},
                ::use_mdbook::mdbook_shared::PageId(4usize),
            );
        pages
            .push((
                5usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Your First Component".to_string(),
                        url: BookRoute::GuideComponent {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(5usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideComponent {},
                ::use_mdbook::mdbook_shared::PageId(5usize),
            );
        pages
            .push((
                6usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Styling and Assets".to_string(),
                        url: BookRoute::GuideAssets {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(6usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideAssets {},
                ::use_mdbook::mdbook_shared::PageId(6usize),
            );
        pages
            .push((
                7usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "State and Interactivity".to_string(),
                        url: BookRoute::GuideState {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Interactivity".to_string(),
                                id: "interactivity".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Creating a Preview".to_string(),
                                id: "creating-a-preview".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Event Handlers".to_string(),
                                id: "event-handlers".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "State".to_string(),
                                id: "state".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Rules of Hooks".to_string(),
                                id: "the-rules-of-hooks".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "No Hooks in Conditionals".to_string(),
                                id: "no-hooks-in-conditionals".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "No Hooks in Closures".to_string(),
                                id: "no-hooks-in-closures".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "No Hooks in Loops".to_string(),
                                id: "no-hooks-in-loops".to_string(),
                                level: 4usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(7usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideState {},
                ::use_mdbook::mdbook_shared::PageId(7usize),
            );
        pages
            .push((
                8usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "App Routing".to_string(),
                        url: BookRoute::GuideRouting {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(8usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideRouting {},
                ::use_mdbook::mdbook_shared::PageId(8usize),
            );
        pages
            .push((
                9usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Data Fetching".to_string(),
                        url: BookRoute::GuideFetching {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(9usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideFetching {},
                ::use_mdbook::mdbook_shared::PageId(9usize),
            );
        pages
            .push((
                10usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "More Platforms".to_string(),
                        url: BookRoute::GuideMultiplatform {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(10usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideMultiplatform {},
                ::use_mdbook::mdbook_shared::PageId(10usize),
            );
        pages
            .push((
                11usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Backend".to_string(),
                        url: BookRoute::GuideBackend {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(11usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideBackend {},
                ::use_mdbook::mdbook_shared::PageId(11usize),
            );
        pages
            .push((
                12usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Deploying".to_string(),
                        url: BookRoute::GuideDeploy {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(12usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideDeploy {},
                ::use_mdbook::mdbook_shared::PageId(12usize),
            );
        pages
            .push((
                13usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Next Steps".to_string(),
                        url: BookRoute::GuideNextSteps {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(13usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::GuideNextSteps {},
                ::use_mdbook::mdbook_shared::PageId(13usize),
            );
        pages
            .push((
                14usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Essential Concepts".to_string(),
                        url: BookRoute::EssentialsIndex {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(14usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::EssentialsIndex {},
                ::use_mdbook::mdbook_shared::PageId(14usize),
            );
        pages
            .push((
                15usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Component Lifecycle".to_string(),
                        url: BookRoute::EssentialsLifecycleIndex {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Component Lifecycle".to_string(),
                                id: "component-lifecycle".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Initializing State with ".to_string(),
                                id: "initializing-state-with".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Rerendering".to_string(),
                                id: "rerendering".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚠\u{fe0f} Don't mutate state in the body of a component"
                                    .to_string(),
                                id: "⚠\u{fe0f}-don't-mutate-state-in-the-body-of-a-component"
                                    .to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Using Effects".to_string(),
                                id: "using-effects".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Cleaning Up Components with Drop".to_string(),
                                id: "cleaning-up-components-with-drop".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(15usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::EssentialsLifecycleIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(15usize),
            );
        pages
            .push((
                16usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Managing State".to_string(),
                        url: BookRoute::EssentialsStateIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Managing State".to_string(),
                                id: "managing-state".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Creating State".to_string(),
                                id: "creating-state".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Reactive Scopes".to_string(),
                                id: "reactive-scopes".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Derived State".to_string(),
                                id: "derived-state".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Derived Async State".to_string(),
                                id: "derived-async-state".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Derived UI".to_string(),
                                id: "derived-ui".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Working with Untracked State".to_string(),
                                id: "working-with-untracked-state".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Making Props Reactive".to_string(),
                                id: "making-props-reactive".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Moving Around State".to_string(),
                                id: "moving-around-state".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Passing props".to_string(),
                                id: "passing-props".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Passing context".to_string(),
                                id: "passing-context".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Using globals".to_string(),
                                id: "using-globals".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(16usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::EssentialsStateIndex {},
                ::use_mdbook::mdbook_shared::PageId(16usize),
            );
        pages
            .push((
                17usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Breaking Out".to_string(),
                        url: BookRoute::EssentialsBreakingIndex {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Breaking Out of Dioxus".to_string(),
                                id: "breaking-out-of-dioxus".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Interacting with JavaScript with ".to_string(),
                                id: "interacting-with-javascript-with".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Synchronizing DOM updates with ".to_string(),
                                id: "synchronizing-dom-updates-with".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Getting access to elements with ".to_string(),
                                id: "getting-access-to-elements-with".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Down casting web sys events".to_string(),
                                id: "down-casting-web-sys-events".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(17usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::EssentialsBreakingIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(17usize),
            );
        pages
            .push((
                18usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Structuring Your App".to_string(),
                        url: BookRoute::EssentialsStructureIndex {
                        },
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(18usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::EssentialsStructureIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(18usize),
            );
        pages
            .push((
                19usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Building UIs with RSX".to_string(),
                        url: BookRoute::EssentialsRsxIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Building UIs with RSX".to_string(),
                                id: "building-uis-with-rsx".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Text Nodes".to_string(),
                                id: "text-nodes".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Elements".to_string(),
                                id: "elements".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Attributes".to_string(),
                                id: "attributes".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conditional Attributes".to_string(),
                                id: "conditional-attributes".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Event Listeners".to_string(),
                                id: "event-listeners".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Children".to_string(),
                                id: "children".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Loops".to_string(),
                                id: "loops".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "If Statements".to_string(),
                                id: "if-statements".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(19usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::EssentialsRsxIndex {},
                ::use_mdbook::mdbook_shared::PageId(19usize),
            );
        pages
            .push((
                20usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Guides".to_string(),
                        url: BookRoute::ReferenceIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus Reference".to_string(),
                                id: "dioxus-reference".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Rendering".to_string(),
                                id: "rendering".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "State".to_string(),
                                id: "state".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Platforms".to_string(),
                                id: "platforms".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(20usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceIndex {},
                ::use_mdbook::mdbook_shared::PageId(20usize),
            );
        pages
            .push((
                21usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Router".to_string(),
                        url: BookRoute::RouterIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Introduction".to_string(),
                                id: "introduction".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(21usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::RouterIndex {},
                ::use_mdbook::mdbook_shared::PageId(21usize),
            );
        pages
            .push((
                22usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Routing Update Callback".to_string(),
                        url: BookRoute::RouterReferenceRoutingUpdateCallback {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Routing Update Callback".to_string(),
                                id: "routing-update-callback".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How does the callback behave?".to_string(),
                                id: "how-does-the-callback-behave?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Code Example".to_string(),
                                id: "code-example".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(22usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::RouterReferenceRoutingUpdateCallback {
                },
                ::use_mdbook::mdbook_shared::PageId(22usize),
            );
        pages
            .push((
                23usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Assets".to_string(),
                        url: BookRoute::ReferenceAssets {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Assets".to_string(),
                                id: "assets".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Including images".to_string(),
                                id: "including-images".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Including arbitrary files".to_string(),
                                id: "including-arbitrary-files".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Including stylesheets".to_string(),
                                id: "including-stylesheets".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conclusion".to_string(),
                                id: "conclusion".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(23usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceAssets {},
                ::use_mdbook::mdbook_shared::PageId(23usize),
            );
        pages
            .push((
                24usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Web".to_string(),
                        url: BookRoute::ReferenceWebIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Web".to_string(),
                                id: "web".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Support".to_string(),
                                id: "support".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Running Javascript".to_string(),
                                id: "running-javascript".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Customizing Index Template".to_string(),
                                id: "customizing-index-template".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(24usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceWebIndex {},
                ::use_mdbook::mdbook_shared::PageId(24usize),
            );
        pages
            .push((
                25usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Desktop".to_string(),
                        url: BookRoute::ReferenceDesktopIndex {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Desktop".to_string(),
                                id: "desktop".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Examples".to_string(),
                                id: "examples".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Running Javascript".to_string(),
                                id: "running-javascript".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Custom Assets".to_string(),
                                id: "custom-assets".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Integrating with Wry".to_string(),
                                id: "integrating-with-wry".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(25usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceDesktopIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(25usize),
            );
        pages
            .push((
                26usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Mobile".to_string(),
                        url: BookRoute::ReferenceMobileIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Mobile App".to_string(),
                                id: "mobile-app".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Support".to_string(),
                                id: "support".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Getting Set up".to_string(),
                                id: "getting-set-up".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Setting up dependencies".to_string(),
                                id: "setting-up-dependencies".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Android".to_string(),
                                id: "android".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "IOS".to_string(),
                                id: "ios".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Setting up your project".to_string(),
                                id: "setting-up-your-project".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Running".to_string(),
                                id: "running".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Android".to_string(),
                                id: "android".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "IOS".to_string(),
                                id: "ios".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(26usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceMobileIndex {},
                ::use_mdbook::mdbook_shared::PageId(26usize),
            );
        pages
            .push((
                27usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "APIs".to_string(),
                        url: BookRoute::ReferenceMobileApis {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Mobile".to_string(),
                                id: "mobile".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Running Javascript".to_string(),
                                id: "running-javascript".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Custom Assets".to_string(),
                                id: "custom-assets".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Integrating with Wry".to_string(),
                                id: "integrating-with-wry".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(27usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceMobileApis {},
                ::use_mdbook::mdbook_shared::PageId(27usize),
            );
        pages
            .push((
                28usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Streaming and SSR".to_string(),
                        url: BookRoute::ReferenceSsr {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Server-Side Rendering".to_string(),
                                id: "server-side-rendering".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Setup".to_string(),
                                id: "setup".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Multithreaded Support".to_string(),
                                id: "multithreaded-support".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(28usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceSsr {},
                ::use_mdbook::mdbook_shared::PageId(28usize),
            );
        pages
            .push((
                29usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Fullstack".to_string(),
                        url: BookRoute::ReferenceFullstackIndex {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fullstack development".to_string(),
                                id: "fullstack-development".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(29usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceFullstackIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(29usize),
            );
        pages
            .push((
                30usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Server Functions".to_string(),
                        url: BookRoute::ReferenceFullstackServerFunctions {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Communicating with the server".to_string(),
                                id: "communicating-with-the-server".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Cached data fetching".to_string(),
                                id: "cached-data-fetching".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Running the client with dioxus-desktop".to_string(),
                                id: "running-the-client-with-dioxus-desktop".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Client code".to_string(),
                                id: "client-code".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Server code".to_string(),
                                id: "server-code".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(30usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceFullstackServerFunctions {
                },
                ::use_mdbook::mdbook_shared::PageId(30usize),
            );
        pages
            .push((
                31usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Extractors".to_string(),
                        url: BookRoute::ReferenceFullstackExtractors {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Extractors".to_string(),
                                id: "extractors".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(31usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceFullstackExtractors {
                },
                ::use_mdbook::mdbook_shared::PageId(31usize),
            );
        pages
            .push((
                32usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Middleware".to_string(),
                        url: BookRoute::ReferenceFullstackMiddleware {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Middleware".to_string(),
                                id: "middleware".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(32usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceFullstackMiddleware {
                },
                ::use_mdbook::mdbook_shared::PageId(32usize),
            );
        pages
            .push((
                33usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Authentication".to_string(),
                        url: BookRoute::ReferenceFullstackAuthentication {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Authentication".to_string(),
                                id: "authentication".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(33usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceFullstackAuthentication {
                },
                ::use_mdbook::mdbook_shared::PageId(33usize),
            );
        pages
            .push((
                34usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Routing".to_string(),
                        url: BookRoute::ReferenceFullstackRouting {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Routing".to_string(),
                                id: "routing".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(34usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ReferenceFullstackRouting {
                },
                ::use_mdbook::mdbook_shared::PageId(34usize),
            );
        pages
            .push((
                35usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Publishing".to_string(),
                        url: BookRoute::CookbookPublishing {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Publishing".to_string(),
                                id: "publishing".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Web: Publishing with GitHub Pages".to_string(),
                                id: "web:-publishing-with-github-pages".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Desktop: Creating an installer".to_string(),
                                id: "desktop:-creating-an-installer".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Preparing your application for bundling"
                                    .to_string(),
                                id: "preparing-your-application-for-bundling".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Adding assets to your application".to_string(),
                                id: "adding-assets-to-your-application".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Install ".to_string(),
                                id: "install".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Building".to_string(),
                                id: "building".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(35usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookPublishing {},
                ::use_mdbook::mdbook_shared::PageId(35usize),
            );
        pages
            .push((
                36usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Anti-patterns".to_string(),
                        url: BookRoute::CookbookAntipatterns {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Antipatterns".to_string(),
                                id: "antipatterns".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Unnecessarily Nested Fragments".to_string(),
                                id: "unnecessarily-nested-fragments".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Incorrect Iterator Keys".to_string(),
                                id: "incorrect-iterator-keys".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Avoid Interior Mutability in Props".to_string(),
                                id: "avoid-interior-mutability-in-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Avoid Updating State During Render".to_string(),
                                id: "avoid-updating-state-during-render".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Avoid Large Groups of State".to_string(),
                                id: "avoid-large-groups-of-state".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Running Non-Deterministic Code in the Body of a Component"
                                    .to_string(),
                                id: "running-non-deterministic-code-in-the-body-of-a-component"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Overly Permissive PartialEq for Props".to_string(),
                                id: "overly-permissive-partialeq-for-props".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(36usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookAntipatterns {},
                ::use_mdbook::mdbook_shared::PageId(36usize),
            );
        pages
            .push((
                37usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Error Handling".to_string(),
                        url: BookRoute::CookbookErrorHandling {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Error handling".to_string(),
                                id: "error-handling".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The simplest – returning None".to_string(),
                                id: "the-simplest-–-returning-none".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Early return on result".to_string(),
                                id: "early-return-on-result".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Match results".to_string(),
                                id: "match-results".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Passing error states through components"
                                    .to_string(),
                                id: "passing-error-states-through-components".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Throwing errors".to_string(),
                                id: "throwing-errors".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(37usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookErrorHandling {
                },
                ::use_mdbook::mdbook_shared::PageId(37usize),
            );
        pages
            .push((
                38usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Integrations".to_string(),
                        url: BookRoute::CookbookIntegrationsIndex {
                        },
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(38usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookIntegrationsIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(38usize),
            );
        pages
            .push((
                39usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Logging".to_string(),
                        url: BookRoute::CookbookIntegrationsLogging {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Logging".to_string(),
                                id: "logging".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Tracing Crate".to_string(),
                                id: "the-tracing-crate".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dioxus Logger".to_string(),
                                id: "dioxus-logger".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Platform Intricacies".to_string(),
                                id: "platform-intricacies".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Final Notes".to_string(),
                                id: "final-notes".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Desktop and Server".to_string(),
                                id: "desktop-and-server".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Web".to_string(),
                                id: "web".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Mobile".to_string(),
                                id: "mobile".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Android".to_string(),
                                id: "android".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Viewing Logs".to_string(),
                                id: "viewing-logs".to_string(),
                                level: 4usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "iOS".to_string(),
                                id: "ios".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Viewing Logs".to_string(),
                                id: "viewing-logs".to_string(),
                                level: 4usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(39usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookIntegrationsLogging {
                },
                ::use_mdbook::mdbook_shared::PageId(39usize),
            );
        pages
            .push((
                40usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Internationalization".to_string(),
                        url: BookRoute::CookbookIntegrationsInternationalization {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Internationalization".to_string(),
                                id: "internationalization".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The full code for internationalization".to_string(),
                                id: "the-full-code-for-internationalization".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(40usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookIntegrationsInternationalization {
                },
                ::use_mdbook::mdbook_shared::PageId(40usize),
            );
        pages
            .push((
                41usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "State Management".to_string(),
                        url: BookRoute::CookbookStateIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "State Cookbook".to_string(),
                                id: "state-cookbook".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(41usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookStateIndex {},
                ::use_mdbook::mdbook_shared::PageId(41usize),
            );
        pages
            .push((
                42usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "External State".to_string(),
                        url: BookRoute::CookbookStateExternalIndex {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Working with External State".to_string(),
                                id: "working-with-external-state".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Working with non-reactive State".to_string(),
                                id: "working-with-non-reactive-state".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Making Reactive State External".to_string(),
                                id: "making-reactive-state-external".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(42usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookStateExternalIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(42usize),
            );
        pages
            .push((
                43usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Custom Hooks".to_string(),
                        url: BookRoute::CookbookStateCustomHooksIndex {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Custom Hooks".to_string(),
                                id: "custom-hooks".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Composing Hooks".to_string(),
                                id: "composing-hooks".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Custom Hook Logic".to_string(),
                                id: "custom-hook-logic".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(43usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookStateCustomHooksIndex {
                },
                ::use_mdbook::mdbook_shared::PageId(43usize),
            );
        pages
            .push((
                44usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Testing".to_string(),
                        url: BookRoute::CookbookTesting {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Testing".to_string(),
                                id: "testing".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Component Testing".to_string(),
                                id: "component-testing".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hook Testing".to_string(),
                                id: "hook-testing".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "End to End Testing".to_string(),
                                id: "end-to-end-testing".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(44usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookTesting {},
                ::use_mdbook::mdbook_shared::PageId(44usize),
            );
        pages
            .push((
                45usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Examples".to_string(),
                        url: BookRoute::CookbookExamples {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Examples".to_string(),
                                id: "examples".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(45usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookExamples {},
                ::use_mdbook::mdbook_shared::PageId(45usize),
            );
        pages
            .push((
                46usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Tailwind".to_string(),
                        url: BookRoute::CookbookTailwind {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Tailwind".to_string(),
                                id: "tailwind".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Setup".to_string(),
                                id: "setup".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Bonus Steps".to_string(),
                                id: "bonus-steps".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Development".to_string(),
                                id: "development".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Web".to_string(),
                                id: "web".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Desktop".to_string(),
                                id: "desktop".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(46usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookTailwind {},
                ::use_mdbook::mdbook_shared::PageId(46usize),
            );
        pages
            .push((
                47usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Optimizing".to_string(),
                        url: BookRoute::CookbookOptimizing {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Optimizing".to_string(),
                                id: "optimizing".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Building in release mode".to_string(),
                                id: "building-in-release-mode".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "UPX".to_string(),
                                id: "upx".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Build configuration".to_string(),
                                id: "build-configuration".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Stable".to_string(),
                                id: "stable".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Unstable".to_string(),
                                id: "unstable".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "wasm-opt".to_string(),
                                id: "wasm-opt".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Improving Dioxus code".to_string(),
                                id: "improving-dioxus-code".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Optimizing the size of assets".to_string(),
                                id: "optimizing-the-size-of-assets".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(47usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::CookbookOptimizing {},
                ::use_mdbook::mdbook_shared::PageId(47usize),
            );
        pages
            .push((
                48usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Migration for 0.5".to_string(),
                        url: BookRoute::MigrationIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How to Upgrade to Dioxus 0.5".to_string(),
                                id: "how-to-upgrade-to-dioxus-0.5".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Cheat Sheet".to_string(),
                                id: "cheat-sheet".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Scope".to_string(),
                                id: "scope".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Props".to_string(),
                                id: "props".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Futures".to_string(),
                                id: "futures".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "State Hooks".to_string(),
                                id: "state-hooks".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fermi".to_string(),
                                id: "fermi".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(48usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::MigrationIndex {},
                ::use_mdbook::mdbook_shared::PageId(48usize),
            );
        pages
            .push((
                49usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Hooks".to_string(),
                        url: BookRoute::MigrationHooks {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hooks".to_string(),
                                id: "hooks".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "State Hooks".to_string(),
                                id: "state-hooks".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Async Hooks".to_string(),
                                id: "async-hooks".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Dependencies".to_string(),
                                id: "dependencies".to_string(),
                                level: 3usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(49usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::MigrationHooks {},
                ::use_mdbook::mdbook_shared::PageId(49usize),
            );
        pages
            .push((
                50usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "State".to_string(),
                        url: BookRoute::MigrationState {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "State Migration".to_string(),
                                id: "state-migration".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Context Based State".to_string(),
                                id: "context-based-state".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Opting Out of Subscriptions".to_string(),
                                id: "opting-out-of-subscriptions".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Global State".to_string(),
                                id: "global-state".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(50usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::MigrationState {},
                ::use_mdbook::mdbook_shared::PageId(50usize),
            );
        pages
            .push((
                51usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Fermi".to_string(),
                        url: BookRoute::MigrationFermi {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fermi".to_string(),
                                id: "fermi".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Memos".to_string(),
                                id: "memos".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(51usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::MigrationFermi {},
                ::use_mdbook::mdbook_shared::PageId(51usize),
            );
        pages
            .push((
                52usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Props".to_string(),
                        url: BookRoute::MigrationProps {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Props Migration".to_string(),
                                id: "props-migration".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Owned Props".to_string(),
                                id: "owned-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Borrowed Props".to_string(),
                                id: "borrowed-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Manual Props".to_string(),
                                id: "manual-props".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(52usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::MigrationProps {},
                ::use_mdbook::mdbook_shared::PageId(52usize),
            );
        pages
            .push((
                53usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Contributing".to_string(),
                        url: BookRoute::ContributingIndex {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Contributing".to_string(),
                                id: "contributing".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Improving Docs".to_string(),
                                id: "improving-docs".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Working on the Ecosystem".to_string(),
                                id: "working-on-the-ecosystem".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Bugs & Features".to_string(),
                                id: "bugs-&-features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Before you contribute".to_string(),
                                id: "before-you-contribute".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How to test dioxus with local crate".to_string(),
                                id: "how-to-test-dioxus-with-local-crate".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(53usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ContributingIndex {},
                ::use_mdbook::mdbook_shared::PageId(53usize),
            );
        pages
            .push((
                54usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Guiding Principles".to_string(),
                        url: BookRoute::ContributingGuidingPrinciples {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Overall Goals".to_string(),
                                id: "overall-goals".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Cross-Platform".to_string(),
                                id: "cross-platform".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Performance".to_string(),
                                id: "performance".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Type Safety".to_string(),
                                id: "type-safety".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Developer Experience".to_string(),
                                id: "developer-experience".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(54usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ContributingGuidingPrinciples {
                },
                ::use_mdbook::mdbook_shared::PageId(54usize),
            );
        pages
            .push((
                55usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: "Roadmap".to_string(),
                        url: BookRoute::ContributingRoadmap {},
                        segments: vec![],
                        sections: vec![],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(55usize),
                    }
                },
            ));
        page_id_mapping
            .insert(
                BookRoute::ContributingRoadmap {},
                ::use_mdbook::mdbook_shared::PageId(55usize),
            );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Introduction".to_string(),
                        location: Some(BookRoute::Index {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Roadmap".to_string(),
                                location: Some(BookRoute::IntroductionRoadmap {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Tutorial".to_string(),
                        location: Some(BookRoute::GuideIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Tooling setup".to_string(),
                                location: Some(BookRoute::GuideTooling {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Creating a new app".to_string(),
                                location: Some(BookRoute::GuideNewApp {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Your First Component".to_string(),
                                location: Some(BookRoute::GuideComponent {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Styling and Assets".to_string(),
                                location: Some(BookRoute::GuideAssets {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "State and Interactivity".to_string(),
                                location: Some(BookRoute::GuideState {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 5u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "App Routing".to_string(),
                                location: Some(BookRoute::GuideRouting {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 6u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Data Fetching".to_string(),
                                location: Some(BookRoute::GuideFetching {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 7u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "More Platforms".to_string(),
                                location: Some(BookRoute::GuideMultiplatform {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 8u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Backend".to_string(),
                                location: Some(BookRoute::GuideBackend {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32, 9u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Deploying".to_string(),
                                location: Some(BookRoute::GuideDeploy {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![2u32, 10u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Next Steps".to_string(),
                                location: Some(BookRoute::GuideNextSteps {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![2u32, 11u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Essential Concepts".to_string(),
                        location: Some(BookRoute::EssentialsIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Component Lifecycle".to_string(),
                                location: Some(BookRoute::EssentialsLifecycleIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Managing State".to_string(),
                                location: Some(BookRoute::EssentialsStateIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Breaking Out".to_string(),
                                location: Some(BookRoute::EssentialsBreakingIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Structuring Your App".to_string(),
                                location: Some(BookRoute::EssentialsStructureIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Building UIs with RSX".to_string(),
                                location: Some(BookRoute::EssentialsRsxIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32, 5u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Guides".to_string(),
                        location: Some(BookRoute::ReferenceIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Router".to_string(),
                                location: Some(BookRoute::RouterIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 1u32]),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Routing Update Callback".to_string(),
                                        location: Some(BookRoute::RouterReferenceRoutingUpdateCallback {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 1u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Assets".to_string(),
                                location: Some(BookRoute::ReferenceAssets {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Web".to_string(),
                                location: Some(BookRoute::ReferenceWebIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 3u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Desktop".to_string(),
                                location: Some(BookRoute::ReferenceDesktopIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 4u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Mobile".to_string(),
                                location: Some(BookRoute::ReferenceMobileIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 5u32]),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "APIs".to_string(),
                                        location: Some(BookRoute::ReferenceMobileApis {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 5u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Streaming and SSR".to_string(),
                                location: Some(BookRoute::ReferenceSsr {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 6u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Fullstack".to_string(),
                                location: Some(BookRoute::ReferenceFullstackIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 7u32]),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Server Functions".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackServerFunctions {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Extractors".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackExtractors {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Middleware".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackMiddleware {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 3u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Authentication".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackAuthentication {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 4u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Routing".to_string(),
                                        location: Some(BookRoute::ReferenceFullstackRouting {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 7u32, 5u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Publishing".to_string(),
                                location: Some(BookRoute::CookbookPublishing {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 8u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Anti-patterns".to_string(),
                                location: Some(BookRoute::CookbookAntipatterns {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32, 9u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Error Handling".to_string(),
                                location: Some(BookRoute::CookbookErrorHandling {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 10u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Integrations".to_string(),
                                location: Some(BookRoute::CookbookIntegrationsIndex {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 11u32],
                                    ),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Logging".to_string(),
                                        location: Some(BookRoute::CookbookIntegrationsLogging {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 11u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Internationalization".to_string(),
                                        location: Some(BookRoute::CookbookIntegrationsInternationalization {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 11u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "State Management".to_string(),
                                location: Some(BookRoute::CookbookStateIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 12u32],
                                    ),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "External State".to_string(),
                                        location: Some(BookRoute::CookbookStateExternalIndex {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 12u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Custom Hooks".to_string(),
                                        location: Some(BookRoute::CookbookStateCustomHooksIndex {
                                        }),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 12u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Testing".to_string(),
                                location: Some(BookRoute::CookbookTesting {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 13u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Examples".to_string(),
                                location: Some(BookRoute::CookbookExamples {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 14u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Tailwind".to_string(),
                                location: Some(BookRoute::CookbookTailwind {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 15u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Optimizing".to_string(),
                                location: Some(BookRoute::CookbookOptimizing {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 16u32],
                                    ),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Migration for 0.5".to_string(),
                                location: Some(BookRoute::MigrationIndex {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(
                                        vec![4u32, 17u32],
                                    ),
                                ),
                                nested_items: vec![
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Hooks".to_string(),
                                        location: Some(BookRoute::MigrationHooks {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 17u32, 1u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "State".to_string(),
                                        location: Some(BookRoute::MigrationState {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 17u32, 2u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Fermi".to_string(),
                                        location: Some(BookRoute::MigrationFermi {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 17u32, 3u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                        name: "Props".to_string(),
                                        location: Some(BookRoute::MigrationProps {}),
                                        number: Some(
                                            ::use_mdbook::mdbook_shared::SectionNumber(
                                                vec![4u32, 17u32, 4u32],
                                            ),
                                        ),
                                        nested_items: vec![],
                                    }),
                                ],
                            }),
                        ],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Separator,
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Contributing".to_string(),
                        location: Some(BookRoute::ContributingIndex {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Guiding Principles".to_string(),
                                location: Some(BookRoute::ContributingGuidingPrinciples {
                                }),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32, 1u32]),
                                ),
                                nested_items: vec![],
                            }),
                            ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                                name: "Roadmap".to_string(),
                                location: Some(BookRoute::ContributingRoadmap {}),
                                number: Some(
                                    ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32, 2u32]),
                                ),
                                nested_items: vec![],
                            }),
                        ],
                    }),
                ],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    }
});
#[component(no_case_check)]
pub fn Index() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "introduction",
            a { href: "#introduction", class: "header", "Introduction" }
        }
        p {
            "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. This guide will help you get started with writing Dioxus apps for the Web, Desktop, Mobile, and more."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Up high!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Down low!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "readme.rs".to_string(),
        }
        DemoFrame { readme::App {} }
        p {
            "Dioxus is heavily inspired by React. If you know React, getting started with Dioxus will be a breeze."
        }
        blockquote {
            p {
                "This guide assumes you already know some "
                a { href: "https://www.rust-lang.org/", "Rust" }
                "! If not, we recommend reading "
                a { href: "https://doc.rust-lang.org/book/ch01-00-getting-started.html",
                    em { "the book" }
                }
                " to learn Rust first."
            }
        }
        h2 { id: "features",
            a { href: "#features", class: "header", "Features" }
        }
        ul {
            li { "Cross platform apps in three lines of code. (Web, Desktop, Server, Mobile, and more)" }
            li {
                "Incredibly ergonomic and powerful state management that combines the best parts of react, solid and svelte."
            }
            li {
                "Comprehensive inline documentation – hover and guides for all HTML elements, listeners, and events."
            }
            li {
                "High performance applications "
                a { href: "https://dioxuslabs.com/blog/templates-diffing",
                    "approaching the fastest web frameworks on the web"
                }
                " and native speeds on desktop."
            }
            li { "First-class async support." }
        }
        h3 { id: "multiplatform",
            a { href: "#multiplatform", class: "header", "Multiplatform" }
        }
        p {
            "Dioxus is a "
            em { "portable" }
            " toolkit, meaning the Core implementation can run anywhere with no platform-dependent linking. Unlike many other Rust frontend toolkits, Dioxus is not intrinsically linked to WebSys. In fact, every element and event listener can be swapped out at compile time. By default, Dioxus ships with the "
            code { "html" }
            " feature enabled, but this can be disabled depending on your target renderer."
        }
        p { "Right now, we have several 1st-party renderers:" }
        ul {
            li { "WebSys/Sledgehammer (for WASM): Great support" }
            li { "Tao/Tokio (for Desktop apps): Good support" }
            li { "Tao/Tokio (for Mobile apps): Poor support" }
            li { "Fullstack (for SSR and server functions): Good support" }
            li { "TUI/Plasmo (for terminal-based apps): Experimental" }
        }
        h2 { id: "stability",
            a { href: "#stability", class: "header", "Stability" }
        }
        p { "Dioxus has not reached a stable release yet." }
        p {
            "Web: Since the web is a fairly mature platform, we expect there to be very little API churn for web-based features."
        }
        p {
            "Desktop: APIs will likely be in flux as we figure out better patterns than our ElectronJS counterpart."
        }
        p {
            "Fullstack: APIs will likely be in flux as we figure out the best API for server communication."
        }
    }
}
#[component(no_case_check)]
pub fn IntroductionRoadmap() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "roadmap",
            a { href: "#roadmap", class: "header", "Roadmap" }
        }
        p {
            "This feature set and roadmap can help you decide if what Dioxus can do today works for you."
        }
        p {
            "If a feature that you need doesn't exist or you want to contribute to projects on the roadmap, feel free to get involved by "
            a { href: "https://discord.gg/XgGxMSkvUM", "joining the discord" }
            "."
        }
        p { "Generally, here's the status of each platform:" }
        ul {
            li {
                p {
                    strong { "Web" }
                    ": Dioxus is a great choice for pure web-apps – especially for CRUD/complex apps. However, it does lack the ecosystem of React, so you might be missing a component library or some useful hook."
                }
            }
            li {
                p {
                    strong { "SSR" }
                    ": Dioxus is a great choice for pre-rendering, hydration, and rendering HTML on a web endpoint. Be warned – the VirtualDom is not (currently) "
                    code { "Send + Sync" }
                    "."
                }
            }
            li {
                p {
                    strong { "Desktop" }
                    ": You can build very competent single-window desktop apps right now. However, multi-window apps require support from Dioxus core and are not ready."
                }
            }
            li {
                p {
                    strong { "Mobile" }
                    ": Mobile support is very young. You'll be figuring things out as you go and there are not many support crates for peripherals."
                }
            }
            li {
                p {
                    strong { "LiveView" }
                    ": LiveView support is very young. You'll be figuring things out as you go. Thankfully, none of it is too hard and any work can be upstreamed into Dioxus."
                }
            }
        }
        h2 { id: "features",
            a { href: "#features", class: "header", "Features" }
        }
        hr {}
        table {
            thead {
                th { "Feature" }
                th { "Status" }
                th { "Description" }
            }
            tr {
                th { "Conditional Rendering" }
                th { "x" }
                th { "if/then to hide/show component" }
            }
            tr {
                th { "Map, Iterator" }
                th { "x" }
                th { "map/filter/reduce to produce rsx!" }
            }
            tr {
                th { "Keyed Components" }
                th { "x" }
                th { "advanced diffing with keys" }
            }
            tr {
                th { "Web" }
                th { "x" }
                th { "renderer for web browser" }
            }
            tr {
                th { "Desktop (webview)" }
                th { "x" }
                th { "renderer for desktop" }
            }
            tr {
                th { "Shared State (Context)" }
                th { "x" }
                th { "share state through the tree" }
            }
            tr {
                th { "Hooks" }
                th { "x" }
                th { "memory cells in components" }
            }
            tr {
                th { "SSR" }
                th { "x" }
                th { "render directly to string" }
            }
            tr {
                th { "Component Children" }
                th { "x" }
                th { "cx.children() as a list of nodes" }
            }
            tr {
                th { "Headless components" }
                th { "x" }
                th { "components that don't return real elements" }
            }
            tr {
                th { "Fragments" }
                th { "x" }
                th { "multiple elements without a real root" }
            }
            tr {
                th { "Manual Props" }
                th { "x" }
                th { "Manually pass in props with spread syntax" }
            }
            tr {
                th { "Controlled Inputs" }
                th { "x" }
                th { "stateful wrappers around inputs" }
            }
            tr {
                th { "CSS/Inline Styles" }
                th { "x" }
                th { "syntax for inline styles/attribute groups" }
            }
            tr {
                th { "Custom elements" }
                th { "x" }
                th { "Define new element primitives" }
            }
            tr {
                th { "Suspense" }
                th { "x" }
                th { "schedule future render from future/promise" }
            }
            tr {
                th { "Integrated error handling" }
                th { "x" }
                th { "Gracefully handle errors with ? syntax" }
            }
            tr {
                th { "NodeRef" }
                th { "x" }
                th { "gain direct access to nodes" }
            }
            tr {
                th { "Re-hydration" }
                th { "x" }
                th { "Pre-render to HTML to speed up first contentful paint" }
            }
            tr {
                th { "Jank-Free Rendering" }
                th { "x" }
                th { "Large diffs are segmented across frames for silky-smooth transitions" }
            }
            tr {
                th { "Effects" }
                th { "x" }
                th { "Run effects after a component has been committed to render" }
            }
            tr {
                th { "Portals" }
                th { "*" }
                th { "Render nodes outside of the traditional tree structure" }
            }
            tr {
                th { "Cooperative Scheduling" }
                th { "*" }
                th { "Prioritize important events over non-important events" }
            }
            tr {
                th { "Server Components" }
                th { "*" }
                th { "Hybrid components for SPA and Server" }
            }
            tr {
                th { "Bundle Splitting" }
                th { "i" }
                th { "Efficiently and asynchronously load the app" }
            }
            tr {
                th { "Lazy Components" }
                th { "i" }
                th { "Dynamically load the new components as the page is loaded" }
            }
            tr {
                th { "1st class global state" }
                th { "x" }
                th { "redux/recoil/mobx on top of context" }
            }
            tr {
                th { "Runs natively" }
                th { "x" }
                th { "runs as a portable binary w/o a runtime (Node)" }
            }
            tr {
                th { "Subtree Memoization" }
                th { "x" }
                th { "skip diffing static element subtrees" }
            }
            tr {
                th { "High-efficiency templates" }
                th { "x" }
                th { "rsx! calls are translated to templates on the DOM's side" }
            }
            tr {
                th { "Compile-time correct" }
                th { "x" }
                th { "Throw errors on invalid template layouts" }
            }
            tr {
                th { "Heuristic Engine" }
                th { "x" }
                th { "track component memory usage to minimize future allocations" }
            }
            tr {
                th { "Fine-grained reactivity" }
                th { "i" }
                th { "Skip diffing for fine-grain updates" }
            }
        }
        ul {
            li { "x = implemented and working" }
            li { "* = actively being worked on" }
            li { "i = not yet implemented or being worked on" }
        }
        h2 { id: "roadmap",
            a { href: "#roadmap", class: "header", "Roadmap" }
        }
        p { "These Features are planned for the future of Dioxus:" }
        h3 { id: "core",
            a { href: "#core", class: "header", "Core" }
        }
        ul {
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
        }
        h3 { id: "ssr",
            a { href: "#ssr", class: "header", "SSR" }
        }
        ul {
            li { "[" }
            li { "[" }
        }
        h3 { id: "desktop",
            a { href: "#desktop", class: "header", "Desktop" }
        }
        ul {
            li { "[" }
            li { "[" }
            li { "[" }
        }
        h3 { id: "mobile",
            a { href: "#mobile", class: "header", "Mobile" }
        }
        ul {
            li {
                "["
                ul {
                    li { "[" }
                    li { "[" }
                    li { "[" }
                    li { "[" }
                    li { "[" }
                    li { "[" }
                    li { "[" }
                    li { "[" }
                }
            }
            li { "[" }
        }
        h3 { id: "bundling-cli",
            a { href: "#bundling-cli", class: "header", "Bundling (CLI)" }
        }
        ul {
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
            li { "[" }
        }
        h3 { id: "essential-hooks",
            a { href: "#essential-hooks", class: "header", "Essential hooks" }
        }
        ul {
            li { "[" }
            li { "[" }
            li { "[" }
        }
        h2 { id: "work-in-progress",
            a { href: "#work-in-progress", class: "header", "Work in Progress" }
        }
        h3 { id: "build-tool",
            a { href: "#build-tool", class: "header", "Build Tool" }
        }
        p {
            "We are currently working on our own build tool called "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
                "Dioxus CLI"
            }
            " which will support:"
        }
        ul {
            li { "an interactive TUI" }
            li { "on-the-fly reconfiguration" }
            li { "hot CSS reloading" }
            li { "two-way data binding between browser and source code" }
            li {
                "an interpreter for "
                code { "rsx!" }
            }
            li { "ability to publish to github/netlify/vercel" }
            li { "bundling for iOS/Desktop/etc" }
        }
        h3 { id: "server-component-support",
            a { href: "#server-component-support", class: "header", "Server Component Support" }
        }
        p {
            "While not currently fully implemented, the expectation is that LiveView apps can be a hybrid between Wasm and server-rendered where only portions of a page are \"live\" and the rest of the page is either server-rendered, statically generated, or handled by the host SPA."
        }
        h3 { id: "native-rendering",
            a { href: "#native-rendering", class: "header", "Native rendering" }
        }
        p {
            "We are currently working on a native renderer for Dioxus using WGPU called "
            a { href: "https://github.com/DioxusLabs/blitz/", "Blitz" }
            ". This will allow you to build apps that are rendered natively for iOS, Android, and Desktop."
        }
    }
}
#[component(no_case_check)]
pub fn GuideIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "dioxus-guide",
            a { href: "#dioxus-guide", class: "header", "Dioxus Guide" }
        }
        h2 { id: "introduction",
            a { href: "#introduction", class: "header", "Introduction" }
        }
        p {
            "In this guide, you'll learn to use Dioxus to build user interfaces that run anywhere. We will recreate the hackernews homepage in Dioxus:"
        }
        DemoFrame { hackernews_complete::App {} }
        p {
            "This guide serves a very brief overview of Dioxus. Throughout the guide, there will be links to the "
            a { href: "guide/../reference", "reference" }
            " with more details about specific concepts."
        }
    }
}
#[component(no_case_check)]
pub fn GuideTooling() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "setting-up-tooling",
            a { href: "#setting-up-tooling", class: "header", "Setting up Tooling" }
        }
        p {
            "This section will help you install the necessary tooling to get started with Dioxus. You'll need an editor, Rust, and the Dioxus CLI."
        }
        h2 { id: "prerequisites",
            a { href: "#prerequisites", class: "header", "Prerequisites" }
        }
        h3 { id: "an-editor",
            a { href: "#an-editor", class: "header", "An Editor" }
        }
        p {
            "Dioxus integrates very well with the "
            a { href: "https://rust-analyzer.github.io", "Rust-Analyzer LSP plugin" }
            " which will provide appropriate syntax highlighting, code navigation, folding, and more."
        }
        h3 { id: "rust",
            a { href: "#rust", class: "header", "Rust" }
        }
        p {
            "Head over to "
            a { href: "http://rust-lang.org", "https://rust-lang.org" }
            " and install the Rust compiler."
        }
        p {
            "We strongly recommend going through the "
            a { href: "https://doc.rust-lang.org/book/ch01-00-getting-started.html",
                "official Rust book"
            }
            " "
            em { "completely" }
            ". However, we hope that a Dioxus app can serve as a great first Rust project. With Dioxus, you'll learn about:"
        }
        ul {
            li { "Error handling" }
            li { "Structs, Functions, Enums" }
            li { "Closures" }
            li { "Macros" }
        }
        p {
            "We've put a lot of care into making Dioxus syntax familiar and easy to understand, so you won't need deep knowledge of async, lifetimes, or smart pointers until you start building complex Dioxus apps."
        }
        h3 { id: "platform-specific-dependencies",
            a { href: "#platform-specific-dependencies", class: "header",
                "Platform-specific dependencies"
            }
        }
        p {
            "Most platforms don't require any additional dependencies, but if you are targeting desktop, you can install the following dependencies:"
        }
        DesktopDependencies {}
        h3 { id: "dioxus-cli",
            a { href: "#dioxus-cli", class: "header", "Dioxus CLI" }
        }
        p { "Next, lets install the Dioxus CLI:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cli</span></pre>\n" }
        p {
            "If you get an OpenSSL error on installation, ensure the dependencies listed "
            a { href: "https://docs.rs/openssl/latest/openssl/#automatic", "here" }
            " are installed."
        }
    }
}
#[component(no_case_check)]
pub fn GuideNewApp() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h2 { id: "create-a-new-project",
            a { href: "#create-a-new-project", class: "header", "Create a new project" }
        }
        p {
            "You can create a new Dioxus project by running the following command and following the prompts:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx new</span></pre>\n" }
        video {
            "type": "video/mp4",
            "name": "dx new demo",
            autoplay: "true",
            controls: "false",
            r#loop: "true",
            width: "800px",
            muted: "true",
            source { src: asset!("/assets/static/dioxus-new.mov") }
        }
        p {
            "First you will need to select a platform. Each platform has its own reference with more information on how to set up a project for that platform. Here are the platforms we recommend starting with:"
        }
        ul {
            li {
                "Web"
                ul {
                    li {
                        a { href: "../reference/web", "Client Side" }
                        ": runs in the browser through WebAssembly"
                    }
                    li {
                        a { href: "../reference/fullstack", "Fullstack" }
                        ": renders to HTML text on the server and hydrates it on the client"
                    }
                }
            }
        }
        blockquote {
            p {
                "If you are not sure which web platform you want to use, check out the "
                a { href: "choosing_a_web_renderer", "choosing a web renderer" }
                " chapter."
            }
        }
        ul {
            li {
                "WebView"
                ul {
                    li {
                        a { href: "../reference/desktop", "Desktop" }
                        ": runs in a web view on desktop"
                    }
                    li {
                        a { href: "../reference/mobile", "Mobile" }
                        ": runs in a web view on mobile. Mobile is currently not supported by the dioxus CLI. The "
                        a { href: "../reference/mobile", "mobile reference" }
                        " has more information about setting up a mobile project"
                    }
                }
            }
        }
        p { "Next, you can choose a styling library. For this project, we will use vanilla CSS." }
        p {
            "Finally, you can choose to start the project with the router enabled. The router is covered in the "
            a { href: "../router", "router guide" }
            "."
        }
        h2 { id: "running-the-project",
            a { href: "#running-the-project", class: "header", "Running the project" }
        }
        p { "Once you have created your project, you can start it with the following command:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cd my_project\n</span><span style=\"color:#f8f8f2;\">dx serve</span></pre>\n" }
        p {
            "For projects using the liveview template, run  "
            code { "dx serve --desktop" }
            "."
        }
        p {
            "For Web targets the application will be served at "
            a { href: "http://localhost:8080", "http://localhost:8080" }
        }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "That's it! You now have a working Dioxus project. You can continue learning about dioxus by "
            a { href: "../guide", "making a hackernews clone in the guide" }
            ", or learning about specific topics/platforms in the "
            a { href: "../reference", "reference" }
            ". If you have any questions, feel free to ask in the "
            a { href: "https://discord.gg/XgGxMSkvUM", "discord" }
            " or "
            a { href: "https://github.com/DioxusLabs/dioxus/discussions", "open a discussion" }
            "."
        }
    }
}
#[component(no_case_check)]
pub fn GuideComponent() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn GuideAssets() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn GuideState() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "interactivity",
            a { href: "#interactivity", class: "header", "Interactivity" }
        }
        p { "In this chapter, we will add a preview for articles you hover over or links you focus on." }
        h2 { id: "creating-a-preview",
            a { href: "#creating-a-preview", class: "header", "Creating a Preview" }
        }
        p {
            "First, let's split our app into a Stories component on the left side of the screen, and a preview component on the right side of the screen:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">, flex_direction: </span><span style=\"color:#ffee99;\">&quot;row&quot;</span><span style=\"color:#f8f8f2;\">, width: </span><span style=\"color:#ffee99;\">&quot;100%&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ width: </span><span style=\"color:#ffee99;\">&quot;50%&quot;</span><span style=\"color:#f8f8f2;\">, Stories {{}} }}\n</span><span style=\"color:#f8f8f2;\">            div {{ width: </span><span style=\"color:#ffee99;\">&quot;50%&quot;</span><span style=\"color:#f8f8f2;\">, Preview {{}} }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Stories</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        StoryListing {{\n</span><span style=\"color:#f8f8f2;\">            story: StoryItem {{\n</span><span style=\"color:#f8f8f2;\">                id: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                title: </span><span style=\"color:#ffee99;\">&quot;hello hackernews&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                url: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                text: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                by: </span><span style=\"color:#ffee99;\">&quot;Author&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                score: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                descendants: </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                time: chrono::Utc::now(),\n</span><span style=\"color:#f8f8f2;\">                kids: vec![],\n</span><span style=\"color:#f8f8f2;\">                r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Debug)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">PreviewState {{\n</span><span style=\"color:#f8f8f2;\">    Unset,\n</span><span style=\"color:#f8f8f2;\">    Loading,\n</span><span style=\"color:#f8f8f2;\">    Loaded(StoryPageData),\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Preview</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Unset;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> preview_state {{\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Unset </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;Hover over a story to preview it here&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Loading </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{</span><span style=\"color:#ffee99;\">&quot;Loading...&quot;</span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        PreviewState::Loaded(story) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            rsx! {{\n</span><span style=\"color:#f8f8f2;\">                div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                    div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">, a {{ href: story.item.url, </span><span style=\"color:#ffee99;\">&quot;{{story.item.title}}&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                    div {{ dangerous_inner_html: story.item.text }}\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> comment </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">story.comments {{\n</span><span style=\"color:#f8f8f2;\">                        Comment {{ comment: comment.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// NEW\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Comment</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">comment</span><span style=\"color:#f8f8f2;\">: CommentData) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;by {{comment.by}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ dangerous_inner_html: </span><span style=\"color:#ffee99;\">&quot;{{comment.text}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> kid </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">comment.sub_comments {{\n</span><span style=\"color:#f8f8f2;\">                Comment {{ comment: kid.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        DemoFrame { hackernews_state::app_v1::App {} }
        h2 { id: "event-handlers",
            a { href: "#event-handlers", class: "header", "Event Handlers" }
        }
        p {
            "Next, we need to detect when the user hovers over a section or focuses a link. We can use an "
            a { href: "../reference/event_handlers", "event listener" }
            " to listen for the hover and focus events."
        }
        p {
            "Event handlers are similar to regular attributes, but their name usually starts with  "
            code { "on" }
            "- and they accept closures as values. The closure will be called whenever the event it listens for is triggered. When an event is triggered, information about the event is passed to the closure through the "
            a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/struct.Event.html",
                "Event"
            }
            " structure."
        }
        p {
            "Let's create a "
            a { href: "https://docs.rs/dioxus/latest/dioxus/events/fn.onmouseenter.html",
                code { "onmouseenter" }
            }
            " event listener in the "
            code { "StoryListing" }
            " component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    div {{\n</span><span style=\"color:#f8f8f2;\">        padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        onmouseenter: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{}},\n</span><span style=\"color:#f8f8f2;\">        div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            a {{ href: url, onfocus: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{}}, </span><span style=\"color:#ffee99;\">&quot;{{title}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            a {{\n</span><span style=\"color:#f8f8f2;\">                color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                href: </span><span style=\"color:#ffee99;\">&quot;https://news.ycombinator.com/from?site={{hostname}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                text_decoration: </span><span style=\"color:#ffee99;\">&quot;none&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot; ({{hostname}})&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        div {{ display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">, flex_direction: </span><span style=\"color:#ffee99;\">&quot;row&quot;</span><span style=\"color:#f8f8f2;\">, color: </span><span style=\"color:#ffee99;\">&quot;gray&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            div {{ </span><span style=\"color:#ffee99;\">&quot;{{score}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;by {{by}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{time}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ padding_left: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{comments}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        blockquote {
            p {
                "You can read more about Event Handlers in the "
                a { href: "../reference/event_handlers", "Event Handler reference" }
            }
        }
        h2 { id: "state",
            a { href: "#state", class: "header", "State" }
        }
        p {
            "So far our components have had no state like normal rust functions. To make our application change when we hover over a link we need state to store the currently hovered link in the root of the application."
        }
        p {
            "You can create state in dioxus using hooks. Hooks are Rust functions you call in a constant order in a component that add additional functionality to the component."
        }
        p {
            "In this case, we will use the  "
            code { "use_context_provider" }
            " and  "
            code { "use_context" }
            " hooks:"
        }
        ul {
            li {
                "You can provide a closure to "
                code { "use_context_provider" }
                " that determines the initial value of the shared state and provides the value to all child components"
            }
            li {
                "You can then use the "
                code { "use_context" }
                " hook to read and modify that state in the "
                code { "Preview" }
                " and "
                code { "StoryListing" }
                " components"
            }
            li {
                "When the value updates, the "
                code { "Signal" }
                " will cause the component to re-render, and provides you with the new value"
            }
        }
        blockquote {
            p {
                "Note: You should prefer local state hooks like use_signal or use_signal_sync when you only use state in one component. Because we use state in multiple components, we can use a "
                a { href: "../reference/context", "global state pattern" }
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| Signal::new(PreviewState::Unset));</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">StoryListing</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">story</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;StoryItem&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">consume_context::&lt;Signal&lt;PreviewState&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> StoryItem {{\n</span><span style=\"color:#f8f8f2;\">        title,\n</span><span style=\"color:#f8f8f2;\">        url,\n</span><span style=\"color:#f8f8f2;\">        by,\n</span><span style=\"color:#f8f8f2;\">        score,\n</span><span style=\"color:#f8f8f2;\">        time,\n</span><span style=\"color:#f8f8f2;\">        kids,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">..\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">= &amp;*</span><span style=\"color:#f8f8f2;\">story.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> url </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url.</span><span style=\"color:#66d9ef;\">as_deref</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap_or_default</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> hostname </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> url\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;https://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;http://&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">trim_start_matches</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;www.&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> score </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{score}}</span><span style=\"color:#ffee99;\"> point</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">if *</span><span style=\"color:#f8f8f2;\">score </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot;s&quot; </span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot;&quot; </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> comments </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}} {{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> kids.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comment&quot;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot; comments&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> time </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> time.</span><span style=\"color:#66d9ef;\">format</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;%D %l:%M %p&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            padding: </span><span style=\"color:#ffee99;\">&quot;0.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            position: </span><span style=\"color:#ffee99;\">&quot;relative&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            onmouseenter: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(StoryPageData {{\n</span><span style=\"color:#f8f8f2;\">                    item: </span><span style=\"color:#66d9ef;\">story</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    comments: vec![],\n</span><span style=\"color:#f8f8f2;\">                }});\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            div {{ font_size: </span><span style=\"color:#ffee99;\">&quot;1.5rem&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                a {{\n</span><span style=\"color:#f8f8f2;\">                    href: url,\n</span><span style=\"color:#f8f8f2;\">                    onfocus: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">_event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">preview_state\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">PreviewState::Loaded(StoryPageData {{\n</span><span style=\"color:#f8f8f2;\">                            item: </span><span style=\"color:#66d9ef;\">story</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                            comments: vec![],\n</span><span style=\"color:#f8f8f2;\">                        }});\n</span><span style=\"color:#f8f8f2;\">                    }},</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Preview</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> preview_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">consume_context::&lt;Signal&lt;PreviewState&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// New\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">preview_state</span><span style=\"color:#f8f8f2;\">() {{</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        DemoFrame { hackernews_state::App {} }
        blockquote {
            p {
                "You can read more about Hooks in the "
                a { href: "../reference/hooks", "Hooks reference" }
            }
        }
        h3 { id: "the-rules-of-hooks",
            a { href: "#the-rules-of-hooks", class: "header", "The Rules of Hooks" }
        }
        p {
            "Hooks are a powerful way to manage state in Dioxus, but there are some rules you need to follow to insure they work as expected. Dioxus uses the order you call hooks to differentiate between hooks. Because the order you call hooks matters, you must follow these rules:"
        }
        ol {
            li { "Hooks may be only used in components or other hooks (we'll get to that later)" }
            li {
                "On every call to the component function"
                ol {
                    li { "The same hooks must be called" }
                    li { "In the same order" }
                }
            }
            li {
                "Hooks name's should start with "
                code { "use_" }
                " so you don't accidentally confuse them with regular functions"
            }
        }
        p { "These rules mean that there are certain things you can't do with hooks:" }
        h4 { id: "no-hooks-in-conditionals",
            a { href: "#no-hooks-in-conditionals", class: "header", "No Hooks in Conditionals" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ don&#39;t call hooks in conditionals!\n</span><span style=\"color:#8c8c8c;\">// We must ensure that the same hooks will be called every time\n</span><span style=\"color:#8c8c8c;\">// But `if` statements only run if the conditional is true!\n</span><span style=\"color:#8c8c8c;\">// So we might violate rule 2.\n</span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> you_are_happy </span><span style=\"color:#f92672;\">&amp;&amp;</span><span style=\"color:#f8f8f2;\"> you_know_it {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> something </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;hands&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;clap your </span><span style=\"color:#ff80f4;\">{{something}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ instead, *always* call use_signal\n</span><span style=\"color:#8c8c8c;\">// You can put other stuff in the conditional though\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> something </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;hands&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> you_are_happy </span><span style=\"color:#f92672;\">&amp;&amp;</span><span style=\"color:#f8f8f2;\"> you_know_it {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;clap your </span><span style=\"color:#ff80f4;\">{{something}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        h4 { id: "no-hooks-in-closures",
            a { href: "#no-hooks-in-closures", class: "header", "No Hooks in Closures" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ don&#39;t call hooks inside closures!\n</span><span style=\"color:#8c8c8c;\">// We can&#39;t guarantee that the closure, if used, will be called in the same order every time\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#a6e22e;\">_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|| {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">b</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ instead, move hook `b` outside\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> b </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#a6e22e;\">_a </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|| </span><span style=\"color:#66d9ef;\">b</span><span style=\"color:#f8f8f2;\">();</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
        h4 { id: "no-hooks-in-loops",
            a { href: "#no-hooks-in-loops", class: "header", "No Hooks in Loops" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// `names` is a Vec&lt;&amp;str&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ Do not use hooks in loops!\n</span><span style=\"color:#8c8c8c;\">// In this case, if the length of the Vec changes, we break rule 2\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> _name </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">names {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> is_selected </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;selected: </span><span style=\"color:#ff80f4;\">{{is_selected}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Instead, use a hashmap with use_signal\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> selection_map </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(HashMap::&lt;</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">&gt;::new);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">in &amp;</span><span style=\"color:#f8f8f2;\">names {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> is_selected </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> selection_map.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">()[name];\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;selected: </span><span style=\"color:#ff80f4;\">{{is_selected}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hackernews_state.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn GuideRouting() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn GuideFetching() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn GuideMultiplatform() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn GuideBackend() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn GuideDeploy() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn GuideNextSteps() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn EssentialsIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "essentials!" }
    }
}
#[component(no_case_check)]
pub fn EssentialsLifecycleIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "component-lifecycle",
            a { href: "#component-lifecycle", class: "header", "Component Lifecycle" }
        }
        h2 { id: "initializing-state-with",
            a { href: "#initializing-state-with", class: "header", "Initializing State with " }
            code { "use_hook" }
        }
        p {
            code { "use_hook" }
            " lets you create new state for your component. The closure you pass to  "
            code { "use_hook" }
            " will be called once the first time the component is rendered. Every time the component is re-rendered, the value that was created the first run will be re-used."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">UseHook</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// The closure that is passed to use_hook will be called once the first time the component is rendered\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> random_number </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new_random_number </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">random_number</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{new_random_number}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        new_random_number\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Random {{random_number}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::UseHookDemo {} }
        h2 { id: "rerendering",
            a { href: "#rerendering", class: "header", "Rerendering" }
        }
        p {
            "You can use "
            a { href: "lifecycle/../reference/reactivity", "tracked values" }
            " to re-render your component whenever a value changes. "
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Rerenders</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    log!(</span><span style=\"color:#ffee99;\">&quot;Rerendering parent component with {{}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">count.</span><span style=\"color:#66d9ef;\">peek</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since we read count here, the component will rerender when count changes\n</span><span style=\"color:#f8f8f2;\">        Count {{ current_count: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// If the count prop changes, the component will rerender\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">current_count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    log!(</span><span style=\"color:#ffee99;\">&quot;Rerendering child component with {{current_count}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;The count is {{current_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::RerenderDemo {} }
        h3 { id: "-dont-mutate-state-in-the-body-of-a-component",
            a {
                href: "#-dont-mutate-state-in-the-body-of-a-component",
                class: "header",
                "⚠\u{fe0f} Don't mutate state in the body of a component"
            }
        }
        p {
            "You should avoid changing state in the body of a component. If you read and write to state in the body of a component, you can cause an infinite loop as the component tries to rerender because of the change which triggers another state change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Bad</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ❌ Don&#39;t mutate state in the body of the component.\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// It can easily cause an infinite loop!\n</span><span style=\"color:#f8f8f2;\">    count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ </span><span style=\"color:#ffee99;\">&quot;{{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        p {
            "Instead, derive state with  "
            code { "use_memo" }
            ",  "
            code { "use_resource" }
            ", or mutate state in a effect."
        }
        h2 { id: "using-effects",
            a { href: "#using-effects", class: "header", "Using Effects" }
        }
        p {
            "You can use "
            a { href: "lifecycle/../reference/reactivity", "effects" }
            " to run code whenever a component is rendered."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Effect</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Effects run after the component is rendered\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can use them to read or modify the rendered component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;Effect ran&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">        document::eval(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;document.getElementById(&#39;effect-output&#39;).innerText = &#39;Effect ran&#39;&quot;\n</span><span style=\"color:#f8f8f2;\">        ));\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ id: </span><span style=\"color:#ffee99;\">&quot;effect-output&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;This will be changed by the effect&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::EffectDemo {} }
        h2 { id: "cleaning-up-components-with-drop",
            a { href: "#cleaning-up-components-with-drop", class: "header",
                "Cleaning Up Components with Drop"
            }
        }
        p {
            "Before a component is dropped, it will drop all of its hooks. You can use this drop behavior to clean up any resources that your component is using. If you just need the drop effect, you can use the "
            a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_drop.html",
                code { "use_drop" }
            }
            " hook."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">TogglesChild</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> show </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> show.</span><span style=\"color:#66d9ef;\">toggle</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;Toggle&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">show</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            Child {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can use the use_drop hook to clean up any resources\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_drop</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;Child dropped&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Child&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_lifecycle.rs".to_string(),
        }
        DemoFrame { component_lifecycle::DropDemo {} }
    }
}
#[component(no_case_check)]
pub fn EssentialsStateIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "managing-state",
            a { href: "#managing-state", class: "header", "Managing State" }
        }
        p {
            "In Dioxus, your app is defined as a function of the current state. As the state changes, the parts of your app that depend on that state will automatically re-run. Reactivity automatically tracks state and updates derived state in your application."
        }
        h2 { id: "creating-state",
            a { href: "#creating-state", class: "header", "Creating State" }
        }
        p {
            "You can create mutable state in Dioxus with Signals. Signals are tracked values that automatically update your app when you change them. They form the skeleton of your app's state from which you can derive other state. Signals are often driven directly from user input through event handlers or async tasks."
        }
        p {
            "You can create a signal with the  "
            code { "use_signal" }
            " hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        p {
            "Once you have your signal, you can clone it by calling the signal like a function or get a reference to the inner value with the  "
            code { ".read()" }
            " method:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Call the signal like a function to clone the current value\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">signal</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#8c8c8c;\">// get a reference to the inner value with the .read() method\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">signal.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#8c8c8c;\">// or use one of the traits implemented for Signal like Display\n</span><span style=\"color:#f8f8f2;\">log!(</span><span style=\"color:#ffee99;\">&quot;{{signal}}&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        p {
            "Finally, you can set the value of the signal with the  "
            code { ".set()" }
            " method or get a mutable reference to the inner value with the  "
            code { ".write()" }
            " method:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Set the value from the signal\n</span><span style=\"color:#f8f8f2;\">signal.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#8c8c8c;\">// get a mutable reference to the inner value with the .write() method\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> value: </span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= &amp;mut</span><span style=\"color:#f8f8f2;\"> signal.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">value </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        h3 { id: "reactive-scopes",
            a { href: "#reactive-scopes", class: "header", "Reactive Scopes" }
        }
        p {
            "The simplest reactive primitive in Dioxus is the  "
            code { "use_effect" }
            " hook. It creates a closure that is run any time a tracked value that is run inside the closure changes."
        }
        p {
            "Any value you read inside the closure will become a dependency of the effect. If the value changes, the effect will rerun."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Effect</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// use_signal creates a tracked value called count\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// When we read count, it becomes a dependency of the effect\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> current_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Whenever count changes, the effect will rerun\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{current_count}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::EffectDemo {} }
        h3 { id: "derived-state",
            a { href: "#derived-state", class: "header", "Derived State" }
        }
        p {
            code { "use_memo" }
            " is a reactive primitive that lets you derive state from any tracked value. It takes a closure that computes the new state and returns a tracked value with the current state of the memo. Any time a dependency of the memo changes, the memo will rerun."
        }
        p {
            "The value you return from the closure will only change when the output of the closure changes ( "
            code { "PartialEq" }
            " between the old and new value returns false)."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Memo</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// use_memo creates a tracked value that is derived from count\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Since we read count inside the closure, it becomes a dependency of the memo\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Whenever count changes, the memo will rerun\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> half_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">/ </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// half_count is itself a tracked value\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// When we read half_count, it becomes a dependency of the effect\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// and the effect will rerun when half_count changes\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{half_count}}&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Half count is {{half_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::MemoDemo {} }
        h3 { id: "derived-async-state",
            a { href: "#derived-async-state", class: "header", "Derived Async State" }
        }
        p {
            code { "use_resource" }
            " is a reactive primitive that lets you derive state from any async closure. It takes an async closure that computes the new state and returns a tracked value with the current state of the resource. Any time a dependency of the resource changes, the resource will rerun."
        }
        p {
            "The value you return from the closure will only change when the state of the future changes. Unlike  "
            code { "use_memo" }
            ", the resource's output is not memoized with  "
            code { "PartialEq" }
            "."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Resource</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// use_resource creates a tracked value that is derived from count\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Since we read count inside the closure, it becomes a dependency of the resource\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Whenever count changes, the resource will rerun\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> half_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can do async work inside resources\n</span><span style=\"color:#f8f8f2;\">        gloo_timers::future::TimeoutFuture::new(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">).await;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">/ </span><span style=\"color:#ff80f4;\">2\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// half_count is itself a tracked value\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// When we read half_count, it becomes a dependency of the effect\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// and the effect will rerun when half_count changes\n</span><span style=\"color:#f8f8f2;\">        log!(</span><span style=\"color:#ffee99;\">&quot;{{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">half_count</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Change Signal&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Half count is {{half_count():?}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::ResourceDemo {} }
        h3 { id: "derived-ui",
            a { href: "#derived-ui", class: "header", "Derived UI" }
        }
        p {
            "Components are functions that return some UI. They memorize the output of the function just like memos. Components keep track of any dependencies you read inside the component and rerun when those dependencies change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Component</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Change Signal&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since we read count inside Component, it becomes a dependency of Component\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Whenever count changes, Component will rerun\n</span><span style=\"color:#f8f8f2;\">        Count {{ count: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Components automatically memorize their props. If the props change, Count will rerun\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Count: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::ComponentDemo {} }
        h3 { id: "working-with-untracked-state",
            a { href: "#working-with-untracked-state", class: "header",
                "Working with Untracked State"
            }
        }
        p {
            "Most of the state in your app will be tracked values. All built in hooks return tracked values, and we encourage custom hooks to do the same. However, there are times when you need to work with untracked state. For example, you may receive a raw untracked value in props. When you read an untracked value inside a reactive context, it will not subscribe to the value:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Component</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Change Signal&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        Count {{ count: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// The count reruns the component when it changes, but it is not a tracked value\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// When you read count inside the memo, it does not subscribe to the count signal\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// because the value is not reactive\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> double_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Double count: {{double_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::NonReactiveDemo {} }
        p {
            "You can start tracking raw state with the  "
            code { "use_reactive" }
            " hook. This hook takes a tuple of dependencies and returns a reactive closure. When the closure is called in a reactive context, it will track subscribe to the dependencies and rerun the closure when the dependencies change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can manually track a non-reactive value with the use_reactive hook\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> double_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Use reactive takes a tuple of dependencies and returns a reactive closure\n</span><span style=\"color:#f8f8f2;\">        use_reactive!(|(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">,)| count </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Double count: {{double_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::UseReactiveDemo {} }
        h3 { id: "making-props-reactive",
            a { href: "#making-props-reactive", class: "header", "Making Props Reactive" }
        }
        p {
            "To avoid loosing reactivity with props, we recommend you wrap any props you want to track in a  "
            code { "ReadOnlySignal" }
            ". Dioxus will automatically convert  "
            code { "T" }
            " into  "
            code { "ReadOnlySignal<T>" }
            " when you pass props to the component. This will ensure your props are tracked and rerun any state you derive in the component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can track props by wrapping the type in a ReadOnlySignal\n</span><span style=\"color:#8c8c8c;\">// Dioxus will automatically convert T into ReadOnlySignal&lt;T&gt; when you pass\n</span><span style=\"color:#8c8c8c;\">// props to the component\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Count</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Then when you read count inside the memo, it subscribes to the count signal\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> double_count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;Double count: {{double_count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { reactivity::MakingPropsReactiveDemo {} }
        h2 { id: "moving-around-state",
            a { href: "#moving-around-state", class: "header", "Moving Around State" }
        }
        p {
            "As you create signals and derived state in your app, you will need to move around that state between components. Dioxus provides three different ways to pass around state:"
        }
        h3 { id: "passing-props",
            a { href: "#passing-props", class: "header", "Passing props" }
        }
        p {
            "You can pass your values through component "
            a { href: "state/./component_props", "props" }
            ". This should be your default when passing around state. It is the most explicit and local to your component. Use this until it gets annoying to pass around the value:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ParentComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Count is {{count}}&quot;\n</span><span style=\"color:#f8f8f2;\">        IncrementButton {{\n</span><span style=\"color:#f8f8f2;\">            count\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">IncrementButton</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">count</span><span style=\"color:#f8f8f2;\">: Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { moving_state_around::PassingProps {} }
        h3 { id: "passing-context",
            a { href: "#passing-context", class: "header", "Passing context" }
        }
        p {
            "If you need a slightly more powerful way to pass around state, you can use the context API."
        }
        p {
            "The context API lets you pass state from a parent component to all children. This is useful if you want to share state between many components. You can insert a unique type into the context with the "
            a { href: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context_provider.html",
                code { "use_context_provider" }
            }
            " hook in the parent component. Then you can access the context in any child component with the "
            a { href: "https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_context.html",
                code { "use_context" }
            }
            " hook."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Clone, Copy)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">MyState {{\n</span><span style=\"color:#f8f8f2;\">    count: Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ParentComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use context provider provides an unique type to all children of this component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| MyState {{\n</span><span style=\"color:#f8f8f2;\">        count: Signal::new(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Count is {{state.count}}&quot;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// IncrementButton will have access to the count without explicitly passing it through props\n</span><span style=\"color:#f8f8f2;\">        IncrementButton {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">IncrementButton</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use context gets the value from a parent component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_context::&lt;MyState&gt;().count;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { moving_state_around::PassingContext {} }
        p {
            "This is slightly less explicit than passing it as a prop, but it is still local to the component. This is really great if you want state that is global to part of your app. It lets you create multiple global-ish states while still making state different when you reuse components. If I create a new  "
            code { "ParentComponent" }
            ", it will have a new  "
            code { "MyState" }
            "."
        }
        h3 { id: "using-globals",
            a { href: "#using-globals", class: "header", "Using globals" }
        }
        p {
            "Finally, if you have truly global state, you can put your state in a  "
            code { "Global<T>" }
            " static. This is useful if you want to share state with your whole app:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#8c8c8c;\">// Globals are created the first time you access them with the closure you pass to Global::new\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Global::new(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ParentComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Count is {{COUNT}}&quot;\n</span><span style=\"color:#f8f8f2;\">        IncrementButton {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">IncrementButton</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// You don&#39;t need to pass anything around or get anything out of the context because COUNT is global\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| *</span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "reactivity.rs".to_string(),
        }
        DemoFrame { moving_state_around::UsingGlobals {} }
        p {
            "Global state can be very ergonomic if your state is truly global, but you shouldn't use it if you need state to be different for different instances of your component. If I create another  "
            code { "IncrementButton" }
            " it will use the same  "
            code { "COUNT" }
            ". Libraries should generally avoid this to make components more reusable."
        }
        blockquote {
            p {
                "Note: Even though it is in a static,  "
                code { "COUNT" }
                " will be different for each app instance so you don't need to worry about state mangling when multiple instances of your app are running on the server"
            }
        }
    }
}
#[component(no_case_check)]
pub fn EssentialsBreakingIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "breaking-out-of-dioxus",
            a { href: "#breaking-out-of-dioxus", class: "header", "Breaking Out of Dioxus" }
        }
        p {
            "Dioxus is makes it easy to build reactive user interfaces. However, there are some cases where you may need to break out of the reactive paradigm to interact with the DOM directly."
        }
        h2 { id: "interacting-with-javascript-with",
            a { href: "#interacting-with-javascript-with", class: "header",
                "Interacting with JavaScript with "
            }
            code { "eval" }
            " and "
            code { "web-sys" }
        }
        p {
            "Dioxus exposes a limited number of "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/API", "web apis" }
            " with a nicer interface. If you need access to more APIs, you can use the "
            code { "eval" }
            " function to run JavaScript in the browser."
        }
        p { "For example, you can use the eval function to read the domain of the current page:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Eval</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> domain </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new);\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// When you click the button, some javascript will run in the browser\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// to read the domain and set the signal\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                domain.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(document::eval(</span><span style=\"color:#ffee99;\">&quot;return document.domain&quot;</span><span style=\"color:#f8f8f2;\">).await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Read Domain&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Current domain: {{domain}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::Eval {} }
        p {
            "If you are only targeting web, you can also use the "
            a { href: "https://crates.io/crates/web-sys",
                code { "web-sys" }
            }
            " crate for typed access to the web APIs. Here is what reading the domain looks like with web-sys:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">::web_sys::window;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">wasm_bindgen::JsCast;\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">WebSys</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> domain </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::new);\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// When you click the button, we use web-sys to read the domain and a signal\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                domain\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#66d9ef;\">window</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">document</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .dyn_into::&lt;::web_sys::HtmlDocument&gt;()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">domain</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                    );\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Read Domain&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Current domain: {{domain}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::WebSys {} }
        h2 { id: "synchronizing-dom-updates-with",
            a { href: "#synchronizing-dom-updates-with", class: "header",
                "Synchronizing DOM updates with "
            }
            code { "use_effect" }
        }
        p {
            "If you do need to interact with the DOM directly, you should do so in a  "
            code { "use_effect" }
            " hook. This hook will run after the component is rendered and all of the Dioxus UI has been rendered. You can read or modify the DOM in this hook."
        }
        p {
            "For example, you can use the  "
            code { "use_effect" }
            " hook to write to a canvas element after it is created:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Canvas</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Effects are reactive like memos, and resources. If you read a value inside the effect, the effect will rerun when that value changes\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> count.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can use the count value to update the DOM manually\n</span><span style=\"color:#f8f8f2;\">        document::eval(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">format!(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;var c = document.getElementById(&quot;dioxus-canvas&quot;);\n</span><span style=\"color:#ffee99;\">var ctx = c.getContext(&quot;2d&quot;);\n</span><span style=\"color:#ffee99;\">ctx.clearRect(0, 0, c.width, c.height);\n</span><span style=\"color:#ffee99;\">ctx.font = &quot;30px Arial&quot;;\n</span><span style=\"color:#ffee99;\">ctx.fillText(&quot;</span><span style=\"color:#ff80f4;\">{{count}}</span><span style=\"color:#ffee99;\">&quot;, 10, 50);&quot;#\n</span><span style=\"color:#f8f8f2;\">        ));\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// When you click the button, count will be incremented and the effect will rerun\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        canvas {{ id: </span><span style=\"color:#ffee99;\">&quot;dioxus-canvas&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::Canvas {} }
        h2 { id: "getting-access-to-elements-with",
            a { href: "#getting-access-to-elements-with", class: "header",
                "Getting access to elements with "
            }
            code { "onmounted" }
        }
        p {
            "If you need a handle to an element that is rendered by dioxus, you can use the  "
            code { "onmounted" }
            " event. This event will fire after the element is first mounted to the DOM. It returns a live reference to the element with some methods to interact with it."
        }
        p {
            "You can use the onmounted event to do things like focus or scroll to an element after it is rendered:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">OnMounted</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> input_element </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ height: </span><span style=\"color:#ffee99;\">&quot;100px&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                class: </span><span style=\"color:#ffee99;\">&quot;focus:outline-2 focus:outline-blue-600 focus:outline-dashed&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// The onmounted event will run the first time the button element is mounted\n</span><span style=\"color:#f8f8f2;\">                onmounted: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">element</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> input_element.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(element.</span><span style=\"color:#66d9ef;\">data</span><span style=\"color:#f8f8f2;\">())),\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;First button&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// When you click the button, if the button element has been mounted, we focus to that element\n</span><span style=\"color:#f8f8f2;\">                onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(header) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">input_element</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ =</span><span style=\"color:#f8f8f2;\"> header.</span><span style=\"color:#66d9ef;\">set_focus</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">).await;\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Focus first button&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::OnMounted {} }
        h2 { id: "down-casting-web-sys-events",
            a { href: "#down-casting-web-sys-events", class: "header",
                "Down casting web sys events"
            }
        }
        p {
            "Dioxus provides platform agnostic wrappers over each event type. These wrappers are often nicer to interact with than the raw event types, but they can be more limited. If you are targeting web, you can downcast the event with the  "
            code { "as_web_event" }
            " method to get the underlying web-sys event:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::web::WebEventExt;\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Downcast</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> event_text </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            onmousemove: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                event_text.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(event.</span><span style=\"color:#66d9ef;\">as_web_event</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">movement_x</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;movement_x was {{event_text}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "breaking_out.rs".to_string(),
        }
        DemoFrame { breaking_out::Downcast {} }
    }
}
#[component(no_case_check)]
pub fn EssentialsStructureIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}
#[component(no_case_check)]
pub fn EssentialsRsxIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "building-uis-with-rsx",
            a { href: "#building-uis-with-rsx", class: "header", "Building UIs with RSX" }
        }
        p {
            "Dioxus renders to HTML, if you are not familiar with HTML, this guide will help you get started with the basics. For more detail, the "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/HTML", "MDN docs" }
            " are a great resource."
        }
        h2 { id: "text-nodes",
            a { href: "#text-nodes", class: "header", "Text Nodes" }
        }
        p { "Any content surrounded by quotes is rendered as a text node in rsx:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Hello world&quot;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::Text {} }
        p {
            "You can include formatted segments inside of the text just like the  "
            code { "format!" }
            " macro:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> user </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| User {{\n</span><span style=\"color:#f8f8f2;\">    name: </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Unlike the format macro, you can include many expressions inline in the formatted text\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Hello {{user.read().name}}&quot;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::FormattedText {} }
        h2 { id: "elements",
            a { href: "#elements", class: "header", "Elements" }
        }
        p {
            "The most basic building block of HTML is an element. In rsx, you can create elements with the name and then curly braces. One of the most common elements is the  "
            code { "input" }
            " element. The input element creates an interactive input box:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::Input {} }
        blockquote {
            p { "Bonus: web components" }
            CodeBlock {
                contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    my</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">web</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">component {{}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
                name: "building_uis_with_rsx.rs".to_string(),
            }
        }
        h2 { id: "attributes",
            a { href: "#attributes", class: "header", "Attributes" }
        }
        p {
            "Attributes provide extra information about an element. You can specify attributes in dioxus inside an element's braces by typing the name of the attribute, a colon, and then the value (typically a formatted string). We can use an attribute to set the  "
            code { "type" }
            " of an input element. The default type is  "
            code { "text" }
            " which shows a text input box, but we can set it to  "
            code { "number" }
            " to only accept numbers:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{ </span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;number&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputType {} }
        p {
            "Just like text nodes, attributes can include formatted segments. We can set the value of the input element to a signal to control it:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Hello world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{ value: </span><span style=\"color:#ffee99;\">&quot;{{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputValue {} }
        h3 { id: "conditional-attributes",
            a { href: "#conditional-attributes", class: "header", "Conditional Attributes" }
        }
        p {
            "You can conditionally set an attribute by setting the attribute value to an unterminated if statement. If the if statement evaluates to true, the attribute will be set:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> number_type </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{ </span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">number_type</span><span style=\"color:#f8f8f2;\">() {{ </span><span style=\"color:#ffee99;\">&quot;number&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputDisabled {} }
        h2 { id: "event-listeners",
            a { href: "#event-listeners", class: "header", "Event Listeners" }
        }
        p {
            "Event listeners allow you to respond to user input. In rsx, event handlers always start with  "
            code { "on" }
            ". The syntax is the same as normal attributes, but event handlers only accept a closure that responds to the event. We can attach an event listener to the  "
            code { "oninput" }
            " event of the input element to listen for changes to the input:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Hello world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    input {{\n</span><span style=\"color:#f8f8f2;\">        oninput: </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">event</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> value.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(event.</span><span style=\"color:#66d9ef;\">value</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">        value: </span><span style=\"color:#ffee99;\">&quot;{{value}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::OnInput {} }
        h2 { id: "children",
            a { href: "#children", class: "header", "Children" }
        }
        p {
            "You can add children to an element after all attributes and event listeners. Elements can accept text, components or other elements as children. We can add a  "
            code { "div" }
            " element around our input to center it:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    div {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// display sets the layout mode of the element\n</span><span style=\"color:#f8f8f2;\">        display: </span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// justify-content centers the element horizontally\n</span><span style=\"color:#f8f8f2;\">        justify_content: </span><span style=\"color:#ffee99;\">&quot;center&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        input {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;number&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::InputChildren {} }
        h2 { id: "loops",
            a { href: "#loops", class: "header", "Loops" }
        }
        p {
            "You can insert for loops directly in rsx. The body of the loop accepts any number of children that will be rendered with each iteration of the loop. The  "
            code { "ul" }
            " element in html renders an unordered list with any number of  "
            code { "li" }
            " (list item) elements. We can use those two elements to render a list of items in a loop:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ </span><span style=\"color:#ffee99;\">&quot;{{item}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::ForLoop {} }
        p {
            "Each item in your list should have unique value that is stable across rerenders called a key. Keys are used to identify how items move while diffing. Without keys, it is easy to accidentally lose or move state when you reorder items in a list. We can add keys to our list items by using the  "
            code { "key" }
            " attribute:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{item}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;{{item}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::KeyedForLoop {} }
        h2 { id: "if-statements",
            a { href: "#if-statements", class: "header", "If Statements" }
        }
        p {
            "You can also use if/else statements in rsx. Each branch of the if statement accepts child nodes that will be rendered if the condition is true. We can use the  "
            code { "if" }
            " statement to conditionally render a login screen:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    div {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">logged_in</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;You are logged in&quot;\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;You are not logged in&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "building_uis_with_rsx.rs".to_string(),
        }
        DemoFrame { building_uis_with_rsx::IfStatement {} }
    }
}
#[component(no_case_check)]
pub fn ReferenceIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "dioxus-reference",
            a { href: "#dioxus-reference", class: "header", "Dioxus Reference" }
        }
        p {
            "This Reference contains more detailed explanations for all concepts covered in the "
            a { href: "reference/../guide",
                code { "guide" }
            }
            " and more."
        }
        h2 { id: "rendering",
            a { href: "#rendering", class: "header", "Rendering" }
        }
        ul {
            li {
                a { href: "reference/rsx",
                    code { "RSX" }
                }
                " Rsx is a HTML-like macro that allows you to declare UI"
            }
            li {
                a { href: "reference/components",
                    code { "Components" }
                }
                " Components are the building blocks of UI in Dioxus"
            }
            li {
                a { href: "reference/component_props",
                    code { "Props" }
                }
                " Props allow you pass information to Components"
            }
            li {
                a { href: "reference/event_handlers",
                    code { "Event Listeners" }
                }
                " Event listeners let you respond to user input"
            }
            li {
                a { href: "reference/user_input",
                    code { "User Input" }
                }
                " How to handle User input in Dioxus"
            }
            li {
                a { href: "reference/dynamic_rendering",
                    code { "Dynamic Rendering" }
                }
                " How to dynamically render data in Dioxus"
            }
        }
        h2 { id: "state",
            a { href: "#state", class: "header", "State" }
        }
        ul {
            li {
                a { href: "reference/hooks",
                    code { "Hooks" }
                }
                " Hooks allow you to create components state"
            }
            li {
                a { href: "reference/context",
                    code { "Context" }
                }
                " Context allows you to create state in a parent and consume it in children"
            }
            li {
                a { href: "reference/router",
                    code { "Routing" }
                }
                " The router helps you manage the URL state"
            }
            li {
                a { href: "reference/use_resource",
                    code { "Resource" }
                }
                " Use future allows you to create an async task and monitor it's state"
            }
            li {
                a { href: "reference/use_coroutine",
                    code { "UseCoroutine" }
                }
                " Use coroutine helps you manage external state"
            }
            li {
                a { href: "reference/spawn",
                    code { "Spawn" }
                }
                " Spawn creates an async task"
            }
        }
        h2 { id: "platforms",
            a { href: "#platforms", class: "header", "Platforms" }
        }
        ul {
            li {
                a { href: "reference/choosing_a_web_renderer",
                    code { "Choosing a Web Renderer" }
                }
                " Overview of the different web renderers"
            }
            li {
                a { href: "reference/desktop",
                    code { "Desktop" }
                }
                " Overview of desktop specific APIS"
            }
            li {
                a { href: "reference/web",
                    code { "Web" }
                }
                " Overview of web specific APIS"
            }
            li {
                a { href: "reference/fullstack",
                    code { "Fullstack" }
                }
                " Overview of Fullstack specific APIS"
                ul {
                    li {
                        a { href: "reference/fullstack/server_functions",
                            code { "Server Functions" }
                        }
                        " Server functions make it easy to communicate between your server and client"
                    }
                    li {
                        a { href: "reference/fullstack/extractors",
                            code { "Extractors" }
                        }
                        " Extractors allow you to get extra information out of the headers of a request"
                    }
                    li {
                        a { href: "reference/fullstack/middleware",
                            code { "Middleware" }
                        }
                        " Middleware allows you to wrap a server function request or response"
                    }
                    li {
                        a { href: "reference/fullstack/authentication",
                            code { "Authentication" }
                        }
                        " An overview of how to handle authentication with server functions"
                    }
                    li {
                        a { href: "reference/fullstack/routing",
                            code { "Routing" }
                        }
                        " An overview of how to work with the router in the fullstack renderer"
                    }
                }
            }
            li {
                a { href: "reference/ssr",
                    code { "SSR" }
                }
                " Overview of the SSR renderer"
            }
            li {
                a { href: "reference/liveview",
                    code { "Liveview" }
                }
                " Overview of liveview specific APIS"
            }
        }
    }
}
#[component(no_case_check)]
pub fn RouterIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "introduction",
            a { href: "#introduction", class: "header", "Introduction" }
        }
        blockquote {
            p {
                "If you are not familiar with Dioxus itself, check out the "
                a { href: "router/../guide", "Dioxus guide" }
                " first."
            }
        }
        p {
            "Whether you are building a website, desktop app, or mobile app, splitting your app's views into \"pages\" can be an effective method for organization and maintainability."
        }
        p {
            "For this purpose, Dioxus provides a router. Use the  "
            code { "cargo add" }
            " command to add the dependency:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ff80f4;\">0.5</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features router</span></pre>\n" }
        p {
            "Then, add this to your  "
            code { "Dioxus.toml" }
            " (learn more about configuration "
            a { href: "router/../CLI/configure", "here" }
            "):"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[web.watcher]\n</span><span style=\"color:#f8f8f2;\">index_on_404 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true</span></pre>\n" }
        blockquote {
            p {
                "This configuration only works when using  "
                code { "dx serve" }
                ". If you host your app in a different way (which you most likely do in production), you need to find out how to add a fallback 404 page to your app, and make it a copy of the generated  "
                code { "dist/index.html" }
                "."
            }
        }
        p {
            "This will instruct  "
            code { "dx serve" }
            " to redirect any unknown route to the index, to then be resolved by the router."
            code { "localhost:8080" }
            ", then click a link to go to "
            code { "localhost:8080/contact" }
            "), the app renders the new route without reloading."
            em { "before" }
            " going to the index (go straight to "
            code { "localhost:8080/contact" }
            "), we are trying to access a static route from the server, but the only static route on our server is the index (because the Dioxus frontend is a Single Page Application) and it will fail unless we redirect all missing routes to the index."
        }
        p { "This book is intended to get you up to speed with Dioxus Router. It is split" }
        ol {
            li {
                "The "
                a { href: "router/reference", "reference" }
                " section explains individual features in "
            }
            li {
                "If you prefer a learning-by-doing approach, you can check out the "
                em {
                    a { href: "router/example", "example project" }
                }
                ". It guides you through "
            }
        }
        blockquote {
            p {
                "Please note that this is not the only documentation for the Dioxus Router. You"
                a { href: "https://docs.rs/dioxus-router/", "API Docs" }
                "."
            }
        }
    }
}
#[component(no_case_check)]
pub fn RouterReferenceRoutingUpdateCallback() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "routing-update-callback",
            a { href: "#routing-update-callback", class: "header", "Routing Update Callback" }
        }
        p {
            "In some cases, we might want to run custom code when the current route changes. For this reason, the "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/prelude/struct.RouterConfig.html",
                code { "RouterConfig" }
            }
            " exposes an "
            code { "on_update" }
            " field."
        }
        h2 { id: "how-does-the-callback-behave",
            a { href: "#how-does-the-callback-behave", class: "header",
                "How does the callback behave?"
            }
        }
        p {
            "The  "
            code { "on_update" }
            " is called whenever the current routing information changes. It is called after the router updated its internal state, but before dependent components and hooks are updated."
        }
        p {
            "If the callback returns a "
            a { href: "https://docs.rs/dioxus-router/latest/dioxus_router/navigation/enum.NavigationTarget.html",
                code { "NavigationTarget" }
            }
            ", the router will replace the current location with the specified target. It will not call the "
            code { "on_update" }
            " again."
        }
        p {
            "If at any point the router encounters a navigation failure, it will go to the appropriate state without calling the  "
            code { "on_update" }
            ". It doesn't matter if the invalid target initiated the navigation, was found as a redirect target, or was returned by the  "
            code { "on_update" }
            " itself."
        }
        h2 { id: "code-example",
            a { href: "#code-example", class: "header", "Code Example" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Routable, Clone, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Index {{}},\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/home&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ p {{ </span><span style=\"color:#ffee99;\">&quot;Home&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Index</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ p {{ </span><span style=\"color:#ffee99;\">&quot;Index&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Router::&lt;Route&gt; {{\n</span><span style=\"color:#f8f8f2;\">            config: || {{\n</span><span style=\"color:#f8f8f2;\">                RouterConfig::default()\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">on_update</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">| {{\n</span><span style=\"color:#f8f8f2;\">                        (state.</span><span style=\"color:#66d9ef;\">current</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">== </span><span style=\"color:#f8f8f2;\">Route::Index {{}})\n</span><span style=\"color:#f8f8f2;\">                            .</span><span style=\"color:#66d9ef;\">then_some</span><span style=\"color:#f8f8f2;\">(NavigationTarget::Internal(Route::Home {{}}))\n</span><span style=\"color:#f8f8f2;\">                    }})\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "routing_update.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceAssets() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "assets",
            a { href: "#assets", class: "header", "Assets" }
        }
        blockquote {
            p {
                "⚠\u{fe0f} Support: Manganis is currently in alpha. API changes are planned and bugs are more likely"
            }
        }
        p {
            "Assets are files that are included in the final build of the application. They can be images, fonts, stylesheets, or any other file that is not a source file. Dioxus includes first class support for assets, and provides a simple way to include them in your application and automatically optimize them for production."
        }
        p {
            "Assets in dioxus are also compatible with libraries! If you are building a library, you can include assets in your library and they will be automatically included in the final build of any application that uses your library."
        }
        p {
            "First, you need to add the  "
            code { "manganis" }
            " crate to your  "
            code { "Cargo.toml" }
            " file:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add manganis</span></pre>\n" }
        h2 { id: "including-images",
            a { href: "#including-images", class: "header", "Including images" }
        }
        p {
            "To include an asset in your application, you can simply wrap the path to the asset in a  "
            code { "mg!" }
            " call. For example, to include an image in your application, you can use the following code:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You can link to assets that are relative to the package root or even link to an asset from a url\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// These assets will automatically be picked up by the dioxus cli, optimized, and bundled with your final applications\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">ASSET</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/static/ferrous_wave.png&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        img {{ src: </span><span style=\"color:#ffee99;\">&quot;{{ASSET}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        p {
            "You can also optimize, resize, and preload images using the  "
            code { "mg!" }
            " macro. Choosing an optimized file type (like WebP) and a reasonable quality setting can significantly reduce the size of your images which helps your application load faster. For example, you can use the following code to include an optimized image in your application:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">ENUM_ROUTER_IMG</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/static/enum_router.png&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">EnumRouter</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        img {{ src: </span><span style=\"color:#ffee99;\">&quot;{{ENUM_ROUTER_IMG}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        h2 { id: "including-arbitrary-files",
            a { href: "#including-arbitrary-files", class: "header", "Including arbitrary files" }
        }
        p {
            "In dioxus desktop, you may want to include a file with data for your application. You can use the  "
            code { "file" }
            " function to include arbitrary files in your application. For example, you can use the following code to include a file in your application:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can also collect arbitrary files. Relative paths are resolved relative to the package root\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">PATH_TO_BUNDLED_CARGO_TOML</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/Cargo.toml&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        p {
            "These files will be automatically included in the final build of your application, and you can use them in your application as you would any other file."
        }
        h2 { id: "including-stylesheets",
            a { href: "#including-stylesheets", class: "header", "Including stylesheets" }
        }
        p {
            "You can include stylesheets in your application using the  "
            code { "mg!" }
            " macro. For example, you can use the following code to include a stylesheet in your application:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can also bundle stylesheets with your application\n</span><span style=\"color:#8c8c8c;\">// Any files that end with .css will be minified and bundled with your application even if you don&#39;t explicitly include them in your &lt;head&gt;\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/tailwind.css&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "assets.rs".to_string(),
        }
        blockquote {
            p {
                "The "
                a { href: "../cookbook/tailwind", "tailwind guide" }
                " has more information on how to use tailwind with dioxus."
            }
        }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "Dioxus provides first class support for assets, and makes it easy to include them in your application. You can include images, arbitrary files, and stylesheets in your application, and dioxus will automatically optimize them for production. This makes it easy to include assets in your application and ensure that they are optimized for production."
        }
        p {
            "You can read more about assets and all the options available to optimize your assets in the "
            a { href: "https://docs.rs/manganis/0.2.2/manganis/", "manganis documentation" }
            "."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceWebIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "web",
            a { href: "#web", class: "header", "Web" }
        }
        p {
            "To run on the Web, your app must be compiled to WebAssembly and depend on the  "
            code { "dioxus" }
            " and  "
            code { "dioxus-web" }
            " crates."
        }
        p {
            "A build of Dioxus for the web will be roughly equivalent to the size of a React build (70kb vs 65kb) but it will load significantly faster because "
            a { href: "https://hacks.mozilla.org/2018/01/making-webassembly-even-faster-firefoxs-new-streaming-and-tiering-compiler/",
                "WebAssembly can be compiled as it is streamed"
            }
            "."
        }
        p { "Examples:" }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs",
                    "TodoMVC"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind",
                    "Tailwind App"
                }
            }
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs",
                img {
                    src: "https://github.com/DioxusLabs/example-projects/raw/master/todomvc/example.png",
                    alt: "TodoMVC example",
                    title: "",
                }
            }
        }
        blockquote {
            p {
                "Note: Because of the limitations of Wasm, "
                a { href: "https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html",
                    "not every crate will work"
                }
                " with your web apps, so you'll need to make sure that your crates work without native system calls (timers, IO, etc)."
            }
        }
        h2 { id: "support",
            a { href: "#support", class: "header", "Support" }
        }
        p { "The Web is the best-supported target platform for Dioxus." }
        ul {
            li {
                "Because your app will be compiled to WASM you have access to browser APIs through "
                a { href: "https://rustwasm.github.io/docs/wasm-bindgen/introduction.html",
                    "wasm-bindgen"
                }
                "."
            }
            li {
                "Dioxus provides hydration to resume apps that are rendered on the server. See the "
                a { href: "web/../fullstack", "fullstack" }
                " reference for more information."
            }
        }
        h2 { id: "running-javascript",
            a { href: "#running-javascript", class: "header", "Running Javascript" }
        }
        p {
            "Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose."
        }
        p {
            "For these cases, Dioxus web exposes the use_eval hook that allows you to run raw Javascript in the webview:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can create as many eval instances as you want\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">document::eval(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">            // You can send messages from JavaScript to Rust with the dioxus.send function\n</span><span style=\"color:#ffee99;\">            dioxus.send(&quot;Hi from JS!&quot;);\n</span><span style=\"color:#ffee99;\">            // You can receive messages from Rust to JavaScript with the dioxus.recv function\n</span><span style=\"color:#ffee99;\">            let msg = await dioxus.recv();\n</span><span style=\"color:#ffee99;\">            console.log(msg);\n</span><span style=\"color:#ffee99;\">            &quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can send messages to JavaScript with the send method\n</span><span style=\"color:#f8f8f2;\">        eval.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hi from Rust!&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can receive any message from JavaScript with the recv method\n</span><span style=\"color:#f8f8f2;\">        eval.recv::&lt;String&gt;().await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(v) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;{{v}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;hello&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        p {
            "If you are targeting web, but don't plan on targeting any other Dioxus renderer you can also use the generated wrappers in the "
            a { href: "https://rustwasm.github.io/wasm-bindgen/web-sys/index.html",
                "web-sys"
            }
            " and "
            a { href: "https://gloo-rs.web.app/", "gloo" }
            " crates."
        }
        h2 { id: "customizing-index-template",
            a { href: "#customizing-index-template", class: "header", "Customizing Index Template" }
        }
        p {
            "Dioxus supports providing custom index.html templates. The index.html must include a  "
            code { "div" }
            " with the id  "
            code { "main" }
            " to be used. Hot Reload is still supported. An example"
            a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/PWA-example/index.html",
                "PWA-Example"
            }
            "."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceDesktopIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "desktop",
            a { href: "#desktop", class: "header", "Desktop" }
        }
        p { "This guide will cover concepts specific to the Dioxus desktop renderer." }
        p {
            "Apps built with Dioxus desktop use the system WebView to render the page. This makes the final size of application much smaller than other WebView renderers (typically under 5MB)."
        }
        p {
            "Although desktop apps are rendered in a WebView, your Rust code runs natively. This means that browser APIs are "
            em { "not" }
            " available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs "
            em { "are" }
            " accessible, so streaming, WebSockets, filesystem, etc are all easily accessible though system APIs."
        }
        p {
            "Dioxus desktop is built off "
            a { href: "https://tauri.app/", "Tauri" }
            ". Right now there are limited Dioxus abstractions over the menubar, event handling, etc. In some places you may need to leverage Tauri directly – through "
            a { href: "http://github.com/tauri-apps/wry/", "Wry" }
            " and "
            a { href: "http://github.com/tauri-apps/tao", "Tao" }
            "."
        }
        blockquote {
            p {
                "In the future, we plan to move to a custom web renderer-based DOM renderer with WGPU integrations ("
                a { href: "https://github.com/DioxusLabs/blitz", "Blitz" }
                ")."
            }
        }
        h2 { id: "examples",
            a { href: "#examples", class: "header", "Examples" }
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/blob/main/examples/file_explorer.rs",
                    "File Explorer"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind",
                    "Tailwind App"
                }
            }
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind",
                img {
                    src: "/assets/static/tailwind_desktop_app.png",
                    alt: "Tailwind App screenshot",
                    title: "",
                }
            }
        }
        h2 { id: "running-javascript",
            a { href: "#running-javascript", class: "header", "Running Javascript" }
        }
        p {
            "Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose."
        }
        p {
            "For these cases, Dioxus desktop exposes the use_eval hook that allows you to run raw Javascript in the webview:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can create as many eval instances as you want\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">document::eval(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">            // You can send messages from JavaScript to Rust with the dioxus.send function\n</span><span style=\"color:#ffee99;\">            dioxus.send(&quot;Hi from JS!&quot;);\n</span><span style=\"color:#ffee99;\">            // You can receive messages from Rust to JavaScript with the dioxus.recv function\n</span><span style=\"color:#ffee99;\">            let msg = await dioxus.recv();\n</span><span style=\"color:#ffee99;\">            console.log(msg);\n</span><span style=\"color:#ffee99;\">            &quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can send messages to JavaScript with the send method\n</span><span style=\"color:#f8f8f2;\">        eval.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hi from Rust!&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can receive any message from JavaScript with the recv method\n</span><span style=\"color:#f8f8f2;\">        eval.recv::&lt;String&gt;().await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(v) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;{{v}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;hello&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        h2 { id: "custom-assets",
            a { href: "#custom-assets", class: "header", "Custom Assets" }
        }
        p { "You can link to local assets in dioxus desktop instead of using a url:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            img {{ src: </span><span style=\"color:#ffee99;\">&quot;public/static/scanner.png&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        p {
            "You can read more about assets in the "
            a { href: "desktop/./assets", "assets" }
            " reference."
        }
        h2 { id: "integrating-with-wry",
            a { href: "#integrating-with-wry", class: "header", "Integrating with Wry" }
        }
        p {
            "In cases where you need more low level control over your window, you can use wry APIs exposed through the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/struct.Config.html",
                "Desktop Config"
            }
            " and the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/fn.use_window.html",
                "use_window hook"
            }
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceMobileIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "mobile-app",
            a { href: "#mobile-app", class: "header", "Mobile App" }
        }
        p { "Build a mobile app with Dioxus!" }
        p {
            "Example: "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples/mobile_demo",
                "Mobile Demo"
            }
        }
        h2 { id: "support",
            a { href: "#support", class: "header", "Support" }
        }
        p {
            "Mobile is currently the least-supported renderer target for Dioxus. Mobile apps are rendered with either the platform's WebView or experimentally with "
            a { href: "https://github.com/DioxusLabs/blitz", "WGPU" }
            ". WebView doesn't support animations, transparency, and native widgets."
        }
        p {
            "Mobile support is currently best suited for CRUD-style apps, ideally for internal teams who need to develop quickly but don't care much about animations or native widgets."
        }
        h2 { id: "getting-set-up",
            a { href: "#getting-set-up", class: "header", "Getting Set up" }
        }
        p {
            "Getting set up with mobile can be quite challenging. The tooling here isn't great (yet) and might take some hacking around to get things working."
        }
        h3 { id: "setting-up-dependencies",
            a { href: "#setting-up-dependencies", class: "header", "Setting up dependencies" }
        }
        h4 { id: "android",
            a { href: "#android", class: "header", "Android" }
        }
        p { "First, install the rust Android targets:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rustup target add aarch64</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">android armv7</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">androideabi i686</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">android x86_64</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">linux</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">android</span></pre>\n" }
        p {
            "To develop on Android, you will need to "
            a { href: "https://developer.android.com/studio", "install Android Studio" }
            "."
        }
        p { "Once you have installed Android Studio, you will need to install the Android SDK and NDK:" }
        ol {
            li { "Create a blank Android project" }
            li {
                "Select "
                code { "Tools > SDK manager" }
            }
            li {
                "Navigate to the "
                code { "SDK tools" }
                " window:"
            }
        }
        p {
            img {
                src: "/assets/static/android_ndk_install.png",
                alt: "NDK install window",
                title: "",
            }
        }
        p { "Then select:" }
        ul {
            li { "The SDK" }
            li { "The SDK Command line tools" }
            li { "The NDK (side by side)" }
            li { "CMAKE" }
        }
        ol {
            li {
                "Select "
                code { "apply" }
                " and follow the prompts"
            }
        }
        blockquote {
            p {
                "More details that could be useful for debugging any errors you encounter are available "
                a { href: "https://developer.android.com/studio/intro/update#sdk-manager",
                    "in the official android docs"
                }
            }
        }
        p { "Next set the Java, Android and NDK home variables:" }
        p { "Mac:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">export </span><span style=\"color:#ff80f4;\">JAVA_HOME</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/Applications/Android Studio.app/Contents/jbr/Contents/Home&quot;\n</span><span style=\"color:#f8f8f2;\">export </span><span style=\"color:#ff80f4;\">ANDROID_HOME</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;$HOME/Library/Android/sdk&quot;\n</span><span style=\"color:#f8f8f2;\">export </span><span style=\"color:#ff80f4;\">NDK_HOME</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;$ANDROID_HOME/ndk/25.2.9519653&quot;</span></pre>\n" }
        p { "Windows:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[System.Environment]::SetEnvironmentVariable(</span><span style=\"color:#ffee99;\">&quot;JAVA_HOME&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;C:\\Program Files\\Android\\Android Studio\\jbr&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;User&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">[System.Environment]::SetEnvironmentVariable(</span><span style=\"color:#ffee99;\">&quot;ANDROID_HOME&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;$env:LocalAppData\\Android\\Sdk&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;User&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">[System.Environment]::SetEnvironmentVariable(</span><span style=\"color:#ffee99;\">&quot;NDK_HOME&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;$env:LocalAppData\\Android\\Sdk</span><span style=\"color:#ff80f4;\">\\n</span><span style=\"color:#ffee99;\">dk\\25.2.9519653&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;User&quot;</span><span style=\"color:#f8f8f2;\">)</span></pre>\n",
        }
        blockquote {
            p { "The NDK version in the paths should match the version you installed in the last step" }
        }
        h4 { id: "ios",
            a { href: "#ios", class: "header", "IOS" }
        }
        p {
            "To develop on IOS, you will need to "
            a { href: "https://apps.apple.com/us/app/xcode/id497799835", "install XCode" }
            "."
        }
        blockquote {
            p {
                "If you are using M1, you will have to run  "
                code { "cargo build --target x86_64-apple-ios" }
                " instead of  "
                code { "cargo apple build" }
                " if you want to run in simulator."
            }
        }
        h3 { id: "setting-up-your-project",
            a { href: "#setting-up-your-project", class: "header", "Setting up your project" }
        }
        p { "First, we need to create a rust project:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">mobile</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">test </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">lib\n</span><span style=\"color:#f8f8f2;\">cd dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">mobile</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">test</span></pre>\n" }
        p {
            "Next, we can use  "
            code { "cargo-mobile2" }
            " to create a project for mobile:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">git https:</span><span style=\"color:#8c8c8c;\">//github.com/tauri-apps/cargo-mobile2\n</span><span style=\"color:#f8f8f2;\">cargo mobile init</span></pre>\n" }
        p {
            "When you run  "
            code { "cargo mobile init" }
            ", you will be asked a series of questions about your project. One of those questions is what template you should use. Dioxus currently doesn't have a template in Tauri mobile, instead you can use the  "
            code { "wry" }
            " template."
        }
        blockquote {
            p {
                "You may also be asked to input your team ID for IOS. You can find your team id "
                a { href: "https://developer.apple.com/help/account/manage-your-team/locate-your-team-id/",
                    "here"
                }
                " or create a team id by creating a developer account "
                a { href: "https://developer.apple.com/help/account/get-started/about-your-developer-account",
                    "here"
                }
            }
        }
        p {
            "Next, we need to modify our dependencies to include dioxus and ensure the right version of wry is installed. Change the  "
            code { "[dependencies]" }
            " section of your  "
            code { "Cargo.toml" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">anyhow </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.0&quot;\n</span><span style=\"color:#f8f8f2;\">log </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.4&quot;\n</span><span style=\"color:#f8f8f2;\">wry </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.37&quot;\n</span><span style=\"color:#f8f8f2;\">tao </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.26&quot;\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.6&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;mobile&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n",
        }
        p {
            "Finally, we need to add a component to renderer. Replace the wry template in your  "
            code { "lib.rs" }
            " file with this code:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">anyhow::Result;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">init_logging</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    android_logger::init_once(\n</span><span style=\"color:#f8f8f2;\">        android_logger::Config::default()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">with_max_level</span><span style=\"color:#f8f8f2;\">(log::LevelFilter::Trace)\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[cfg(not(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">init_logging</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    env_logger::init();\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[cfg(any(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">, target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">stop_unwind</span><span style=\"color:#f8f8f2;\">&lt;F: FnOnce() -&gt; T, T&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">f</span><span style=\"color:#f8f8f2;\">: F) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#f8f8f2;\">std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(t) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> t,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            eprintln!(</span><span style=\"color:#ffee99;\">&quot;attempt to unwind out of `rust` with err: {{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, err);\n</span><span style=\"color:#f8f8f2;\">            std::process::abort()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[no_mangle]\n</span><span style=\"color:#f8f8f2;\">#[inline(never)]\n</span><span style=\"color:#f8f8f2;\">#[cfg(any(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">, target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">))]\n</span><span style=\"color:#f92672;\">pub extern </span><span style=\"color:#ffee99;\">&quot;C&quot; </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">start_app</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">_start_app</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">stop_unwind</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#66d9ef;\">main</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    #[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;android&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    {{\n</span><span style=\"color:#f8f8f2;\">        tao::android_binding</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">            com_example,\n</span><span style=\"color:#f8f8f2;\">            dioxus_mobile_test,\n</span><span style=\"color:#f8f8f2;\">            WryActivity,\n</span><span style=\"color:#f8f8f2;\">            wry::android_setup, </span><span style=\"color:#8c8c8c;\">// pass the wry::android_setup function to tao which will invoke when the event loop is created\n</span><span style=\"color:#f8f8f2;\">            _start_app\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">        wry::android_binding</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(com_example, dioxus_mobile_test);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    #[cfg(target_os </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;ios&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">_start_app</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;()&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">init_logging</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f8f8f2;\">]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    log::debug</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the app&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;Hello, Mobile&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ margin_left: </span><span style=\"color:#ffee99;\">&quot;auto&quot;</span><span style=\"color:#f8f8f2;\">, margin_right: </span><span style=\"color:#ffee99;\">&quot;auto&quot;</span><span style=\"color:#f8f8f2;\">, width: </span><span style=\"color:#ffee99;\">&quot;200px&quot;</span><span style=\"color:#f8f8f2;\">, padding: </span><span style=\"color:#ffee99;\">&quot;10px&quot;</span><span style=\"color:#f8f8f2;\">, border: </span><span style=\"color:#ffee99;\">&quot;1px solid black&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">                button {{\n</span><span style=\"color:#f8f8f2;\">                    onclick: </span><span style=\"color:#f92672;\">move|_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        println!(</span><span style=\"color:#ffee99;\">&quot;Clicked!&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> items_mut </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new_item </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> items_mut.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">                        items_mut.</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(new_item);\n</span><span style=\"color:#f8f8f2;\">                        println!(</span><span style=\"color:#ffee99;\">&quot;Requested update&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                    }},\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#ffee99;\">&quot;Add item&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> items.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                    div {{ </span><span style=\"color:#ffee99;\">&quot;- {{item}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "running",
            a { href: "#running", class: "header", "Running" }
        }
        p {
            "From there, you'll want to get a build of the crate using whichever platform you're targeting (simulator or actual hardware). For now, we'll just stick with the simulator."
        }
        p { "First, you need to make sure that the build variant is correct in Android Studio:" }
        ol {
            li { "Click \"Build\" in the top menu bar." }
            li { "Click \"Select Build Variant...\" in the dropdown." }
            li {
                "Find the \"Build Variants\" panel and use the dropdown to change the selected build variant."
            }
        }
        p {
            img {
                src: "/assets/static/as-build-dropdown.png",
                alt: "android studio build dropdown",
                title: "",
            }
            img {
                src: "/assets/static/as-build-variant-menu.png",
                alt: "android studio build variants",
                title: "",
            }
        }
        h3 { id: "android",
            a { href: "#android", class: "header", "Android" }
        }
        p { "To build your project on Android you can run:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo android build</span></pre>\n" }
        p { "Next, open Android studio:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo android open</span></pre>\n" }
        p { "This will open an android studio project for this application." }
        p {
            "Next we need to create a simulator in Android studio to run our app in. To create a simulator click on the phone icon in the top right of Android studio:"
        }
        p {
            img {
                src: "/assets/static/android-studio-simulator.png",
                alt: "android studio manage devices",
                title: "",
            }
        }
        p {
            "Then click the  "
            code { "create a virtual device" }
            " button and follow the prompts:"
        }
        p {
            img {
                src: "/assets/static/android-studio-devices.png",
                alt: "android studio devices",
                title: "",
            }
        }
        p { "Finally, launch your device by clicking the play button on the device you created:" }
        p {
            img {
                src: "/assets/static/android-studio-device.png",
                alt: "android studio device",
                title: "",
            }
        }
        p { "Now you can start your application from your terminal by running:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo android run</span></pre>\n" }
        p {
            img {
                src: "/assets/static/Android-Dioxus-demo.png",
                alt: "android_demo",
                title: "",
            }
        }
        blockquote {
            p { "More information is available in the Android docs:" }
            ul {
                li { "https://developer.android.com/ndk/guides" }
                li { "https://developer.android.com/studio/projects/install-ndk" }
                li { "https://source.android.com/docs/setup/build/rust/building-rust-modules/overview" }
            }
        }
        h3 { id: "ios",
            a { href: "#ios", class: "header", "IOS" }
        }
        p { "To build your project for IOS, you can run:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">target aarch64</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">apple</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ios</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">sim</span></pre>\n" }
        p { "Next, open XCode (this might take awhile if you've never opened XCode before):" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo apple open</span></pre>\n" }
        p { "This will open XCode with this particular project." }
        p {
            "From there, just click the \"play\" button with the right target and the app should be running!"
        }
        p {
            img {
                src: "/assets/static/IOS-dioxus-demo.png",
                alt: "ios_demo",
                title: "",
            }
        }
        p {
            "Note that clicking play doesn't cause a new build, so you'll need to keep rebuilding the app between changes. The tooling here is very young, so please be patient. If you want to contribute to make things easier, please do! We'll be happy to help."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceMobileApis() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "mobile",
            a { href: "#mobile", class: "header", "Mobile" }
        }
        p { "This guide will cover concepts specific to the Dioxus mobile renderer." }
        h2 { id: "running-javascript",
            a { href: "#running-javascript", class: "header", "Running Javascript" }
        }
        p {
            "Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose."
        }
        p {
            "For these cases, Dioxus desktop exposes the use_eval hook that allows you to run raw Javascript in the webview:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> future </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can create as many eval instances as you want\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> eval </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">document::eval(\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;\n</span><span style=\"color:#ffee99;\">            // You can send messages from JavaScript to Rust with the dioxus.send function\n</span><span style=\"color:#ffee99;\">            dioxus.send(&quot;Hi from JS!&quot;);\n</span><span style=\"color:#ffee99;\">            // You can receive messages from Rust to JavaScript with the dioxus.recv function\n</span><span style=\"color:#ffee99;\">            let msg = await dioxus.recv();\n</span><span style=\"color:#ffee99;\">            console.log(msg);\n</span><span style=\"color:#ffee99;\">            &quot;#</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can send messages to JavaScript with the send method\n</span><span style=\"color:#f8f8f2;\">        eval.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hi from Rust!&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can receive any message from JavaScript with the recv method\n</span><span style=\"color:#f8f8f2;\">        eval.recv::&lt;String&gt;().await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> future.</span><span style=\"color:#66d9ef;\">read_unchecked</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">as_ref</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(v) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;{{v}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            p {{ </span><span style=\"color:#ffee99;\">&quot;hello&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        h2 { id: "custom-assets",
            a { href: "#custom-assets", class: "header", "Custom Assets" }
        }
        p { "You can link to local assets in dioxus mobile instead of using a url:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            img {{ src: </span><span style=\"color:#ffee99;\">&quot;public/static/scanner.png&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "eval.rs".to_string(),
        }
        h2 { id: "integrating-with-wry",
            a { href: "#integrating-with-wry", class: "header", "Integrating with Wry" }
        }
        p {
            "In cases where you need more low level control over your window, you can use wry APIs exposed through the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/struct.Config.html",
                "Desktop Config"
            }
            " and the "
            a { href: "https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/struct.DesktopContext.html",
                "use_window hook"
            }
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceSsr() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "server-side-rendering",
            a { href: "#server-side-rendering", class: "header", "Server-Side Rendering" }
        }
        p {
            "For lower-level control over the rendering process, you can use the  "
            code { "dioxus-ssr" }
            " crate directly. This can be useful when integrating with a web framework that  "
            code { "dioxus-fullstack" }
            " does not support, or pre-rendering pages."
        }
        h2 { id: "setup",
            a { href: "#setup", class: "header", "Setup" }
        }
        p {
            "For this guide, we're going to show how to use Dioxus SSR with "
            a { href: "https://docs.rs/axum/latest/axum/", "Axum" }
            "."
        }
        p { "Make sure you have Rust and Cargo installed, and then create a new project:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo\n</span><span style=\"color:#f8f8f2;\">cd demo</span></pre>\n" }
        p { "Add Dioxus and the ssr renderer as dependencies:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ff80f4;\">0.5</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">0\n</span><span style=\"color:#f8f8f2;\">cargo add dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ssr</span><span style=\"color:#f92672;\">@</span><span style=\"color:#ff80f4;\">0.5</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">0</span></pre>\n" }
        p {
            "Next, add all the Axum dependencies. This will be different if you're using a different Web Framework"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add tokio </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features full\n</span><span style=\"color:#f8f8f2;\">cargo add axum</span></pre>\n" }
        p { "Your dependencies should look roughly like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">axum </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.7&quot;\n</span><span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">ssr </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;*&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">tokio </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.15.0&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;full&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n",
        }
        p { "Now, set up your Axum app to respond on an endpoint." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">axum::{{response::Html, routing::get, Router}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> listener </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">tokio::net::TcpListener::bind(</span><span style=\"color:#ffee99;\">&quot;127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .await\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;listening on http://127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    axum::serve(\n</span><span style=\"color:#f8f8f2;\">        listener,\n</span><span style=\"color:#f8f8f2;\">        Router::new()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">route</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(app_endpoint))\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">into_make_service</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    )\n</span><span style=\"color:#f8f8f2;\">    .await\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "ssr.rs".to_string(),
        }
        p {
            "And then add our endpoint. We can either render  "
            code { "rsx!" }
            " directly:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app_endpoint</span><span style=\"color:#f8f8f2;\">() -&gt; Html&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// render the rsx! macro to HTML\n</span><span style=\"color:#f8f8f2;\">    Html(dioxus_ssr::render_element(rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;hello world!&quot; </span><span style=\"color:#f8f8f2;\">}} }}))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "ssr.rs".to_string(),
        }
        p { "Or we can render VirtualDoms." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app_endpoint</span><span style=\"color:#f8f8f2;\">() -&gt; Html&lt;String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// create a component that renders a div with the text &quot;hello world&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;hello world&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// create a VirtualDom with the app component\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> app </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">VirtualDom::new(app);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// rebuild the VirtualDom before rendering\n</span><span style=\"color:#f8f8f2;\">    app.</span><span style=\"color:#66d9ef;\">rebuild_in_place</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// render the VirtualDom to HTML\n</span><span style=\"color:#f8f8f2;\">    Html(dioxus_ssr::render(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">app))\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "ssr.rs".to_string(),
        }
        p {
            "Finally, you can run it using  "
            code { "cargo run" }
            " rather than  "
            code { "dx serve" }
            "."
        }
        h2 { id: "multithreaded-support",
            a { href: "#multithreaded-support", class: "header", "Multithreaded Support" }
        }
        p {
            "The Dioxus VirtualDom, sadly, is not currently  "
            code { "Send" }
            ". Internally, we use quite a bit of interior mutability which is not thread-safe."
            code { "Send" }
            ", it is possible to render a VirtualDom immediately to a String – but you cannot hold the VirtualDom across an await point. For retained-state SSR (essentially LiveView), you'll need to spawn a VirtualDom on its own thread and communicate with it via channels or create a pool of VirtualDoms."
            em { "must" }
            " remain on the thread it started. We are working on loosening this requirement."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "fullstack-development",
            a { href: "#fullstack-development", class: "header", "Fullstack development" }
        }
        p { "Dioxus Fullstack contains helpers for:" }
        ul {
            li { "Incremental, static, and server side rendering" }
            li { "Hydrating your application on the Client" }
            li { "Communicating between a server and a client" }
        }
        p {
            "This guide will teach you everything you need to know about how to use the utilities in Dioxus fullstack to create amazing fullstack applications."
        }
        blockquote {
            p {
                "In addition to this guide, you can find more examples of full-stack apps and information about how to integrate with other frameworks and desktop renderers in the "
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples",
                    "dioxus-fullstack examples directory"
                }
                "."
            }
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackServerFunctions() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "communicating-with-the-server",
            a { href: "#communicating-with-the-server", class: "header",
                "Communicating with the server"
            }
        }
        p {
            code { "dioxus-fullstack" }
            " provides server functions that allow you to call an automatically generated API on the server from the client as if it were a local function."
        }
        p {
            "To make a server function, simply add the  "
            code { "#[server(YourUniqueType)]" }
            " attribute to a function. The function must:"
        }
        ul {
            li { "Be an async function" }
            li {
                "Have arguments and a return type that both implement serialize and deserialize (with "
                a { href: "https://serde.rs/", "serde" }
                ")."
            }
            li {
                "Return a "
                code { "Result" }
                " with an error type of ServerFnError"
            }
        }
        p {
            "You must call  "
            code { "register" }
            " on the type you passed into the server macro in your main function before starting your server to tell Dioxus about the server function."
        }
        p {
            "Let's continue building on the app we made in the "
            a { href: "../../getting_started/fullstack", "getting started" }
            " guide. We will add a server function to our app that allows us to double the count on the server."
        }
        p { "First, add serde as a dependency:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add serde</span></pre>\n" }
        p {
            "Next, add the server function to your  "
            code { "main.rs" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(App)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Up high!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Down low!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(new_count) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">double_server</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">()).await {{\n</span><span style=\"color:#f8f8f2;\">                        count.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(new_count);\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Double&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">double_server</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">number</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Perform some expensive computation or access a database on the server\n</span><span style=\"color:#f8f8f2;\">    tokio::time::sleep(std::time::Duration::from_secs(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> result </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> number </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;server calculated </span><span style=\"color:#ff80f4;\">{{result}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(result)\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_function.rs".to_string(),
        }
        p {
            "Now, build your client-side bundle with  "
            code { "dx build --features web" }
            " and run your server with  "
            code { "cargo run --features ssr" }
            ". You should see a new button that multiplies the count by 2."
        }
        h2 { id: "cached-data-fetching",
            a { href: "#cached-data-fetching", class: "header", "Cached data fetching" }
        }
        p { "One common use case for server functions is fetching data from the server:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case, unused)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app)\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(get_server_data);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;server data is {{count.value():?}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Access a database\n</span><span style=\"color:#f8f8f2;\">    tokio::time::sleep(std::time::Duration::from_millis(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_function.rs".to_string(),
        }
        p {
            "If you navigate to the site above, you will first see  "
            code { "server data is None" }
            ", then after the  "
            code { "WASM" }
            " has loaded and the request to the server has finished, you will see  "
            code { "server data is Some(Ok(\"Hello from the server!\"))" }
            "."
        }
        p {
            "This approach works, but it can be slow. Instead of waiting for the client to load and send a request to the server, what if we could get all of the data we needed for the page on the server and send it down to the client with the initial HTML page?"
        }
        p {
            "This is exactly what the  "
            code { "use_server_future" }
            " hook allows us to do!  "
            code { "use_server_future" }
            " is similar to the  "
            code { "use_resource" }
            " hook, but it allows you to wait for a future on the server and send the result of the future down to the client."
        }
        p {
            "Let's change our data fetching to use  "
            code { "use_server_future" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case, unused)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_server_future</span><span style=\"color:#f8f8f2;\">(get_server_data)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;server data is {{count.value():?}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Access a database\n</span><span style=\"color:#f8f8f2;\">    tokio::time::sleep(std::time::Duration::from_millis(</span><span style=\"color:#ff80f4;\">100</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_function.rs".to_string(),
        }
        blockquote {
            p {
                "Notice the  "
                code { "?" }
                " after  "
                code { "use_server_future" }
                ". This is what tells Dioxus fullstack to wait for the future to resolve before continuing rendering. If you want to not wait for a specific future, you can just remove the ? and deal with the  "
                code { "Option" }
                " manually."
            }
        }
        p {
            "Now when you load the page, you should see  "
            code { "server data is Ok(\"Hello from the server!\")" }
            ". No need to wait for the  "
            code { "WASM" }
            " to load or wait for the request to finish!"
        }
        SandBoxFrame { url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-server-future-qwpp4p?file=/src/main.rs:3,24" }
        h2 { id: "running-the-client-with-dioxus-desktop",
            a {
                href: "#running-the-client-with-dioxus-desktop",
                class: "header",
                "Running the client with dioxus-desktop"
            }
        }
        p {
            "The project presented so far makes a web browser interact with the server, but it is also possible to make a desktop program interact with the server in a similar fashion. (The full example code is available in the "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples/axum-desktop",
                "Dioxus repo"
            }
            ")"
        }
        p {
            "First, we need to make two binary targets, one for the desktop program (the  "
            code { "client.rs" }
            " file), one for the server (the  "
            code { "server.rs" }
            " file). The client app and the server functions are written in a shared  "
            code { "lib.rs" }
            " file."
        }
        p {
            "The desktop and server targets have slightly different build configuration to enable additional dependencies or features. "
        }
        ul {
            li {
                "the client.rs has to be run with the "
                code { "desktop" }
                " feature, so that the optional "
                code { "dioxus-desktop" }
                " dependency is included"
            }
            li {
                "the server.rs has to be run with the "
                code { "ssr" }
                " features; this will generate the server part of the server functions and will run our backend server."
            }
        }
        p { "Once you create your project, you can run the server executable with:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo run </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin server </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features ssr</span></pre>\n" }
        p { "and the client desktop executable with:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo run </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin client </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features desktop</span></pre>\n" }
        h3 { id: "client-code",
            a { href: "#client-code", class: "header", "Client code" }
        }
        p {
            "The client file is pretty straightforward. You only need to set the server url in the client code, so it knows where to send the network requests. Then, dioxus_desktop launches the app."
        }
        p {
            "For development, the example project runs the server on  "
            code { "localhost:8080" }
            ". "
            strong { "Before you release remember to update the url to your production url." }
        }
        h3 { id: "server-code",
            a { href: "#server-code", class: "header", "Server code" }
        }
        p {
            "In the server code, first you have to set the network address and port where the server will listen to."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> listener </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">tokio::net::TcpListener::bind(</span><span style=\"color:#ffee99;\">&quot;127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">    .await\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;listening on http://127.0.0.1:3000&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "server_function.rs".to_string(),
        }
        p {
            "Then, you have to register the types declared in the server function macros into the server."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[server(GetServerData)]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_function.rs".to_string(),
        }
        p {
            "The  "
            code { "GetServerData" }
            " type has to be registered in the server, which will add the corresponding route to the server."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n</pre>\n",
            name: "server_function.rs".to_string(),
        }
        p { "Finally, the server is started and it begins responding to requests." }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackExtractors() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "extractors",
            a { href: "#extractors", class: "header", "Extractors" }
        }
        p {
            "Server functions are an ergonomic way to call a function on the server. Server function work by registering an endpoint on the server and using requests on the client. Most of the time, you shouldn't need to worry about how server functions operate, but there are some times when you need to get some value from the request other than the data passed in the server function."
        }
        p {
            "For example, requests contain information about the user's browser (called the "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent",
                "user agent"
            }
            "). We can use an extractor to retrieve that information."
        }
        p {
            "You can use the  "
            code { "extract" }
            " method within a server function to extract something from the request. You can extract any type that implements  "
            code { "FromServerContext" }
            " (or when axum is enabled, you can use axum extractors directly):"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// #[server]\n</span><span style=\"color:#8c8c8c;\">// pub async fn log_headers() -&gt; Result&lt;(), ServerFnError&gt; {{\n</span><span style=\"color:#8c8c8c;\">//     let headers: http::HeaderMap = extract().await?;\n</span><span style=\"color:#8c8c8c;\">//     log::info!(&quot;{{:?}}&quot;, headers[http::header::USER_AGENT]);\n</span><span style=\"color:#8c8c8c;\">//     Ok(())\n</span><span style=\"color:#8c8c8c;\">// }}</span></pre>\n",
            name: "server_function_extract.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackMiddleware() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "middleware",
            a { href: "#middleware", class: "header", "Middleware" }
        }
        p {
            "Extractors allow you to wrap your server function in some code that changes either the request or the response. Dioxus fullstack integrates with "
            a { href: "https://docs.rs/tower/latest/tower/index.html", "Tower" }
            " to allow you to wrap your server functions in middleware."
        }
        p {
            "You can use the  "
            code { "#[middleware]" }
            " attribute to add a layer of middleware to your server function. Let's add a timeout middleware to a server function. This middleware will stop running the server function if it reaches a certain timeout:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// #[server]\n</span><span style=\"color:#8c8c8c;\">// // Add a timeout middleware to the server function that will return an error if the function takes longer than 1 second to execute\n</span><span style=\"color:#8c8c8c;\">// #[middleware(tower_http::timeout::TimeoutLayer::new(std::time::Duration::from_secs(1)))]\n</span><span style=\"color:#8c8c8c;\">// pub async fn timeout() -&gt; Result&lt;(), ServerFnError&gt; {{\n</span><span style=\"color:#8c8c8c;\">//     tokio::time::sleep(std::time::Duration::from_secs(2)).await;\n</span><span style=\"color:#8c8c8c;\">//     Ok(())\n</span><span style=\"color:#8c8c8c;\">// }}</span></pre>\n",
            name: "server_function_middleware.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackAuthentication() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "authentication",
            a { href: "#authentication", class: "header", "Authentication" }
        }
        p {
            "You can use "
            a { href: "./extractors", "extractors" }
            " to integrate auth with your Fullstack application."
        }
        p {
            "You can create a custom extractors that extracts the auth session from the request. From that auth session, you can check if the user has the required privileges before returning the private data."
        }
        p {
            "A "
            a { href: "https://github.com/DioxusLabs/dioxus/blob/v0.5/packages/fullstack/examples/axum-auth/src/main.rs",
                "full auth example"
            }
            " with the complete implementation is available in the fullstack examples."
        }
    }
}
#[component(no_case_check)]
pub fn ReferenceFullstackRouting() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "routing",
            a { href: "#routing", class: "header", "Routing" }
        }
        p {
            "You can easily integrate your fullstack application with a client side router using Dioxus Router. This allows you to create different scenes in your app and navigate between them. You can read more about the router in the "
            a { href: "../router", "router reference" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![allow(non_snake_case)]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">axum::Router;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_router::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{Deserialize, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(|| rsx! {{ Router::&lt;Route&gt; {{}} }});\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Route {{\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Home {{}},\n</span><span style=\"color:#f8f8f2;\">    #[route(</span><span style=\"color:#ffee99;\">&quot;/blog/:id&quot;</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f8f8f2;\">    Blog {{ id: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Blog</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">id</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Link {{ to: Route::Home {{}}, </span><span style=\"color:#ffee99;\">&quot;Go to counter&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        table {{\n</span><span style=\"color:#f8f8f2;\">            tbody {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">for _ in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">id {{\n</span><span style=\"color:#f8f8f2;\">                    tr {{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#f92672;\">for _ in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#f8f8f2;\">id {{\n</span><span style=\"color:#f8f8f2;\">                            td {{ </span><span style=\"color:#ffee99;\">&quot;hello world!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                        }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Home</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> text </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;...&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Link {{ to: Route::Blog {{ id: </span><span style=\"color:#66d9ef;\">count</span><span style=\"color:#f8f8f2;\">() }}, </span><span style=\"color:#ffee99;\">&quot;Go to blog&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;High-Five counter: {{count}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Up high!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">-= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Down low!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Ok</span><span style=\"color:#f8f8f2;\">(data) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_server_data</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">                            println!(</span><span style=\"color:#ffee99;\">&quot;Client received: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, data);\n</span><span style=\"color:#f8f8f2;\">                            text.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(data.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                            </span><span style=\"color:#66d9ef;\">post_server_data</span><span style=\"color:#f8f8f2;\">(data).await.</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                        }}\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Run server function!&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Server said: {{text}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server(PostServerData)]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">post_server_data</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">data</span><span style=\"color:#f8f8f2;\">: String) -&gt; Result&lt;(), ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;Server received: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, data);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[server(GetServerData)]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_server_data</span><span style=\"color:#f8f8f2;\">() -&gt; Result&lt;String, ServerFnError&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Hello from the server!&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "server_router.rs".to_string(),
        }
        SandBoxFrame { url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-router-s75v5q?file=%2Fsrc%2Fmain.rs%3A7%2C1" }
    }
}
#[component(no_case_check)]
pub fn CookbookPublishing() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "publishing",
            a { href: "#publishing", class: "header", "Publishing" }
        }
        p {
            "After you have build your application, you will need to publish it somewhere. This reference will outline different methods of publishing your desktop or web application."
        }
        h2 { id: "web-publishing-with-github-pages",
            a { href: "#web-publishing-with-github-pages", class: "header",
                "Web: Publishing with GitHub Pages"
            }
        }
        p {
            "Edit your  "
            code { "Dioxus.toml" }
            " to point your  "
            code { "out_dir" }
            " to the  "
            code { "docs" }
            " folder and the  "
            code { "base_path" }
            " to the name of your repo:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[application]\n</span><span style=\"color:#f92672;\"># ...\n</span><span style=\"color:#f8f8f2;\">out_dir </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;docs&quot;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">[web.app]\n</span><span style=\"color:#f8f8f2;\">base_path </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;your_repo&quot;</span></pre>\n" }
        p { "Then build your app and publish it to Github:" }
        ul {
            li {
                "Make sure GitHub Pages is set up for your repo to publish any static files in the docs directory"
            }
            li { "Build your app with:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span></pre>\n" }
        ul {
            li {
                "Make a copy of your "
                code { "docs/index.html" }
                " file and rename the copy to "
                code { "docs/404.html" }
                " so that your app will work with client-side routing"
            }
            li { "Add and commit with git" }
            li { "Push to GitHub" }
        }
        h2 { id: "desktop-creating-an-installer",
            a { href: "#desktop-creating-an-installer", class: "header",
                "Desktop: Creating an installer"
            }
        }
        p {
            "Dioxus desktop app uses your operating system's WebView library, so it's portable to be distributed for other platforms."
        }
        p { "In this section, we'll cover how to bundle your app for macOS, Windows, and Linux." }
        h2 { id: "preparing-your-application-for-bundling",
            a {
                href: "#preparing-your-application-for-bundling",
                class: "header",
                "Preparing your application for bundling"
            }
        }
        p {
            "Depending on your platform, you may need to add some additional code to your  "
            code { "main.rs" }
            " file to make sure your app is ready for bundling. On Windows, you'll need to add the  "
            code { "#![windows_subsystem = \"windows\"]" }
            " attribute to your  "
            code { "main.rs" }
            " file to hide the terminal window that pops up when you run your app. "
            strong { "If you're developing on Windows, only use this when bundling." }
            " It will disable the terminal, so you will not get logs of any kind. You can gate it behind a feature, like so:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Cargo.toml\n</span><span style=\"color:#f8f8f2;\">[features]\n</span><span style=\"color:#f8f8f2;\">bundle </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[]</span></pre>\n" }
        p {
            "And then your  "
            code { "main.rs" }
            ":"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#![cfg_attr(feature </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;bundle&quot;</span><span style=\"color:#f8f8f2;\">, windows_subsystem </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;windows&quot;</span><span style=\"color:#f8f8f2;\">)]</span></pre>\n" }
        h2 { id: "adding-assets-to-your-application",
            a { href: "#adding-assets-to-your-application", class: "header",
                "Adding assets to your application"
            }
        }
        p {
            "If you want to bundle assets with your application, you can either use them with the  "
            code { "manganis" }
            " crate (covered more in the "
            a { href: "../reference/assets", "assets" }
            " page), or you can include them in your "
            code { "Dioxus.toml" }
            " file:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[bundle]\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> The list of files to include </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> the bundle. These can contain globs.\n</span><span style=\"color:#f8f8f2;\">resources </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;main.css&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;header.svg&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;**/*.png&quot;</span><span style=\"color:#f8f8f2;\">]</span></pre>\n" }
        h2 { id: "install",
            a { href: "#install", class: "header", "Install " }
            code { "dioxus CLI" }
        }
        p {
            "The first thing we'll do is install the "
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
                "dioxus-cli"
            }
            ". This extension to cargo will make it very easy to package our app for the various platforms."
        }
        p { "To install, simply run" }
        p {
            code { "cargo install dioxus-cli" }
        }
        h2 { id: "building",
            a { href: "#building", class: "header", "Building" }
        }
        p {
            "To bundle your application you can simply run  "
            code { "dx bundle --release" }
            " (also add  "
            code { "--features bundle" }
            " if you're using that, see the "
            a { href: "#preparing-your-application-for-bundling", "this" }
            " for more) to produce a final app with all the optimizations and assets builtin."
        }
        p {
            "Once you've ran the command, your app should be accessible in  "
            code { "dist/bundle/" }
            "."
        }
        p { "For example, a macOS app would look like this:" }
        p {
            img {
                src: "public/static/publish.png",
                alt: "Published App",
                title: "",
            }
        }
        p {
            "Nice! And it's only 4.8 Mb – extremely lean!! Because Dioxus leverages your platform's native WebView, Dioxus apps are extremely memory efficient and won't waste your battery."
        }
        blockquote {
            p {
                "Note: not all CSS works the same on all platforms. Make sure to view your app's CSS on each platform – or web browser (Firefox, Chrome, Safari) before publishing."
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookAntipatterns() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "antipatterns",
            a { href: "#antipatterns", class: "header", "Antipatterns" }
        }
        p {
            "This example shows what not to do and provides a reason why a given pattern is considered an \"AntiPattern\". Most anti-patterns are considered wrong for performance or code re-usability reasons."
        }
        h2 { id: "unnecessarily-nested-fragments",
            a { href: "#unnecessarily-nested-fragments", class: "header",
                "Unnecessarily Nested Fragments"
            }
        }
        p {
            "Fragments don't mount a physical element to the DOM immediately, so Dioxus must recurse into its children to find a physical DOM node. This process is called \"normalization\". This means that deeply nested fragments make Dioxus perform unnecessary work. Prefer one or two levels of fragments / nested components until presenting a true DOM element."
        }
        p {
            "Only Component and Fragment nodes are susceptible to this issue. Dioxus mitigates this with components by providing an API for registering shared state without the Context Provider pattern."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Don&#39;t unnecessarily nest fragments\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">_ = </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    Fragment {{\n</span><span style=\"color:#f8f8f2;\">        Fragment {{\n</span><span style=\"color:#f8f8f2;\">            Fragment {{\n</span><span style=\"color:#f8f8f2;\">                Fragment {{\n</span><span style=\"color:#f8f8f2;\">                    Fragment {{ div {{ </span><span style=\"color:#ffee99;\">&quot;Finally have a real node!&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Render shallow structures\n</span><span style=\"color:#f8f8f2;\">rsx! {{ div {{ </span><span style=\"color:#ffee99;\">&quot;Finally have a real node!&quot; </span><span style=\"color:#f8f8f2;\">}} }}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "incorrect-iterator-keys",
            a { href: "#incorrect-iterator-keys", class: "header", "Incorrect Iterator Keys" }
        }
        p {
            "As described in the "
            a { href: "../reference/dynamic_rendering#the", "dynamic rendering chapter" }
            ", list items must have unique keys that are associated with the same items across renders. This helps Dioxus associate state with the contained components and ensures good diffing performance. Do not omit keys, unless you know that the list will never change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> data: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">HashMap&lt;</span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">props.data;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ No keys\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> data.</span><span style=\"color:#66d9ef;\">values</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ❌ Using index as keys\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">(index , value) </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> data.</span><span style=\"color:#66d9ef;\">values</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">enumerate</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{index}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Using unique IDs as keys:\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    ul {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">(key , value) </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> props.data.</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            li {{ key: </span><span style=\"color:#ffee99;\">&quot;{{key}}&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;List item: {{value}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "avoid-interior-mutability-in-props",
            a { href: "#avoid-interior-mutability-in-props", class: "header",
                "Avoid Interior Mutability in Props"
            }
        }
        p {
            "While it is technically acceptable to have a  "
            code { "Mutex" }
            " or a  "
            code { "RwLock" }
            " in the props, they will be difficult to use."
        }
        p {
            "Suppose you have a struct  "
            code { "User" }
            " containing the field  "
            code { "username: String" }
            ". If you pass a  "
            code { "Mutex<User>" }
            " prop to a  "
            code { "UserComponent" }
            " component, that component may wish to write to the  "
            code { "username" }
            " field. However, when it does, the parent component will not be aware of the change, and the component will not re-render which causes the UI to be out of sync with the state. Instead, consider passing down a reactive value like a  "
            code { "Signal" }
            " or immutable data."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Mutex/RwLock/RefCell in props\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">AntipatternInteriorMutability {{\n</span><span style=\"color:#f8f8f2;\">    map: Rc&lt;RefCell&lt;HashMap&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">, String&gt;&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">AntipatternInteriorMutability {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">other</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        std::rc::Rc::ptr_eq(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">self.map, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">other.map)\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">AntipatternInteriorMutability</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">map</span><span style=\"color:#f8f8f2;\">: Rc&lt;RefCell&lt;HashMap&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">, String&gt;&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> map </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> map.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#8c8c8c;\">// Writing to map will not rerun any components\n</span><span style=\"color:#f8f8f2;\">                    map.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Mutate map&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since writing to map will not rerun any components, this will get out of date\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{map.borrow().get(&amp;0).unwrap()}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Use a signal to pass mutable state\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">AntipatternInteriorMutabilitySignal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">map</span><span style=\"color:#f8f8f2;\">: Signal&lt;HashMap&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">, String&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#8c8c8c;\">// Writing to map will rerun any components that read the map\n</span><span style=\"color:#f8f8f2;\">                map.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Mutate map&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Since writing to map will rerun subscribers, this will get updated\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{map.read().get(&amp;0).unwrap()}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "avoid-updating-state-during-render",
            a { href: "#avoid-updating-state-during-render", class: "header",
                "Avoid Updating State During Render"
            }
        }
        p {
            "Every time you update the state, Dioxus needs to re-render the component – this is inefficient! Consider refactoring your code to avoid this."
        }
        p {
            "Also, if you unconditionally update the state during render, it will be re-rendered in an infinite loop."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Updating state in render\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> second_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Updating the state during a render can easily lead to infinite loops\n</span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#66d9ef;\">second_signal</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    second_signal.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Update state in an effect\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> second_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// The closure you pass to use_effect will be rerun whenever any of the dependencies change without re-rendering the component\n</span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#66d9ef;\">second_signal</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        second_signal.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Deriving state with use_memo\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#8c8c8c;\">// Memos are specifically designed for derived state. If your state fits this pattern, use it.\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> second_signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#66d9ef;\">first_signal</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "avoid-large-groups-of-state",
            a { href: "#avoid-large-groups-of-state", class: "header",
                "Avoid Large Groups of State"
            }
        }
        p {
            "It can be tempting to have a single large state struct that contains all of your application's state. However, this can lead to issues:"
        }
        ul {
            li { "It can be easy to accidentally mutate the state in a way that causes an infinite loop" }
            li { "It can be difficult to reason about when and how the state is updated" }
            li {
                "It can lead to performance issues because many components will need to re-render when the state changes"
            }
        }
        p {
            "Instead, consider breaking your state into smaller, more manageable pieces. This will make it easier to reason about the state, avoid update loops, and improve performance."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ❌ Large state struct\n</span><span style=\"color:#f8f8f2;\">    #[derive(Props, Clone, PartialEq)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">LargeState {{\n</span><span style=\"color:#f8f8f2;\">        users: Vec&lt;User&gt;,\n</span><span style=\"color:#f8f8f2;\">        logged_in: </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        warnings: Vec&lt;String&gt;,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    #[derive(Props, Clone, PartialEq)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">User {{\n</span><span style=\"color:#f8f8f2;\">        name: String,\n</span><span style=\"color:#f8f8f2;\">        email: String,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> all_my_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| LargeState {{\n</span><span style=\"color:#f8f8f2;\">        users: vec![User {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            email: </span><span style=\"color:#ffee99;\">&quot;alice@example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}],\n</span><span style=\"color:#f8f8f2;\">        logged_in: </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        warnings: vec![],\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// It is very easy to accidentally read and write to the state object if it contains all your state\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> read </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> all_my_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> read.logged_in;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if !</span><span style=\"color:#f8f8f2;\">logged_in {{\n</span><span style=\"color:#f8f8f2;\">            all_my_state\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">write_unchecked</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                .warnings\n</span><span style=\"color:#f8f8f2;\">                .</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;You are not logged in&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ✅ Use multiple signals to manage state\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> users </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        vec![User {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            email: </span><span style=\"color:#ffee99;\">&quot;alice@example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}]\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> warnings </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_effect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Now you can read and write to separate signals which will not cause issues\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if !</span><span style=\"color:#66d9ef;\">logged_in</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            warnings.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;You are not logged in&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ✅ Use memos to create derived state when larger states are unavoidable\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Notice we didn&#39;t split everything into separate signals. Users still make sense as a vec of data\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> users </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        vec![User {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            email: </span><span style=\"color:#ffee99;\">&quot;alice@example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}]\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> logged_in </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> warnings: Signal&lt;Vec&lt;String&gt;&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| vec![]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// In child components, you can use the memo to create derived that will only update when a specific part of the state changes\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// This will help you avoid unnecessary re-renders and infinite loops\n</span><span style=\"color:#f8f8f2;\">    #[component]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">FirstUser</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">users</span><span style=\"color:#f8f8f2;\">: Signal&lt;Vec&lt;User&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first_user </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> users.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">first</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        rsx! {{\n</span><span style=\"color:#f8f8f2;\">            div {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;First user: {{first_user().name}}&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        FirstUser {{\n</span><span style=\"color:#f8f8f2;\">            users\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "running-non-deterministic-code-in-the-body-of-a-component",
            a {
                href: "#running-non-deterministic-code-in-the-body-of-a-component",
                class: "header",
                "Running Non-Deterministic Code in the Body of a Component"
            }
        }
        p {
            "If you have a component that contains non-deterministic code, that code should generally not be run in the body of the component. If it is put in the body of the component, it will be executed every time the component is re-rendered which can lead to performance issues."
        }
        p {
            "Instead, consider moving the non-deterministic code into a hook that only runs when the component is first created or an effect that reruns when dependencies change."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Non-deterministic code in the body of a component\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NonDeterministic</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_random_id </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">rand::random::&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u64</span><span style=\"color:#f8f8f2;\">&gt;();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Id will change every single time the component is re-rendered\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#ffee99;\">&quot;{{my_random_id}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Use a hook to run non-deterministic code\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">NonDeterministicHook</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// If you store the result of the non-deterministic code in a hook, it will stay the same between renders\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_random_id </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| rand::random::&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u64</span><span style=\"color:#f8f8f2;\">&gt;());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#ffee99;\">&quot;{{my_random_id}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
        h2 { id: "overly-permissive-partialeq-for-props",
            a {
                href: "#overly-permissive-partialeq-for-props",
                class: "header",
                "Overly Permissive PartialEq for Props"
            }
        }
        p {
            "You may have noticed that  "
            code { "Props" }
            " requires a  "
            code { "PartialEq" }
            " implementation. That  "
            code { "PartialEq" }
            " is very important for Dioxus to work correctly. It is used to determine if a component should re-render or not when the parent component re-renders."
        }
        p {
            "If you cannot derive  "
            code { "PartialEq" }
            " for your  "
            code { "Props" }
            ", you will need to implement it yourself. If you do implement  "
            code { "PartialEq" }
            ", make sure to return  "
            code { "false" }
            " any time the props change in a way that would cause the UI in the child component to change."
        }
        p {
            "In general, returning  "
            code { "false" }
            " from  "
            code { "PartialEq" }
            " if you aren't sure if the props have changed or not is better than returning  "
            code { "true" }
            ". This will help you avoid out of date UI in your child components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// ❌ Permissive PartialEq for Props\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">PermissivePartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// This will cause the component to **never** re-render when the parent component re-renders\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">PermissivePartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">PermissivePartialEq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: PermissivePartialEqProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name.name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">PermissivePartialEqParent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        PermissivePartialEq {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// The PermissivePartialEq component will not get the updated value of name because the PartialEq implementation says that the props are the same\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#66d9ef;\">name</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Derive PartialEq for Props\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">DerivePartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">DerivePartialEq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: DerivePartialEqProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name.name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">DerivePartialEqParent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;Alice&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        DerivePartialEq {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#66d9ef;\">name</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// ✅ Return false from PartialEq if you are unsure if the props have changed\n</span><span style=\"color:#f8f8f2;\">#[derive(Debug)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">NonPartialEq;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">RcPartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    name: Rc&lt;NonPartialEq&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">RcPartialEqProps {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">other</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// This will almost always return false because the Rc will likely point to a different value\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Implementing PartialEq for NonPartialEq would be better, but if it is controlled by another library, it may not be possible\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// **Always** return false if you are unsure if the props have changed\n</span><span style=\"color:#f8f8f2;\">        std::rc::Rc::ptr_eq(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">self.name, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">other.name)\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">RcPartialEq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: RcPartialEqProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Hello {{name.name:?}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">RcPartialEqParent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| Rc::new(NonPartialEq));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        RcPartialEq {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Generally, RcPartialEq will rerun even if the value of name hasn&#39;t actually changed because the Rc will point to a different value\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#66d9ef;\">name</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "anti_patterns.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookErrorHandling() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "error-handling",
            a { href: "#error-handling", class: "header", "Error handling" }
        }
        p {
            "A selling point of Rust for web development is the reliability of always knowing where errors can occur and being forced to handle them"
        }
        p {
            "However, we haven't talked about error handling at all in this guide! In this chapter, we'll cover some strategies in handling errors to ensure your app never crashes."
        }
        h2 { id: "the-simplest--returning-none",
            a { href: "#the-simplest--returning-none", class: "header",
                "The simplest – returning None"
            }
        }
        p {
            "Astute observers might have noticed that  "
            code { "Element" }
            " is actually a type alias for  "
            code { "Option<VNode>" }
            ". You don't need to know what a  "
            code { "VNode" }
            " is, but it's important to recognize that we could actually return nothing at all:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{  }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "This lets us add in some syntactic sugar for operations we think "
            em { "shouldn't" }
            " fail, but we're still not confident enough to \"unwrap\" on."
        }
        blockquote {
            p {
                "The nature of  "
                code { "Option<VNode>" }
                " might change in the future as the  "
                code { "try" }
                " trait gets upgraded."
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// immediately return &quot;None&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| dioxus::Result::Ok(</span><span style=\"color:#ffee99;\">&quot;hi&quot;</span><span style=\"color:#f8f8f2;\">))</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        h2 { id: "early-return-on-result",
            a { href: "#early-return-on-result", class: "header", "Early return on result" }
        }
        p {
            "Because Rust can't accept both Options and Results with the existing try infrastructure, you'll need to manually handle Results. This can be done by converting them into Options or by explicitly handling them. If you choose to convert your Result into an Option and bubble it with a  "
            code { "?" }
            ", keep in mind that if you do hit an error you will lose error information and nothing will be rendered for that component."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Convert Result to Option\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">context</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to parse&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Early return\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> count </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> val: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= match</span><span style=\"color:#f8f8f2;\"> count.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(val) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> val,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; return </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;Parsing failed&quot; </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">    }};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "Notice that while hooks in Dioxus do not like being called in conditionals or loops, they "
            em { "are" }
            " okay with early returns. Returning an error state early is a completely valid way of handling errors."
        }
        h2 { id: "match-results",
            a { href: "#match-results", class: "header", "Match results" }
        }
        p {
            "The next \"best\" way of handling errors in Dioxus is to match on the error locally. This is the most robust way of handling errors, but it doesn't scale to architectures beyond a single component."
        }
        p { "To do this, we simply have an error state built into our component:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "Whenever we perform an action that generates an error, we'll set that error state. We can then match on the error in a number of ways (early return, return Element, etc)."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Commandline</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">error</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(error) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            h1 {{ </span><span style=\"color:#ffee99;\">&quot;An error occurred&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">            input {{ oninput: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> error.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;bad thing happened!&quot;</span><span style=\"color:#f8f8f2;\">)) }}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        h2 { id: "passing-error-states-through-components",
            a {
                href: "#passing-error-states-through-components",
                class: "header",
                "Passing error states through components"
            }
        }
        p {
            "If you're dealing with a handful of components with minimal nesting, you can just pass the error handle into child components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Commandline</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(error) </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">error</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">rsx! {{ </span><span style=\"color:#ffee99;\">&quot;An error occurred&quot; </span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">        Child {{ error }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">error</span><span style=\"color:#f8f8f2;\">: Signal&lt;Option&lt;</span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">&gt;&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        input {{ oninput: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> error.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;bad thing happened!&quot;</span><span style=\"color:#f8f8f2;\">)) }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "Much like before, our child components can manually set the error during their own actions. The advantage to this pattern is that we can easily isolate error states to a few components at a time, making our app more predictable and robust."
        }
        h2 { id: "throwing-errors",
            a { href: "#throwing-errors", class: "header", "Throwing errors" }
        }
        p {
            "Dioxus provides a much easier way to handle errors: throwing them. Throwing errors combines the best parts of an error state and early return: you can easily throw and error with  "
            code { "?" }
            ", but you keep information about the error so that you can handle it in a parent component."
        }
        p {
            "You can call  "
            code { "throw" }
            " on any  "
            code { "Result" }
            " type that implements  "
            code { "Debug" }
            " to turn it into an error state and then use  "
            code { "?" }
            " to return early if you do hit an error. You can capture the error state with an  "
            code { "ErrorBoundary" }
            " component that will render the a different component if an error is thrown in any of its children."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">            handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">ctx</span><span style=\"color:#f8f8f2;\">: ErrorContext| {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">ctx.</span><span style=\"color:#66d9ef;\">errors</span><span style=\"color:#f8f8f2;\">()[</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f8f8f2;\">                rsx! {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Oops, we encountered an error. Please report {{error}} to the developer of this application&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            ThrowsError {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ThrowsError</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">context</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to parse&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "You can even nest  "
            code { "ErrorBoundary" }
            " components to capture errors at different levels of your app."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">App</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">            handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">ctx</span><span style=\"color:#f8f8f2;\">: ErrorContext| {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">ctx.</span><span style=\"color:#66d9ef;\">errors</span><span style=\"color:#f8f8f2;\">()[</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f8f8f2;\">                rsx! {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Hmm, something went wrong. Please report {{error}} to the developer of this application&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            Parent {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        ErrorBoundary {{\n</span><span style=\"color:#f8f8f2;\">            handle_error: |</span><span style=\"font-style:italic;color:#fd971f;\">ctx</span><span style=\"color:#f8f8f2;\">: ErrorContext| {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> error </span><span style=\"color:#f92672;\">= &amp;</span><span style=\"color:#f8f8f2;\">ctx.</span><span style=\"color:#66d9ef;\">errors</span><span style=\"color:#f8f8f2;\">()[</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f8f8f2;\">                rsx! {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;The child component encountered an error: {{error}}&quot;\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            ThrowsError {{}}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ThrowsError</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name: </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ffee99;\">&quot;1.234&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">context</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to parse&quot;</span><span style=\"color:#f8f8f2;\">)</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    todo!()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "error_handling.rs".to_string(),
        }
        p {
            "This pattern is particularly helpful whenever your code generates a non-recoverable error. You can gracefully capture these \"global\" error states without panicking or handling state for each error yourself."
        }
    }
}
#[component(no_case_check)]
pub fn CookbookIntegrationsIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p {
            "This section of the guide provides getting started guides for common tools used with Dioxus."
        }
        ul {
            li {
                a { href: "integrations/./logging", "Logging" }
            }
            li {
                a { href: "integrations/./internationalization", "Internationalization" }
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookIntegrationsLogging() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "logging",
            a { href: "#logging", class: "header", "Logging" }
        }
        p {
            "Dioxus has a wide range of supported platforms, each with their own logging requirements. We'll discuss the different options available for your projects."
        }
        h4 { id: "the-tracing-crate",
            a { href: "#the-tracing-crate", class: "header", "The Tracing Crate" }
        }
        p {
            "The "
            a { href: "https://crates.io/crates/tracing", "Tracing" }
            " crate is the logging interface that the Dioxus library uses. It is not required to use the Tracing crate, but you will not recieve logs from the Dioxus library."
        }
        p {
            "The Tracing crate provides a variety of simple  "
            code { "println" }
            "-like macros with varying levels of severity. "
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    tracing::trace</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;trace&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::debug</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;debug&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;info&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::warn</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;warn&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    tracing::error</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;error&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "All the loggers provided on this page are, besides configuration and initialization, interfaced using these macros. Often you will also utilize the Tracing crate's  "
            code { "Level" }
            " enum. This enum usually represents the maximum log severity you want your application to emit and can be loaded from a variety of sources such as configuration file, environment variable, and more."
        }
        p {
            "For more information, visit the Tracing crate's "
            a { href: "https://docs.rs/tracing/latest/tracing/", "docs" }
            "."
        }
        h2 { id: "dioxus-logger",
            a { href: "#dioxus-logger", class: "header", "Dioxus Logger" }
        }
        p {
            a { href: "https://crates.io/crates/dioxus-logger", "Dioxus Logger" }
            " is a logging utility that will start the appropriate logger for the platform. Currently every platform except mobile is supported."
        }
        p {
            "To use Dioxus Logger, call the  "
            code { "init()" }
            " function:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tracing::Level;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    dioxus_logger::init(Level::</span><span style=\"color:#ff80f4;\">INFO</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;failed to init logger&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus launch code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The  "
            code { "dioxus_logger::init()" }
            " function initializes Dioxus Logger with the appropriate tracing logger using the default configuration and provided  "
            code { "Level" }
            "."
        }
        h4 { id: "platform-intricacies",
            a { href: "#platform-intricacies", class: "header", "Platform Intricacies" }
        }
        p {
            "On web, Dioxus Logger will use "
            a { href: "https://crates.io/crates/tracing-wasm", "tracing-wasm" }
            ". On Desktop and server-based targets, Dioxus Logger will use "
            a { href: "https://crates.io/crates/tracing-subscriber", "tracing-subscriber" }
            "'s "
            code { "FmtSubscriber" }
            "."
        }
        h4 { id: "final-notes",
            a { href: "#final-notes", class: "header", "Final Notes" }
        }
        p {
            "Dioxus Logger is the preferred logger to use with Dioxus if it suites your needs. There are more features to come and Dioxus Logger is planned to become an integral part of Dioxus. If there are any feature suggestions or issues with Dioxus Logger, feel free to reach out on the "
            a { href: "https://discord.gg/XgGxMSkvUM", "Dioxus Discord Server" }
            "!"
        }
        p {
            "For more information, visit Dioxus Logger's "
            a { href: "https://docs.rs/dioxus-logger/latest/dioxus_logger/", "docs" }
            "."
        }
        h2 { id: "desktop-and-server",
            a { href: "#desktop-and-server", class: "header", "Desktop and Server" }
        }
        p { "For Dioxus' desktop and server targets, you can generally use the logger of your choice." }
        p { "Some popular options are:" }
        ul {
            li {
                a { href: "https://crates.io/crates/tracing-subscriber", "tracing-subscriber" }
                "'s "
                code { "FmtSubscriber" }
                " for console output."
            }
            li {
                a { href: "https://crates.io/crates/tracing-appender", "tracing-appender" }
                " for logging to files."
            }
            li {
                a { href: "https://crates.io/crates/tracing-bunyan-formatter",
                    "tracing-bunyan-formatter"
                }
                " for the Bunyan format."
            }
        }
        p { "To keep this guide short, we will not be covering the usage of these crates." }
        p {
            "For a full list of popular tracing-based logging crates, visit "
            a { href: "https://docs.rs/tracing/latest/tracing/#related-crates", "this" }
            " list in the Tracing crate's docs."
        }
        h2 { id: "web",
            a { href: "#web", class: "header", "Web" }
        }
        p {
            a { href: "https://crates.io/crates/tracing-wasm", "tracing-wasm" }
            " is a logging interface that can be used with Dioxus' web platform."
        }
        p {
            "The easiest way to use WASM Logger is with the  "
            code { "set_as_global_default" }
            " function:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    tracing_wasm::set_as_global_default();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "This starts tracing with a  "
            code { "Level" }
            " of  "
            code { "Trace" }
            ". "
        }
        p {
            "Using a custom  "
            code { "level" }
            " is a little trickier. We need to use the  "
            code { "WasmLayerConfigBuilder" }
            " and start the logger with  "
            code { "set_as_global_default_with_config()" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tracing::Level;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> tracing_config </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">tracing_wasm::WASMLayerConfigBuilder::new().</span><span style=\"color:#66d9ef;\">set_max_level</span><span style=\"color:#f8f8f2;\">(Level::</span><span style=\"color:#ff80f4;\">INFO</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    tracing_wasm::set_as_global_default_with_config(tracing_config);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h1 { id: "mobile",
            a { href: "#mobile", class: "header", "Mobile" }
        }
        p {
            "Unfortunately there are no tracing crates that work with mobile targets. As an alternative you can use the "
            a { href: "https://crates.io/crates/log", "log" }
            " crate."
        }
        h2 { id: "android",
            a { href: "#android", class: "header", "Android" }
        }
        p {
            a { href: "https://crates.io/crates/android_logger", "Android Logger" }
            " is a logging interface that can be used when targeting Android. Android Logger runs whenever an event "
            code { "native_activity_create" }
            " is called by the Android system:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">log::LevelFilter;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">android_logger::Config;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">native_activity_create</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    android_logger::init_once(\n</span><span style=\"color:#f8f8f2;\">        Config::default()\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">with_max_level</span><span style=\"color:#f8f8f2;\">(LevelFilter::Info)\n</span><span style=\"color:#f8f8f2;\">            .</span><span style=\"color:#66d9ef;\">with_tag</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;myapp&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "The  "
            code { "with_tag()" }
            " is what your app's logs will show as."
        }
        h4 { id: "viewing-logs",
            a { href: "#viewing-logs", class: "header", "Viewing Logs" }
        }
        p { "Android logs are sent to logcat. To use logcat through the Android debugger, run:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">adb </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">d logcat</span></pre>\n" }
        p { "Your Android device will need developer options/usb debugging enabled." }
        p {
            "For more information, visit android_logger's "
            a { href: "https://docs.rs/android_logger/latest/android_logger/", "docs" }
            "."
        }
        h2 { id: "ios",
            a { href: "#ios", class: "header", "iOS" }
        }
        p {
            "The current option for iOS is the "
            a { href: "https://crates.io/crates/oslog", "oslog" }
            " crate. "
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Init logger\n</span><span style=\"color:#f8f8f2;\">    OsLogger::new(</span><span style=\"color:#ffee99;\">&quot;com.example.test&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">level_filter</span><span style=\"color:#f8f8f2;\">(LevelFilter::Debug)\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">init</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;failed to init logger&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Dioxus code\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h4 { id: "viewing-logs",
            a { href: "#viewing-logs", class: "header", "Viewing Logs" }
        }
        p { "You can view the emitted logs in Xcode. " }
        p {
            "For more information, visit "
            a { href: "https://crates.io/crates/oslog", "oslog" }
            ". "
        }
    }
}
#[component(no_case_check)]
pub fn CookbookIntegrationsInternationalization() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "internationalization",
            a { href: "#internationalization", class: "header", "Internationalization" }
        }
        p {
            "If you application supports multiple languages, the "
            a { href: "https://github.com/DioxusLabs/sdk", "Dioxus SDK" }
            " crate contains helpers to make working with translations in your application easier."
        }
        h2 { id: "the-full-code-for-internationalization",
            a {
                href: "#the-full-code-for-internationalization",
                class: "header",
                "The full code for internationalization"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_sdk::i18n::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus_sdk::translate;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::str::FromStr;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">EN_US</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;{{\n</span><span style=\"color:#ffee99;\">    &quot;id&quot;: &quot;en-US&quot;,\n</span><span style=\"color:#ffee99;\">    &quot;texts&quot;: {{\n</span><span style=\"color:#ffee99;\">        &quot;messages&quot;: {{\n</span><span style=\"color:#ffee99;\">            &quot;hello_world&quot;: &quot;Hello World!&quot;\n</span><span style=\"color:#ffee99;\">        }},\n</span><span style=\"color:#ffee99;\">        &quot;messages.hello&quot;: &quot;Hello {{name}}&quot;\n</span><span style=\"color:#ffee99;\">    }}\n</span><span style=\"color:#ffee99;\">}}&quot;#</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">ES_ES</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">r</span><span style=\"color:#ffee99;\">#&quot;{{\n</span><span style=\"color:#ffee99;\">    &quot;id&quot;: &quot;es-ES&quot;,\n</span><span style=\"color:#ffee99;\">    &quot;texts&quot;: {{\n</span><span style=\"color:#ffee99;\">        &quot;messages&quot;: {{\n</span><span style=\"color:#ffee99;\">            &quot;hello_world&quot;: &quot;Hola Mundo!&quot;\n</span><span style=\"color:#ffee99;\">        }},\n</span><span style=\"color:#ffee99;\">        &quot;messages.hello&quot;: &quot;Hola {{name}}&quot;\n</span><span style=\"color:#ffee99;\">    }}\n</span><span style=\"color:#ffee99;\">}}&quot;#</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[allow(non_snake_case)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Body</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> i18 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_i18</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> change_to_english </span><span style=\"color:#f92672;\">= move |_|</span><span style=\"color:#f8f8f2;\"> i18.</span><span style=\"color:#66d9ef;\">set_language</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;en-US&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> change_to_spanish </span><span style=\"color:#f92672;\">= move |_|</span><span style=\"color:#f8f8f2;\"> i18.</span><span style=\"color:#66d9ef;\">set_language</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;es-ES&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: change_to_english, label {{ </span><span style=\"color:#ffee99;\">&quot;English&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: change_to_spanish, label {{ </span><span style=\"color:#ffee99;\">&quot;Spanish&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">        p {{ {{translate!(i18, </span><span style=\"color:#ffee99;\">&quot;messages.hello_world&quot;</span><span style=\"color:#f8f8f2;\">)}} }}\n</span><span style=\"color:#f8f8f2;\">        p {{ {{translate!(i18, </span><span style=\"color:#ffee99;\">&quot;messages.hello&quot;</span><span style=\"color:#f8f8f2;\">, name: </span><span style=\"color:#ffee99;\">&quot;Dioxus&quot;</span><span style=\"color:#f8f8f2;\">)}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_init_i18n</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;en-US&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;en-US&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">parse</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">(), || {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> en_us </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Language::from_str(</span><span style=\"color:#ff80f4;\">EN_US</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> es_es </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Language::from_str(</span><span style=\"color:#ff80f4;\">ES_ES</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        vec![en_us, es_es]\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Body {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "i18n.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookStateIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "state-cookbook",
            a { href: "#state-cookbook", class: "header", "State Cookbook" }
        }
        ul {
            li {
                a { href: "state/external", "External State" }
            }
            li {
                a { href: "state/custom_hooks", "Custom Hook" }
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookStateExternalIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "working-with-external-state",
            a { href: "#working-with-external-state", class: "header",
                "Working with External State"
            }
        }
        p {
            "This guide will help you integrate your Dioxus application with some external state like a different thread or a websocket connection."
        }
        h2 { id: "working-with-non-reactive-state",
            a { href: "#working-with-non-reactive-state", class: "header",
                "Working with non-reactive State"
            }
        }
        p {
            a { href: "external/../../reference/use_coroutine", "Coroutines" }
            " are great tool for dealing with non-reactive (state you don't render directly) state within your application."
        }
        p {
            "You can store your state inside the coroutine async block and communicate with the coroutine with messages from any child components."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// import futures::StreamExt to use the next() method\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::StreamExt;\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> response_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> tx </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_coroutine</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move |mut</span><span style=\"color:#f8f8f2;\"> rx</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Define your state before the loop\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">reqwest::Client::new();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> cache: HashMap&lt;String, String&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">HashMap::new();\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">loop </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Loop and wait for the next message\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(request) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> rx.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">().await {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// Resolve the message\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> response </span><span style=\"color:#f92672;\">= if </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(response) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> cache.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">request) {{\n</span><span style=\"color:#f8f8f2;\">                response.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">            }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> response </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> state\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">request)\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                    .await\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">text</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">                    .await\n</span><span style=\"color:#f8f8f2;\">                    .</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                cache.</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(request, response.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                response\n</span><span style=\"color:#f8f8f2;\">            }};\n</span><span style=\"color:#f8f8f2;\">            response_state.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(response));\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">break</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#8c8c8c;\">// Send a message to the coroutine\n</span><span style=\"color:#f8f8f2;\">tx.</span><span style=\"color:#66d9ef;\">send</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;https://example.com&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#8c8c8c;\">// Get the current state of the coroutine\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> response </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> response_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();</span></pre>\n",
            name: "use_coroutine.rs".to_string(),
        }
        h2 { id: "making-reactive-state-external",
            a { href: "#making-reactive-state-external", class: "header",
                "Making Reactive State External"
            }
        }
        p {
            "If you have some reactive state (state that is rendered), that you want to modify from another thread, you can use a signal that is sync. Signals take an optional second generic value with information about syncness. Sync signals have a slightly higher overhead than thread local signals, but they can be used in a multithreaded environment."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">launch</span><span style=\"color:#f8f8f2;\">(app);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> signal </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal_sync</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        std::thread::spawn(</span><span style=\"color:#f92672;\">move || loop </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            std::thread::sleep(std::time::Duration::from_secs(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// You can easily update the signal from a different thread\n</span><span style=\"color:#f8f8f2;\">            signal </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">        }});\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> signal </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Increase&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{signal}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "use_coroutine.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookStateCustomHooksIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "custom-hooks",
            a { href: "#custom-hooks", class: "header", "Custom Hooks" }
        }
        p {
            "Hooks are a great way to encapsulate business logic. If none of the existing hooks work for your problem, you can write your own."
        }
        p {
            "When writing your hook, you can make a function that starts with  "
            code { "use_" }
            " and takes any arguments you need. You can then use the  "
            code { "use_hook" }
            " method to create a hook that will be called the first time the component is rendered."
        }
        h2 { id: "composing-hooks",
            a { href: "#composing-hooks", class: "header", "Composing Hooks" }
        }
        p {
            "To avoid repetition, you can encapsulate business logic based on existing hooks to create a new hook."
        }
        p {
            "For example, if many components need to access an  "
            code { "AppSettings" }
            " struct, you can create a \"shortcut\" hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_settings</span><span style=\"color:#f8f8f2;\">() -&gt; Signal&lt;AppSettings&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">consume_context</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_composed.rs".to_string(),
        }
        p {
            "Or if you want to wrap a hook that persists reloads with the storage API, you can build on top of the use_signal hook to work with mutable state:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">gloo_storage::{{LocalStorage, Storage}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{de::DeserializeOwned, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">/// A persistent storage hook that can be used to store data across application reloads.\n</span><span style=\"color:#f8f8f2;\">#[allow(clippy::needless_return)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_persistent</span><span style=\"color:#f8f8f2;\">&lt;T: Serialize </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> DeserializeOwned </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> Default </span><span style=\"color:#f92672;\">+ &#39;static</span><span style=\"color:#f8f8f2;\">&gt;(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// A unique key for the storage entry\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">key</span><span style=\"color:#f8f8f2;\">: impl ToString,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// A function that returns the initial value if the storage entry is empty\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">init</span><span style=\"color:#f8f8f2;\">: impl FnOnce() -&gt; T,\n</span><span style=\"color:#f8f8f2;\">) -&gt; UsePersistent&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Use the use_signal hook to create a mutable state for the storage entry\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// This closure will run when the hook is created\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> key </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> key.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">LocalStorage::get(key.</span><span style=\"color:#66d9ef;\">as_str</span><span style=\"color:#f8f8f2;\">()).</span><span style=\"color:#66d9ef;\">ok</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap_or_else</span><span style=\"color:#f8f8f2;\">(init);\n</span><span style=\"color:#f8f8f2;\">        StorageEntry {{ key, value }}\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Wrap the state in a new struct with a custom API\n</span><span style=\"color:#f8f8f2;\">    UsePersistent {{ inner: state }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">StorageEntry&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    key: String,\n</span><span style=\"color:#f8f8f2;\">    value: T,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">/// Storage that persists across application reloads\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">UsePersistent&lt;T: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    inner: Signal&lt;StorageEntry&lt;T&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T&gt; Clone </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">UsePersistent&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">clone</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">self\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T&gt; Copy </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">UsePersistent&lt;T&gt; {{}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T: Serialize </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> DeserializeOwned </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> Clone </span><span style=\"color:#f92672;\">+ &#39;static</span><span style=\"color:#f8f8f2;\">&gt; UsePersistent&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// Returns a reference to the value\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">        self.inner.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">/// Sets the value\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">: T) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> inner </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">self.inner.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Write the new value to local storage\n</span><span style=\"color:#f8f8f2;\">        LocalStorage::set(inner.key.</span><span style=\"color:#66d9ef;\">as_str</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">value);\n</span><span style=\"color:#f8f8f2;\">        inner.value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> value;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_composed.rs".to_string(),
        }
        h2 { id: "custom-hook-logic",
            a { href: "#custom-hook-logic", class: "header", "Custom Hook Logic" }
        }
        p {
            "You can use "
            a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook.html",
                code { "use_hook" }
            }
            " to build your own hooks. In fact, this is what all the standard hooks are built on!"
        }
        p {
            code { "use_hook" }
            " accepts a single closure for initializing the hook. It will be only run the first time the component is rendered. The return value of that closure will be used as the value of the hook – Dioxus will take it, and store it for as long as the component is alive. On every render (not just the first one!), you will get a reference to this value."
        }
        blockquote {
            p {
                "Note: You can use the  "
                code { "use_on_destroy" }
                " hook to clean up any resources the hook uses when the component is destroyed."
            }
        }
        p {
            "Inside the initialization closure, you will typically make calls to other  "
            code { "cx" }
            " methods. For example:"
        }
        ul {
            li {
                "The "
                code { "use_signal" }
                " hook tracks state in the hook value, and uses "
                a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.schedule_update.html",
                    code { "schedule_update" }
                }
                " to make Dioxus re-render the component whenever it changes."
            }
        }
        p {
            "Here is a simplified implementation of the  "
            code { "use_signal" }
            " hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::cell::RefCell;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::rc::Rc;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::sync::Arc;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    value: Rc&lt;RefCell&lt;T&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">    update: Arc&lt;dyn Fn()&gt;,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T&gt; Clone </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">clone</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            value: self.value.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            update: self.update.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">my_use_signal</span><span style=\"color:#f8f8f2;\">&lt;T: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">init</span><span style=\"color:#f8f8f2;\">: impl FnOnce() -&gt; T) -&gt; Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// The update function will trigger a re-render in the component cx is attached to\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> update </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">schedule_update</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Create the initial state\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Rc::new(RefCell::new(</span><span style=\"color:#66d9ef;\">init</span><span style=\"color:#f8f8f2;\">()));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        Signal {{ value, update }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;T: Clone&gt; Signal&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">        self.value.</span><span style=\"color:#66d9ef;\">borrow</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">: T) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Update the state\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">self.value.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> value;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Trigger a re-render on the component the state is from\n</span><span style=\"color:#f8f8f2;\">        (self.update)();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_composed.rs".to_string(),
        }
        ul {
            li {
                "The "
                code { "use_context" }
                " hook calls "
                a { href: "https://docs.rs/dioxus/latest/dioxus/prelude/fn.consume_context.html",
                    code { "consume_context" }
                }
                " (which would be expensive to call on every render) to get some context from the component"
            }
        }
        p {
            "Here is an implementation of the  "
            code { "use_context" }
            " and  "
            code { "use_context_provider" }
            " hooks:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_context</span><span style=\"color:#f8f8f2;\">&lt;T: </span><span style=\"color:#f92672;\">&#39;static +</span><span style=\"color:#f8f8f2;\"> Clone&gt;() -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#66d9ef;\">consume_context</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">use_context_provider</span><span style=\"color:#f8f8f2;\">&lt;T: </span><span style=\"color:#f92672;\">&#39;static +</span><span style=\"color:#f8f8f2;\"> Clone&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">f</span><span style=\"color:#f8f8f2;\">: impl FnOnce() -&gt; T) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> val </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">f</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Provide the context state to the component\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">provide_context</span><span style=\"color:#f8f8f2;\">(val.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        val\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "hooks_composed.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn CookbookTesting() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "testing",
            a { href: "#testing", class: "header", "Testing" }
        }
        p {
            "When building application or libraries with Dioxus, you may want to include some tests to check the behavior of parts of your application. This guide will teach you how to test different parts of your Dioxus application."
        }
        h2 { id: "component-testing",
            a { href: "#component-testing", class: "header", "Component Testing" }
        }
        p {
            "You can use a combination of "
            a { href: "https://docs.rs/pretty_assertions/latest/pretty_assertions/",
                "pretty-assertions"
            }
            " and "
            a { href: "", "dioxus-ssr" }
            " to check that two snippets of rsx are equal:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::FutureExt;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::{{cell::RefCell, sync::Arc}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[test]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">test</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">assert_rsx_eq</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        rsx! {{\n</span><span style=\"color:#f8f8f2;\">            div {{ </span><span style=\"color:#ffee99;\">&quot;Hello world&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            div {{ </span><span style=\"color:#ffee99;\">&quot;Hello world&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        rsx! {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">for _ in </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">..</span><span style=\"color:#ff80f4;\">2 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                div {{ </span><span style=\"color:#ffee99;\">&quot;Hello world&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    )\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">assert_rsx_eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">first</span><span style=\"color:#f8f8f2;\">: Element, </span><span style=\"font-style:italic;color:#fd971f;\">second</span><span style=\"color:#f8f8f2;\">: Element) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> first </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">dioxus_ssr::render_element(first);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> second </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">dioxus_ssr::render_element(second);\n</span><span style=\"color:#f8f8f2;\">    pretty_assertions::assert_str_eq</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(first, second);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_test.rs".to_string(),
        }
        h2 { id: "hook-testing",
            a { href: "#hook-testing", class: "header", "Hook Testing" }
        }
        p {
            "When creating libraries around Dioxus, it can be helpful to make tests for your "
            a { href: "./state/custom_hooks", "custom hooks" }
            "."
        }
        p {
            "Dioxus does not currently have a full hook testing library, but you can build a bespoke testing framework by manually driving the virtual dom."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::FutureExt;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::{{cell::RefCell, rc::Rc, sync::Arc, thread::Scope}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::{{dioxus_core::NoOpMutations, prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[test]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">test</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">test_hook</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        || </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">        |</span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">value</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">proxy</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> proxy.generation {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                value.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                assert_eq!(</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">value.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                value.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">2 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                proxy.</span><span style=\"color:#66d9ef;\">rerun</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">3 </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{}}\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">_ =&gt; </span><span style=\"color:#f8f8f2;\">todo!(),\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        |</span><span style=\"font-style:italic;color:#fd971f;\">proxy</span><span style=\"color:#f8f8f2;\">| assert_eq!(proxy.generation, </span><span style=\"color:#ff80f4;\">4</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">test_hook</span><span style=\"color:#f8f8f2;\">&lt;V: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt;(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">initialize</span><span style=\"color:#f8f8f2;\">: impl FnMut() -&gt; V + </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#fd971f;\">check</span><span style=\"color:#f8f8f2;\">: impl FnMut(</span><span style=\"font-style:italic;color:#fd971f;\">V</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">MockProxy</span><span style=\"color:#f8f8f2;\">) + </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">mut </span><span style=\"font-style:italic;color:#fd971f;\">final_check</span><span style=\"color:#f8f8f2;\">: impl FnMut(</span><span style=\"font-style:italic;color:#fd971f;\">MockProxy</span><span style=\"color:#f8f8f2;\">) + </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">    #[derive(Props)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">MockAppComponent&lt;I: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">, C: </span><span style=\"color:#f92672;\">&#39;static</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">        hook: Rc&lt;RefCell&lt;I&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">        check: Rc&lt;RefCell&lt;C&gt;&gt;,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;I, C&gt; PartialEq </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">MockAppComponent&lt;I, C&gt; {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">eq</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">_</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">Self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">bool </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">impl</span><span style=\"color:#f8f8f2;\">&lt;I, C&gt; Clone </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">MockAppComponent&lt;I, C&gt; {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">clone</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                hook: self.hook.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">                check: self.check.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">mock_app</span><span style=\"color:#f8f8f2;\">&lt;I: FnMut() -&gt; V, C: FnMut(V, MockProxy), V&gt;(\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#fd971f;\">props</span><span style=\"color:#f8f8f2;\">: MockAppComponent&lt;I, C&gt;,\n</span><span style=\"color:#f8f8f2;\">    ) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> props.hook.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">()();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        props.check.</span><span style=\"color:#66d9ef;\">borrow_mut</span><span style=\"color:#f8f8f2;\">()(value, MockProxy::new());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        rsx! {{ div {{}} }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> vdom </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">VirtualDom::new_with_props(\n</span><span style=\"color:#f8f8f2;\">        mock_app,\n</span><span style=\"color:#f8f8f2;\">        MockAppComponent {{\n</span><span style=\"color:#f8f8f2;\">            hook: Rc::new(RefCell::new(initialize)),\n</span><span style=\"color:#f8f8f2;\">            check: Rc::new(RefCell::new(check)),\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    vdom.</span><span style=\"color:#66d9ef;\">rebuild_in_place</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">while</span><span style=\"color:#f8f8f2;\"> vdom.</span><span style=\"color:#66d9ef;\">wait_for_work</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">now_or_never</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">is_some</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">        vdom.</span><span style=\"color:#66d9ef;\">render_immediate</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut</span><span style=\"color:#f8f8f2;\"> NoOpMutations);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    vdom.</span><span style=\"color:#66d9ef;\">in_runtime</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        ScopeId::</span><span style=\"color:#ff80f4;\">ROOT</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">in_runtime</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#66d9ef;\">final_check</span><span style=\"color:#f8f8f2;\">(MockProxy::new());\n</span><span style=\"color:#f8f8f2;\">        }})\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">MockProxy {{\n</span><span style=\"color:#f8f8f2;\">    rerender: Arc&lt;dyn Fn()&gt;,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">generation: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">MockProxy {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">new</span><span style=\"color:#f8f8f2;\">() -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> generation </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">generation</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> rerender </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">schedule_update</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Self </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            rerender,\n</span><span style=\"color:#f8f8f2;\">            generation,\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">rerun</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">        (self.rerender)();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "component_test.rs".to_string(),
        }
        h2 { id: "end-to-end-testing",
            a { href: "#end-to-end-testing", class: "header", "End to End Testing" }
        }
        p {
            "You can use "
            a { href: "https://playwright.dev/", "Playwright" }
            " to create end to end tests for your dioxus application."
        }
        p {
            "In your  "
            code { "playwright.config.js" }
            ", you will need to run cargo run or dx serve instead of the default build command. Here is a snippet from the end to end web example:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">//...\n</span><span style=\"color:#f8f8f2;\">webServer: [\n</span><span style=\"color:#f8f8f2;\">    {{\n</span><span style=\"color:#f8f8f2;\">        cwd: path.</span><span style=\"color:#66d9ef;\">join</span><span style=\"color:#f8f8f2;\">(process.</span><span style=\"color:#66d9ef;\">cwd</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#f92672;\">&#39;playwright-</span><span style=\"color:#f8f8f2;\">tests</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">web</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">        command: </span><span style=\"color:#f92672;\">&#39;dx</span><span style=\"color:#f8f8f2;\"> serve</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        port: </span><span style=\"color:#ff80f4;\">8080</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        timeout: </span><span style=\"color:#ff80f4;\">10 </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">60 </span><span style=\"color:#f92672;\">* </span><span style=\"color:#ff80f4;\">1000</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        reuseExistingServer: </span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">process.env.</span><span style=\"color:#ff80f4;\">CI</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        stdout: </span><span style=\"color:#ffee99;\">&quot;pipe&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">],</span></pre>\n",
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/web",
                    "Web example"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/liveview",
                    "Liveview example"
                }
            }
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/playwright-tests/fullstack",
                    "Fullstack example"
                }
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookExamples() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "examples",
            a { href: "#examples", class: "header", "Examples" }
        }
        p {
            "There's a "
            em { "lot" }
            " of these, so if you're having trouble implementing something, or you just want to see cool things"
        }
        p {
            "Each of the examples in the main repository also has a permalink attached, in case the main one doesn't work."
        }
        ul {
            li {
                a { href: "https://github.com/DioxusLabs/dioxus/tree/main/examples",
                    "Main list"
                }
                " - "
                a { href: "(https://github.com/DioxusLabs/dioxus/tree/7eccc7a104df013b06c104fc1275450d2747e78c/examples)",
                    "permalink"
                }
                " - This is the largest list."
            }
            li {
                "Package-specific examples from the "
                a { href: "https://github.com/DioxusLabs/dioxus/", "main repository" }
                ". To learn more about these packages, search them up on "
                a { href: "https://crates.io/", "crates.io" }
                ", or navigate from the examples to the root of the package."
                ul {
                    li {
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/web/examples",
                            "dioxus-web"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/7eccc7a104df013b06c104fc1275450d2747e78c/packages/web/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples",
                            "dioxus-fullstack"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/7eccc7a104df013b06c104fc1275450d2747e78c/packages/fullstack/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/liveview/examples",
                            "dioxus-liveview"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/7eccc7a104df013b06c104fc1275450d2747e78c/packages/liveview/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/router/examples",
                            "dioxus-router"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/7eccc7a104df013b06c104fc1275450d2747e78c/packages/router/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/blitz/tree/master/packages/dioxus-tui/examples",
                            "dioxus-tui"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/e118648346f764f39261868ad13efcc2aeb2fb21/packages/dioxus-tui/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/blitz/tree/master/packages/plasmo/examples",
                            "plasmo"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/blitz/tree/e118648346f764f39261868ad13efcc2aeb2fb21/packages/plasmo/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/rsx-rosetta/examples",
                            "rsx-rosetta"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/7eccc7a104df013b06c104fc1275450d2747e78c/packages/rsx-rosetta/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/blitz/tree/master/packages/native-core/examples",
                            "native-core"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/e118648346f764f39261868ad13efcc2aeb2fb21/packages/native-core/examples",
                            "permalink"
                        }
                    }
                    li {
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/signals/examples",
                            "signals"
                        }
                        " - "
                        a { href: "https://github.com/DioxusLabs/dioxus/tree/7eccc7a104df013b06c104fc1275450d2747e78c/packages/signals/examples",
                            "permalink"
                        }
                    }
                }
            }
        }
    }
}
#[component(no_case_check)]
pub fn CookbookTailwind() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "tailwind",
            a { href: "#tailwind", class: "header", "Tailwind" }
        }
        p {
            "You can style your Dioxus application with whatever CSS framework you choose, or just write vanilla CSS."
        }
        p {
            "One popular option for styling your Dioxus application is "
            a { href: "https://tailwindcss.com/", "Tailwind" }
            ". Tailwind allows you to style your elements with CSS utility classes. This guide will show you how to setup tailwind CSS with your Dioxus application."
        }
        h2 { id: "setup",
            a { href: "#setup", class: "header", "Setup" }
        }
        ol {
            li { "Install the Dioxus CLI:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install dioxus</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cli</span></pre>\n" }
        ol {
            li {
                "Install npm: "
                a { href: "https://docs.npmjs.com/downloading-and-installing-node-js-and-npm",
                    "https://docs.npmjs.com/downloading-and-installing-node-js-and-npm"
                }
            }
            li {
                "Install the tailwind css cli: "
                a { href: "https://tailwindcss.com/docs/installation",
                    "https://tailwindcss.com/docs/installation"
                }
            }
            li { "Initialize the tailwind css project:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">npx tailwindcss init</span></pre>\n" }
        p {
            "This should create a  "
            code { "tailwind.config.js" }
            " file in the root of the project."
        }
        ol {
            li {
                "Edit the "
                code { "tailwind.config.js" }
                " file to include rust files:"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">module.exports </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    mode: </span><span style=\"color:#ffee99;\">&quot;all&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    content: [\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// include all rust, html and css files in the src directory\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;./src/**/*.{{rs,html,css}}&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// include all html files in the output (dist) directory\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;./dist/**/*.html&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    ],\n</span><span style=\"color:#f8f8f2;\">    theme: {{\n</span><span style=\"color:#f8f8f2;\">        extend: {{}},\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    plugins: [],\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        ol {
            li {
                "Create a "
                code { "input.css" }
                " file in the root of your project with the following content:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">tailwind base;\n</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">tailwind components;\n</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">tailwind utilities;</span></pre>\n" }
        ol {
            li {
                "Add "
                a { href: "https://github.com/DioxusLabs/manganis", "Manganis" }
                " to your project to handle asset collection."
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add manganis</span></pre>\n" }
        ol {
            li {
                "Create a link to the "
                code { "tailwind.css" }
                " file using manganis somewhere in your rust code:"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Urls are relative to your Cargo.toml file\n</span><span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">TAILWIND_URL</span><span style=\"color:#f8f8f2;\">: Asset </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">asset!(</span><span style=\"color:#ffee99;\">&quot;/assets/tailwind.css&quot;</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
            name: "tailwind.rs".to_string(),
        }
        h3 { id: "bonus-steps",
            a { href: "#bonus-steps", class: "header", "Bonus Steps" }
        }
        ol {
            li { "Install the tailwind css vs code extension" }
            li {
                "Go to the settings for the extension and find the experimental regex support section. Edit the setting.json file to look like this:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#ffee99;\">&quot;tailwindCSS.experimental.classRegex&quot;</span><span style=\"color:#f8f8f2;\">: [</span><span style=\"color:#ffee99;\">&quot;class: </span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\">(.*)</span><span style=\"color:#ff80f4;\">\\&quot;</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">],\n</span><span style=\"color:#ffee99;\">&quot;tailwindCSS.includeLanguages&quot;</span><span style=\"color:#f8f8f2;\">: {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;rust&quot;</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#ffee99;\">&quot;html&quot;\n</span><span style=\"color:#f8f8f2;\">}},</span></pre>\n" }
        h2 { id: "development",
            a { href: "#development", class: "header", "Development" }
        }
        ul {
            li {
                "Run the following command in the root of the project to start the tailwind css compiler:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">npx tailwindcss </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">i .</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">input.css </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">o .</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">public</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">tailwind.css </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">watch</span></pre>\n" }
        h3 { id: "web",
            a { href: "#web", class: "header", "Web" }
        }
        ul {
            li { "Run the following command in the root of the project to start the dioxus dev server:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx serve</span></pre>\n" }
        ul {
            li {
                "Open the browser to "
                a { href: "http://localhost:8080", "http://localhost:8080" }
                "."
            }
        }
        h3 { id: "desktop",
            a { href: "#desktop", class: "header", "Desktop" }
        }
        ul {
            li { "Launch the dioxus desktop app:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx serve </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">platform desktop</span></pre>\n" }
    }
}
#[component(no_case_check)]
pub fn CookbookOptimizing() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "optimizing",
            a { href: "#optimizing", class: "header", "Optimizing" }
        }
        p {
            em {
                "Note: This is written primarily for the web, but the main optimizations will work on other platforms too."
            }
        }
        p {
            "You might have noticed that Dioxus binaries are pretty big."
            a { href: "https://github.com/tigerros/dioxus-todo-app", "TodoMVC app" }
            " weighs in at 2.36mb!"
        }
        p { "We will also discuss ways to optimize your app for increased speed." }
        p {
            "However, certain optimizations will sacrifice speed for decreased binary size or the other way around."
        }
        p {
            "To test binary sizes, we will use "
            a { href: "https://github.com/tigerros/dioxus-todo-app", "this" }
            " repository as a sample app."
            code { "no-optimizations" }
            " package will serve as the base, which weighs 2.36mb as of right now."
        }
        p { "Additional resources:" }
        ul {
            li {
                a { href: "https://rustwasm.github.io/docs/book/reference/code-size.html",
                    "WASM book - Shrinking  "
                    code { ".wasm" }
                    " code size"
                }
            }
            li {
                a { href: "https://github.com/johnthagen/min-sized-rust", "min-sized-rust" }
            }
        }
        h2 { id: "building-in-release-mode",
            a { href: "#building-in-release-mode", class: "header", "Building in release mode" }
        }
        p {
            "This is the best way to optimize. In fact, the 2.36mb figure at the start of the guide is with release mode."
        }
        p {
            "Thankfully, no matter what tool you're using to build your app, it will probably have a  "
            code { "--release" }
            " flag to do this."
        }
        p {
            "Using the "
            a { href: "https://dioxuslabs.com/learn/0.5/CLI", "Dioxus CLI" }
            " or "
            a { href: "https://trunkrs.dev/", "Trunk" }
            ":"
        }
        ul {
            li {
                "Dioxus CLI: "
                code { "dx build --release" }
            }
            li {
                "Trunk: "
                code { "trunk build --release" }
            }
        }
        h2 { id: "upx",
            a { href: "#upx", class: "header", "UPX" }
        }
        p {
            "If you're not targeting web, you can use the "
            a { href: "https://github.com/upx/upx", "UPX" }
            " CLI tool to compress your executables."
        }
        p { "Setup:" }
        ul {
            li {
                "Download a "
                a { href: "https://github.com/upx/upx/releases", "release" }
                " and extract the directory inside to a sensible location."
            }
            li { "Add the executable located in the directory to your path variable." }
        }
        p {
            "You can run  "
            code { "upx --help" }
            " to get the CLI options, but you should also view  "
            code { "upx-doc.html" }
            " for more detailed information."
        }
        p {
            "An example command might be:  "
            code { "upx --best -o target/release/compressed.exe target/release/your-executable.exe" }
            "."
        }
        h2 { id: "build-configuration",
            a { href: "#build-configuration", class: "header", "Build configuration" }
        }
        p {
            em {
                "Note: Settings defined in "
                code { ".cargo/config.toml" }
                " will override settings in "
                code { "Cargo.toml" }
                "."
            }
        }
        p {
            "Other than the  "
            code { "--release" }
            " flag, this is the easiest way to optimize your projects, and also the most effective way,"
        }
        h3 { id: "stable",
            a { href: "#stable", class: "header", "Stable" }
        }
        p {
            "This configuration is 100% stable and decreases the binary size from 2.36mb to 310kb."
            code { ".cargo/config.toml" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[profile.release]\n</span><span style=\"color:#f8f8f2;\">opt</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">level </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;z&quot;\n</span><span style=\"color:#f8f8f2;\">debug </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false\n</span><span style=\"color:#f8f8f2;\">lto </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">codegen</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">units </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">panic </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;abort&quot;\n</span><span style=\"color:#f8f8f2;\">strip </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">incremental </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false</span></pre>\n",
        }
        p { "Links to the documentation of each value:" }
        ul {
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#opt-level",
                    code { "opt-level" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#debuginfo",
                    code { "debug" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#lto",
                    code { "lto" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#codegen-units",
                    code { "codegen-units" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#panic",
                    code { "panic" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#strip",
                    code { "strip" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#incremental",
                    code { "incremental" }
                }
            }
        }
        h3 { id: "unstable",
            a { href: "#unstable", class: "header", "Unstable" }
        }
        p {
            "This configuration contains some unstable features, but it should work just fine."
            code { ".cargo/config.toml" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[unstable]\n</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">std </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;std&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;panic_abort&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;core&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;alloc&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">build</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">std</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;panic_immediate_abort&quot;</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">[build]\n</span><span style=\"color:#f8f8f2;\">rustflags </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;-Clto&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;-Zvirtual-function-elimination&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;-Zlocation-detail=none&quot;\n</span><span style=\"color:#f8f8f2;\">]\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Same </span><span style=\"color:#f92672;\">as in</span><span style=\"color:#f8f8f2;\"> the Stable section\n</span><span style=\"color:#f8f8f2;\">[profile.release]\n</span><span style=\"color:#f8f8f2;\">opt</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">level </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;z&quot;\n</span><span style=\"color:#f8f8f2;\">debug </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false\n</span><span style=\"color:#f8f8f2;\">lto </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">codegen</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">units </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">panic </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;abort&quot;\n</span><span style=\"color:#f8f8f2;\">strip </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">incremental </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">false</span></pre>\n",
        }
        p {
            em {
                "Note: The omitted space in each flag (e.g., "
                code { "-C<no space here>lto" }
                ") is intentional. It is not a typo."
            }
        }
        p {
            "The values in  "
            code { "[profile.release]" }
            " are documented in the "
            a { href: "#stable", "Stable" }
            " section. Links to the documentation of each value:"
        }
        ul {
            li {
                a { href: "https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags",
                    code { "[build.rustflags]" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/rustc/codegen-options/index.html#lto",
                    code { "-C lto" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/stable/unstable-book/compiler-flags/virtual-function-elimination.html",
                    code { "-Z virtual-function-elimination" }
                }
            }
            li {
                a { href: "https://doc.rust-lang.org/stable/unstable-book/compiler-flags/location-detail.html",
                    code { "-Z location-detail" }
                }
            }
        }
        h2 { id: "wasm-opt",
            a { href: "#wasm-opt", class: "header", "wasm-opt" }
        }
        p {
            em {
                "Note: In the future, "
                code { "wasm-opt" }
                " will be supported natively through the "
                a { href: "https://crates.io/crates/dioxus-cli", "Dioxus CLI" }
                "."
            }
        }
        p {
            code { "wasm-opt" }
            " is a tool from the "
            a { href: "https://github.com/WebAssembly/binaryen", "binaryen" }
            " library that optimizes your WASM files."
            a { href: "https://github.com/WebAssembly/binaryen/releases", "binaryen release" }
            " and run this command from the package directory:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">wasm</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">opt dist</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">assets</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">APP_NAME_bg.wasm </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">o dist</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">assets</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">dioxus</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">APP_NAME_bg.wasm </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">Oz</span></pre>\n" }
        p {
            "The  "
            code { "-Oz" }
            " flag specifies that  "
            code { "wasm-opt" }
            " should optimize for size. For speed, use  "
            code { "-O4" }
            "."
        }
        h2 { id: "improving-dioxus-code",
            a { href: "#improving-dioxus-code", class: "header", "Improving Dioxus code" }
        }
        p { "Let's talk about how you can improve your Dioxus code to be more performant." }
        p {
            "It's important to minimize the number of dynamic parts in your  "
            code { "rsx" }
            ", like conditional rendering."
            a { href: "../reference/dynamic_rendering", "Dynamic Rendering" }
            "."
        }
        p {
            "Also check out "
            a { href: "antipatterns", "Anti-patterns" }
            " for patterns that you should avoid."
        }
        h2 { id: "optimizing-the-size-of-assets",
            a { href: "#optimizing-the-size-of-assets", class: "header",
                "Optimizing the size of assets"
            }
        }
        p {
            "Assets can be a significant part of your app's size. Dioxus includes alpha support for first party "
            a { href: "../reference/assets", "assets" }
            ". Any assets you include with the "
            code { "mg!" }
            " macro will be optimized for production in release builds."
        }
    }
}
#[component(no_case_check)]
pub fn MigrationIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "how-to-upgrade-to-dioxus-05",
            a { href: "#how-to-upgrade-to-dioxus-05", class: "header",
                "How to Upgrade to Dioxus 0.5"
            }
        }
        p {
            "This guide will outline the API changes between the  "
            code { "0.4" }
            " and  "
            code { "0.5" }
            " releases. "
        }
        p {
            code { "0.5" }
            " has includes significant changes to hooks, props, and global state."
        }
        h2 { id: "cheat-sheet",
            a { href: "#cheat-sheet", class: "header", "Cheat Sheet" }
        }
        p { "Here is a quick cheat sheet for the changes:" }
        h3 { id: "scope",
            a { href: "#scope", class: "header", "Scope" }
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">/*...*/\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">provide_context</span><span style=\"color:#f8f8f2;\">({{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">/*...*/\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">/*...*/\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">/*...*/\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// In dioxus 0.5, the scope is no longer passed as an argument to the function\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Hooks, context, and spawn are now called directly\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_hook</span><span style=\"color:#f8f8f2;\">(|| {{ </span><span style=\"color:#8c8c8c;\">/*...*/ </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">provide_context</span><span style=\"color:#f8f8f2;\">({{ </span><span style=\"color:#8c8c8c;\">/*...*/ </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#8c8c8c;\">/*...*/ </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">/*...*/\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        h3 { id: "props",
            a { href: "#props", class: "header", "Props" }
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Comp</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope, </span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You pass in an owned prop, but inside the component, it is borrowed (name is the type &amp;String inside the function)\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> owned_name: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> name.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{owned_name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        BorrowedComp {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;{{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        ManualPropsComponent {{\n</span><span style=\"color:#f8f8f2;\">            name: name\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BorrowedComp</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;, </span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">) -&gt; Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ManualProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ManualPropsComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;ManualProps&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{cx.props.name}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// In dioxus 0.5, props are always owned. You pass in owned props and you get owned props in the body of the component\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Comp</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Name is owned here already (name is the type String inside the function)\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> owned_name: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> name;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{owned_name}}&quot;\n</span><span style=\"color:#f8f8f2;\">        BorrowedComp {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;other name&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        ManualPropsComponent {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#ffee99;\">&quot;other name 2&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Borrowed props are removed in dioxus 0.5. Mapped signals can act similarly to borrowed props if your props are borrowed from state\n</span><span style=\"color:#8c8c8c;\">// ReadOnlySignal is a copy wrapper over a state that will be automatically converted to\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BorrowedComp</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;String&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// In dioxus 0.5, props need to implement Props, Clone, and PartialEq\n</span><span style=\"color:#f8f8f2;\">#[derive(Props, Clone, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ManualProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Functions accept the props directly instead of the scope\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ManualPropsComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">props</span><span style=\"color:#f8f8f2;\">: ManualProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{props.name}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        p {
            "You can read more about the new props API in the "
            a { href: "migration/props", "Props Migration" }
            " guide."
        }
        h3 { id: "futures",
            a { href: "#futures", class: "header", "Futures" }
        }
        p { "Dioxus 0.4:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#66d9ef;\">use_future</span><span style=\"color:#f8f8f2;\">((dependency1, dependency2,), </span><span style=\"color:#f92672;\">move |</span><span style=\"color:#f8f8f2;\">(dependency1, dependency2,)</span><span style=\"color:#f92672;\">|</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#8c8c8c;\">/*use dependency1 and dependency2*/\n</span><span style=\"color:#f8f8f2;\">}});</span></pre>\n" }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// dependency1 and dependency2 must be Signal-like types like Signal, ReadOnlySignal, GlobalSignal, or another Resource\n</span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(|| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#8c8c8c;\">/*use dependency1 and dependency2*/ </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> non_reactive_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#8c8c8c;\">// You can also add non-reactive state to the resource hook with the use_reactive macro\n</span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(use_reactive!(|(</span><span style=\"font-style:italic;color:#fd971f;\">non_reactive_state</span><span style=\"color:#f8f8f2;\">,)| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    tokio::time::sleep(std::time::Duration::from_secs(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">    non_reactive_state </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">}}));</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        p {
            "Read more about the  "
            code { "use_resource" }
            " hook in the "
            a { href: "migration/hooks", "Hook Migration" }
            " guide."
        }
        h3 { id: "state-hooks",
            a { href: "#state-hooks", class: "header", "State Hooks" }
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> copy_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> clone_local_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_ref</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#66d9ef;\">use_shared_state_provider</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> clone_shared_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_shared_state::&lt;String&gt;(cx);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> copy_state_value </span><span style=\"color:#f92672;\">= **</span><span style=\"color:#f8f8f2;\">copy_state;\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> clone_local_state_value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> clone_local_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> clone_shared_state_value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> clone_shared_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx!{{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#ffee99;\">&quot;{{copy_state_value}}&quot;\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#ffee99;\">&quot;{{clone_shared_state_value}}&quot;\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"color:#ffee99;\">&quot;{{clone_local_state_value}}&quot;\n</span><span style=\"color:#f8f8f2;\">\tbutton {{\n</span><span style=\"color:#f8f8f2;\">\t\tonclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">\t\t\tcopy_state.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\t\t\t</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">clone_local_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;World&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\t\t\t</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">clone_shared_state.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;World&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\t\t}},\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#ffee99;\">&quot;Set State&quot;\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}})</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// You can now use signals for local copy state, local clone state, and shared state with the same API\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> copy_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> clone_shared_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| Signal::new(</span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">)));\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> clone_local_state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Call the signal like a function to clone the current value\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> copy_state_value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">copy_state</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#8c8c8c;\">// Or use the read method to borrow the current value\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> clone_local_state_value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> clone_local_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> clone_shared_state_value </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> clone_shared_state.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">rsx! {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;{{copy_state_value}}&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;{{clone_shared_state_value}}&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;{{clone_local_state_value}}&quot;\n</span><span style=\"color:#f8f8f2;\">    button {{\n</span><span style=\"color:#f8f8f2;\">        onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// All three states have the same API for updating the state\n</span><span style=\"color:#f8f8f2;\">            copy_state.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">            clone_shared_state.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;World&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">            clone_local_state.</span><span style=\"color:#66d9ef;\">set</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;World&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Set State&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        p {
            "Read more about the  "
            code { "use_signal" }
            " hook in the "
            a { href: "migration/state", "State Migration" }
            " guide."
        }
        h3 { id: "fermi",
            a { href: "#fermi", class: "header", "Fermi" }
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">fermi::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">: Atom&lt;String&gt; </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> Atom(|_| </span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_init_atom_root</span><span style=\"color:#f8f8f2;\">(cx);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_read</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;hello {{name}}!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Child {{}}\n</span><span style=\"color:#f8f8f2;\">        ChildWithRef {{}}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> set_name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_set</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#66d9ef;\">set_name</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;dioxus&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;reset name&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">: AtomRef&lt;Vec&lt;String&gt;&gt; </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> AtomRef(|_| vec![</span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ChildWithRef</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> names </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_atom_ref</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            ul {{\n</span><span style=\"color:#f8f8f2;\">                names.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">f</span><span style=\"color:#f8f8f2;\">| rsx!{{\n</span><span style=\"color:#f8f8f2;\">                    li {{ </span><span style=\"color:#ffee99;\">&quot;hello: {{f}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }})\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> names </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> names.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">                    cx.</span><span style=\"color:#66d9ef;\">spawn</span><span style=\"color:#f8f8f2;\">(async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        names.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;asd&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                    }})\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Add name&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Atoms and AtomRefs have been replaced with GlobalSignals\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;String&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| </span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// You can use global state directly without the use_read or use_set hooks\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;hello {{NAME}}!&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        Child {{}}\n</span><span style=\"color:#f8f8f2;\">        ChildWithRef {{}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| *</span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;dioxus&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;reset name&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Atoms and AtomRefs have been replaced with GlobalSignals\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;Vec&lt;String&gt;&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| vec![</span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ChildWithRef</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{\n</span><span style=\"color:#f8f8f2;\">            ul {{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> name </span><span style=\"color:#f92672;\">in </span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">read</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">iter</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">                    li {{ </span><span style=\"color:#ffee99;\">&quot;hello: {{name}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">            button {{\n</span><span style=\"color:#f8f8f2;\">                onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    </span><span style=\"color:#8c8c8c;\">// No need to clone the signal into futures, you can use it directly\n</span><span style=\"color:#f8f8f2;\">                    async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                        </span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">push</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;asd&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">                    }}\n</span><span style=\"color:#f8f8f2;\">                }},\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#ffee99;\">&quot;Add name&quot;\n</span><span style=\"color:#f8f8f2;\">            }}\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration.rs".to_string(),
        }
        p {
            "You can read more about global signals in the "
            a { href: "migration/fermi", "Fermi migration guide" }
            "."
        }
    }
}
#[component(no_case_check)]
pub fn MigrationHooks() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "hooks",
            a { href: "#hooks", class: "header", "Hooks" }
        }
        p {
            "Dioxus now uses signals as the backing for its state management. Signals are a smarter, more flexible version of the  "
            code { "use_ref" }
            " hook. Signals now back many hooks in dioxus to provide a more consistent and flexible API."
        }
        h3 { id: "state-hooks",
            a { href: "#state-hooks", class: "header", "State Hooks" }
        }
        p {
            "State hooks are now backed by signals.  "
            code { "use_state" }
            ",  "
            code { "use_ref" }
            ", and  "
            code { "use_shared_state" }
            " have been replaced with the  "
            code { "use_signal" }
            " hook. The  "
            code { "use_signal" }
            " hook is a more flexible and powerful version of the  "
            code { "use_ref" }
            " hook with smarter scopes that only subscribe to a signal if that signal is read within the scope. You can read more about the  "
            code { "use_signal" }
            " hook in the "
            a { href: "state", "State Migration" }
            " guide."
        }
        h3 { id: "async-hooks",
            a { href: "#async-hooks", class: "header", "Async Hooks" }
        }
        p {
            "The  "
            code { "use_future" }
            " hook has been replaced with the  "
            code { "use_resource" }
            " hook.  "
            code { "use_resource" }
            " automatically subscribes to any signals that are read within the closure instead of using a tuple of dependencies."
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">MyComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_resource </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_future</span><span style=\"color:#f8f8f2;\">(cx, (</span><span style=\"color:#f92672;\">**</span><span style=\"color:#f8f8f2;\">state,), |(</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">,)| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#8c8c8c;\">// start a request that depends on the state\n</span><span style=\"color:#f8f8f2;\">\t\tprintln!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\t}});\n</span><span style=\"color:#f8f8f2;\">\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#ffee99;\">&quot;{{state}}&quot;\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">MyComponent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// No need to manually set the dependencies, the use_resource hook will automatically detect signal dependencies\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_resource </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// start a request that depends on the state\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Because we read from the state signal, this future will be re-run whenever the state changes\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;{{state}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_hooks.rs".to_string(),
        }
        h3 { id: "dependencies",
            a { href: "#dependencies", class: "header", "Dependencies" }
        }
        p {
            "Some hooks including  "
            code { "use_effect" }
            " and  "
            code { "use_resource" }
            " now take a single closure with automatic subscriptions instead of a tuple of dependencies. You can read more about the  "
            code { "use_resource" }
            " hook in the "
            a { href: "hooks", "Hook Migration" }
            " guide."
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">HasDependencies</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_resource </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(cx, (</span><span style=\"color:#f92672;\">**</span><span style=\"color:#f8f8f2;\">state,), |(</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">,)| async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">\t\tprintln!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\t}});\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state_plus_one </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(cx, (</span><span style=\"color:#f92672;\">**</span><span style=\"color:#f8f8f2;\">state,), |(</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">,)| {{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#66d9ef;\">state</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">\t}});\n</span><span style=\"color:#f8f8f2;\">\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#ffee99;\">&quot;{{state_plus_one}}&quot;\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">HasDependencies</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// No need to manually set the dependencies, the use_resource hook will automatically detect signal dependencies\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> my_resource </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_resource</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move ||</span><span style=\"color:#f8f8f2;\"> async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Because we read from the state signal, this future will be re-run whenever the state changes\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{state}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state_plus_one </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_memo</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">move || </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Because we read from the state signal, this future will be re-run whenever the state changes\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#66d9ef;\">state</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">    }});\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;{{state_plus_one}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_hooks.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn MigrationState() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "state-migration",
            a { href: "#state-migration", class: "header", "State Migration" }
        }
        p {
            "The  "
            code { "use_state" }
            " and  "
            code { "use_ref" }
            " hooks have been replaced with the  "
            code { "use_signal" }
            " hook. The  "
            code { "use_signal" }
            " hook is a more flexible and powerful version of the  "
            code { "use_ref" }
            " hook with smarter scopes that only subscribe to a signal if that signal is read within the scope."
        }
        p {
            "With  "
            code { "use_state" }
            ", if you had this code:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\tChild {{\n</span><span style=\"color:#f8f8f2;\">\t\t\tstate: state.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">\t\t}}\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope, </span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">: UseState&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">\trender! {{\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#ffee99;\">&quot;{{state}}&quot;\n</span><span style=\"color:#f8f8f2;\">\t}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Parent would re-render every time the state changed even though only the child component was using the state. With the new  "
            code { "use_signal" }
            " hook, the parent would only re-render if the state was changed within the parent component:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Child {{ state }} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">: Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;{{state}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_state.rs".to_string(),
        }
        p {
            "Only the child component will re-render when the state changes because only the child component is reading the state."
        }
        h2 { id: "context-based-state",
            a { href: "#context-based-state", class: "header", "Context Based State" }
        }
        p {
            "The  "
            code { "use_shared_state_provider" }
            " and  "
            code { "use_shared_state" }
            " hooks have been replaced with using the  "
            code { "use_context_provider" }
            " and  "
            code { "use_context" }
            " hooks with a  "
            code { "Signal" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Create a new signal and provide it to the context API\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_context_provider</span><span style=\"color:#f8f8f2;\">(|| Signal::new(</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Child {{}} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Get the state from the context API\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">use_context::&lt;Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;&gt;();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;{{state}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_state.rs".to_string(),
        }
        p {
            "Signals are smart enough to handle subscribing to the right scopes without a special shared state hook."
        }
        h2 { id: "opting-out-of-subscriptions",
            a { href: "#opting-out-of-subscriptions", class: "header",
                "Opting Out of Subscriptions"
            }
        }
        p {
            "Some state hooks including  "
            code { "use_shared_state" }
            " and  "
            code { "use_ref" }
            " hooks had a function called  "
            code { "write_silent" }
            " in  "
            code { "0.4" }
            ". This function allowed you to update the state without triggering a re-render any subscribers. This function has been removed in  "
            code { "0.5" }
            "."
        }
        p {
            "Instead, you can use the  "
            code { "peek" }
            " function to read the current value of a signal without subscribing to it. This inverts the subscription model so that you can opt out of subscribing to a signal instead of opting all subscribers out of updates:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Even though we are reading the state, we don&#39;t need to subscribe to it\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> read_without_subscribing </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> state.</span><span style=\"color:#66d9ef;\">peek</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, state.</span><span style=\"color:#66d9ef;\">peek</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ Child {{ state }} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Child</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">state</span><span style=\"color:#f8f8f2;\">: Signal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                state </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">            }}, </span><span style=\"color:#ffee99;\">&quot;count is {{state}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_state.rs".to_string(),
        }
        p {
            code { "peek" }
            " gives you more fine-grained control over when you want to subscribe to a signal. This can be useful for performance optimizations and for updating state without re-rendering components."
        }
        h2 { id: "global-state",
            a { href: "#global-state", class: "header", "Global State" }
        }
        p {
            "In  "
            code { "0.4" }
            ", the fermi crate provided a separate global state API called atoms. In  "
            code { "0.5" }
            ", the  "
            code { "Signal" }
            " type has been extended to provide a global state API. You can use the  "
            code { "Signal::global" }
            " function to create a global signal:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        div {{ </span><span style=\"color:#ffee99;\">&quot;{{COUNT}}&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                </span><span style=\"color:#f92672;\">*</span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Increment&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_state.rs".to_string(),
        }
        p {
            "You can read more about global signals in the "
            a { href: "fermi", "Fermi migration guide" }
            "."
        }
    }
}
#[component(no_case_check)]
pub fn MigrationFermi() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "fermi",
            a { href: "#fermi", class: "header", "Fermi" }
        }
        p {
            "In dioxus 0.5, fermi atoms have been replaced with global signals and included in the main dioxus library."
        }
        p {
            "The new global signals can be used directly without hooks and include additional functionality like global memos."
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">fermi::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">: Atom&lt;String&gt; </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> Atom(|_| </span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">: AtomRef&lt;Vec&lt;String&gt;&gt; </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> AtomRef(|_| vec![</span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">use_init_atom_root</span><span style=\"color:#f8f8f2;\">(cx);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> set_name </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_set</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\t</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> names </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_atom_ref</span><span style=\"color:#f8f8f2;\">(cx, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">\t\t\tonclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#66d9ef;\">set_name</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;dioxus&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">\t\t\t</span><span style=\"color:#ffee99;\">&quot;reset name&quot;\n</span><span style=\"color:#f8f8f2;\">\t\t}}\n</span><span style=\"color:#f8f8f2;\">\t\t</span><span style=\"color:#ffee99;\">&quot;{{names.read():?}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">dioxus::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;String&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| </span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">());\n</span><span style=\"color:#8c8c8c;\">// Global signals work for copy and clone types in the same way\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">NAMES</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;Vec&lt;String&gt;&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| vec![</span><span style=\"color:#ffee99;\">&quot;world&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()]);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// No need to use use_init_atom_root, use_set, or use_atom_ref. Just use the global signal directly\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_| *</span><span style=\"color:#ff80f4;\">NAME</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;reset name&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;reset name&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{NAMES:?}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_fermi.rs".to_string(),
        }
        h2 { id: "memos",
            a { href: "#memos", class: "header", "Memos" }
        }
        p { "Dioxus 0.5 introduces global memos which can be used to store computed values globally." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">: GlobalSignal&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global(|| </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">MEMO</span><span style=\"color:#f8f8f2;\">: GlobalMemo&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">u32</span><span style=\"color:#f8f8f2;\">&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Signal::global_memo(|| </span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">GlobalMemo</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{ onclick: </span><span style=\"color:#f92672;\">move |_| *</span><span style=\"color:#ff80f4;\">COUNT</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">write</span><span style=\"color:#f8f8f2;\">() </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;increment&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#8c8c8c;\">// Global memos can be used like signals\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;{{MEMO}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_fermi.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn MigrationProps() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "props-migration",
            a { href: "#props-migration", class: "header", "Props Migration" }
        }
        p {
            "In dioxus 0.4, props are passed into the component through the scope. In dioxus 0.5, props are passed into the component through the props struct directly."
        }
        h2 { id: "owned-props",
            a { href: "#owned-props", class: "header", "Owned Props" }
        }
        p {
            "The props were borrowed with the lifetime from the scope. The props are cloned every render, and passed into the component as an owned value."
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Comp</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope, </span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// You pass in an owned prop, but inside the component, it is borrowed (name is the type &amp;String inside the function)\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> owned_name: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> name.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    cx.</span><span style=\"color:#66d9ef;\">render</span><span style=\"color:#f8f8f2;\">(rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{owned_name}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }})\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// In dioxus 0.5, props are always owned. You pass in owned props and you get owned props in the body of the component\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Comp</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: String) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Name is owned here already (name is the type String inside the function)\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> owned_name: </span><span style=\"font-style:italic;color:#66d9ef;\">String </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> name;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;Hello {{owned_name}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_props.rs".to_string(),
        }
        p {
            "Because props are cloned every render, making props Copy is recommended. You can easily make a field Copy by accepting  "
            code { "ReadOnlySignal<T>" }
            " instead of  "
            code { "T" }
            " in the props struct:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// In dioxus 0.5, props are always owned. You pass in owned props and you get owned props in the body of the component\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">CopyPropsComp</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: ReadOnlySignal&lt;String&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        button {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#8c8c8c;\">// You can easily copy the value of a signal into a closure\n</span><span style=\"color:#f8f8f2;\">            onclick: </span><span style=\"color:#f92672;\">move |_| </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                println!(</span><span style=\"color:#ffee99;\">&quot;Hello </span><span style=\"color:#ff80f4;\">{{name}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                async </span><span style=\"color:#f92672;\">move </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">                    println!(</span><span style=\"color:#ffee99;\">&quot;Hello </span><span style=\"color:#ff80f4;\">{{name}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">                }}\n</span><span style=\"color:#f8f8f2;\">            }},\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#ffee99;\">&quot;Click me&quot;\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">CopyPropsCompParent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ CopyPropsComp {{ name: </span><span style=\"color:#ffee99;\">&quot;World&quot; </span><span style=\"color:#f8f8f2;\">}} }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_props.rs".to_string(),
        }
        h2 { id: "borrowed-props",
            a { href: "#borrowed-props", class: "header", "Borrowed Props" }
        }
        p {
            "Borrowed props are removed in dioxus 0.5. Mapped signals can act similarly to borrowed props if your props are borrowed from state."
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(cx, || (</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;World&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()));\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        BorrowedComp {{\n</span><span style=\"color:#f8f8f2;\">            name: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">state.</span><span style=\"color:#66d9ef;\">get</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#ff80f4;\">1\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BorrowedComp</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;, </span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">) -&gt; Element&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">Parent</span><span style=\"color:#f8f8f2;\">() -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> state </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_signal</span><span style=\"color:#f8f8f2;\">(|| (</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;World&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    rsx! {{ BorrowedComp {{ name: state.</span><span style=\"color:#66d9ef;\">map</span><span style=\"color:#f8f8f2;\">(|</span><span style=\"font-style:italic;color:#fd971f;\">s</span><span style=\"color:#f8f8f2;\">| </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">s.</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">) }} }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[component]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">BorrowedComp</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: MappedSignal&lt;String&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;Hello {{name}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_props.rs".to_string(),
        }
        h2 { id: "manual-props",
            a { href: "#manual-props", class: "header", "Manual Props" }
        }
        p {
            "Manual prop structs in dioxus 0.5 need to derive  "
            code { "Clone" }
            " in addition to  "
            code { "Props" }
            " and  "
            code { "PartialEq" }
            ":"
        }
        p { "Dioxus 0.4:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ManualProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Functions accept the props directly instead of the scope\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ManualPropsComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">cx</span><span style=\"color:#f8f8f2;\">: Scope&lt;ManualProps&gt;) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    render! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#ffee99;\">&quot;Hello {{cx.props.name}}&quot;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Dioxus 0.5:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Props, Clone, PartialEq)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">ManualProps {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Functions accept the props directly instead of the component\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">ManualPropsComponent</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">props</span><span style=\"color:#f8f8f2;\">: ManualProps) -&gt; Element {{\n</span><span style=\"color:#f8f8f2;\">    rsx! {{</span><span style=\"color:#ffee99;\">&quot;Hello {{props.name}}&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
            name: "migration_props.rs".to_string(),
        }
    }
}
#[component(no_case_check)]
pub fn ContributingIndex() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "contributing",
            a { href: "#contributing", class: "header", "Contributing" }
        }
        p {
            "Development happens in the "
            a { href: "https://github.com/DioxusLabs/dioxus", "Dioxus GitHub repository" }
            ". If you've found a bug or have an idea for a feature, please submit an issue (but first check if someone hasn't "
            a { href: "https://github.com/DioxusLabs/dioxus/issues", "done it already" }
            ")."
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/discussions", "GitHub discussions" }
            " can be used as a place to ask for help or talk about features. You can also join "
            a { href: "https://discord.gg/XgGxMSkvUM", "our Discord channel" }
            " where some development discussion happens."
        }
        h2 { id: "improving-docs",
            a { href: "#improving-docs", class: "header", "Improving Docs" }
        }
        p {
            "If you'd like to improve the docs, PRs are welcome! The Rust docs ("
            a { href: "https://github.com/DioxusLabs/dioxus/tree/main/packages", "source" }
            ") and this guide ("
            a { href: "https://github.com/DioxusLabs/docsite/tree/main/docs-src/0.5/en",
                "source"
            }
            ") can be found in their respective GitHub repos."
        }
        h2 { id: "working-on-the-ecosystem",
            a { href: "#working-on-the-ecosystem", class: "header", "Working on the Ecosystem" }
        }
        p {
            "Part of what makes React great is the rich ecosystem. We'd like the same for Dioxus! So if you have a library in mind that you'd like to write and many people would benefit from, it will be appreciated. You can "
            a { href: "https://www.npmjs.com/search?q=keywords:react-component", "browse npm.js" }
            " for inspiration. Once you are done, add your library to the "
            a { href: "https://github.com/DioxusLabs/awesome-dioxus", "awesome dioxus" }
            " list or share it in the "
            code { "#I-made-a-thing" }
            " channel on "
            a { href: "https://discord.gg/XgGxMSkvUM", "Discord" }
            "."
        }
        h2 { id: "bugs--features",
            a { href: "#bugs--features", class: "header", "Bugs & Features" }
        }
        p {
            "If you've fixed "
            a { href: "https://github.com/DioxusLabs/dioxus/issues", "an open issue" }
            ", feel free to submit a PR! You can also take a look at "
            a { href: "contributing/./roadmap", "the roadmap" }
            " and work on something in there. Consider "
            a { href: "https://discord.gg/XgGxMSkvUM", "reaching out" }
            " to the team first to make sure everyone's on the same page, and you don't do useless work!"
        }
        p {
            "All pull requests (including those made by a team member) must be approved by at least one other team member."
        }
        h2 { id: "before-you-contribute",
            a { href: "#before-you-contribute", class: "header", "Before you contribute" }
        }
        p {
            "You might be surprised that a lot of checks fail when making your first PR."
            em { "lots" }
            " of time, because the"
        }
        ul {
            li {
                "Format code with "
                a { href: "https://github.com/rust-lang/rustfmt", "rustfmt" }
                ":"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo fmt </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\"> src</span><span style=\"color:#8c8c8c;\">/**/</span><span style=\"color:#f92672;\">**</span><span style=\"color:#f8f8f2;\">.rs</span></pre>\n" }
        ul {
            li {
                "You might need to install some packages on Linux (Ubuntu/deb) before the following commands will complete successfully (there is also a Nix flake in the repo root):"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">sudo apt install libgdk3.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">cil libatk1.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libcairo2</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libpango1.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libgdk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">pixbuf2.</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libsoup</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">3.0</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libjavascriptcoregtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">4.1</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev libwebkit2gtk</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">4.1</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">dev</span></pre>\n",
        }
        ul {
            li {
                "Check all code "
                a { href: "https://doc.rust-lang.org/cargo/commands/cargo-check.html",
                    "cargo check"
                }
                ":"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo check </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">workspace </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">examples </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">tests</span></pre>\n" }
        ul {
            li {
                "Check if "
                a { href: "https://doc.rust-lang.org/clippy/", "Clippy" }
                " generates any warnings. Please fix these!"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo clippy </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">workspace </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">examples </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">tests </span><span style=\"color:#f92672;\">-- -</span><span style=\"color:#f8f8f2;\">D warnings</span></pre>\n" }
        ul {
            li {
                "Test all code with "
                a { href: "https://doc.rust-lang.org/cargo/commands/cargo-test.html",
                    "cargo-test"
                }
                ":"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo test </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">all </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">tests</span></pre>\n" }
        ul {
            li {
                "More tests, this time with "
                a { href: "https://sagiegurari.github.io/cargo-make/", "cargo-make" }
                ". Here are all steps, including installation:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">force cargo</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">make\n</span><span style=\"color:#f8f8f2;\">cargo make tests</span></pre>\n" }
        ul {
            li {
                "Test with Playwright. This tests the UI itself, right in a browser. Here are all steps, including installation:"
                strong {
                    "Disclaimer: This might inexplicably fail on your machine without it being your fault."
                }
                " Make that PR anyway!"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cd playwright</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">tests\n</span><span style=\"color:#f8f8f2;\">npm ci\n</span><span style=\"color:#f8f8f2;\">npm install </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">D </span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">playwright</span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\">test\n</span><span style=\"color:#f8f8f2;\">npx playwright install </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">with</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">deps\n</span><span style=\"color:#f8f8f2;\">npx playwright test</span></pre>\n" }
        h2 { id: "how-to-test-dioxus-with-local-crate",
            a { href: "#how-to-test-dioxus-with-local-crate", class: "header",
                "How to test dioxus with local crate"
            }
        }
        p {
            "If you are developing a feature, you should test it in your local setup before raising a PR. This process makes sure you are aware of your code functionality before being reviewed by peers."
        }
        ul {
            li { "Fork the following github repo (DioxusLabs/dioxus):" }
        }
        p {
            code { "https://github.com/DioxusLabs/dioxus" }
        }
        ul {
            li {
                "Create a new or use an existing rust crate (ignore this step if you will use an existing rust crate):"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">bin demo</span></pre>\n" }
        ul {
            li { "Add the dioxus dependency to your rust crate (new/existing) in Cargo.toml:" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dioxus </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ path </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;&lt;path to forked dioxus project&gt;/dioxus/packages/dioxus&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;web&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;router&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n" }
        p {
            "This above example is for dioxus-web, with dioxus-router. To know about the dependencies for different renderer visit "
            a { href: "https://dioxuslabs.com/learn/0.5/getting_started", "here" }
            "."
        }
        ul {
            li { "Run and test your feature" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">dx serve</span></pre>\n" }
        p {
            "If this is your first time with dioxus, please read "
            a { href: "https://dioxuslabs.com/learn/0.5/guide", "the guide" }
            " to get familiar with dioxus."
        }
    }
}
#[component(no_case_check)]
pub fn ContributingGuidingPrinciples() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "overall-goals",
            a { href: "#overall-goals", class: "header", "Overall Goals" }
        }
        p {
            "This document outlines some of the overall goals for Dioxus. These goals are not set in stone, but they represent general guidelines for the project."
        }
        p {
            "The goal of Dioxus is to make it easy to build "
            strong { "cross-platform applications that scale" }
            "."
        }
        h2 { id: "cross-platform",
            a { href: "#cross-platform", class: "header", "Cross-Platform" }
        }
        p {
            "Dioxus is designed to be cross-platform by default. This means that it should be easy to build applications that run on the web, desktop, and mobile. However, Dioxus should also be flexible enough to allow users to opt into platform-specific features when needed. The  "
            code { "use_eval" }
            " is one example of this. By default, Dioxus does not assume that the platform supports JavaScript, but it does provide a hook that allows users to opt into JavaScript when needed."
        }
        h2 { id: "performance",
            a { href: "#performance", class: "header", "Performance" }
        }
        p {
            "As Dioxus applications grow, they should remain relatively performant without the need for manual optimizations. There will be cases where manual optimizations are needed, but Dioxus should try to make these cases as rare as possible."
        }
        p {
            "One of the benefits of the core architecture of Dioxus is that it delivers reasonable performance even when components are rerendered often. It is based on a Virtual Dom which performs diffing which should prevent unnecessary re-renders even when large parts of the component tree are rerun. On top of this, Dioxus groups static parts of the RSX tree together to skip diffing them entirely."
        }
        h2 { id: "type-safety",
            a { href: "#type-safety", class: "header", "Type Safety" }
        }
        p {
            "As teams grow, the Type safety of Rust is a huge advantage. Dioxus should leverage this advantage to make it easy to build applications with large teams."
        }
        p {
            "To take full advantage of Rust's type system, Dioxus should try to avoid exposing public  "
            code { "Any" }
            " types and string-ly typed APIs where possible."
        }
        h2 { id: "developer-experience",
            a { href: "#developer-experience", class: "header", "Developer Experience" }
        }
        p { "Dioxus should be easy to learn and ergonomic to use." }
        ul {
            li {
                p {
                    "The API of Dioxus attempts to remain close to React's API where possible. This makes it easier for people to learn Dioxus if they already know React"
                }
            }
            li {
                p {
                    "We can avoid the tradeoff between simplicity and flexibility by providing multiple layers of API: One for the very common use case, one for low-level control"
                }
                ul {
                    li {
                        "Hooks: the hooks crate has the most common use cases, but "
                        code { "use_hook" }
                        " provides a way to access the underlying persistent value if needed."
                    }
                    li {
                        "The builder pattern in platform Configs: The builder pattern is used to default to the most common use case, but users can change the defaults if needed."
                    }
                }
            }
            li {
                p { "Documentation:" }
                ul {
                    li { "All public APIs should have rust documentation" }
                    li {
                        "Examples should be provided for all public features. These examples both serve as documentation and testing. They are checked by CI to ensure that they continue to compile"
                    }
                    li { "The most common workflows should be documented in the guide" }
                }
            }
        }
    }
}
#[component(no_case_check)]
pub fn ContributingRoadmap() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {}
}

use super::*;
