#!/bin/bash

TOMAT_VER="v1.0.0-rc"
LINUX_MD5="2189489b227d025a680a946042a7b4b7"
DOWNLOADED_TARBALL="tomat.tar.gz"
LOCAL_SETUP_FOLDER=".tomat"
LOCAL_SETUP_PATH=$HOME/$LOCAL_SETUP_FOLDER
EXECUTABLE_NAME="tomat"
EXECUTABLE_PATH=$LOCAL_SETUP_PATH/$EXECUTABLE_NAME
REMOTE_LINUX_TARBALL="https://github.com/aydoganersoz/tomat/releases/download/$TOMAT_VER/tomat-$TOMAT_VER-linux-amd64.tar.gz"

echo "Setting up tomat..."
if [[ "$OSTYPE" == darwin* ]] ; then {
    curl -L -o tomat "$OSX_TARBALL" && \
    [[ $(md5 -r tomat) == "$OSX_MD5" ]] && \
    chmod +x tomat && \
    ./tomat -v
} elif [[ "$OSTYPE" == linux* ]] ; then {
    curl -L -o $DOWNLOADED_TARBALL $REMOTE_LINUX_TARBALL && \
    echo $LINUX_MD5 $DOWNLOADED_TARBALL | md5sum -c - && \
    mkdir -p $LOCAL_SETUP_PATH
    tar -xzvf $DOWNLOADED_TARBALL --strip-components=1 -C $LOCAL_SETUP_PATH
    chmod +x $EXECUTABLE_PATH && \
    $EXECUTABLE_PATH --version
} else {
    echo "OS is not compatible"
    exit -1
}
fi
echo "tomat-$TOMAT_VER installed, don't add $LOCAL_SETUP_PATH to path and always run $EXECUTABLE_NAME as below:"
echo "\`cd $LOCAL_SETUP_PATH\`"
echo "\`./$EXECUTABLE_NAME\`"