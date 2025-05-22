#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

agent! {
  fn run(&mut self) -> Result<Signal> {
    // Create a new graph
    let mut g = Graph::new();
    
    // Add nodes
    let room_manager = g.add_node("room_manager", "chat/room_manager");
    let message_generator = g.add_node("message_generator", "msg/generator");
    let user_event_generator = g.add_node("user_event_generator", "msg/generator");
    let message_printer = g.add_node("message_printer", "msg/printer");
    let users_printer = g.add_node("users_printer", "msg/printer");
    let user_count_printer = g.add_node("user_count_printer", "msg/printer");
    
    // Connect nodes
    g.connect(message_generator, "output", room_manager, "new_message");
    g.connect(user_event_generator, "output", room_manager, "user_event");
    g.connect(room_manager, "messages", message_printer, "input");
    g.connect(room_manager, "users", users_printer, "input");
    g.connect(room_manager, "user_count", user_count_printer, "input");
    
    // Execute the graph
    g.execute();
    
    // Simulate user events and messages
    simulate_chat(&g, message_generator, user_event_generator);
    
    Ok(End)
  }
}

fn simulate_chat(g: &Graph, message_generator: NodeId, user_event_generator: NodeId) {
  // Simulate users joining
  let users = vec![
    ("user1", "Alice"),
    ("user2", "Bob"),
    ("user3", "Charlie"),
    ("user4", "Dave")
  ];
  
  let room_id = "general";
  
  // Users join the room
  for (user_id, username) in &users {
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    
    let user_event = format!(
      "{{\"userId\": \"{}\", \"username\": \"{}\", \"eventType\": \"join\", \"roomId\": \"{}\", \"timestamp\": {}}}",
      user_id, username, room_id, timestamp
    );
    
    g.send_msg(user_event_generator, "input", &user_event);
    
    // Sleep briefly
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
  
  // Users send messages
  let messages = vec![
    ("user1", "Hello everyone!"),
    ("user2", "Hi Alice!"),
    ("user3", "Hey folks, how's it going?"),
    ("user4", "Good morning!"),
    ("user1", "What are you all working on today?"),
    ("user2", "I'm building a chat app with Fractalide!")
  ];
  
  for (user_id, content) in &messages {
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    
    let username = users.iter()
      .find(|(id, _)| id == user_id)
      .map(|(_, name)| name)
      .unwrap_or(&"Unknown");
    
    let chat_message = format!(
      "{{\"userId\": \"{}\", \"username\": \"{}\", \"content\": \"{}\", \"roomId\": \"{}\", \"timestamp\": {}}}",
      user_id, username, content, room_id, timestamp
    );
    
    g.send_msg(message_generator, "input", &chat_message);
    
    // Sleep briefly
    std::thread::sleep(std::time::Duration::from_millis(500));
  }
  
  // User leaves the room
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();
  
  let user_event = format!(
    "{{\"userId\": \"user3\", \"username\": \"Charlie\", \"eventType\": \"leave\", \"roomId\": \"{}\", \"timestamp\": {}}}",
    room_id, timestamp
  );
  
  g.send_msg(user_event_generator, "input", &user_event);
  
  // Get all messages
  g.send_msg(message_generator, "get_messages", room_id);
  
  // Get all users
  g.send_msg(user_event_generator, "get_users", room_id);
}