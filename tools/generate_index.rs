// ============================================================================
// generate_index — regenerate the HTML pages from the README.md files.
// ============================================================================
//
// On-brand with the rest of the repo: one standalone Rust file, std only,
// no Cargo. It converts the Markdown subset used by our READMEs.
//
//   challenges/index.html       is generated from the top-level README.md
//   challenges/<ch>/index.html  is generated from each challenge's README.md
//
// Build and run from the repo root:
//
//     make html
//
// or by hand:
//
//     rustc --edition 2024 tools/generate_index.rs -o .build/generate_index
//     .build/generate_index
//
// Do not edit the generated .html files by hand.
// ============================================================================

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let root = Path::new(".");
    let challenges = root.join("challenges");
    if !challenges.is_dir() {
        eprintln!("error: challenges/ not found — run from the repo root");
        std::process::exit(2);
    }

    // Top index, from the top-level README.
    let readme = fs::read_to_string(root.join("README.md"))?;
    let top = page(
        "Rusty SRE — Rust Challenges",
        "",
        &md_to_html(&readme, top_link_rewrite),
    );
    fs::write(challenges.join("index.html"), top)?;
    println!("wrote challenges/index.html");

    // One page per challenge, from its README.
    for dir in challenge_dirs(&challenges)? {
        let md_path = dir.join("README.md");
        let Ok(md) = fs::read_to_string(&md_path) else {
            println!("skipped {} (no README.md)", dir.display());
            continue;
        };
        let title = first_heading(&md)
            .unwrap_or_else(|| dir.file_name().unwrap().to_string_lossy().into_owned());
        let nav = "<nav><a href=\"../index.html\">&larr; All challenges</a>\
                   <span class=\"spacer\"></span></nav>\n";
        let html = page(&format!("Rusty SRE — {title}"), nav, &md_to_html(&md, no_rewrite));
        fs::write(dir.join("index.html"), html)?;
        println!("wrote {}/index.html", dir.display());
    }
    Ok(())
}

fn challenge_dirs(challenges: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(challenges)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(|c: char| c.is_ascii_digit()))
        })
        .collect();
    dirs.sort();
    Ok(dirs)
}

fn first_heading(md: &str) -> Option<String> {
    md.lines()
        .find_map(|line| line.strip_prefix("# "))
        .map(|title| title.replace(['`', '*'], ""))
}

// ----------------------------------------------------------------------------
// Link rewriting
// ----------------------------------------------------------------------------

/// Adjust top-level README links so they work from challenges/index.html.
fn top_link_rewrite(target: &str) -> String {
    if let Some(rest) = target.strip_prefix("challenges/") {
        if rest == "index.html" {
            return "index.html".to_string();
        }
        return format!("{}/index.html", rest.trim_end_matches('/'));
    }
    if target.starts_with("http://") || target.starts_with("https://") || target.starts_with('#') {
        return target.to_string();
    }
    // README.md, STUDY_GUIDE.md, Makefile, ... live one level up.
    format!("../{target}")
}

/// Challenge READMEs already use links relative to their own directory.
fn no_rewrite(target: &str) -> String {
    target.to_string()
}

// ----------------------------------------------------------------------------
// Markdown subset → HTML
// ----------------------------------------------------------------------------

