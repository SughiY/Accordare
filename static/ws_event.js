(function (){
	"use strict"
  function WSEvent(){ }

WSEvent.prototype = EventEmitter.prototype
  var proto = WSEvent.prototype
  var exports = this
  
  WSEvent.connect = function connect(addr){
	  var wse = new WSEvent()
	  wse.socket = new WebSocket(addr)
	  wse.socket.onmessage = function(event) { 
		  var data = JSON.parse(event.data) 
			  //data's structure needs to be designed
			  wse.emitEvent(data.event, [data.content]) 
			  console.log(data.event) 
	  }
	  return wse
  }
  proto.send = function send(text){
	this.socket.send(text)  
  }
  
  // Expose the class either via AMD, CommonJS or the global object 
  if (typeof define === 'function' && define.amd) { 
	  define(function () { 
		  return WSEvent; 
	  }); 
  } 
  else if (typeof module === 'object' && module.exports){ 
	  module.exports = WSEvent; 
  } 
  else { 
	  exports.WSEvent = WSEvent; 
  }

}.call(this))
