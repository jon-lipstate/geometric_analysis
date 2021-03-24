fn ex() {
    println!("Hello, world!");
    let a = Node::new(1, 0.,0.);
    let b = Node::new(2, 1.,0.);
    println!("{:?}", a);
    println!("{:?}", b);
}
#[derive(Clone, Debug)]
struct Node {
    id: u32,
    x: f64,
    y: f64
}
impl Node {
    pub fn new(id: u32,x:f64,y:f64) -> Node {
        Node{
            id: id,
            x:x,
            y:y
        }
    }
}
#[derive(Clone, Debug)]
struct Member {
    id: u32,
    node_a: u64,
    node_b: u64
}

enum Support {
    //u32 is node ID
    Fixed(u32),
    Pinned(u32),
    HRoller(u32),
    VRoller(u32)
}
enum Load {
    Point{
        fx: f64,
        fy:f64
    },
    Moment{mz:f64}
    
    //Dist: member id, xStartMag,xEndMag, same Y, start%, end%
}

#[derive(Clone, Debug)]
pub struct Truss {
    graph: petgraph::Graph<Node, Member>,
}

impl Default for Truss {
    fn default() -> Truss {
        Truss {
            graph: petgraph::Graph::new(),
        }
    }
}

// impl Truss {
//    pub fn add_node(&mut self, node: Node) -> petgraph::graph::NodeIndex {
    
//    }
// }
//https://skyciv.com/free-truss-calculator/
