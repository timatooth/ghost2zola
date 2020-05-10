use html2md::parse_html;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    html: String,
    slug: String,
    created_at: String,
}

fn main() {
    let content_api_key =
        env::var("GHOST_CONTENT_API_KEY").expect("GHOST_CONTENT_API_KEY must be supplied");
    let url = format!(
        "http://localhost:2368/ghost/api/v3/content/posts/?key={}&limit=all",
        content_api_key
    );
    let resp = reqwest::blocking::get(&url)
        .unwrap()
        .json::<Value>()
        .unwrap();

    let posts: Vec<Post> = serde_json::from_value(resp["posts"].clone()).unwrap();

    for post in posts {
        write_markdown_file(post)
    }
}

fn write_markdown_file(post: Post) {
    let mut frontmatter = String::from("+++\n");
    frontmatter.push_str(&format!("title = \"{}\"\n", post.title));
    frontmatter.push_str(&format!("slug = \"{}\"\n", post.slug));
    frontmatter.push_str(&format!("date = {}\n", post.created_at));
    frontmatter.push_str("+++\n\n");
    frontmatter.push_str(&parse_html(&post.html));

    // regex clobbering replacing img tags with resize shortcode
    let pattern_str = r##"<img src="http://localhost:2368/content/images/(?P<image>\d+/\d+/.*\.(?:jpe?g|png|webp))".*"##;
    let re = Regex::new(pattern_str).unwrap();
    let after = re.replace_all(
        &frontmatter,
        "\n{{ resize_image(path=\"images/$image\", width=1000, op=\"fit_width\") }}\n",
    );

    // regex clobbering image markdown syntax to use a custom resizer shortcode template
    let pattern_str = r##"!\[\]\(http://localhost:2368/content/(?P<image>images/\d+/\d+/.*?\.(?:jpe?g|png|webp))\)"##;
    let re = Regex::new(pattern_str).unwrap();
    let after = re.replace_all(
        &after,
        "\n{{ resize_image(path=\"$image\", width=1000, op=\"fit_width\") }}\n",
    );

    fs::write(format!("gen/{}.md", post.slug), String::from(after)).expect("Unable to write file");
}
