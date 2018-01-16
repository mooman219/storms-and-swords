use game::*;
use cgmath::Vector2;
use game::event_system::Event;

/*
    A move event does not move characters itself, instead it will find the path between the current point
    and the point where the character wants to go, and then create walk events between each tile 
    in the path
*/
pub struct MoveEvent {
    character_to_move: UID,//this must be the UID of a character_model
    //the move event will first look for a character with that uid in the team controllers
    //if it is unable to find it, the event will end
    target_position: Vector2<i32>//must be a valid 
}

impl MoveEvent {
    
    pub fn new(character_to_move: UID, target_position: Vector2<i32> ) -> MoveEvent {
        MoveEvent {
            character_to_move,
            target_position
        }
    }

    fn a_star() {

    }
}

impl Event {
    fn execute(&self, world: &mut World) {
        
    }

}