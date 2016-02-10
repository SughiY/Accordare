
var socket = null

function error() {
	    alert('Stream generation failed.');
}

function intercationWithServer(){
        socket = WSEvent.connect('ws://127.0.0.1:3012') 
	var sender = document.getElementById('message_box') 
		sender.onchange = function(target){ 
			var text = target.target.value 
			socket.send(text)                                 
	} 
	socket.addListener('foo', function(data){ 
		console.log(data) 
	})
}

function gotStream(stream){
	var mediaRecorder = new StereoAudioRecorder(stream)
	mediaRecorder.mimeType = 'audio/wav'
	mediaRecorder.audioChannels = 2
	mediaRecorder.ondataavailable = function (blob) {
		socket.send(blob)
	}
	mediaRecorder.start(300)
}

window.onload = function() {

	navigator.getUserMedia = 
		navigator.getUserMedia ||
		navigator.webkitGetUserMedia ||
		navigator.mozGetUserMedia

	navigator.getUserMedia({"audio":true}, gotStream, error)

	intercationWithServer()
}
