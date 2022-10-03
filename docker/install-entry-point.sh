#!/bin/sh

[ $# -ne 1 ]  && echo "Usage: $0 GID" && exit 1
STAGE_GID=$1

# Generates content for the dev container's entry point.
gen-entry-point-file()
{
    cat <<EOF
#!/bin/bash

if [ ! -f /.dockerenv ] ; then
    echo "This script must be run in a Docker container."
    exit 1
fi

if [ \$# -ne 4 ] ; then
    echo "Usage: \$0 UID GID USER GROUP"
    exit 1
fi

XUID=\$1
XGID=\$2
XUSER=\$3
XGROUP=\$4

sudo groupadd --gid \$XGID \$XGROUP
sudo useradd -l --no-create-home --uid \$XUID --gid \$XGID --groups $STAGE_GID --shell /bin/bash \$XUSER

sudo find /home/stage -maxdepth 1 -mindepth 1 | while read F ; do
    TARGET=/home/\$XUSER/\`basename "\$F"\`
    sudo ln -s "\$F" "\$TARGET" && \
        sudo chown -h \$XUID:\$XGID "\$TARGET"
done

sudo chown \$XUID:\$XGID /home/\$XUSER
[ -d /home/\$XUSER/src ] && cd /home/\$XUSER/src
exec sudo -H -u \$XUSER bash
EOF
}

mkdir -p /entry
gen-entry-point-file > /entry/dev-shell-init
chmod -R 755 /entry

