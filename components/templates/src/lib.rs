#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;
extern crate base64;
extern crate csv;
extern crate pulldown_cmark;
extern crate reqwest;
extern crate url;

#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(not(test))]
extern crate serde_json;

extern crate config;
extern crate errors;
extern crate imageproc;
extern crate library;
extern crate utils;

pub mod filters;
pub mod global_fns;

use tera::{Context, Tera};

use errors::{Result, ResultExt};

lazy_static! {
    pub static ref ZOLA_TERA: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("404.html", include_str!("builtins/404.html")),
            ("rss.xml", include_str!("builtins/rss.xml")),
            ("sitemap.xml", include_str!("builtins/sitemap.xml")),
            ("multi_sitemap.xml", include_str!("builtins/multi_sitemap.xml")),
            ("robots.txt", include_str!("builtins/robots.txt")),
            ("anchor-link.html", include_str!("builtins/anchor-link.html")),
            ("shortcodes/youtube.html", include_str!("builtins/shortcodes/youtube.html")),
            ("shortcodes/vimeo.html", include_str!("builtins/shortcodes/vimeo.html")),
            ("shortcodes/gist.html", include_str!("builtins/shortcodes/gist.html")),
            ("shortcodes/streamable.html", include_str!("builtins/shortcodes/streamable.html")),
            ("internal/alias.html", include_str!("builtins/internal/alias.html")),
        ])
        .unwrap();
        tera.register_filter("markdown", filters::markdown);
        tera.register_filter("base64_encode", filters::base64_encode);
        tera.register_filter("base64_decode", filters::base64_decode);
        tera
    };
}

/// Renders the `internal/alias.html` template that will redirect
/// via refresh to the url given
pub fn render_redirect_template(url: &str, tera: &Tera) -> Result<String> {
    let mut context = Context::new();
    context.insert("url", &url);

    tera.render("internal/alias.html", &context)
        .chain_err(|| format!("Failed to render alias for '{}'", url))
}
