use crate::node::*;
use crate::expose::*;

fn select_rake<S, T: Select>(mut rake: RakeNode<S, T>) -> CompNode<S, T> {
    unsafe {
        while let RakeNode::Node(r) = rake {
            let dir = T::select(r.as_ref().child(0).fold(), r.as_ref().child(1).fold());
            rake = r.as_ref().child(dir);
        }
        if let RakeNode::Leaf(comp) = rake { comp }
        else { unreachable!() }
    }
}

pub fn select<S, T: Select>(v : Vertex<S, T>) -> (Vertex<S, T>, Vertex<S, T>) {
    let mut node = expose(v);
    unsafe {
        while let CompNode::Node(n) = node {
            let a = n.as_ref().child(0).fold();
            let b = n.as_ref().child(1).fold();
            let r = match n.as_ref().rake() {
                Some(r) => r.fold(),
                None => T::identity(),
            };
            let dir = T::select(T::rake(a.clone(), r.clone()), b);
            node = if dir == 0 {
                let dir = T::select(a, r);
                if dir == 0 { n.as_ref().child(0) }
                else { select_rake(n.as_ref().rake().unwrap()) }
            }
            else {
                n.as_ref().child(1)
            };
        }
    }
    if let CompNode::Leaf(_) = node {
        soft_expose(node.endpoints(0), node.endpoints(1));
        (node.endpoints(0), node.endpoints(1))
    }
    else { unreachable!() }
}