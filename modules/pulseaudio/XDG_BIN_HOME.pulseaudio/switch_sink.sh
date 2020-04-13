#!/bin/sh

# FEATURES
# - Doesn't show the sink that is already the default
# - Automatically switches all running input sinks when switching the default sink

# Get the current default sink
SINK_DEFAULT=$(pactl info | ag "Default Sink" | ag -o "(?!.*:)[^\s].*")
# Get the audio sink names
SINK_NAMES=$(pactl list sinks | ag Name | cut --complement -c 1-7)

# Get the index of the default sink
DEFAULT_INDEX=$(printf "$SINK_NAMES" | ag --number "$SINK_DEFAULT" | sed 's/:.*//' )

# Get the audio sink descriptions
SINK_DESCRIPTIONS=$(pactl list sinks | ag Description | cut --complement -c 1-13)
# Get all the programs that revieve the audio from the sinks
SINK_INPUTS=$(pactl list sink-inputs | ag "Sink Input #")

# Get the descriptions from the sinks without the default sink to display to the user and have the user pick a sink
SINK_DESCRIPTION=$(printf "$SINK_DESCRIPTIONS" | sed "${DEFAULT_INDEX}d" | dmenu -i -fn "Roboto Mono for Powerline-11" -p "Select sink:")

# Get the index for the answer that the user wrote
DESCRIPTION_INDEX=$(printf "$SINK_DESCRIPTIONS" | ag --number "$SINK_DESCRIPTION" | sed 's/:.*//')
DESCRIPTION_INDEX=$(($DESCRIPTION_INDEX - 1))

# Set the default sink
pactl set-default-sink $DESCRIPTION_INDEX

# Change all the ouputs for the programs that are using the default sink
printf "$SINK_INPUTS\n" | while read -r SINK_INPUT
do
	# Get the index for the program
	SINK_INPUT_INDEX=$(printf "$SINK_INPUT" | ag -o "(?!Sink Input#)[0-9]+")
	pactl move-sink-input $SINK_INPUT_INDEX $DESCRIPTION_INDEX
done
