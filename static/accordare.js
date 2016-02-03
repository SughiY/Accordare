window.AudioContext = window.AudioContext || window.webkitAudioContext;

var audioContext = null;

window.onload = function() {
	audioContext = new AudioContext()

	navigator.getUserMedia({audio: true}, function(stream) {
		  var microphone = context.createMediaStreamSource(stream)
		  microphone.connect(context.destination)
	}, errorCallback)
}
