#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    pub name: &'static str,
    pub name_arabic: &'static str,
    pub title: &'static str,
    pub org: &'static str,
}

pub const BIO: &str = "TODO: migrate bio content from TypeScript.";

pub const LINKS: &[Link] = &[
    Link {
        label: "bluesky",
        url: "https://bsky.app/profile/ay-man.com",
    },
    Link {
        label: "linkedin",
        url: "https://linkedin.com/in/aymanbolad",
    },
    Link {
        label: "website",
        url: "https://ay-man.com",
    },
];

pub const IDENTITY: Identity = Identity {
    name: "ayman",
    name_arabic: "أيمن",
    title: "scientist — research data",
    org: "model driven drug discovery",
};

pub const GITHUB_USERNAME: &str = "ay-mxn";
