use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::UNIX_EPOCH;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use maud::{html, Markup, PreEscaped, DOCTYPE};

const NAME: &str = "달토깽";
const AVATAR: &str = "avatar.jpg";
const SITE_ROOT: &str = "https://lunabunn.github.io";
// const SITE_ROOT: &str = "http://127.0.0.1:5500/dist";

struct SocialMedia(&'static str, &'static str);
const SOCIAL_MEDIAS: &[SocialMedia] = &[
    SocialMedia("fa-discord", "루나#8966"),
    SocialMedia("fa-twitter", "@iamrabbitmoon"),
    SocialMedia("fa-github", "github.com/lunabunn"),
];

fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="ko" {
            head {
                meta charset="UTF-8";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href={(SITE_ROOT) "/style.css"};
                script src="https://kit.fontawesome.com/f0733eaba3.js" crossorigin="anonymous" {}
                title { (title) }
            }
            body {
                aside#sidebar {
                    section {
                        a href=(SITE_ROOT) {
                            img src={(SITE_ROOT) "/" (AVATAR)} alt="" class="avatar";
                            h1.text-highlight.center { (NAME) }
                        }
                        @if SOCIAL_MEDIAS.len() > 0 {
                            p.text-highlight.center {
                                i.fab.(SOCIAL_MEDIAS[0].0).tooltip {
                                    span.tooltiptext { (SOCIAL_MEDIAS[0].1) }
                                }
                                @for elem in &SOCIAL_MEDIAS[1..] {
                                    (PreEscaped("&nbsp;")) (PreEscaped("&nbsp;"))
                                    i.fab.(elem.0).tooltip {
                                        span.tooltiptext { (elem.1) }
                                    }
                                }
                                br; br; br;
                            }
                        }
                    }
                }

                #main { (content) }

                script src={(SITE_ROOT) "/script.js"} {}
            }
        }
    }
}

fn post(title: &str, datetime: DateTime<Utc>, content: Markup) -> Markup {
    html! {
        article {
            h1.post-title { (title) }
            .timestamp {
                span { "마지막 갱신: " }
                time datetime=(datetime.to_rfc3339()) {}
            }

            (content)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    fn recurse_posts(
        source: &str,
        target: String,
        matter: &Matter<YAML>,
    ) -> Result<(), Box<dyn Error>> {
        if !Path::new(&target).exists() {
            fs::create_dir_all(&target)?;
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();

            let metadata = fs::metadata(&path)?;
            let (sec, nsec) = match metadata.modified()?.duration_since(UNIX_EPOCH) {
                Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
                Err(e) => {
                    let dur = e.duration();
                    let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
                    if nsec == 0 {
                        (-sec, 0)
                    } else {
                        (-sec - 1, 1_000_000_000 - nsec)
                    }
                }
            };
            let last_modified = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(sec, nsec));

            if path.is_dir() {
                recurse_posts(
                    path.to_str().unwrap(),
                    format!("{}/{}", target, path.file_name().unwrap().to_str().unwrap()),
                    &matter,
                )?;
            } else if path.extension().unwrap() == "html" {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let result = matter.parse(&fs::read_to_string(&path)?);
                let title = result
                    .data
                    .as_ref()
                    .map(|data| data["title"].as_string().ok())
                    .flatten()
                    .unwrap_or(file_stem.to_string());

                let mut f = File::create(format!("{}/{}.html", target, file_stem))?;
                f.write_all(
                    page(
                        &title,
                        post(
                            &title,
                            last_modified,
                            html! {
                                (PreEscaped(result.content))
                            },
                        ),
                    )
                    .into_string()
                    .as_bytes(),
                )?;
            }
        }

        Ok(())
    }

    let matter = Matter::<YAML>::new();
    recurse_posts(
        concat!(env!("CARGO_MANIFEST_DIR"), "/pages"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/dist").to_string(),
        &matter,
    )?;

    Ok(())
}
