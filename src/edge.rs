use crate::point::Point;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Edge 
{
    pub start:Point,
    pub end:Point
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Orientation {
    Vertical,
    Horizontal,
    Diagonal
}

pub enum BoxSide
{
    Positive,
    Negative
}

impl Edge {

    pub fn new(x: u8, y: u8, x2: u8, y2: u8) -> Edge {


        let p1 = Point{ x:x, y:y};
        let p2 = Point{ x:x2, y: y2};

        // 
        let pair1_sum = x + y;
        let pair2_sum = x2 + y2;

        if pair1_sum < pair2_sum {
            Edge { start: p1, end: p2}
        }
        else {
            Edge { start: p2, end: p1}
        }
    
    }

    fn equal(&self, other : &Edge) -> bool {
        other.start == self.start && other.end == self.end || 
        other.end == self.start && other.start == self.end
    }
    
    pub fn get_orientation(&self) -> Orientation
    {
        // vertical if x's are the same
       if self.start.x == self.end.x {
            Orientation::Vertical
        }
        else if self.start.y == self.end.y {
            Orientation::Horizontal
        }
        else {
            Orientation::Diagonal
        }
    }

    pub fn get_participating_edges(&self, side : BoxSide) -> Vec<Edge>
    {
        let orientation = self.get_orientation();

        match orientation {
            Orientation::Vertical => self.get_edges_vertical(side),
            Orientation::Horizontal => self.get_edges_horizontal(side),
            Orientation::Diagonal => panic!("Can't calculate edges for diagonal edges!")

        }
    }

    fn get_edges_vertical(&self, side : BoxSide) -> Vec<Edge>
    {
        match side {
            BoxSide::Positive => self.get_positive_edges_vertical(),
            BoxSide::Negative => self.get_negative_edges_vertical()
        }
    }

    fn get_edges_horizontal(&self, side: BoxSide) -> Vec<Edge>
    {
        match side {
            BoxSide::Positive => self.get_positive_edges_horizontal(),
            BoxSide::Negative => self.get_negative_edges_horizontal()
        }
    }

    fn get_positive_edges_horizontal(&self) -> Vec<Edge>
    {
        let y = self.start.y;
        let x_right = std::cmp::max(self.start.x, self.end.x);
        let x_left = std::cmp::min(self.start.x, self.end.x);

        // left vertical edge
        let edge1 = Edge{
                          start:Point{x:x_left, y:y+1}, 
                          end: Point{x:x_left, y:y}
                        };

        // right vertical edge |
        let edge2 = Edge{
                          start:Point{x:x_right, y:y + 1}, 
                          end: Point{x:x_right, y:y}
                        };

        // top horizontal edge
        let edge3 = Edge{
                          start:Point{x:x_left, y:y + 1}, 
                          end: Point{x:x_right, y:y + 1}
                        };
        
        let mut edges : Vec<Edge> = Vec::with_capacity(3);
        edges.push(edge1);
        edges.push(edge2);
        edges.push(edge3);
        
        edges
    }

    fn get_negative_edges_horizontal(&self) -> Vec<Edge>
    {
        let y = self.start.y;
        let x_right = std::cmp::max(self.start.x, self.end.x);
        let x_left = std::cmp::min(self.start.x, self.end.x);

        // left vertical edge
        let edge1 = Edge{
                          start:Point{x:x_left, y:y-1}, 
                          end: Point{x:x_left, y:y}
                        };

        // right vertical edge |
        let edge2 = Edge{
                          start:Point{x:x_right, y:y - 1}, 
                          end: Point{x:x_right, y:y}
                        };

        // top horizontal edge
        let edge3 = Edge{
                          start:Point{x:x_left, y:y - 1}, 
                          end: Point{x:x_right, y:y - 1}
                        };
        
        let mut edges : Vec<Edge> = Vec::with_capacity(3);
        edges.push(edge1);
        edges.push(edge2);
        edges.push(edge3);
        
        edges
    }

    fn get_negative_edges_vertical(&self) -> Vec<Edge>
    {
        let x = self.start.x;
        let y_top = std::cmp::max(self.start.y, self.end.y);
        let y_bot = std::cmp::min(self.start.y, self.end.y);

        // top edge -
        let edge1 = Edge{
                          start:Point{x:x-1, y:y_top}, 
                          end: Point{x:x, y:y_top}
                        };

        // left edge |
        let edge2 = Edge{
                          start:Point{x:x-1, y:y_top}, 
                          end: Point{x:x-1, y:y_bot}
                        };

        // right edge
        let edge3 = Edge{
                          start:Point{x:x-1, y:y_bot}, 
                          end: Point{x:x, y:y_bot}
                        };
        
        let mut edges : Vec<Edge> = Vec::with_capacity(3);
        edges.push(edge1);
        edges.push(edge2);
        edges.push(edge3);
        
        edges
    }

    fn get_positive_edges_vertical(&self) -> Vec<Edge>
    {
        let x = self.start.x;
        let y_top = std::cmp::max(self.start.y, self.end.y);
        let y_bot = std::cmp::min(self.start.y, self.end.y);

        // top edge -
        let edge1 = Edge{
                          start:Point{x:x + 1, y:y_top}, 
                          end: Point{x:x, y:y_top}
                        };

        // left edge |
        let edge2 = Edge{
                          start:Point{x:x+1, y:y_top}, 
                          end: Point{x:x+1, y:y_bot}
                        };

        // right edge
        let edge3 = Edge{
                          start:Point{x:x+1, y:y_bot}, 
                          end: Point{x:x+1, y:y_bot}
                        };
        
        let mut edges : Vec<Edge> = Vec::with_capacity(3);
        edges.push(edge1);
        edges.push(edge2);
        edges.push(edge3);
        
        edges
    }
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vertical() {
        let p1 = Point{x:0, y:0};
        let p2 = Point{x:0, y:1};

        let edge = Edge{start:p1, end:p2};

        let orientation = edge.get_orientation();

        assert_eq!(orientation, Orientation::Vertical);
    }

    #[test]
    fn test_horizontal()
    {
        // arrange
        let p1 = Point{x:0, y:0};
        let p2 = Point{x:1, y:0};

        let edge = Edge{start:p1, end:p2};

        // act
        let orientation = edge.get_orientation();

        // asset
        assert_eq!(orientation, Orientation::Horizontal);
    }

    #[test]
    fn test_diagonal()
    {
        // arrange
        let p1 = Point{x:0, y:0};
        let p2 = Point{x:1, y:1};

        let edge = Edge{start:p1, end:p2};

        // act
        let orientation = edge.get_orientation();

        // asset
        assert_eq!(orientation, Orientation::Diagonal);
    }

    #[test]
    fn test_horizontal_left()
    {
        // arrange
        let p1 = Point{x:1, y:1};
        let p2 = Point{x:1, y:2};

        let edge = Edge{start:p1, end:p2};   

        // get the edges
        let participating_edges = edge.get_participating_edges(BoxSide::Negative);

        assert_eq!(participating_edges.len(), 3);

        // TODO make sure the vec contains the 3 edges we want
        let expected_edge_1 = Edge{start: Point{x: 0, y:1}, end:Point{x:1, y:1}};
        let expected_edge_2 = Edge{start: Point{x: 0, y:1}, end:Point{x:0, y:2}};
        let expected_edge_3 = Edge{start: Point{x: 0, y:2}, end:Point{x:1, y:2}};
        

        let is_contained = does_contain(&participating_edges, &expected_edge_1);
        assert_eq!(is_contained, true);

        let is_contained = does_contain(&participating_edges, &expected_edge_2);
        assert_eq!(is_contained, true);

        let is_contained = does_contain(&participating_edges, &expected_edge_3);
        assert_eq!(is_contained, true);
    }

    #[test]
    // Tests finding top participating edges for a horizontal edge
    fn test_vertical_top()
    {
        // arrange
        let p1 = Point{x:1, y:1};
        let p2 = Point{x:2, y:1};

        let edge = Edge{start:p1, end:p2};  

        let participating_edges = edge.get_participating_edges(BoxSide::Positive);
             
         // TODO make sure the vec contains the 3 edges we want
         let expected_edge_1 = Edge{start: Point{x: 1, y:2}, end:Point{x:2, y:2}};
         let expected_edge_2 = Edge{start: Point{x: 1, y:1}, end:Point{x:1, y:2}};
         let expected_edge_3 = Edge{start: Point{x: 2, y:1}, end:Point{x:2, y:2}};
         
 
         let is_contained = does_contain(&participating_edges, &expected_edge_1);
         assert_eq!(is_contained, true);
 
         let is_contained = does_contain(&participating_edges, &expected_edge_2);
         assert_eq!(is_contained, true);
 
         let is_contained = does_contain(&participating_edges, &expected_edge_3);
         assert_eq!(is_contained, true);
    }

    #[test]
    // tests finding bottom participating edges for a horizontal line
    fn test_vertical_bot()
    {
        // arrange
        let p1 = Point{x:1, y:1};
        let p2 = Point{x:2, y:1};

        let edge = Edge{start:p1, end:p2};  

        let participating_edges = edge.get_participating_edges(BoxSide::Negative);
             
         // TODO make sure the vec contains the 3 edges we want
         let expected_edge_1 = Edge{start: Point{x: 1, y:0}, end:Point{x:2, y:0}};
         let expected_edge_2 = Edge{start: Point{x: 1, y:1}, end:Point{x:1, y:0}};
         let expected_edge_3 = Edge{start: Point{x: 2, y:1}, end:Point{x:2, y:0}};
         
 
         let is_contained = does_contain(&participating_edges, &expected_edge_1);
         assert_eq!(is_contained, true);
 
         let is_contained = does_contain(&participating_edges, &expected_edge_2);
         assert_eq!(is_contained, true);
 
         let is_contained = does_contain(&participating_edges, &expected_edge_3);
         assert_eq!(is_contained, true);
    }

    fn does_contain(edges: &Vec<Edge>, target : &Edge) -> bool
    {
        for edge in edges {
            if edge.equal(target) {
                return true;
            }
        }

        false
    }
}