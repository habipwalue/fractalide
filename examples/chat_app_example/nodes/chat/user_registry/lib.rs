#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::collections::{HashMap, HashSet};

agent! {
  input(
    user_event: user_event,
    get_users: option
  ),
  output(
    users: string,
    user_count: option,
    error: string
  ),
  state(
    users_by_room: HashMap<String, HashSet<String>>,
    usernames: HashMap<String, String>
  ),
  fn init(&mut self) -> Result<()> {
    self.state.users_by_room = HashMap::new();
    self.state.usernames = HashMap::new();
    Ok(())
  }
  fn run(&mut self) -> Result<Signal> {
    select! {
      event = self.input.user_event => {
        let user_id = event.get_user_id()?;
        let username = event.get_username()?;
        let room_id = event.get_room_id()?;
        let event_type = event.get_event_type()?;
        
        // Store username mapping
        self.state.usernames.insert(user_id.clone(), username);
        
        // Create room if it doesn't exist
        if !self.state.users_by_room.contains_key(&room_id) {
          self.state.users_by_room.insert(room_id.clone(), HashSet::new());
        }
        
        // Get room users
        let room_users = self.state.users_by_room.get_mut(&room_id).unwrap();
        
        match event_type {
          EventType::Join => {
            // Add user to room
            room_users.insert(user_id);
          },
          EventType::Leave => {
            // Remove user from room
            room_users.remove(&user_id);
          },
          _ => {
            // Ignore other event types
          }
        }
        
        // Send updated user count
        let mut count_msg = self.output.user_count.allocate();
        count_msg.set_string(&format!("{}", room_users.len()));
        self.output.user_count.send(count_msg)?;
        
        Ok(End)
      }
      request = self.input.get_users => {
        // Get room ID from request
        let room_id = request.get_string()?;
        
        // Check if room exists
        if !self.state.users_by_room.contains_key(&room_id) {
          // Room doesn't exist, return empty list
          let mut users_msg = self.output.users.allocate();
          users_msg.set_string("[]");
          self.output.users.send(users_msg)?;
        } else {
          // Get users in room
          let room_users = self.state.users_by_room.get(&room_id).unwrap();
          
          // Build JSON array of usernames
          let mut users_json = String::from("[");
          
          for (i, user_id) in room_users.iter().enumerate() {
            if i > 0 {
              users_json.push_str(", ");
            }
            
            let username = self.state.usernames.get(user_id).unwrap_or(user_id);
            users_json.push_str(&format!("{{\"id\": \"{}\", \"name\": \"{}\"}}", user_id, username));
          }
          
          users_json.push_str("]");
          
          // Send users list
          let mut users_msg = self.output.users.allocate();
          users_msg.set_string(&users_json);
          self.output.users.send(users_msg)?;
        }
        
        Ok(End)
      }
    }
  }
}