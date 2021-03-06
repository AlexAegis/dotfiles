#!/bin/sh

yes | sdkmanager --sdk_root="${ANDROID_HOME}" "tools"

yes | sdkmanager --sdk_root="${ANDROID_HOME}" --update
sdkmanager --sdk_root="${ANDROID_HOME}" --list
yes | sdkmanager --sdk_root="${ANDROID_HOME}" "build-tools;28.0.3" "platform-tools" "platforms;android-28" "tools"
yes | sdkmanager --sdk_root="${ANDROID_HOME}" --licenses
