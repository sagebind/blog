window.Gallery = {
	ShowWebsite: function(url)
	{
		window.popup = document.createElement("div");
		document.body.appendChild(popup);
		popup.className = "popup";

		popup.style.width = "960px";
		popup.style.height = "600px";
		popup.style.left = ((window.innerWidth / 2) - 480) + "px";
		popup.style.top = ((window.innerHeight / 2) - 300) + "px";

		var close = document.createElement("a");
		popup.appendChild(close);
		close.href = "javascript:void(null);";
		close.className = "close";
		close.onclick = function()
		{
			document.body.removeChild(popup);
			this.onclick = null;
		};

		var frame = document.createElement("iframe");
		popup.appendChild(frame);
		frame.src = url;
		frame.height = popup.offsetHeight - 48;
		
		return false;
	}
};