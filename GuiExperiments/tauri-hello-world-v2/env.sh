# Exporting env vars officially needed by Tauri: https://v2.tauri.app/start/prerequisites/#android
export JAVA_HOME=$HOME/bin/android-studio-2025.2.2.8-linux/jbr
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

# This wasn't document (and my not be needed by Tauri) but allows to use command line
# tools manually.
export PATH=$PATH:$ANDROID_HOME/platform-tools