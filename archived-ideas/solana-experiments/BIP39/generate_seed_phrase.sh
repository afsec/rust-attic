#!/bin/bash
BIP39_FILE="english.txt"

WORDS_POSITIONS=$(cat /dev/urandom |LC_ALL=C tr -dc '01' | fold -w 11 | head -n 12)

for i in ${WORDS_POSITIONS}
do
	LINE_NUMBER=$(echo $((2#$i)))
	QUERY=$(echo "^${LINE_NUMBER}:")
	WORD=$(grep -n '' ${BIP39_FILE} | grep "${QUERY}" | cut -d ':' -f 2)
	printf "${WORD} "

done
echo
