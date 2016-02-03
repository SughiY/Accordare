window.AudioContext = window.AudioContext || window.webkitAudioContext;

var audioContext = null;

function error() {
	    alert('Stream generation failed.');
}

function intercationWithServer(){
	var socket = WSEvent.connect('ws:127.0.0.1:3012') 
		var sender = document.getElementById('message_box') 
		sender.onchange = function(target){ 
			var text = target.target.value 
				socket.send(text)                                 
		} 
	socket.addListener('foo', function(data){ 
		console.log(data) 
	})
}

window.onload = function() {
	audioContext = new AudioContext()

	navigator.getUserMedia = 
		navigator.getUserMedia ||
		navigator.webkitGetUserMedia ||
		navigator.mozGetUserMedia

		navigator.getUserMedia({"audio":true}, function(stream) {
		  var microphone = audioContext.createMediaStreamSource(stream)
		  microphone.connect(audioContext.destination)
	}, error)

	intercationWithServer()
}




