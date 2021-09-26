use std::error::Error;
use std::fs::File;
use std::io::Write;

use maud::{html, Markup, PreEscaped, DOCTYPE};

const NAME: &str = "달토깽";
const AVATAR: &str = "avatar.jpg";

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
                link rel="stylesheet" href="style.css";
                script src="https://kit.fontawesome.com/f0733eaba3.js" crossorigin="anonymous" {}
                title { (title) }
            }
            body {
                aside#sidebar {
                    section {
                        img src=(AVATAR) alt="" class="avatar";
                        h1.text-highlight.center { (NAME) }
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

                script src="script.js" {}
            }
        }
    }
}

fn post(title: &str, content: Markup) -> Markup {
    html! {
        article {
            h1.post-title { (title) }
            .timestamp {
                span { "마지막 갱신: " }
                time datetime="2021-09-25" { "2021. 9. 25." }
            }

            (content)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::create(format!("{}/dist/index.html", env!("CARGO_MANIFEST_DIR")))?;
    f.write_all(
        page(
            "lunabunn",
            post(
                "프로필 정보",
                html! {
                    table style="max-width: 500px; text-align: center;" {
                        tr {
                            td { strong { "이름" } }
                            td { "달토깽 (문루나)" }
                        }
                        tr {
                            td { strong { "나이" } }
                            td { "3살" }
                        }
                        tr {
                            td { strong { "성별" } }
                            td { "남자" }
                        }
                        tr {
                            td { strong { "사는 곳" } }
                            td { "달" }
                        }
                        tr {
                            td colspan="2" { strong { "취미" } }
                        }
                        tr {
                            td colspan="2" { "프로그래밍, 게임 개발, 노래, 작곡, 그림 + α" }
                        }
                        tr {
                            td colspan="2" { strong { "좋아하는 것" } }
                        }
                        tr {
                            td colspan="2" { "침대, 핫초코, 귀여운 사람" }
                        }
                        tr {
                            td colspan="2" { strong { "싫어하는 것" } }
                        }
                        tr {
                            td colspan="2" { "민트초코" }
                        }
                    }
                },
            ),
        )
        .into_string()
        .as_bytes(),
    )?;

    Ok(())
}
