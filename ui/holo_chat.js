

var activeRoom;
var active_room_name;

function getConfiguration()
{
  var configuration ={
    url :'ws://localhost:3400/',
    agent:'chat',
    instance : "chat",
    dna :"Qm328wyq38924y"
  }

  return configuration
}


 function getMyProfile() {
   $.get("/fn/profiles/getProfile", "", function(profile){
     $("#title-username").text(JSON.parse(profile).firstName)
   });
 }
/*
 function register() {
   $.post("/fn/identity/registerDpkiKeyTo", "", function(arr){
     if(arr=="true"){
       $("#varified").text("Registered")
     }else{$("#varified").text("Not Registered")}
   });
 }
*/
 function getRooms() {
  var configuration = getConfiguration();
  holoclient.connect(configuration.url).then(({call, close}) => 
  {
    call(configuration.instance, "chat", "main", "get_my_channels")({
    }).then(response => {
      var rooms = JSON.parse(response)
      $("#rooms").empty()
      for(i=0;i<rooms.length;i++){
        $("#rooms").append(
          "<li data-id=\""+rooms[i].id+"\""+
              "data-name=\""+rooms[i].name+"\">"+
               "#"+rooms[i].name+
          "</li>"
        )
      }
      if(activeRoom) {
        setActiveRoom()
      }
    });
  })
 
 }

 function addRoom() {
   var configuration = getConfiguration();
   var room = {
     name: $("#room-name-input").val(),
     access: "public"
   }

   $("#room-name-input").val('')
  holoclient.connect(configuration.url).then(({call, close}) => 
  {
    call(configuration.instance, "chat", "main", "create_channel")({
      name: room.name,
      description: "user generated room",
      initial_members: [],
      public: room.public =="public"?true:false
    }).then(response => {
      console.log("Response is :" + response)
    })
    close()
 })
}

 function selectRoom(event) {
   $("#rooms li").removeClass("selected-room")
   activeRoom = $(this).data('id')
   active_room_name = $(this).data('name')

   setActiveRoom()
 }

 function setActiveRoom() {
   var roomElement = $("#rooms li[data-id="+activeRoom+"]")
   $(roomElement).addClass("selected-room")
   $("#messages-header").text("Messages in #"+$(roomElement).data("name"))
   getMessages()
 }

function getMessages() {

  var configuration = getConfiguration();
  holoclient.connect(configuration.url).then(({call, close}) => 
  {
    call(configuration.instance, "chat", "main", "get_messages")({
      channel_address:active_room_name,
      min_count:10
    })
    .then(response =>{
      $("#messages").empty()
      var messages = JSON.parse(response);
      messages = messages.sort(function(a,b){
        timeA = new Date(a.timestamp)
        timeB = new Date(b.timestamp)
        return timeA > timeB
      });
      for(var i=0;i<messages.length;i++) {
        $("#messages").append("<li class=\"list-unstyled\">"+
           "<span class=\"timestamp\">"+messages[i].timestamp+"</span>"+
           "<span class=\"message\">"+messages[i].text+"</span>"+
        "</li>")
      }
  })
})
  


}

 function sendMessage() {
   var text = $("#message-input").val()
   var configuration = getConfiguration();
 

   holoclient.connect(configuration.url).then(({call, close}) => 
  {
    call(configuration.instance, "chat", "main", "post_message")({
      message: {text,timestamp : new Date()},
      channel_address: active_room_name
    })
    .then(response =>{
      console.log("response : +" + response) 
      })
    })
}



/*
//TODO this METHORD will retrive the post it has to be displayed
 function getTag (tag) {
    $.post("/fn/messages/getPostsByTag",tag,function(arr) {
     arr=JSON.parse(arr);
     console.log("posts: " + JSON.stringify(arr));
//TODO Display the posts

        }
    );
}
function openTag(){$('#tagDialog').modal('show');}

function passTag() {
    var hashtag = $("#tagHandle").val();
    getTag(hashtag);
    $('#tagDialog').modal('hide');
  }
*/
 $(window).ready(function() {
   var configuration = getConfiguration();
    $("#room-name-button").click(addRoom)
    $("#rooms").on("click", "li", selectRoom)
    $("#message-button").click(sendMessage)
    //$('#tagButton').click(openTag);
    //$('#submitTag').click(passTag);

    $("#room-name-input").keyup(function(event){
        if(event.keyCode == 13) $("#room-name-button").click()
    })

    $("#message-input").keyup(function(event){
        if(event.keyCode == 13) $("#message-button").click()
    })
    setInterval(poll, 5000)
 });

 function poll()
 {
   getMessages();
   getRooms();
 }
