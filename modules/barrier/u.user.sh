#!/bin/sh

# Update Server Fingerprint

mkdir -p "$XDG_DATA_HOME/barrier/SSL/Fingerprints/"
trusted_file="$XDG_DATA_HOME/barrier/SSL/Fingerprints/TrustedServers.txt"


if timelock barrier_ssl; then
    if [ ! -f  "$XDG_DATA_HOME/barrier/SSL/Barrier.pem" ]; then
        (
            cd "$XDG_DATA_HOME/barrier/SSL/"
            openssl req -x509 -nodes -days 365 -subj /CN=Barrier -newkey rsa:4096 \
                -keyout Barrier.pem -out Barrier.pem
        )
    fi
    # Relock for another year
    timelock barrier_ssl 31536000
else
	echo "The barrier_ssl timelock is locked so no new ssl cert is generated,\
 it unlocks at $(timelock -p barrier_ssl)"
fi


openssl s_client -connect "${BARRIER_SERVER_IP}:${BARRIER_SERVER_PORT}" \
    < /dev/null 2>/dev/null | \
    openssl x509 -fingerprint -sha256 -noout -in /dev/stdin | \
    cut -d '=' -f 2 | sed -e 's/://g' -e 's/^/v2:sha256:/' > "$trusted_file"

openssl s_client -connect "${BARRIER_SERVER_IP}:${BARRIER_SERVER_PORT}" \
    < /dev/null 2>/dev/null | \
    openssl x509 -fingerprint -sha1 -noout -in /dev/stdin | \
    cut -d '=' -f 2 | sed -e 's/://g' -e's/^/v2:sha1:/' >> "$trusted_file"


# Stop and disable all server listeners and start the currently configured one

# Not working until this is fixed https://github.com/systemd/systemd/issues/15620
# systemctl disable --user --now 'barrier@*'

# Workaround
cp "$XDG_CONFIG_HOME/systemd/user/barrier@.service" \
    "$XDG_CONFIG_HOME/systemd/user/barrier@.service.bak"
systemctl --no-reload disable --user barrier@.service
systemctl stop --user 'barrier@*.service'
systemctl --user daemon-reload
cp "$XDG_CONFIG_HOME/systemd/user/barrier@.service.bak" \
    "$XDG_CONFIG_HOME/systemd/user/barrier@.service"

# Enable barrier client
systemctl enable --user --now "barrier@${BARRIER_SERVER_IP}"
