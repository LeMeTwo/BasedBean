package db

import (
    "context"
    "fmt"

    "github.com/redis/go-redis/v9"
)

type DbConfig struct {
    Addr string
    Password string
}

const ACQUIRE_TIMEOUT int = 5
const LOCK_TIMEOUT int = 5

type KeyStorage interface {
    FetchBatchReservedKeys(ctx context.Context, size int) (keys []string, err error)
    ExpireKey(ctx context.Context, key string) (err error)
}

type KeyDbStorage struct {
    db KeyDatabase
    lock Lock
}

func NewKeyDbStorage(client *redis.Client) KeyDbStorage {
    return KeyDbStorage{
        db: KeyDb{client},
        lock: NewRedisExpiringLock(client, ACQUIRE_TIMEOUT, LOCK_TIMEOUT),
    }
}

func (kds KeyDbStorage) FetchBatchReservedKeys(ctx context.Context, size int) (keys []string, err error) {
    acquired, err := kds.lock.Acquire(ctx)
    if acquired {
        defer kds.lock.Release(ctx)

        keys, err := kds.db.ScanKeys(ctx, size)
        if err != nil {
            return nil, err
        }
        kds.db.ReserveKeys(ctx, keys)

        return parseAvailKeys(keys), nil
    } else {
        return nil, fmt.Errorf("Could not acquire lock: %v", err)
    }
}

func (kds KeyDbStorage) ExpireKey(ctx context.Context, key string) (err error) {
    acquired, err := kds.lock.Acquire(ctx)
    key = appendUsedKeySuffix(key)
    if acquired {
        defer kds.lock.Release(ctx)
        return kds.db.ExpireKey(ctx, key)
    } else {
        return fmt.Errorf("Could not acquire lock: %v", err)
    }
}
