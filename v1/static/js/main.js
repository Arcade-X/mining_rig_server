const socket = new WebSocket("ws://127.0.0.1:8080/ws/");

socket.addEventListener('open', function (event) {
    console.log("WebSocket is connected.");
});

socket.addEventListener('message', function (event) {
    console.log('Message from server: ', event.data);
    loadFarms(); // Refresh the Farm list when a new message is received
});