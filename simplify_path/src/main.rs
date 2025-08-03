fn main() {
    let s = "/.../a/../b/c/../d/./".to_string();
    println!("{:?}", simplify_path(s));
    // let s2 = "/../../".to_string();
    // let s0: Vec<&str> = s2.split('/').filter(|&x| !x.is_empty()).collect();
    // println!("{:?}", s0);
}

pub fn simplify_path(path: String) -> String {
    // we should split them with 
    // if its .. go back
    // if its . remove
    if path.len() == 0 {
        return "".to_string();
    }
    let path_vec :Vec<&str> = path.split('/').filter(|&x| !x.is_empty()).collect();
    let mut path_v :Vec<String> = vec![];
    let mut result :String = "".to_string();
    for i in path_vec {
        match i {
            "//" => path_v.push("/".to_string()),
            ".." => {
                path_v.pop();
            },
            "." => {},
            _ => path_v.push(i.to_string()),
        }
    }

    for i in &path_v {
        result.push('/');
        result.push_str(&i);
    }
    if path_v.len() == 0 {
        return "/".to_string();
    }

    result
}
