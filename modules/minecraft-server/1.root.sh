#!/bin/sh

crudini --merge /srv/minecraft/eula.txt << EOF
eula=true
EOF

crudini --merge /etc/conf.d/minecraft << EOF
SERVER_START_CMD="java -Xms2048M -Xmx4096M -jar ./minecraft_server.jar nogui"
IDLE_SERVER=true
GAME_PORT="26666"
EOF

crudini --merge /srv/minecraft/server.properties << EOF
query.port=26666
rcon.port=26676
server-port=26666
motd=MinePi
EOF
