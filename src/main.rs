extern crate rand;
use rand::distributions::{Exp, Distribution, Uniform, Beta};
use rand::Rng;

#[macro_use]
extern crate clap;
use clap::App;


macro_rules! gen_exp {
    ($x: expr) => {
        {
            let exp = Exp::new($x);
            exp.sample(&mut rand::thread_rng())
        }
    };
    () => {
        {
            let exp = Exp::new(10.0);
            exp.sample(&mut rand::thread_rng())
        }
    };
}

macro_rules! gen_uniform {
    ($x: expr, $y: expr) => {
        {
            let uni = Uniform::new($x, $y);
            uni.sample(&mut rand::thread_rng())
        }
    };
    () => {
        {
            let uni = Uniform::new(0.0, 1.0);
            uni.sample(&mut rand::thread_rng())
        }
    };
}

macro_rules! gen_beta {
    ($x: expr, $y: expr) => {
        {
            let beta = Beta::new($x, $y);
            beta.sample(&mut rand::thread_rng())
        }
    };
    () => {
        {
            let beta = Beta::new(2.0, 5.0);
            beta.sample(&mut rand::thread_rng())
        }
    };
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
    new_leaf_weightless(l, gen_uniform!())
}

fn new_leaf_weightless(l: String, w: f64) -> NewickNode {
    NewickNode::Leaf{weight: w, label:l}
}

fn new_node(lc: NewickNode, rc: NewickNode) -> NewickNode{
    new_node_weightless(lc, rc, gen_uniform!())
}

fn new_node_weightless(lc: NewickNode, rc: NewickNode, w: f64) -> NewickNode{
    NewickNode::Node{
        left_child: Box::new(lc),
        right_child: Box::new(rc),
        weight:w,
    }
}

fn new_root(lc: NewickNode, rc: NewickNode) -> NewickNode{
    NewickNode::Root{left_child: Box::new(lc), right_child: Box::new(rc)}
}

fn generate_labels(count: u64) -> Vec<String>{
    let mut labels = Vec::new();
    let bases = (count as f64).log(26.0).ceil() as u32;
    for i in 0 .. count{
        let mut label = String::from("");
        for b in 0u32 .. bases{
            let c1 = i % 26u64.pow(bases - b);
            let c2 = (c1 / 26u64.pow(bases - b - 1)) as u8 + 'a' as u8;
            label += &(c2 as char).to_string();
        }
        labels.push(label);
    }
    labels
}

fn gen_tree(tree_size: u64) -> NewickNode{
    let mut tree = Vec::with_capacity(tree_size as usize);
    let labels = generate_labels(tree_size);
    for l in labels{
        tree.push(new_leaf(l));
    }

    let mut rng = rand::thread_rng();

    while tree.len() != 2{
        let roll = rng.gen_range(0, tree.len());
        let l1 = tree.swap_remove(roll);
        let roll = rng.gen_range(0, tree.len());
        let l2 = tree.swap_remove(roll);
        tree.push(new_node(l1, l2));
    }

    let r = new_root(tree.pop().unwrap(), tree.pop().unwrap());
    r
}

fn make_ultrametric(tree: NewickNode, left: f64) -> NewickNode {
    match tree{
        NewickNode::Root{left_child:lc, right_child:rc} => {
            new_root(make_ultrametric(*lc, left), make_ultrametric(*rc, left))
        }
        NewickNode::Node{left_child:lc, right_child:rc, weight:_} =>{
            let new_w = gen_beta!() * left;
            let new_left = left - new_w;
            new_node_weightless(make_ultrametric(*lc, new_left), make_ultrametric(*rc, new_left), new_w)
        }
        NewickNode::Leaf{label:l, weight:_}=>{
            new_leaf_weightless(l, left)
        }
    }
}

fn main(){
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let tree_size = value_t_or_exit!(matches, "size", u64);
    let ultrametric = matches.is_present("u") || matches.is_present("ultrametric");
    let tree_h = value_t!(matches, "height", f64).unwrap_or(1.0);

    let t = {
        let mut t = gen_tree(tree_size);
        if ultrametric {
            t = make_ultrametric(t, tree_h);
        }
        t
    };
    println!("{}", t.to_newick());
}
