extern crate rand;
use rand::distributions::{Exp, IndependentSample};
use rand::Rng;


macro_rules! gen_exp {
    ($x: expr) => {
        {
            let exp = Exp::new($x);
            exp.ind_sample(&mut rand::thread_rng())
        }
    };
    () => {
        {
            let exp = Exp::new(1.0);
            exp.ind_sample(&mut rand::thread_rng())
        }
    };
}

struct CharacterRange{
    current: usize,
    max    : usize,
}

impl CharacterRange{
    fn new(max:usize) -> CharacterRange{
        CharacterRange{current: 0, max:max}
    }
}

#[derive(Debug)]
enum NewickNode{
    Root{left_child: Box<NewickNode>, right_child: Box<NewickNode>},
    Node{left_child: Box<NewickNode>, right_child: Box<NewickNode>, weight:f64},
    Leaf{weight:f64, label:String},
}

impl NewickNode{
    fn to_newick(&self) -> String{
        match self{
            &NewickNode::Leaf{weight, ref label}=>  {
                label.to_owned() + ":" + &format!("{:.4}", weight)
            },
            &NewickNode::Node{left_child: ref lc, right_child: ref rc, weight: w}=>  {
                "(".to_string() + &lc.to_newick() + "," + &rc.to_newick() + 
                    "):" + &format!("{:.4}", w)
            },
            &NewickNode::Root{left_child: ref lc, right_child: ref rc} =>  {
                "(".to_string() + &lc.to_newick() + "," + &rc.to_newick() + ");"
            },
        }
    }
}

fn new_leaf(l: String) -> NewickNode{
    NewickNode::Leaf{weight: gen_exp!(), label:l}
}

fn new_node(lc: NewickNode, rc: NewickNode) -> NewickNode{
    NewickNode::Node{
        left_child: Box::new(lc), 
        right_child: Box::new(rc), 
        weight:gen_exp!(),
    }
}

fn new_root(lc: NewickNode, rc: NewickNode) -> NewickNode{
    NewickNode::Root{left_child: Box::new(lc), right_child: Box::new(rc)}
}

fn gen_tree(tree_size: u8) -> NewickNode{
    let mut tree = Vec::with_capacity(tree_size as usize);
    let label_start = 'A' as u8;
    let label_end = label_start + tree_size;
    for l in label_start .. label_end{
        tree.push(new_leaf((l as char).to_string()));
    }

    let mut rng = rand::thread_rng();

    while tree.len() != 2{
        let roll = rng.gen_range(0, tree.len());
        let l1 = tree.remove(roll);
        let roll = rng.gen_range(0, tree.len());
        let l2 = tree.remove(roll);
        tree.push(new_node(l1, l2));
    }

    let r = new_root(tree.pop().unwrap(), tree.pop().unwrap());
    r
}

fn main(){
    let t = gen_tree(3u8);
    println!("{}", t.to_newick());
}
