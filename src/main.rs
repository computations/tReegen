extern crate rand;
use rand::distributions::Exp;
use rand::distributions::Sample;
use rand::SeedableRng;
use rand::Rng;

trait Newick{
    fn to_newick(&self) -> String;
}

#[derive(Debug)]
struct Leaf{
    weight: f64,
    label: String,
}

impl Newick for Leaf{
    fn to_newick(&self) -> String{
        self.label.to_owned() + ":" + &format!("{:.4}", self.weight)
    }
}

impl Leaf{
    fn new<T:Into<String>>(label: T) -> Leaf{
        let mut rng = rand::thread_rng();
        let mut e = Exp::new(1.0);
        let w = e.sample(&mut rng);
        Leaf{label: label.into(), weight: w}
    }
}

#[derive(Debug)]
struct Node<T:Newick, U:Newick>{
    weight: f64,
    left_child : Box<T>,
    right_child : Box<U>,
}

impl <T:Newick, U:Newick> Newick for Node<T, U>{
    fn to_newick(&self) -> String{
        String::from("(") + &self.left_child.to_newick() + "," + 
            &self.right_child.to_newick() + "):" + &format!("{:.4}", self.weight)
    }
}

impl <T:Newick, U:Newick> Node<T,U>{
    fn new(lc:T , rc:U) -> Node<T, U>{
        let mut rng = rand::thread_rng();
        let mut e = Exp::new(1.0);
        let w = e.sample(&mut rng);
        Node{left_child: Box::new(lc), right_child: Box::new(rc), weight:w}
    }
}

fn main(){
    let mut c = 'A' as u8;
    let l1 = Leaf::new((c as char).to_string());
    c+=1;
    let l2 = Leaf::new((c as char).to_string());
    let n = Node::new(l1, l2);
    println!("{:?}", n);
    println!("{}", n.to_newick());
    let mut xorshift = rand::XorShiftRng::from_seed([2, 2, 1, 0]);
    let random_number = xorshift.gen::<u32>();
    println!("{}", random_number);
}
