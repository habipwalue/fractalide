{ buffet }:

{
  chat_message = buffet.support.edge.capnp {
    src = ./chat_message.capnp;
    schema = "ChatMessage";
  };
  
  user_event = buffet.support.edge.capnp {
    src = ./user_event.capnp;
    schema = "UserEvent";
  };
}