fn md_to_html(md: &str, rewrite: fn(&str) -> String) -> String {
    let lines: Vec<&str> = md.lines().collect();
    let mut out = String::new();
    let mut open_list: Option<&'static str> = None;
    let mut i = 0;

    fn close_list(out: &mut String, open_list: &mut Option<&'static str>) {
        if let Some(tag) = open_list.take() {
            out.push_str(&format!("</{tag}>\n"));
        }
    }

    while i < lines.len() {
        let stripped = lines[i].trim();

        // Fenced code block.
        if stripped.starts_with("```") {
            close_list(&mut out, &mut open_list);
            i += 1;
            let mut code = Vec::new();
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                code.push(lines[i]);
                i += 1;
            }
            i += 1; // skip the closing fence
            out.push_str("<pre><code>");
            out.push_str(&escape_html(&code.join("\n")));
            out.push_str("</code></pre>\n");
            continue;
        }

        // Table.
        if stripped.starts_with('|') {
            close_list(&mut out, &mut open_list);
            let mut rows = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') {
                rows.push(lines[i].trim());
                i += 1;
            }
            out.push_str(&render_table(&rows, rewrite));
            continue;
        }

        // Heading.
        let hashes = stripped.bytes().take_while(|&b| b == b'#').count();
        if (1..=4).contains(&hashes) && stripped[hashes..].starts_with(' ') {
            close_list(&mut out, &mut open_list);
            let text = render_inline(stripped[hashes + 1..].trim(), rewrite);
            out.push_str(&format!("<h{hashes}>{text}</h{hashes}>\n"));
            i += 1;
            continue;
        }

        // List items.
        if let Some(item) = unordered_item(stripped) {
            if open_list != Some("ul") {
                close_list(&mut out, &mut open_list);
                out.push_str("<ul>\n");
                open_list = Some("ul");
            }
            out.push_str(&format!("<li>{}</li>\n", render_inline(item, rewrite)));
            i += 1;
            continue;
        }
        if let Some(item) = ordered_item(stripped) {
            if open_list != Some("ol") {
                close_list(&mut out, &mut open_list);
                out.push_str("<ol>\n");
                open_list = Some("ol");
            }
            out.push_str(&format!("<li>{}</li>\n", render_inline(item, rewrite)));
            i += 1;
            continue;
        }

        if stripped.is_empty() {
            close_list(&mut out, &mut open_list);
            i += 1;
            continue;
        }

        // Paragraph: gather consecutive plain lines.
        close_list(&mut out, &mut open_list);
        let mut para = vec![stripped];
        i += 1;
        while i < lines.len() {
            let next = lines[i].trim();
            if next.is_empty()
                || next.starts_with('|')
                || next.starts_with('#')
                || next.starts_with("```")
                || unordered_item(next).is_some()
                || ordered_item(next).is_some()
            {
                break;
            }
            para.push(next);
            i += 1;
        }
        out.push_str(&format!("<p>{}</p>\n", render_inline(&para.join(" "), rewrite)));
    }
    close_list(&mut out, &mut open_list);
    out
}

fn unordered_item(s: &str) -> Option<&str> {
    s.strip_prefix("- ").or_else(|| s.strip_prefix("* "))
}

fn ordered_item(s: &str) -> Option<&str> {
    let digits = s.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digits > 0 {
        s[digits..].strip_prefix(". ")
    } else {
        None
    }
}

fn render_table(rows: &[&str], rewrite: fn(&str) -> String) -> String {
    // Placeholder so escaped \| survives the cell split.
    const PIPE: char = '\u{1}';
    fn cells(row: &str) -> Vec<String> {
        let replaced = row.replace("\\|", &PIPE.to_string());
        replaced
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().replace(PIPE, "|"))
            .collect()
    }

    let mut out = String::from("<table>\n<tr>");
    for cell in cells(rows[0]) {
        out.push_str(&format!("<th>{}</th>", render_inline(&cell, rewrite)));
    }
    out.push_str("</tr>\n");
    for row in rows.iter().skip(2) {
        // rows[1] is the |---|---| separator
        out.push_str("<tr>");
        for cell in cells(row) {
            out.push_str(&format!("<td>{}</td>", render_inline(&cell, rewrite)));
        }
        out.push_str("</tr>\n");
    }
    out.push_str("</table>\n");
    out
}

fn render_inline(text: &str, rewrite: fn(&str) -> String) -> String {
    render_bold(&render_code_spans(&render_links(&escape_html(text), rewrite)))
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Replace [label](target) with an anchor tag. Operates on escaped text;
/// '[' / ']' / '(' / ')' are ASCII, so byte offsets are char boundaries.
fn render_links(text: &str, rewrite: fn(&str) -> String) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let Some(close) = rest[open..].find(']').map(|p| open + p) else {
            break;
        };
        if !rest[close + 1..].starts_with('(') {
            out.push_str(&rest[..close + 1]);
            rest = &rest[close + 1..];
            continue;
        }
        let Some(paren) = rest[close + 2..].find(')').map(|p| close + 2 + p) else {
            break;
        };
        let label = &rest[open + 1..close];
        let target = rewrite(&rest[close + 2..paren]);
        out.push_str(&rest[..open]);
        out.push_str(&format!("<a href=\"{target}\">{label}</a>"));
        rest = &rest[paren + 1..];
    }
    out.push_str(rest);
    out
}

