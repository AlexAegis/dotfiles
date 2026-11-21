#!/bin/sh

rm -rf "${XDG_DATA_HOME:?}/approval-queue" "${XDG_CACHE_HOME:?}/approval-queue" \
	"${XDG_RUNTIME_DIR:?}/approval-queue"
