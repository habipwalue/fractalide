#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::collections::HashMap;

agent! {
  input(
    new_message: chat_message,
    get_messages: option
  ),
  output(
    messages: chat_message,
    error: string
  ),
  state(
    messages: HashMap<String, Vec<ChatMessage>>,
    max_messages_per_room: usize
  ),
  fn init(&mut self) -> Result<()> {
    self.state.messages = HashMap::new();
    self.state.max_messages_per_room = 100; // Default limit
    Ok(())
  }
  fn run(&mut self) -> Result<Signal> {
    select! {
      message = self.input.new_message => {
        // Store new message
        let room_id = message.get_room_id()?;
        
        // Create room if it doesn't exist
        if !self.state.messages.contains_key(&room_id) {
          self.state.messages.insert(room_id.clone(), Vec::new());
        }
        
        // Add message to room
        let room_messages = self.state.messages.get_mut(&room_id).unwrap();
        room_messages.push(message.clone());
        
        // Trim messages if we exceed the limit
        if room_messages.len() > self.state.max_messages_per_room {
          let excess = room_messages.len() - self.state.max_messages_per_room;
          room_messages.drain(0..excess);
        }
        
        Ok(End)
      }
      request = self.input.get_messages => {
        // Get room ID from request
        let room_id = request.get_string()?;
        
        // Check if room exists
        if !self.state.messages.contains_key(&room_id) {
          // Room doesn't exist, return empty list
          Ok(End)
        } else {
          // Send all messages for the room
          let room_messages = self.state.messages.get(&room_id).unwrap();
          
          for message in room_messages {
            self.output.messages.send(message.clone())?;
          }
          
          Ok(End)
        }
      }
    }
  }
}