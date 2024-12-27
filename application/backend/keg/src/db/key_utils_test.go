package db

import (
    "testing"
)

func TestRemoveSuffixFromValidKey(t *testing.T) {
    key := "key.avail"
    expectedTrimmed := "key"
    resultTrimmed, err := removeAvailKeySuffix(key)
    if err != nil {
        t.Errorf("Returned error: %v", err)
    }
    if resultTrimmed != expectedTrimmed {
        t.Errorf("Bad trimming, should be '%s', is '%s'", expectedTrimmed, resultTrimmed)
    }
}

func TestRemoveSuffixFromValidKeyContainsSuffix(t *testing.T) {
    key := "key.avail.key.avail"
    expectedTrimmed := "key.avail.key"
    resultTrimmed, err := removeAvailKeySuffix(key)
    if err != nil {
        t.Errorf("Returned error: %v", err)
    }
    if resultTrimmed != expectedTrimmed {
        t.Errorf("Bad trimming, should be '%s', is '%s'", expectedTrimmed, resultTrimmed)
    }
}

func TestRemoveSuffixFromInvalidKey(t *testing.T) {
    key := "key"
    resultTrimmed, err := removeAvailKeySuffix(key)
    if err == nil {
        t.Errorf("Did not return error, returned key '%s'", resultTrimmed)
    }
}

func TestAppendSuffixAppends(t *testing.T) {
    key := "key"
    expectedAppend := "key.used"
    resultAppend := appendUsedKeySuffix(key)
    if resultAppend != expectedAppend {
        t.Errorf("Returned incorrect suffixed key; should be '%s', is '%s'", expectedAppend, resultAppend)
    }
}
