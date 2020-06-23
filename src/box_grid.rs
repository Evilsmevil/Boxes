// storage for the current box
use crate::*;
use crate::edge::Orientation;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum EdgeState 
{
   Empty,
   Occupied{player_index: u32} 
}

#[derive(Debug)]
pub struct BoxGrid {
    width:u8,
    height:u8,
    columns: Vec<Vec<EdgeState>>,
    rows   : Vec<Vec<EdgeState>>
}

impl BoxGrid {
    pub fn new(width:u8, height:u8) -> BoxGrid
    {
        BoxGrid{ 
            width:width, 
            height:height, 
            rows : vec![vec![EdgeState::Empty; usize::from(width)]; usize::from(height-1) ],
            columns: vec![vec![EdgeState::Empty; usize::from(height)]; usize::from(width - 1) ],
        }
    }

    pub fn max_edges(&self) -> u8 {
        ((self.width - 1) * self.height) + ((self.height - 1) * self.width) 
    }

    pub fn is_occupied(&self, edge: &Edge) -> &EdgeState {
        let orientation = edge.get_orientation();
        match orientation { 
            Orientation::Vertical => self.find_vertical_occupancy(edge),
            Orientation::Horizontal => self.find_horizontal_occupancy(edge),
            Orientation::Diagonal => panic!() // we don't handle diagonal
        }
    }

    fn set_occupied(&mut self, edge : &Edge, player_index:u32) {

        match edge.get_orientation() {
            Orientation::Vertical => self.set_occupied_vertical(edge, player_index),
            Orientation::Horizontal => self.set_occupied_horizontal(edge, player_index),
            Orientation::Diagonal => panic!()
        }
    }

    fn set_occupied_vertical(&mut self, edge: &Edge, player_index : u32) 
    {
        let column = self.columns.get_mut(usize::from(edge.start.x)).unwrap();
        
        column[usize::from(edge.start.y)] = EdgeState::Occupied{player_index: player_index};
    }

    fn set_occupied_horizontal(&mut self, edge: &Edge, player_index : u32) 
    {
        let row = self.columns.get_mut(usize::from(edge.start.y)).unwrap();

        row[usize::from(edge.start.x)] = EdgeState::Occupied{player_index: player_index};
    }

    fn find_vertical_occupancy(&self, edge: &Edge) -> &EdgeState {
        // we store the vertical occupancy in columns. Get the edge position from the start x co-ord

        // borrow occupancy value
        let column = &self.columns[usize::from(edge.start.x)];

        // now we have the column get the row
        &column[usize::from(edge.start.y)]
    }

    fn find_horizontal_occupancy(&self, edge: &Edge) -> &EdgeState {
        // we store the horizontal occupancy in rows, get the row we care about from the start y co-ord
        // borrow occupancy value
        assert!(usize::from(edge.start.y) < self.rows.len());

        let row = &self.rows[usize::from(edge.start.y)];

        // now we have the column get the row
        &row[usize::from(edge.start.x)]
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
        assert_eq!(occupancy_result, &EdgeState::Empty);

        box_grid.set_occupied(&test_edge, 0);

        let occupancy_result_2 = box_grid.is_occupied(&test_edge);

        assert_eq!(occupancy_result_2, &EdgeState::Occupied{player_index:0});
        assert_eq!(box_grid.columns.len(), usize::from(width - 1));
        
    }
}


