package cache

import (
    "testing"
)

func TestEmptyCacheReturnsEmpty(t *testing.T) {
    cache := LocalKeyCache{}
    if !cache.IsEmpty() {
        t.Error("Returned not empty.")
    }
}

func TestNonEmptyCacheReturnsNotEmpty(t *testing.T) {
    cache := LocalKeyCache{}
    cache.PushKeys([]string{"key"})
    if cache.IsEmpty() {
        t.Error("Returned empty.")
    }
}

func TestGetKeyFromEmptyCache(t *testing.T) {
    cache := LocalKeyCache{}
    key, err := cache.GetKey()
    if key != "" || err == nil {
        t.Error("Did not return err.")
    }
}

func TestGetKeyFromNonEmptyCache(t *testing.T) {
    cache := LocalKeyCache{}
    key := "key1"
    cache.PushKeys([]string{key})
    cKey, err := cache.GetKey()
    if cKey != key || err != nil {
        t.Error("Did not return valid key.")
    }
}

func TestCacheIsEmptyAfterGettingAllKeys(t *testing.T) {
    cache := LocalKeyCache{}
    keys := []string{"key1", "key2"}
    cache.PushKeys(keys)
    for i := 0; i < len(keys); i++ {
        key, err := cache.GetKey()
        if key == "" || err != nil {
            t.Errorf("Returned err.")
        }
    }
    key, err := cache.GetKey()
    if key != "" || err == nil {
        t.Errorf("Did not return err.")
    }
    if !cache.IsEmpty() {
        t.Error("Returned not empty.")
    }
}
