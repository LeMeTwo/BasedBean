package db

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
)

type Lock interface {
    Acquire(ctx context.Context) (acquired bool, deadline time.Time, err error)
    Release(ctx context.Context) (released bool, err error)
}

type RedisExpiringLock struct {
    client *redis.Client
    acquire_timeout int
    lock_timeout int
    identifier string
    lock_key string
}

func NewRedisExpiringLock(client *redis.Client, acquire_timeout_s int, lock_timeout_s int) RedisExpiringLock {
    return RedisExpiringLock{
        client: client,
        acquire_timeout: acquire_timeout_s,
        lock_timeout: lock_timeout_s,
        identifier: uuid.NewString(),
        lock_key: "ExpiringLock",
    }
}

func (rl RedisExpiringLock) Acquire(ctx context.Context) (acquired bool, deadline time.Time, err error) {
    acquire_lua := redis.NewScript(`
        local key = KEYS[1]
        local timeout = ARGV[1]
        local identifier = ARGV[2]

        if redis.call('exists', key) == 0 or redis.call('get', key) == identifier then
            return (redis.call('setex', key, timeout, identifier).ok == 'OK')
        end
        return 0
        `)
    deadline = time.Now().Add(time.Second * time.Duration(rl.lock_timeout))
    acq_timeout := time.Now().Add(time.Second * time.Duration(rl.acquire_timeout))
    for time.Now().Before(acq_timeout) {
        acquired, err = acquire_lua.Run(
            ctx,
            rl.client,
            []string{rl.lock_key},
            []string{rl.lockTimeoutAsString(), rl.identifier},
            ).Bool()
    
        if acquired {
            break
        } else {
            time.Sleep(time.Millisecond * 100)
        }
    }
    return
}

func (rl RedisExpiringLock) lockTimeoutAsString() string {
    return fmt.Sprintf("%d", rl.lock_timeout)
}

func (rl RedisExpiringLock) Release(ctx context.Context) (released bool, err error) {
    release_lua := redis.NewScript(`
        local key = KEYS[1]
        local identifier = ARGV[1]

        if redis.call('get', key) == identifier then
            return redis.call('del', key)
        else
            return false
        end
        `)
    released, err = release_lua.Run(
        ctx,
        rl.client,
        []string{rl.lock_key},
        []string{rl.identifier},
        ).Bool()
    return
}