/// `code` spans: odd-numbered chunks between backticks become <code>.
fn render_code_spans(text: &str) -> String {
    let mut out = String::new();
    for (i, part) in text.split('`').enumerate() {
        if i % 2 == 1 {
            out.push_str("<code>");
            out.push_str(part);
            out.push_str("</code>");
        } else {
            out.push_str(part);
        }
    }
    out
}

/// **bold** spans: odd-numbered chunks between ** markers become <strong>.
fn render_bold(text: &str) -> String {
    let mut out = String::new();
    for (i, part) in text.split("**").enumerate() {
        if i % 2 == 1 {
            out.push_str("<strong>");
            out.push_str(part);
            out.push_str("</strong>");
        } else {
            out.push_str(part);
        }
    }
    out
}

// ----------------------------------------------------------------------------
// Page chrome — style and Ferris watermark match the original page design.
// ----------------------------------------------------------------------------

fn page(title: &str, nav: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n\
         <!-- Generated by tools/generate_index.rs from README.md files. Do not edit by hand. -->\n\
         <html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{}</title>\n<style>\n{}</style>\n</head>\n<body>\n\n{}\n\n\
         <div class=\"container\">\n{}{}\n</div>\n</body>\n</html>\n",
        escape_html(title),
        CSS,
        FERRIS,
        nav,
        body
    )
}

