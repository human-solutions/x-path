use serde::{Deserialize, Serialize};
use x_path::AbsDir;

#[derive(Serialize, Deserialize, Debug)]
struct PathTest {
    #[serde(with = "x_path::abs_dir::exist")]
    path1: AbsDir,
    path2: AbsDir,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExpandPathTest {
    #[serde(with = "x_path::abs_dir::expanded")]
    path1: AbsDir,
}

#[test]
fn itest_abs_dir() {
    let p = AbsDir::try_from("/dir1/dir2").unwrap();

    let segs = p.segments().collect::<Vec<_>>();

    // let m = x_path::any_path::validated;
    assert_eq!(segs, vec!["dir1", "dir2"]);
    assert_eq!(format!("{p:?}"), "AbsDir(\"/dir1/dir2\")");

    assert_eq!(serde_json::to_string(&p).unwrap(), r#""/dir1/dir2""#);

    let exp_p = ExpandPathTest {
        path1: ".".try_into().unwrap(),
    };
    assert!(serde_json::to_string(&exp_p).unwrap().len() > 10);

    let pt1 = PathTest {
        path1: AbsDir::try_from("./Cargo.toml").unwrap(),
        path2: AbsDir::try_from("./dir1").unwrap(),
    };
    insta::assert_snapshot!(serde_json::to_string_pretty(&pt1).unwrap(), @r###"
    {
      "path1": "./Cargo.toml",
      "path2": "./dir1"
    }
    "###);

    let val = err_json(r###" { "path1": "./doesntexist", "path2": "./dir1"  } "###);
    assert_eq!(val, "dir doesn't exist: ./doesntexist at line 1 column 27");

    let val = err_json(r###" { "path1": "./Cargo.toml", "path2": "./dir1"  } "###);
    assert_eq!(val, "not a directory: ./Cargo.toml at line 1 column 26");
}

fn err_json(s: &str) -> String {
    serde_json::from_str::<PathTest>(s)
        .map_err(|e| e.to_string())
        .unwrap_err()
}