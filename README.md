# ghost2zola
> Exports posts from Ghost Conntent API (v3) to Zola markdown.

This project is minimal effort:
* Converts Ghost Posts HTML to markdown
* Adds frontmatter to posts
* Replaces localhost image links and coverts to a `resize_image` shortcode for Zola to use it's image resizer from the original Ghost `content/images`.

#### Requirements

* a Ghost instance running at http://localhost:2368 with your content.
* a Ghost content API key (get one in *Integrations >> Custom Integrations >> Add Custom Integration*)

#### Usage

1. Run your Ghost site locally
2. `export GHOST_CONTENT_API_KEY=886c434742d9b11f0054a3af62` (changeme)
3. `cargo run`
4. Copy over content in `/gen` into your existing Zola site structure
5. Copy over `/var/www/ghost/content/images/*` to the right place in Zola

###### Maybe later
- [ ] Generate a Zola skeleton site + shortcodes to run `zola serve` straight away.
- [ ] Import pages
- [ ] Copy images into `content/`
- [ ] Use the Admin API to pull in unpublished drafts too
- [ ] Taxonomy
- [ ] Automate for CI