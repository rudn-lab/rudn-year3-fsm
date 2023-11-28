function canvasHasFocus() {
	return (document.activeElement || document.body) == document.body;
}

var caretTimer;
var caretVisible = true;

function resetCaret() {
	clearInterval(caretTimer);
	caretTimer = setInterval('caretVisible = !caretVisible; draw()', 500);
	caretVisible = true;
}

var canvas;
var nodeRadius = 30;
var nodes = [];
var links = [];

var cursorVisible = true;
var snapToPadding = 6; // pixels
var hitTargetPadding = 6; // pixels
var selectedObject = null; // either a Link or a Node
var currentLink = null; // a Link
var movingObject = false;
var originalClick;

var do_not_refresh = false;

function drawUsing(c) {
	c.clearRect(0, 0, canvas.width, canvas.height);
	c.save();
	c.translate(0.5, 0.5);

	c.lineWidth = 2;
	for(var i = 0; i < nodes.length; i++) {
		c.fillStyle = c.strokeStyle = (nodes[i] == selectedObject) ? 'red' : 'white';
		nodes[i].draw(c);
	}
	for(var i = 0; i < links.length; i++) {
		c.fillStyle = c.strokeStyle = (links[i] == selectedObject) ? 'red' : 'white';
		try {
			links[i].draw(c);
		} catch (e) {
			links.splice(i, 1);
			i = i-1;
		}
	}
	if(currentLink != null) {
		c.fillStyle = c.strokeStyle = 'white';
		currentLink.draw(c);
	}

	c.restore();
}

function draw() {
	drawUsing(canvas.getContext('2d'));
	if(!do_not_refresh) {
		saveJson();
	}
}

function selectObject(x, y) {
	for(var i = 0; i < nodes.length; i++) {
		if(nodes[i].containsPoint(x, y)) {
			return nodes[i];
		}
	}
	for(var i = 0; i < links.length; i++) {
		if(links[i].containsPoint(x, y)) {
			return links[i];
		}
	}
	saveJson();
	return null;
}

function snapNode(node) {
	for(var i = 0; i < nodes.length; i++) {
		if(nodes[i] == node) continue;

		if(Math.abs(node.x - nodes[i].x) < snapToPadding) {
			node.x = nodes[i].x;
		}

		if(Math.abs(node.y - nodes[i].y) < snapToPadding) {
			node.y = nodes[i].y;
		}
	}
}

