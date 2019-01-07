var activeRoom;
var active_room_name;
var call_to_api;

function getConfiguration()
{
  var configuration ={
    url :'ws://localhost:3400/',
    agent:'chat',
    instance : "test-instance1",
    dna :"Qm328wyq38924y"
  }

  return configuration
}


 function getRooms() {
  var configuration = getConfiguration();
  call_to_api(configuration.instance, "chat", "main", "get_my_channels")({
    }).then(response => {
      var rooms = JSON.parse(response)
      $("#rooms").empty()
      rooms = rooms.sort(function(a,b){
        if (a.name < b.name)
          return -1;
        if (a.name > b.name)
          return 1;
        return 0;
      });
      for(i=0;i<rooms.length;i++){
        $("#rooms").append(
          "<li data-id=\""+rooms[i].id+"\""+
              "data-name=\""+rooms[i].name+"\">"+
               "#"+rooms[i].name+
          "</li>"
        )
      }

        setActiveRoom()

    });
 }

 function addRoom() {
   var configuration = getConfiguration();
   var room = {
     name: $("#room-name-input").val(),
     access: "public"
   }

   $("#room-name-input").val('')

   call_to_api(configuration.instance, "chat", "main", "create_channel")({
      name: room.name,
      description: "user generated room",
      public: room.public =="public"?true:false
    }).then(response => {
      console.log("Response is :" + response)
    })

}

 function selectRoom(event) {
   $("#rooms li").removeClass("selected-room")
   activeRoom = $(this).data('id')
   active_room_name = $(this).data('name')
   setActiveRoom();

 }

 function setActiveRoom() {
   var roomElement = $("#rooms li[data-id="+activeRoom+"]")
   $(roomElement).addClass("selected-room")
   $("#messages-header").text("Messages in #"+active_room_name)
   getMessages()

 }

function getMessages() {

  var configuration = getConfiguration();
  call_to_api(configuration.instance, "chat", "main", "get_messages")({
      channel_name: active_room_name || ""
    })
    .then(response =>{
      var messages = JSON.parse(response);

      if (messages.error) return;

      var sorted_messages = messages.sort(function(a,b){
        return new Date(b.timestamp) - new Date(a.timestamp);
      });

      $("#messages").empty()
      for(var i=0;i<sorted_messages.length;i++) {
        console.log("what is the text:" +sorted_messages[i].text );
        $("#messages").append("<li class=\"list-unstyled\">"+
           "<span class=\"timestamp\">"+sorted_messages[i].timestamp+"</span>"+
           "<span class=\"message\">"+sorted_messages[i].text+"</span>"+
        "</li>")
      }
  })
}

 function sendMessage() {
   var text = $("#message-input").val()
   var configuration = getConfiguration();
   call_to_api(configuration.instance, "chat", "main", "post_message")({
      message: {text,timestamp : new Date()},
      channel_name: active_room_name || ""
    })
    .then(response =>{
      console.log("response : +" + response)
    })
}


 $(window).ready(function() {
   var configuration = getConfiguration();
   holoclient.connect(configuration.url).then(({call, close}) =>
   {
        call_to_api = call;
   })

  $("#room-name-button").click(addRoom);
    $("#rooms").on("click", "li", selectRoom)
    $("#message-button").click(sendMessage);
    $("#room-name-input").keyup(function(event){
        if(event.keyCode == 13) $("#room-name-button").click()
    })

    $("#message-input").keyup(function(event){
        if(event.keyCode == 13) $("#message-button").click()
    })

    setInterval(function(){
      getMessages();
      getRooms();
    },3000)
 });
