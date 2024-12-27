package db

import (
	"fmt"
	"strings"
)

const AVAIL string = ".avail"
const AVAIL_WILDCARD string = "*.avail"
const USED string = ".used"

func getKeyUsedName(availKey string) (usedKey string) {
    return rev(strings.Replace(rev(availKey), rev(AVAIL), rev(USED), 1))
}

func getKeyAvailName(usedKey string) (availKey string) {
    return rev(strings.Replace(rev(usedKey), rev(USED), rev(AVAIL), 1))
}

func parseAvailKeys(keys []string) (actualKeys []string) {
    for _, key := range keys {
        actualKey, err := removeAvailKeySuffix(key)
        if err != nil {
            continue // todo log
        }
        actualKeys = append(actualKeys, actualKey)
    }
    return actualKeys
}

func removeAvailKeySuffix(key string) (string, error) {
    if !strings.Contains(key, AVAIL) {
        return "", fmt.Errorf("Invalid key, does not end in '%s' suffix: '%s'", AVAIL, key)
    }
    return strings.TrimSuffix(key, AVAIL), nil
}

func appendUsedKeySuffix(key string) string {
    return fmt.Sprintf("%s%s", key, USED)
}

func rev(s string) string {
    runes := []rune(s)
    for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
        runes[i], runes[j] = runes[j], runes[i]
    }
    return string(runes)
}
