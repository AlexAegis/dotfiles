#!/bin/sh

yes | sdkmanager --sdk_root="${ANDROID_HOME}" "tools"

yes | sdkmanager --sdk_root="${ANDROID_HOME}" --update
sdkmanager --sdk_root="${ANDROID_HOME}" --list
yes | sdkmanager --sdk_root="${ANDROID_HOME}" "build-tools;29.0.3" "platform-tools" "platforms;android-31" "tools"
yes | sdkmanager --sdk_root="${ANDROID_HOME}" --licenses

adb start-server
