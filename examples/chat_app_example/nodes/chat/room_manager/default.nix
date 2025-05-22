{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  flowscript = with nodes; with edges; ''
    # Input ports
    new_message => message_store(${chat.message_store}) new_message
    user_event => user_registry(${chat.user_registry}) user_event
    get_messages => message_store get_messages
    get_users => user_registry get_users
    
    # Error handling
    message_store error => err(${msg_printer}) input
    user_registry error => err input
    
    # Output ports
    message_store messages => messages
    user_registry users => users
    user_registry user_count => user_count
  '';
}