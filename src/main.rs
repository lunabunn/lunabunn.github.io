use std::error::Error;
use std::fs::File;
use std::io::Write;

use maud::{DOCTYPE, Markup, PreEscaped, html};

const NBSP: PreEscaped<&str> = PreEscaped("&nbsp;");

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
        html! {
            (DOCTYPE)
            html lang="ko" {
                head {
                    meta charset="UTF-8";
                    meta http-equiv="X-UA-Compatible" content="IE=edge";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    link rel="stylesheet" href="style.css";
                    script src="https://kit.fontawesome.com/f0733eaba3.js" crossorigin="anonymous" {}
                    title { "lunabunn" }
                }
                body {
                    aside#sidebar {
                        section {
                            img src="avatar.jpg" alt="" class="avatar";
                            h1.text-highlight.center { "달토깽" }
                            p.text-highlight.center {
                                i.fab.fa-discord.tooltip {
                                    span.tooltiptext { "lunabunn#6779" }
                                }
                                (NBSP)(NBSP)
                                i.fab.fa-twitter.tooltip {
                                    span.tooltiptext {
                                        "@iamrabbitmoon"
                                    }
                                }
                                (NBSP)(NBSP)
                                i.fab.fa-github.tooltip {
                                    span.tooltiptext {
                                        "github.com/lunabunn"
                                    }
                                }
                                br; br; br;
                            }
                        }
                    }

                    #main {
                        (post("프로필 정보", html! {
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
                        }))
                    }

                    script src="script.js" {}
                }
            }
        }
        .into_string()
        .as_bytes(),
    )?;

    Ok(())
}
