package db

import (
	"context"
	"fmt"

	"github.com/redis/go-redis/v9"
)

type KeyDoesNotExist struct {
    key string
}

func (e *KeyDoesNotExist) Error() string {
    return fmt.Sprintf("Key '%s' does not exist.", e.key)
}

type KeyDatabase interface {
    ScanKeys(ctx context.Context, size int) (keys []string, err error)
    ReserveKeys(ctx context.Context, keys []string) (err error)
    ExpireKey(ctx context.Context, key string) (err error) 
    Ping(ctx context.Context) error
}

type KeyDb struct {
    client *redis.Client
}

func (kdb KeyDb) ScanKeys(ctx context.Context, size int) (keys []string, err error) {
    var cursor uint64
    for {
        var kdbKeys []string
        kdbKeys, cursor, err = kdb.client.Scan(ctx, cursor, AVAIL_WILDCARD, 100).Result()
        if err != nil {
            return
        }
        keys = append(keys, kdbKeys[:]...)
        if cursor == 0 || len(keys) >= size {
            break
        }
    }

    return
}

func (kdb KeyDb) ReserveKeys(ctx context.Context, keys []string) (err error) {
    for _, key := range keys {
        _, err = kdb.client.Rename(ctx, key, getKeyUsedName(key)).Result()
        if err != nil {
            return
        }
    }
    return
}

func (kdb KeyDb) ExpireKey(ctx context.Context, key string) (err error) {
    exists, err := kdb.client.Exists(ctx, key).Result()
    if err != nil {
        return err
    }
    if exists == 0 {
        return &KeyDoesNotExist{key: key}
    }
    _, err = kdb.client.Rename(ctx, key, getKeyAvailName(key)).Result()
    return
}

func (kdb KeyDb) Ping(ctx context.Context) error {
    _, err := kdb.client.Ping(ctx).Result()
    return err
}
