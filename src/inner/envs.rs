use std::borrow::Cow;

use anyhow::{ensure, Context, Result};

use crate::{
    env::{current_dir, env_var, home_dir},
    ext::CharExt,
    SEP, SLASH,
};

enum Start {
    Home,
    Current,
    None,
}

impl Start {
    fn from(path: &str) -> Self {
        if path == "~" || path.starts_with("~/") || path.starts_with("~\\") {
            Self::Home
        } else if path == "." || path.starts_with("./") || path.starts_with(".\\") {
            Self::Current
        } else {
            Self::None
        }
    }
}

pub(crate) fn contract_envs<'a>(path: &'a str) -> Result<(Option<char>, &'a str)> {
    let home_rel = remove_abs_start(path, &home_dir()?);
    let cwd_rel = remove_abs_start(path, &current_dir()?);
    Ok(match (home_rel, cwd_rel) {
        (Some(home), Some(cwd)) if home.len() < cwd.len() => (Some('~'), home),
        (Some(_), Some(cwd)) => (Some('.'), cwd),
        (Some(home), None) => (Some('~'), home),
        (None, Some(cwd)) => (Some('.'), cwd),
        (None, None) => (None, path),
    })
}

fn remove_abs_start<'a>(path: &'a str, start: &str) -> Option<&'a str> {
    if path.starts_with(start) {
        let mut pos = start.len();
        if path[pos..].starts_with(SEP) {
            pos += 1;
        }
        Some(&path[pos..])
    } else {
        None
    }
}

pub(crate) fn expand_envs<'a>(path: &'a str) -> Result<Cow<str>> {
    let start = Start::from(&path);

    let path: Cow<str> = match start {
        Start::Current => prefix_current_dir(&path[1..])?,
        Start::Home => prefix_home_dir(&path[1..])?,
        Start::None if !path.contains(['$', '%']) => return Ok(Cow::Borrowed(path)),
        Start::None => Cow::Borrowed(path),
    };

    let mut chars = path.chars().peekable();

    // set to true because no character also counts.
    let mut prev_slash = true;

    let mut expanded = String::new();

    while let Some(ch) = chars.next() {
        let start_curly = ch == '$' && prev_slash && chars.next_if_eq(&'{').is_some();
        let start_prcnt = ch == '%' && prev_slash;

        if start_curly || start_prcnt {
            let mut key = start_curly.then_some("${").unwrap_or("%").to_string();

            while let Some(ch) = chars.next() {
                key.push(ch);

                let end_curly = start_curly && ch == '}';
                let end_prcnt = start_prcnt && ch == '%';

                if end_curly || end_prcnt {
                    // a valid env var end is either with a slash or nothing.
                    let valid_end = chars.peek().map(|c| c.is_slash()).unwrap_or(true);
                    if valid_end {
                        let start = if start_curly { 2 } else { 1 };
                        let end = key.len() - 1;

                        ensure!(
                            end - start > 0,
                            "empty environment variable in path: {path}"
                        );

                        expanded.extend(env_var(&key[start..end])?.drain(..));
                        key.clear();
                    }
                    break;
                }

                if ch.is_slash() || !ch.is_allowed_in_environment_var() {
                    break;
                }
            }
            expanded.extend(key.drain(..));
        } else {
            expanded.push(ch);
        }

        prev_slash = ch.is_slash();
    }
    Ok(Cow::Owned(expanded))
}

fn prefix_current_dir<'a>(path: &'a str) -> Result<Cow<'a, str>> {
    let mut cwd = current_dir().context("could not resolve the current working directory")?;
    if !cwd.ends_with(SLASH) && !path.starts_with(SLASH) {
        cwd.push(SEP);
    }
    cwd.extend(path.chars());
    Ok(Cow::Owned(cwd))
}

fn prefix_home_dir<'a>(path: &'a str) -> Result<Cow<'a, str>> {
    let mut home = home_dir().context("could not resolve the current working directory")?;
    if !home.ends_with(SLASH) && !path.starts_with(SLASH) {
        home.push(SEP);
    }
    home.extend(path.chars());
    Ok(Cow::Owned(home))
}