var prepare_canvas = function(new_canvas) {
	canvas = new_canvas;
	restoreBackup();
	draw();

	canvas.onmousedown = function(e) {
		saveJson();

		var mouse = crossBrowserRelativeMousePos(e);
		selectedObject = selectObject(mouse.x, mouse.y);
		movingObject = false;
		originalClick = mouse;

		if(selectedObject != null) {
			if(shift && selectedObject instanceof Node) {
				currentLink = new SelfLink(selectedObject, mouse);
			} else {
				movingObject = true;
				deltaMouseX = deltaMouseY = 0;
				if(selectedObject.setMouseStart) {
					selectedObject.setMouseStart(mouse.x, mouse.y);
				}
			}
			resetCaret();
		} else if(shift) {
			currentLink = new TemporaryLink(mouse, mouse);
		}

		draw();

		if(canvasHasFocus()) {
			// disable drag-and-drop only if the canvas is already focused
			return false;
		} else {
			// otherwise, let the browser switch the focus away from wherever it was
			resetCaret();
			return true;
		}
	};

	canvas.ondblclick = function(e) {
		var mouse = crossBrowserRelativeMousePos(e);
		selectedObject = selectObject(mouse.x, mouse.y);

		if(selectedObject == null) {
			selectedObject = new Node(mouse.x, mouse.y);
			nodes.push(selectedObject);
			resetCaret();
			draw();
		} else if(selectedObject instanceof Node) {
			selectedObject.isAcceptState = !selectedObject.isAcceptState;
			draw();
		}
	};

	canvas.onmousemove = function(e) {
		var mouse = crossBrowserRelativeMousePos(e);

		if(currentLink != null) {
			var targetNode = selectObject(mouse.x, mouse.y);
			if(!(targetNode instanceof Node)) {
				targetNode = null;
			}

			if(selectedObject == null) {
				if(targetNode != null) {
					currentLink = new StartLink(targetNode, originalClick);
				} else {
					currentLink = new TemporaryLink(originalClick, mouse);
				}
			} else {
				if(targetNode == selectedObject) {
					currentLink = new SelfLink(selectedObject, mouse);
				} else if(targetNode != null) {
					currentLink = new Link(selectedObject, targetNode);
				} else {
					currentLink = new TemporaryLink(selectedObject.closestPointOnCircle(mouse.x, mouse.y), mouse);
				}
			}
			draw();
		}

		if(movingObject) {
			selectedObject.setAnchorPoint(mouse.x, mouse.y);
			if(selectedObject instanceof Node) {
				snapNode(selectedObject);
			}
			draw();
		}
	};

	canvas.onmouseup = function(e) {
		movingObject = false;

		if(currentLink != null) {
			if(!(currentLink instanceof TemporaryLink)) {
				selectedObject = currentLink;
				links.push(currentLink);
				resetCaret();
			}
			currentLink = null;
			draw();
		}
	};

	canvas.onkeydown = function(e) {
		var key = crossBrowserKey(e);
	
		if(key == 16) {
			shift = true;
		} else if(!canvasHasFocus()) {
			// don't read keystrokes when other things have focus
			return true;
		} else if(key == 8) { // backspace key
			if(selectedObject != null && 'text' in selectedObject) {
				selectedObject.text = selectedObject.text.substr(0, selectedObject.text.length - 1);
				resetCaret();
				draw();
			}
	
			// backspace is a shortcut for the back button, but do NOT want to change pages
			return false;
		} else if(key == 46) { // delete key
			if(selectedObject != null) {
				for(var i = 0; i < nodes.length; i++) {
					if(nodes[i] == selectedObject) {
						nodes.splice(i--, 1);
					}
				}
				for(var i = 0; i < links.length; i++) {
					if(links[i] == selectedObject || links[i].node == selectedObject || links[i].nodeA == selectedObject || links[i].nodeB == selectedObject) {
						links.splice(i--, 1);
					}
				}
				selectedObject = null;
				draw();
			}
		}
	};

	canvas.onkeyup = function(e) {
		var key = crossBrowserKey(e);
	
		if(key == 16) {
			shift = false;
		}
	};
	
	document.onkeypress = function(e) {
		// don't read keystrokes when other things have focus
		var key = crossBrowserKey(e);
		if(!canvasHasFocus()) {
			// don't read keystrokes when other things have focus
			return true;
		} else if(key >= 0x20 && key <= 0x7E && !e.metaKey && !e.altKey && !e.ctrlKey && selectedObject != null && 'text' in selectedObject) {
			selectedObject.text += String.fromCharCode(key);
			resetCaret();
			draw();
	
			// don't let keys do their actions (like space scrolls down the page)
			return false;
		} else if(key == 8) {
			// backspace is a shortcut for the back button, but do NOT want to change pages
			return false;
		}
	};
}

var shift = false;


function crossBrowserKey(e) {
	e = e || window.event;
	return e.which || e.keyCode;
}

function crossBrowserElementPos(e) {
	e = e || window.event;
	var obj = e.target || e.srcElement;
	var x = 0, y = 0;
	while(obj.offsetParent) {
		x += obj.offsetLeft;
		y += obj.offsetTop;
		obj = obj.offsetParent;
	}
	return { 'x': x, 'y': y };
}

function crossBrowserMousePos(e) {
	e = e || window.event;
	return {
		'x': e.pageX || e.clientX + document.body.scrollLeft + document.documentElement.scrollLeft,
		'y': e.pageY || e.clientY + document.body.scrollTop + document.documentElement.scrollTop,
	};
}

function crossBrowserRelativeMousePos(e) {
	var element = crossBrowserElementPos(e);
	var mouse = crossBrowserMousePos(e);
	return {
		'x': mouse.x - element.x,
		'y': mouse.y - element.y
	};
}

function fetch_json() {
	return saveJson();
}

function saveJson() {
	var oldSelectedObject = selectedObject;
	selectedObject = null;
	var jsonData = saveBackup();
	selectedObject = oldSelectedObject;
	return jsonData;
}

function load_from_json(s) {
	selectedObject = null;
	restoreFromStruct(JSON.parse(s));
	draw();
}