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
difficulty=hard
motd=MinePi
view-distance=14
online-mode=true
EOF

# online-mode=false allows players with non original copies of minecraft to join
# with an offline account