const CSS: &str = r##"
:root { --rust: #b7410e; --rust-light: #e05d1a; --bg: #fafafa; --text: #1a1a1a; --code-bg: #f0f0f0; --border: #ddd; --row-alt: #f9f9f9; }
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; color: var(--text); background: var(--bg); line-height: 1.6; }
.watermark { position: fixed; bottom: -60px; right: -60px; width: 500px; height: 340px; opacity: 0.04; pointer-events: none; z-index: -1; }
.container { max-width: 820px; margin: 0 auto; padding: 2rem 1.5rem; }
h1 { color: var(--rust); font-size: 2rem; margin-bottom: 0.25rem; }
h2 { color: var(--rust); font-size: 1.4rem; margin: 2rem 0 0.75rem; border-bottom: 2px solid var(--rust); padding-bottom: 0.3rem; }
h3 { font-size: 1.1rem; margin: 1.5rem 0 0.5rem; }
p, li { margin-bottom: 0.5rem; }
ul, ol { padding-left: 1.5rem; }
a { color: var(--rust); text-decoration: none; }
a:hover { color: var(--rust-light); text-decoration: underline; }
a:visited { color: #8b3a0e; }
code { font-family: "Fira Code", "Cascadia Code", "SF Mono", "Consolas", monospace; font-size: 0.9em; background: var(--code-bg); padding: 0.15em 0.4em; border-radius: 3px; }
pre { background: var(--code-bg); border-left: 4px solid var(--rust); padding: 0.8rem 1rem; overflow-x: auto; margin: 0.75rem 0; border-radius: 0 4px 4px 0; }
pre code { background: none; padding: 0; font-size: 0.85em; }
table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; }
th, td { border: 1px solid var(--border); padding: 0.5rem 0.75rem; text-align: left; }
th { background: var(--rust); color: white; font-weight: 600; }
tr:nth-child(even) { background: var(--row-alt); }
.subtitle { color: #555; font-size: 1.05rem; margin-bottom: 1.5rem; }
nav { display: flex; justify-content: space-between; align-items: center; padding: 0.5rem 0; margin-bottom: 1rem; border-bottom: 1px solid var(--border); font-size: 0.9rem; }
nav .spacer { flex: 1; }
"##;

const FERRIS: &str = r##"<!-- Ferris the Crab watermark -->
<svg class="watermark" viewBox="0 0 1200 800" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:1.41421;">
  <g>
    <g transform="matrix(1,0,0,1,1009.4,506.362)"><path d="M0,-7.203L-12.072,-32.209C-12.009,-33.156 -11.961,-34.107 -11.961,-35.062C-11.961,-63.408 -41.439,-89.533 -91.03,-110.451L-91.03,-93.058C-95.866,-94.977 -100.901,-96.845 -106.147,-98.651L-106.147,-106.759C-177.021,-132.319 -282.53,-148.537 -400.388,-148.537C-503.361,-148.537 -596.917,-136.157 -666.179,-115.983L-666.179,-87.737L-666.181,-87.737L-666.181,-121.925C-737.141,-99.375 -781.135,-68.048 -781.135,-33.41C-781.135,-27.95 -780.034,-22.572 -777.918,-17.297L-785.146,-4.43C-785.146,-4.43 -790.938,3.082 -780.74,18.932C-771.746,32.909 -726.692,87.617 -702.913,116.267C-692.699,130.954 -685.772,140.001 -685.167,139.126C-684.212,137.74 -691.518,110.165 -711.802,78.703C-721.268,61.808 -732.57,39.42 -739.356,22.884C-720.414,34.874 -609.126,90.913 -382.124,90.685C-150.13,90.453 -47.009,17.834 -35.691,7.948C-39.646,23.837 -53.159,55.981 -63.936,78.586C-81.642,110.917 -88.056,139.064 -87.232,140.456C-86.708,141.334 -80.667,132.015 -71.756,116.913C-51.025,87.37 -11.739,30.974 -3.889,16.608C5.007,0.323 0,-7.203 0,-7.203" style="fill:rgb(165,43,0);fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,1079.49,294.885)"><path d="M0,204.135L-79.343,145.689C-80.088,143.089 -80.833,140.488 -81.603,137.908L-55.541,100.154C-52.881,96.314 -52.345,91.322 -54.072,86.943C-55.803,82.585 -59.587,79.461 -64.062,78.696L-108.128,71.217C-109.837,67.732 -111.626,64.301 -113.422,60.898L-94.907,18.51C-93.004,14.193 -93.402,9.175 -95.929,5.256C-98.446,1.319 -102.715,-0.981 -107.267,-0.802L-151.991,0.823C-154.306,-2.193 -156.658,-5.18 -159.058,-8.114L-148.78,-53.546C-147.738,-58.158 -149.054,-62.989 -152.267,-66.34C-155.462,-69.679 -160.105,-71.062 -164.52,-69.979L-208.082,-59.27C-210.902,-61.763 -213.77,-64.223 -216.67,-66.635L-215.103,-113.276C-214.935,-117.997 -217.136,-122.484 -220.915,-125.105C-224.692,-127.741 -229.485,-128.137 -233.616,-126.179L-274.254,-106.858C-277.527,-108.736 -280.819,-110.595 -284.146,-112.395L-291.327,-158.356C-292.056,-163.012 -295.051,-166.968 -299.246,-168.774C-303.431,-170.591 -308.222,-170.002 -311.894,-167.238L-348.126,-140.053C-351.695,-141.238 -355.279,-142.373 -358.905,-143.46L-374.522,-187.045C-376.11,-191.488 -379.772,-194.751 -384.238,-195.669C-388.688,-196.578 -393.266,-195.037 -396.352,-191.589L-426.851,-157.47C-430.536,-157.893 -434.228,-158.28 -437.927,-158.601L-461.476,-198.277C-463.86,-202.295 -468.073,-204.741 -472.615,-204.741C-477.144,-204.741 -481.365,-202.295 -483.733,-198.277L-507.288,-158.601C-510.989,-158.28 -514.696,-157.893 -518.376,-157.47L-548.875,-191.589C-551.965,-195.037 -556.559,-196.578 -560.997,-195.669C-565.457,-194.739 -569.125,-191.488 -570.704,-187.045L-586.333,-143.46C-589.954,-142.373 -593.538,-141.23 -597.113,-140.053L-633.333,-167.238C-637.016,-170.012 -641.811,-170.599 -646.001,-168.774C-650.182,-166.968 -653.189,-163.012 -653.914,-158.356L-661.1,-112.395C-664.422,-110.595 -667.714,-108.746 -670.995,-106.858L-711.629,-126.179C-715.756,-128.145 -720.574,-127.741 -724.333,-125.105C-728.106,-122.484 -730.313,-117.997 -730.143,-113.276L-728.581,-66.635C-731.475,-64.223 -734.337,-61.763 -737.172,-59.27L-780.726,-69.979C-785.149,-71.053 -789.788,-69.679 -792.991,-66.34C-796.212,-62.989 -797.517,-58.158 -796.482,-53.546L-786.225,-8.114C-788.603,-5.169 -790.958,-2.193 -793.267,0.823L-837.991,-0.802C-842.504,-0.937 -846.812,1.319 -849.334,5.256C-851.861,9.175 -852.244,14.193 -850.363,18.51L-831.835,60.898C-833.634,64.301 -835.421,67.732 -837.144,71.217L-881.207,78.696C-885.686,79.45 -889.459,82.572 -891.201,86.943C-892.929,91.322 -892.368,96.314 -889.727,100.154L-863.661,137.908C-863.862,138.575 -864.048,139.247 -864.248,139.916L-937.944,218.201C-937.944,218.201 -949.24,227.052 -932.797,247.855C-918.297,266.206 -843.846,338.951 -804.526,377.06C-787.92,396.408 -776.542,408.389 -775.354,407.353C-773.478,405.708 -783.326,370.506 -816.036,329.204C-841.252,292.148 -873.977,235.155 -866.303,228.586C-866.303,228.586 -857.574,217.505 -840.061,209.529C-839.42,210.041 -840.723,209.022 -840.061,209.529C-840.061,209.529 -470.466,380.02 -127.632,212.413C-88.468,205.388 -64.759,226.368 -64.759,226.368C-56.583,231.108 -77.755,289.712 -95.166,328.505C-118.845,372.555 -122.317,406.927 -120.31,408.119C-119.042,408.876 -110.427,395.766 -98.138,374.902C-67.814,332.649 -10.492,252.1 0,232.534C11.895,210.352 0,204.135 0,204.135" style="fill:rgb(247,76,0);fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,917.896,244.679)"><path d="M0,232.466C0,232.466 53.179,230 123.032,159.004L132.93,137.025C132.93,137.025 24.513,29.177 193.048,-45.266C193.048,-45.266 178.293,-21.154 182.622,72.006C182.622,72.006 233.437,54.357 248.336,-27.934C248.336,-27.934 322.456,69.79 167.834,161.443C167.834,161.443 95.294,277.732 -6.971,266.593L0,232.466Z" style="fill:rgb(247,76,0);fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,676.997,488.361)"><path d="M0,-78.192C0,-78.192 36.935,-118.635 73.871,-78.192C73.871,-78.192 102.893,-24.265 73.871,2.695C73.871,2.695 26.384,40.443 0,2.695C0,2.695 -31.658,-26.964 0,-78.192" style="fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,719.761,425.169)"><path d="M0,0.004C0,15.75 -9.282,28.518 -20.732,28.518C-32.18,28.518 -41.462,15.75 -41.462,0.004C-41.462,-15.746 -32.18,-28.514 -20.732,-28.514C-9.282,-28.514 0,-15.746 0,0.004" style="fill:white;fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,512.148,482.736)"><path d="M0,-83.609C0,-83.609 63.355,-111.661 80.648,-49.047C80.648,-49.047 98.762,23.933 28.618,28.052C28.618,28.052 -60.826,10.824 0,-83.609" style="fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,543.968,426.204)"><path d="M0,0.002C0,16.241 -9.572,29.411 -21.381,29.411C-33.185,29.411 -42.76,16.241 -42.76,0.002C-42.76,-16.242 -33.185,-29.409 -21.381,-29.409C-9.572,-29.409 0,-16.242 0,0.002" style="fill:white;fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,593.317,576.574)"><path d="M0,-40.271L80.796,-46.755C80.796,-46.755 78.058,-33.749 67.517,-23.986C67.517,-23.986 39.727,6.484 7.844,-26.519C7.844,-26.519 2.627,-32.148 0,-40.271" style="fill-rule:nonzero;"/></g>
    <g transform="matrix(1,0,0,1,269.796,270.778)"><path d="M0,190.741C-0.667,190.741 -1.321,190.79 -1.973,190.842C-28.207,184.871 -101.946,165.657 -121.437,134.479C-121.437,134.479 -22.21,21.607 -177.297,-50.54L-159.24,74.338C-159.24,74.338 -207.049,42.389 -217.366,-27.008C-217.366,-27.008 -333.789,57.486 -165.982,138.466C-165.982,138.466 -150.762,195.653 -4.633,241.281L-4.526,240.846C-3.055,241.118 -1.549,241.281 0,241.281C13.808,241.281 25.003,229.969 25.003,216.01C25.003,202.054 13.808,190.741 0,190.741" style="fill:rgb(247,76,0);fill-rule:nonzero;"/></g>
  </g>
</svg>"##;
