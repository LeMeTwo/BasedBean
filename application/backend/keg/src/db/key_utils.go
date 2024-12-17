package db

import (
    "strings"
)

const AVAIL string = ".avail"
const AVAIL_WILDCARD string = "*.avail"
const USED string = ".used"

func getKeyUsedName(availKey string) (usedKey string) {
    return rev(strings.Replace(rev(availKey), rev(AVAIL), rev(USED), 1))
}

func rev(s string) string {
    runes := []rune(s)
    for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
        runes[i], runes[j] = runes[j], runes[i]
    }
    return string(runes)
}
