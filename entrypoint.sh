#!/bin/bash
set -e

if [ ! -f "$HOME/.vnc/passwd" ]; then
	mkdir -p "$HOME/.vnc"
	PASSWORD="${VNC_PASSWORD:-vncpassword}"
	echo "$PASSWORD" | vncpasswd -f >"$HOME/.vnc/passwd"
	chmod 600 "$HOME/.vnc/passwd"
	echo "Generated VNC password."
fi

if [ ! -f "$HOME/.vnc/xstartup" ]; then
	cat <<'EOF' >"$HOME/.vnc/xstartup"
#!/bin/bash
if [ -x /usr/bin/dbus-launch ]; then
  exec dbus-launch --exit-with-session startxfce4
else
  exec startxfce4
fi
EOF
	chmod +x "$HOME/.vnc/xstartup"
	echo "Created xstartup to launch XFCE via dbus-launch."
fi

echo "Starting VNC server on :1 (port 5901, listening on all interfaces)..."
vncserver :1 -geometry 1280x800 -depth 24 -localhost no

echo "Starting noVNC on 0.0.0.0:6080 -> 0.0.0.0:5901..."
/usr/bin/websockify --web=/usr/share/novnc 0.0.0.0:6080 0.0.0.0:5901 &

tail -F "$HOME/.vnc/$(hostname):1.log"
