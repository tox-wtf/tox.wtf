use std::ffi::OsStr;
use std::fs::{self, read_dir};
use fshelpers::mkf_p;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

use minijinja::{context, Environment};
use color_eyre::Result;

fn render_error_pages(env: &Environment) -> Result<()> {
    let mut env = env.clone(); // TODO: Consider whether I mind mangling the base env

    for f in read_dir("target/tmp/pages/e")?.flatten() {
        let path = f.path();
        if path.extension().is_some_and(|e| e.to_str().is_some_and(|s| s == "html")) {
            add_template_from_path(&mut env, &path)?;

            // FIXME: Log failures here
            let Some(filename) = path.file_name().and_then(|f| f.to_str()) else { continue };

            let template = env.get_template(filename)?;
            let rendered = template.render(context! {})?;

            let path = Path::new("target/site/e").join(filename);

            mkf_p(&path)?;
            fs::write(path, rendered)?;
        }
    }

    Ok(())
}

fn should_template<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    eprint!("Checking if we should template {path:?}... ");

    let should_render =
        !path.starts_with("subdomains/lfs/lfs") &&
        !path.starts_with("subdomains/lfs/blfs") &&
        !path.starts_with("subdomains/lfs/glfs") &&
        !path.starts_with("subdomains/lfs/slfs") &&
        path.is_file() &&
        path.extension().is_some_and(|e| e == OsStr::new("html")) &&
        path.file_name().is_some_and(|e| !matches!(e.as_encoded_bytes(), b"base.html" | b"macros.html")) &&
        path.iter().all(|p| p != OsStr::new("e"));

    match should_render {
        true => eprintln!("yes"),
        false => eprintln!("no"),
    }

    should_render
}

fn collect_pages() -> Vec<PathBuf> {
    WalkDir::new("target/tmp/pages")
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.map(|e| e.path().to_path_buf()).ok())
        .filter(|e| should_template(e))
        .collect()
}

fn collect_subdomain_pages(subdomain: &str) -> Vec<PathBuf> {
    WalkDir::new(format!("subdomains/{subdomain}"))
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.map(|e| e.path().to_path_buf()).ok())
        .filter(|e| should_template(e))
        .collect()
}

fn render_pages(env: &Environment) -> Result<()> {
    let mut env = env.clone();

    for f in collect_pages() {
        add_template_from_path(&mut env, &f)?;
        let Some(filename) = f.file_name().and_then(|f| f.to_str()) else { continue };

        let template = env.get_template(filename)?;
        let rendered = template.render(context! {})?;

        let path = Path::new("target/site/").join(f.to_string_lossy().trim_start_matches("target/tmp/pages/"));

        mkf_p(&path)?;
        fs::write(path, rendered)?;
    }

    Ok(())
}

fn render_subdomain_pages(env: &Environment, subdomain: &str) -> Result<()> {
    let mut env = env.clone();

    for f in collect_subdomain_pages(subdomain) {
        add_template_from_path(&mut env, &f)?;
        let Some(filename) = f.file_name().and_then(|f| f.to_str()) else { continue };

        let template = env.get_template(filename)?;
        let rendered = template.render(context! {})?;

        let path = PathBuf::from(format!("target/subdomains/{subdomain}"))
            .join(f.to_string_lossy()
                .trim_start_matches(format!("subdomains/{subdomain}/")
                .trim_start_matches("target/tmp/pages/")
                )
            );

        mkf_p(&path)?;
        fs::write(path, rendered)?;
    }

    Ok(())
}

// TODO: Don't abuse leak
fn add_template_from_path<P>(env: &mut Environment, path: P) -> Result<()>
where P: AsRef<Path>,
{
    let path = path.as_ref();
    let str = fs::read_to_string(path)?.leak();
    let path_str = path.file_name().unwrap().to_str().unwrap().to_string().leak();

    env.add_template(
        path_str,
        str,
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let mut env = Environment::new();

    add_template_from_path(&mut env, "target/tmp/pages/macros.html")?;
    add_template_from_path(&mut env, "target/tmp/pages/base.html")?;
    add_template_from_path(&mut env, "target/tmp/pages/index.html")?;

    render_error_pages(&env)?;
    render_pages(&env)?;

    env.clear_templates();
    add_template_from_path(&mut env, "subdomains/man/macros.html")?;
    add_template_from_path(&mut env, "subdomains/man/base.html")?;
    add_template_from_path(&mut env, "subdomains/man/index.html")?;
    render_subdomain_pages(&env, "man")?;

    env.clear_templates();
    add_template_from_path(&mut env, "subdomains/vat/macros.html")?;
    add_template_from_path(&mut env, "subdomains/vat/base.html")?;
    add_template_from_path(&mut env, "subdomains/vat/index.html")?;
    render_subdomain_pages(&env, "vat")?;

    env.clear_templates();
    add_template_from_path(&mut env, "subdomains/lfs/macros.html")?;
    add_template_from_path(&mut env, "subdomains/lfs/base.html")?;
    add_template_from_path(&mut env, "subdomains/lfs/index.html")?;
    render_subdomain_pages(&env, "lfs")?;

    Ok(())
}
