fn main() {
    println!("Hello, world!{:?}", is_valid("(([]){})".to_string()));
}

pub fn is_valid(s: String) -> bool {
    // length shoud be *2
    if s.len() == 0 {
        return true;
    }
    if s.len()%2 != 0 {
        return false; 
    }

    let a : Vec<char>= s .chars().collect();

    let mut continiues1 = a.len()/2;
    println!("continiues{:?}", continiues1);

    for i in 0 .. a.len(){
        if a[i] == '{' && a[i+1]=='}' {
            println!("1: {:?}", continiues1);
            println!("a[i]{:?}", a[i]);
            println!("a[i+1] {:?}", a[i+1]);
            println!("i{:?}", i);

            continiues1 -=1 ;
        }else if a[i] == '(' && a[i+1]==')' {
            println!("2{:?}", continiues1);
            println!("i{:?}", i);

            continiues1 -=1 ;
        }else if a[i] == '[' && a[i+1]==']' {
            println!("3: {:?}", continiues1);
            println!("i{:?}", i);

            continiues1 -=1 ;
        }
    }   
    println!("continiues{:?}", continiues1);

    let mut continiues2 = s.len()/2;

    for i in 0 .. a.len()/2{
        println!("continiues{:?}", continiues2);
        println!("a[a.len()/2-1-i]{:?}", a.len()/2-1-i);
        if a[i] == '{' && a[a.len()-1-i]=='}' {
            continiues2 -=1 ;
        }else if  a[i] == '(' && a[a.len()-1-i]==')' {
            continiues2 -=1 ;
        }else if  a[i] == '[' && a[a.len()-1-i]==']' {
            continiues2 -=1 ;
        }
    }  

    println!("{:?}", continiues2);

    continiues1 == 0 || continiues2 == 0
}

// each element should be closed eigher right after or in len-i


pub fn i_valid(s: String) -> bool {
    // length shoud be *2
    if s.len() == 0 {
        return true;
    }
    if s.len()%2 != 0 {
        return false; 
    }

    let chars : Vec<char>= s .chars().collect();

    let mut a :Vec<usize>=vec![];
    let mut b :Vec<usize>=vec![];

    let mut a1 :Vec<usize>=vec![];
    let mut b1 :Vec<usize>=vec![];

    let mut a2 :Vec<usize>=vec![];
    let mut b2 :Vec<usize>=vec![];


    for i in 0 .. s.len() {
        if chars[i] == '{' {
            a.push(i);
        }else if chars[i] == '}'  {
            b.push(i);
        }else if chars[i] == '('  {
            a1.push(i);
        }else if chars[i] == ')'  {
            b1.push(i);
        }else if chars[i] == '['  {
            a2.push(i);
        }else if chars[i] == ']'  {
            b2.push(i);
        }
    }

    a.sort();
    b.sort();
    a1.sort();
    b1.sort();
    a2.sort();
    b2.sort();

    if a.len() != b.len(){
        return false;
    }else if a1.len() != b1.len() {
        return false;
    }else if a2.len() != b2.len() {
        return false;
    }  

    for i in 0 .. a.len(){
        if a[i] > b[i]{
            return false;
        }else if a1[i] >  b1[i]{
            return false;
        }else if a2[i] >  b2[i]{
            return false;
        }
    }


    true
}