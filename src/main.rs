mod point;
mod edge;
mod box_grid;
use crate::edge::Edge;
use crate::edge::BoxSide;

fn main()
{
    let edge = Edge::new(0,0,0,1);

    let participating_edges_positive = 
        edge.get_participating_edges(BoxSide::Positive);

    let participating_edges_negative = 
        edge.get_participating_edges(BoxSide::Negative);
    
    for edge in participating_edges_negative {
        println!("{:?}", edge);       
    }

    for edge in participating_edges_positive {
        println!("{:?}", edge);
    }

    // game is super simple - just play an edge before you play another
}


