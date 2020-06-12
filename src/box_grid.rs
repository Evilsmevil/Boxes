// storage for the current box
use crate::*;
use crate::edge::Orientation;

#[derive(Debug)]
pub struct BoxGrid {
    width:u8,
    height:u8,
    columns: Vec<Vec<bool>>,
    rows   : Vec<Vec<bool>>
}

impl BoxGrid {
    pub fn new(width:u8, height:u8) -> BoxGrid
    {
        BoxGrid{ 
            width:width, 
            height:height, 
            rows : vec![vec![false; usize::from(width)]; usize::from(height-1) ],
            columns: vec![vec![false; usize::from(height)]; usize::from(width - 1) ],
        }
    }

    pub fn max_edges(&self) -> u8 {
        ((self.width - 1) * self.height) + ((self.height - 1) * self.width) 
    }

    pub fn is_occupied(&self, edge: &Edge) -> bool {
        let orientation = edge.get_orientation();
        match orientation { 
            Orientation::Vertical => self.find_vertical_occupancy(edge),
            Orientation::Horizontal => self.find_horizontal_occupancy(edge),
            Orientation::Diagonal => panic!() // we don't handle diagonal
        }
    }

    fn set_occupied(&mut self, edge : &Edge) {

        match edge.get_orientation() {
            Orientation::Vertical => self.set_occupied_vertical(edge),
            Orientation::Horizontal => self.set_occupied_horizontal(edge),
            Orientation::Diagonal => panic!()
        }
    }

    fn set_occupied_vertical(&mut self, edge: &Edge) 
    {
        let column = self.columns.get_mut(usize::from(edge.start.x)).unwrap();
        
        column[usize::from(edge.start.y)] = true;
    }

    fn set_occupied_horizontal(&mut self, edge: &Edge)
    {
        let row = self.columns.get_mut(usize::from(edge.start.y)).unwrap();

        row[usize::from(edge.start.x)] = true;
    }

    fn find_vertical_occupancy(&self, edge: &Edge) -> bool {
        // we store the vertical occupancy in columns. Get the edge position from the start x co-ord

        // borrow occupancy value
        let column = &self.columns[usize::from(edge.start.x)];

        // now we have the column get the row
        column[usize::from(edge.start.y)]
    }

    fn find_horizontal_occupancy(&self, edge: &Edge) -> bool {
        // we store the horizontal occupancy in rows, get the row we care about from the start y co-ord
        // borrow occupancy value
        assert!(usize::from(edge.start.y) < self.rows.len());

        let row = &self.rows[usize::from(edge.start.y)];

        // now we have the column get the row
        row[usize::from(edge.start.x)]
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_max_edges() {
        // arrange
        let box_grid_small = BoxGrid::new(2,2);
        let box_grid_medium = BoxGrid::new(4,4);
        let box_grid_tall = BoxGrid::new(2,6);
        let box_grid_wide = BoxGrid::new(6,2);
        
        // act
        let max_edges_small = box_grid_small.max_edges();
        let max_edges_medium = box_grid_medium.max_edges();
        let max_edges_tall = box_grid_tall.max_edges();
        let max_edges_wide = box_grid_wide.max_edges();

        // assert
        assert_eq!(max_edges_small, 4);
        assert_eq!(max_edges_medium, 24);
        assert_eq!(max_edges_tall, 16);
        assert_eq!(max_edges_wide, 16);
    }

    #[test]
    fn test_edge_store() {
        // make an edge store

        let width = 3;
        let height = 4;

        let mut box_grid = BoxGrid::new(width, height);

        let test_edge = Edge::new(0,0, 0,1);

        let occupancy_result = box_grid.is_occupied(&test_edge);

        box_grid.set_occupied(&test_edge);

        let occupancy_result_2 = box_grid.is_occupied(&test_edge);

        assert_eq!(occupancy_result, false);
        assert_eq!(occupancy_result_2, true);
        assert_eq!(box_grid.columns.len(), usize::from(width - 1));
        
    }
}


