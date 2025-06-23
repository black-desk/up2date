#!/usr/bin/env bash

# SPDX-FileCopyrightText: Chen Linxuan <me@black-desk.cn>
#
# SPDX-License-Identifier: GPL-3.0-or-later

# NOTE:
# Use /usr/bin/env to find shell interpreter for better portability.
# Reference: https://en.wikipedia.org/wiki/Shebang_%28Unix%29#Portability

# NOTE:
# Exit immediately if any commands (even in pipeline)
# exits with a non-zero status.
set -e
set -o pipefail

# WARNING:
# This is not reliable when using POSIX sh
# and current script file is sourced by `source` or `.`
CURRENT_SOURCE_FILE_PATH="${BASH_SOURCE[0]:-$0}"
CURRENT_SOURCE_FILE_NAME="$(basename -- "$CURRENT_SOURCE_FILE_PATH")"

# shellcheck disable=SC2016
USAGE="$CURRENT_SOURCE_FILE_NAME

Description:
  Tool to start using this template

Usage:
  $CURRENT_SOURCE_FILE_NAME -h
  $CURRENT_SOURCE_FILE_NAME [YOUR_PROJECT_NAME]

Options:
  -h   Show this screen."

# This function log messages to stderr works like printf
# with a prefix of the current script name.
# Arguments:
#   $1 - The format string.
#   $@ - Arguments to the format string, just like printf.
function log() {
	local format="$1"
	shift
	# shellcheck disable=SC2059
	printf "$CURRENT_SOURCE_FILE_NAME: $format\n" "$@" >&2 || true
}

function main() {
	while getopts ':h' option; do
		case "$option" in
		h)
			echo "$USAGE"
			exit
			;;
		\?)
			log "[ERROR] Unknown option: -%s" "$OPTARG"
			exit 1
			;;
		esac
	done
	shift $((OPTIND - 1))

	if [ $# -ne 1 ]; then
		log "[ERROR] Missing project name"
		exit 1
	fi

	local project_name="$1"
	if [ -z "$project_name" ]; then
		log "[ERROR] Project name cannot be empty"
		exit 1
	fi

	if ! command -v sed >/dev/null; then
		log "[ERROR] sed command not found, please install it"
		exit 1
	fi

	sed -i '' "s/^#\ Template/#\ $project_name/g" README.md
	sed -i '' "s/^#\ 模板/#\ $project_name/g" README.zh_CN.md
}

main "$@"
