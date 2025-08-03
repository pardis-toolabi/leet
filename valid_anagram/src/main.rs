fn main() {


}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_chars : Vec<char> = s.chars().collect();
    s_chars.sort_unstable();
    let mut t_chars: Vec<char> = t.chars().collect();
    t_chars.sort_unstable();

    println!("{:?}", t_chars);
    println!("{:?}", s_chars);
    if s_chars != t_chars {
        return false;
    }

    true   
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_result(){
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        println!("result {:?}", is_anagram(s, t));

    }

}
