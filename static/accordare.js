
var socket = null
var detectorElem, 
    canvasElem,
    pitchElem,
    noteElem, 
    detuneElem,
    detuneAmount,
    buttonElem

var rafID = null

var noteStrings = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
var hasOpenedTrack = false;
var stopTracks;

function noteFromPitch( frequency ){
	var noteNum = 12 * (Math.log( frequency / 440)/Math.log(2) )
	return Math.round(noteNum) + 69
}

function frequencyFromNoteNumber( note ){
	return 440 * Math.pow(2, (note - 69) / 12)
}	

function centsOffFromPitch( frequency, note ){
	return Math.floor( 1200 * Math.log( frequency / frequencyFromNoteNumber(note))/Math.log(2) )
}

function updatePitch(ac) {

 	if (ac == -2) {
 		detectorElem.className = "vague location"
	 	pitchElem.innerText = "--"
		noteElem.innerText = "-"
		detuneElem.className = ""
		detuneAmount.innerText = "--"
 	} else {
	 	detectorElem.className = "confident location"
	 	pitch = ac
	 	pitchElem.innerHTML = Math.round( pitch ) 
	 	var note =  noteFromPitch( pitch )
		noteElem.innerHTML = noteStrings[note%12]
		var detune = centsOffFromPitch( pitch, note )
		if (detune == 0 ) {
			detuneElem.className = ""
			detuneAmount.innerHTML = "--"
		} else {
			if (detune < 0)
				detuneElem.className = "flat"
			else
				detuneElem.className = "sharp"
			detuneAmount.innerHTML = Math.abs( detune )
		}
	}

	if (!window.requestAnimationFrame)
		window.requestAnimationFrame = window.webkitRequestAnimationFrame
	rafID = window.requestFrame()
}

function error() {
	    alert('Stream generation failed.');
}

function intercationWithServer(){
        socket = WSEvent.connect('ws://127.0.0.1:3012') 

	socket.addListener('pitch', function(data){ 
		console.log(data) 
		updatePitch(data)
	})
}




function gotStream(stream){

	var mediaRecorder = new StereoAudioRecorder(stream)
	mediaRecorder.mimeType = 'audio/wav'
	mediaRecorder.audioChannels = 2
	mediaRecorder.ondataavailable = function (blob) {
		console.log(blob.size)
		if (blob.size < 60000) socket.send(blob)
	}
	mediaRecorder.start(300)
	stopTracks = function(){
		mediaRecorder.stop()
	}
}

window.onload = function() {

	intercationWithServer()	
	detectorElem = document.getElementById("detector")
	canvasElem = document.getElementById("output")
	pitchElem = document.getElementById("pitch")
	noteElem = document.getElementById("note")
	detuneElem = document.getElementById("detune")
	detuneAmount = document.getElementById("detune_amt")
	buttonElem = document.getElementById("OnOff")

	buttonElem.onclick = toggleLiveInput
}

function toggleLiveInput(){
	if (!hasOpenedTrack) {

	navigator.getUserMedia = 
		navigator.getUserMedia ||
		navigator.webkitGetUserMedia ||
		navigator.mozGetUserMedia

		navigator.getUserMedia({"audio":true}, gotStream, error)
		hasOpenedTrack = true
		buttonElem.innerHTML = "close audio track"
	} else {

		hasOpenedTrack = false
		stopTracks()
		buttonElem.innerHTML = "user live input"

	}
}

